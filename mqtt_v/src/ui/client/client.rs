use backend::message::{Event, MqttOpts, ToBackend,Publish, ToClient, Topic,QoS};
use eframe::{
    egui::{style::Margin, Frame, Label, Layout, RichText, Ui},
    emath::Align,
    epaint::Color32,
};
use tokio::sync::mpsc::Sender;

use crate::ui::THEME;
pub struct Client {
    pub options: MqttOpts,
    pub packets: Vec<Event>,
    pub publish_tx: Option<Sender<ToClient>>,
    pub subscriptions : Vec<(Topic,QoS)>,
    pub recv: u32,
}

pub fn  create_client(options: MqttOpts,tx:  Sender<ToBackend>) -> Client {
    
    let _ = tx.try_send(ToBackend::NewClient(options.clone()));
    Client{
        options,
        packets: vec![],
        publish_tx: None,
        subscriptions: vec![],
        recv: 0,
    }
}

impl Client {
    pub fn show(&
        self, ui: &mut Ui, client_id: &str, active: bool, on_click: impl FnOnce()) {
        let (title_color, bg) = if active {
            (Color32::LIGHT_BLUE, Color32::BLACK)
        } else {
            (Color32::WHITE, THEME.colors.darker_gray)
        };
        let client_frame = Frame {
            fill: bg,
            inner_margin: Margin::same(6.0),
            rounding: THEME.rounding.big,
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let cli_id = Label::new(RichText::new(client_id).color(title_color));

                ui.add(cli_id);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .button(RichText::new("⚙").color(Color32::LIGHT_BLUE))
                        .clicked()
                    {
                        on_click();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("recv: ");
                ui.colored_label(Color32::YELLOW, self.recv.to_string());
                if ui.button(RichText::new("🚫").color(Color32::LIGHT_RED)).clicked(){
                    if let Some(tx) = & self.publish_tx {
                       // tx.try_send(ToClient::Disconnect(("#".to_owned(),QoS::AtMostOnce)));
                       tx.try_send(ToClient::Disconnect);
                    }
                   
                }
            });
        });
        if client_frame.response.clicked() {
            println!("frame click")
        }
    }
}

