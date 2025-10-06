use crate::api::{
    client::BiliClient,
    endpoints::{API_BANGUMI_FOLLOW, API_BANGUMI_LIST, API_BANGUMI_UNFOLLOW},
    error::BiliError,
    models::{ApiResult, Bangumi, ClearResult, RestoreResult},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 追番追剧服务
///
/// 提供追番追剧的备份、还原、清空等功能。
pub struct BangumiService {
    client: Arc<RwLock<BiliClient>>,
}

/// 追番列表响应
#[derive(Debug, Clone, Deserialize)]
struct BangumiListResponse {
    /// 列表数据
    list: Vec<Bangumi>,
    /// 总数
    total: usize,
}

impl BangumiService {
    /// 创建新的追番追剧服务实例
    ///
    /// # 参数
    ///
    /// * `client` - B站API客户端
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::BangumiService;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = BangumiService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份追番追剧列表
    ///
    /// # 参数
    ///
    /// * `type_` - 类型 (1:番剧 2:电影 3:纪录片 4:国创 5:电视剧 7:综艺)
    ///
    /// # 返回
    ///
    /// 追番追剧列表
    ///
    /// # 错误
    ///
    /// 如果API请求失败或解析失败，返回 BiliError
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use bilibili_backup_tauri::services::BangumiService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = BangumiService::new(client);
    /// let bangumi_list = service.backup_bangumi(1).await?; // 备份番剧
    /// println!("备份了 {} 个番剧", bangumi_list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn backup_bangumi(&self, type_: i32) -> Result<Vec<Bangumi>, BiliError> {
        let mut all_bangumi = Vec::new();
        let mut pn = 1;
        let ps = 20; // 每页20个

        loop {
            let url = format!(
                "{}?type={}&follow_status=0&pn={}&ps={}",
                API_BANGUMI_LIST, type_, pn, ps
            );

            tracing::info!("获取追番列表 (类型:{}, 页码:{})", type_, pn);

            let client = self.client.read().await;
            let response = client.get_with_retry(&url).await?;
            drop(client);

            // 解析响应
            let result: ApiResult<BangumiListResponse> = response.json().await?;
            let data = result.into_data()?;

            // 如果没有更多数据，退出循环
            if data.list.is_empty() {
                tracing::info!("追番列表获取完成，共 {} 个", all_bangumi.len());
                break;
            }

            let count = data.list.len();
            all_bangumi.extend(data.list);

            tracing::info!(
                "本次获取 {} 个，累计 {} 个",
                count,
                all_bangumi.len()
            );

            // 如果已经获取到所有数据，退出循环
            if all_bangumi.len() >= data.total {
                break;
            }

            pn += 1;

            // 防风控延迟
            let client = self.client.read().await;
            client.delay_random().await;
        }

        Ok(all_bangumi)
    }

    /// 还原追番追剧列表
    ///
    /// # 参数
    ///
    /// * `bangumi_list` - 要还原的追番列表
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
    /// # use bilibili_backup_tauri::services::BangumiService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// # let bangumi_list = vec![];
    /// let service = BangumiService::new(client);
    /// let result = service.restore_bangumi(bangumi_list).await?;
    /// println!("还原成功 {} 个，失败 {} 个", result.success_count, result.failed_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_bangumi(
        &self,
        bangumi_list: Vec<Bangumi>,
    ) -> Result<RestoreResult, BiliError> {
        let total = bangumi_list.len();
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut failed_items = Vec::new();

        tracing::info!("开始还原 {} 个追番", total);

        for bangumi in bangumi_list {
            let result = self.follow_bangumi(bangumi.season_id).await;

            match result {
                Ok(_) => {
                    success_count += 1;
                    tracing::debug!("追番成功: {} (season_id: {})", bangumi.title, bangumi.season_id);
                }
                Err(e) => {
                    failed_count += 1;
                    let error_msg = format!(
                        "{} (season_id: {}): {}",
                        bangumi.title, bangumi.season_id, e
                    );
                    failed_items.push(error_msg.clone());
                    tracing::warn!("追番失败: {}", error_msg);
                }
            }

            // 防风控延迟
            let client = self.client.read().await;
            client.delay_random().await;
        }

        let message = if failed_count == 0 {
            format!("成功还原 {} 个追番", success_count)
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

    /// 清空追番追剧列表
    ///
    /// # 参数
    ///
    /// * `type_` - 类型 (1:番剧 2:电影 3:纪录片 4:国创 5:电视剧 7:综艺)
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
    /// # use bilibili_backup_tauri::services::BangumiService;
    /// # use std::sync::Arc;
    /// # use tokio::sync::RwLock;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = BangumiService::new(client);
    /// let result = service.clear_bangumi(1).await?;
    /// println!("{}", result.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_bangumi(&self, type_: i32) -> Result<ClearResult, BiliError> {
        tracing::warn!("清空追番列表 (类型: {})", type_);

        // 先获取所有追番
        let bangumi_list = self.backup_bangumi(type_).await?;
        let total = bangumi_list.len();

        let mut cleared_count = 0;

        for bangumi in bangumi_list {
            let result = self.unfollow_bangumi(bangumi.season_id).await;

            match result {
                Ok(_) => {
                    cleared_count += 1;
                    tracing::debug!("取消追番: {} (season_id: {})", bangumi.title, bangumi.season_id);
                }
                Err(e) => {
                    tracing::warn!(
                        "取消追番失败: {} (season_id: {}): {}",
                        bangumi.title,
                        bangumi.season_id,
                        e
                    );
                }
            }

            // 防风控延迟
            let client = self.client.read().await;
            client.delay_random().await;
        }

        Ok(ClearResult {
            cleared_count,
            message: format!("已清空 {} 个追番 (共 {} 个)", cleared_count, total),
        })
    }

    /// 追番/追剧
    ///
    /// # 参数
    ///
    /// * `season_id` - 剧集ID
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    async fn follow_bangumi(&self, season_id: u64) -> Result<(), BiliError> {
        let body = json!({
            "season_id": season_id,
        });

        let client = self.client.read().await;
        let response = client
            .post_json_with_retry(API_BANGUMI_FOLLOW, &body)
            .await?;
        drop(client);

        let result: ApiResult<()> = response.json().await?;
        result.into_data()?;

        Ok(())
    }

    /// 取消追番/追剧
    ///
    /// # 参数
    ///
    /// * `season_id` - 剧集ID
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    async fn unfollow_bangumi(&self, season_id: u64) -> Result<(), BiliError> {
        let body = json!({
            "season_id": season_id,
        });

        let client = self.client.read().await;
        let response = client
            .post_json_with_retry(API_BANGUMI_UNFOLLOW, &body)
            .await?;
        drop(client);

        let result: ApiResult<()> = response.json().await?;
        result.into_data()?;

        Ok(())
    }

    /// 导出追番列表到JSON文件
    ///
    /// # 参数
    ///
    /// * `bangumi_list` - 追番列表
    /// * `file_path` - 导出文件路径
    ///
    /// # 返回
    ///
    /// 成功返回 (), 失败返回错误
    pub async fn export_to_file(
        &self,
        bangumi_list: &[Bangumi],
        file_path: &str,
    ) -> Result<(), BiliError> {
        let json = serde_json::to_string_pretty(bangumi_list)
            .map_err(|e| BiliError::parse(format!("序列化失败: {}", e)))?;

        tokio::fs::write(file_path, json)
            .await
            .map_err(|e| BiliError::io(format!("写入文件失败: {}", e)))?;

        tracing::info!("追番列表已导出到: {}", file_path);
        Ok(())
    }

    /// 从JSON文件导入追番列表
    ///
    /// # 参数
    ///
    /// * `file_path` - 导入文件路径
    ///
    /// # 返回
    ///
    /// 追番列表
    pub async fn import_from_file(&self, file_path: &str) -> Result<Vec<Bangumi>, BiliError> {
        let json = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| BiliError::io(format!("读取文件失败: {}", e)))?;

        let bangumi_list: Vec<Bangumi> = serde_json::from_str(&json)
            .map_err(|e| BiliError::parse(format!("反序列化失败: {}", e)))?;

        tracing::info!(
            "从 {} 导入了 {} 个追番",
            file_path,
            bangumi_list.len()
        );
        Ok(bangumi_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bangumi_service() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = BangumiService::new(client);
        // 只是测试创建不会panic
    }
}
