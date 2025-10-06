use crate::services::{FavFolderWithMedia, FavRestoreOptions, FavoritesService};
use crate::api::models::RestoreResult;
use tauri::State;

/// 备份收藏夹
///
/// 获取当前用户的所有收藏夹及其内容
///
/// # 返回
///
/// - 成功：返回所有收藏夹及其内容
/// - 失败：返回错误信息字符串
#[tauri::command]
pub async fn backup_favorites(
    service: State<'_, FavoritesService>,
) -> Result<Vec<FavFolderWithMedia>, String> {
    service
        .backup_favorites()
        .await
        .map_err(|e| e.to_string())
}

/// 还原收藏夹
///
/// 将备份的收藏夹还原到当前账号
///
/// # 参数
///
/// * `folders` - 要还原的收藏夹列表
/// * `options` - 还原选项（可选，使用默认值）
///
/// # 返回
///
/// - 成功：返回还原结果统计
/// - 失败：返回错误信息字符串
#[tauri::command]
pub async fn restore_favorites(
    service: State<'_, FavoritesService>,
    folders: Vec<FavFolderWithMedia>,
    options: Option<FavRestoreOptions>,
) -> Result<RestoreResult, String> {
    let options = options.unwrap_or_default();
    service
        .restore_favorites(folders, options)
        .await
        .map_err(|e| e.to_string())
}

/// 清空所有收藏夹
///
/// 清空当前用户所有收藏夹的内容（不删除收藏夹本身）
///
/// # 返回
///
/// - 成功：返回清空的视频总数
/// - 失败：返回错误信息字符串
#[tauri::command]
pub async fn clear_favorites(service: State<'_, FavoritesService>) -> Result<usize, String> {
    service
        .clear_all_folders()
        .await
        .map_err(|e| e.to_string())
}
