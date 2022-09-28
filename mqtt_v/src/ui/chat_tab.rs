use eframe::{egui::{self, ScrollArea, Layout, TextEdit}, emath::Align};

use super::{widgets::{docking, packet::PacketUI}, client::Client};

pub struct ChatView {
    title: String,
    filter: String
}

impl ChatView {
 pub   fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            filter: "".to_owned()
        }
    }
}

impl docking::Tab<Client> for ChatView {
    fn title(&self) -> &str {
        &self.title
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
       ui.push_id("chart_view", |ui|{
         ui.add_space(4.0);

         ui.horizontal(|ui|{
            ui.set_width(ui.available_width());
             ui.with_layout(Layout::left_to_right(Align::Center),|ui| {
            ui.label("🔭");
            ui.add(TextEdit::singleline(&mut self.filter).desired_width(120.0));
            self.filter = self.filter.to_lowercase();
            if ui.button("ｘ").clicked() {
                self.filter.clear();
            }
        });
        });
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
