use std::ffi::c_void;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

use crate::error::WindowInspectorError;
use crate::exist::is_window_exist;
use crate::result::Result;

/// 获取前台窗口句柄。
pub fn get_foreground_hwnd() -> usize {
    unsafe { GetForegroundWindow() }.0 as usize
}

/// 判断窗口是否处于前台。
pub fn is_foreground(hwnd: usize) -> bool {
    hwnd == get_foreground_hwnd()
}

/// 设置前台窗口。
pub fn set_foreground_window(hwnd: usize) -> Result<()> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    if !unsafe { SetForegroundWindow(HWND(hwnd as *mut c_void)) }.as_bool() {
        return Err(WindowInspectorError::SetForegroundWindowFailed);
    }

    Ok(())
}
