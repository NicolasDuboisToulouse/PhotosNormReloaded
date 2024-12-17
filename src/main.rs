use crate::metadata::tag::DisplayEnumSet;
use clap::{builder::ArgPredicate, Args, CommandFactory, Parser, Subcommand};
use clap_markdown::MarkdownOptions;
use colored::Colorize;
use metadata::Metadata;
use std::fs;

mod metadata;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub const DOC: &str = "PhotosNorm: A simple tool to lossless manipulate images properties.\n\
                       \n\
                       info: display some EXIF info.\n\
                       set:  Update some EXIF tags. More info below or with set --help.\n\
                       fix:  Fix properties like orientation, file name, ... More info below or with fix --help.\n\
                       \n\
                       To each command, you can provide one or more files and/or folders.\n\
                       Each known files (aka images) will be processed, other ones will be ignored.\n\
                       For each folder, all files within will be analysed like described just before. Sub-folders will be \
                       ignored (this is non-recursive).";

#[derive(Parser)]
#[command(version, about = DOC, long_about = None)]
#[command(propagate_version = true)]
#[command(flatten_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// info: display some EXIF info
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
    #[clap(required = true, value_name = "IMAGES/FOLDERS")]
    files: Vec<std::path::PathBuf>,
}

#[derive(Args, Debug)]
struct SetArgs {
    #[command(flatten)]
    setters: SetArgsSetters,

    /// Allows to set same tag values to several images
    #[arg(short, long)]
    force: bool,

    /// images to update
    #[clap(required = true, value_name = "IMAGES/FOLDERS")]
    files: Vec<std::path::PathBuf>,
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
    #[clap(required = true, value_name = "IMAGES/FOLDERS")]
    files: Vec<std::path::PathBuf>,
}

#[derive(Args, Debug)]
#[group(id = "fixers", required = false, multiple = true)]
struct FixArgsFixers {
    /// Fix ExifImageWidth/Height according to real image width/height
    #[arg(short, long)]
    dimensions: bool,

    /// Fix file name to %Y_%m_%d-%H_%M_%S[ - %description].
    /// File names may be numbered to prevent erasing file with same name.
    #[arg(short, long)]
    name: bool,

    /// Fix image orientation (lossless rotate the image).
    /// Only JPEG files are supported.
    #[arg(short, long)]
    orientation: bool,
}

macro_rules! print_table {
    ($input1:expr, $input2:expr) => {
        println!("{0:<15} {1:}", $input1, $input2);
    };
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    // Parse command and grab file list
    let files = match &args.command {
        Commands::Info(args) => &args.files,
        Commands::Set(args) => &args.files,
        Commands::Fix(args) => &args.files,
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

    // list images from file list (aka read folders)
    let mut images: Vec<std::path::PathBuf> = Vec::new();
    for file in files.iter() {
        if !file.is_dir() {
            images.push(file.to_path_buf());
        } else {
            match fs::read_dir(file) {
                // Let open display the error and process next file.
                Err(_) => images.push(file.to_path_buf()),
                // Add all files to image list
                Ok(files) => {
                    for entry in files {
                        let file = entry.unwrap().path();
                        // non-recursive
                        if file.is_file() {
                            images.push(file.to_path_buf());
                        }
                    }
                }
            }
        }
    }

    // Check parameters
    if let Commands::Set(ref args) = args.command {
        if !args.force && images.len() != 1 {
            panic!("{}: Setting same tag values to several images is not allowed unless you use {} option.", "error".red(), "--force".yellow());
        }
    }

    // Process all images
    for image in images.iter() {
        print_table!("File:", image.display());

        let result = Metadata::new(image);
        if result.is_err() {
            print_table!("Error!".red(), result.err().expect("Unexpected error."));
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
                        .unwrap_or("{No exif date!}".yellow().to_string())
                );
                print_table!(
                    "Desription:",
                    metadata
                        .description()
                        .unwrap_or("{No exif description!}".yellow().to_string())
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
                            "{}: Cannot parse date: '{}': {}!",
                            "error".red(),
                            args.setters.date.as_ref().unwrap().yellow(),
                            result.err().unwrap()
                        );
                    }
                }

                match metadata.save() {
                    Err(e) => {
                        print_table!("Error!".red(), e);
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
                if args.all || args.setters.orientation {
                    metadata.fix_orientation();
                }
                match metadata.save() {
                    Err(e) => {
                        print_table!("Error!".red(), e);
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
