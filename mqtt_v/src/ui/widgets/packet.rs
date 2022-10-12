use backend::message::{Event, Outgoing, Packet, QoS};
use eframe::{
    egui::{self, style::Margin, Button, CursorIcon, Frame, Label, Layout, RichText, Ui},
    emath::Align,
    epaint::{Color32, FontId, Rounding},
};

pub struct PacketUI {
    event: Event,
}

impl PacketUI {
    pub fn new(pkt: Event) -> Self {
        PacketUI { event: pkt }
    }

    pub fn show(self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            match self.event {
                Event::Incoming(packet) => {
                    let layout = Layout::left_to_right(Align::Center);
                    ui.with_layout(layout, |ui| {
                        ui.set_width(ui.available_width());
                        render_incomming(ui, packet);
                    });
                }
                Event::Outgoing(outgoing) => {
                    let layout = Layout::right_to_left(Align::Center);
                    ui.with_layout(layout, |ui| {
                        ui.set_width(ui.available_width());
                        render_outgoing(ui, outgoing)
                    });
                }
            }
        });
        ui.add_space(4.0);
    }
}

fn render_incomming(ui: &mut Ui, packet: Packet) {
    match packet {
        // Packet::Connect(_) => {}
        // Packet::ConnAck(_) => {}
        Packet::Publish(p) => {
            Frame {
                fill: Color32::BLACK,
                inner_margin: Margin::same(6.0),
                rounding: Rounding {
                    nw: 6.0,
                    ne: 6.0,
                    sw: 6.0,
                    se: 0.0,
                },
                ..Frame::default()
            }
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width() * 0.5);
                ui.vertical(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let tooltip_ui = |ui: &mut Ui| {
                            //  ui.label(RichText::new(p.topic.clone()));
                            ui.label(RichText::new("Click to copy"));
                        };

                        let topic_btn = ui
                            .add(
                                Button::new(RichText::new(p.topic.clone()).color(Color32::KHAKI))
                                    .frame(false),
                            )
                            .on_hover_ui(tooltip_ui);

                        if topic_btn.clicked() {
                            ui.output().copied_text = p.topic;
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.colored_label(
                                Color32::from_rgb(128, 140, 255),
                                match p.qos {
                                    QoS::AtMostOnce => "0",
                                    QoS::AtLeastOnce => "1",
                                    QoS::ExactlyOnce => "2",
                                },
                            );
                        })
                    });
                    ui.add_space(2.0);
                    if let Ok(mut x) = String::from_utf8(p.payload.to_vec()) {
                        ui.add(
                            egui::TextEdit::multiline(&mut x)
                                .font(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                .text_color(Color32::LIGHT_GREEN)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY), // .layouter(&mut layouter),
                        );
                    } else {
                        let mut t = format!("{:x}", p.payload);
                        ui.add(
                            egui::TextEdit::multiline(&mut t)
                                .font(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                // .desired_rows(10)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY), // .layouter(&mut layouter),
                        );
                    }
                });
            });

            //  });
        }
        // Packet::PubAck(_) => {}
        // Packet::PubRec(_) => {}
        // Packet::PubRel(_) => {}
        // Packet::PubComp(_) => todo!(),
        // Packet::Subscribe(_) => todo!(),
        // Packet::SubAck(_) => todo!(),
        // Packet::Unsubscribe(_) => todo!(),
        // Packet::UnsubAck(_) => todo!(),
        Packet::PingReq => {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.vertical(|ui| {
                    ui.set_width(100.0);
                    Frame {
                        fill: Color32::BLACK,
                        inner_margin: Margin::same(6.0),
                        ..Frame::default()
                    }
                    .show(ui, |ui| {
                        ui.colored_label(Color32::GREEN, "ping");
                    });
                });
            });
        }
        Packet::PingResp => {
            ui.horizontal(|ui| {
                ui.set_width(100.0);
                Frame {
                    fill: Color32::BLACK,
                    inner_margin: Margin::same(6.0),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    ui.colored_label(Color32::YELLOW, "pong");
                });
            });
        }
        // Packet::Disconnect => todo!(),
        p => {
            ui.horizontal(|ui| {
                ui.set_width(100.0);
                Frame {
                    fill: Color32::BLACK,
                    inner_margin: Margin::same(6.0),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    ui.colored_label(Color32::GREEN, format!("{:#?}", p));
                });
            });
        }
    }
}

fn render_outgoing(ui: &mut Ui, outgoing: Outgoing) {
    match outgoing {
        Outgoing::Publish(p) => {
            Frame {
                fill: Color32::BLACK,
                inner_margin: Margin::same(6.0),

                ..Frame::default()
            }
            .show(ui, |ui| {
                ui.set_max_width(400.0);
                ui.vertical(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.label(p.to_string());
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.colored_label(
                                Color32::from_rgb(128, 140, 255),
                                format!("{:#?}", p.to_owned()),
                            );
                        })
                    });
                });
            });
        }
        // Outgoing::Subscribe(_) => todo!(),
        // Outgoing::Unsubscribe(_) => todo!(),
        // Outgoing::PubAck(_) => todo!(),
        // Outgoing::PubRec(_) => todo!(),
        // Outgoing::PubRel(_) => todo!(),
        // Outgoing::PubComp(_) => todo!(),
        Outgoing::PingReq => {
            ui.horizontal(|ui| {
                ui.set_width(100.0);
                Frame {
                    fill: Color32::BLACK,
                    inner_margin: Margin::same(6.0),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    ui.colored_label(Color32::GREEN, "ping");
                });
            });
        }
        // Outgoing::PingResp => todo!(),
        // Outgoing::Disconnect => todo!(),
        // Outgoing::AwaitAck(_) => todo!(),
        p => {
            ui.horizontal(|ui| {
                ui.set_width(100.0);
                Frame {
                    fill: Color32::BLACK,
                    inner_margin: Margin::same(6.0),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    ui.colored_label(Color32::GREEN, format!("{:#?}", p));
                });
            });
        }
    }
}
