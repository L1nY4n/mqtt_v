use std::time::Duration;

use rumqttc::MqttOptions;
pub use rumqttc::{ClientError, Event, Outgoing, Packet, Publish, QoS, Subscribe};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MqttOpts {
    V3(OptionsV3),
    V5(OptionsV5),
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsV3 {
    pub client_id: String,
    pub broker_addr: String,
    pub port: u16,
    pub keep_alive: bool,
    pub heatbbeat: u64,
    pub clean_session: bool,
    pub max_packet_size: (u16, u16),
    pub credentials: bool,
    pub username: String,
    pub password: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsV5 {
    pub client_id: String,
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
            max_packet_size: (u16::MAX, u16::MAX),
            credentials: false,
            username: "".to_owned(),
            password: "".to_owned(),
        })
    }
}

impl MqttOpts {
    pub fn client_id(&self) -> String {
        match self {
            MqttOpts::V3(v3) => v3.client_id.clone(),
            MqttOpts::V5(v5) => v5.client_id.clone(),
        }
    }
}

impl OptionsV3 {
    pub fn convert(self) -> MqttOptions {
        let mut opts = MqttOptions::new(self.client_id, self.broker_addr, self.port);
        opts.set_max_packet_size(self.max_packet_size.0.into(), self.max_packet_size.1.into());
        if self.credentials {
            opts.set_credentials(self.username, self.password);
        }

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

pub type PublishRef = String;

#[derive(Debug)]
pub enum FromClient {
    Disconnected,
    Event(Event),
    PublishReslt(PublishRef, Result<(), ClientError>),
}

#[derive(Debug)]
pub enum ToClient {
    Connect,
    Disconnect,
    Publish(PublishRef, Publish),
    Subscribe((Topic, QoS)),
    Unsubscribe(Topic),
}
#[derive(Debug)]
pub enum ToFrontend {
    ClientCreated(ClientId, Sender<ToClient>),
    ClientMsg(ClientId, FromClient),
}
