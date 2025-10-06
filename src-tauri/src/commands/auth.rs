use crate::api::models::*;
use crate::services::auth::{AuthService, AuthUser};
use tauri::State;

/// 生成登录二维码
///
/// # 返回
///
/// - Ok(QRCode): 包含二维码URL和密钥
/// - Err(String): 错误信息
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const qrcode = await invoke<QRCode>('generate_login_qrcode');
/// console.log('二维码URL:', qrcode.url);
/// console.log('二维码密钥:', qrcode.qrcode_key);
/// ```
#[tauri::command]
pub async fn generate_login_qrcode(auth: State<'_, AuthService>) -> Result<QRCode, String> {
    auth.generate_qrcode()
        .await
        .map_err(|e| e.to_string())
}

/// 轮询登录状态
///
/// 查询二维码是否被扫描以及登录状态
///
/// # 参数
///
/// - `qrcode_key`: 二维码密钥
///
/// # 返回
///
/// - Ok(LoginResult): 登录结果（包含状态码和消息）
/// - Err(String): 错误信息
///
/// # 状态码说明
///
/// - 0: 登录成功
/// - 86038: 二维码已失效
/// - 86101: 二维码未扫描
/// - 86090: 二维码已扫描，等待确认
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const result = await invoke<LoginResult>('poll_login_status', {
///   qrcodeKey: 'your-qrcode-key'
/// });
///
/// if (result.code === 0) {
///   console.log('登录成功!');
/// } else if (result.code === 86101) {
///   console.log('等待扫描...');
/// }
/// ```
#[tauri::command]
pub async fn poll_login_status(
    auth: State<'_, AuthService>,
    qrcode_key: String,
) -> Result<LoginResult, String> {
    auth.poll_qrcode(&qrcode_key)
        .await
        .map_err(|e| e.to_string())
}

/// 使用Cookie登录
///
/// # 参数
///
/// - `cookie`: Cookie字符串（包含DedeUserID和bili_jct）
///
/// # 返回
///
/// - Ok(AuthUser): 登录成功的用户信息
/// - Err(String): 错误信息
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const user = await invoke<AuthUser>('login_with_cookie', {
///   cookie: 'DedeUserID=123456; bili_jct=xxx; SESSDATA=yyy'
/// });
/// console.log('登录成功，用户ID:', user.uid);
/// ```
#[tauri::command]
pub async fn login_with_cookie(
    auth: State<'_, AuthService>,
    cookie: String,
) -> Result<AuthUser, String> {
    auth.login_with_cookie(&cookie)
        .await
        .map_err(|e| e.to_string())
}

/// 获取用户信息
///
/// 获取当前登录用户的导航信息
///
/// # 返回
///
/// - Ok(NavInfo): 用户导航信息
/// - Err(String): 错误信息
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const navInfo = await invoke<NavInfo>('get_user_info');
/// console.log('用户名:', navInfo.uname);
/// console.log('用户ID:', navInfo.mid);
/// ```
#[tauri::command]
pub async fn get_user_info(auth: State<'_, AuthService>) -> Result<NavInfo, String> {
    auth.get_nav_info()
        .await
        .map_err(|e| e.to_string())
}

/// 获取当前用户
///
/// 获取当前登录的用户信息（本地缓存）
///
/// # 返回
///
/// - Ok(Option<AuthUser>): 当前用户（如果已登录）
/// - Err(String): 错误信息（实际上不会返回错误）
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const user = await invoke<AuthUser | null>('get_current_user');
/// if (user) {
///   console.log('当前用户:', user.uid);
/// } else {
///   console.log('未登录');
/// }
/// ```
#[tauri::command]
pub async fn get_current_user(auth: State<'_, AuthService>) -> Result<Option<AuthUser>, String> {
    Ok(auth.get_current_user().await)
}

/// 登出
///
/// 清空当前用户状态和Cookie
///
/// # 返回
///
/// - Ok(()): 登出成功
/// - Err(String): 错误信息（实际上不会返回错误）
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// await invoke('logout');
/// console.log('已登出');
/// ```
#[tauri::command]
pub async fn logout(auth: State<'_, AuthService>) -> Result<(), String> {
    auth.logout().await;
    Ok(())
}
