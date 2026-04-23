use crate::ffi;
use std::error;
use std::ffi::CStr;
use std::fmt;

/// The result type used throughout this crate's high-level API.
pub type Result<T> = std::result::Result<T, Error>;

/// An error reported by libfreefare or by this crate's wrapper layer.
///
/// Wrapper-generated errors use `None` for the code. Native libfreefare
/// failures may carry both a human-readable message and the original return
/// code when one is available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    code: Option<i32>,
    message: String,
}

impl Error {
    /// Creates an error with a message but no libfreefare error code.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
        }
    }

    /// Creates an error from a libfreefare tag and negative return code.
    pub(crate) fn from_tag(tag: ffi::FreefareTag, code: i32, fallback: impl Into<String>) -> Self {
        let fallback = fallback.into();
        let message = if tag.is_null() {
            fallback
        } else {
            unsafe {
                let ptr = ffi::freefare_strerror(tag);
                if ptr.is_null() {
                    fallback
                } else {
                    CStr::from_ptr(ptr).to_string_lossy().into_owned()
                }
            }
        };

        Self {
            code: Some(code),
            message,
        }
    }

    /// Returns the native libfreefare error code when one is available.
    pub fn code(&self) -> Option<i32> {
        self.code
    }

    /// Returns the human-readable error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "{} ({code})", self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn wrapper_error_has_no_code() {
        let error = Error::new("wrapper failure");
        assert_eq!(error.code(), None);
        assert_eq!(error.message(), "wrapper failure");
        assert_eq!(error.to_string(), "wrapper failure");
    }

    #[test]
    fn string_conversion_preserves_message() {
        let error: Error = "converted failure".into();
        assert_eq!(error.code(), None);
        assert_eq!(error.message(), "converted failure");
    }
}
