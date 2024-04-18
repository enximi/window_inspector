use crate::exist::is_window_exist;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};
use crate::error::Error;

/// 获取前台窗口句柄。
pub fn get_foreground_hwnd() -> isize {
    unsafe { GetForegroundWindow() }.0
}

/// 判断窗口是否处于前台。
pub fn is_foreground(hwnd: isize) -> bool {
    hwnd == get_foreground_hwnd()
}

/// 设置前台窗口。
pub fn set_foreground_window(hwnd: isize) -> Result<(), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    if !unsafe { SetForegroundWindow(HWND(hwnd)) }.as_bool() {
        return Err(Error::Win32ApiFailed {
            api_name: "SetForegroundWindow".to_string(),
            error_code: None.into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        });
    }

    Ok(())
}
