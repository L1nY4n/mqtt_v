use backend::message::Event;
use eframe::{
    egui::{self, InnerResponse, Layout, ScrollArea, TextEdit, style::Margin},
    emath::Align,
    epaint::Color32,
    Frame,
};

use crate::ui::widgets::{docking, packet::PacketUI};

use super::{
    client::Client,
};

pub struct ChatTab {
    filter: Filter,
}

#[derive(Default)]
struct Filter {
    pub direct: Direct,
    pub topic: String,
    pub content: String,
}

#[derive(Default)]
struct Direct {
    published: bool,
    received: bool,
}
impl Filter {
    fn filter(&self, event: &Event) -> bool {
        true
    }
}

impl ChatTab {
    pub fn new() -> Self {
        Self {
            filter: Default::default(),
        }
    }
}

impl docking::Tab<Client> for ChatTab {
    fn title(&self) -> &str {
        "   Event"
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
        ui.push_id("chat_tab", |ui| {
            egui::Frame::default()
                .outer_margin(Margin::symmetric (2.,6.))
                .inner_margin(4.)
                .rounding(4.)
                .fill(Color32::BLACK)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                     
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .selectable_label(self.filter.direct.published, "published")
                                .clicked()
                            {
                                self.filter.direct.published = !self.filter.direct.published
                            };
                            ui.separator();
                            if ui
                            .selectable_label(self.filter.direct.received, "received")
                            .clicked()
                        {
                            self.filter.direct.received = !self.filter.direct.received
                        };
                        ui.separator();
                        if ui.button("ｘ").clicked() {
                            self.filter.topic.clear();
                        }
                        ui.add(
                            TextEdit::singleline(&mut self.filter.topic).desired_width(120.0),
                        );

                     
                        ui.colored_label(Color32::YELLOW,"🔭");
                        });
                    });
                });
            ui.add_space(12.);
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
