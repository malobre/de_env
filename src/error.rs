use std::{
    ffi::{OsStr, OsString},
    fmt::{self, Display},
    num::{ParseFloatError, ParseIntError},
};

/// Convenience alias for a `Result` with this crate [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represent an error that may arise when deserializing.
#[derive(Debug, Clone)]
#[must_use]
pub struct Error(Box<ErrorCode>);

#[derive(Debug, Clone)]
enum ErrorCode {
    Message(Box<str>),
    UnsupportedType(&'static str),
    InvalidUnicode(Box<OsStr>),
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
    InvalidBool(Box<OsStr>),
}

impl Error {
    fn new(code: ErrorCode) -> Self {
        Self(Box::new(code))
    }

    pub(crate) fn unsupported_type(ty: &'static str) -> Self {
        Self::new(ErrorCode::UnsupportedType(ty))
    }

    pub(crate) fn invalid_unicode(value: OsString) -> Self {
        Self::new(ErrorCode::InvalidUnicode(value.into_boxed_os_str()))
    }

    pub(crate) fn invalid_bool(value: OsString) -> Self {
        Self::new(ErrorCode::InvalidBool(value.into_boxed_os_str()))
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::new(ErrorCode::Message(msg.to_string().into_boxed_str()))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0.as_ref() {
            ErrorCode::Message(msg) => formatter.write_str(msg),
            ErrorCode::UnsupportedType(ty) => formatter.write_fmt(format_args!(
                "`{ty}` cannot be deserialized from environment variables"
            )),
            ErrorCode::InvalidUnicode(value) => formatter.write_fmt(format_args!(
                "`{}` could not be deserialized and parsed as it is not valid unicode",
                value.to_string_lossy()
            )),
            ErrorCode::InvalidInteger(err) => err.fmt(formatter),
            ErrorCode::InvalidFloat(err) => err.fmt(formatter),
            ErrorCode::InvalidBool(value) => formatter.write_fmt(format_args!(
                "`{}` is not a boolean",
                value.to_string_lossy()
            )),
        }
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::new(ErrorCode::InvalidInteger(error))
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        Self::new(ErrorCode::InvalidFloat(error))
    }
}
