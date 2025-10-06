/// 关注管理相关命令
///
/// 提供关注列表的备份、还原、清空等功能的Tauri命令

use crate::services::{
    blacklist::{BlacklistClearResult, BlacklistRestoreOptions, BlacklistRestoreResult, BlacklistService},
    follower::FollowerService,
    following::{FollowingClearResult, FollowingRestoreResult, FollowingService, RestoreOptions},
};
use crate::api::models::{Relation, RelationTag, User};
use tauri::State;

// ==================== 关注管理命令 ====================

/// 备份关注列表
///
/// 获取当前用户的所有关注UP主,包含分组信息。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const relations = await invoke<Relation[]>('backup_following');
/// console.log(`备份了 ${relations.length} 个关注`);
/// ```
#[tauri::command]
pub async fn backup_following(
    service: State<'_, FollowingService>,
) -> Result<Vec<Relation>, String> {
    service
        .backup_following()
        .await
        .map_err(|e| format!("备份关注列表失败: {}", e))
}

/// 还原关注列表
///
/// 根据备份数据还原关注关系和分组。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const options = {
///   create_missing_tags: true,
///   continue_on_error: false,
///   batch_size: 20,
///   delay_ms: null
/// };
///
/// const result = await invoke<FollowingRestoreResult>('restore_following', {
///   relations: backupData,
///   options: options
/// });
/// console.log(`成功: ${result.success_count}, 失败: ${result.failed_count}`);
/// ```
#[tauri::command]
pub async fn restore_following(
    service: State<'_, FollowingService>,
    relations: Vec<Relation>,
    options: RestoreOptions,
) -> Result<FollowingRestoreResult, String> {
    service
        .restore_following(relations, options)
        .await
        .map_err(|e| format!("还原关注列表失败: {}", e))
}

/// 清空关注列表
///
/// 取消所有关注。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const result = await invoke<FollowingClearResult>('clear_following');
/// console.log(`取消了 ${result.success_count} 个关注`);
/// ```
#[tauri::command]
pub async fn clear_following(
    service: State<'_, FollowingService>,
) -> Result<FollowingClearResult, String> {
    service
        .clear_following()
        .await
        .map_err(|e| format!("清空关注列表失败: {}", e))
}

/// 获取关注分组列表
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const tags = await invoke<RelationTag[]>('get_relation_tags');
/// for (const tag of tags) {
///   console.log(`分组: ${tag.name} (ID: ${tag.tag_id})`);
/// }
/// ```
#[tauri::command]
pub async fn get_relation_tags(
    service: State<'_, FollowingService>,
) -> Result<Vec<RelationTag>, String> {
    service
        .get_relation_tags()
        .await
        .map_err(|e| format!("获取分组列表失败: {}", e))
}

/// 创建关注分组
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const tagId = await invoke<number>('create_relation_tag', {
///   tagName: '我的分组'
/// });
/// console.log(`创建分组成功, ID: ${tagId}`);
/// ```
#[tauri::command]
pub async fn create_relation_tag(
    service: State<'_, FollowingService>,
    tag_name: String,
) -> Result<i64, String> {
    service
        .create_tag(&tag_name)
        .await
        .map_err(|e| format!("创建分组失败: {}", e))
}

// ==================== 粉丝管理命令 ====================

/// 备份粉丝列表
///
/// 获取当前用户的所有粉丝。
/// 注意: B站API不支持还原粉丝列表,此命令仅用于备份。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const followers = await invoke<Relation[]>('backup_followers');
/// console.log(`备份了 ${followers.length} 个粉丝`);
/// ```
#[tauri::command]
pub async fn backup_followers(
    service: State<'_, FollowerService>,
) -> Result<Vec<Relation>, String> {
    service
        .backup_followers()
        .await
        .map_err(|e| format!("备份粉丝列表失败: {}", e))
}

// ==================== 黑名单管理命令 ====================

/// 备份黑名单
///
/// 获取当前用户的所有黑名单用户。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const blacklist = await invoke<User[]>('backup_blacklist');
/// console.log(`备份了 ${blacklist.length} 个黑名单用户`);
/// ```
#[tauri::command]
pub async fn backup_blacklist(
    service: State<'_, BlacklistService>,
) -> Result<Vec<User>, String> {
    service
        .backup_blacklist()
        .await
        .map_err(|e| format!("备份黑名单失败: {}", e))
}

/// 还原黑名单
///
/// 根据备份数据还原黑名单。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const options = {
///   continue_on_error: false,
///   batch_size: 20,
///   delay_ms: null
/// };
///
/// const result = await invoke<BlacklistRestoreResult>('restore_blacklist', {
///   users: backupData,
///   options: options
/// });
/// console.log(`成功: ${result.success_count}, 失败: ${result.failed_count}`);
/// ```
#[tauri::command]
pub async fn restore_blacklist(
    service: State<'_, BlacklistService>,
    users: Vec<User>,
    options: BlacklistRestoreOptions,
) -> Result<BlacklistRestoreResult, String> {
    service
        .restore_blacklist(users, options)
        .await
        .map_err(|e| format!("还原黑名单失败: {}", e))
}

/// 清空黑名单
///
/// 移除所有黑名单用户。
///
/// # 前端调用示例
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const result = await invoke<BlacklistClearResult>('clear_blacklist');
/// console.log(`移除了 ${result.success_count} 个黑名单用户`);
/// ```
#[tauri::command]
pub async fn clear_blacklist(
    service: State<'_, BlacklistService>,
) -> Result<BlacklistClearResult, String> {
    service
        .clear_blacklist()
        .await
        .map_err(|e| format!("清空黑名单失败: {}", e))
}
