use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::message::{ClientId, ClientMsg};

pub async fn new(
    sender: Sender<(ClientId, ClientMsg)>,
    mut receiver: Receiver<Publish>,
    mqttoptions: MqttOptions,
) {
    let client_id = mqttoptions.client_id();
    let client_id2 = mqttoptions.client_id();
    let sender2 = sender.clone();
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let client_tx = client.clone();
    std::thread::spawn(move || loop {
       
        while let Ok(publish) = receiver.try_recv() {
            let Publish {
                qos,
                retain,
                topic,
                payload,
                dup: _,
                pkid: _,
            } = publish;
            let result = client_tx.try_publish(topic, qos, retain, payload);
            let _ = sender.try_send((client_id.clone(), ClientMsg::PublishReslt(result)));
        }
    });


    loop {
        while let Ok(notification) = eventloop.poll().await {
            let event_clone =notification.clone();
            let event_msg = (client_id2.clone(), ClientMsg::Event(event_clone));
             let _ = sender2.try_send(event_msg);
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
