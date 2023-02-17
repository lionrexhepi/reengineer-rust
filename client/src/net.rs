use core::{ panic };
use std::{ sync::mpsc::channel, net::{ TcpStream, Incoming }, io::{ Read, ErrorKind } };

use log::error;
use shared::{
    net::{ PacketData, Packet, PacketDirection, NetworkHandler, PacketType },
    cbs::PacketBuf,
    util::Boxable,
};

pub struct ClientNetworkHandler {
    outgoing_sender: UnboundedSender<Packet>,
    incoming_receiver: UnboundedReceiver<Packet>,
    server_thread: JoinHandle<()>,
}

impl ClientNetworkHandler {
    fn try_read_packet(stream: &TcpStream) -> anyhow::Result<Option<Packet>> {
        let mut incoming_type = [0u8; 3];

        match stream.read_exact(&mut incoming_type) {
            Ok(_) => Ok(None),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                wait_for_fd();
                stream.set_nonblocking(false);

                let packet_type = u16::from_le_bytes(incoming_type) as PacketType;
                let size = None;

                if packet_type.size_can_vary() {
                    let packet_size_buf = [0u8; 1];

                    stream.read_exact(buf)?;

                    size = Some(packet_size_buf[0]);
                }

                let size = packet_type.get_required_buffer_size(size);

                let mut data_buffer = vec![0u8; size];

                stream.read_exact(&mut data_buffer[0..])?;

                let buffer = PacketBuf::new(data_buffer.into_boxed_slice());

                let data = packet_type.read_data(&mut buffer);

                stream.set_nonblocking(true);

                Ok(Some(Packet { direction: PacketDirection::FromServer, packet_type, data }))
            }
            Err(e) => {
                bail!(e);
            }
        }
    }

    pub fn for_server(address: &str) -> Self {
        let (send_out, mut receive_out) = channel::<Packet>();
        let (send_in, receive_in) = channel();

        let mut stream = TcpStream::connect(address).unwrap();
        stream.set_nonblocking(true).unwrap();

        let handle_thread = spawn(move || {
            'main_loop: loop {
                if let Some(packet) = Self::try_read_packet(&stream)? {
                    if send_in.send(packet).is_err() {
                        break 'main_loop;
                    }
                }

                let mut writer = BufWriter::new(&mut stream);
                while let Ok(packet) = receive_out.try_recv() {
                    if let PacketDirection::ToServer = packet.direction {
                    } else {
                        error!(
                            "Packet with invalid direction '{:?}': {:?}",
                            packet.direction,
                            packet
                        );
                        panic!("Packet has invalid direction : {:?}", packet.direction);
                    }

                    if let Err(e) = packet.data.write_to_buffer(&mut writer).await {
                        error!("Failed to serialize packet: {}", e);
                    }
                }

                writer.flush().unwrap();
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
    outgoing: UnboundedSender<Packet>,
    incoming: UnboundedReceiver<Packet>,
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
        let (to_client, from_server) = unbounded_channel();
        let (to_server, from_client) = unbounded_channel();

        (
            Self { incoming: from_server, outgoing: to_server },
            Self { incoming: from_client, outgoing: to_client },
        )
    }
}