#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod assets;
mod main_panel;
mod taffy_tools;
mod welcome_panel;
use clap::Parser;
use core::{assets as core_assets, tools};
use main_panel::MainPanel;
use welcome_panel::WelcomePanel;

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
        WelcomePanel::show()
    } else {
        args.files
    };

    let images = tools::expand_folders(&paths);

    if !images.is_empty() {
        return MainPanel::show(&images);
    }

    Ok(())
}
