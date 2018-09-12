//! Error handling for marcos file manager

use std::{fmt, result};
use std::io;

use log::SetLoggerError;

use failure;
use failure::{Fail, Context, Backtrace};


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

#[derive(Debug,Fail)]
pub enum ErrorKind {
    #[fail(display = "IO Error")]
    Io(#[cause] io::Error),

    #[fail(display = "Error while initializing logger!")]
    LogInitError(#[cause] SetLoggerError),
    // TODO handle generic error
    #[fail(display = "Generic Error")]
    GenericError,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind)}
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(kind: Context<ErrorKind>) -> Error {
        Error {inner: kind}
    }
}

impl From<io::Error> for Error {
    fn from(kind: io::Error) -> Error {
        Error { inner: Context::new(ErrorKind::Io(kind))}
    }
}

impl From<SetLoggerError> for Error {
    fn from(kind: SetLoggerError) -> Error {
        Error { inner: Context::new(ErrorKind::LogInitError(kind))}
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
