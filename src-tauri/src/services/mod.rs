/// 业务逻辑服务层
///
/// 该模块包含所有业务逻辑的实现，每个服务对应一个功能模块。
/// 各服务模块由不同的Agent负责开发。

/// 认证服务模块
pub mod auth;

/// 历史记录服务模块
pub mod history;

/// 追番追剧服务模块
pub mod bangumi;

/// 稍后再看服务模块
pub mod toview;

/// 关注管理服务模块
pub mod following;

/// 粉丝管理服务模块
pub mod follower;

/// 黑名单管理服务模块
pub mod blacklist;

/// 收藏夹管理服务模块
pub mod favorites;

// 导出常用类型
pub use auth::{AuthService, AuthUser};
pub use bangumi::BangumiService;
pub use blacklist::{BlacklistClearResult, BlacklistRestoreOptions, BlacklistRestoreResult, BlacklistService};
pub use follower::FollowerService;
pub use following::{FollowingClearResult, FollowingRestoreResult, FollowingService, RestoreOptions};
pub use history::HistoryService;
pub use toview::ToViewService;
pub use favorites::{FavoritesService, FavFolderWithMedia, FavRestoreOptions};
