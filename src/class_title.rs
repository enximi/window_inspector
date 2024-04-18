use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetClassNameW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowTextW;

use crate::error::Error;
use crate::exist::is_window_exist;

/// 获取窗口类名。
pub fn get_window_class(hwnd: isize) -> Result<String, Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    let mut buffer = [0u16; 1024];
    match unsafe { GetClassNameW(HWND(hwnd), &mut buffer) } {
        0 => Err(Error::Win32ApiFailed {
            api_name: "GetClassNameW".to_string(),
            error_code: unsafe { GetLastError() }.0.into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        }),
        n => Ok(String::from_utf16_lossy(&buffer[..n as usize])),
    }
}

/// 获取窗口标题。
pub fn get_window_title(hwnd: isize) -> Result<String, Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    let mut buffer = [0u16; 1024];
    match unsafe { GetWindowTextW(HWND(hwnd), &mut buffer) } {
        0 => Err(Error::Win32ApiFailed {
            api_name: "GetWindowTextW".to_string(),
            error_code: unsafe { GetLastError() }.0.into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        }),
        n => Ok(String::from_utf16_lossy(&buffer[..n as usize])),
    }
}
