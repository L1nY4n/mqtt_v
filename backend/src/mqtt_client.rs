
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::message::{ClientId, FromClient, ToClient};

pub async fn new(
    sender: Sender<(ClientId, FromClient)>,
    mut receiver: Receiver<ToClient>,
    mqttoptions: MqttOptions,
) {
    let client_id = mqttoptions.client_id();
    let client_id2 = mqttoptions.client_id();
    let sender2 = sender.clone();
   
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);
    let client_tx = client.clone();
   
    std::thread::spawn(move || loop {
        while let Ok(msg) = receiver.try_recv() {
            match msg {
                ToClient::Publish(publish) => {
                    let Publish {
                        qos,
                        retain,
                        topic,
                        payload,
                        dup: _,
                        pkid: _,
                    } = publish;
                    let result = client_tx.try_publish(topic, qos, retain, payload);
                    let _ = sender.try_send((client_id.clone(), FromClient::PublishReslt(result)));
                },
                ToClient::Subscribe((topic,qos)) => {
                   let _ =  client_tx.try_subscribe(topic, qos);
                },
                ToClient::Connect =>{
                 // todo stop eventloop poll
                },
                ToClient::Disconnect => {
                  let _ =   client_tx.try_disconnect();
                },
               
            }
        
        }
    });


    loop {
       
        while let Ok(notification) = eventloop.poll().await {
            let event_clone =notification.clone();
            let event_msg = (client_id2.clone(), FromClient::Event(event_clone));
             let _ = sender2.try_send(event_msg);
            match notification {
                Event::Incoming(packet) => {
                    //  println!("Incoming  {:?}", packet);
                    match packet {
                        Packet::ConnAck(_) => {
                           
                        }
                        _ => {
                          //  println!("inComming  {:?}", packet);
                        }
                    }
                }
                Event::Outgoing(p) => {
                    println!("Outgoing  {:?}", p);
                }
            }
        }
    }
}
