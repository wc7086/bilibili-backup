/// API层模块
pub mod api;
/// 业务逻辑层模块
pub mod services;
/// Tauri命令层模块
pub mod commands;
/// 工具函数模块
pub mod utils;

// 重新导出常用类型
pub use api::{BiliClient, BiliError, Result};
