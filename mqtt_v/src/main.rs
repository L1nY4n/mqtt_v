#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use eframe::{NativeOptions, epaint::Vec2};
use ui::MqttAppUI;

mod util;
mod ui;

#[tokio::main]
async fn  main() {
   util::enable_tracing();
  let native_options = NativeOptions{
    initial_window_size: Some(Vec2::new(970., 600.)),
    min_window_size: Some(Vec2::new(600., 300.)),
    .. NativeOptions::default()
  };

  eframe::run_native("mqtt v", native_options, Box::new(|cc| Box::new(MqttAppUI::new(cc))));

}
