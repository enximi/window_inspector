use std::ffi::c_void;

use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetClassNameW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowTextW;

use crate::error::WindowInspectorError;
use crate::exist::is_window_exist;
use crate::result::Result;

/// 获取窗口类名。
pub fn get_window_class(hwnd: usize) -> Result<String> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    let mut buffer = [0u16; 1024];
    match unsafe { GetClassNameW(HWND(hwnd as *mut c_void), &mut buffer) } {
        0 => Err(WindowInspectorError::GetClassNameWFailed {
            error_code: unsafe { GetLastError() }.0,
        }),
        n => Ok(String::from_utf16_lossy(&buffer[..n as usize])),
    }
}

/// 获取窗口标题。
pub fn get_window_title(hwnd: usize) -> Result<String> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    let mut buffer = [0u16; 1024];
    match unsafe { GetWindowTextW(HWND(hwnd as *mut c_void), &mut buffer) } {
        0 => Err(WindowInspectorError::GetClassNameWFailed {
            error_code: unsafe { GetLastError() }.0,
        }),
        n => Ok(String::from_utf16_lossy(&buffer[..n as usize])),
    }
}
