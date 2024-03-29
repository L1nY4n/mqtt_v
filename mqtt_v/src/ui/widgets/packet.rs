use backend::message::{Event, Outgoing, Packet, QoS};
use chrono::{DateTime, Local};
use eframe::{
    egui::{self, style::Margin, Frame, Layout, RichText, Sense, Ui},
    emath::Align,
    epaint::{Color32, Rounding, Stroke, Vec2},
};

use crate::ui::client::client::{ClientPacket, PacketData, Subcribe};

pub struct PacketUI {
    event: Event,
}

impl PacketUI {
    pub fn new(pkt: Event) -> Self {
        PacketUI { event: pkt }
    }

    pub fn show(ui: &mut Ui, pkt: &ClientPacket, subs: &Vec<Subcribe>) {
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            match &pkt.data {
                crate::ui::client::client::PacketData::Event(event) => match event {
                    Event::Incoming(incoming) => {
                        let layout = Layout::left_to_right(Align::Center);
                        ui.with_layout(layout, |ui| {
                            ui.set_width(ui.available_width());
                            render_incomming(ui, incoming, &pkt.time, subs);
                        });
                    }
                    Event::Outgoing(outgoing) => {
                        let layout = Layout::right_to_left(Align::Center);
                        ui.with_layout(layout, |ui| {
                            ui.set_width(ui.available_width());
                            render_outgoing(ui, outgoing)
                        });
                    }
                },
                PacketData::PublishPacket(p) => {
                    let layout = Layout::right_to_left(Align::Center);
                    ui.with_layout(layout, |ui| {
                        ui.set_width(ui.available_width());
                        Frame {
                            fill: Color32::BLACK,
                            inner_margin: Margin::same(6.0),
                            rounding: Rounding::same(6.0),
                            ..Frame::default()
                        }
                        .show(ui, |ui| {
                            ui.set_max_width(ui.available_width() * 0.5);

                            ui.vertical(|ui| {
                                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                    ui.scope(|ui| {
                                        ui.spacing_mut().item_spacing = Vec2::new(2.0, 1.0);
                                        ui.horizontal_centered(|ui| {
                                            ui.label(RichText::new("⬈").color(Color32::GREEN));
                                        });
                                    });

                                    ui.scope(|ui| {
                                        ui.spacing_mut().item_spacing.x = 1.0;
                                        let mut i = 0;
                                        for x in p.topic.split('/') {
                                            if i == 0 {
                                                if !x.is_empty() {
                                                    ui.label(
                                                        RichText::new(x).color(Color32::KHAKI),
                                                    );
                                                }
                                            } else {
                                                ui.label(RichText::new("/").color(Color32::WHITE));
                                                ui.label(RichText::new(x).color(Color32::KHAKI));
                                            }
                                            i += 1;
                                        }
                                    });

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
                                            .text_color(Color32::LIGHT_BLUE)
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
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}", pkt.time.format("%Y-%m-%d %H:%M")))
                                })
                            });
                        });
                    });
                }
            }
        });
        ui.add_space(4.0);
    }
}

fn render_incomming(ui: &mut Ui, packet: &Packet, time: &DateTime<Local>, subs: &Vec<Subcribe>) {
    match packet {
        // Packet::Connect(_) => {}
        // Packet::ConnAck(_) => {}
        Packet::Publish(p) => {
            Frame {
                fill: Color32::BLACK,
                inner_margin: Margin::same(6.0),
                rounding: Rounding::same(6.0),
                ..Frame::default()
            }
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width() * 0.6);

                ui.vertical(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let mut sub_matches = vec![];
                        for s in subs.iter() {
                            if s.matches(&p.topic) {
                                sub_matches.push((s.topic.clone(), s.color));
                            }
                        }
                        ui.scope(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(2.0, 1.0);
                            ui.horizontal_centered(|ui| {
                                for (topic, color) in &sub_matches {
                                    let desired_size = Vec2::new(4.0, 14.0);
                                    let (rect, resp) =
                                        ui.allocate_exact_size(desired_size, Sense::hover());
                                    ui.painter().rect(
                                        rect,
                                        ui.style().visuals.noninteractive().rounding,
                                        *color,
                                        Stroke::none(),
                                    );
                                    resp.on_hover_text(topic);
                                }
                            });
                        });

                        let tooltip_ui = |ui: &mut Ui| {
                            for (topic, color) in &sub_matches {
                                ui.label(RichText::new(topic).color(*color));
                            }

                            ui.label(RichText::new("click to copy"));
                        };
                        let response = ui
                            .scope(|ui| {
                                ui.spacing_mut().item_spacing.x = 1.0;
                                let mut i = 0;
                                for x in p.topic.split('/') {
                                    if i == 0 {
                                        if !x.is_empty() {
                                            ui.label(RichText::new(x).color(Color32::KHAKI));
                                        }
                                    } else {
                                        ui.label(RichText::new("/").color(Color32::WHITE));
                                        ui.label(RichText::new(x).color(Color32::KHAKI));
                                    }
                                    i += 1;
                                }
                            })
                            .response
                            .on_hover_ui_at_pointer(tooltip_ui)
                            .interact(Sense::click());
                        if response.clicked() {
                            ui.output().copied_text = p.topic.clone();
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
                    ui.add_space(2.0);
                    ui.horizontal(|ui| ui.label(format!("{}", time.format("%Y-%m-%d %H:%M"))))
                });
            });

            //  });
        }
        Packet::PubAck(_) => {}
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

fn render_outgoing(ui: &mut Ui, outgoing: &Outgoing) {
    match outgoing {
        Outgoing::Publish(_p) => {
            // Frame {
            //     fill: Color32::BLACK,
            //     inner_margin: Margin::same(6.0),

            //     ..Frame::default()
            // }
            // .show(ui, |ui| {
            //     ui.set_max_width(400.0);
            //     ui.vertical(|ui| {
            //         ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            //             ui.label(p.to_string());
            //             ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            //                 ui.colored_label(
            //                     Color32::from_rgb(128, 140, 255),
            //                     format!("{:#?}", p.to_owned()),
            //                 );
            //             })
            //         });
            //     });
            // });
        }
        // Outgoing::Subscribe(_) => todo!(),
        // Outgoing::Unsubscribe(_) => todo!(),
        Outgoing::PubAck(_) => {}
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
