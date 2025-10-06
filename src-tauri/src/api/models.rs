use serde::{Deserialize, Serialize};

// ==================== 通用响应和分页结构 ====================

/// API统一响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    /// 状态码：0表示成功
    pub code: i32,
    /// 响应消息
    pub message: String,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    /// 判断是否成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// 判断是否失败
    pub fn is_fail(&self) -> bool {
        self.code != 0
    }

    /// 获取数据或返回错误
    pub fn into_data(self) -> Result<T, crate::api::error::BiliError> {
        if self.is_success() {
            self.data
                .ok_or_else(|| crate::api::error::BiliError::api("响应数据为空"))
        } else {
            Err(crate::api::error::BiliError::api(format!(
                "API错误 [{}]: {}",
                self.code, self.message
            )))
        }
    }
}

/// 普通分页数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData<T> {
    /// 数据列表
    pub list: Vec<T>,
    /// 总数
    pub total: usize,
    /// 当前页码 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pn: Option<usize>,
    /// 每页大小 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<usize>,
}

impl<T> PageData<T> {
    /// 获取数据项
    pub fn items(&self) -> &[T] {
        &self.list
    }

    /// 是否还有更多数据
    pub fn has_more(&self, current_total: usize) -> bool {
        current_total < self.total
    }
}

/// 游标分页数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPageData<T> {
    /// 数据列表
    pub list: Vec<T>,
    /// 下一页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// 是否还有更多数据
    #[serde(default)]
    pub has_more: bool,
}

impl<T> CursorPageData<T> {
    /// 获取数据项
    pub fn items(&self) -> &[T] {
        &self.list
    }
}

/// 普通分页数据包装(兼容B站API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalPageData<T> {
    /// 数据列表
    pub list: Vec<T>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

// ==================== 用户相关 ====================

/// 用户基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    pub mid: u64,
    /// 用户名
    pub uname: String,
    /// 头像URL
    pub face: String,
    /// 个性签名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    /// 性别 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 等级 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

/// UP主信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upper {
    /// UP主ID
    pub mid: u64,
    /// UP主名称
    pub name: String,
    /// 头像URL
    pub face: String,
    /// 性别 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 等级 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 是否无头像 (可选)
    #[serde(skip_serializing_if = "Option::is_none", rename = "noFace")]
    pub no_face: Option<bool>,
}

/// VIP会员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vip {
    /// VIP类型
    #[serde(skip_serializing_if = "Option::is_none", rename = "vipType")]
    pub vip_type: Option<i32>,
    /// VIP到期时间
    #[serde(skip_serializing_if = "Option::is_none", rename = "vipDueDate")]
    pub vip_due_date: Option<i64>,
    /// 到期提醒
    #[serde(skip_serializing_if = "Option::is_none", rename = "dueRemark")]
    pub due_remark: Option<String>,
    /// VIP状态
    #[serde(skip_serializing_if = "Option::is_none", rename = "vipStatus")]
    pub vip_status: Option<i32>,
    /// 昵称颜色
    #[serde(skip_serializing_if = "Option::is_none", rename = "nicknameColor")]
    pub nickname_color: Option<String>,
}

// ==================== 关注相关 ====================

/// 关注数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// 用户ID
    pub mid: u64,
    /// 用户名
    pub uname: String,
    /// 头像
    pub face: String,
    /// 签名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    /// 修改时间（时间戳）
    pub mtime: i64,
    /// 关注属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<i32>,
    /// 特别关注标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<i32>,
    /// 所属分组ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<i64>>,
    /// VIP信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<Vip>,
}

/// 关注分组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationTag {
    /// 分组ID
    #[serde(rename = "tagid")]
    pub tag_id: i64,
    /// 分组名称
    pub name: String,
    /// 分组内关注数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    /// 提示信息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,
}

impl RelationTag {
    /// 创建新分组
    pub fn new(tag_id: i64, name: String) -> Self {
        Self {
            tag_id,
            name,
            count: None,
            tip: None,
        }
    }
}

/// 关注操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationAct {
    /// 操作类型
    pub act: i32,
    /// 分组ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fids: Option<Vec<i64>>,
}

// ==================== 收藏相关 ====================

/// 收藏夹元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavInfo {
    /// 收藏夹ID
    pub id: u64,
    /// 收藏夹fid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fid: Option<u64>,
    /// 创建者mid
    pub mid: u64,
    /// 属性标记位
    pub attr: i32,
    /// 收藏夹标题
    pub title: String,
    /// 封面URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctime: Option<i64>,
    /// 收藏数量
    #[serde(rename = "mediaCount")]
    pub media_count: u32,
}

impl FavInfo {
    /// 是否为默认收藏夹
    pub fn is_default(&self) -> bool {
        (self.attr >> 1 & 1) != 1
    }

    /// 获取剩余可收藏数量
    pub fn remaining_count(&self) -> i32 {
        if self.is_default() {
            50000 - self.media_count as i32
        } else {
            1000 - self.media_count as i32
        }
    }
}

/// 收藏夹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavFolder {
    /// 基础信息
    #[serde(flatten)]
    pub info: FavInfo,
    /// 简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 收藏状态
    #[serde(skip_serializing_if = "Option::is_none", rename = "favState")]
    pub fav_state: Option<i32>,
    /// 收藏的媒体列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medias: Option<Vec<Media>>,
}

/// 状态数信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CntInfo {
    /// 收藏数
    pub collect: i32,
    /// 播放数
    pub play: i32,
    /// 弹幕数
    pub danmaku: i32,
    /// 分享数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<i32>,
    /// 点赞数
    #[serde(rename = "thumb_up")]
    pub thumb_up: i32,
}

/// 收藏的媒体项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    /// 项目ID
    pub id: u64,
    /// 类型（2:视频 12:专栏等）
    #[serde(rename = "type")]
    pub item_type: u32,
    /// 标题
    pub title: String,
    /// 封面URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 分P数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// 时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// UP主信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper: Option<Upper>,
    /// 属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<i32>,
    /// 状态数
    #[serde(skip_serializing_if = "Option::is_none", rename = "cntInfo")]
    pub cnt_info: Option<CntInfo>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctime: Option<i64>,
    /// 发布时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubtime: Option<i64>,
    /// 收藏时间
    #[serde(skip_serializing_if = "Option::is_none", rename = "favTime")]
    pub fav_time: Option<i64>,
    /// BV号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvid: Option<String>,
    /// BV ID (兼容字段)
    #[serde(skip_serializing_if = "Option::is_none", rename = "bvId")]
    pub bv_id: Option<String>,
}

// ==================== 追番追剧 ====================

/// 番剧/追剧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bangumi {
    /// 剧集season_id
    #[serde(rename = "seasonId")]
    pub season_id: u64,
    /// 剧集media_id
    #[serde(rename = "mediaId")]
    pub media_id: u64,
    /// 剧集类型
    #[serde(skip_serializing_if = "Option::is_none", rename = "seasonType")]
    pub season_type: Option<String>,
    /// 剧集类型名称
    #[serde(skip_serializing_if = "Option::is_none", rename = "seasonTypeName")]
    pub season_type_name: Option<String>,
    /// 标题
    pub title: String,
    /// 封面URL
    pub cover: String,
    /// 总集数
    #[serde(skip_serializing_if = "Option::is_none", rename = "totalCount")]
    pub total_count: Option<i32>,
    /// 徽章文字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
    /// 徽章类型
    #[serde(skip_serializing_if = "Option::is_none", rename = "badgeType")]
    pub badge_type: Option<i32>,
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 追番状态
    #[serde(skip_serializing_if = "Option::is_none", rename = "followStatus")]
    pub follow_status: Option<i32>,
}

// ==================== 历史记录 ====================

/// 历史记录业务类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryBusiness {
    /// 业务类型标识
    pub business: String,
}

/// 历史记录项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    /// 目标ID
    pub oid: u64,
    /// 剧集epid (仅用于剧集)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epid: Option<u64>,
    /// 稿件bvid (仅用于稿件视频)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvid: Option<String>,
    /// 观看到的视频分P数 (仅用于稿件视频)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// 观看到的对象ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<u64>,
    /// 观看到的视频分P标题 (仅用于稿件视频)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<String>,
    /// 业务类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
    /// 记录查看的平台代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<i32>,
}

/// 历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    /// 标题
    pub title: String,
    /// 封面URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// 重定向URL (仅用于剧集和直播)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// 历史记录项详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryItem>,
    /// 视频分P数目 (仅用于稿件视频)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<i32>,
    /// 作者名称
    #[serde(skip_serializing_if = "Option::is_none", rename = "authorName")]
    pub author_name: Option<String>,
    /// 作者mid
    #[serde(skip_serializing_if = "Option::is_none", rename = "authorMid")]
    pub author_mid: Option<u64>,
    /// 查看时间（时间戳）
    #[serde(skip_serializing_if = "Option::is_none", rename = "viewAt")]
    pub view_at: Option<i64>,
    /// 视频观看进度（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 分P标题
    #[serde(skip_serializing_if = "Option::is_none", rename = "showTitle")]
    pub show_title: Option<String>,
    /// 视频总时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 条目目标ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<u64>,
}

// ==================== 视频相关 ====================

/// 视频稿件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    /// 稿件avid
    pub aid: u64,
    /// 稿件bvid
    pub bvid: String,
    /// 封面URL
    pub pic: String,
    /// 标题
    pub title: String,
    /// 发布时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubdate: Option<i64>,
    /// 时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// UP主信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Upper>,
}

/// 视频分P信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPart {
    /// 分P的cid
    pub cid: u64,
    /// 分P序号
    pub page: i32,
    /// 分P标题
    pub part: String,
    /// 分P时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

// ==================== 登录相关 ====================

/// 二维码信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRCode {
    /// 二维码URL
    pub url: String,
    /// 二维码密钥
    #[serde(rename = "qrcode_key")]
    pub qrcode_key: String,
}

/// 登录结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    /// 跳转URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 刷新令牌
    #[serde(skip_serializing_if = "Option::is_none", rename = "refresh_token")]
    pub refresh_token: Option<String>,
    /// 时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// 状态码
    pub code: i32,
    /// 消息
    pub message: String,
}

impl LoginResult {
    /// 是否登录成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

/// 导航信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavInfo {
    /// 用户mid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<u64>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uname: Option<String>,
    /// 头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
    /// 是否登录
    #[serde(skip_serializing_if = "Option::is_none", rename = "isLogin")]
    pub is_login: Option<bool>,
    /// WBI图片信息
    #[serde(skip_serializing_if = "Option::is_none", rename = "wbi_img")]
    pub wbi_img: Option<WbiImg>,
}

/// WBI签名图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WbiImg {
    /// img_url
    #[serde(rename = "img_url")]
    pub img_url: String,
    /// sub_url
    #[serde(rename = "sub_url")]
    pub sub_url: String,
}

// ==================== 稍后再看 ====================

/// 稍后再看视频
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToView {
    /// 稿件avid
    pub aid: u64,
    /// 稿件bvid (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvid: Option<String>,
    /// 视频cid
    pub cid: u64,
    /// 标题
    pub title: String,
    /// 封面URL
    pub pic: String,
    /// UP主信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Upper>,
    /// 添加时间
    #[serde(skip_serializing_if = "Option::is_none", rename = "add_at")]
    pub add_at: Option<i64>,
    /// 视频时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 视频状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    /// 视频分P数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<i32>,
}

/// 稍后再看列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToViewList {
    /// 稍后再看数量
    pub count: i32,
    /// 稍后再看列表
    pub list: Vec<ToView>,
}

// ==================== 操作结果 ====================

/// 清除操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearResult {
    /// 成功清除的数量
    pub cleared_count: usize,
    /// 操作消息
    pub message: String,
}

/// 还原操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    /// 成功还原的数量
    pub success_count: usize,
    /// 失败的数量
    pub failed_count: usize,
    /// 总数
    pub total_count: usize,
    /// 失败的项目 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<String>>,
    /// 操作消息
    pub message: String,
}

// ==================== 其他数据结构 ====================

/// 游标信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    /// 下一页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// 上一页游标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
}

/// 用户卡片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCard {
    /// 用户mid
    pub mid: u64,
    /// 用户名
    pub name: String,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 头像
    pub face: String,
    /// 个性签名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    /// 等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

/// 已注销账号信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelledAccountInfo {
    /// 是否已注销
    pub is_cancelled: bool,
    /// 注销提示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_result_success() {
        let result = ApiResult {
            code: 0,
            message: "success".to_string(),
            data: Some("test_data".to_string()),
        };
        assert!(result.is_success());
        assert!(!result.is_fail());
        assert_eq!(result.into_data().unwrap(), "test_data");
    }

    #[test]
    fn test_api_result_error() {
        let result: ApiResult<String> = ApiResult {
            code: -1,
            message: "error".to_string(),
            data: None,
        };
        assert!(!result.is_success());
        assert!(result.is_fail());
        assert!(result.into_data().is_err());
    }

    #[test]
    fn test_fav_info_is_default() {
        let fav_default = FavInfo {
            id: 1,
            fid: Some(1),
            mid: 123,
            attr: 0, // 默认收藏夹
            title: "默认收藏夹".to_string(),
            cover: None,
            ctime: None,
            media_count: 100,
        };
        assert!(fav_default.is_default());

        let fav_other = FavInfo {
            id: 2,
            fid: Some(2),
            mid: 123,
            attr: 22, // 其他收藏夹
            title: "其他收藏夹".to_string(),
            cover: None,
            ctime: None,
            media_count: 50,
        };
        assert!(!fav_other.is_default());
    }

    #[test]
    fn test_fav_info_remaining_count() {
        let fav_default = FavInfo {
            id: 1,
            fid: Some(1),
            mid: 123,
            attr: 0,
            title: "默认收藏夹".to_string(),
            cover: None,
            ctime: None,
            media_count: 100,
        };
        assert_eq!(fav_default.remaining_count(), 49900);

        let fav_other = FavInfo {
            id: 2,
            fid: Some(2),
            mid: 123,
            attr: 22,
            title: "其他收藏夹".to_string(),
            cover: None,
            ctime: None,
            media_count: 50,
        };
        assert_eq!(fav_other.remaining_count(), 950);
    }

    #[test]
    fn test_login_result_is_success() {
        let success = LoginResult {
            url: Some("https://www.bilibili.com".to_string()),
            refresh_token: Some("token".to_string()),
            timestamp: Some(1234567890),
            code: 0,
            message: "success".to_string(),
        };
        assert!(success.is_success());

        let fail = LoginResult {
            url: None,
            refresh_token: None,
            timestamp: None,
            code: -1,
            message: "failed".to_string(),
        };
        assert!(!fail.is_success());
    }

    #[test]
    fn test_page_data_has_more() {
        let page = PageData {
            list: vec![1, 2, 3],
            total: 100,
            pn: Some(1),
            ps: Some(20),
        };
        assert!(page.has_more(3));
        assert!(page.has_more(99));
        assert!(!page.has_more(100));
        assert!(!page.has_more(101));
    }

    #[test]
    fn test_relation_tag_new() {
        let tag = RelationTag::new(1, "测试分组".to_string());
        assert_eq!(tag.tag_id, 1);
        assert_eq!(tag.name, "测试分组");
        assert!(tag.count.is_none());
        assert!(tag.tip.is_none());
    }
}
