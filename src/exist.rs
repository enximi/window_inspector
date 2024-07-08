use std::ffi::c_void;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::IsWindow;

/// 判断窗口是否存在。
/// 是[`IsWindow`]的封装。
///
/// [`IsWindow`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.IsWindow.html
pub fn is_window_exist(hwnd: usize) -> bool {
    unsafe { IsWindow(HWND(hwnd as *mut c_void)) }.as_bool()
}
