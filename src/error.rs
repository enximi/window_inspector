#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("window class and title cannot be both empty")]
    WindowClassTitleBothEmpty,
    #[error("cannot find window, window class: {window_class}, window title: {window_title}")]
    CannotFindWindow {
        window_class: String,
        window_title: String,
    },
    #[error("window 0x{window_handle:X} does not exist")]
    WindowNotExist { window_handle: isize },
    #[error("{api_name} failed, {error_code}, message: {message}")]
    Win32ApiFailed {
        api_name: String,
        error_code: ErrorCode,
        message: String,
    },
}

#[derive(Debug)]
pub struct ErrorCode {
    pub code: Option<i32>,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            Some(code) => write!(f, "error code: 0x{:X}", code),
            None => write!(f, "no error code"),
        }
    }
}

impl From<i32> for ErrorCode {
    fn from(code: i32) -> Self {
        ErrorCode { code: Some(code) }
    }
}
