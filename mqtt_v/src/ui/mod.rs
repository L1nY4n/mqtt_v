use backend::{
    message::{Event, MqttOpts, OptionsV3},
    Backend,
};
use eframe::{
    egui::{
        menu, style::Margin, Button, CentralPanel, Checkbox, Context, DragValue, Frame, Id,
        InnerResponse, Label, LayerId, Layout, RichText, ScrollArea, SidePanel, Slider, TextEdit,
        TextStyle, TopBottomPanel, Ui, Window,
    },
    emath::{Align, Align2},
    epaint::{
        ahash::{HashMap, HashMapExt},
        Color32,
    },
    CreationContext,
};
use once_cell::sync::Lazy;
use std::thread;
use tokio::sync::mpsc::{Receiver, Sender};

use self::{
    app_theme::AppTheme,
    client::Client,
    widgets::{
        docking::{self, NodeIndex},
        packet::PacketUI,
    },
};

mod app_theme;
mod chat_tab;
mod client;
mod tree_tab;
mod widgets;

use backend::message::{ToBackend, ToFrontend};

static THEME: Lazy<AppTheme> = Lazy::new(AppTheme::default);

type ClientId = String;
// #[derive(Default)]
pub struct MqttAppUI {
    // Data transferring
    front_tx: Sender<ToBackend>,
    back_rx: Receiver<ToFrontend>,
    // state
    state: State,
    filter: String,
    clients: HashMap<ClientId, Client>,

    style: docking::Style,
    tree: Option<docking::Tree<Client>>,
}

struct State {
    show_add: bool,
    mqtt_options: MqttOpts,
    active_client: Option<ClientId>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            show_add: Default::default(),
            active_client: None,
            mqtt_options: MqttOpts::V3(OptionsV3 {
                client_id: "mosquitto".to_string(),
                broker_addr: "test.mosquitto.org".to_string(),
                port: 1883,
                keep_alive: true,
                heatbbeat: 20,
            }),
        }
    }
}

impl MqttAppUI {
    pub fn new(cc: &CreationContext) -> Self {
        let (front_tx, front_rx) = tokio::sync::mpsc::channel(10);
        let (back_tx, back_rx) = tokio::sync::mpsc::channel(10);

        let frame_clone = cc.egui_ctx.clone();
        thread::spawn(move || {
            Backend::new(back_tx, front_rx, frame_clone).init();
        });
        let clients = HashMap::with_capacity(100);
        MqttAppUI {
            front_tx,
            back_rx,
            state: State::default(),
            filter: "".to_owned(),
            clients: clients,
            style: docking::Style::default(),
            tree: None,
        }
    }
}

impl eframe::App for MqttAppUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.handle_backend_msg(ctx);
        self.render_side_panel(ctx);
        self.render_central_panel(ctx);
    }
}

impl MqttAppUI {
    fn handle_backend_msg(&mut self, ctx: &Context) {
        match self.back_rx.try_recv() {
            Ok(msg) => {
                match msg {
                    ToFrontend::Packet(client_id, event) => {
                        if let Some(client) = self.clients.get_mut(&client_id) {
                            client.packets.push(event);
                            client.recv += 1;
                        }
                    }
                }
                //  ctx.request_repaint();
            }
            Err(err) => {
                let _ = err;
            }
        }
    }
}

// ui
impl MqttAppUI {
    fn render_side_panel(&mut self, ctx: &Context) -> InnerResponse<()> {
        SidePanel::left("options_panel")
            .frame(Frame {
                inner_margin: THEME.margin.frame_margin,
                fill: THEME.colors.dark_gray,
                ..Frame::default()
            })
            .resizable(false)
            .default_width(200.)
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = THEME.spacing.widget_spacing;
                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // config button
                        let config_btn = ui.add(Button::new(
                            RichText::new("✚")
                                .text_style(TextStyle::Heading)
                                .color(Color32::LIGHT_BLUE),
                        ));

                        if config_btn.clicked() {
                            self.state.show_add = !self.state.show_add;
                            let scene = Box::new(chat_tab::ChatView::new("Scene"));
                            let node_tree = Box::new(tree_tab::TreeView::new("Scene2"));
                            let tree = docking::Tree::new(vec![scene, node_tree]);

                            self.tree = Some(tree)
                        }
                    });
                });

                if self.state.show_add {
                    self.render_add_client(ctx)
                }
                for (k, v) in &self.clients {
                    let active = if let Some(s) = &self.state.active_client {
                        s == k
                    } else {
                        false
                    };
                    v.show(ui, k, active, || {
                        self.state.active_client = Some(k.to_string())
                    });
                    ui.add_space(3.0);
                }
            })
    }

    fn render_central_panel(&mut self, ctx: &Context) -> InnerResponse<()> {
        CentralPanel::default().show(ctx, |ui| {
            //  ui.set_height(ui.available_height());
            if let Some(active) = &self.state.active_client {
                if let Some(tree) = &mut self.tree {
                    let mut client = self.clients.get_mut(active).unwrap();
                    self.style = docking::Style::from_egui(ctx.style().as_ref());

                    let id = Id::new("some hashable string");
                    let layer_id = LayerId::background();
                    let max_rect = ui.max_rect();
                    let clip_rect = ui.clip_rect();

                    let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);
                    docking::show(&mut ui, id, &self.style, tree, &mut client)
                }

                // ScrollArea::vertical()
                //     .stick_to_bottom(true)
                //    // .max_width(ui.available_width())
                //     .show(ui, |ui| {
                //         for pkt in &client.packets {
                //             PacketUI::new(pkt.clone()).show(ui)
                //         }
                //     });
            } else {
                ui.label("no active selected");
            }
        })
    }

    fn render_add_client(&mut self, ctx: &Context) {
        let window = Window::new("add client")
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .scroll2([false; 2])
            .vscroll(false)
            .anchor(Align2::CENTER_CENTER, [0.0, -60.0]);
        window.show(ctx, |ui| {
            Frame::default().show(ui, |ui| {
                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let close_btn = ui.add(Button::new(
                            RichText::new("❌")
                                .text_style(TextStyle::Body)
                                .color(Color32::LIGHT_RED),
                        ));
                        if close_btn.clicked() {
                            self.state.show_add = false;
                        }
                    });
                    //    ui.add_sized(Vec2::new(120.0, ui.available_height()), Label::new("add client"));
                });
                ui.separator();

                match &mut self.state.mqtt_options {
                    MqttOpts::V3(v3) => {
                        let key = v3.client_id.clone();
                        ui.horizontal(|ui| {
                            ui.label("client_id");
                            let client_id = TextEdit::singleline(&mut v3.client_id)
                                .hint_text(RichText::new("client_id").color(THEME.colors.gray));

                            ui.add(client_id)
                        });

                        ui.horizontal(|ui| {
                            let addr = TextEdit::singleline(&mut v3.broker_addr).hint_text(
                                RichText::new("broker_address").color(THEME.colors.gray),
                            );
                            let port_widget = DragValue::new(&mut v3.port).clamp_range(0..=65535);

                            ui.label("broker_address:");
                            ui.add(addr);
                            ui.separator();
                            ui.add(port_widget);
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.add(Checkbox::new(&mut v3.keep_alive, "keep_alive"));

                            if v3.keep_alive {
                                ui.separator();
                                ui.add(Slider::new(&mut v3.heatbbeat, 5..=30).suffix("s"));
                            }
                        });
                        ui.separator();
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if ui
                                .button(
                                    RichText::new("✅")
                                        .text_style(TextStyle::Heading)
                                        .color(Color32::GREEN),
                                )
                                .clicked()
                            {
                                let opts_c = self.state.mqtt_options.clone();
                                let _ = self.front_tx.try_send(ToBackend::NewClient(opts_c));

                                self.clients.insert(
                                    key,
                                    Client {
                                        options: self.state.mqtt_options.clone(),
                                        packets: vec![],
                                        recv: 0,
                                    },
                                );
                                self.state.show_add = false;
                            }
                        });
                    }
                    MqttOpts::V5() => todo!(),
                };
            });
        });
    }
}
