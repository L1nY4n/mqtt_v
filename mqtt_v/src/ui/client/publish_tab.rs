use backend::message::{Publish, QoS};
use eframe::{
    egui::{self, style::Margin, Layout, TextEdit},
    emath::Align,
    epaint::Color32,
};

use crate::ui::widgets::docking;

use super::client::Client;

pub struct PubulishTab {
    topic: String,
    qos: QoS,
    retain: bool,
    payload: String,
}

impl PubulishTab {
    pub fn new() -> Self {
        Self {
            topic: "".to_owned(),
            qos: QoS::AtLeastOnce,
            retain: false,
            payload: "".to_owned(),
        }
    }
}

impl docking::Tab<Client> for PubulishTab {
    fn title(&self) -> &str {
        "publish"
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
        ui.push_id("pubulish_tab", |ui| {
            egui::Frame::default()
                .outer_margin(Margin::symmetric(2., 6.))
                .inner_margin(4.)
                .rounding(4.)
                .fill(Color32::BLACK)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.group(|ui| {
                                if ui.selectable_label(self.retain, "retain").clicked() {
                                    self.retain = !self.retain;
                                }
                                ui.separator();
                                ui.radio_value(&mut self.qos, QoS::AtMostOnce, "AtMostOnce");
                                ui.radio_value(&mut self.qos, QoS::AtLeastOnce, "AtLeastOnce");
                                ui.radio_value(&mut self.qos, QoS::ExactlyOnce, "ExactlyOnce");
                                ui.label("Qos:");
                            });
                            ui.group(|ui| {
                                let w = ui.available_width() - 48.0;
                                let topic_edit_w = if w > 0.0 { w } else { 20.0 };

                                ui.add(
                                    TextEdit::singleline(&mut self.topic)
                                        .desired_width(topic_edit_w),
                                );
                                ui.label("Topic:");
                            });
                        });
                    });
                    ui.add_space(2.);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.group(|ui| {
                            ui.set_width(200.);
                            if ui.button("publish").clicked() {
                                if let Some(tx) = &client.publish_tx {
                                    let mut publish = Publish::new(
                                        self.topic.clone(),
                                        self.qos,
                                        self.payload.as_bytes(),
                                    );
                                    publish.retain = self.retain;
                                    println!("{:?}", publish);
                                    let res =
                                        tx.try_send(backend::message::ToClient::Publish(publish));
                                    println!("{:?}", res);
                                } else {
                                    println!("no tx")
                                }
                            }
                        });
                        ui.group(|ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.payload)
                                    .code_editor()
                                    .desired_width(f32::INFINITY),
                            );
                        });
                    });
                });
        });
    }
}
