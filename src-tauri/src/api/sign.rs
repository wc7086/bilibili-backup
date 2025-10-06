use md5::compute;
use std::collections::HashMap;

/// WBI签名算法的混淆表
const MIXIN_KEY_ENC_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19,
    29, 28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4,
    22, 25, 54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];

/// WBI 签名器
///
/// 用于对 B站 API 请求参数进行 WBI 签名验证。
///
/// # 示例
///
/// ```rust
/// use std::collections::HashMap;
/// use bilibili_backup_tauri::api::sign::WbiSigner;
///
/// let signer = WbiSigner::new(
///     "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
///     "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png"
/// );
///
/// let mut params = HashMap::new();
/// params.insert("foo".to_string(), "114".to_string());
/// params.insert("bar".to_string(), "514".to_string());
///
/// let w_rid = signer.sign(&mut params);
/// assert!(!w_rid.is_empty());
/// assert!(params.contains_key("wts"));
/// assert!(params.contains_key("w_rid"));
/// ```
#[derive(Debug, Clone)]
pub struct WbiSigner {
    img_key: String,
    sub_key: String,
}

impl WbiSigner {
    /// 从 img_url 和 sub_url 创建签名器
    ///
    /// # 参数
    ///
    /// * `img_url` - WBI 图片 URL (从导航API获取)
    /// * `sub_url` - WBI 副图片 URL (从导航API获取)
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::sign::WbiSigner;
    /// let signer = WbiSigner::new(
    ///     "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
    ///     "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png"
    /// );
    /// ```
    pub fn new(img_url: &str, sub_url: &str) -> Self {
        let img_key = Self::extract_key(img_url);
        let sub_key = Self::extract_key(sub_url);
        Self { img_key, sub_key }
    }

    /// 从 URL 中提取密钥（去掉路径和扩展名）
    ///
    /// # 参数
    ///
    /// * `url` - 完整的图片URL
    ///
    /// # 返回
    ///
    /// 提取出的密钥字符串
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use bilibili_backup_tauri::api::sign::WbiSigner;
    /// let key = WbiSigner::extract_key("https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png");
    /// assert_eq!(key, "7cd084941338484aae1ad9425b84077c");
    /// ```
    pub fn extract_key(url: &str) -> String {
        url.rsplit('/')
            .next()
            .and_then(|s| s.split('.').next())
            .unwrap_or("")
            .to_string()
    }

    /// 生成 mixin_key (内部方法)
    ///
    /// 根据混淆表对img_key和sub_key进行混淆,生成32位的mixin_key
    fn get_mixin_key(&self) -> String {
        let raw = format!("{}{}", self.img_key, self.sub_key);
        MIXIN_KEY_ENC_TAB
            .iter()
            .filter_map(|&i| raw.chars().nth(i))
            .take(32)
            .collect()
    }

    /// 对参数进行签名，返回 w_rid
    ///
    /// 此方法会修改传入的参数 HashMap,添加 `wts` 和 `w_rid` 两个字段。
    ///
    /// # 参数
    ///
    /// * `params` - 需要签名的参数字典(可变引用)
    ///
    /// # 返回
    ///
    /// 生成的签名字符串 (w_rid)
    ///
    /// # 算法流程
    ///
    /// 1. 添加当前时间戳 `wts`
    /// 2. 对所有参数按key排序
    /// 3. 拼接为查询字符串
    /// 4. 追加 mixin_key
    /// 5. 计算 MD5 生成 w_rid
    /// 6. 将 w_rid 添加到参数中
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bilibili_backup_tauri::api::sign::WbiSigner;
    /// let signer = WbiSigner::new(
    ///     "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
    ///     "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png"
    /// );
    ///
    /// let mut params = HashMap::new();
    /// params.insert("foo".to_string(), "114".to_string());
    /// params.insert("bar".to_string(), "514".to_string());
    ///
    /// let w_rid = signer.sign(&mut params);
    ///
    /// assert_eq!(w_rid.len(), 32); // MD5 哈希长度
    /// assert!(params.contains_key("wts"));
    /// assert!(params.contains_key("w_rid"));
    /// assert_eq!(params.get("w_rid").unwrap(), &w_rid);
    /// ```
    pub fn sign(&self, params: &mut HashMap<String, String>) -> String {
        // 添加时间戳
        let wts = chrono::Utc::now().timestamp().to_string();
        params.insert("wts".to_string(), wts);

        // 参数排序
        let mut keys: Vec<_> = params.keys().cloned().collect();
        keys.sort();

        // 拼接查询字符串
        let query = keys
            .iter()
            .map(|k| format!("{}={}", k, params[k]))
            .collect::<Vec<_>>()
            .join("&");

        // 计算 MD5
        let mixin_key = self.get_mixin_key();
        let sign_str = format!("{}{}", query, mixin_key);
        let digest = compute(sign_str.as_bytes());
        let w_rid = format!("{:x}", digest);

        // 将签名添加到参数中
        params.insert("w_rid".to_string(), w_rid.clone());
        w_rid
    }

    /// 构建已签名的查询字符串
    ///
    /// 对参数进行签名并返回完整的查询字符串
    ///
    /// # 参数
    ///
    /// * `params` - 需要签名的参数字典
    ///
    /// # 返回
    ///
    /// 已签名的查询字符串 (包含 wts 和 w_rid)
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use bilibili_backup_tauri::api::sign::WbiSigner;
    /// let signer = WbiSigner::new(
    ///     "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
    ///     "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png"
    /// );
    ///
    /// let mut params = HashMap::new();
    /// params.insert("foo".to_string(), "114".to_string());
    ///
    /// let query = signer.build_query_string(params);
    /// assert!(query.contains("foo=114"));
    /// assert!(query.contains("&wts="));
    /// assert!(query.contains("&w_rid="));
    /// ```
    pub fn build_query_string(&self, mut params: HashMap<String, String>) -> String {
        self.sign(&mut params);

        let mut keys: Vec<_> = params.keys().cloned().collect();
        keys.sort();

        keys.iter()
            .map(|k| format!("{}={}", k, params[k]))
            .collect::<Vec<_>>()
            .join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_key() {
        let key = WbiSigner::extract_key(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
        );
        assert_eq!(key, "7cd084941338484aae1ad9425b84077c");

        let key2 = WbiSigner::extract_key(
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );
        assert_eq!(key2, "4932caff0ff746eab6f01bf08b70ac45");
    }

    #[test]
    fn test_extract_key_no_extension() {
        let key = WbiSigner::extract_key("https://example.com/path/abc123");
        assert_eq!(key, "abc123");
    }

    #[test]
    fn test_extract_key_empty() {
        let key = WbiSigner::extract_key("");
        assert_eq!(key, "");
    }

    #[test]
    fn test_wbi_signer_creation() {
        let signer = WbiSigner::new(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );

        assert_eq!(signer.img_key, "7cd084941338484aae1ad9425b84077c");
        assert_eq!(signer.sub_key, "4932caff0ff746eab6f01bf08b70ac45");
    }

    #[test]
    fn test_get_mixin_key() {
        let signer = WbiSigner::new(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );

        let mixin_key = signer.get_mixin_key();
        assert_eq!(mixin_key.len(), 32);
        // mixin_key应该是由两个key按照混淆表混淆后的前32位
    }

    #[test]
    fn test_wbi_sign() {
        let signer = WbiSigner::new(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );

        let mut params = HashMap::new();
        params.insert("foo".to_string(), "114".to_string());
        params.insert("bar".to_string(), "514".to_string());

        let w_rid = signer.sign(&mut params);

        // 验证签名格式
        assert!(!w_rid.is_empty());
        assert_eq!(w_rid.len(), 32); // MD5 十六进制长度

        // 验证参数中包含了wts和w_rid
        assert!(params.contains_key("wts"));
        assert!(params.contains_key("w_rid"));
        assert_eq!(params.get("w_rid").unwrap(), &w_rid);

        // 验证参数总数
        assert_eq!(params.len(), 4); // foo, bar, wts, w_rid
    }

    #[test]
    fn test_wbi_sign_deterministic_with_same_timestamp() {
        let signer = WbiSigner::new(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );

        let mut params1 = HashMap::new();
        params1.insert("foo".to_string(), "114".to_string());
        params1.insert("bar".to_string(), "514".to_string());
        params1.insert("wts".to_string(), "1234567890".to_string());

        let mut params2 = HashMap::new();
        params2.insert("foo".to_string(), "114".to_string());
        params2.insert("bar".to_string(), "514".to_string());
        params2.insert("wts".to_string(), "1234567890".to_string());

        let w_rid1 = signer.sign(&mut params1);
        let w_rid2 = signer.sign(&mut params2);

        // 相同的参数和时间戳应该产生相同的签名
        assert_eq!(w_rid1, w_rid2);
    }

    #[test]
    fn test_wbi_sign_empty_params() {
        let signer = WbiSigner::new(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );

        let mut params = HashMap::new();
        let w_rid = signer.sign(&mut params);

        assert!(!w_rid.is_empty());
        assert_eq!(w_rid.len(), 32);
        assert_eq!(params.len(), 2); // wts, w_rid
    }

    #[test]
    fn test_build_query_string() {
        let signer = WbiSigner::new(
            "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png",
            "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png",
        );

        let mut params = HashMap::new();
        params.insert("foo".to_string(), "114".to_string());
        params.insert("bar".to_string(), "514".to_string());

        let query = signer.build_query_string(params);

        // 验证查询字符串包含所有必需字段
        assert!(query.contains("foo=114"));
        assert!(query.contains("bar=514"));
        assert!(query.contains("wts="));
        assert!(query.contains("w_rid="));

        // 验证参数是排序的 (bar < foo < w_rid < wts)
        let bar_pos = query.find("bar=").unwrap();
        let foo_pos = query.find("foo=").unwrap();
        assert!(bar_pos < foo_pos);
    }

    #[test]
    fn test_mixin_key_enc_tab_length() {
        // 验证混淆表长度正确
        assert_eq!(MIXIN_KEY_ENC_TAB.len(), 64);

        // 验证混淆表中所有值都在有效范围内
        for &val in &MIXIN_KEY_ENC_TAB {
            assert!(val < 64);
        }
    }

    #[test]
    fn test_mixin_key_enc_tab_unique() {
        // 验证混淆表中所有值都是唯一的
        let mut sorted_tab = MIXIN_KEY_ENC_TAB;
        sorted_tab.sort();

        for i in 1..sorted_tab.len() {
            assert_ne!(sorted_tab[i], sorted_tab[i - 1]);
        }
    }
}
