//! 用于获取窗口信息，简单操作窗口。适用于Windows。
//! # 获取信息
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
//! ## 进程
//! - 获取窗口所属进程。[`get_window_process`]
//! - 获取进程路径。[`get_process_path`]
//! - 获取窗口所属进程的路径。[`get_window_process_path`]
//! # 简单操作窗口
//! ## 前台
//! - 设置前台窗口。[`set_foreground_window`]
//! ## 移动
//! - 移动窗口到xywh。[`move_window_to_xywh`]
//! ## 置顶
//! - 设置窗口置顶。[`set_window_top_most`]
//! - 取消窗口置顶。[`cancel_window_top_most`]
//! - 切换窗口置顶状态。[`switch_window_top_most`]
//! # 性能
//! 不建议使用[`get_window_handle`]，因为它每次都会调用[`FindWindowW`]。
//! 建议使用[`get_window_handle_ref_cache`]。
//! 其他函数性能较好。
//!
//! [`FindWindowW`]: https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.FindWindowW.html

pub use error::*;
pub use information::*;
pub use operation::*;

mod error;
mod information;
mod operation;
