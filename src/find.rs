use std::num::NonZeroUsize;
use std::ptr::null;
use std::sync::Mutex;

use lazy_static::lazy_static;
use lru::LruCache;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::FindWindowExW;

use crate::error::WindowInspectorError;
use crate::exist::is_window_exist;
use crate::result::Result;

/// 获取窗口句柄。
/// 是[`FindWindowExW`]的封装。
/// 与[`FindWindowExW`]不同的是，[`get_hwnd`]不允许两个参数同时为空。
/// 如果两个参数同时为空，将返回[`Error::WindowClassTitleBothEmpty`]。
/// 性能较差，建议使用[`get_hwnd_ref_cache`]。
///
/// [`FindWindowW`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.FindWindowExW.html
pub fn get_hwnd(window_class: &str, window_title: &str) -> Result<usize> {
    if window_class.is_empty() && window_title.is_empty() {
        return Err(WindowInspectorError::WindowClassTitleBothEmpty);
    }
    fn str_to_pcwstr(s: &str) -> PCWSTR {
        if s.is_empty() {
            PCWSTR(null())
        } else {
            let v: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
            PCWSTR(v.as_ptr())
        }
    }
    match unsafe {
        FindWindowExW(
            None,
            None,
            str_to_pcwstr(window_class),
            str_to_pcwstr(window_title),
        )
    } {
        Ok(hwnd) => Ok(hwnd.0 as usize),
        Err(e) => Err(WindowInspectorError::FindWindowExWFailed {
            window_class: window_class.to_string(),
            window_title: window_title.to_string(),
            error_message: format!("{:?}", e),
        }),
    }
}

lazy_static! {
    static ref HWND_CACHE: Mutex<LruCache<(String, String), usize>> =
        Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap()));
}

/// 获取窗口句柄，参考缓存。
/// # 可能不符合预期的行为
/// 调用该函数成功找到窗口一次之后，如果窗口标题改变，但是还使用原先的参数调用该函数，将依然返回原先的窗口句柄。
/// 因为缓存中有窗口句柄且窗口仍然存在。
pub fn get_hwnd_ref_cache(window_class: &str, window_title: &str) -> Result<usize> {
    if window_class.is_empty() && window_title.is_empty() {
        return Err(WindowInspectorError::WindowClassTitleBothEmpty);
    }
    let key = (window_class.to_string(), window_title.to_string());
    let hwnd = HWND_CACHE.lock().unwrap().get(&key).copied();
    if hwnd.is_some_and(is_window_exist) {
        Ok(hwnd.unwrap())
    } else {
        HWND_CACHE.lock().unwrap().pop(&key);
        let hwnd = get_hwnd(window_class, window_title)?;
        HWND_CACHE.lock().unwrap().put(key, hwnd);
        Ok(hwnd)
    }
}

#[test]
fn test_get_hwnd() {
    for _ in 0..1000 {
        let hwnd = get_hwnd("", "无标题").unwrap();
        assert!(is_window_exist(hwnd));
    }
}
