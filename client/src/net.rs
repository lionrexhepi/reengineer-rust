use core::panic;

use log::error;
use metrohash::MetroHashMap;
use shared::net::{ PacketData, Packet, NetworkHandler, ClientId };
use tokio::{
    spawn,
    sync::mpsc::{ unbounded_channel, UnboundedSender, UnboundedReceiver },
    task::JoinHandle,
    net::{ TcpListener, TcpStream },
    io::{ AsyncReadExt, BufWriter },
};
pub struct ClientNetworkHandler {
    outgoing_sender: UnboundedSender<Packet>,
    incoming_receiver: UnboundedReceiver<Packet>,
    server_thread: JoinHandle<()>,
}

impl ClientNetworkHandler {
    pub async fn for_server(address: &str) -> Self {
        let (send_out, mut receive_out) = unbounded_channel::<Packet>();
        let (send_in, receive_in) = unbounded_channel();

        let stream = TcpStream::connect(address).await.unwrap();

        let handle_thread = spawn(async move {
            let mut clients = MetroHashMap::default();

            'main_loop: loop {
                let mut buffer = Vec::new();
                if let Err(e) = stream.read_to_end(&mut buffer).await {
                    error!("Error loading Packet data: {}", e);
                }
                let packet_data = PacketData::from_bytes(&buffer);

                match packet_data {
                    Ok(packet_data) => {
                        let packet = Packet {
                            data: packet_data,
                            direction: PacketDirection::Clientbound(id.clone()),
                        };

                        if send_in.send(packet).is_err() {
                            break 'main_loop;
                        }
                    }

                    Err(e) => error!("Failed to parse Packet: {}", e),
                }
            }

            while let Ok(packet) = receive_out.try_recv() {
                let target = match packet.direction {
                    PacketDirection::Serverbound(_) =>
                        panic!(
                            "Packets sent by the server are NOT supposed to be serverbound. Packet in question: {:?}",
                            packet
                        ),
                    PacketDirection::Clientbound(target) => target,
                };

                if let Some(stream) = clients.get_mut(&target) {
                    let writer = BufWriter::new(&mut *stream);
                    if let Err(e) = packet.data.write_to_buffer(writer).await {
                        error!("Failed to serialize packet: {}", e);
                    }
                } else {
                    error!(
                        "Client with ID {:?} not found. They might have disconnected already.",
                        target
                    );
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