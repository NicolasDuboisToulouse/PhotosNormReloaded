use clap::{Args, Parser, Subcommand};
use metadata::Metadata;

mod metadata;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(flatten_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print some metadata from provided files
    Info(InfoArgs),
}

#[derive(Args)]
struct InfoArgs {
    /// images to load
    #[clap(required = true, value_name = "FILES")]
    images: Vec<std::path::PathBuf>,
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
    };

    for image in images.iter() {
        print_table!("File:", image.display());

        let result = Metadata::new(image);
        if result.is_err() {
            print_table!("Error!", result.err().expect("Unexpected error."));
            println!();
            continue;
        }

        let metadata = result.unwrap();
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
        println!();
    }

    Ok(())
}
