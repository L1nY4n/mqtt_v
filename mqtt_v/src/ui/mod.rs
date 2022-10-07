use backend::{message::MqttOpts, Backend};

use eframe::{
    egui::{
        menu, Button, CentralPanel, Checkbox, Context, DragValue, Frame, Id, InnerResponse, Label,
        LayerId, Layout, RichText, SidePanel, Slider, TextEdit, TextStyle, Ui, Window,
    },
    emath::{Align, Align2},
    epaint::{
        ahash::{HashMap, HashMapExt},
        Color32,
    },
    CreationContext,
};
use once_cell::sync::Lazy;
use std::{thread};
use tokio::sync::mpsc::{Receiver, Sender};

use self::{
    app_theme::AppTheme,
    client::publish_tab,
    widgets::docking::{self, NodeIndex},
};

mod app_theme;
mod client;
mod widgets;

use client::{chat_tab, client::Client, tree_tab};

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

#[derive(Default)]
struct State {
    show_add: bool,
    mqtt_options: MqttOpts,
    active_client: Option<ClientId>,
}

impl MqttAppUI {
    pub fn new(cc: &CreationContext) -> Self {
        let (front_tx, front_rx) = tokio::sync::mpsc::channel(2);
        let (back_tx, back_rx) = tokio::sync::mpsc::channel(10);
        thread::spawn(move || {
            Backend::new(back_tx, front_rx).init();
        });

        let event_tab = Box::new(chat_tab::ChatTab::new());
        let tree_tab = Box::new(tree_tab::TreeView::new("Tree"));
        let publish_tab = Box::new(publish_tab::PubulishTab::new());
        let mut tree = docking::Tree::new(vec![event_tab, tree_tab]);

        let [_a, _b] = tree.split_below(NodeIndex::root(), 0.75, vec![publish_tab]);

        let clients = HashMap::with_capacity(100);

        let mut app = MqttAppUI {
            front_tx,
            back_rx,
            state: State::default(),
            filter: "".to_owned(),
            clients,
            style: docking::Style::default(),
            tree: Some(tree),
        };
        // load storage
        if let Some(storage) = cc.storage {
            if let Some(client_opts_list) =
                eframe::get_value::<Vec<MqttOpts>>(storage, eframe::APP_KEY)
            {
                if !client_opts_list.is_empty() {
                    client_opts_list.iter().for_each(|opts| {
                        let key = opts.client_id();
                        let client =
                            client::client::create_client(opts.clone(), app.front_tx.clone());
                        app.clients.insert(key, client);
                    });
                    app.state.active_client = Some(client_opts_list[0].client_id())
                }
            }
        }

        app
    }
}

impl eframe::App for MqttAppUI {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.handle_backend_msg(ctx);
        self.render_side_panel(ctx);
        self.render_central_panel(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let connect_opts: Vec<MqttOpts> = self
            .clients.values().map(|v| v.options.clone())
            .collect();
        eframe::set_value(storage, eframe::APP_KEY, &connect_opts);
    }
}

impl MqttAppUI {
    fn handle_backend_msg(&mut self, _ctx: &Context) {
        let mut i = 100;
        while i > 0 {
            i -= 1;

            match self.back_rx.try_recv() {
                Ok(msg) => {
                    match msg {
                        ToFrontend::ClientMsg(client_id, msg) => {
                            if let Some(client) = self.clients.get_mut(&client_id) {
                                client.handle_msg(msg)
                            }
                        }
                        ToFrontend::ClientCreated(client_id, tx) => {
                            if let Some(client) = self.clients.get_mut(&client_id) {
                                println!("created");
                                client.publish_tx = Some(tx)
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
}

// ui
impl MqttAppUI {
    fn render_side_panel(&mut self, ctx: &Context) -> InnerResponse<()> {
        SidePanel::left("left_panel")
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
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.add(Label::new(RichText::new("Connections")));
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // config button
                        let config_btn = ui.add(Button::new(
                            RichText::new("✚")
                                .text_style(TextStyle::Heading)
                                .color(Color32::LIGHT_BLUE),
                        ));

                        if config_btn.clicked() {
                            self.state.show_add = !self.state.show_add;
                        }
                    });
                });

                if self.state.show_add {
                    self.render_add_client(ctx)
                }
                for (k, v) in &mut self.clients {
                    let active = if let Some(s) = &self.state.active_client {
                        s == k
                    } else {
                        false
                    };

                    let on_click = || self.state.active_client = Some(k.to_string());
                    let opts = v.options.clone();
                    let on_dbclick = || {
                        self.state.mqtt_options = opts;
                        self.state.show_add = true;
                    };
                    v.show(ui, k, active, self.front_tx.clone(), on_click, on_dbclick);
                }
            })
    }

    fn render_central_panel(&mut self, ctx: &Context) -> InnerResponse<()> {
        CentralPanel::default().show(ctx, |ui| {
            //  ui.set_height(ui.available_height());
            if let Some(active) = &self.state.active_client {
                if let Some(tree) = &mut self.tree {
                    let client = self.clients.get_mut(active).unwrap();
                    self.style = docking::Style::from_egui(ctx.style().as_ref());

                    let id = Id::new("mqtt_docking");
                    let layer_id = LayerId::background();
                    let max_rect = ui.max_rect();
                    let clip_rect = ui.clip_rect();

                    let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);
                    docking::show(&mut ui, id, &self.style, tree, client)
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("no active selected");
                });
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
            Frame::none().show(ui, |ui| {
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
                        ui.group(|ui| {
                            if ui.selectable_label(v3.credentials, "credentials").clicked() {
                                v3.credentials = !v3.credentials
                            }
                            if v3.credentials {
                                ui.label("username");
                                ui.add(TextEdit::singleline(&mut v3.username));
                                ui.label("password");
                                ui.add(TextEdit::singleline(&mut v3.password).password(true));
                            }
                        });
                        ui.group(|ui| {
                            ui.label("max packet size");
                            ui.label("incoming");
                            ui.add(
                                Slider::new(&mut v3.max_packet_size.0, 5..=u16::MAX)
                                    .suffix(" bytes"),
                            );
                            ui.label("outgoing");
                            ui.add(
                                Slider::new(&mut v3.max_packet_size.1, 5..=u16::MAX)
                                    .suffix(" bytes"),
                            );
                        });
                        ui.end_row();
                        ui.add(Checkbox::new(&mut v3.clean_session, "clean_session"));
                        ui.separator();
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            if ui
                                .button(
                                    RichText::new("➖")
                                        .text_style(TextStyle::Heading)
                                        .color(Color32::RED),
                                )
                                .clicked()
                            {
                                self.clients.remove(&key);
                                self.state.show_add = false;
                                if matches!(&self.state.active_client, Some(k) if  k==&key) {
                                    self.state.active_client = None;
                                }
                            }

                            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                                if ui
                                    .button(
                                        RichText::new("✅")
                                            .text_style(TextStyle::Heading)
                                            .color(Color32::GREEN),
                                    )
                                    .clicked()
                                {
                                    let client = client::client::create_client(
                                        self.state.mqtt_options.clone(),
                                        self.front_tx.clone(),
                                    );
                                    let key1 = key.clone();
                                    self.clients.insert(key, client);
                                    self.state.show_add = false;
                                    if self.state.active_client.is_none() {
                                        self.state.active_client = Some(key1);
                                    }
                                }
                            });
                        });
                    }
                    MqttOpts::V5(_v5) => todo!(),
                };
            });
        });
    }
}
