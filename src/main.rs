use clap::Parser;
use metadata::Metadata;
use std::io::Error;

mod metadata;

#[derive(Parser)]
struct Cli {
    /// image to load
    image: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let result = Metadata::new(&args.image);

    if result.is_err() {
        return Err(Error::other(format!(
            "Cannot parse file '{}'",
            args.image.display()
        )));
    }

    let metadata = result.unwrap();
    println!("File: {}", metadata.path().display());
    println!("Dimensions: {}, {}", metadata.width(), metadata.height());
    println!(
        "Date: {}",
        metadata.exif_date().unwrap_or("No date!".to_string())
    );

    Ok(())
}
