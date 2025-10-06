// 禁用Windows控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::RwLock;
use bilibili_backup_tauri::{
    api::BiliClient,
    services::{
        AuthService,
        FollowingService,
        FollowerService,
        BlacklistService,
        FavoritesService,
        HistoryService,
        BangumiService,
        ToViewService,
    },
    commands,
};
use tracing_subscriber::EnvFilter;

fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with_target(false)
        .init();

    tracing::info!("启动哔哩哔哩账号备份工具 v{}", env!("CARGO_PKG_VERSION"));

    // 创建共享HTTP客户端
    let client = Arc::new(RwLock::new(BiliClient::new()));

    // 创建所有服务实例
    let auth_service = AuthService::new();
    let following_service = FollowingService::new(client.clone());
    let follower_service = FollowerService::new(client.clone());
    let blacklist_service = BlacklistService::new(client.clone());
    let favorites_service = FavoritesService::new(client.clone());
    let history_service = HistoryService::new(client.clone());
    let bangumi_service = BangumiService::new(client.clone());
    let toview_service = ToViewService::new(client.clone());

    // 启动Tauri应用
    tauri::Builder::default()
        // 注册所有服务状态
        .manage(auth_service)
        .manage(following_service)
        .manage(follower_service)
        .manage(blacklist_service)
        .manage(favorites_service)
        .manage(history_service)
        .manage(bangumi_service)
        .manage(toview_service)

        // 注册所有命令
        .invoke_handler(tauri::generate_handler![
            // 基础命令
            commands::greet,
            commands::get_version,

            // 认证命令（6个）
            commands::generate_login_qrcode,
            commands::poll_login_status,
            commands::login_with_cookie,
            commands::get_user_info,
            commands::get_current_user,
            commands::logout,

            // 关注管理命令（5个）
            commands::backup_following,
            commands::restore_following,
            commands::clear_following,
            commands::get_relation_tags,
            commands::create_relation_tag,

            // 粉丝管理命令（1个）
            commands::backup_followers,

            // 黑名单管理命令（3个）
            commands::backup_blacklist,
            commands::restore_blacklist,
            commands::clear_blacklist,

            // 收藏管理命令（3个）
            commands::backup_favorites,
            commands::restore_favorites,
            commands::clear_favorites,

            // 历史记录命令（4个）
            commands::backup_history,
            commands::clear_history,
            commands::export_history,
            commands::import_history,

            // 追番追剧命令（5个）
            commands::backup_bangumi,
            commands::restore_bangumi,
            commands::clear_bangumi,
            commands::export_bangumi,
            commands::import_bangumi,

            // 稍后再看命令（5个）
            commands::backup_toview,
            commands::restore_toview,
            commands::clear_toview,
            commands::export_toview,
            commands::import_toview,
        ])
        .run(tauri::generate_context!())
        .expect("启动Tauri应用失败");
}
