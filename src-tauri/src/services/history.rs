use crate::api::{
    client::BiliClient,
    endpoints::{API_HISTORY_CLEAR, API_HISTORY_LIST},
    error::BiliError,
    models::{ApiResult, ClearResult, History},
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 历史记录服务
///
/// 提供历史记录的备份、清空等功能。
/// 注意：B站API不支持还原历史记录。
pub struct HistoryService {
    client: Arc<RwLock<BiliClient>>,
}

/// 历史记录API响应数据结构
#[derive(Debug, Clone, Deserialize)]
struct HistoryCursorData {
    /// 游标信息
    cursor: HistoryCursor,
    /// 历史记录列表
    #[serde(default)]
    list: Vec<History>,
}

/// 历史记录游标
#[derive(Debug, Clone, Deserialize)]
struct HistoryCursor {
    /// 最大历史记录ID
    #[serde(default)]
    max: u64,
    /// 查看时间戳
    #[serde(default)]
    view_at: i64,
}

impl HistoryService {
    /// 创建新的历史记录服务实例
    ///
    /// # 参数
    ///
    /// * `client` - B站API客户端
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::HistoryService;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = HistoryService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份历史记录
    ///
    /// 使用游标分页获取所有历史记录。
    ///
    /// # 返回
    ///
    /// 包含所有历史记录的 Vec<History>
    ///
    /// # 错误
    ///
    /// 如果API请求失败或解析失败，返回 BiliError
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use bilibili_backup_tauri::services::HistoryService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = HistoryService::new(client);
    /// let history = service.backup_history().await?;
    /// println!("备份了 {} 条历史记录", history.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn backup_history(&self) -> Result<Vec<History>, BiliError> {
        let mut all_history = Vec::new();
        let mut max: Option<u64> = None;
        let mut view_at: Option<i64> = None;

        loop {
            // 构建请求URL
            let url = if let (Some(max_val), Some(view_at_val)) = (max, view_at) {
                format!(
                    "{}?max={}&view_at={}",
                    API_HISTORY_LIST, max_val, view_at_val
                )
            } else {
                API_HISTORY_LIST.to_string()
            };

            tracing::info!("获取历史记录: {}", url);

            // 发送请求
            let client = self.client.read().await;
            let response = client.get_with_retry(&url).await?;
            drop(client); // 释放读锁

            // 解析响应
            let result: ApiResult<HistoryCursorData> = response.json().await?;
            let data = result.into_data()?;

            // 如果没有更多数据，退出循环
            if data.list.is_empty() {
                tracing::info!("历史记录获取完成，共 {} 条", all_history.len());
                break;
            }

            // 添加到结果集
            let count = data.list.len();
            all_history.extend(data.list);

            // 更新游标
            max = Some(data.cursor.max);
            view_at = Some(data.cursor.view_at);

            tracing::info!(
                "本次获取 {} 条，累计 {} 条",
                count,
                all_history.len()
            );

            // 防风控延迟
            let client = self.client.read().await;
            client.delay_random().await;
        }

        Ok(all_history)
    }

    /// 清空历史记录
    ///
    /// 清空所有历史记录（不可恢复）。
    ///
    /// # 返回
    ///
    /// 清除操作结果
    ///
    /// # 错误
    ///
    /// 如果API请求失败，返回 BiliError
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use bilibili_backup_tauri::services::HistoryService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = HistoryService::new(client);
    /// let result = service.clear_history().await?;
    /// println!("{}", result.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_history(&self) -> Result<ClearResult, BiliError> {
        tracing::warn!("清空历史记录（不可恢复）");

        let client = self.client.read().await;
        let response = client.post_form_with_retry(API_HISTORY_CLEAR, &[]).await?;
        drop(client);

        let result: ApiResult<()> = response.json().await?;
        result.into_data()?;

        Ok(ClearResult {
            cleared_count: 0, // B站API不返回清除数量
            message: "历史记录已清空".to_string(),
        })
    }

    /// 导出历史记录到JSON文件
    ///
    /// # 参数
    ///
    /// * `history` - 历史记录列表
    /// * `file_path` - 导出文件路径
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use bilibili_backup_tauri::services::HistoryService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = HistoryService::new(client);
    /// let history = service.backup_history().await?;
    /// service.export_to_file(&history, "history.json").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn export_to_file(
        &self,
        history: &[History],
        file_path: &str,
    ) -> Result<(), BiliError> {
        let json = serde_json::to_string_pretty(history)
            .map_err(|e| BiliError::parse(format!("序列化失败: {}", e)))?;

        tokio::fs::write(file_path, json)
            .await
            .map_err(|e| BiliError::io(format!("写入文件失败: {}", e)))?;

        tracing::info!("历史记录已导出到: {}", file_path);
        Ok(())
    }

    /// 从JSON文件导入历史记录
    ///
    /// # 参数
    ///
    /// * `file_path` - 导入文件路径
    ///
    /// # 返回
    ///
    /// 历史记录列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::HistoryService;
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = HistoryService::new(client);
    /// let history = service.import_from_file("history.json").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn import_from_file(&self, file_path: &str) -> Result<Vec<History>, BiliError> {
        let json = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| BiliError::io(format!("读取文件失败: {}", e)))?;

        let history: Vec<History> = serde_json::from_str(&json)
            .map_err(|e| BiliError::parse(format!("反序列化失败: {}", e)))?;

        tracing::info!("从 {} 导入了 {} 条历史记录", file_path, history.len());
        Ok(history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_history_service() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = HistoryService::new(client);
        // 只是测试创建不会panic
    }
}
