//! Error handling for marcos file manager

use std::io;
use std::{fmt, result};

use log::SetLoggerError;
use toml::de;

use failure;
use failure::{Backtrace, Context, Fail};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// Kinds of error which need to be handled
#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO Error")]
    Io(#[cause] io::Error),

    #[fail(display = "Directory not found: {}", dirname)]
    DirNotFound { dirname: String },

    #[fail(display = "Error while initializing logger!")]
    LogInitError(#[cause] SetLoggerError),
    // TODO handle generic error
    #[fail(display = "Toml deserialization error")]
    TomlDeError(#[cause] de::Error),

    #[fail(display = "Generic Error")]
    GenericError,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(kind: Context<ErrorKind>) -> Error {
        Error { inner: kind }
    }
}

impl From<io::Error> for Error {
    fn from(kind: io::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::Io(kind)),
        }
    }
}

impl From<SetLoggerError> for Error {
    fn from(kind: SetLoggerError) -> Error {
        Error {
            inner: Context::new(ErrorKind::LogInitError(kind)),
        }
    }
}

impl From<de::Error> for Error {
    fn from(kind: de::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::TomlDeError(kind)),
        }
    }
}

/// Return a prettily formatted error, including its entire causal chain.
pub fn failure_to_string(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }
    pretty
}
