#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod assets;
mod taffy_tools;
mod welcome_pannel;
use clap::Parser;
use core::assets as core_assets;
use welcome_pannel::WelcomePannel;

const CARGO_BIN_NAME: &str = env!("CARGO_BIN_NAME");

#[derive(Parser)]
#[command(version, about = core_assets::DOC_GUI, long_about = None, name=CARGO_BIN_NAME)]
#[command(propagate_version = true)]
#[command(flatten_help = true)]
struct Cli {
    /// images to load
    #[clap(required = false, value_name = "IMAGES/FOLDERS")]
    files: Vec<std::path::PathBuf>,
}

fn main() -> eframe::Result {
    let args = Cli::parse();
    let paths = if args.files.is_empty() {
        WelcomePannel::show()
    } else {
        args.files
    };

    println!(
        "Paths: {}",
        paths
            .iter()
            .map(|f| f.to_string_lossy())
            .collect::<Vec<_>>()
            .join(",\n")
    );

    Ok(())
}
