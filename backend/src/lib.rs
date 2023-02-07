use message::{ToBackend, ToFrontend};
pub mod message;
pub mod mqtt_client;
use tokio::{
    runtime::Builder,
    sync::mpsc::{Receiver, Sender},
};

pub struct Backend {
    back_tx: Sender<ToFrontend>,
    front_rx: Receiver<ToBackend>,
    //    egui_context: eframe::egui::Context,
}

impl Backend {
    pub fn new(back_tx: Sender<ToFrontend>, front_rx: Receiver<ToBackend>) -> Self {
        Self { back_tx, front_rx }
    }

    pub fn init(&mut self) {
        println!("Initializing backend");

        let rt = Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();

        loop {
            if let Some(message) = self.front_rx.blocking_recv() {
                match message {
                    ToBackend::NewClient(opts) => {
                        let (incomming_tx, mut incomming_rx) = tokio::sync::mpsc::channel(100);
                        let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::channel(10);
                        let back_tx = self.back_tx.clone();
                        rt.spawn(async move {
                            match opts {
                                message::MqttOpts::V3(opt) => {
                                    let cli_id = opt.client_id.clone();
                                    let _res = back_tx
                                        .send(ToFrontend::ClientCreated(cli_id, outgoing_tx))
                                        .await;
                                    mqtt_client::new(incomming_tx, outgoing_rx, opt.convert())
                                        .await;
                                }

                                message::MqttOpts::V5(_v5) => {}
                            }
                        });
                        let tx = self.back_tx.clone();

                        rt.spawn(async move {
                            while let Some((client_id, client_msg)) = incomming_rx.recv().await {
                                let _ = tx.try_send(ToFrontend::ClientMsg(client_id, client_msg));
                            }
                        });
                    }

                    ToBackend::Shutdown => {}
                    ToBackend::Startup => todo!(),
                }
            }
        }
    }
}
