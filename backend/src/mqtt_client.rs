use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn new(
    sender: Sender<(String, Event)>,
    mut receiver: Receiver<Publish>,
    mqttoptions: MqttOptions,
) {
    let client_id = mqttoptions.client_id();
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    std::thread::spawn( move|| {
        while let Ok(_) = receiver.try_recv() {
            println!("to publish");
        }
    }).join().unwrap();     
    loop {
         while  let  Ok(notification) = eventloop.poll().await {
            let _ = sender.try_send((client_id.clone(), notification.clone()));
            match notification {
                Event::Incoming(packet) => {
                  //  println!("Incoming  {:?}", packet);
                    match packet {
                        Packet::ConnAck(_) => {
                            client.subscribe("#", QoS::AtMostOnce).await.unwrap();
                        }
                        _ => {}
                    }
                }
                Event::Outgoing(p) => {
                  //  println!("Outgoing  {:?}", p);
                },

            }
         }

    }
}
