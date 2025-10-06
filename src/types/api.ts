/**
 * API 类型定义
 *
 * 与 Rust 后端的数据结构保持一致
 */

// ==================== 认证相关 ====================

export * from './auth';

// ==================== 关注相关 ====================

export interface Relation {
  mid: number;
  uname: string;
  face: string;
  sign?: string;
  mtime: number;
  attribute?: number;
  special?: number;
  tag?: number[];
  vip?: Vip;
}

export interface RelationTag {
  tagid: number;
  name: string;
  count?: number;
  tip?: string;
}

export interface Vip {
  vipType?: number;
  vipDueDate?: number;
  dueRemark?: string;
  vipStatus?: number;
  nicknameColor?: string;
}

// ==================== 粉丝相关 ====================

export interface Follower {
  mid: number;
  uname: string;
  face: string;
  sign?: string;
  mtime: number;
  attribute?: number;
}

// ==================== 黑名单相关 ====================

export interface BlacklistUser {
  mid: number;
  uname: string;
  face: string;
}

// ==================== 收藏相关 ====================

export interface FavFolder {
  id: number;
  fid?: number;
  mid: number;
  attr: number;
  title: string;
  cover?: string;
  ctime?: number;
  mediaCount: number;
  intro?: string;
  favState?: number;
  medias?: Media[];
}

export interface Media {
  id: number;
  type: number;
  title: string;
  cover?: string;
  intro?: string;
  page?: number;
  duration?: number;
  upper?: Upper;
  attr?: number;
  cntInfo?: CntInfo;
  link?: string;
  ctime?: number;
  pubtime?: number;
  favTime?: number;
  bvid?: string;
  bvId?: string;
}

export interface Upper {
  mid: number;
  name: string;
  face: string;
  sex?: string;
  level?: number;
  noFace?: boolean;
}

export interface CntInfo {
  collect: number;
  play: number;
  danmaku: number;
  share?: number;
  thumb_up: number;
}

// ==================== 历史记录相关 ====================

export interface History {
  title: string;
  cover?: string;
  uri?: string;
  history?: HistoryItem;
  videos?: number;
  authorName?: string;
  authorMid?: number;
  viewAt?: number;
  progress?: number;
  showTitle?: string;
  duration?: number;
  kid?: number;
}

export interface HistoryItem {
  oid: number;
  epid?: number;
  bvid?: string;
  page?: number;
  cid?: number;
  part?: string;
  business?: string;
  dt?: number;
}

// ==================== 追番追剧相关 ====================

export interface Bangumi {
  seasonId: number;
  mediaId: number;
  seasonType?: string;
  seasonTypeName?: string;
  title: string;
  cover: string;
  totalCount?: number;
  badge?: string;
  badgeType?: number;
  url?: string;
  followStatus?: number;
}

// ==================== 稍后再看相关 ====================

export interface ToView {
  aid: number;
  bvid?: string;
  cid: number;
  title: string;
  pic: string;
  owner?: Upper;
  add_at?: number;
  duration?: number;
  state?: number;
  videos?: number;
}

// ==================== 操作结果 ====================

export interface RestoreResult {
  success_count: number;
  failed_count: number;
  total_count: number;
  failed_items?: string[];
  message: string;
}

export interface ClearResult {
  cleared_count: number;
  message: string;
}

// ==================== 操作选项 ====================

export interface RestoreOptions {
  create_missing_tags?: boolean;
  continue_on_error?: boolean;
  batch_size?: number;
  delay_ms?: number | null;
}

export interface RestoreBlacklistOptions {
  continue_on_error?: boolean;
  batch_size?: number;
  delay_ms?: number | null;
}

export interface RestoreFavoritesOptions {
  continue_on_error?: boolean;
  batch_size?: number;
  delay_ms?: number | null;
}
