use shared::net::NetworkHandler;

pub mod chunk;

pub struct ServerController {
    net_handler: dyn NetworkHandler
}

