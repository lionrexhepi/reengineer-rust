










pub mod packet;
pub mod packet_data;

pub trait NetworkHandler {
    fn enqueue_packet(&self, packet: packet::Packet) -> anyhow::Result<()>;

    fn retrieve_incoming(&mut self) -> Vec<packet::Packet>;

    fn close_all(self);
}