use std::time::Duration;

use rumqttc::MqttOptions;
pub use rumqttc::{mqttbytes::QoS, ClientError, Event, Outgoing, Packet, Publish, Subscribe};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub enum MqttOpts {
    V3(OptionsV3),
    V5(),
}

#[derive(Default, Clone)]
pub struct OptionsV3 {
    pub client_id: String,
    pub broker_addr: String,
    pub port: u16,
    pub keep_alive: bool,
    pub heatbbeat: u64,
    pub clean_session: bool,
}

impl Default for MqttOpts {
    fn default() -> Self {
        MqttOpts::V3(OptionsV3 {
            client_id: "mosquitto".to_string(),
            broker_addr: "test.mosquitto.org".to_string(),
            port: 1883,
            keep_alive: true,
            heatbbeat: 20,
            clean_session: false,
        })
    }
}

impl OptionsV3 {
    pub fn convert(self) -> MqttOptions {
        let mut opts = MqttOptions::new(self.client_id, self.broker_addr, self.port);
        if self.keep_alive {
            opts.set_keep_alive(Duration::from_secs(self.heatbbeat));
            opts.set_clean_session(self.clean_session);
        }
        opts
    }
}

pub enum ToBackend {
    Startup,
    Shutdown,

    NewClient(MqttOpts),
}

pub type ClientId = String;
pub type Topic = String;

#[derive(Debug)]
pub enum FromClient {
    Event(Event),
    PublishReslt(Result<(), ClientError>),
}

#[derive(Debug)]
pub enum ToClient {
  Connect,
   Disconnect,
    Publish(Publish),
    Subscribe((Topic, QoS)),
}
#[derive(Debug)]
pub enum ToFrontend {
    ClientCreated(ClientId, Sender<ToClient>),
    ClientMsg(ClientId, FromClient),
}
