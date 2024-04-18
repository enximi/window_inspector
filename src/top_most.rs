use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowPos, GWL_EXSTYLE, HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE,
    SWP_NOSIZE, WS_EX_TOPMOST,
};

use crate::error::Error;
use crate::exist::is_window_exist;

/// 获取窗口置顶状态。
pub fn get_window_top_most(hwnd: isize) -> Result<bool, Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    match unsafe { GetWindowLongW(HWND(hwnd), GWL_EXSTYLE) } {
        0 => Err(Error::Win32ApiFailed {
            api_name: "GetWindowLongW".to_string(),
            error_code: unsafe { GetLastError() }.0.into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        }),
        n => Ok((n as u32 & WS_EX_TOPMOST.0) != 0),
    }
}

/// 设置窗口置顶状态。
fn set_window_top_most_status(hwnd: isize, is_top_most: bool) -> Result<(), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    unsafe {
        if let Err(e) = SetWindowPos(
            HWND(hwnd),
            if is_top_most {
                HWND_TOPMOST
            } else {
                HWND_NOTOPMOST
            },
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        ) {
            return Err(Error::Win32ApiFailed {
                api_name: "SetWindowPos".to_string(),
                error_code: (e.code().0 as u32).into(),
                message: format!("hwnd: 0x{:X}", hwnd),
            });
        }
    }
    Ok(())
}

/// 设置窗口置顶。
pub fn set_window_top_most(hwnd: isize) -> Result<(), Error> {
    set_window_top_most_status(hwnd, true)
}

/// 取消窗口置顶。
pub fn cancel_window_top_most(hwnd: isize) -> Result<(), Error> {
    set_window_top_most_status(hwnd, false)
}

/// 切换窗口置顶状态。
pub fn toggle_window_top_most(hwnd: isize) -> Result<(), Error> {
    let is_top_most = get_window_top_most(hwnd)?;
    if is_top_most {
        cancel_window_top_most(hwnd)
    } else {
        set_window_top_most(hwnd)
    }
}
