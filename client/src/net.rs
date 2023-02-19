use std::{
    sync::mpsc::{ channel, Sender, Receiver },
    net::{ TcpStream },
    io::{ BufWriter, Write },
    thread::{ JoinHandle, spawn },
};

use log::error;
use shared::net::{ packet::{ Packet, PacketSource }, NetworkHandler };

pub struct ClientNetworkHandler {
    outgoing_sender: Sender<Packet>,
    incoming_receiver: Receiver<Packet>,
    server_thread: JoinHandle<()>,
}

impl ClientNetworkHandler {
    fn handle_incoming(stream: &mut TcpStream, send: &Sender<Packet>) -> anyhow::Result<()> {
        while let Some(packet) = Packet::try_from_stream(stream, PacketSource::Server)? {
            send.send(packet)?;
        }

        Ok(())
    }

    fn handle_outgoing(stream: &mut TcpStream, receive: &Receiver<Packet>) -> anyhow::Result<()> {
        let mut writer = BufWriter::new(stream);
        while let Ok(packet) = receive.try_recv() {
            packet.write_to_buffer(&mut writer)?;
        }
        writer.flush()?;

        Ok(())
    }

    pub fn for_server(address: &str) -> Self {
        let (send_out, receive_out) = channel::<Packet>();
        let (send_in, receive_in) = channel();

        let mut stream = TcpStream::connect(address).unwrap();
        stream.set_nonblocking(true).unwrap();

        let handle_thread = spawn(move || {
            loop {
                if let Err(error) = Self::handle_incoming(&mut stream, &send_in) {
                    error!("Failed to receive incoming Packet(s): {}", error);
                } else if let Err(error) = Self::handle_outgoing(&mut stream, &receive_out) {
                    error!("Failed to dispatch outgoing Packet(s): {}", error);
                }
            }
        });

        Self {
            server_thread: handle_thread,
            outgoing_sender: send_out,
            incoming_receiver: receive_in,
        }
    }
}

pub struct FakeNetworkHandler {
    outgoing: Sender<Packet>,
    incoming: Receiver<Packet>,
}

impl NetworkHandler for FakeNetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()> {
        match self.outgoing.send(packet) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn retrieve_incoming(&mut self) -> Vec<Packet> {
        let mut result = Vec::new();
        while let Ok(packet) = self.incoming.try_recv() {
            result.push(packet);
        }
        result
    }

    fn close_all(self) {}
}

impl FakeNetworkHandler {
    pub fn new_pair() -> (Self, Self) {
        let (to_client, from_server) = channel();
        let (to_server, from_client) = channel();

        (
            Self { incoming: from_server, outgoing: to_server },
            Self { incoming: from_client, outgoing: to_client },
        )
    }
}