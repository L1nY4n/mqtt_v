use backend::message::{Publish, QoS, ToBackend};
use eframe::egui::{self, RadioButton, ScrollArea, TextEdit};

use crate::ui::widgets::{docking, packet::PacketUI};

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
        &"publish"
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
        ui.push_id("pubulish_tab", |ui| {
            ui.add(TextEdit::singleline(&mut self.topic).desired_width(120.0));

            // ui.horizontal(|ui| {
            //     ui.
            //     ui.selectable_value(&mut self.qos, QoS::AtLeastOnce, "AtLeastOnce");
            //     ui.selectable_value(&mut self.qos, QoS::AtMostOnce, "AtMostOnce");
            //     ui.selectable_value(&mut self.qos, QoS::ExactlyOnce, "ExactlyOnce");
            // });

           
            ui.horizontal(|ui| {
                ui.label("Qos:");
               ui
                    .radio_value(&mut self.qos, QoS::AtMostOnce, "AtMostOnce");
                 
                  ui
                    .radio_value(&mut self.qos, QoS::AtLeastOnce, "AtLeastOnce");
                 
                   ui
                    .radio_value(&mut self.qos, QoS::ExactlyOnce, "ExactlyOnce");

                  ui.separator();

                  if  ui.selectable_label(self.retain, "retain").clicked(){
                    self.retain = ! self.retain;
                  }
                 
            });

            ui.add(
                egui::TextEdit::multiline(&mut self.payload)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    // .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY), // .layouter(&mut layouter),
            );

            if ui.button("publish").clicked() {
                if let Some(tx) = &client.publish_tx {
                    let mut publish = Publish::new(self.topic.clone(), self.qos, self.payload.as_bytes());
                    publish.retain = self.retain;
                    println!("{:?}", publish);
                    let res = tx.try_send(publish);
                    println!("{:?}", res);
                } else {
                    println!("no tx")
                }
            }
        });
    }
}
