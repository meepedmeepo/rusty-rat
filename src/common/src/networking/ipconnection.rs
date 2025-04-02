use std::{net::Ipv4Addr, str::FromStr};

use super::NetworkError;


///# IP Connection string
/// acts as a simple way of combining an ip address and a port together to easily generate connections 
#[derive(Debug)]
pub struct IPConnection {
    ip: Ipv4Addr,
    port: i32,
}

impl IPConnection {
    pub fn new(ip: &str, port: i32) -> Result<Self, NetworkError> {
        let ip = Ipv4Addr::from_str(ip).map_err(NetworkError::InvalidIP)?;
        if port <= 0 || port > 65535 {
            return Err(NetworkError::InvalidPort(port));
        }
        Ok(Self { ip, port })
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn octets_string(&self) -> String {
        self.ip.to_string()
    }

    pub fn octets(&self) -> [u8; 4] {
        self.ip.octets()
    }

    pub fn get_port(&self) -> i32 {
        self.port
    }
}

impl Default for IPConnection {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::LOCALHOST,
            port: 80,
        }
    }
}

impl std::fmt::Display for IPConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.ip.to_string(), self.port)
    }
}
