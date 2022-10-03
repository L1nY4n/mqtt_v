use backend::message::{
    Event, FromClient, MqttOpts, Outgoing, Packet, QoS, ToBackend, ToClient, Topic,
};
use eframe::{
    egui::{style::Margin, CursorIcon, Frame, Label, Layout, RichText, Sense, Ui},
    emath::Align,
    epaint::Color32,
};
use tokio::sync::mpsc::Sender;

use crate::ui::{widgets::status_led::StatusLed, THEME};

pub struct Client {
    pub connected: bool,
    pub options: MqttOpts,
    pub packets: Vec<Event>,
    pub publish_tx: Option<Sender<ToClient>>,
    pub subscriptions: Vec<Subcribe>,
    pub recv: u32,
}

#[derive(Clone)]
pub struct Subcribe {
    pub topic: String,
    pub qos: QoS,
    pub color: Color32,
}

pub fn create_client(options: MqttOpts, tx: Sender<ToBackend>) -> Client {
    let _ = tx.try_send(ToBackend::NewClient(options.clone()));
    Client {
        connected: false,
        options,
        packets: vec![],
        publish_tx: None,
        subscriptions: vec![],
        recv: 0,
    }
}

impl Client {
    pub fn handle_msg(&mut self, msg: FromClient) {
        match msg {
            FromClient::Event(event) => {
                let event_c = event.clone();
                match event {
                    Event::Incoming(income) => match income {
                        Packet::Connect(_) => {}
                        Packet::ConnAck(_) => {
                            self.connected = true;
                        }
                        Packet::Publish(_p) => {
                            self.recv += 1;
                        }
                        Packet::PubAck(_) => {}
                        Packet::PubRec(_) => {}
                        Packet::PubRel(_) => {}
                        Packet::PubComp(_) => {}
                        Packet::Subscribe(_) => {}
                        Packet::SubAck(_) => {}
                        Packet::Unsubscribe(_) => {}
                        Packet::UnsubAck(_) => {}
                        Packet::PingReq => {}
                        Packet::PingResp => {}
                        Packet::Disconnect => self.connected = false,
                    },
                    Event::Outgoing(outgoing) => match outgoing {
                        Outgoing::Publish(_) => {}
                        Outgoing::Subscribe(_) => {}
                        Outgoing::Unsubscribe(_) => {}
                        Outgoing::PubAck(_) => {}
                        Outgoing::PubRec(_) => {}
                        Outgoing::PubRel(_) => {}
                        Outgoing::PubComp(_) => {}
                        Outgoing::PingReq => {}
                        Outgoing::PingResp => {}
                        Outgoing::Disconnect => {}
                        Outgoing::AwaitAck(_) => {}
                    },
                }

                self.packets.push(event_c);
            }
            FromClient::PublishReslt(result) => {
                println!("pub result: {:#?}", result);
            }
            FromClient::Disconnected => self.connected = false,
        }
    }

    pub fn subscribe(&mut self, subcribe: Subcribe) {
        if let Some(tx) = &self.publish_tx {
            let _ = tx.try_send(ToClient::Subscribe((subcribe.topic.clone(), subcribe.qos)));
        }

        self.subscriptions.push(subcribe)
    }

    pub fn unsubscribe(&mut self, topic: Topic) {
        if let Some(tx) = &self.publish_tx {
            if let Ok(()) = tx.try_send(ToClient::Unsubscribe(topic.to_string())) {
                self.subscriptions.retain(|x| x.topic != topic);
            }
        }
    }
}

impl Client {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        client_id: &str,
        active: bool,
        front_tx: Sender<ToBackend>,
        on_click: impl FnOnce(),
    ) {
        let (title_color, bg) = if active {
            (Color32::LIGHT_BLUE, Color32::BLACK)
        } else {
            (Color32::WHITE, THEME.colors.darker_gray)
        };
        let client_frame = Frame {
            fill: bg,
            inner_margin: Margin::same(5.0),
            rounding: THEME.rounding.big,
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.add(StatusLed::new(&self.connected));
                let cli_id = Label::new(RichText::new(client_id).color(title_color));

                ui.add(cli_id);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if self.connected {
                        let disconn_btn = ui.button(RichText::new("🚫").color(Color32::LIGHT_RED));
                        if disconn_btn.clicked() {
                            if let Some(tx) = &self.publish_tx {
                                let _ = tx.try_send(ToClient::Disconnect);
                            }
                        }
                    } else {
                        let conn_btn = ui.button(RichText::new("⚡").color(Color32::YELLOW));
                        if conn_btn.clicked() {
                            let _ = front_tx.try_send(ToBackend::NewClient(self.options.clone()));
                        }
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("recv: ");
                ui.colored_label(Color32::YELLOW, self.recv.to_string());
            });
        });
        let response = client_frame.response;
        if response
            .on_hover_cursor(CursorIcon::PointingHand)
            .interact(Sense::click())
            .clicked()
        {
            on_click();
        }
    }
}
