use backend::message::{ToBackend, Publish, QoS};
use eframe::egui::{self, ScrollArea};

use crate::ui::widgets::{docking, packet::PacketUI};

use super::client::Client;



pub struct PubulishTab {
  
}

impl PubulishTab {
 pub   fn new() -> Self {
        Self {
       
         
        }
    }
}

impl docking::Tab<Client> for PubulishTab {
    fn title(&self) -> &str {
        &"publish"
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
        ui.push_id("pubulish_tab", |ui|{
          if  ui.button("publish").clicked() {
            if let Some(tx) =  &client.publish_tx {
                  let res =  tx.try_send(Publish::new("a", QoS::AtLeastOnce, "test"));
                  println!("{:?}",res);
            }else {
                println!("no tx")
            }
           
          }
        });
    }
}
