use thiserror::Error;

/// 统一的错误类型定义
#[derive(Error, Debug)]
pub enum BiliError {
    /// 网络请求失败
    #[error("网络请求失败: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// API错误（B站返回的错误）
    #[error("API错误: {0}")]
    ApiError(String),

    /// JSON解析失败
    #[error("JSON解析失败: {0}")]
    JsonError(#[from] serde_json::Error),

    /// 业务逻辑错误
    #[error("业务错误: {0}")]
    BusinessError(String),

    /// 认证失败
    #[error("认证失败: {0}")]
    AuthError(String),

    /// 参数错误
    #[error("参数错误: {0}")]
    ParamError(String),

    /// IO错误
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}

/// 统一的Result类型
pub type Result<T> = std::result::Result<T, BiliError>;

impl BiliError {
    /// 创建API错误
    pub fn api(msg: impl Into<String>) -> Self {
        Self::ApiError(msg.into())
    }

    /// 创建业务错误
    pub fn business(msg: impl Into<String>) -> Self {
        Self::BusinessError(msg.into())
    }

    /// 创建认证错误
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::AuthError(msg.into())
    }

    /// 创建参数错误
    pub fn param(msg: impl Into<String>) -> Self {
        Self::ParamError(msg.into())
    }

    /// 创建IO错误
    pub fn io(msg: impl Into<String>) -> Self {
        Self::IoError(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
    }

    /// 创建解析错误
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::ParamError(format!("解析错误: {}", msg.into()))
    }
}

/// 将错误转换为可序列化的字符串（用于Tauri命令返回）
impl From<BiliError> for String {
    fn from(err: BiliError) -> Self {
        err.to_string()
    }
}
