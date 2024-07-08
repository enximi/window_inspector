use thiserror::Error;
use windows::Win32::Foundation::HWND;

#[derive(Error, Debug)]
pub enum WindowInspectorError {
    #[error("窗口类名和标题都为空")]
    WindowClassTitleBothEmpty,
    #[error(
        "FindWindowExW失败，窗口类名：{window_class}，窗口标题：{window_title}，{error_message}"
    )]
    FindWindowExWFailed {
        window_class: String,
        window_title: String,
        error_message: String,
    },
    #[error("GetWindowTextW失败，error_code: {error_code:#X}")]
    GetClassNameWFailed { error_code: u32 },
    #[error("SetForegroundWindow失败")]
    SetForegroundWindowFailed,
    #[error("GetWindowRect失败，{hwnd:?}，{error_message}")]
    GetWindowRectFailed { hwnd: HWND, error_message: String },
    #[error("DwmGetWindowAttribute失败，{hwnd:?}，{error_message}")]
    DwmGetWindowAttributeFailed { hwnd: HWND, error_message: String },
    #[error("ClientToScreen失败，{hwnd:?}")]
    ClientToScreenFailed { hwnd: HWND },
    #[error("GetClientRect失败，{hwnd:?}，{error_message}")]
    GetClientRectFailed { hwnd: HWND, error_message: String },
    #[error("MoveWindow失败，{hwnd:?}，{error_message}")]
    MoveWindowFailed { hwnd: HWND, error_message: String },
    #[error("GetWindowThreadProcessId失败，error_code: {error_code:#X}")]
    GetWindowThreadProcessIdFailed { error_code: u32 },
    #[error("OpenProcess失败，error_code: {error_message}")]
    OpenProcessFailed {
        process_id: u32,
        error_message: String,
    },
    #[error(
        "QueryFullProcessImageNameW失败，process_id: {process_id}，error_code: {error_message}"
    )]
    QueryFullProcessImageNameWFailed {
        process_id: u32,
        error_message: String,
    },
    #[error("GetWindowLongW失败，error_code: {error_code:#X}")]
    GetWindowLongWFailed { error_code: u32 },
    #[error("SetWindowPos失败，{hwnd:?}，{error_message}")]
    SetWindowPosFailed { hwnd: HWND, error_message: String },
    #[error("窗口不存在，{hwnd:?}")]
    WindowNotExist { hwnd: HWND },
}
