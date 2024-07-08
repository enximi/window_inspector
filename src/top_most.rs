use std::ffi::c_void;

use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::GWL_EXSTYLE;
use windows::Win32::UI::WindowsAndMessaging::HWND_NOTOPMOST;
use windows::Win32::UI::WindowsAndMessaging::HWND_TOPMOST;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;

use crate::error::WindowInspectorError;
use crate::exist::is_window_exist;
use crate::result::Result;

/// 获取窗口置顶状态。
pub fn get_window_top_most(hwnd: usize) -> Result<bool> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    match unsafe { GetWindowLongW(HWND(hwnd as *mut c_void), GWL_EXSTYLE) } {
        0 => Err(WindowInspectorError::GetWindowLongWFailed {
            error_code: unsafe { GetLastError() }.0,
        }),
        n => Ok((n as u32 & WS_EX_TOPMOST.0) != 0),
    }
}

/// 设置窗口置顶状态。
fn set_window_top_most_status(hwnd: usize, is_top_most: bool) -> Result<()> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    unsafe {
        if let Err(e) = SetWindowPos(
            HWND(hwnd as *mut c_void),
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
            return Err(WindowInspectorError::SetWindowPosFailed {
                hwnd: HWND(hwnd as *mut c_void),
                error_message: format!("{:?}", e),
            });
        }
    }
    Ok(())
}

/// 设置窗口置顶。
pub fn set_window_top_most(hwnd: usize) -> Result<()> {
    set_window_top_most_status(hwnd, true)
}

/// 取消窗口置顶。
pub fn cancel_window_top_most(hwnd: usize) -> Result<()> {
    set_window_top_most_status(hwnd, false)
}

/// 切换窗口置顶状态。
pub fn toggle_window_top_most(hwnd: usize) -> Result<()> {
    let is_top_most = get_window_top_most(hwnd)?;
    if is_top_most {
        cancel_window_top_most(hwnd)
    } else {
        set_window_top_most(hwnd)
    }
}
