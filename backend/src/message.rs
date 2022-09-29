use std::time::Duration;

use rumqttc::{MqttOptions};
pub use rumqttc::{Event,Packet,Outgoing,Publish,mqttbytes::QoS};
use tokio::sync::mpsc::Sender;


#[derive(Clone)]
pub enum MqttOpts {
    V3(OptionsV3),
    V5()
} 

#[derive(Default,Clone)]
pub struct OptionsV3{
  pub  client_id: String,
  pub  broker_addr: String,
  pub  port: u16,
  pub keep_alive: bool,
  pub heatbbeat: u64
  
}

impl OptionsV3 {
  pub  fn convert(self)-> MqttOptions{
      let mut  opts =   MqttOptions::new(self.client_id, self.broker_addr, self.port);
      if self.keep_alive {
        opts.set_keep_alive(Duration::from_secs(self.heatbbeat));
      }
      opts
    }
}

pub enum ToBackend {
    Startup,
    Shutdown,

    NewClient(MqttOpts),
}

type  ClientId  = String;
#[derive(Debug)]
pub enum ToFrontend {
   ClientCreated(ClientId,Sender<rumqttc::Publish>),
   Packet(ClientId,Event)
}