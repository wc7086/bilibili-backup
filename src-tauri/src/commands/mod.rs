use tauri::State;

/// 认证相关命令
pub mod auth;

/// 关注管理相关命令
pub mod following;

/// 收藏夹管理相关命令
pub mod favorites;

/// 历史记录、追番追剧、稍后再看相关命令
pub mod history;

/// Tauri命令示例：打招呼
///
/// 这是一个简单的示例命令，用于验证前后端通信是否正常。
#[tauri::command]
pub async fn greet(name: String) -> Result<String, String> {
    Ok(format!("你好, {}! 欢迎使用B站备份工具", name))
}

/// Tauri命令示例：获取版本信息
#[tauri::command]
pub async fn get_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

// 重新导出命令
pub use auth::*;
pub use following::*;
pub use favorites::*;
pub use history::*;
