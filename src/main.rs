use std::fs;

use crate::metadata::tag::DisplayEnumSet;
use clap::{builder::ArgPredicate, Args, CommandFactory, Parser, Subcommand};
use clap_markdown::MarkdownOptions;
use metadata::Metadata;

mod metadata;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(flatten_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// info: Print some metadata from provided files
    Info(InfoArgs),

    /// set: Update tags
    Set(SetArgs),

    /// fix: Fix file properties
    Fix(FixArgs),

    #[command(hide = true)]
    GenerateReadmeMd,
}

#[derive(Args, Debug)]
struct InfoArgs {
    /// images to load
    #[clap(required = true, value_name = "FILES")]
    images: Vec<std::path::PathBuf>,
}

#[derive(Args, Debug)]
struct SetArgs {
    #[command(flatten)]
    setters: SetArgsSetters,

    /// Allows to set same tag values to several images
    #[arg(short, long)]
    force: bool,

    /// images to update
    #[clap(required = true, value_name = "FILES")]
    images: Vec<std::path::PathBuf>,
}
#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
struct SetArgsSetters {
    /// Update ImageDescription tag (-t: title)
    #[arg(short = 't', long)]
    description: Option<String>,

    /// Update DateTimeOriginal and CreateDate tags
    #[arg(short, long)]
    date: Option<String>,
}

#[derive(Args, Debug)]
struct FixArgs {
    /// Apply all fixes (default)
    #[arg(
        short,
        long,
        default_value_if("fixers", ArgPredicate::IsPresent, "false"),
        default_value("true")
    )]
    all: bool,

    #[command(flatten)]
    setters: FixArgsFixers,

    /// images to fix
    #[clap(required = true, value_name = "FILES")]
    images: Vec<std::path::PathBuf>,
}

#[derive(Args, Debug)]
#[group(id = "fixers", required = false, multiple = true)]
struct FixArgsFixers {
    /// Fix ExifImageWidth/Height according to real image width/height
    #[arg(short, long)]
    dimensions: bool,

    /// Fix file name to %Y_%m_%d-%H_%M_%S[ - %description]
    #[arg(short, long)]
    name: bool,
}

macro_rules! print_table {
    ($input1:expr, $input2:expr) => {
        println!("{0:<15} {1:}", $input1, $input2);
    };
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let images = match &args.command {
        Commands::Info(args) => &args.images,
        Commands::Set(args) => {
            if !args.force && args.images.len() != 1 {
                panic!("Set same tag values to several images is not allowed unless you use --force option.");
            }
            &args.images
        }
        Commands::Fix(args) => &args.images,
        Commands::GenerateReadmeMd => {
            let readme_text = clap_markdown::help_markdown_command_custom(
                &Cli::command(),
                &MarkdownOptions::new()
                    .title(CARGO_PKG_NAME.to_string())
                    .show_footer(false)
                    .show_table_of_contents(true),
            );
            fs::write("README.md", readme_text).expect("Unable to write README.md");
            return Ok(());
        }
    };

    for image in images.iter() {
        print_table!("File:", image.display());

        let result = Metadata::new(image);
        if result.is_err() {
            print_table!("Error!", result.err().expect("Unexpected error."));
            println!();
            continue;
        }

        let mut metadata = result.unwrap();

        match &args.command {
            //
            // Command info
            //
            Commands::Info(_) => {
                print_table!(
                    "Dimensions:",
                    format!("{}, {}", metadata.width(), metadata.height())
                );
                print_table!(
                    "Date:",
                    metadata
                        .exif_date()
                        .unwrap_or("{No exif date!}".to_string())
                );
                print_table!(
                    "Desription:",
                    metadata
                        .description()
                        .unwrap_or("{No exif description!}".to_string())
                );
                print_table!("Camera:", metadata.camera_info());
            }

            //
            // Command set
            //
            Commands::Set(args) => {
                if args.setters.description.is_some() {
                    metadata.set_description(args.setters.description.as_ref().unwrap());
                }
                if args.setters.date.is_some() {
                    let result = metadata
                        .set_date_from_exif(args.setters.date.as_ref().unwrap().to_string());
                    if result.is_err() {
                        panic!(
                            "Cannot parse date: '{}': {}!",
                            args.setters.date.as_ref().unwrap(),
                            result.err().unwrap()
                        );
                    }
                }

                match metadata.save() {
                    Err(e) => {
                        print_table!("Error!", e);
                    }
                    Ok(tags) => {
                        print_table!("Updated tags:", tags.to_string_coma());
                    }
                }
            }
            Commands::Fix(args) => {
                if args.all || args.setters.dimensions {
                    metadata.fix_dimentions();
                }
                if args.all || args.setters.name {
                    metadata.fix_file_name();
                }
                match metadata.save() {
                    Err(e) => {
                        print_table!("Error!", e);
                    }
                    Ok(tags) => {
                        print_table!("Updated tags:", tags.to_string_coma());
                    }
                }
            }

            Commands::GenerateReadmeMd => {
                panic!("Cannot reach this code!");
            }
        }

        println!();
    }

    Ok(())
}
