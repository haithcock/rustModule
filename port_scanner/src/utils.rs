use std::net::{SocketAddr, ToSocketAddrs};

pub fn resolve_first(host: &str, port: u16) -> Option<SocketAddr> {
    let addr_str = format!("{host}:{port}");
    addr_str.to_socket_addrs().ok()?.next()
}
