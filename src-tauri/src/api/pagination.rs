/// 分页数据获取模块
///
/// 提供通用的分页数据获取功能,支持:
/// - 普通分页 (页码分页)
/// - 游标分页 (cursor分页)
///
/// # 示例
///
/// ```rust
/// use bilibili_backup_tauri::api::{BiliClient, pagination::fetch_all_pages};
/// use bilibili_backup_tauri::api::endpoints::API_FOLLOWING_LIST;
/// use bilibili_backup_tauri::api::models::{ApiResult, PageData, Relation};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BiliClient::new();
/// let url = format!("{}?vmid=123&order=attention", API_FOLLOWING_LIST);
///
/// let all_data = fetch_all_pages::<Relation>(
///     &client,
///     &url,
///     20,  // 每页20条
///     None // 无最大页数限制
/// ).await?;
///
/// println!("获取到 {} 条数据", all_data.len());
/// # Ok(())
/// # }
/// ```

use crate::api::client::BiliClient;
use crate::api::error::{BiliError, Result};
use crate::api::models::{ApiResult, CursorPageData, PageData};
use serde::de::DeserializeOwned;

/// 获取所有分页数据 (普通分页)
///
/// 自动处理分页逻辑,获取所有数据直到没有更多数据为止。
///
/// # 类型参数
///
/// * `T` - 数据项类型 (必须实现 DeserializeOwned)
///
/// # 参数
///
/// * `client` - HTTP客户端
/// * `base_url` - 基础URL (不含分页参数)
/// * `page_size` - 每页大小
/// * `max_pages` - 最大页数限制 (None表示无限制)
///
/// # 返回
///
/// Result<Vec<T>, BiliError> - 所有数据项的列表
///
/// # 错误
///
/// - 网络请求失败
/// - JSON解析失败
/// - API返回错误
///
/// # 示例
///
/// ```rust
/// # use bilibili_backup_tauri::api::{BiliClient, pagination::fetch_all_pages};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BiliClient::new();
/// let data = fetch_all_pages::<serde_json::Value>(
///     &client,
///     "https://api.bilibili.com/x/relation/followings?vmid=123",
///     20,
///     Some(10) // 最多10页
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn fetch_all_pages<T>(
    client: &BiliClient,
    base_url: &str,
    page_size: usize,
    max_pages: Option<usize>,
) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let mut all_items = Vec::new();
    let mut page = 1;

    loop {
        // 检查是否达到最大页数
        if let Some(max) = max_pages {
            if page > max {
                break;
            }
        }

        // 构建URL
        let separator = if base_url.contains('?') { '&' } else { '?' };
        let url = format!("{}{}pn={}&ps={}", base_url, separator, page, page_size);

        // 发送请求
        let response = client.get_with_retry(&url).await?;

        // 解析响应
        let api_result: ApiResult<PageData<T>> = response.json().await?;

        // 检查API错误
        if !api_result.is_success() {
            return Err(BiliError::api(format!(
                "API错误 [{}]: {}",
                api_result.code, api_result.message
            )));
        }

        // 获取数据
        if let Some(page_data) = api_result.data {
            let items = page_data.list;
            let current_total = all_items.len() + items.len();
            let has_more = current_total < page_data.total;

            all_items.extend(items);

            if !has_more {
                break;
            }
        } else {
            break;
        }

        page += 1;

        // 随机延迟,防风控
        client.delay_random().await;
    }

    Ok(all_items)
}

/// 获取所有游标分页数据
///
/// 自动处理游标分页逻辑,获取所有数据直到没有更多数据为止。
///
/// # 类型参数
///
/// * `T` - 数据项类型 (必须实现 DeserializeOwned)
///
/// # 参数
///
/// * `client` - HTTP客户端
/// * `base_url` - 基础URL
/// * `max_iterations` - 最大迭代次数 (防止无限循环)
///
/// # 返回
///
/// Result<Vec<T>, BiliError> - 所有数据项的列表
///
/// # 示例
///
/// ```rust
/// # use bilibili_backup_tauri::api::{BiliClient, pagination::fetch_cursor_pages};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BiliClient::new();
/// let data = fetch_cursor_pages::<serde_json::Value>(
///     &client,
///     "https://api.bilibili.com/x/web-interface/history/cursor",
///     100 // 最多100次迭代
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn fetch_cursor_pages<T>(
    client: &BiliClient,
    base_url: &str,
    max_iterations: usize,
) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;
    let mut iteration = 0;

    loop {
        // 检查迭代次数
        if iteration >= max_iterations {
            tracing::warn!("达到最大迭代次数: {}", max_iterations);
            break;
        }

        // 构建URL
        let url = if let Some(ref c) = cursor {
            let separator = if base_url.contains('?') { '&' } else { '?' };
            format!("{}{}cursor={}", base_url, separator, c)
        } else {
            base_url.to_string()
        };

        // 发送请求
        let response = client.get_with_retry(&url).await?;

        // 解析响应
        let api_result: ApiResult<CursorPageData<T>> = response.json().await?;

        // 检查API错误
        if !api_result.is_success() {
            return Err(BiliError::api(format!(
                "API错误 [{}]: {}",
                api_result.code, api_result.message
            )));
        }

        // 获取数据
        if let Some(page_data) = api_result.data {
            all_items.extend(page_data.list);

            // 检查是否还有更多数据
            if !page_data.has_more || page_data.cursor.is_none() {
                break;
            }

            cursor = page_data.cursor;
        } else {
            break;
        }

        iteration += 1;

        // 随机延迟,防风控
        client.delay_random().await;
    }

    Ok(all_items)
}

/// 获取单页数据
///
/// 只获取指定页的数据,不进行自动分页。
///
/// # 类型参数
///
/// * `T` - 数据项类型
///
/// # 参数
///
/// * `client` - HTTP客户端
/// * `url` - 完整URL (含所有参数)
///
/// # 返回
///
/// Result<PageData<T>, BiliError>
///
/// # 示例
///
/// ```rust
/// # use bilibili_backup_tauri::api::{BiliClient, pagination::fetch_single_page};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BiliClient::new();
/// let page_data = fetch_single_page::<serde_json::Value>(
///     &client,
///     "https://api.bilibili.com/x/relation/followings?vmid=123&pn=1&ps=20"
/// ).await?;
/// println!("总数: {}", page_data.total);
/// # Ok(())
/// # }
/// ```
pub async fn fetch_single_page<T>(client: &BiliClient, url: &str) -> Result<PageData<T>>
where
    T: DeserializeOwned,
{
    let response = client.get_with_retry(url).await?;
    let api_result: ApiResult<PageData<T>> = response.json().await?;
    api_result.into_data()
}

/// 分页数据获取器 (构建器模式)
///
/// 提供更灵活的分页数据获取方式,支持自定义配置。
///
/// # 示例
///
/// ```rust
/// # use bilibili_backup_tauri::api::{BiliClient, pagination::PageFetcher};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = BiliClient::new();
/// let data = PageFetcher::<serde_json::Value>::new(&client, "https://api.bilibili.com/example")
///     .page_size(30)
///     .max_pages(5)
///     .start_page(1)
///     .fetch_all()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct PageFetcher<'a, T> {
    client: &'a BiliClient,
    base_url: String,
    page_size: usize,
    max_pages: Option<usize>,
    start_page: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> PageFetcher<'a, T>
where
    T: DeserializeOwned,
{
    /// 创建新的分页获取器
    ///
    /// # 参数
    ///
    /// * `client` - HTTP客户端
    /// * `base_url` - 基础URL (不含分页参数)
    pub fn new(client: &'a BiliClient, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            page_size: 20,
            max_pages: None,
            start_page: 1,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 设置每页大小
    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size;
        self
    }

    /// 设置最大页数
    pub fn max_pages(mut self, max: usize) -> Self {
        self.max_pages = Some(max);
        self
    }

    /// 设置起始页码
    pub fn start_page(mut self, page: usize) -> Self {
        self.start_page = page;
        self
    }

    /// 获取所有数据
    pub async fn fetch_all(self) -> Result<Vec<T>> {
        let mut all_items = Vec::new();
        let mut page = self.start_page;

        loop {
            // 检查是否达到最大页数
            if let Some(max) = self.max_pages {
                if page >= self.start_page + max {
                    break;
                }
            }

            // 构建URL
            let separator = if self.base_url.contains('?') { '&' } else { '?' };
            let url = format!(
                "{}{}pn={}&ps={}",
                self.base_url, separator, page, self.page_size
            );

            // 发送请求
            let response = self.client.get_with_retry(&url).await?;

            // 解析响应
            let api_result: ApiResult<PageData<T>> = response.json().await?;

            // 检查API错误
            if !api_result.is_success() {
                return Err(BiliError::api(format!(
                    "API错误 [{}]: {}",
                    api_result.code, api_result.message
                )));
            }

            // 获取数据
            if let Some(page_data) = api_result.data {
                let items = page_data.list;
                let current_total = all_items.len() + items.len();
                let has_more = current_total < page_data.total;

                all_items.extend(items);

                if !has_more {
                    break;
                }
            } else {
                break;
            }

            page += 1;

            // 随机延迟,防风控
            self.client.delay_random().await;
        }

        Ok(all_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_fetcher_builder() {
        let client = BiliClient::new();
        let fetcher = PageFetcher::<serde_json::Value>::new(&client, "https://example.com")
            .page_size(30)
            .max_pages(5)
            .start_page(2);

        assert_eq!(fetcher.page_size, 30);
        assert_eq!(fetcher.max_pages, Some(5));
        assert_eq!(fetcher.start_page, 2);
    }

    #[test]
    fn test_page_fetcher_default() {
        let client = BiliClient::new();
        let fetcher = PageFetcher::<serde_json::Value>::new(&client, "https://example.com");

        assert_eq!(fetcher.page_size, 20);
        assert_eq!(fetcher.max_pages, None);
        assert_eq!(fetcher.start_page, 1);
    }
}
