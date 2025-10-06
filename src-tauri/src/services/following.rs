/// 关注管理服务
///
/// 提供关注列表的备份、还原、清空等功能
///
/// # 功能特性
///
/// - 备份关注列表 (包含分组信息)
/// - 还原关注列表 (自动映射分组ID)
/// - 清空关注列表
/// - 关注分组管理 (创建、查询)
/// - 批量操作 (每次20个用户, 避免风控)
/// - 延迟机制 (每操作后延迟1-3秒)
/// - 错误处理 (部分失败可选择继续或中断)

use crate::api::{
    client::BiliClient,
    endpoints::*,
    error::{BiliError, Result},
    models::*,
    pagination::fetch_all_pages,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 关注服务
pub struct FollowingService {
    client: Arc<RwLock<BiliClient>>,
}

/// 还原选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOptions {
    /// 是否创建缺失的分组
    pub create_missing_tags: bool,
    /// 遇到错误是否继续 (false: 立即中断)
    pub continue_on_error: bool,
    /// 批量操作大小 (默认20)
    pub batch_size: usize,
    /// 操作延迟(毫秒) (默认1000-3000)
    pub delay_ms: Option<(u64, u64)>,
}

impl Default for RestoreOptions {
    fn default() -> Self {
        Self {
            create_missing_tags: true,
            continue_on_error: false,
            batch_size: 20,
            delay_ms: None,
        }
    }
}

/// 关注还原结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowingRestoreResult {
    /// 成功数量
    pub success_count: usize,
    /// 失败数量
    pub failed_count: usize,
    /// 失败的用户ID和原因
    pub failures: Vec<(u64, String)>,
    /// 分组映射 (旧ID -> 新ID)
    pub tag_mapping: HashMap<i64, i64>,
}

/// 关注清空结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowingClearResult {
    /// 成功数量
    pub success_count: usize,
    /// 失败数量
    pub failed_count: usize,
    /// 失败的用户ID和原因
    pub failures: Vec<(u64, String)>,
}

impl FollowingService {
    /// 创建新的关注服务实例
    ///
    /// # 参数
    ///
    /// * `client` - HTTP客户端 (需要已登录)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::following::FollowingService;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = FollowingService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份关注列表
    ///
    /// 获取当前用户的所有关注UP主,包含分组信息。
    ///
    /// # 返回
    ///
    /// Result<Vec<Relation>, BiliError> - 关注列表
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
    /// # use bilibili_backup_tauri::services::following::FollowingService;
    /// # async fn example(service: FollowingService) -> Result<(), Box<dyn std::error::Error>> {
    /// let relations = service.backup_following().await?;
    /// println!("备份了 {} 个关注", relations.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn backup_following(&self) -> Result<Vec<Relation>> {
        let client = self.client.read().await;

        // 1. 获取当前用户信息
        let nav_info = self.get_user_info(&client).await?;
        let mid = nav_info.mid.ok_or_else(|| BiliError::auth("未登录"))?;

        // 2. 分页获取所有关注
        let base_url = format!("{}?vmid={}&order=attention", API_FOLLOWING_LIST, mid);
        let relations = fetch_all_pages::<Relation>(&client, &base_url, 50, None).await?;

        tracing::info!("备份了 {} 个关注", relations.len());
        Ok(relations)
    }

    /// 还原关注列表
    ///
    /// 根据备份数据还原关注关系和分组。
    /// 会自动建立分组映射,处理批量操作和延迟。
    ///
    /// # 参数
    ///
    /// * `relations` - 备份的关注列表
    /// * `options` - 还原选项
    ///
    /// # 返回
    ///
    /// Result<FollowingRestoreResult, BiliError> - 还原结果统计
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::following::{FollowingService, RestoreOptions};
    /// # async fn example(service: FollowingService, relations: Vec<bilibili_backup_tauri::api::models::Relation>) -> Result<(), Box<dyn std::error::Error>> {
    /// let options = RestoreOptions::default();
    /// let result = service.restore_following(relations, options).await?;
    /// println!("成功: {}, 失败: {}", result.success_count, result.failed_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn restore_following(
        &self,
        relations: Vec<Relation>,
        options: RestoreOptions,
    ) -> Result<FollowingRestoreResult> {
        let mut client = self.client.write().await;

        // 配置延迟
        if let Some((min_ms, max_ms)) = options.delay_ms {
            *client = client
                .clone()
                .with_delay_range(min_ms, max_ms);
        }

        // 1. 获取所有旧分组
        let old_tags: Vec<RelationTag> = relations
            .iter()
            .filter_map(|r| r.tag.as_ref())
            .flatten()
            .map(|&tag_id| RelationTag::new(tag_id, format!("Tag_{}", tag_id)))
            .collect();

        // 去重
        let mut seen_tags = std::collections::HashSet::new();
        let old_tags: Vec<RelationTag> = old_tags
            .into_iter()
            .filter(|tag| seen_tags.insert(tag.tag_id))
            .collect();

        // 2. 建立分组映射
        let tag_mapping = if options.create_missing_tags && !old_tags.is_empty() {
            self.build_tag_mapping(&client, old_tags).await?
        } else {
            HashMap::new()
        };

        // 3. 批量关注
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut failures = Vec::new();

        for chunk in relations.chunks(options.batch_size) {
            for relation in chunk {
                // 关注用户
                match self.follow_user(&client, relation.mid).await {
                    Ok(_) => {
                        success_count += 1;

                        // 设置分组
                        if let Some(ref tag_ids) = relation.tag {
                            let new_tag_ids: Vec<i64> = tag_ids
                                .iter()
                                .filter_map(|old_id| tag_mapping.get(old_id).copied())
                                .collect();

                            if !new_tag_ids.is_empty() {
                                if let Err(e) = self
                                    .add_users_to_tags(&client, vec![relation.mid], new_tag_ids)
                                    .await
                                {
                                    tracing::warn!(
                                        "设置用户 {} 分组失败: {}",
                                        relation.mid,
                                        e
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        failed_count += 1;
                        let error_msg = format!("{}", e);
                        failures.push((relation.mid, error_msg.clone()));
                        tracing::error!("关注用户 {} 失败: {}", relation.mid, error_msg);

                        if !options.continue_on_error {
                            return Ok(FollowingRestoreResult {
                                success_count,
                                failed_count,
                                failures,
                                tag_mapping,
                            });
                        }
                    }
                }

                // 延迟
                client.delay_random().await;
            }
        }

        Ok(FollowingRestoreResult {
            success_count,
            failed_count,
            failures,
            tag_mapping,
        })
    }

    /// 清空关注列表
    ///
    /// 取消所有关注。
    ///
    /// # 返回
    ///
    /// Result<FollowingClearResult, BiliError> - 清空结果统计
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::following::FollowingService;
    /// # async fn example(service: FollowingService) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = service.clear_following().await?;
    /// println!("取消了 {} 个关注", result.success_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_following(&self) -> Result<FollowingClearResult> {
        let client = self.client.read().await;

        // 1. 获取所有关注
        let relations = self.backup_following().await?;

        // 2. 批量取消关注
        let mut success_count = 0;
        let mut failed_count = 0;
        let mut failures = Vec::new();

        for relation in relations {
            match self.unfollow_user(&client, relation.mid).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    failed_count += 1;
                    failures.push((relation.mid, format!("{}", e)));
                }
            }

            // 延迟
            client.delay_random().await;
        }

        Ok(FollowingClearResult {
            success_count,
            failed_count,
            failures,
        })
    }

    /// 获取关注分组列表
    ///
    /// # 返回
    ///
    /// Result<Vec<RelationTag>, BiliError> - 分组列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::following::FollowingService;
    /// # async fn example(service: FollowingService) -> Result<(), Box<dyn std::error::Error>> {
    /// let tags = service.get_relation_tags().await?;
    /// for tag in tags {
    ///     println!("分组: {} (ID: {})", tag.name, tag.tag_id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_relation_tags(&self) -> Result<Vec<RelationTag>> {
        let client = self.client.read().await;
        let response = client.get_with_retry(API_RELATION_TAGS).await?;
        let api_result: ApiResult<Vec<RelationTag>> = response.json().await?;
        api_result.into_data()
    }

    /// 创建关注分组
    ///
    /// # 参数
    ///
    /// * `tag_name` - 分组名称
    ///
    /// # 返回
    ///
    /// Result<i64, BiliError> - 新分组的ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::following::FollowingService;
    /// # async fn example(service: FollowingService) -> Result<(), Box<dyn std::error::Error>> {
    /// let tag_id = service.create_tag("我的分组").await?;
    /// println!("创建分组成功, ID: {}", tag_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_tag(&self, tag_name: &str) -> Result<i64> {
        let client = self.client.read().await;

        let form = vec![("tag".to_string(), tag_name.to_string())];
        let response = client.post_form_with_retry(API_TAG_CREATE, &form).await?;

        // 解析响应 (B站返回的是 {code: 0, message: "ok", data: {tagid: 123}})
        let api_result: ApiResult<serde_json::Value> = response.json().await?;
        let data = api_result.into_data()?;

        let tag_id = data["tagid"]
            .as_i64()
            .ok_or_else(|| BiliError::parse("无法解析分组ID"))?;

        Ok(tag_id)
    }

    // ==================== 私有辅助方法 ====================

    /// 获取用户信息
    async fn get_user_info(&self, client: &BiliClient) -> Result<NavInfo> {
        let response = client.get_with_retry(API_NAV).await?;
        let api_result: ApiResult<NavInfo> = response.json().await?;
        api_result.into_data()
    }

    /// 关注用户
    ///
    /// # 参数
    ///
    /// * `fid` - 目标用户ID
    async fn follow_user(&self, client: &BiliClient, fid: u64) -> Result<()> {
        let form = vec![
            ("fid".to_string(), fid.to_string()),
            ("act".to_string(), "1".to_string()), // 1: 关注
            ("re_src".to_string(), "11".to_string()),
        ];

        let response = client.post_form_with_retry(API_RELATION_MODIFY, &form).await?;
        let api_result: ApiResult<serde_json::Value> = response.json().await?;

        if api_result.is_success() {
            Ok(())
        } else {
            Err(BiliError::api(format!(
                "关注用户失败 [{}]: {}",
                api_result.code, api_result.message
            )))
        }
    }

    /// 取消关注用户
    ///
    /// # 参数
    ///
    /// * `fid` - 目标用户ID
    async fn unfollow_user(&self, client: &BiliClient, fid: u64) -> Result<()> {
        let form = vec![
            ("fid".to_string(), fid.to_string()),
            ("act".to_string(), "2".to_string()), // 2: 取消关注
            ("re_src".to_string(), "11".to_string()),
        ];

        let response = client.post_form_with_retry(API_RELATION_MODIFY, &form).await?;
        let api_result: ApiResult<serde_json::Value> = response.json().await?;

        if api_result.is_success() {
            Ok(())
        } else {
            Err(BiliError::api(format!(
                "取消关注失败 [{}]: {}",
                api_result.code, api_result.message
            )))
        }
    }

    /// 将用户添加到分组
    ///
    /// # 参数
    ///
    /// * `fids` - 用户ID列表
    /// * `tag_ids` - 分组ID列表
    async fn add_users_to_tags(
        &self,
        client: &BiliClient,
        fids: Vec<u64>,
        tag_ids: Vec<i64>,
    ) -> Result<()> {
        let fids_str = fids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let tag_ids_str = tag_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = vec![
            ("fids".to_string(), fids_str),
            ("tagids".to_string(), tag_ids_str),
        ];

        let response = client
            .post_form_with_retry(API_TAG_ADD_USERS, &form)
            .await?;
        let api_result: ApiResult<serde_json::Value> = response.json().await?;

        if api_result.is_success() {
            Ok(())
        } else {
            Err(BiliError::api(format!(
                "添加分组失败 [{}]: {}",
                api_result.code, api_result.message
            )))
        }
    }

    /// 建立分组映射 (旧ID -> 新ID)
    ///
    /// 1. 获取当前账号的所有分组
    /// 2. 尝试根据名称匹配
    /// 3. 如果没有匹配,则创建新分组
    ///
    /// # 参数
    ///
    /// * `old_tags` - 旧分组列表
    ///
    /// # 返回
    ///
    /// Result<HashMap<i64, i64>, BiliError> - 分组映射 (旧ID -> 新ID)
    async fn build_tag_mapping(
        &self,
        client: &BiliClient,
        old_tags: Vec<RelationTag>,
    ) -> Result<HashMap<i64, i64>> {
        let mut mapping = HashMap::new();

        // 获取当前分组列表
        let response = client.get_with_retry(API_RELATION_TAGS).await?;
        let api_result: ApiResult<Vec<RelationTag>> = response.json().await?;
        let current_tags = api_result.into_data()?;

        // 建立名称到ID的映射
        let name_to_id: HashMap<String, i64> = current_tags
            .iter()
            .map(|tag| (tag.name.clone(), tag.tag_id))
            .collect();

        for old_tag in old_tags {
            // 尝试根据名称匹配
            if let Some(&new_id) = name_to_id.get(&old_tag.name) {
                mapping.insert(old_tag.tag_id, new_id);
                tracing::info!("分组映射: {} ({} -> {})", old_tag.name, old_tag.tag_id, new_id);
            } else {
                // 创建新分组
                match self.create_tag(&old_tag.name).await {
                    Ok(new_id) => {
                        mapping.insert(old_tag.tag_id, new_id);
                        tracing::info!(
                            "创建新分组: {} ({} -> {})",
                            old_tag.name,
                            old_tag.tag_id,
                            new_id
                        );
                    }
                    Err(e) => {
                        tracing::warn!("创建分组 {} 失败: {}", old_tag.name, e);
                    }
                }

                // 延迟
                client.delay_random().await;
            }
        }

        Ok(mapping)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_options_default() {
        let options = RestoreOptions::default();
        assert!(options.create_missing_tags);
        assert!(!options.continue_on_error);
        assert_eq!(options.batch_size, 20);
        assert!(options.delay_ms.is_none());
    }

    #[test]
    fn test_restore_result_new() {
        let result = FollowingRestoreResult {
            success_count: 10,
            failed_count: 2,
            failures: vec![(123, "错误".to_string())],
            tag_mapping: HashMap::new(),
        };
        assert_eq!(result.success_count, 10);
        assert_eq!(result.failed_count, 2);
        assert_eq!(result.failures.len(), 1);
    }

    #[test]
    fn test_clear_result_new() {
        let result = FollowingClearResult {
            success_count: 5,
            failed_count: 1,
            failures: vec![(456, "错误".to_string())],
        };
        assert_eq!(result.success_count, 5);
        assert_eq!(result.failed_count, 1);
    }

    #[tokio::test]
    async fn test_following_service_create() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = FollowingService::new(client);
        assert!(service.client.read().await.get_cookie().is_none());
    }
}
