use std::collections::HashMap;
use std::ptr::null;
use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

use crate::error::Error;
use crate::exist::is_window_exist;

pub struct HwndGetterWithCache {
    cache: HashMap<(String, String), isize>,
}

impl HwndGetterWithCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 获取窗口句柄，参考缓存。
    pub fn get_hwnd(&mut self, window_class: &str, window_title: &str) -> Result<isize, Error> {
        if window_class.is_empty() && window_title.is_empty() {
            return Err(Error::WindowClassTitleBothEmpty);
        }
        let key = (window_class.to_string(), window_title.to_string());
        let hwnd = self.cache.get(&key);
        if hwnd.is_some_and(|hwnd| is_window_exist(*hwnd)) {
            Ok(*hwnd.unwrap())
        } else {
            self.cache.remove(&key);
            let hwnd = get_hwnd_ref_cache(window_class, window_title)?;
            self.cache.insert(key, hwnd);
            Ok(hwnd)
        }
    }
}

impl Default for HwndGetterWithCache {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取窗口句柄。
/// 是[`FindWindowW`]的封装。
/// 与[`FindWindowW`]不同的是，[`get_hwnd`]不允许两个参数同时为空。
/// 如果两个参数同时为空，将返回[`Error::WindowClassTitleBothEmpty`]。
/// 性能较差，建议使用[`get_hwnd_ref_cache`]。
///
/// [`FindWindowW`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.FindWindowW.html
pub fn get_hwnd(window_class: &str, window_title: &str) -> Result<isize, Error> {
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
        hwnd => Ok(hwnd),
    }
}

lazy_static! {
    static ref HWND_CACHE: Mutex<HashMap<(String, String), isize>> = Mutex::new(HashMap::new());
}

/// 获取窗口句柄，参考缓存。
/// # 可能不符合预期的行为
/// 调用该函数成功找到窗口一次之后，如果窗口标题改变，但是还使用原先的参数调用该函数，将依然返回原先的窗口句柄。
/// 因为缓存中有窗口句柄且窗口仍然存在。
pub fn get_hwnd_ref_cache(window_class: &str, window_title: &str) -> Result<isize, Error> {
    if window_class.is_empty() && window_title.is_empty() {
        return Err(Error::WindowClassTitleBothEmpty);
    }
    let mut cache = HWND_CACHE.lock().unwrap();
    let key = (window_class.to_string(), window_title.to_string());
    let hwnd = cache.get(&key);
    if hwnd.is_some_and(|hwnd| is_window_exist(*hwnd)) {
        Ok(*hwnd.unwrap())
    } else {
        cache.remove(&key);
        let hwnd = get_hwnd(window_class, window_title)?;
        cache.insert(key, hwnd);
        Ok(hwnd)
    }
}
