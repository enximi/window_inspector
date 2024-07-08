use std::ffi::c_void;

use windows::core::PWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::QueryFullProcessImageNameW;
use windows::Win32::System::Threading::PROCESS_NAME_FORMAT;
use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
use windows::Win32::System::Threading::PROCESS_VM_READ;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

use crate::error::WindowInspectorError;
use crate::result::Result;

/// 获取窗口所属进程。
pub fn get_window_process(hwnd: isize) -> Result<u32> {
    let mut process_id = 0;
    if unsafe { GetWindowThreadProcessId(HWND(hwnd as *mut c_void), Some(&mut process_id)) } == 0 {
        return Err(WindowInspectorError::GetWindowThreadProcessIdFailed {
            error_code: unsafe { GetLastError() }.0,
        });
    }
    Ok(process_id)
}

/// 获取进程路径。
pub fn get_process_path(process_id: u32) -> Result<String> {
    let process_handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )
    }
    .map_err(|e| WindowInspectorError::OpenProcessFailed {
        process_id,
        error_message: format!("{}", e),
    })?;

    let mut buffer = [0u16; 1024];
    let pwstr = PWSTR(buffer.as_mut_ptr());
    let mut buffer_size = 1024;
    match unsafe {
        QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_FORMAT(0),
            pwstr,
            &mut buffer_size,
        )
    } {
        Ok(_) => Ok(unsafe { pwstr.to_string() }.unwrap()),
        Err(e) => Err(WindowInspectorError::QueryFullProcessImageNameWFailed {
            process_id,
            error_message: format!("{}", e),
        }),
    }
}

/// 获取窗口所属进程的路径。
pub fn get_window_process_path(hwnd: isize) -> Result<String> {
    get_process_path(get_window_process(hwnd)?)
}
