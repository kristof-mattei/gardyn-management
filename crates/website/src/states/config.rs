use std::net::SocketAddr;

#[derive(Clone)]
pub struct Config {
    pub bind_to: SocketAddr,
}
