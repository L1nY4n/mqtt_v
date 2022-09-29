use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use tokio::{
    sync::mpsc::{Receiver, Sender},
};

pub async fn new(
    sender: Sender<(String, Event)>,
    mut receiver: Receiver<Publish>,
    mqttoptions: MqttOptions,
  
) {
    let client_id = mqttoptions.client_id();
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let client_tx = client.clone();
    std::thread::spawn( move || {
        loop {
            while let Ok(publish) = receiver.try_recv() {
                let res = client_tx.try_publish(
                    publish.topic,
                    publish.qos,
                    publish.retain,
                    publish.payload,
                );
                print!("publihs result {:?}", res);
            }
        }
    });

    loop {
        while let Ok(notification) = eventloop.poll().await {
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
                     println!("Outgoing  {:?}", p);
                }
            }
        }
    }
}
