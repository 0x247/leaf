mod vmess;
pub use vmess::*;

pub mod tcp;
pub mod udp;

pub use tcp::Handler as TcpHandler;
pub use udp::Handler as UdpHandler;

pub static NAME: &'static str = "vmess";
