use windows::core::PWSTR;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_INFORMATION,
    PROCESS_VM_READ,
};
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

use crate::error::Error;

/// 获取窗口所属进程。
pub fn get_window_process(hwnd: isize) -> Result<u32, Error> {
    let mut process_id = 0;
    if unsafe { GetWindowThreadProcessId(HWND(hwnd), Some(&mut process_id)) } == 0 {
        return Err(Error::Win32ApiFailed {
            api_name: "GetWindowThreadProcessId".to_string(),
            error_code: unsafe { GetLastError() }.0.into(),
            message: format!("hwnd: 0x{:X}", hwnd),
        });
    }
    Ok(process_id)
}

/// 获取进程路径。
pub fn get_process_path(process_id: u32) -> Result<String, Error> {
    let process_handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )
    }
    .map_err(|e| Error::Win32ApiFailed {
        api_name: "OpenProcess".to_string(),
        error_code: (e.code().0 as u32).into(),
        message: format!("process id: 0x{:X}", process_id),
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
        Err(e) => Err(Error::Win32ApiFailed {
            api_name: "QueryFullProcessImageNameW".to_string(),
            error_code: (e.code().0 as u32).into(),
            message: format!("process id: {}", process_id),
        }),
    }
}

/// 获取窗口所属进程的路径。
pub fn get_window_process_path(hwnd: isize) -> Result<String, Error> {
    get_process_path(get_window_process(hwnd)?)
}
