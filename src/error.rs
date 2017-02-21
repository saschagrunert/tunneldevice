//! Everything related to error handling
use std::error::Error;
use std::{fmt, io, net, convert};

/// Common Tunnel Result type
pub type TunnelResult<T> = Result<T, TunnelError>;

/// The global Error type for wiki
pub struct TunnelError {
    /// A further description for the error
    pub description: String,

    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

/// Representation of an error case
impl TunnelError {
    /// Creates a new `TunnelError`
    pub fn new(description: &str) -> Self {
        TunnelError {
            description: description.to_string(),
            cause: None,
        }
    }

    /// Returns the corresponding `io::ErrorKind` for this error
    pub fn kind(&self) -> io::ErrorKind {
        io::ErrorKind::Other
    }
}

impl fmt::Display for TunnelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl fmt::Debug for TunnelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl convert::From<TunnelError> for io::Error {
    fn from(tunnel_error: TunnelError) -> Self {
        io::Error::new(io::ErrorKind::Other, tunnel_error.description)
    }
}

impl Error for TunnelError {
    fn description(&self) -> &str {
        &self.description
    }
}

macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for TunnelError {
            fn from(err: $p) -> Self {
                TunnelError {
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
    net::AddrParseError,
}

macro_rules! bail {
    ($($fmt:tt)*) => (
        #[cfg_attr(feature = "cargo-clippy", allow(useless_format))]
        return Err(::error::TunnelError::new(&format!($($fmt)*)))
    )
}
