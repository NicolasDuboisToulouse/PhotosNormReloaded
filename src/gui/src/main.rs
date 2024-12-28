#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod assets;
mod taffy_tools;
mod welcome_pannel;
use welcome_pannel::WelcomePannel;

fn main() -> eframe::Result {
    let paths = WelcomePannel::show();
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
