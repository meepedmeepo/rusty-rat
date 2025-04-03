mod ipconnection;


pub use ipconnection::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NetworkError {
    #[error("Failed to parse Ipv4 Address")]
    InvalidIP(#[from] std::net::AddrParseError),
    #[error("Port number {} out of valid range 0-65,535", .0)]
    InvalidPort(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipconnections() {
        let result1 = IPConnection::new("2.4.23.223", 22);
        let result2 = IPConnection::new("003.4.3.54", 40);
        let result3 = IPConnection::new("192.0.0.254", -1);
        let result4 = IPConnection::new("blaster", 22);

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert_eq!(result3.err(), Some(NetworkError::InvalidPort(-1)));
        assert!(result4.is_err());
    }
}
