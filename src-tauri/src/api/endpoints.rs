/// B站API端点定义
///
/// 本模块包含所有B站API的端点URL常量,便于统一管理和维护。
///
/// # 分类
///
/// - 登录认证相关
/// - 用户信息相关
/// - 关注管理相关
/// - 收藏管理相关
/// - 追番追剧相关
/// - 历史记录相关
/// - 稍后再看相关
/// - 黑名单相关

// ==================== 登录认证相关 ====================

/// 生成二维码登录密钥
///
/// 返回: `QRCode` (包含 url 和 qrcode_key)
pub const API_QR_GENERATE: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";

/// 轮询二维码登录状态
///
/// 参数: `qrcode_key`
///
/// 返回: `LoginResult`
pub const API_QR_POLL: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";

/// 获取浏览器指纹
///
/// 用于某些接口的风控验证
pub const API_FINGER_SPI: &str = "https://api.bilibili.com/x/frontend/finger/spi";

// ==================== 用户信息相关 ====================

/// 获取导航栏用户信息
///
/// 返回: `NavInfo` (包含用户基本信息和WBI图片信息)
pub const API_NAV: &str = "https://api.bilibili.com/x/web-interface/nav";

/// 获取用户卡片信息
///
/// 参数: `mid` - 用户ID
///
/// 返回: `UserCard`
pub const API_USER_CARD: &str = "https://api.bilibili.com/x/web-interface/card";

/// 获取用户空间信息
///
/// 参数: `mid` - 用户ID
pub const API_USER_SPACE: &str = "https://api.bilibili.com/x/space/acc/info";

// ==================== 关注管理相关 ====================

/// 获取关注列表
///
/// 参数:
/// - `vmid` - 用户ID
/// - `pn` - 页码
/// - `ps` - 每页数量
/// - `order` - 排序方式 (attention/desc)
///
/// 返回: `PageData<Relation>`
pub const API_FOLLOWING_LIST: &str = "https://api.bilibili.com/x/relation/followings";

/// 获取粉丝列表
///
/// 参数:
/// - `vmid` - 用户ID
/// - `pn` - 页码
/// - `ps` - 每页数量
///
/// 返回: `PageData<Relation>`
pub const API_FOLLOWER_LIST: &str = "https://api.bilibili.com/x/relation/followers";

/// 获取关注分组列表
///
/// 返回: `Vec<RelationTag>`
pub const API_RELATION_TAGS: &str = "https://api.bilibili.com/x/relation/tags";

/// 修改关注关系
///
/// 参数:
/// - `fid` - 目标用户ID
/// - `act` - 操作类型 (1:关注 2:取消关注)
/// - `re_src` - 来源
///
/// 返回: 操作结果
pub const API_RELATION_MODIFY: &str = "https://api.bilibili.com/x/relation/modify";

/// 创建关注分组
///
/// 参数:
/// - `tag` - 分组名称
///
/// 返回: 分组ID
pub const API_TAG_CREATE: &str = "https://api.bilibili.com/x/relation/tag/create";

/// 重命名关注分组
///
/// 参数:
/// - `tagid` - 分组ID
/// - `name` - 新名称
pub const API_TAG_UPDATE: &str = "https://api.bilibili.com/x/relation/tag/update";

/// 删除关注分组
///
/// 参数:
/// - `tagid` - 分组ID
pub const API_TAG_DEL: &str = "https://api.bilibili.com/x/relation/tag/del";

/// 将用户添加到分组
///
/// 参数:
/// - `fids` - 用户ID列表
/// - `tagids` - 分组ID列表
pub const API_TAG_ADD_USERS: &str = "https://api.bilibili.com/x/relation/tags/addUsers";

/// 查询指定用户的特别关注状态
///
/// 参数:
/// - `fid` - 目标用户ID
pub const API_RELATION_SPECIAL: &str = "https://api.bilibili.com/x/relation/special";

// ==================== 收藏管理相关 ====================

/// 获取收藏夹列表
///
/// 参数:
/// - `up_mid` - UP主ID
/// - `type` - 类型 (0:全部)
///
/// 返回: `Vec<FavInfo>`
pub const API_FAV_LIST: &str = "https://api.bilibili.com/x/v3/fav/folder/created/list-all";

/// 获取收藏夹详情
///
/// 参数:
/// - `media_id` - 收藏夹ID
///
/// 返回: `FavFolder`
pub const API_FAV_INFO: &str = "https://api.bilibili.com/x/v3/fav/folder/info";

/// 获取收藏夹内容
///
/// 参数:
/// - `media_id` - 收藏夹ID
/// - `pn` - 页码
/// - `ps` - 每页数量
///
/// 返回: `PageData<Media>`
pub const API_FAV_RESOURCES: &str = "https://api.bilibili.com/x/v3/fav/resource/list";

/// 创建收藏夹
///
/// 参数:
/// - `title` - 收藏夹标题
/// - `intro` - 简介
/// - `privacy` - 是否私密 (0:公开 1:私密)
///
/// 返回: 收藏夹ID
pub const API_FAV_CREATE: &str = "https://api.bilibili.com/x/v3/fav/folder/add";

/// 修改收藏夹信息
///
/// 参数:
/// - `media_id` - 收藏夹ID
/// - `title` - 收藏夹标题
/// - `intro` - 简介
/// - `privacy` - 是否私密
pub const API_FAV_EDIT: &str = "https://api.bilibili.com/x/v3/fav/folder/edit";

/// 删除收藏夹
///
/// 参数:
/// - `media_ids` - 收藏夹ID列表
pub const API_FAV_DEL: &str = "https://api.bilibili.com/x/v3/fav/folder/del";

/// 收藏/取消收藏视频
///
/// 参数:
/// - `rid` - 稿件ID
/// - `type` - 内容类型 (2:视频)
/// - `add_media_ids` - 添加到的收藏夹ID列表
/// - `del_media_ids` - 从中移除的收藏夹ID列表
pub const API_FAV_COLLECT: &str = "https://api.bilibili.com/x/v3/fav/resource/deal";

/// 复制收藏内容到其他收藏夹
///
/// 参数:
/// - `src_media_id` - 源收藏夹ID
/// - `tar_media_id` - 目标收藏夹ID
/// - `mid` - 用户ID
/// - `resources` - 资源ID列表
pub const API_FAV_COPY: &str = "https://api.bilibili.com/x/v3/fav/resource/copy";

/// 移动收藏内容到其他收藏夹
///
/// 参数同上
pub const API_FAV_MOVE: &str = "https://api.bilibili.com/x/v3/fav/resource/move";

/// 批量删除收藏内容
///
/// 参数:
/// - `media_id` - 收藏夹ID
/// - `resources` - 资源ID列表
pub const API_FAV_BATCH_DEL: &str = "https://api.bilibili.com/x/v3/fav/resource/batch-del";

/// 获取收藏的合集/视频列表
///
/// 参数:
/// - `up_mid` - UP主ID
/// - `pn` - 页码
/// - `ps` - 每页数量
pub const API_FAV_COLLECTED_SEASONS: &str = "https://api.bilibili.com/x/v2/fav/season/list";

// ==================== 追番追剧相关 ====================

/// 获取追番追剧列表
///
/// 参数:
/// - `type` - 类型 (1:番剧 2:电影 3:纪录片 4:国创 5:电视剧 7:综艺)
/// - `follow_status` - 追番状态 (1:想看 2:在看 3:看过)
/// - `pn` - 页码
/// - `ps` - 每页数量
///
/// 返回: `PageData<Bangumi>`
pub const API_BANGUMI_LIST: &str = "https://api.bilibili.com/x/space/bangumi/follow/list";

/// 追番/取消追番
///
/// 参数:
/// - `season_id` - 剧集ID
pub const API_BANGUMI_FOLLOW: &str = "https://api.bilibili.com/pgc/web/follow/add";

/// 取消追番
///
/// 参数:
/// - `season_id` - 剧集ID
pub const API_BANGUMI_UNFOLLOW: &str = "https://api.bilibili.com/pgc/web/follow/del";

// ==================== 历史记录相关 ====================

/// 获取历史记录 (游标分页)
///
/// 参数:
/// - `max` - 历史记录起始ID (第一页不传)
/// - `view_at` - 查看时间戳 (第一页不传)
/// - `business` - 业务类型
///
/// 返回: `CursorPageData<History>`
pub const API_HISTORY_LIST: &str = "https://api.bilibili.com/x/web-interface/history/cursor";

/// 删除历史记录
///
/// 参数:
/// - `kid` - 历史记录ID
pub const API_HISTORY_DELETE: &str = "https://api.bilibili.com/x/v2/history/delete";

/// 清空历史记录
pub const API_HISTORY_CLEAR: &str = "https://api.bilibili.com/x/v2/history/clear";

/// 停止记录历史
///
/// 参数:
/// - `switch` - 开关 (true/false)
pub const API_HISTORY_SHADOW: &str = "https://api.bilibili.com/x/v2/history/shadow/set";

// ==================== 稍后再看相关 ====================

/// 获取稍后再看列表
///
/// 返回: `Vec<Video>`
pub const API_TOVIEW_LIST: &str = "https://api.bilibili.com/x/v2/history/toview";

/// 添加稍后再看
///
/// 参数:
/// - `aid` - 稿件ID
pub const API_TOVIEW_ADD: &str = "https://api.bilibili.com/x/v2/history/toview/add";

/// 删除稍后再看
///
/// 参数:
/// - `aid` - 稿件ID (多个用逗号分隔)
pub const API_TOVIEW_DEL: &str = "https://api.bilibili.com/x/v2/history/toview/del";

/// 清空稍后再看
pub const API_TOVIEW_CLEAR: &str = "https://api.bilibili.com/x/v2/history/toview/clear";

// ==================== 黑名单相关 ====================

/// 获取黑名单列表
///
/// 参数:
/// - `pn` - 页码
/// - `ps` - 每页数量
///
/// 返回: `PageData<User>`
pub const API_BLACK_LIST: &str = "https://api.bilibili.com/x/relation/blacks";

/// 添加到黑名单
///
/// 参数:
/// - `fid` - 用户ID
pub const API_BLACK_ADD: &str = "https://api.bilibili.com/x/relation/modify";

/// 从黑名单移除
///
/// 参数:
/// - `fid` - 用户ID
pub const API_BLACK_REMOVE: &str = "https://api.bilibili.com/x/relation/modify";

// ==================== 视频相关 ====================

/// 获取视频详情
///
/// 参数:
/// - `bvid` 或 `aid` - 稿件ID
///
/// 返回: `Video`
pub const API_VIDEO_INFO: &str = "https://api.bilibili.com/x/web-interface/view";

/// 获取视频分P列表
///
/// 参数:
/// - `bvid` 或 `aid` - 稿件ID
///
/// 返回: `Vec<VideoPart>`
pub const API_VIDEO_PAGELIST: &str = "https://api.bilibili.com/x/player/pagelist";

// ==================== 私信相关 ====================

/// 获取会话列表
///
/// 参数:
/// - `session_type` - 会话类型 (1:私信)
/// - `begin_seqno` - 起始序号
/// - `size` - 数量
pub const API_SESSION_LIST: &str = "https://api.vc.bilibili.com/session_svr/v1/session_svr/get_sessions";

/// 获取会话消息
///
/// 参数:
/// - `talker_id` - 对方ID
/// - `session_type` - 会话类型
/// - `begin_seqno` - 起始序号
/// - `size` - 数量
pub const API_SESSION_MSGS: &str = "https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs";

// ==================== 弹幕相关 ====================

/// 获取弹幕列表 (protobuf格式)
///
/// 参数:
/// - `oid` - 视频CID
/// - `type` - 弹幕类型 (1:视频)
/// - `segment_index` - 分段索引 (从1开始)
pub const API_DM_SEG_SO: &str = "https://api.bilibili.com/x/v2/dm/web/seg.so";

// ==================== 工具函数 ====================

/// 构建带查询参数的URL
///
/// # 示例
///
/// ```rust
/// use bilibili_backup_tauri::api::endpoints::{build_url, API_USER_CARD};
///
/// let url = build_url(API_USER_CARD, &[("mid", "123456")]);
/// assert_eq!(url, "https://api.bilibili.com/x/web-interface/card?mid=123456");
/// ```
pub fn build_url(base: &str, params: &[(&str, &str)]) -> String {
    if params.is_empty() {
        return base.to_string();
    }

    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    format!("{}?{}", base, query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_no_params() {
        let url = build_url(API_NAV, &[]);
        assert_eq!(url, API_NAV);
    }

    #[test]
    fn test_build_url_single_param() {
        let url = build_url(API_USER_CARD, &[("mid", "123456")]);
        assert_eq!(url, "https://api.bilibili.com/x/web-interface/card?mid=123456");
    }

    #[test]
    fn test_build_url_multiple_params() {
        let url = build_url(API_FOLLOWING_LIST, &[("vmid", "123"), ("pn", "1"), ("ps", "20")]);
        assert!(url.contains("vmid=123"));
        assert!(url.contains("pn=1"));
        assert!(url.contains("ps=20"));
        assert!(url.starts_with("https://api.bilibili.com/x/relation/followings?"));
    }

    #[test]
    fn test_all_endpoints_https() {
        // 验证所有端点都使用HTTPS
        let endpoints = vec![
            API_QR_GENERATE,
            API_QR_POLL,
            API_NAV,
            API_USER_CARD,
            API_FOLLOWING_LIST,
            API_FOLLOWER_LIST,
            API_RELATION_TAGS,
            API_RELATION_MODIFY,
            API_FAV_LIST,
            API_FAV_RESOURCES,
            API_BANGUMI_LIST,
            API_HISTORY_LIST,
            API_TOVIEW_LIST,
            API_BLACK_LIST,
        ];

        for endpoint in endpoints {
            assert!(endpoint.starts_with("https://"), "端点不是HTTPS: {}", endpoint);
        }
    }
}
