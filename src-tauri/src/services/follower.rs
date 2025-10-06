/// 粉丝管理服务
///
/// 提供粉丝列表的备份功能
///
/// # 注意
///
/// B站API不支持还原粉丝列表,因此本服务仅提供备份功能。
/// 粉丝关系是由其他用户主动关注产生的,无法通过API模拟。

use crate::api::{
    client::BiliClient,
    endpoints::*,
    error::{BiliError, Result},
    models::*,
    pagination::fetch_all_pages,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 粉丝服务
pub struct FollowerService {
    client: Arc<RwLock<BiliClient>>,
}

impl FollowerService {
    /// 创建新的粉丝服务实例
    ///
    /// # 参数
    ///
    /// * `client` - HTTP客户端 (需要已登录)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::follower::FollowerService;
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = FollowerService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份粉丝列表
    ///
    /// 获取当前用户的所有粉丝。
    ///
    /// # 返回
    ///
    /// Result<Vec<Relation>, BiliError> - 粉丝列表
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
    /// # use bilibili_backup_tauri::services::follower::FollowerService;
    /// # async fn example(service: FollowerService) -> Result<(), Box<dyn std::error::Error>> {
    /// let followers = service.backup_followers().await?;
    /// println!("备份了 {} 个粉丝", followers.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn backup_followers(&self) -> Result<Vec<Relation>> {
        let client = self.client.read().await;

        // 1. 获取当前用户信息
        let nav_info = self.get_user_info(&client).await?;
        let mid = nav_info.mid.ok_or_else(|| BiliError::auth("未登录"))?;

        // 2. 分页获取所有粉丝
        let base_url = format!("{}?vmid={}", API_FOLLOWER_LIST, mid);
        let followers = fetch_all_pages::<Relation>(&client, &base_url, 50, None).await?;

        tracing::info!("备份了 {} 个粉丝", followers.len());
        Ok(followers)
    }

    // ==================== 私有辅助方法 ====================

    /// 获取用户信息
    async fn get_user_info(&self, client: &BiliClient) -> Result<NavInfo> {
        let response = client.get_with_retry(API_NAV).await?;
        let api_result: ApiResult<NavInfo> = response.json().await?;
        api_result.into_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_follower_service_create() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = FollowerService::new(client);
        assert!(service.client.read().await.get_cookie().is_none());
    }
}
