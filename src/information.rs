//! 信息获取模块
//! # 功能
//! ## 句柄
//! - 获取窗口句柄。[`get_window_handle`]
//! - 获取窗口句柄，参考缓存。[`get_window_handle_ref_cache`]
//! ## 存在
//! - 判断窗口是否存在。[`is_window_exist`]
//! ## 前台
//! - 获取前台窗口句柄。[`get_foreground_window_handle`]
//! - 判断窗口是否处于前台。[`is_foreground`]
//! ## 类名和标题
//! - 获取窗口类名。[`get_window_class`]
//! - 获取窗口标题。[`get_window_title`]
//! ## 尺寸和位置
//! - 获取窗口位置尺寸（包括阴影），相对于屏幕。[`get_window_xywh_include_shadow`]
//! - 获取窗口位置尺寸（不包括阴影），相对于屏幕。[`get_window_xywh_exclude_shadow`]
//! - 获取客户区左上角坐标，相对于屏幕。[`get_client_xy`]
//! - 获取客户区尺寸。[`get_client_wh`]
//! - 获取客户区位置尺寸，相对于屏幕。[`get_client_xywh`]
//! ## 置顶
//! - 获取窗口置顶状态。[`get_window_top_most`]

use std::collections::HashMap;
use std::mem::size_of;
use std::ptr::null;
use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetClassNameW, GetClientRect, GetForegroundWindow, GetWindowLongW, GetWindowRect,
    GetWindowTextW, IsWindow, GWL_EXSTYLE, WS_EX_TOPMOST,
};

use crate::error::Error;

// # 句柄

/// 获取窗口句柄。
/// 是[`FindWindowW`]的封装。
/// 与[`FindWindowW`]不同的是，[`get_window_handle`]不允许两个参数同时为空。
/// 如果两个参数同时为空，将返回[`Error::WindowClassTitleBothEmpty`]。
/// 性能较差，建议使用[`get_window_handle_ref_cache`]。
///
/// [`FindWindowW`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.FindWindowW.html
pub fn get_window_handle(window_class: &str, window_title: &str) -> Result<isize, Error> {
    if window_class.is_empty() && window_title.is_empty() {
        return Err(Error::WindowClassTitleBothEmpty);
    }
    fn str_to_pcwstr(s: &str) -> PCWSTR {
        if s.is_empty() {
            PCWSTR(null())
        } else {
            let v: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
            PCWSTR(v.as_ptr())
        }
    }
    match unsafe { FindWindowW(str_to_pcwstr(window_class), str_to_pcwstr(window_title)) }.0 {
        0 => Err(Error::CannotFindWindow {
            window_class: window_class.to_string(),
            window_title: window_title.to_string(),
        }),
        handle => Ok(handle),
    }
}

lazy_static! {
    static ref WINDOW_HANDLE_CACHE: Mutex<HashMap<(String, String), isize>> =
        Mutex::new(HashMap::new());
}

/// 获取窗口句柄，参考缓存。
/// # 可能不符合预期的行为
/// 调用该函数成功找到窗口一次之后，如果窗口标题改变，但是还使用原先的参数调用该函数，将依然返回原先的窗口句柄。
/// 因为缓存中有窗口句柄且窗口仍然存在。
pub fn get_window_handle_ref_cache(window_class: &str, window_title: &str) -> Result<isize, Error> {
    if window_class.is_empty() && window_title.is_empty() {
        return Err(Error::WindowClassTitleBothEmpty);
    }
    let mut cache = WINDOW_HANDLE_CACHE.lock().unwrap();
    let key = (window_class.to_string(), window_title.to_string());
    let handle = cache.get(&key);
    if handle.is_some_and(|handle| is_window_exist(*handle)) {
        Ok(*handle.unwrap())
    } else {
        cache.remove(&key);
        let handle = get_window_handle_ref_cache(window_class, window_title)?;
        cache.insert(key, handle);
        Ok(handle)
    }
}

// # 存在

/// 判断窗口是否存在。
/// 是[`IsWindow`]的封装。
///
/// [`IsWindow`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.IsWindow.html
pub fn is_window_exist(window_handle: isize) -> bool {
    unsafe { IsWindow(HWND(window_handle)) }.as_bool()
}

// # 前台

/// 获取前台窗口句柄。
pub fn get_foreground_window_handle() -> isize {
    unsafe { GetForegroundWindow() }.0
}

/// 判断窗口是否处于前台。
pub fn is_foreground(window_handle: isize) -> bool {
    window_handle == get_foreground_window_handle()
}

// # 类名和标题

/// 获取窗口类名。
pub fn get_window_class(window_handle: isize) -> Result<String, Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    let mut buffer = [0u16; 1024];
    match unsafe { GetClassNameW(HWND(window_handle), &mut buffer) } {
        0 => Err(Error::Win32ApiFailed {
            api_name: "GetClassNameW".to_string(),
            message: format!("window handle: {}", window_handle),
            error_code: None,
        }),
        n => Ok(String::from_utf16_lossy(&buffer[..n as usize])),
    }
}

/// 获取窗口标题。
pub fn get_window_title(window_handle: isize) -> Result<String, Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    let mut buffer = [0u16; 1024];
    match unsafe { GetWindowTextW(HWND(window_handle), &mut buffer) } {
        0 => Err(Error::Win32ApiFailed {
            api_name: "GetWindowTextW".to_string(),
            message: format!("window handle: {}", window_handle),
            error_code: None,
        }),
        n => Ok(String::from_utf16_lossy(&buffer[..n as usize])),
    }
}

// # 尺寸和位置

/// 获取窗口位置尺寸（包括阴影），相对于屏幕。
/// # 返回
/// (x, y, width, height)
pub fn get_window_xywh_include_shadow(window_handle: isize) -> Result<(i32, i32, u32, u32), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    let mut rect = RECT::default();
    match unsafe { GetWindowRect(HWND(window_handle), &mut rect) } {
        Ok(_) => Ok((
            rect.left,
            rect.top,
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(Error::Win32ApiFailed {
            api_name: "GetWindowRect".to_string(),
            message: format!("window handle: {}", window_handle),
            error_code: Some(e.code().0),
        }),
    }
}

/// 获取窗口位置尺寸（不包括阴影），相对于屏幕。许多截屏软件获取窗口矩形时，不包括阴影。这个函数得到的窗口大小与截屏软件得到的窗口大小一致。
/// # 返回
/// (x, y, width, height)
pub fn get_window_xywh_exclude_shadow(window_handle: isize) -> Result<(i32, i32, u32, u32), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    let mut rect = RECT::default();
    match unsafe {
        DwmGetWindowAttribute(
            HWND(window_handle),
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut _,
            size_of::<RECT>() as u32,
        )
    } {
        Ok(_) => Ok((
            rect.left,
            rect.top,
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(Error::Win32ApiFailed {
            api_name: "DwmGetWindowAttribute".to_string(),
            message: format!("window handle: {}", window_handle),
            error_code: Some(e.code().0),
        }),
    }
}

/// 获取客户区左上角坐标，相对于屏幕。
/// # 返回
/// (x, y)
pub fn get_client_xy(window_handle: isize) -> Result<(i32, i32), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    let mut point = POINT::default();
    unsafe { ClientToScreen(HWND(window_handle), &mut point) };
    Ok((point.x, point.y))
}

/// 获取客户区尺寸。
/// # 返回
/// (width, height)
pub fn get_client_wh(window_handle: isize) -> Result<(u32, u32), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    let mut rect = RECT::default();
    match unsafe { GetClientRect(HWND(window_handle), &mut rect) } {
        Ok(_) => Ok((
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(Error::Win32ApiFailed {
            api_name: "GetClientRect".to_string(),
            message: format!("window handle: {}", window_handle),
            error_code: Some(e.code().0),
        }),
    }
}

/// 获取客户区位置尺寸，相对于屏幕。
/// # 返回
/// (x, y, width, height)
pub fn get_client_xywh(window_handle: isize) -> Result<(i32, i32, u32, u32), Error> {
    let (x, y) = get_client_xy(window_handle)?;
    let (width, height) = get_client_wh(window_handle)?;
    Ok((x, y, width, height))
}

// # 置顶

/// 获取窗口置顶状态。
pub fn get_window_top_most(window_handle: isize) -> Result<bool, Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    match unsafe { GetWindowLongW(HWND(window_handle), GWL_EXSTYLE) } {
        0 => Err(Error::Win32ApiFailed {
            api_name: "GetWindowLongW".to_string(),
            message: format!("window handle: {}", window_handle),
            error_code: None,
        }),
        n => Ok((n as u32 & WS_EX_TOPMOST.0) != 0),
    }
}
