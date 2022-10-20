use eframe::egui::{self};

use crate::ui::widgets::docking;

use super::client::Client;

pub struct StatTab {}

impl StatTab {
    pub fn new() -> Self {
        Self {}
    }
}

impl docking::Tab<Client> for StatTab {
    fn title(&self) -> &str {
        "📈 stat"
    }

    fn ui(&mut self, ui: &mut egui::Ui, _client: &mut Client) {
        ui.push_id("statistics", |_ui| {});
    }
}
