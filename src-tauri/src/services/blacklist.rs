/// 黑名单管理服务
///
/// 提供黑名单的备份、还原、清空等功能
///
/// # 功能特性
///
/// - 备份黑名单
/// - 还原黑名单
/// - 清空黑名单
/// - 批量操作 (避免风控)
/// - 延迟机制 (防风控)

use crate::api::{
    client::BiliClient,
    endpoints::*,
    error::{BiliError, Result},
    models::*,
    pagination::fetch_all_pages,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 黑名单服务
pub struct BlacklistService {
    client: Arc<RwLock<BiliClient>>,
}

/// 黑名单还原选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistRestoreOptions {
    /// 遇到错误是否继续 (false: 立即中断)
    pub continue_on_error: bool,
    /// 批量操作大小 (默认20)
    pub batch_size: usize,
    /// 操作延迟(毫秒) (默认1000-3000)
    pub delay_ms: Option<(u64, u64)>,
}

impl Default for BlacklistRestoreOptions {
    fn default() -> Self {
        Self {
            continue_on_error: false,
            batch_size: 20,
            delay_ms: None,
        }
    }
}

/// 黑名单还原结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistRestoreResult {
    /// 成功数量
    pub success_count: usize,
    /// 失败数量
    pub failed_count: usize,
    /// 失败的用户ID和原因
    pub failures: Vec<(u64, String)>,
}

/// 黑名单清空结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistClearResult {
    /// 成功数量
    pub success_count: usize,
    /// 失败数量
    pub failed_count: usize,
    /// 失败的用户ID和原因
    pub failures: Vec<(u64, String)>,
}

impl BlacklistService {
    /// 创建新的黑名单服务实例
    ///
    /// # 参数
    ///
    /// * `client` - HTTP客户端 (需要已登录)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::blacklist::BlacklistService;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = BlacklistService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份黑名单
    ///
    /// 获取当前用户的所有黑名单用户。
    ///
    /// # 返回
    ///
    /// Result<Vec<User>, BiliError> - 黑名单列表
    ///
    /// # 错误
    ///
    /// - 未登录
    /// - 网络请求失败
    /// - API返回错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::blacklist::BlacklistService;
    /// # async fn example(service: BlacklistService) -> Result<(), Box<dyn std::error::Error>> {
    /// let blacklist = service.backup_blacklist().await?;
    /// println!("备份了 {} 个黑名单用户", blacklist.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn backup_blacklist(&self) -> Result<Vec<User>> {
        let client = self.client.read().await;

        // 分页获取所有黑名单用户
        let blacklist = fetch_all_pages::<User>(&client, API_BLACK_LIST, 50, None).await?;

        tracing::info!("备份了 {} 个黑名单用户", blacklist.len());
        Ok(blacklist)
    }

    /// 还原黑名单
    ///
    /// 根据备份数据还原黑名单。
    ///
    /// # 参数
    ///
    /// * `users` - 备份的黑名单用户列表
    /// * `options` - 还原选项
    ///
    /// # 返回
    ///
    /// Result<BlacklistRestoreResult, BiliError> - 还原结果统计
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::blacklist::{BlacklistService, BlacklistRestoreOptions};
    /// # async fn example(service: BlacklistService, users: Vec<bilibili_backup_tauri::api::models::User>) -> Result<(), Box<dyn std::error::Error>> {
    /// let options = BlacklistRestoreOptions::default();
    /// let result = service.restore_blacklist(users, options).await?;
    /// println!("成功: {}, 失败: {}", result.success_count, result.failed_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_blacklist(
        &self,
        users: Vec<User>,
        options: BlacklistRestoreOptions,
    ) -> Result<BlacklistRestoreResult> {
        let mut client = self.client.write().await;

        // 配置延迟
        if let Some((min_ms, max_ms)) = options.delay_ms {
            *client = client.clone().with_delay_range(min_ms, max_ms);
        }

        // 批量添加到黑名单
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut failures = Vec::new();

        for chunk in users.chunks(options.batch_size) {
            for user in chunk {
                match self.add_to_blacklist(&client, user.mid).await {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        failed_count += 1;
                        let error_msg = format!("{}", e);
                        failures.push((user.mid, error_msg.clone()));
                        tracing::error!("添加用户 {} 到黑名单失败: {}", user.mid, error_msg);

                        if !options.continue_on_error {
                            return Ok(BlacklistRestoreResult {
                                success_count,
                                failed_count,
                                failures,
                            });
                        }
                    }
                }

                // 延迟
                client.delay_random().await;
            }
        }

        Ok(BlacklistRestoreResult {
            success_count,
            failed_count,
            failures,
        })
    }

    /// 清空黑名单
    ///
    /// 移除所有黑名单用户。
    ///
    /// # 返回
    ///
    /// Result<BlacklistClearResult, BiliError> - 清空结果统计
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::blacklist::BlacklistService;
    /// # async fn example(service: BlacklistService) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = service.clear_blacklist().await?;
    /// println!("移除了 {} 个黑名单用户", result.success_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_blacklist(&self) -> Result<BlacklistClearResult> {
        let client = self.client.read().await;

        // 1. 获取所有黑名单
        let users = self.backup_blacklist().await?;

        // 2. 批量移除
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut failures = Vec::new();

        for user in users {
            match self.remove_from_blacklist(&client, user.mid).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    failed_count += 1;
                    failures.push((user.mid, format!("{}", e)));
                }
            }

            // 延迟
            client.delay_random().await;
        }

        Ok(BlacklistClearResult {
            success_count,
            failed_count,
            failures,
        })
    }

    // ==================== 私有辅助方法 ====================

    /// 添加用户到黑名单
    ///
    /// # 参数
    ///
    /// * `fid` - 目标用户ID
    async fn add_to_blacklist(&self, client: &BiliClient, fid: u64) -> Result<()> {
        let form = vec![
            ("fid".to_string(), fid.to_string()),
            ("act".to_string(), "5".to_string()), // 5: 加入黑名单
            ("re_src".to_string(), "11".to_string()),
        ];

        let response = client.post_form_with_retry(API_BLACK_ADD, &form).await?;
        let api_result: ApiResult<serde_json::Value> = response.json().await?;

        if api_result.is_success() {
            Ok(())
        } else {
            Err(BiliError::api(format!(
                "添加黑名单失败 [{}]: {}",
                api_result.code, api_result.message
            )))
        }
    }

    /// 从黑名单移除用户
    ///
    /// # 参数
    ///
    /// * `fid` - 目标用户ID
    async fn remove_from_blacklist(&self, client: &BiliClient, fid: u64) -> Result<()> {
        let form = vec![
            ("fid".to_string(), fid.to_string()),
            ("act".to_string(), "6".to_string()), // 6: 移出黑名单
            ("re_src".to_string(), "11".to_string()),
        ];

        let response = client
            .post_form_with_retry(API_BLACK_REMOVE, &form)
            .await?;
        let api_result: ApiResult<serde_json::Value> = response.json().await?;

        if api_result.is_success() {
            Ok(())
        } else {
            Err(BiliError::api(format!(
                "移除黑名单失败 [{}]: {}",
                api_result.code, api_result.message
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blacklist_restore_options_default() {
        let options = BlacklistRestoreOptions::default();
        assert!(!options.continue_on_error);
        assert_eq!(options.batch_size, 20);
        assert!(options.delay_ms.is_none());
    }

    #[test]
    fn test_blacklist_restore_result() {
        let result = BlacklistRestoreResult {
            success_count: 10,
            failed_count: 2,
            failures: vec![(123, "错误".to_string())],
        };
        assert_eq!(result.success_count, 10);
        assert_eq!(result.failed_count, 2);
        assert_eq!(result.failures.len(), 1);
    }

    #[test]
    fn test_blacklist_clear_result() {
        let result = BlacklistClearResult {
            success_count: 5,
            failed_count: 1,
            failures: vec![(456, "错误".to_string())],
        };
        assert_eq!(result.success_count, 5);
        assert_eq!(result.failed_count, 1);
    }

    #[tokio::test]
    async fn test_blacklist_service_create() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = BlacklistService::new(client);
        assert!(service.client.read().await.get_cookie().is_none());
    }
}
