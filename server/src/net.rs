use std::{
    sync::mpsc::{ Sender, Receiver, channel, TrySendError },
    thread::{ spawn },
    net::{ TcpListener, TcpStream },
    io::BufWriter,
};

use anyhow::{ bail, anyhow };
use log::error;
use metrohash::MetroHashMap;
use shared::net::{
    packet::{ Packet, ClientId, PacketSource, PacketDirection },
    NetworkHandler,
};

pub struct ServerNetworkHandler {
    outgoing_sender: Sender<Packet>,
    incoming_receiver: Receiver<Packet>,
    terminate: Sender<()>,
}

impl ServerNetworkHandler {
    fn receive_client_packets(
        client: &ClientId,
        stream: &mut TcpStream,
        send: &Sender<Packet>
    ) -> anyhow::Result<()> {
        while
            let Some(packet) = Packet::try_from_stream(
                stream,
                PacketSource::Client(client.clone())
            )?
        {
            send.send(packet)?;
        }

        Ok(())
    }

    fn dispatch_packet(
        packet: Packet,
        clients: &MetroHashMap<ClientId, TcpStream>
    ) -> anyhow::Result<()> {
        let mut stream = if let PacketDirection::ToClient(id) = &packet.direction {
            clients.get(id).ok_or_else(|| anyhow!("Client with id {:?} doesn't exist!", id))?
        } else {
            bail!(
                "Invalid packet direction: {:?}, expected PacketDirection::ToClient(ClientId)",
                packet.direction
            )
        };

        let mut writer = BufWriter::new(&mut stream);

        packet.write_to_buffer(&mut writer)?;

        Ok(())
    }

    pub fn init() -> Self {
        let (send_out, receive_out) = channel::<Packet>();
        let (send_in, receive_in) = channel();
        let (terminate_sender, terminate_receiver) = channel();

        let listener = TcpListener::bind("127.0.0.1:19354").unwrap();

        listener.set_nonblocking(true).unwrap();

        let handle_thread = spawn(move || {
            let mut clients = MetroHashMap::default();

            'main_loop: loop {
                if terminate_receiver.try_recv().is_ok() {
                    break;
                }

                let client = listener.accept();
                if let Ok((stream, _)) = client {
                    clients.insert(ClientId::new(), stream);
                }

                for (id, stream) in clients.iter_mut() {
                    if let Err(error) = Self::receive_client_packets(id, stream, &send_in) {
                        error!("Failed to receive Packet(s) sent by client {:?}: {}", id, error);

                        if error.is::<TrySendError<Packet>>() {
                            break 'main_loop;
                        }
                    }
                }

                while let Ok(packet) = receive_out.try_recv() {
                    if let Err(error) = Self::dispatch_packet(packet, &clients) {
                        error!("Failed to dispatch packet: {}", error)
                    }
                }
            }

            for (_, stream) in clients.into_iter() {
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
        });

        Self {
            outgoing_sender: send_out,
            incoming_receiver: receive_in,
            terminate: terminate_sender,
        }
    }
}

impl NetworkHandler for ServerNetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()> {
        self.outgoing_sender.send(packet)?;
        Ok(())
    }

    fn retrieve_incoming(&mut self) -> Vec<Packet> {
        let mut result = Vec::new();

        while let Ok(packet) = self.incoming_receiver.try_recv() {
            result.push(packet);
        }

        result
    }

    fn close_all(self) {
        self.terminate.send(()).unwrap();
    }
}