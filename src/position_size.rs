use std::ffi::c_void;
use std::mem::size_of;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::POINT;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Dwm::DwmGetWindowAttribute;
use windows::Win32::Graphics::Dwm::DWMWA_EXTENDED_FRAME_BOUNDS;
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
use windows::Win32::UI::WindowsAndMessaging::MoveWindow;

use crate::error::WindowInspectorError;
use crate::exist::is_window_exist;
use crate::result::Result;

/// 获取窗口位置尺寸（包括阴影），相对于屏幕。
/// # 返回
/// (x, y, width, height)
pub fn get_window_xywh_include_shadow(hwnd: usize) -> Result<(i32, i32, u32, u32)> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    let mut rect = RECT::default();
    match unsafe { GetWindowRect(HWND(hwnd as *mut c_void), &mut rect) } {
        Ok(_) => Ok((
            rect.left,
            rect.top,
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(WindowInspectorError::GetWindowRectFailed {
            hwnd: HWND(hwnd as *mut c_void),
            error_message: format!("{:?}", e),
        }),
    }
}

/// 获取窗口位置尺寸（不包括阴影），相对于屏幕。许多截屏软件获取窗口矩形时，不包括阴影。这个函数得到的窗口大小与截屏软件得到的窗口大小一致。
/// # 返回
/// (x, y, width, height)
pub fn get_window_xywh_exclude_shadow(hwnd: usize) -> Result<(i32, i32, u32, u32)> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    let mut rect = RECT::default();
    match unsafe {
        DwmGetWindowAttribute(
            HWND(hwnd as *mut c_void),
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
        Err(e) => Err(WindowInspectorError::DwmGetWindowAttributeFailed {
            hwnd: HWND(hwnd as *mut c_void),
            error_message: format!("{:?}", e),
        }),
    }
}

/// 获取客户区左上角坐标，相对于屏幕。
/// # 返回
/// (x, y)
pub fn get_client_xy(hwnd: usize) -> Result<(i32, i32)> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    let mut point = POINT::default();
    if !unsafe { ClientToScreen(HWND(hwnd as *mut c_void), &mut point) }.as_bool() {
        return Err(WindowInspectorError::ClientToScreenFailed {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    Ok((point.x, point.y))
}

/// 获取客户区尺寸。
/// # 返回
/// (width, height)
pub fn get_client_wh(hwnd: usize) -> Result<(u32, u32)> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    let mut rect = RECT::default();
    match unsafe { GetClientRect(HWND(hwnd as *mut c_void), &mut rect) } {
        Ok(_) => Ok((
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )),
        Err(e) => Err(WindowInspectorError::GetClientRectFailed {
            hwnd: HWND(hwnd as *mut c_void),
            error_message: format!("{:?}", e),
        }),
    }
}

/// 获取客户区位置尺寸，相对于屏幕。
/// # 返回
/// (x, y, width, height)
pub fn get_client_xywh(hwnd: usize) -> Result<(i32, i32, u32, u32)> {
    let (x, y) = get_client_xy(hwnd)?;
    let (width, height) = get_client_wh(hwnd)?;
    Ok((x, y, width, height))
}

/// 移动窗口到xywh。
pub fn move_window_to_xywh(hwnd: usize, x: i32, y: i32, width: u32, height: u32) -> Result<()> {
    if !is_window_exist(hwnd) {
        return Err(WindowInspectorError::WindowNotExist {
            hwnd: HWND(hwnd as *mut c_void),
        });
    }
    unsafe {
        if let Err(e) = MoveWindow(
            HWND(hwnd as *mut c_void),
            x,
            y,
            width as i32,
            height as i32,
            true,
        ) {
            return Err(WindowInspectorError::MoveWindowFailed {
                hwnd: HWND(hwnd as *mut c_void),
                error_message: format!("{:?}", e),
            });
        }
    }
    Ok(())
}
