use crate::api::models::{Bangumi, ClearResult, History, RestoreResult, ToView};
use crate::services::{BangumiService, HistoryService, ToViewService};
use tauri::State;

// ==================== 历史记录命令 ====================

/// 备份历史记录
///
/// # 返回
///
/// 成功返回历史记录列表，失败返回错误信息
#[tauri::command]
pub async fn backup_history(
    service: State<'_, HistoryService>,
) -> Result<Vec<History>, String> {
    service
        .backup_history()
        .await
        .map_err(|e| format!("备份历史记录失败: {}", e))
}

/// 清空历史记录
///
/// # 返回
///
/// 成功返回清除结果，失败返回错误信息
#[tauri::command]
pub async fn clear_history(
    service: State<'_, HistoryService>,
) -> Result<ClearResult, String> {
    service
        .clear_history()
        .await
        .map_err(|e| format!("清空历史记录失败: {}", e))
}

/// 导出历史记录到文件
///
/// # 参数
///
/// * `history` - 历史记录列表
/// * `file_path` - 导出文件路径
///
/// # 返回
///
/// 成功返回空，失败返回错误信息
#[tauri::command]
pub async fn export_history(
    service: State<'_, HistoryService>,
    history: Vec<History>,
    file_path: String,
) -> Result<(), String> {
    service
        .export_to_file(&history, &file_path)
        .await
        .map_err(|e| format!("导出历史记录失败: {}", e))
}

/// 从文件导入历史记录
///
/// # 参数
///
/// * `file_path` - 导入文件路径
///
/// # 返回
///
/// 成功返回历史记录列表，失败返回错误信息
#[tauri::command]
pub async fn import_history(
    service: State<'_, HistoryService>,
    file_path: String,
) -> Result<Vec<History>, String> {
    service
        .import_from_file(&file_path)
        .await
        .map_err(|e| format!("导入历史记录失败: {}", e))
}

// ==================== 追番追剧命令 ====================

/// 备份追番追剧列表
///
/// # 参数
///
/// * `type_` - 类型 (1:番剧 2:电影 3:纪录片 4:国创 5:电视剧 7:综艺)
///
/// # 返回
///
/// 成功返回追番列表，失败返回错误信息
#[tauri::command]
pub async fn backup_bangumi(
    service: State<'_, BangumiService>,
    type_: i32,
) -> Result<Vec<Bangumi>, String> {
    service
        .backup_bangumi(type_)
        .await
        .map_err(|e| format!("备份追番列表失败: {}", e))
}

/// 还原追番追剧列表
///
/// # 参数
///
/// * `bangumi_list` - 追番列表
///
/// # 返回
///
/// 成功返回还原结果，失败返回错误信息
#[tauri::command]
pub async fn restore_bangumi(
    service: State<'_, BangumiService>,
    bangumi_list: Vec<Bangumi>,
) -> Result<RestoreResult, String> {
    service
        .restore_bangumi(bangumi_list)
        .await
        .map_err(|e| format!("还原追番列表失败: {}", e))
}

/// 清空追番追剧列表
///
/// # 参数
///
/// * `type_` - 类型 (1:番剧 2:电影 3:纪录片 4:国创 5:电视剧 7:综艺)
///
/// # 返回
///
/// 成功返回清除结果，失败返回错误信息
#[tauri::command]
pub async fn clear_bangumi(
    service: State<'_, BangumiService>,
    type_: i32,
) -> Result<ClearResult, String> {
    service
        .clear_bangumi(type_)
        .await
        .map_err(|e| format!("清空追番列表失败: {}", e))
}

/// 导出追番列表到文件
///
/// # 参数
///
/// * `bangumi_list` - 追番列表
/// * `file_path` - 导出文件路径
///
/// # 返回
///
/// 成功返回空，失败返回错误信息
#[tauri::command]
pub async fn export_bangumi(
    service: State<'_, BangumiService>,
    bangumi_list: Vec<Bangumi>,
    file_path: String,
) -> Result<(), String> {
    service
        .export_to_file(&bangumi_list, &file_path)
        .await
        .map_err(|e| format!("导出追番列表失败: {}", e))
}

/// 从文件导入追番列表
///
/// # 参数
///
/// * `file_path` - 导入文件路径
///
/// # 返回
///
/// 成功返回追番列表，失败返回错误信息
#[tauri::command]
pub async fn import_bangumi(
    service: State<'_, BangumiService>,
    file_path: String,
) -> Result<Vec<Bangumi>, String> {
    service
        .import_from_file(&file_path)
        .await
        .map_err(|e| format!("导入追番列表失败: {}", e))
}

// ==================== 稍后再看命令 ====================

/// 备份稍后再看列表
///
/// # 返回
///
/// 成功返回稍后再看列表，失败返回错误信息
#[tauri::command]
pub async fn backup_toview(
    service: State<'_, ToViewService>,
) -> Result<Vec<ToView>, String> {
    service
        .backup_toview()
        .await
        .map_err(|e| format!("备份稍后再看失败: {}", e))
}

/// 还原稍后再看列表
///
/// # 参数
///
/// * `videos` - 视频列表
///
/// # 返回
///
/// 成功返回还原结果，失败返回错误信息
#[tauri::command]
pub async fn restore_toview(
    service: State<'_, ToViewService>,
    videos: Vec<ToView>,
) -> Result<RestoreResult, String> {
    service
        .restore_toview(videos)
        .await
        .map_err(|e| format!("还原稍后再看失败: {}", e))
}

/// 清空稍后再看
///
/// # 返回
///
/// 成功返回清除结果，失败返回错误信息
#[tauri::command]
pub async fn clear_toview(
    service: State<'_, ToViewService>,
) -> Result<ClearResult, String> {
    service
        .clear_toview()
        .await
        .map_err(|e| format!("清空稍后再看失败: {}", e))
}

/// 导出稍后再看列表到文件
///
/// # 参数
///
/// * `videos` - 视频列表
/// * `file_path` - 导出文件路径
///
/// # 返回
///
/// 成功返回空，失败返回错误信息
#[tauri::command]
pub async fn export_toview(
    service: State<'_, ToViewService>,
    videos: Vec<ToView>,
    file_path: String,
) -> Result<(), String> {
    service
        .export_to_file(&videos, &file_path)
        .await
        .map_err(|e| format!("导出稍后再看失败: {}", e))
}

/// 从文件导入稍后再看列表
///
/// # 参数
///
/// * `file_path` - 导入文件路径
///
/// # 返回
///
/// 成功返回视频列表，失败返回错误信息
#[tauri::command]
pub async fn import_toview(
    service: State<'_, ToViewService>,
    file_path: String,
) -> Result<Vec<ToView>, String> {
    service
        .import_from_file(&file_path)
        .await
        .map_err(|e| format!("导入稍后再看失败: {}", e))
}
