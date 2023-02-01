use core::{ panic };

use log::error;
use shared::net::{ PacketData, Packet, PacketDirection, NetworkHandler };
use tokio::{
    spawn,
    sync::mpsc::{ unbounded_channel, UnboundedSender, UnboundedReceiver },
    task::JoinHandle,
    net::{ TcpStream },
    io::{ AsyncReadExt, BufWriter, AsyncWriteExt },
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

        let mut stream = TcpStream::connect(address).await.unwrap();

        let handle_thread = spawn(async move {
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
                            direction: PacketDirection::FromServer,
                        };

                        if send_in.send(packet).is_err() {
                            break 'main_loop;
                        }
                    }

                    Err(e) => error!("Failed to parse Packet: {}", e),
                }
            }
            let mut writer = BufWriter::new(&mut stream);
            while let Ok(packet) = receive_out.try_recv() {
                if let PacketDirection::ToServer = packet.direction {
                } else {
                    error!("Packet with invalid direction '{:?}': {:?}", packet.direction, packet);
                    panic!("Packet has invalid direction : {:?}", packet.direction);
                }

                if let Err(e) = packet.data.write_to_buffer(&writer).await {
                    error!("Failed to serialize packet: {}", e);
                }
            }

            writer.flush().await.unwrap();
        });

        Self {
            server_thread: handle_thread,
            outgoing_sender: send_out,
            incoming_receiver: receive_in,
        }
    }
}


pub struct FakeNetworkHandler {
    outgoing: UnboundedSender<Packet>,
    incoming: UnboundedReceiver<Packet>
}

impl NetworkHandler for FakeNetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()> {
        match self.outgoing.send(packet) {
            Ok(_)=> Ok(()),
            Err(e)=> Err(e.into())
        }
    }

    fn retrieve_incoming(&mut self) -> Vec<Packet> {
        let mut result = Vec::new();
        while let Ok(packet) = self.incoming.try_recv() {
            result.push(packet);
        }
        result
    }

    fn close_all(self) {
       
    }
}

impl FakeNetworkHandler {
    pub fn new_pair() -> (Self, Self) {
        let (to_client, from_server) = unbounded_channel();
        let (to_server, from_client) = unbounded_channel();

        (
            Self { incoming: from_server, outgoing: to_server},
            Self { incoming: from_client, outgoing: to_client}
        )
    }
}