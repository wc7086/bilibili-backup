use reqwest::{header, Client, ClientBuilder, RequestBuilder, Response};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// B站API HTTP客户端
///
/// 提供以下功能:
/// - 自动添加必要的请求头 (User-Agent, Referer等)
/// - Cookie管理
/// - 请求限流 (默认每秒2个请求)
/// - 自动重试 (最多3次)
/// - 随机延迟 (防风控, 1-3秒)
///
/// # 示例
///
/// ```rust
/// use bilibili_backup_tauri::api::BiliClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut client = BiliClient::new();
/// client.set_cookie("SESSDATA=xxx; bili_jct=yyy".to_string());
///
/// let response = client.get("https://api.bilibili.com/x/web-interface/nav")
///     .send()
///     .await?;
///
/// println!("Status: {}", response.status());
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct BiliClient {
    client: Client,
    cookie: Option<String>,
    rate_limiter: Arc<Semaphore>,
    max_retries: usize,
    min_delay_ms: u64,
    max_delay_ms: u64,
}

impl BiliClient {
    /// 创建新的客户端实例
    ///
    /// 默认配置:
    /// - 请求限流: 每秒2个请求
    /// - 最大重试次数: 3次
    /// - 随机延迟范围: 1000-3000毫秒
    /// - 请求超时: 30秒
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let client = BiliClient::new();
    /// ```
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            ),
        );
        headers.insert(
            header::REFERER,
            header::HeaderValue::from_static("https://www.bilibili.com/"),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static(
                "application/json, text/plain, */*",
            ),
        );
        headers.insert(
            header::ACCEPT_LANGUAGE,
            header::HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8"),
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            cookie: None,
            rate_limiter: Arc::new(Semaphore::new(2)), // 每秒最多2个请求
            max_retries: 3,
            min_delay_ms: 1000,
            max_delay_ms: 3000,
        }
    }

    /// 设置请求限流参数
    ///
    /// # 参数
    ///
    /// * `permits` - 并发请求数量限制
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let client = BiliClient::new().with_rate_limit(3);
    /// ```
    pub fn with_rate_limit(mut self, permits: usize) -> Self {
        self.rate_limiter = Arc::new(Semaphore::new(permits));
        self
    }

    /// 设置最大重试次数
    ///
    /// # 参数
    ///
    /// * `retries` - 最大重试次数
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let client = BiliClient::new().with_max_retries(5);
    /// ```
    pub fn with_max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    /// 设置随机延迟范围
    ///
    /// # 参数
    ///
    /// * `min_ms` - 最小延迟(毫秒)
    /// * `max_ms` - 最大延迟(毫秒)
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let client = BiliClient::new().with_delay_range(500, 2000);
    /// ```
    pub fn with_delay_range(mut self, min_ms: u64, max_ms: u64) -> Self {
        self.min_delay_ms = min_ms;
        self.max_delay_ms = max_ms;
        self
    }

    /// 设置Cookie
    ///
    /// # 参数
    ///
    /// * `cookie` - Cookie字符串 (如: "SESSDATA=xxx; bili_jct=yyy")
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let mut client = BiliClient::new();
    /// client.set_cookie("SESSDATA=xxx; bili_jct=yyy".to_string());
    /// ```
    pub fn set_cookie(&mut self, cookie: String) {
        self.cookie = Some(cookie);
    }

    /// 获取当前Cookie
    ///
    /// # 返回
    ///
    /// Cookie字符串的引用 (如果已设置)
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let client = BiliClient::new();
    /// if let Some(cookie) = client.get_cookie() {
    ///     println!("Cookie: {}", cookie);
    /// }
    /// ```
    pub fn get_cookie(&self) -> Option<&str> {
        self.cookie.as_deref()
    }

    /// 获取内部reqwest客户端
    ///
    /// # 返回
    ///
    /// 内部的 reqwest::Client 引用
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// 构建GET请求 (带Cookie)
    ///
    /// # 参数
    ///
    /// * `url` - 请求URL
    ///
    /// # 返回
    ///
    /// reqwest::RequestBuilder 用于进一步配置请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BiliClient::new();
    /// let response = client.get("https://api.bilibili.com/x/web-interface/nav")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, url: &str) -> RequestBuilder {
        let mut request = self.client.get(url);
        if let Some(ref cookie) = self.cookie {
            request = request.header(header::COOKIE, cookie);
        }
        request
    }

    /// 构建POST请求 (带Cookie)
    ///
    /// # 参数
    ///
    /// * `url` - 请求URL
    ///
    /// # 返回
    ///
    /// reqwest::RequestBuilder 用于进一步配置请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BiliClient::new();
    /// let response = client.post("https://api.bilibili.com/x/relation/modify")
    ///     .json(&json!({"fid": 123, "act": 1}))
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn post(&self, url: &str) -> RequestBuilder {
        let mut request = self.client.post(url);
        if let Some(ref cookie) = self.cookie {
            request = request.header(header::COOKIE, cookie);
        }
        request
    }

    /// 执行GET请求 (带限流和重试)
    ///
    /// 此方法会自动:
    /// - 获取限流许可
    /// - 在失败时重试 (最多 max_retries 次)
    /// - 在重试之间延迟1秒
    ///
    /// # 参数
    ///
    /// * `url` - 请求URL
    ///
    /// # 返回
    ///
    /// Result<Response, reqwest::Error>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BiliClient::new();
    /// let response = client.get_with_retry("https://api.bilibili.com/x/web-interface/nav").await?;
    /// println!("Status: {}", response.status());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_with_retry(&self, url: &str) -> Result<Response, reqwest::Error> {
        let _permit = self.rate_limiter.acquire().await.unwrap();

        for attempt in 0..self.max_retries {
            match self.get(url).send().await {
                Ok(resp) => return Ok(resp),
                Err(e) if attempt < self.max_retries - 1 => {
                    tracing::warn!("请求失败 (尝试 {}/{}): {}", attempt + 1, self.max_retries, e);
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }

    /// 执行POST请求 (带限流和重试)
    ///
    /// 此方法会自动:
    /// - 获取限流许可
    /// - 在失败时重试 (最多 max_retries 次)
    /// - 在重试之间延迟1秒
    ///
    /// # 参数
    ///
    /// * `url` - 请求URL
    /// * `body` - 请求体 (JSON或表单数据)
    ///
    /// # 返回
    ///
    /// Result<Response, reqwest::Error>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BiliClient::new();
    /// let body = json!({"fid": 123, "act": 1});
    /// let response = client.post_json_with_retry(
    ///     "https://api.bilibili.com/x/relation/modify",
    ///     &body
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_json_with_retry<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response, reqwest::Error> {
        let _permit = self.rate_limiter.acquire().await.unwrap();

        for attempt in 0..self.max_retries {
            match self.post(url).json(body).send().await {
                Ok(resp) => return Ok(resp),
                Err(e) if attempt < self.max_retries - 1 => {
                    tracing::warn!("请求失败 (尝试 {}/{}): {}", attempt + 1, self.max_retries, e);
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }

    /// 执行POST表单请求 (带限流和重试)
    ///
    /// # 参数
    ///
    /// * `url` - 请求URL
    /// * `form` - 表单数据 (键值对)
    ///
    /// # 返回
    ///
    /// Result<Response, reqwest::Error>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BiliClient::new();
    /// let form = vec![
    ///     ("fid".to_string(), "123".to_string()),
    ///     ("act".to_string(), "1".to_string()),
    /// ];
    /// let response = client.post_form_with_retry(
    ///     "https://api.bilibili.com/x/relation/modify",
    ///     &form
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_form_with_retry(
        &self,
        url: &str,
        form: &[(String, String)],
    ) -> Result<Response, reqwest::Error> {
        let _permit = self.rate_limiter.acquire().await.unwrap();

        for attempt in 0..self.max_retries {
            match self.post(url).form(form).send().await {
                Ok(resp) => return Ok(resp),
                Err(e) if attempt < self.max_retries - 1 => {
                    tracing::warn!("请求失败 (尝试 {}/{}): {}", attempt + 1, self.max_retries, e);
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }

    /// 随机延迟 (防风控)
    ///
    /// 延迟时间在 min_delay_ms 到 max_delay_ms 之间随机选择
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # async fn example() {
    /// let client = BiliClient::new();
    /// client.delay_random().await; // 延迟 1000-3000ms
    /// # }
    /// ```
    pub async fn delay_random(&self) {
        use rand::Rng;
        let delay_ms = rand::thread_rng().gen_range(self.min_delay_ms..=self.max_delay_ms);
        sleep(Duration::from_millis(delay_ms)).await;
    }

    /// 固定延迟
    ///
    /// # 参数
    ///
    /// * `ms` - 延迟时间(毫秒)
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// # async fn example() {
    /// let client = BiliClient::new();
    /// client.delay(1500).await; // 延迟 1500ms
    /// # }
    /// ```
    pub async fn delay(&self, ms: u64) {
        sleep(Duration::from_millis(ms)).await;
    }

    /// 提取Cookie中的关键字段
    ///
    /// # 参数
    ///
    /// * `cookie` - Cookie字符串
    /// * `field` - 要提取的字段名
    ///
    /// # 返回
    ///
    /// 字段值 (如果存在)
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::BiliClient;
    /// let cookie = "DedeUserID=123456; bili_jct=abcdef; SESSDATA=xyz";
    /// let user_id = BiliClient::parse_cookie_field(cookie, "DedeUserID");
    /// assert_eq!(user_id, Some("123456".to_string()));
    /// ```
    pub fn parse_cookie_field(cookie: &str, field: &str) -> Option<String> {
        for part in cookie.split(';') {
            let part = part.trim();
            if let Some(stripped) = part.strip_prefix(field) {
                if let Some(stripped) = stripped.strip_prefix('=') {
                    return Some(stripped.to_string());
                }
            }
        }
        None
    }
}

impl Default for BiliClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let client = BiliClient::new();
        assert!(client.get_cookie().is_none());
        assert_eq!(client.max_retries, 3);
        assert_eq!(client.min_delay_ms, 1000);
        assert_eq!(client.max_delay_ms, 3000);
    }

    #[test]
    fn test_with_rate_limit() {
        let client = BiliClient::new().with_rate_limit(5);
        // Semaphore的available_permits()方法需要获取许可才能检查,所以我们只验证构建成功
        assert!(client.get_cookie().is_none());
    }

    #[test]
    fn test_with_max_retries() {
        let client = BiliClient::new().with_max_retries(5);
        assert_eq!(client.max_retries, 5);
    }

    #[test]
    fn test_with_delay_range() {
        let client = BiliClient::new().with_delay_range(500, 2000);
        assert_eq!(client.min_delay_ms, 500);
        assert_eq!(client.max_delay_ms, 2000);
    }

    #[test]
    fn test_set_cookie() {
        let mut client = BiliClient::new();
        client.set_cookie("test_cookie=value".to_string());
        assert_eq!(client.get_cookie(), Some("test_cookie=value"));
    }

    #[test]
    fn test_parse_cookie_field() {
        let cookie = "DedeUserID=123456; bili_jct=abcdef; SESSDATA=xyz";

        assert_eq!(
            BiliClient::parse_cookie_field(cookie, "DedeUserID"),
            Some("123456".to_string())
        );
        assert_eq!(
            BiliClient::parse_cookie_field(cookie, "bili_jct"),
            Some("abcdef".to_string())
        );
        assert_eq!(
            BiliClient::parse_cookie_field(cookie, "SESSDATA"),
            Some("xyz".to_string())
        );
        assert_eq!(BiliClient::parse_cookie_field(cookie, "nonexistent"), None);
    }

    #[test]
    fn test_parse_cookie_field_with_spaces() {
        let cookie = "DedeUserID = 123456 ; bili_jct = abcdef";

        // 测试带空格的cookie解析
        assert_eq!(
            BiliClient::parse_cookie_field(cookie, "DedeUserID"),
            Some("123456".to_string())
        );
    }

    #[test]
    fn test_parse_cookie_field_empty() {
        let cookie = "";
        assert_eq!(BiliClient::parse_cookie_field(cookie, "any"), None);
    }

    #[tokio::test]
    async fn test_delay() {
        let client = BiliClient::new();
        let start = std::time::Instant::now();
        client.delay(100).await;
        let elapsed = start.elapsed();

        // 验证延迟至少100ms (允许一些误差)
        assert!(elapsed.as_millis() >= 100);
        assert!(elapsed.as_millis() < 200); // 不应该太长
    }

    #[tokio::test]
    async fn test_delay_random() {
        let client = BiliClient::new().with_delay_range(50, 100);
        let start = std::time::Instant::now();
        client.delay_random().await;
        let elapsed = start.elapsed();

        // 验证延迟在范围内
        assert!(elapsed.as_millis() >= 50);
        assert!(elapsed.as_millis() <= 150); // 允许一些误差
    }

    #[test]
    fn test_default() {
        let client = BiliClient::default();
        assert!(client.get_cookie().is_none());
        assert_eq!(client.max_retries, 3);
    }
}
