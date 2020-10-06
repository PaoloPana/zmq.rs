use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use super::EndpointError;

/// The type of transport used by a given endpoint
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Transport {
    /// TCP transport
    Tcp,
}

impl FromStr for Transport {
    type Err = EndpointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "tcp" => Transport::Tcp,
            _ => return Err(EndpointError::UnknownTransport(s.to_string())),
        };
        Ok(result)
    }
}
impl TryFrom<&str> for Transport {
    type Error = EndpointError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl fmt::Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let s = match self {
            Transport::Tcp => "tcp",
        };
        write!(f, "{}", s)
    }
}
