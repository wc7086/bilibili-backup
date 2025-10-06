use crate::api::client::BiliClient;
use crate::api::endpoints::{API_FAV_BATCH_DEL, API_FAV_COLLECT, API_FAV_CREATE, API_FAV_LIST, API_FAV_RESOURCES};
use crate::api::error::{BiliError, Result};
use crate::api::models::{ApiResult, FavFolder, FavInfo, Media, NormalPageData, RestoreResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 收藏夹备份数据（包含收藏夹信息和内容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavFolderWithMedia {
    /// 收藏夹基础信息
    pub folder: FavInfo,
    /// 收藏夹简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 收藏的媒体列表
    pub media_list: Vec<Media>,
}

/// 收藏夹还原选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavRestoreOptions {
    /// 是否忽略新账号现有数据（true则清空后还原，false则保留现有并追加）
    #[serde(default)]
    pub clear_existing: bool,
    /// 失败时是否继续（true则记录错误继续，false则遇错即停）
    #[serde(default = "default_true")]
    pub continue_on_error: bool,
    /// 批量处理大小（每次收藏的视频数量，最大20）
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

fn default_true() -> bool {
    true
}

fn default_batch_size() -> usize {
    20
}

impl Default for FavRestoreOptions {
    fn default() -> Self {
        Self {
            clear_existing: false,
            continue_on_error: true,
            batch_size: 20,
        }
    }
}

/// 收藏夹服务
pub struct FavoritesService {
    client: Arc<RwLock<BiliClient>>,
}

impl FavoritesService {
    /// 创建新的收藏夹服务实例
    ///
    /// # 参数
    ///
    /// * `client` - B站API客户端
    ///
    /// # 示例
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use bilibili_backup_tauri::api::BiliClient;
    /// use bilibili_backup_tauri::services::FavoritesService;
    ///
    /// let client = Arc::new(RwLock::new(BiliClient::new()));
    /// let service = FavoritesService::new(client);
    /// ```
    pub fn new(client: Arc<RwLock<BiliClient>>) -> Self {
        Self { client }
    }

    /// 备份收藏夹
    ///
    /// 获取用户的所有收藏夹及其内容
    ///
    /// # 返回
    ///
    /// 返回所有收藏夹及其内容的列表
    ///
    /// # 错误
    ///
    /// - `BiliError::AuthError`: 未登录或登录失效
    /// - `BiliError::NetworkError`: 网络请求失败
    /// - `BiliError::ApiError`: API返回错误
    pub async fn backup_favorites(&self) -> Result<Vec<FavFolderWithMedia>> {
        let client = self.client.read().await;

        // 1. 获取用户ID（从Cookie中提取）
        let cookie = client
            .get_cookie()
            .ok_or_else(|| BiliError::auth("未登录"))?;
        let user_id = BiliClient::parse_cookie_field(cookie, "DedeUserID")
            .ok_or_else(|| BiliError::auth("无法从Cookie获取用户ID"))?;

        // 2. 获取收藏夹列表
        tracing::info!("正在获取收藏夹列表...");
        let url = format!("{}?up_mid={}&type=0", API_FAV_LIST, user_id);
        let resp = client.get_with_retry(&url).await?;
        let api_result: ApiResult<NormalPageData<FavInfo>> = resp.json().await?;
        let folders = api_result.into_data()?.list;

        tracing::info!("找到 {} 个收藏夹", folders.len());

        // 3. 遍历每个收藏夹，获取视频列表
        let mut result = Vec::new();
        for (idx, folder) in folders.iter().enumerate() {
            tracing::info!(
                "正在获取收藏夹 [{}/{}] \"{}\" 的内容...",
                idx + 1,
                folders.len(),
                folder.title
            );

            let media_list = self.fetch_folder_media(folder.id, &client).await?;
            tracing::info!(
                "收藏夹 \"{}\" 包含 {} 个视频",
                folder.title,
                media_list.len()
            );

            result.push(FavFolderWithMedia {
                folder: folder.clone(),
                intro: None, // 简介从列表接口获取不完整，暂不填充
                media_list,
            });

            // 防风控延迟
            client.delay_random().await;
        }

        Ok(result)
    }

    /// 获取收藏夹内的所有媒体（分页获取）
    async fn fetch_folder_media(&self, folder_id: u64, client: &BiliClient) -> Result<Vec<Media>> {
        let mut all_media = Vec::new();
        let mut page = 1;
        let page_size = 20;

        loop {
            let url = format!(
                "{}?media_id={}&pn={}&ps={}",
                API_FAV_RESOURCES, folder_id, page, page_size
            );
            let resp = client.get_with_retry(&url).await?;
            let api_result: ApiResult<serde_json::Value> = resp.json().await?;
            let data = api_result.into_data()?;

            // 解析媒体列表
            let medias: Vec<Media> = if let Some(medias_val) = data.get("medias") {
                if medias_val.is_null() {
                    // API返回null表示没有更多数据
                    break;
                }
                serde_json::from_value(medias_val.clone())?
            } else {
                break;
            };

            if medias.is_empty() {
                break;
            }

            all_media.extend(medias);

            // 检查是否还有更多数据
            let has_more = data
                .get("has_more")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !has_more {
                break;
            }

            page += 1;
            client.delay_random().await;
        }

        Ok(all_media)
    }

    /// 还原收藏夹
    ///
    /// 将备份的收藏夹还原到当前账号
    ///
    /// # 参数
    ///
    /// * `folders` - 要还原的收藏夹列表
    /// * `options` - 还原选项
    ///
    /// # 返回
    ///
    /// 返回还原结果统计
    ///
    /// # 错误
    ///
    /// - `BiliError::AuthError`: 未登录或登录失效
    /// - `BiliError::NetworkError`: 网络请求失败
    /// - `BiliError::ApiError`: API返回错误
    pub async fn restore_favorites(
        &self,
        folders: Vec<FavFolderWithMedia>,
        options: FavRestoreOptions,
    ) -> Result<RestoreResult> {
        let client = self.client.read().await;

        // 验证批量大小
        let batch_size = options.batch_size.min(20);

        let mut success_count = 0;
        let mut failed_count = 0;
        let mut total_count = 0;
        let mut failed_items = Vec::new();

        // 1. 如果需要清空现有收藏夹
        if options.clear_existing {
            tracing::info!("正在清空现有收藏夹...");
            if let Err(e) = self.clear_all_folders().await {
                tracing::warn!("清空收藏夹失败: {}", e);
                if !options.continue_on_error {
                    return Err(e);
                }
            }
        }

        // 2. 遍历每个收藏夹进行还原
        for folder_data in folders {
            tracing::info!("正在还原收藏夹: \"{}\"", folder_data.folder.title);

            // 2.1 创建收藏夹
            let folder_id = match self
                .create_folder(
                    &folder_data.folder.title,
                    &folder_data.intro.clone().unwrap_or_default(),
                    if (folder_data.folder.attr & 1) == 1 {
                        1
                    } else {
                        0
                    },
                )
                .await
            {
                Ok(id) => id,
                Err(e) => {
                    tracing::error!("创建收藏夹 \"{}\" 失败: {}", folder_data.folder.title, e);
                    failed_count += folder_data.media_list.len();
                    total_count += folder_data.media_list.len();
                    failed_items.push(format!("收藏夹 \"{}\": {}", folder_data.folder.title, e));
                    if !options.continue_on_error {
                        return Err(e);
                    }
                    continue;
                }
            };

            // 2.2 处理容量上限（非默认收藏夹最多1000个视频）
            let is_default = folder_data.folder.is_default();
            let capacity = if is_default { 50000 } else { 1000 };

            if folder_data.media_list.len() > capacity {
                tracing::warn!(
                    "收藏夹 \"{}\" 包含 {} 个视频，超过容量上限 {}，将分批创建多个收藏夹",
                    folder_data.folder.title,
                    folder_data.media_list.len(),
                    capacity
                );
            }

            // 2.3 分批处理视频
            for (chunk_idx, chunk) in folder_data
                .media_list
                .chunks(capacity)
                .enumerate()
            {
                let current_folder_id = if chunk_idx == 0 {
                    folder_id
                } else {
                    // 超过容量时创建新收藏夹
                    let new_title = format!("{} ({})", folder_data.folder.title, chunk_idx + 1);
                    match self
                        .create_folder(
                            &new_title,
                            &folder_data.intro.clone().unwrap_or_default(),
                            if (folder_data.folder.attr & 1) == 1 {
                                1
                            } else {
                                0
                            },
                        )
                        .await
                    {
                        Ok(id) => id,
                        Err(e) => {
                            tracing::error!("创建分片收藏夹 \"{}\" 失败: {}", new_title, e);
                            failed_count += chunk.len();
                            total_count += chunk.len();
                            if !options.continue_on_error {
                                return Err(e);
                            }
                            continue;
                        }
                    }
                };

                // 2.4 批量添加视频到收藏夹
                for batch in chunk.chunks(batch_size) {
                    let media_ids: Vec<i64> = batch.iter().map(|m| m.id as i64).collect();
                    total_count += media_ids.len();

                    match self.add_to_folder(current_folder_id, media_ids.clone()).await {
                        Ok(_) => {
                            success_count += media_ids.len();
                            tracing::debug!("成功添加 {} 个视频到收藏夹", media_ids.len());
                        }
                        Err(e) => {
                            tracing::error!("添加视频到收藏夹失败: {}", e);
                            failed_count += media_ids.len();
                            failed_items.push(format!(
                                "收藏夹 \"{}\" 中的 {} 个视频: {}",
                                folder_data.folder.title,
                                media_ids.len(),
                                e
                            ));
                            if !options.continue_on_error {
                                return Err(e);
                            }
                        }
                    }

                    // 防风控延迟
                    client.delay_random().await;
                }
            }
        }

        Ok(RestoreResult {
            success_count,
            failed_count,
            total_count,
            failed_items: if failed_items.is_empty() {
                None
            } else {
                Some(failed_items)
            },
            message: format!(
                "还原完成: 成功 {}/{}，失败 {}",
                success_count, total_count, failed_count
            ),
        })
    }

    /// 清空所有收藏夹（仅清空内容，不删除收藏夹本身）
    ///
    /// # 返回
    ///
    /// 返回清空的视频总数
    ///
    /// # 错误
    ///
    /// - `BiliError::AuthError`: 未登录或登录失效
    /// - `BiliError::NetworkError`: 网络请求失败
    /// - `BiliError::ApiError`: API返回错误
    pub async fn clear_all_folders(&self) -> Result<usize> {
        let client = self.client.read().await;

        // 获取用户ID
        let cookie = client
            .get_cookie()
            .ok_or_else(|| BiliError::auth("未登录"))?;
        let user_id = BiliClient::parse_cookie_field(cookie, "DedeUserID")
            .ok_or_else(|| BiliError::auth("无法从Cookie获取用户ID"))?;

        // 获取收藏夹列表
        let url = format!("{}?up_mid={}&type=0", API_FAV_LIST, user_id);
        let resp = client.get_with_retry(&url).await?;
        let api_result: ApiResult<NormalPageData<FavInfo>> = resp.json().await?;
        let folders = api_result.into_data()?.list;

        let mut cleared_count = 0;

        // 遍历每个收藏夹清空内容
        for folder in folders {
            tracing::info!("正在清空收藏夹: \"{}\"", folder.title);

            let media_list = self.fetch_folder_media(folder.id, &client).await?;
            if media_list.is_empty() {
                continue;
            }

            // 批量删除（每次最多20个）
            for batch in media_list.chunks(20) {
                let resources: Vec<String> = batch
                    .iter()
                    .map(|m| format!("{}:{}", m.id, m.item_type))
                    .collect();

                let form = vec![
                    ("media_id".to_string(), folder.id.to_string()),
                    ("resources".to_string(), resources.join(",")),
                ];

                match client.post_form_with_retry(API_FAV_BATCH_DEL, &form).await {
                    Ok(resp) => {
                        let api_result: ApiResult<serde_json::Value> = resp.json().await?;
                        api_result.into_data()?;
                        cleared_count += batch.len();
                    }
                    Err(e) => {
                        tracing::warn!("删除收藏夹 \"{}\" 中的内容失败: {}", folder.title, e);
                        // 继续处理下一批
                    }
                }

                client.delay_random().await;
            }
        }

        tracing::info!("共清空 {} 个视频", cleared_count);
        Ok(cleared_count)
    }

    /// 创建收藏夹
    ///
    /// # 参数
    ///
    /// * `title` - 收藏夹标题
    /// * `intro` - 收藏夹简介
    /// * `privacy` - 是否私密（0:公开 1:私密）
    ///
    /// # 返回
    ///
    /// 返回新创建的收藏夹ID
    async fn create_folder(&self, title: &str, intro: &str, privacy: i32) -> Result<i64> {
        let client = self.client.read().await;

        // 获取bili_jct（CSRF令牌）
        let cookie = client
            .get_cookie()
            .ok_or_else(|| BiliError::auth("未登录"))?;
        let csrf = BiliClient::parse_cookie_field(cookie, "bili_jct")
            .ok_or_else(|| BiliError::auth("无法从Cookie获取bili_jct"))?;

        let form = vec![
            ("title".to_string(), title.to_string()),
            ("intro".to_string(), intro.to_string()),
            ("privacy".to_string(), privacy.to_string()),
            ("csrf".to_string(), csrf),
        ];

        let resp = client.post_form_with_retry(API_FAV_CREATE, &form).await?;
        let api_result: ApiResult<serde_json::Value> = resp.json().await?;
        let data = api_result.into_data()?;

        // 提取收藏夹ID
        let folder_id = data
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| BiliError::api("无法获取收藏夹ID"))?;

        tracing::info!("成功创建收藏夹 \"{}\" (ID: {})", title, folder_id);
        Ok(folder_id)
    }

    /// 添加视频到收藏夹
    ///
    /// # 参数
    ///
    /// * `folder_id` - 收藏夹ID
    /// * `media_ids` - 视频ID列表（最多20个）
    ///
    /// # 返回
    ///
    /// 成功则返回Ok(())
    async fn add_to_folder(&self, folder_id: i64, media_ids: Vec<i64>) -> Result<()> {
        if media_ids.is_empty() {
            return Ok(());
        }

        if media_ids.len() > 20 {
            return Err(BiliError::param("每次最多添加20个视频"));
        }

        let client = self.client.read().await;

        // 获取bili_jct（CSRF令牌）
        let cookie = client
            .get_cookie()
            .ok_or_else(|| BiliError::auth("未登录"))?;
        let csrf = BiliClient::parse_cookie_field(cookie, "bili_jct")
            .ok_or_else(|| BiliError::auth("无法从Cookie获取bili_jct"))?;

        // 批量添加视频
        for &media_id in &media_ids {
            let form = vec![
                ("rid".to_string(), media_id.to_string()),
                ("type".to_string(), "2".to_string()), // 2表示视频
                (
                    "add_media_ids".to_string(),
                    folder_id.to_string(),
                ),
                ("del_media_ids".to_string(), String::new()),
                ("csrf".to_string(), csrf.clone()),
            ];

            let resp = client.post_form_with_retry(API_FAV_COLLECT, &form).await?;
            let api_result: ApiResult<serde_json::Value> = resp.json().await?;
            api_result.into_data()?;

            // 小延迟避免风控
            client.delay(500).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fav_restore_options_default() {
        let options = FavRestoreOptions::default();
        assert!(!options.clear_existing);
        assert!(options.continue_on_error);
        assert_eq!(options.batch_size, 20);
    }

    #[test]
    fn test_fav_folder_with_media_serialization() {
        let folder_data = FavFolderWithMedia {
            folder: FavInfo {
                id: 123,
                fid: Some(123),
                mid: 456,
                attr: 0,
                title: "测试收藏夹".to_string(),
                cover: None,
                ctime: None,
                media_count: 10,
            },
            intro: Some("测试简介".to_string()),
            media_list: vec![],
        };

        let json = serde_json::to_string(&folder_data).unwrap();
        assert!(json.contains("测试收藏夹"));
        assert!(json.contains("测试简介"));
    }
}
