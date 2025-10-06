/**
 * API 工具函数
 *
 * 封装所有与后端的通信逻辑
 */

import { invoke } from '@tauri-apps/api/tauri';
import { open, save } from '@tauri-apps/api/dialog';
import type {
  Relation,
  RelationTag,
  Follower,
  BlacklistUser,
  FavFolder,
  History,
  Bangumi,
  ToView,
  RestoreResult,
  ClearResult,
  RestoreOptions,
  RestoreBlacklistOptions,
  RestoreFavoritesOptions,
} from '../types/api';

// ==================== 关注管理 ====================

export async function backupFollowing(): Promise<Relation[]> {
  return invoke<Relation[]>('backup_following');
}

export async function restoreFollowing(
  relations: Relation[],
  options?: RestoreOptions
): Promise<RestoreResult> {
  return invoke<RestoreResult>('restore_following', { relations, options });
}

export async function clearFollowing(): Promise<ClearResult> {
  return invoke<ClearResult>('clear_following');
}

export async function getRelationTags(): Promise<RelationTag[]> {
  return invoke<RelationTag[]>('get_relation_tags');
}

export async function createRelationTag(tagName: string): Promise<RelationTag> {
  return invoke<RelationTag>('create_relation_tag', { tagName });
}

// ==================== 粉丝管理 ====================

export async function backupFollowers(): Promise<Follower[]> {
  return invoke<Follower[]>('backup_followers');
}

// ==================== 黑名单管理 ====================

export async function backupBlacklist(): Promise<BlacklistUser[]> {
  return invoke<BlacklistUser[]>('backup_blacklist');
}

export async function restoreBlacklist(
  users: BlacklistUser[],
  options?: RestoreBlacklistOptions
): Promise<RestoreResult> {
  return invoke<RestoreResult>('restore_blacklist', { users, options });
}

export async function clearBlacklist(): Promise<ClearResult> {
  return invoke<ClearResult>('clear_blacklist');
}

// ==================== 收藏管理 ====================

export async function backupFavorites(): Promise<FavFolder[]> {
  return invoke<FavFolder[]>('backup_favorites');
}

export async function restoreFavorites(
  folders: FavFolder[],
  options?: RestoreFavoritesOptions
): Promise<RestoreResult> {
  return invoke<RestoreResult>('restore_favorites', { folders, options });
}

export async function clearFavorites(): Promise<ClearResult> {
  return invoke<ClearResult>('clear_favorites');
}

// ==================== 历史记录 ====================

export async function backupHistory(): Promise<History[]> {
  return invoke<History[]>('backup_history');
}

export async function clearHistory(): Promise<ClearResult> {
  return invoke<ClearResult>('clear_history');
}

export async function exportHistory(
  history: History[],
  filePath: string
): Promise<void> {
  return invoke<void>('export_history', { history, filePath });
}

export async function importHistory(filePath: string): Promise<History[]> {
  return invoke<History[]>('import_history', { filePath });
}

// ==================== 追番管理 ====================

export async function backupBangumi(type_: number): Promise<Bangumi[]> {
  return invoke<Bangumi[]>('backup_bangumi', { type_ });
}

export async function restoreBangumi(
  bangumiList: Bangumi[]
): Promise<RestoreResult> {
  return invoke<RestoreResult>('restore_bangumi', { bangumiList });
}

export async function clearBangumi(type_: number): Promise<ClearResult> {
  return invoke<ClearResult>('clear_bangumi', { type_ });
}

export async function exportBangumi(
  bangumiList: Bangumi[],
  filePath: string
): Promise<void> {
  return invoke<void>('export_bangumi', { bangumiList, filePath });
}

export async function importBangumi(filePath: string): Promise<Bangumi[]> {
  return invoke<Bangumi[]>('import_bangumi', { filePath });
}

// ==================== 稍后再看 ====================

export async function backupToview(): Promise<ToView[]> {
  return invoke<ToView[]>('backup_toview');
}

export async function restoreToview(videos: ToView[]): Promise<RestoreResult> {
  return invoke<RestoreResult>('restore_toview', { videos });
}

export async function clearToview(): Promise<ClearResult> {
  return invoke<ClearResult>('clear_toview');
}

export async function exportToview(
  videos: ToView[],
  filePath: string
): Promise<void> {
  return invoke<void>('export_toview', { videos, filePath });
}

export async function importToview(filePath: string): Promise<ToView[]> {
  return invoke<ToView[]>('import_toview', { filePath });
}

// ==================== 文件对话框 ====================

export async function selectJsonFile(): Promise<string | null> {
  const selected = await open({
    filters: [
      {
        name: 'JSON',
        extensions: ['json'],
      },
    ],
    multiple: false,
  });

  return typeof selected === 'string' ? selected : null;
}

export async function saveJsonFile(): Promise<string | null> {
  return await save({
    filters: [
      {
        name: 'JSON',
        extensions: ['json'],
      },
    ],
  });
}
