use crate::api::{
    client::BiliClient,
    endpoints::{API_TOVIEW_ADD, API_TOVIEW_CLEAR, API_TOVIEW_DEL, API_TOVIEW_LIST},
    error::BiliError,
    models::{ApiResult, ClearResult, RestoreResult, ToView, ToViewList},
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 稍后再看服务
///
/// 提供稍后再看的备份、还原、清空等功能。
pub struct ToViewService {
    client: Arc<RwLock<BiliClient>>,
}

impl ToViewService {
    /// 创建新的稍后再看服务实例
    ///
    /// # 参数
    ///
    /// * `client` - B站API客户端
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::ToViewService;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = ToViewService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份稍后再看列表
    ///
    /// # 返回
    ///
    /// 稍后再看视频列表
    ///
    /// # 错误
    ///
    /// 如果API请求失败或解析失败，返回 BiliError
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use bilibili_backup_tauri::services::ToViewService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = ToViewService::new(client);
    /// let toview_list = service.backup_toview().await?;
    /// println!("备份了 {} 个稍后再看", toview_list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn backup_toview(&self) -> Result<Vec<ToView>, BiliError> {
        tracing::info!("获取稍后再看列表");

        let client = self.client.read().await;
        let response = client.get_with_retry(API_TOVIEW_LIST).await?;
        drop(client);

        // 解析响应
        let result: ApiResult<ToViewList> = response.json().await?;
        let data = result.into_data()?;

        tracing::info!("获取到 {} 个稍后再看", data.list.len());
        Ok(data.list)
    }

    /// 还原稍后再看列表
    ///
    /// # 参数
    ///
    /// * `videos` - 要还原的视频列表
    ///
    /// # 返回
    ///
    /// 还原操作结果
    ///
    /// # 错误
    ///
    /// 如果API请求失败，返回 BiliError
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use bilibili_backup_tauri::services::ToViewService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// # let videos = vec![];
    /// let service = ToViewService::new(client);
    /// let result = service.restore_toview(videos).await?;
    /// println!("还原成功 {} 个，失败 {} 个", result.success_count, result.failed_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_toview(&self, videos: Vec<ToView>) -> Result<RestoreResult, BiliError> {
        let total = videos.len();
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut failed_items = Vec::new();

        tracing::info!("开始还原 {} 个稍后再看", total);

        for video in videos {
            let result = self.add_toview(video.aid).await;

            match result {
                Ok(_) => {
                    success_count += 1;
                    tracing::debug!("添加成功: {} (aid: {})", video.title, video.aid);
                }
                Err(e) => {
                    failed_count += 1;
                    let error_msg = format!("{} (aid: {}): {}", video.title, video.aid, e);
                    failed_items.push(error_msg.clone());
                    tracing::warn!("添加失败: {}", error_msg);
                }
            }

            // 防风控延迟
            let client = self.client.read().await;
            client.delay_random().await;
        }

        let message = if failed_count == 0 {
            format!("成功还原 {} 个稍后再看", success_count)
        } else {
            format!(
                "还原完成: 成功 {} 个，失败 {} 个",
                success_count, failed_count
            )
        };

        Ok(RestoreResult {
            success_count,
            failed_count,
            total_count: total,
            failed_items: if failed_items.is_empty() {
                None
            } else {
                Some(failed_items)
            },
            message,
        })
    }

    /// 清空稍后再看
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
    /// # use bilibili_backup_tauri::services::ToViewService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = ToViewService::new(client);
    /// let result = service.clear_toview().await?;
    /// println!("{}", result.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_toview(&self) -> Result<ClearResult, BiliError> {
        tracing::warn!("清空稍后再看");

        let client = self.client.read().await;
        let response = client.post_form_with_retry(API_TOVIEW_CLEAR, &[]).await?;
        drop(client);

        let result: ApiResult<()> = response.json().await?;
        result.into_data()?;

        Ok(ClearResult {
            cleared_count: 0, // B站API不返回清除数量
            message: "稍后再看已清空".to_string(),
        })
    }

    /// 添加视频到稍后再看
    ///
    /// # 参数
    ///
    /// * `aid` - 稿件ID
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    async fn add_toview(&self, aid: u64) -> Result<(), BiliError> {
        let form = vec![
            ("aid".to_string(), aid.to_string()),
        ];

        let client = self.client.read().await;
        let response = client.post_form_with_retry(API_TOVIEW_ADD, &form).await?;
        drop(client);

        let result: ApiResult<()> = response.json().await?;
        result.into_data()?;

        Ok(())
    }

    /// 删除稍后再看
    ///
    /// # 参数
    ///
    /// * `aids` - 稿件ID列表
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    #[allow(dead_code)]
    async fn delete_toview(&self, aids: Vec<u64>) -> Result<(), BiliError> {
        let aids_str = aids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = vec![
            ("aid".to_string(), aids_str),
        ];

        let client = self.client.read().await;
        let response = client.post_form_with_retry(API_TOVIEW_DEL, &form).await?;
        drop(client);

        let result: ApiResult<()> = response.json().await?;
        result.into_data()?;

        Ok(())
    }

    /// 导出稍后再看列表到JSON文件
    ///
    /// # 参数
    ///
    /// * `videos` - 视频列表
    /// * `file_path` - 导出文件路径
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    pub async fn export_to_file(
        &self,
        videos: &[ToView],
        file_path: &str,
    ) -> Result<(), BiliError> {
        let json = serde_json::to_string_pretty(videos)
            .map_err(|e| BiliError::parse(format!("序列化失败: {}", e)))?;

        tokio::fs::write(file_path, json)
            .await
            .map_err(|e| BiliError::io(format!("写入文件失败: {}", e)))?;

        tracing::info!("稍后再看列表已导出到: {}", file_path);
        Ok(())
    }

    /// 从JSON文件导入稍后再看列表
    ///
    /// # 参数
    ///
    /// * `file_path` - 导入文件路径
    ///
    /// # 返回
    ///
    /// 视频列表
    pub async fn import_from_file(&self, file_path: &str) -> Result<Vec<ToView>, BiliError> {
        let json = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| BiliError::io(format!("读取文件失败: {}", e)))?;

        let videos: Vec<ToView> = serde_json::from_str(&json)
            .map_err(|e| BiliError::parse(format!("反序列化失败: {}", e)))?;

        tracing::info!("从 {} 导入了 {} 个稍后再看", file_path, videos.len());
        Ok(videos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_toview_service() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = ToViewService::new(client);
        // 只是测试创建不会panic
    }
}
