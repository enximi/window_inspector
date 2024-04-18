use std::mem::size_of;

use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetWindowRect, MoveWindow};

use crate::error::Error;
use crate::exist::is_window_exist;

/// 获取窗口位置尺寸（包括阴影），相对于屏幕。
/// # 返回
/// (x, y, width, height)
pub fn get_window_xywh_include_shadow(hwnd: isize) -> Result<(i32, i32, u32, u32), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    let mut rect = RECT::default();
    match unsafe { GetWindowRect(HWND(hwnd), &mut rect) } {
        Ok(_) => Ok((
            rect.left,
            rect.top,
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(Error::Win32ApiFailed {
            api_name: "GetWindowRect".to_string(),
            error_code: (e.code().0 as u32).into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        }),
    }
}

/// 获取窗口位置尺寸（不包括阴影），相对于屏幕。许多截屏软件获取窗口矩形时，不包括阴影。这个函数得到的窗口大小与截屏软件得到的窗口大小一致。
/// # 返回
/// (x, y, width, height)
pub fn get_window_xywh_exclude_shadow(hwnd: isize) -> Result<(i32, i32, u32, u32), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    let mut rect = RECT::default();
    match unsafe {
        DwmGetWindowAttribute(
            HWND(hwnd),
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
            error_code: (e.code().0 as u32).into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        }),
    }
}

/// 获取客户区左上角坐标，相对于屏幕。
/// # 返回
/// (x, y)
pub fn get_client_xy(hwnd: isize) -> Result<(i32, i32), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    let mut point = POINT::default();
    unsafe { ClientToScreen(HWND(hwnd), &mut point) };
    Ok((point.x, point.y))
}

/// 获取客户区尺寸。
/// # 返回
/// (width, height)
pub fn get_client_wh(hwnd: isize) -> Result<(u32, u32), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    let mut rect = RECT::default();
    match unsafe { GetClientRect(HWND(hwnd), &mut rect) } {
        Ok(_) => Ok((
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(Error::Win32ApiFailed {
            api_name: "GetClientRect".to_string(),
            error_code: (e.code().0 as u32).into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        }),
    }
}

/// 获取客户区位置尺寸，相对于屏幕。
/// # 返回
/// (x, y, width, height)
pub fn get_client_xywh(hwnd: isize) -> Result<(i32, i32, u32, u32), Error> {
    let (x, y) = get_client_xy(hwnd)?;
    let (width, height) = get_client_wh(hwnd)?;
    Ok((x, y, width, height))
}

/// 移动窗口到xywh。
pub fn move_window_to_xywh(
    hwnd: isize,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<(), Error> {
    if !is_window_exist(hwnd) {
        return Err(Error::WindowNotExist { hwnd });
    }
    unsafe {
        if let Err(e) = MoveWindow(HWND(hwnd), x, y, width as i32, height as i32, true) {
            return Err(Error::Win32ApiFailed {
                api_name: "MoveWindow".to_string(),
                error_code: (e.code().0 as u32).into(),
                message: format!("hwnd: 0x{:X}", hwnd),
            });
        }
    }
    Ok(())
}
