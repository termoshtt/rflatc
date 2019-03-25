use std::{ffi, fmt, str};

#[derive(Debug, Clone)]
pub enum Error {
    /// String is not '\0' terminated
    InvalidString(ffi::FromBytesWithNulError),

    /// String is not encoded by UTF-8
    NonUtf8(str::Utf8Error),
}

impl From<ffi::FromBytesWithNulError> for Error {
    fn from(e: ffi::FromBytesWithNulError) -> Error {
        Error::InvalidString(e)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Error {
        Error::NonUtf8(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidString(e) => e.fmt(f),
            Error::NonUtf8(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidString(e) => Some(e),
            Error::NonUtf8(e) => Some(e),
        }
    }
}
