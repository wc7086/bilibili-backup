use crate::api::{endpoints::*, models::*, BiliClient, BiliError, Result, WbiSigner};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 认证服务
///
/// 提供用户登录、登出、身份验证等功能，包括：
/// - 二维码登录流程（生成二维码、轮询扫码状态）
/// - Cookie登录
/// - 用户信息获取
/// - WBI签名器初始化
///
/// # 示例
///
/// ```rust
/// use bilibili_backup_tauri::services::auth::AuthService;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let auth = AuthService::new();
///
/// // 生成登录二维码
/// let qrcode = auth.generate_qrcode().await?;
/// println!("请扫描二维码: {}", qrcode.url);
///
/// // 轮询登录状态
/// loop {
///     let result = auth.poll_qrcode(&qrcode.qrcode_key).await?;
///     if result.is_success() {
///         println!("登录成功!");
///         break;
///     }
/// }
///
/// // 获取用户信息
/// let nav_info = auth.get_nav_info().await?;
/// println!("当前用户: {:?}", nav_info.uname);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct AuthService {
    /// HTTP客户端
    client: Arc<RwLock<BiliClient>>,
    /// 当前登录用户
    current_user: Arc<RwLock<Option<AuthUser>>>,
    /// WBI签名器
    wbi_signer: Arc<RwLock<Option<WbiSigner>>>,
}

/// 认证用户信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthUser {
    /// 用户ID
    pub uid: String,
    /// Cookie字符串
    pub cookie: String,
    /// CSRF令牌
    pub bili_jct: String,
    /// 是否已注销账号
    #[serde(default)]
    pub is_cancelled_account: bool,
}

impl AuthUser {
    /// 从Cookie字符串创建用户
    ///
    /// # 参数
    ///
    /// * `cookie` - Cookie字符串 (如: "DedeUserID=123456; bili_jct=xxx")
    ///
    /// # 返回
    ///
    /// Result<AuthUser, BiliError>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthUser;
    /// let cookie = "DedeUserID=123456; bili_jct=abcdef1234567890abcdef1234567890";
    /// let user = AuthUser::from_cookie(cookie).unwrap();
    /// assert_eq!(user.uid, "123456");
    /// assert_eq!(user.bili_jct, "abcdef1234567890abcdef1234567890");
    /// ```
    pub fn from_cookie(cookie: &str) -> Result<Self> {
        let uid = BiliClient::parse_cookie_field(cookie, "DedeUserID")
            .ok_or_else(|| BiliError::auth("Cookie中缺少DedeUserID字段"))?;

        let bili_jct = BiliClient::parse_cookie_field(cookie, "bili_jct")
            .ok_or_else(|| BiliError::auth("Cookie中缺少bili_jct字段"))?;

        Ok(Self {
            uid,
            cookie: cookie.to_string(),
            bili_jct,
            is_cancelled_account: false,
        })
    }
}

impl AuthService {
    /// 创建新的认证服务实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// let auth = AuthService::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(BiliClient::new())),
            current_user: Arc::new(RwLock::new(None)),
            wbi_signer: Arc::new(RwLock::new(None)),
        }
    }

    /// 生成登录二维码
    ///
    /// 调用B站API生成二维码，返回二维码URL和密钥
    ///
    /// # 返回
    ///
    /// Result<QRCode, BiliError>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = AuthService::new();
    /// let qrcode = auth.generate_qrcode().await?;
    /// println!("二维码URL: {}", qrcode.url);
    /// println!("二维码密钥: {}", qrcode.qrcode_key);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_qrcode(&self) -> Result<QRCode> {
        let client = self.client.read().await;

        let resp = client
            .get_with_retry(API_QR_GENERATE)
            .await
            .map_err(|e| BiliError::api(format!("生成二维码失败: {}", e)))?;

        let api_result: ApiResult<QRCode> = resp
            .json()
            .await
            .map_err(|e| BiliError::api(format!("解析二维码响应失败: {}", e)))?;

        api_result.into_data()
    }

    /// 轮询二维码登录状态
    ///
    /// 查询二维码是否被扫描以及登录状态，登录成功后自动设置Cookie
    ///
    /// # 参数
    ///
    /// * `qrcode_key` - 二维码密钥
    ///
    /// # 返回
    ///
    /// Result<LoginResult, BiliError>
    ///
    /// # 登录状态码
    ///
    /// - 0: 登录成功
    /// - 86038: 二维码已失效
    /// - 86101: 二维码未扫描
    /// - 86090: 二维码已扫描，等待确认
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = AuthService::new();
    /// let qrcode = auth.generate_qrcode().await?;
    ///
    /// loop {
    ///     let result = auth.poll_qrcode(&qrcode.qrcode_key).await?;
    ///     match result.code {
    ///         0 => {
    ///             println!("登录成功!");
    ///             break;
    ///         }
    ///         86038 => {
    ///             println!("二维码已失效");
    ///             break;
    ///         }
    ///         86101 => {
    ///             println!("等待扫描...");
    ///         }
    ///         86090 => {
    ///             println!("已扫描，等待确认...");
    ///         }
    ///         _ => {}
    ///     }
    ///     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn poll_qrcode(&self, qrcode_key: &str) -> Result<LoginResult> {
        let client = self.client.read().await;

        let url = format!("{}?qrcode_key={}", API_QR_POLL, qrcode_key);
        let resp = client
            .get_with_retry(&url)
            .await
            .map_err(|e| BiliError::api(format!("轮询登录状态失败: {}", e)))?;

        // 提取Set-Cookie头
        let cookie = if let Some(set_cookie) = resp.headers().get("set-cookie") {
            set_cookie.to_str().unwrap_or("").to_string()
        } else {
            String::new()
        };

        let api_result: ApiResult<LoginResult> = resp
            .json()
            .await
            .map_err(|e| BiliError::api(format!("解析登录响应失败: {}", e)))?;

        let login_result = api_result.into_data()?;

        // 如果登录成功，保存Cookie并完善
        if login_result.is_success() && !cookie.is_empty() {
            // 完善Cookie（添加设备指纹）
            let perfected_cookie = self.perfect_cookie(&cookie).await?;

            // 使用Cookie登录
            self.login_with_cookie(&perfected_cookie).await?;
        }

        Ok(login_result)
    }

    /// 完善Cookie（添加设备指纹）
    ///
    /// 添加buvid3和buvid4字段，这些字段用于设备识别
    ///
    /// # 参数
    ///
    /// * `cookie` - 原始Cookie字符串
    ///
    /// # 返回
    ///
    /// Result<String, BiliError> - 完善后的Cookie字符串
    async fn perfect_cookie(&self, cookie: &str) -> Result<String> {
        let client = self.client.read().await;

        let resp = client
            .get_with_retry(API_FINGER_SPI)
            .await
            .map_err(|e| BiliError::api(format!("获取设备指纹失败: {}", e)))?;

        // 从响应头提取设备指纹字段
        let mut perfected = cookie.to_string();

        if let Some(set_cookie) = resp.headers().get("set-cookie") {
            let set_cookie_str = set_cookie.to_str().unwrap_or("");

            // 提取buvid3
            if let Some(buvid3) = BiliClient::parse_cookie_field(set_cookie_str, "buvid3") {
                if !perfected.contains("buvid3=") {
                    perfected.push_str(&format!("; buvid3={}", buvid3));
                }
            }

            // 提取buvid4
            if let Some(buvid4) = BiliClient::parse_cookie_field(set_cookie_str, "buvid4") {
                if !perfected.contains("buvid4=") {
                    perfected.push_str(&format!("; buvid4={}", buvid4));
                }
            }
        }

        Ok(perfected)
    }

    /// 使用Cookie登录
    ///
    /// 设置Cookie并验证是否有效，同时初始化WBI签名器
    ///
    /// # 参数
    ///
    /// * `cookie` - Cookie字符串
    ///
    /// # 返回
    ///
    /// Result<AuthUser, BiliError>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = AuthService::new();
    /// let cookie = "DedeUserID=123456; bili_jct=xxx; SESSDATA=yyy";
    /// let user = auth.login_with_cookie(cookie).await?;
    /// println!("登录成功: {}", user.uid);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login_with_cookie(&self, cookie: &str) -> Result<AuthUser> {
        // 解析Cookie生成用户信息
        let user = AuthUser::from_cookie(cookie)?;

        // 设置客户端Cookie
        {
            let mut client = self.client.write().await;
            client.set_cookie(cookie.to_string());
        }

        // 验证Cookie有效性并获取用户信息
        let nav_info = self.get_nav_info().await?;

        // 检查是否登录
        if nav_info.is_login != Some(true) {
            return Err(BiliError::auth("Cookie无效或已过期"));
        }

        // 初始化WBI签名器
        self.init_wbi_signer().await?;

        // 保存当前用户
        {
            let mut current_user = self.current_user.write().await;
            *current_user = Some(user.clone());
        }

        tracing::info!("用户登录成功: uid={}", user.uid);

        Ok(user)
    }

    /// 获取用户导航信息
    ///
    /// 返回用户的基本信息和WBI签名图片URL
    ///
    /// # 返回
    ///
    /// Result<NavInfo, BiliError>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = AuthService::new();
    /// let nav_info = auth.get_nav_info().await?;
    /// if let Some(uname) = nav_info.uname {
    ///     println!("用户名: {}", uname);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nav_info(&self) -> Result<NavInfo> {
        let client = self.client.read().await;

        let resp = client
            .get_with_retry(API_NAV)
            .await
            .map_err(|e| BiliError::api(format!("获取导航信息失败: {}", e)))?;

        let api_result: ApiResult<NavInfo> = resp
            .json()
            .await
            .map_err(|e| BiliError::api(format!("解析导航信息失败: {}", e)))?;

        api_result.into_data()
    }

    /// 初始化WBI签名器
    ///
    /// 从导航API获取WBI图片URL并创建签名器
    ///
    /// # 返回
    ///
    /// Result<(), BiliError>
    async fn init_wbi_signer(&self) -> Result<()> {
        let nav_info = self.get_nav_info().await?;

        if let Some(wbi_img) = nav_info.wbi_img {
            let signer = WbiSigner::new(&wbi_img.img_url, &wbi_img.sub_url);

            let mut wbi_signer = self.wbi_signer.write().await;
            *wbi_signer = Some(signer);

            tracing::debug!("WBI签名器初始化成功");
        } else {
            tracing::warn!("导航信息中缺少WBI图片信息");
        }

        Ok(())
    }

    /// 获取WBI签名器
    ///
    /// # 返回
    ///
    /// Option<WbiSigner>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() {
    /// let auth = AuthService::new();
    /// if let Some(signer) = auth.get_wbi_signer().await {
    ///     // 使用签名器
    /// }
    /// # }
    /// ```
    pub async fn get_wbi_signer(&self) -> Option<WbiSigner> {
        let signer = self.wbi_signer.read().await;
        signer.clone()
    }

    /// 获取当前登录用户
    ///
    /// # 返回
    ///
    /// Option<AuthUser>
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() {
    /// let auth = AuthService::new();
    /// if let Some(user) = auth.get_current_user().await {
    ///     println!("当前用户: {}", user.uid);
    /// } else {
    ///     println!("未登录");
    /// }
    /// # }
    /// ```
    pub async fn get_current_user(&self) -> Option<AuthUser> {
        let user = self.current_user.read().await;
        user.clone()
    }

    /// 登出
    ///
    /// 清空当前用户状态、Cookie和WBI签名器
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::services::auth::AuthService;
    /// # async fn example() {
    /// let auth = AuthService::new();
    /// auth.logout().await;
    /// println!("已登出");
    /// # }
    /// ```
    pub async fn logout(&self) {
        // 清空当前用户
        {
            let mut current_user = self.current_user.write().await;
            *current_user = None;
        }

        // 清空Cookie
        {
            let mut client = self.client.write().await;
            *client = BiliClient::new();
        }

        // 清空WBI签名器
        {
            let mut wbi_signer = self.wbi_signer.write().await;
            *wbi_signer = None;
        }

        tracing::info!("用户已登出");
    }

    /// 获取HTTP客户端
    ///
    /// 用于其他服务模块访问已配置的HTTP客户端
    ///
    /// # 返回
    ///
    /// Arc<RwLock<BiliClient>>
    pub fn get_client(&self) -> Arc<RwLock<BiliClient>> {
        Arc::clone(&self.client)
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_user_from_cookie() {
        let cookie = "DedeUserID=123456; bili_jct=abcdef1234567890abcdef1234567890";
        let user = AuthUser::from_cookie(cookie).unwrap();

        assert_eq!(user.uid, "123456");
        assert_eq!(user.bili_jct, "abcdef1234567890abcdef1234567890");
        assert_eq!(user.cookie, cookie);
        assert!(!user.is_cancelled_account);
    }

    #[test]
    fn test_auth_user_from_cookie_missing_uid() {
        let cookie = "bili_jct=abcdef1234567890abcdef1234567890";
        let result = AuthUser::from_cookie(cookie);
        assert!(result.is_err());
    }

    #[test]
    fn test_auth_user_from_cookie_missing_bili_jct() {
        let cookie = "DedeUserID=123456";
        let result = AuthUser::from_cookie(cookie);
        assert!(result.is_err());
    }

    #[test]
    fn test_auth_service_new() {
        let auth = AuthService::new();
        // 验证服务创建成功（结构体字段是私有的，只能验证创建不报错）
    }

    #[tokio::test]
    async fn test_auth_service_logout() {
        let auth = AuthService::new();
        auth.logout().await;

        let user = auth.get_current_user().await;
        assert!(user.is_none());

        let signer = auth.get_wbi_signer().await;
        assert!(signer.is_none());
    }

    #[tokio::test]
    async fn test_get_current_user_initially_none() {
        let auth = AuthService::new();
        let user = auth.get_current_user().await;
        assert!(user.is_none());
    }
}
