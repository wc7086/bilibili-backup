# 历史记录服务集成文档

本文档描述了历史记录相关服务（HistoryService、BangumiService、ToViewService）的实现和集成方式。

## 模块概览

### 1. HistoryService（历史记录服务）

**文件位置**: `/src-tauri/src/services/history.rs`

**核心功能**:
- 备份观看历史记录
- 清空历史记录
- 导入/导出历史记录到JSON文件

**API端点**:
- `GET /x/web-interface/history/cursor` - 获取历史记录（游标分页）
- `POST /x/v2/history/clear` - 清空历史记录

**关键特性**:
- 使用游标分页获取所有历史记录
- 支持不同业务类型（视频、直播、专栏等）
- 自动处理分页和防风控延迟
- **注意**: B站API不支持还原历史记录

**数据结构**:
```rust
pub struct History {
    pub title: String,
    pub cover: Option<String>,
    pub uri: Option<String>,
    pub history: Option<HistoryItem>,
    pub videos: Option<i32>,
    pub author_name: Option<String>,
    pub author_mid: Option<u64>,
    pub view_at: Option<i64>,
    pub progress: Option<i32>,
    pub show_title: Option<String>,
    pub duration: Option<i32>,
    pub kid: Option<u64>,
}
```

### 2. BangumiService（追番追剧服务）

**文件位置**: `/src-tauri/src/services/bangumi.rs`

**核心功能**:
- 备份追番追剧列表
- 还原追番追剧
- 清空追番追剧列表
- 导入/导出追番列表到JSON文件

**API端点**:
- `GET /x/space/bangumi/follow/list` - 获取追番列表
- `POST /pgc/web/follow/add` - 追番
- `POST /pgc/web/follow/del` - 取消追番

**类型参数**:
- `1` - 番剧
- `2` - 电影
- `3` - 纪录片
- `4` - 国创
- `5` - 电视剧
- `7` - 综艺

**数据结构**:
```rust
pub struct Bangumi {
    pub season_id: u64,
    pub media_id: u64,
    pub season_type: Option<String>,
    pub season_type_name: Option<String>,
    pub title: String,
    pub cover: String,
    pub total_count: Option<i32>,
    pub badge: Option<String>,
    pub badge_type: Option<i32>,
    pub url: Option<String>,
    pub follow_status: Option<i32>,
}
```

### 3. ToViewService（稍后再看服务）

**文件位置**: `/src-tauri/src/services/toview.rs`

**核心功能**:
- 备份稍后再看列表
- 还原稍后再看
- 清空稍后再看
- 导入/导出稍后再看列表到JSON文件

**API端点**:
- `GET /x/v2/history/toview` - 获取稍后再看列表
- `POST /x/v2/history/toview/add` - 添加稍后再看
- `POST /x/v2/history/toview/del` - 删除稍后再看
- `POST /x/v2/history/toview/clear` - 清空稍后再看

**数据结构**:
```rust
pub struct ToView {
    pub aid: u64,
    pub bvid: Option<String>,
    pub cid: u64,
    pub title: String,
    pub pic: String,
    pub owner: Option<Upper>,
    pub add_at: Option<i64>,
    pub duration: Option<i32>,
    pub state: Option<i32>,
    pub videos: Option<i32>,
}
```

## Tauri命令

**文件位置**: `/src-tauri/src/commands/history.rs`

### 历史记录命令

```rust
// 备份历史记录
#[tauri::command]
pub async fn backup_history(
    service: State<'_, HistoryService>,
) -> Result<Vec<History>, String>

// 清空历史记录
#[tauri::command]
pub async fn clear_history(
    service: State<'_, HistoryService>,
) -> Result<ClearResult, String>

// 导出历史记录
#[tauri::command]
pub async fn export_history(
    service: State<'_, HistoryService>,
    history: Vec<History>,
    file_path: String,
) -> Result<(), String>

// 导入历史记录
#[tauri::command]
pub async fn import_history(
    service: State<'_, HistoryService>,
    file_path: String,
) -> Result<Vec<History>, String>
```

### 追番追剧命令

```rust
// 备份追番列表
#[tauri::command]
pub async fn backup_bangumi(
    service: State<'_, BangumiService>,
    type_: i32,
) -> Result<Vec<Bangumi>, String>

// 还原追番列表
#[tauri::command]
pub async fn restore_bangumi(
    service: State<'_, BangumiService>,
    bangumi_list: Vec<Bangumi>,
) -> Result<RestoreResult, String>

// 清空追番列表
#[tauri::command]
pub async fn clear_bangumi(
    service: State<'_, BangumiService>,
    type_: i32,
) -> Result<ClearResult, String>

// 导出追番列表
#[tauri::command]
pub async fn export_bangumi(
    service: State<'_, BangumiService>,
    bangumi_list: Vec<Bangumi>,
    file_path: String,
) -> Result<(), String>

// 导入追番列表
#[tauri::command]
pub async fn import_bangumi(
    service: State<'_, BangumiService>,
    file_path: String,
) -> Result<Vec<Bangumi>, String>
```

### 稍后再看命令

```rust
// 备份稍后再看
#[tauri::command]
pub async fn backup_toview(
    service: State<'_, ToViewService>,
) -> Result<Vec<ToView>, String>

// 还原稍后再看
#[tauri::command]
pub async fn restore_toview(
    service: State<'_, ToViewService>,
    videos: Vec<ToView>,
) -> Result<RestoreResult, String>

// 清空稍后再看
#[tauri::command]
pub async fn clear_toview(
    service: State<'_, ToViewService>,
) -> Result<ClearResult, String>

// 导出稍后再看
#[tauri::command]
pub async fn export_toview(
    service: State<'_, ToViewService>,
    videos: Vec<ToView>,
    file_path: String,
) -> Result<(), String>

// 导入稍后再看
#[tauri::command]
pub async fn import_toview(
    service: State<'_, ToViewService>,
    file_path: String,
) -> Result<Vec<ToView>, String>
```

## 数据模型

### 操作结果类型

**ClearResult** - 清除操作结果:
```rust
pub struct ClearResult {
    pub cleared_count: usize,
    pub message: String,
}
```

**RestoreResult** - 还原操作结果:
```rust
pub struct RestoreResult {
    pub success_count: usize,
    pub failed_count: usize,
    pub total_count: usize,
    pub failed_items: Option<Vec<String>>,
    pub message: String,
}
```

## 使用示例

### Rust服务端

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::api::BiliClient;
use crate::services::{HistoryService, BangumiService, ToViewService};

// 创建服务实例
let client = Arc::new(RwLock::new(BiliClient::new()));
let history_service = HistoryService::new(client.clone());
let bangumi_service = BangumiService::new(client.clone());
let toview_service = ToViewService::new(client.clone());

// 备份历史记录
let history = history_service.backup_history().await?;
println!("备份了 {} 条历史记录", history.len());

// 备份番剧
let bangumi = bangumi_service.backup_bangumi(1).await?;
println!("备份了 {} 个番剧", bangumi.len());

// 备份稍后再看
let toview = toview_service.backup_toview().await?;
println!("备份了 {} 个稍后再看", toview.len());

// 导出到文件
history_service.export_to_file(&history, "history.json").await?;
bangumi_service.export_to_file(&bangumi, "bangumi.json").await?;
toview_service.export_to_file(&toview, "toview.json").await?;

// 从文件导入
let imported_history = history_service.import_from_file("history.json").await?;
let imported_bangumi = bangumi_service.import_from_file("bangumi.json").await?;
let imported_toview = toview_service.import_from_file("toview.json").await?;

// 还原数据
let restore_result = bangumi_service.restore_bangumi(imported_bangumi).await?;
println!("{}", restore_result.message);

let restore_result = toview_service.restore_toview(imported_toview).await?;
println!("{}", restore_result.message);

// 清空数据（慎用！）
let clear_result = history_service.clear_history().await?;
println!("{}", clear_result.message);
```

### TypeScript/JavaScript前端

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 备份历史记录
const history = await invoke<History[]>('backup_history');
console.log(`备份了 ${history.length} 条历史记录`);

// 备份番剧（type=1表示番剧）
const bangumi = await invoke<Bangumi[]>('backup_bangumi', { type_: 1 });
console.log(`备份了 ${bangumi.length} 个番剧`);

// 备份稍后再看
const toview = await invoke<ToView[]>('backup_toview');
console.log(`备份了 ${toview.length} 个稍后再看`);

// 导出到文件
await invoke('export_history', {
  history,
  filePath: '/path/to/history.json'
});

await invoke('export_bangumi', {
  bangumiList: bangumi,
  filePath: '/path/to/bangumi.json'
});

await invoke('export_toview', {
  videos: toview,
  filePath: '/path/to/toview.json'
});

// 从文件导入
const importedBangumi = await invoke<Bangumi[]>('import_bangumi', {
  filePath: '/path/to/bangumi.json'
});

const importedToview = await invoke<ToView[]>('import_toview', {
  filePath: '/path/to/toview.json'
});

// 还原数据
const restoreResult = await invoke<RestoreResult>('restore_bangumi', {
  bangumiList: importedBangumi
});
console.log(restoreResult.message);

const restoreResult2 = await invoke<RestoreResult>('restore_toview', {
  videos: importedToview
});
console.log(restoreResult2.message);

// 清空数据（慎用！）
const clearResult = await invoke<ClearResult>('clear_history');
console.log(clearResult.message);
```

## 注册命令到Tauri

在 `main.rs` 或 `lib.rs` 中注册命令：

```rust
use crate::commands::*;

fn main() {
    tauri::Builder::default()
        .manage(HistoryService::new(client.clone()))
        .manage(BangumiService::new(client.clone()))
        .manage(ToViewService::new(client.clone()))
        .invoke_handler(tauri::generate_handler![
            // 历史记录
            backup_history,
            clear_history,
            export_history,
            import_history,
            // 追番追剧
            backup_bangumi,
            restore_bangumi,
            clear_bangumi,
            export_bangumi,
            import_bangumi,
            // 稍后再看
            backup_toview,
            restore_toview,
            clear_toview,
            export_toview,
            import_toview,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 技术要点

### 1. 分页处理

- **历史记录**: 使用游标分页（max + view_at）
- **追番追剧**: 使用普通分页（pn + ps）
- **稍后再看**: 一次性获取全部

### 2. 防风控

所有批量操作都包含随机延迟（1-3秒），避免触发B站风控：

```rust
let client = self.client.read().await;
client.delay_random().await;
```

### 3. 错误处理

所有服务方法都返回 `Result<T, BiliError>`，命令层统一转换为 `Result<T, String>` 供前端调用。

### 4. 并发控制

使用 `Arc<RwLock<BiliClient>>` 管理HTTP客户端，支持多服务共享同一客户端实例。

### 5. 限制说明

- **历史记录**: 仅支持备份和清空，不支持还原（B站API限制）
- **追番追剧**: 支持完整的备份、还原、清空
- **稍后再看**: 支持完整的备份、还原、清空

## 测试

每个服务都包含基本的单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_history_service() {
        let client = Arc::new(RwLock::new(BiliClient::new()));
        let service = HistoryService::new(client);
        // 验证服务创建不会panic
    }
}
```

运行测试：
```bash
cd src-tauri
cargo test --lib services::history
cargo test --lib services::bangumi
cargo test --lib services::toview
```

## 后续优化建议

1. **批量操作优化**: 考虑添加批量追番/取消追番接口，减少API调用次数
2. **进度回调**: 为长时间操作添加进度回调，改善用户体验
3. **增量备份**: 支持增量备份，只获取新增/变更的数据
4. **数据压缩**: 对大量数据进行压缩存储，减少文件体积
5. **并发控制**: 添加更精细的并发控制，提高备份速度
6. **错误重试**: 为失败的操作添加自动重试机制

## 相关文档

- API端点定义: `/src-tauri/src/api/endpoints.rs`
- 数据模型: `/src-tauri/src/api/models.rs`
- HTTP客户端: `/src-tauri/src/api/client.rs`
- 错误处理: `/src-tauri/src/api/error.rs`

## 开发者

- Agent 6 - 历史记录相关模块实现
- 实现时间: 2025-10-06
