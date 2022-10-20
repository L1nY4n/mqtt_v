use backend::message::{Event, QoS};
use eframe::{
    egui::{
        self, style::Margin, Context, Id, InnerResponse, Layout, RichText, ScrollArea, TextEdit, Ui,
    },
    emath::Align,
    epaint::{text, Color32, FontId},
};

use crate::ui::widgets::{docking, packet::PacketUI};

use super::client::{Client, Subcribe};

pub struct ChatTab {
    filter: Filter,
    subcribe: Subcribe,
}

#[derive(Default)]
struct Filter {
    pub direct: Direct,
    pub topic: String,
    pub content: String,
}

#[derive(Default)]
struct Direct {
    published: bool,
    received: bool,
}
impl Filter {
    fn filter(&self, _event: &Event) -> bool {
        true
    }
}

impl ChatTab {
    pub fn new() -> Self {
        Self {
            filter: Default::default(),
            subcribe: Subcribe {
                topic: "#".to_owned(),
                qos: QoS::AtMostOnce,
                color: Color32::GREEN,
            },
        }
    }
}

impl docking::Tab<Client> for ChatTab {
    fn title(&self) -> &str {
        "📺   Event"
    }

    fn ui(&mut self, ui: &mut egui::Ui, client: &mut Client) {
        ui.push_id("chat_tab", |ui| {
            egui::Frame::default()
                .outer_margin(Margin::symmetric(2., 6.))
                .inner_margin(4.)
                .rounding(4.)
                .fill(Color32::BLACK)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            let frame = egui::Frame::default();
                            frame.fill(Color32::BLACK).show(ui, |ui| {
                                let icon = text::LayoutJob::simple_singleline(
                                    "📝".into(),
                                    FontId::proportional(16.0),
                                    Color32::GREEN,
                                );
                                let InnerResponse { inner: _, response } =
                                    ui.menu_button(icon, |ui| {
                                        egui::Frame::default()
                                            .inner_margin(Margin::same(4.0))
                                            .show(ui, |ui| {
                                                let scroll = ScrollArea::vertical();

                                                scroll.id_source("subcribtions").show(ui, |ui| {
                                                    for Subcribe { topic, qos, color } in
                                                        client.subscriptions.clone()
                                                    {
                                                        egui::Frame::default()
                                                            .outer_margin(Margin::same(4.0))
                                                            .show(ui, |ui| {
                                                                ui.horizontal(|ui| {
                                                                    let topic_c = topic.clone();
                                                                    let color_c = color;
                                                                    ui.label(
                                                                        RichText::new(topic)
                                                                            .color(color_c),
                                                                    );
                                                                    ui.label(match qos {
                                                                        QoS::AtMostOnce => "0",
                                                                        QoS::AtLeastOnce => "1",
                                                                        QoS::ExactlyOnce => "2",
                                                                    });

                                                                    if ui.button("ｘ").clicked() {
                                                                        client.unsubscribe(topic_c)
                                                                    }
                                                                });
                                                            });
                                                    }
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.color_edit_button_srgba(
                                                        &mut self.subcribe.color,
                                                    );
                                                    ui.separator();
                                                    ui.add(TextEdit::singleline(
                                                        &mut self.subcribe.topic,
                                                    ));
                                                    ui.separator();
                                                    ui.selectable_value(
                                                        &mut self.subcribe.qos,
                                                        QoS::AtMostOnce,
                                                        "0",
                                                    );
                                                    ui.selectable_value(
                                                        &mut self.subcribe.qos,
                                                        QoS::AtLeastOnce,
                                                        "1",
                                                    );
                                                    ui.selectable_value(
                                                        &mut self.subcribe.qos,
                                                        QoS::ExactlyOnce,
                                                        "2",
                                                    );
                                                    ui.separator();

                                                    if ui.button("subcribe").clicked() {
                                                        let subcribe = self.subcribe.clone();
                                                        client.subscribe(subcribe);
                                                    }
                                                });
                                            });
                                    });
                                response.on_hover_cursor(egui::CursorIcon::PointingHand);
                            });
                        });

                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            if ui.button(RichText::new("🗑").color(Color32::RED)).clicked() {
                                client.packets.clear();
                            };
                            if ui
                                .selectable_label(self.filter.direct.published, "published")
                                .clicked()
                            {
                                self.filter.direct.published = !self.filter.direct.published
                            };
                            ui.separator();
                            if ui
                                .selectable_label(self.filter.direct.received, "received")
                                .clicked()
                            {
                                self.filter.direct.received = !self.filter.direct.received
                            };
                            ui.separator();
                            if ui.button("ｘ").clicked() {
                                self.filter.topic.clear();
                            }
                            ui.add(
                                TextEdit::singleline(&mut self.filter.topic).desired_width(120.0),
                            );

                            ui.colored_label(Color32::YELLOW, "🔭");
                        });
                    });
                });
            ui.add_space(12.);
            ScrollArea::vertical()
                .stick_to_bottom(true)
                // .max_width(ui.available_width())
                .show(ui, |ui| {
                    for pkt in &client.packets {
                        PacketUI::new(pkt.clone()).show(ui, &client.subscriptions)
                    }
                });
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct State {
    open: bool,
}

impl State {
    pub fn load(ctx: &Context, id: Id) -> Self {
        ctx.data().get_temp(id).unwrap_or(Self { open: true })
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data().insert_temp(id, self);
    }

    pub fn toggle(&mut self, ui: &Ui) {
        self.open = !self.open;
        ui.ctx().request_repaint();
    }
}
