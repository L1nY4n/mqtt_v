use eframe::egui::{self, ScrollArea};

use super::{widgets::{docking, packet::PacketUI}, client::Client};

pub struct TreeView {
    title: String,
}

impl TreeView {
 pub   fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
         
        }
    }
}

impl docking::Tab<Client> for TreeView {
    fn title(&self) -> &str {
        &self.title
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
        ui.push_id("tree_view", |ui|{
            ui.add_space(4.0);
        ScrollArea::vertical()
            .stick_to_bottom(true)
           // .max_width(ui.available_width())
            .show(ui, |ui| {
                for pkt in &client.packets {
                    PacketUI::new(pkt.clone()).show(ui)
                }
            });
        });
    }
}
