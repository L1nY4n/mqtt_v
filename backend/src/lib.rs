use message::{ToBackend, ToFrontend};
use tracing::{info};
pub mod message;
pub mod mqtt_client;
use tokio::{
    runtime::Builder,
    sync::mpsc::{Receiver, Sender},
};


pub struct Backend {
    back_tx: Sender<ToFrontend>,
    front_rx: Receiver<ToBackend>,
    egui_context: eframe::egui::Context,
}

impl Backend {
    pub fn new(
        back_tx: Sender<ToFrontend>,
        front_rx: Receiver<ToBackend>,
        egui_context: eframe::egui::Context,
    ) -> Self {
        Self {
            back_tx,
            front_rx,
            egui_context,
        }
    }

    pub fn init(&mut self) {
        info!("Initializing backend");

        let rt = Builder::new_multi_thread()
            .worker_threads(64)
            .enable_all()
            .build()
            .unwrap();

        loop {
            match self.front_rx.try_recv() {
                Ok(message) => {
                    match message {
                        ToBackend::NewClient(opts) => {
                            println!("NewClient");
                            
                            let (incomming_tx, mut incomming_rx) = tokio::sync::mpsc::channel(100);
                            let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::channel(100);
                            let back_tx = self.back_tx.clone();
                            rt.spawn(async move {
                                match opts {
                                    message::MqttOpts::V3(opt) => {
                                        let cli_id = opt.client_id.clone();
                                              let _res =    back_tx.send(ToFrontend::ClientCreated(
                                            cli_id,
                                            outgoing_tx,
                                        )).await;  
                                        mqtt_client::new(incomming_tx, outgoing_rx, opt.convert())
                                            .await;

                               
                                    }
                                  
                                    message::MqttOpts::V5() => {}
                                }
                            });
                            let tx = self.back_tx.clone();

                            rt.spawn(async move {
                                loop {
                                    while let Ok((client_id, client_msg)) = incomming_rx.try_recv() {
                                        let _ = tx.try_send(ToFrontend::ClientMsg(client_id, client_msg));
                                        
                                    }
                                }
                            });
                        }

                        ToBackend::Shutdown => {}
                        ToBackend::Startup => todo!(),
                     
                    }
                 
                }
                Err(_error) => {
                    // As the only reason this will error out is if the channel is closed (sender is dropped) a one time log of the error is enough
                    // LOG_CHANNEL_CLOSED.call_once(|| {
                    //     error!(%error, "There was an error when receiving a message from the frontend:");
                    // });
                }
            };
        }
    }
}
