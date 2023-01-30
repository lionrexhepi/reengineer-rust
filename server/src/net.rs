use std::{ sync::mpsc::{ Receiver, Sender, channel, SendError }, thread::{ self, JoinHandle } };

use concurrent_queue::ConcurrentQueue;
use log::info;
use shared::net::{ Packet, NetworkHandler };

pub struct ServerNetworkHandler {
    outgoing_sender: Sender<Packet>,
    incoming_receiver: Receiver<Packet>,
    send_thread: Option<JoinHandle<()>>,
    receive_thread: Option<JoinHandle<()>>,
}

impl ServerNetworkHandler {
    pub fn init() -> Self {
        let (s1, r1) = channel();
        let (s2, r2) = channel();

        (Self {
            incoming_receiver: r2,
            outgoing_sender: s1,
            send_thread: None,
            receive_thread: None,
        }).init_thread(s2, r1)
    }

    fn init_thread(mut self, sx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        self.send_thread = Some(
            thread::spawn(move || {
                loop {
                    let outgoing = rx.recv();

                    if outgoing.is_err() {
                        break;
                    } else {
                        todo!(); //Send packets
                    }
                }
            })
        );

        self.receive_thread = Some(
            thread::spawn(move || {
                loop {
                    let incoming = Packet {
                        direction: shared::net::PacketDirection::Serverbound,
                        data: shared::net::PacketData::Ping,
                    };

                    if sx.send(incoming).is_err() {
                        
                        break;
                    }
                }
            })
        );

        self
    }
}

impl NetworkHandler for ServerNetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()> {
        match self.outgoing_sender.send(packet) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn retrieve_incoming(&self) -> Vec<Packet> {
        self.incoming_receiver.try_iter().collect()
    }
}