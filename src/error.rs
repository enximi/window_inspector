#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("window class and title are both empty")]
    WindowClassTitleBothEmpty,
    #[error("cannot find window class: {window_class}, window title: {window_title}")]
    CannotFindWindow {
        window_class: String,
        window_title: String,
    },
    #[error("window does not exist: {window_handle}")]
    WindowNotExist { window_handle: isize },
    #[error("{api_name} failed, message: {message}, error code: {error_code:?}")]
    Win32ApiFailed {
        api_name: String,
        message: String,
        error_code: Option<i32>,
    },
}
