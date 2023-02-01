use core::panic;

use log::error;
use metrohash::MetroHashMap;
use shared::net::{ PacketData, Packet, NetworkHandler, ClientId, PacketDirection };
use tokio::{
    spawn,
    sync::mpsc::{ unbounded_channel, UnboundedSender, UnboundedReceiver },
    task::JoinHandle,
    net::{ TcpListener },
    io::{ AsyncReadExt, AsyncWriteExt, BufWriter },
};

pub struct ServerNetworkHandler {
    outgoing_sender: UnboundedSender<Packet>,
    incoming_receiver: UnboundedReceiver<Packet>,
    server_thread: JoinHandle<()>,
}

impl ServerNetworkHandler {
    pub async fn init() -> Self {
        let (send_out, mut receive_out) = unbounded_channel::<Packet>();
        let (send_in, receive_in) = unbounded_channel();

        let listener = TcpListener::bind("127.0.0.1:19354").await.unwrap();

        let handle_thread = spawn(async move {
            let mut clients = MetroHashMap::default();

            'main_loop: loop {
                let client = listener.accept().await;
                if let Ok((stream, _)) = client {
                    clients.insert(ClientId::new(), stream);
                }

                for (id, stream) in clients.iter_mut() {
                    let mut buffer = Vec::new();
                    if let Err(e) = stream.read_to_end(&mut buffer).await {
                        error!("Error loading Packet data: {}", e);
                    }
                    let packet_data = PacketData::from_bytes(&buffer);

                    match packet_data {
                        Ok(packet_data) => {
                            let packet = Packet {
                                data: packet_data,
                                direction: PacketDirection::FromClient(id.clone()),
                            };

                            if send_in.send(packet).is_err() {
                                break 'main_loop;
                            }
                        }

                        Err(e) => error!("Failed to parse Packet: {}", e),
                    }
                }

                while let Ok(packet) = receive_out.try_recv() {
                    let target = if let PacketDirection::ToClient(id) = packet.direction {
                        id
                    } else {
                        error!(
                            "Packet with invalid direction '{:?}': {:?}",
                            packet.direction,
                            packet
                        );
                        panic!("Packet has invalid direction : {:?}", packet.direction);
                    };

                    if let Some(stream) = clients.get_mut(&target) {
                        let mut writer = BufWriter::new(&mut *stream);
                        if let Err(e) = packet.data.write_to_buffer(&writer).await {
                            error!("Failed to serialize packet: {}", e);
                        }

                        writer.flush().await.unwrap();
                    } else {
                        error!(
                            "Client with ID {:?} not found. They might have disconnected already.",
                            target
                        );
                    }
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

impl NetworkHandler for ServerNetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()> {
        match self.outgoing_sender.send(packet) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn retrieve_incoming(&mut self) -> Vec<Packet> {
        let mut result = Vec::new();

        while let Ok(packet) = self.incoming_receiver.try_recv() {
            result.push(packet);
        }

        result
    }

    fn close_all(self) {
        self.server_thread.abort();
    }
}