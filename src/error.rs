use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("window class and title cannot be both empty")]
    WindowClassTitleBothEmpty,
    #[error("cannot find window, window class: {window_class}, window title: {window_title}")]
    CannotFindWindow {
        window_class: String,
        window_title: String,
    },
    #[error("window 0x{hwnd:X} does not exist")]
    WindowNotExist { hwnd: isize },
    #[error("{api_name} failed, {error_code}, message: {message}")]
    Win32ApiFailed {
        api_name: String,
        error_code: ErrorCode,
        message: String,
    },
}

#[derive(Debug)]
pub struct ErrorCode {
    pub code: Option<u32>,
}

impl ErrorCode {
    pub fn new(code: Option<u32>) -> Self {
        ErrorCode { code }
    }

    pub fn from_code(code: u32) -> Self {
        ErrorCode { code: Some(code) }
    }

    pub fn none() -> Self {
        ErrorCode { code: None }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            Some(code) => write!(f, "error code: 0x{:X}", code),
            None => write!(f, "no error code"),
        }
    }
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self { code: None }
    }
}

impl From<u32> for ErrorCode {
    fn from(code: u32) -> Self {
        ErrorCode { code: Some(code) }
    }
}

impl From<Option<u32>> for ErrorCode {
    fn from(code: Option<u32>) -> Self {
        ErrorCode { code }
    }
}
