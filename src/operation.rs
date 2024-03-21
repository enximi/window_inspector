//! 窗口操作。
//! # 功能
//! ## 前台
//! - 设置前台窗口。[`set_foreground_window`]
//! ## 移动
//! - 移动窗口到xywh。[`move_window_to_xywh`]
//! ## 置顶
//! - 设置窗口置顶。[`set_window_top_most`]
//! - 取消窗口置顶。[`cancel_window_top_most`]
//! - 切换窗口置顶状态。[`switch_window_top_most`]

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MoveWindow, SetForegroundWindow, SetWindowPos, HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE,
    SWP_NOSIZE,
};

use crate::error::Error;
use crate::information::{get_window_top_most, is_window_exist};

// # 前台

/// 设置前台窗口。
pub fn set_foreground_window(window_handle: isize) -> Result<(), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    unsafe {
        if !SetForegroundWindow(HWND(window_handle)).as_bool() {
            return Err(Error::Win32ApiFailed {
                api_name: "SetForegroundWindow".to_string(),
                message: format!("window handle: {}", window_handle),
                error_code: None,
            });
        }
    }
    Ok(())
}

// # 移动

/// 移动窗口到xywh。
pub fn move_window_to_xywh(
    window_handle: isize,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<(), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    unsafe {
        if let Err(e) = MoveWindow(HWND(window_handle), x, y, width as i32, height as i32, true) {
            return Err(Error::Win32ApiFailed {
                api_name: "MoveWindow".to_string(),
                message: format!("window handle: {}", window_handle),
                error_code: Some(e.code().0),
            });
        }
    }
    Ok(())
}

// # 置顶

/// 设置窗口置顶状态。
fn set_window_top_most_status(window_handle: isize, is_top_most: bool) -> Result<(), Error> {
    if !is_window_exist(window_handle) {
        return Err(Error::WindowNotExist { window_handle });
    }
    unsafe {
        if let Err(e) = SetWindowPos(
            HWND(window_handle),
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
                message: format!("window handle: {}", window_handle),
                error_code: Some(e.code().0),
            });
        }
    }
    Ok(())
}

/// 设置窗口置顶。
pub fn set_window_top_most(window_handle: isize) -> Result<(), Error> {
    set_window_top_most_status(window_handle, true)
}

/// 取消窗口置顶。
pub fn cancel_window_top_most(window_handle: isize) -> Result<(), Error> {
    set_window_top_most_status(window_handle, false)
}

/// 切换窗口置顶状态。
pub fn switch_window_top_most(window_handle: isize) -> Result<(), Error> {
    let is_top_most = get_window_top_most(window_handle)?;
    if is_top_most {
        cancel_window_top_most(window_handle)
    } else {
        set_window_top_most(window_handle)
    }
}
