use std::net::{IpAddr, Ipv4Addr};

pub const MAX_PORT_RANGE: u16 = 65535;
pub const FALLBACK_IP_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
