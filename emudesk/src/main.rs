//! EmuDesk entry point: launches the eframe/egui GUI.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod github;
mod help;
mod profile;
mod rdp;
mod theme;

use app::EmuDeskApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([700.0, 520.0])
            .with_title("EmuDesk"),
        ..Default::default()
    };

    eframe::run_native(
        "com.emudesk.desktop",
        options,
        Box::new(|cc| Ok(Box::new(EmuDeskApp::new(cc)))),
    )
}
