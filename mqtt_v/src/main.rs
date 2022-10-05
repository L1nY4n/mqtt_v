#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{epaint::Vec2, NativeOptions};
use ui::MqttAppUI;

mod ui;
mod util;

#[tokio::main]
async fn main() {
    util::enable_tracing();
    let native_options = NativeOptions {
        initial_window_size: Some(Vec2::new(970., 600.)),
        min_window_size: Some(Vec2::new(600., 300.)),
        ..NativeOptions::default()
    };

    eframe::run_native(
        "mqtt V",
        native_options,
        Box::new(|cc| Box::new(MqttAppUI::new(cc))),
    );
}
