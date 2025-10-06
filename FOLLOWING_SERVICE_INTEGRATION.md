# 关注管理模块集成文档

## 概述

关注管理模块提供了完整的关注、粉丝、黑名单管理功能,包括备份、还原、清空等操作。

## 模块组成

### 1. 服务层 (Services)

#### 1.1 FollowingService (关注管理)

**文件位置**: `src-tauri/src/services/following.rs`

**核心功能**:

- `backup_following()` - 备份关注列表
- `restore_following()` - 还原关注列表
- `clear_following()` - 清空关注列表
- `get_relation_tags()` - 获取关注分组列表
- `create_tag()` - 创建关注分组

**关键特性**:

- 自动建立分组映射 (旧ID -> 新ID)
- 批量操作 (每次20个用户, 可配置)
- 延迟机制 (1-3秒随机延迟, 可配置)
- 错误处理 (部分失败可选择继续或中断)
- 支持特别关注标记

**使用示例**:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use bilibili_backup_tauri::api::BiliClient;
use bilibili_backup_tauri::services::following::{FollowingService, RestoreOptions};

// 创建服务
let client = Arc::new(RwLock::new(BiliClient::new()));
let service = FollowingService::new(client);

// 备份
let relations = service.backup_following().await?;

// 还原
let options = RestoreOptions {
    create_missing_tags: true,
    continue_on_error: false,
    batch_size: 20,
    delay_ms: Some((1000, 3000)),
};
let result = service.restore_following(relations, options).await?;

// 清空
let result = service.clear_following().await?;
```

#### 1.2 FollowerService (粉丝管理)

**文件位置**: `src-tauri/src/services/follower.rs`

**核心功能**:

- `backup_followers()` - 备份粉丝列表

**注意事项**:

- 仅支持备份,不支持还原 (B站API限制)
- 粉丝关系是由其他用户主动关注产生的

**使用示例**:

```rust
use bilibili_backup_tauri::services::follower::FollowerService;

let service = FollowerService::new(client);
let followers = service.backup_followers().await?;
```

#### 1.3 BlacklistService (黑名单管理)

**文件位置**: `src-tauri/src/services/blacklist.rs`

**核心功能**:

- `backup_blacklist()` - 备份黑名单
- `restore_blacklist()` - 还原黑名单
- `clear_blacklist()` - 清空黑名单

**使用示例**:

```rust
use bilibili_backup_tauri::services::blacklist::{BlacklistService, BlacklistRestoreOptions};

let service = BlacklistService::new(client);

// 备份
let users = service.backup_blacklist().await?;

// 还原
let options = BlacklistRestoreOptions::default();
let result = service.restore_blacklist(users, options).await?;

// 清空
let result = service.clear_blacklist().await?;
```

### 2. 命令层 (Commands)

**文件位置**: `src-tauri/src/commands/following.rs`

提供给前端的Tauri命令接口。

#### 2.1 关注管理命令

- `backup_following` - 备份关注列表
- `restore_following` - 还原关注列表
- `clear_following` - 清空关注列表
- `get_relation_tags` - 获取分组列表
- `create_relation_tag` - 创建分组

#### 2.2 粉丝管理命令

- `backup_followers` - 备份粉丝列表

#### 2.3 黑名单管理命令

- `backup_blacklist` - 备份黑名单
- `restore_blacklist` - 还原黑名单
- `clear_blacklist` - 清空黑名单

### 3. 前端调用示例

#### 3.1 备份关注列表

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 备份关注
const relations = await invoke<Relation[]>('backup_following');
console.log(`备份了 ${relations.length} 个关注`);

// 保存到文件
await invoke('save_backup_file', {
  filename: 'following.json',
  data: JSON.stringify(relations)
});
```

#### 3.2 还原关注列表

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 读取备份文件
const backupData = await invoke<string>('load_backup_file', {
  filename: 'following.json'
});
const relations = JSON.parse(backupData) as Relation[];

// 还原配置
const options = {
  create_missing_tags: true,
  continue_on_error: false,
  batch_size: 20,
  delay_ms: null
};

// 执行还原
const result = await invoke<FollowingRestoreResult>('restore_following', {
  relations: relations,
  options: options
});

console.log(`成功: ${result.success_count}, 失败: ${result.failed_count}`);
if (result.failures.length > 0) {
  console.error('失败的用户:', result.failures);
}
```

#### 3.3 清空关注列表

```typescript
import { invoke } from '@tauri-apps/api/tauri';
import { confirm } from '@tauri-apps/api/dialog';

// 确认操作
const confirmed = await confirm('确定要清空所有关注吗?此操作不可撤销!');
if (!confirmed) return;

// 执行清空
const result = await invoke<FollowingClearResult>('clear_following');
console.log(`成功取消 ${result.success_count} 个关注`);
```

#### 3.4 备份粉丝列表

```typescript
import { invoke } from '@tauri-apps/api/tauri';

const followers = await invoke<Relation[]>('backup_followers');
console.log(`备份了 ${followers.length} 个粉丝`);
```

#### 3.5 黑名单管理

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 备份黑名单
const blacklist = await invoke<User[]>('backup_blacklist');

// 还原黑名单
const options = {
  continue_on_error: false,
  batch_size: 20,
  delay_ms: null
};
const result = await invoke<BlacklistRestoreResult>('restore_blacklist', {
  users: blacklist,
  options: options
});

// 清空黑名单
const clearResult = await invoke<BlacklistClearResult>('clear_blacklist');
```

## 数据结构

### Relation (关注/粉丝数据)

```typescript
interface Relation {
  mid: number;           // 用户ID
  uname: string;         // 用户名
  face: string;          // 头像URL
  sign?: string;         // 签名
  mtime: number;         // 修改时间 (时间戳)
  attribute?: number;    // 关注属性
  special?: number;      // 特别关注标记
  tag?: number[];        // 所属分组ID列表
  vip?: Vip;            // VIP信息
}
```

### RelationTag (关注分组)

```typescript
interface RelationTag {
  tag_id: number;        // 分组ID
  name: string;          // 分组名称
  count?: number;        // 分组内关注数量
  tip?: string;          // 提示信息
}
```

### RestoreOptions (还原选项)

```typescript
interface RestoreOptions {
  create_missing_tags: boolean;    // 是否创建缺失的分组
  continue_on_error: boolean;      // 遇到错误是否继续
  batch_size: number;              // 批量操作大小
  delay_ms?: [number, number];     // 操作延迟范围 [min, max]
}
```

### FollowingRestoreResult (还原结果)

```typescript
interface FollowingRestoreResult {
  success_count: number;              // 成功数量
  failed_count: number;               // 失败数量
  failures: [number, string][];       // 失败的用户ID和原因
  tag_mapping: Record<number, number>; // 分组映射 (旧ID -> 新ID)
}
```

### FollowingClearResult (清空结果)

```typescript
interface FollowingClearResult {
  success_count: number;         // 成功数量
  failed_count: number;          // 失败数量
  failures: [number, string][];  // 失败的用户ID和原因
}
```

### User (用户/黑名单数据)

```typescript
interface User {
  mid: number;          // 用户ID
  uname: string;        // 用户名
  face: string;         // 头像URL
  sign?: string;        // 签名
  sex?: string;         // 性别
  level?: number;       // 等级
}
```

### BlacklistRestoreOptions (黑名单还原选项)

```typescript
interface BlacklistRestoreOptions {
  continue_on_error: boolean;      // 遇到错误是否继续
  batch_size: number;              // 批量操作大小
  delay_ms?: [number, number];     // 操作延迟范围
}
```

### BlacklistRestoreResult (黑名单还原结果)

```typescript
interface BlacklistRestoreResult {
  success_count: number;         // 成功数量
  failed_count: number;          // 失败数量
  failures: [number, string][];  // 失败的用户ID和原因
}
```

## API端点

使用的B站API端点 (定义在 `src-tauri/src/api/endpoints.rs`):

- `API_FOLLOWING_LIST` - 获取关注列表
- `API_FOLLOWER_LIST` - 获取粉丝列表
- `API_RELATION_TAGS` - 获取关注分组
- `API_RELATION_MODIFY` - 修改关注关系 (关注/取消关注)
- `API_TAG_CREATE` - 创建分组
- `API_TAG_ADD_USERS` - 将用户添加到分组
- `API_BLACK_LIST` - 获取黑名单
- `API_BLACK_ADD` - 添加到黑名单
- `API_BLACK_REMOVE` - 从黑名单移除

## 错误处理

### 常见错误类型

1. **未登录** - `BiliError::auth("未登录")`
   - 解决方案: 确保已调用登录接口并设置Cookie

2. **网络请求失败** - `BiliError::network(...)`
   - 解决方案: 检查网络连接,重试请求

3. **API返回错误** - `BiliError::api(...)`
   - 解决方案: 检查API返回的错误码和消息

4. **风控限制** - API返回 429 Too Many Requests
   - 解决方案: 增加延迟时间,减小批量操作大小

### 错误处理策略

1. **部分失败处理**:
   - 设置 `continue_on_error: true` 继续执行
   - 在 `failures` 字段中记录失败项

2. **重试机制**:
   - 客户端内置自动重试 (最多3次)
   - 失败后延迟1秒再重试

3. **延迟机制**:
   - 默认随机延迟 1000-3000ms
   - 可通过 `delay_ms` 参数自定义

## 性能优化

### 批量操作

- 默认批量大小: 20个用户/次
- 可通过 `batch_size` 参数调整
- 建议范围: 10-50

### 并发控制

- 客户端内置请求限流 (每秒最多2个请求)
- 可通过 `with_rate_limit()` 调整

### 延迟策略

- 防风控: 每次操作后随机延迟
- 默认范围: 1000-3000ms
- 大批量操作时建议增加延迟

## 测试

### 单元测试

```bash
cd src-tauri
cargo test --lib services::following
cargo test --lib services::follower
cargo test --lib services::blacklist
```

### 集成测试

需要提供真实的Cookie进行测试:

```rust
#[tokio::test]
#[ignore] // 需要真实Cookie,默认忽略
async fn test_backup_following_integration() {
    let mut client = BiliClient::new();
    client.set_cookie("YOUR_COOKIE_HERE".to_string());

    let service = FollowingService::new(Arc::new(RwLock::new(client)));
    let relations = service.backup_following().await.unwrap();

    assert!(!relations.is_empty());
}
```

## 注意事项

### 1. Cookie管理

- 必须先登录并获取Cookie
- Cookie包含 `SESSDATA` 和 `bili_jct` 字段
- Cookie有效期约为30天

### 2. 风控限制

- 频繁操作可能触发风控
- 建议使用默认延迟和批量大小
- 大批量操作建议分批执行

### 3. 数据备份

- 定期备份关注列表
- 备份文件建议使用JSON格式
- 包含完整的分组信息

### 4. 分组映射

- 不同账号的分组ID不同
- 还原时自动建立映射关系
- 支持根据分组名称匹配

### 5. 特别关注

- `special` 字段表示特别关注
- 还原时会保留特别关注标记

## 未来改进

### 待实现功能

1. 进度回调机制
   - 使用Tauri事件系统
   - 实时更新进度条

2. 增量备份
   - 仅备份变化的数据
   - 减少网络请求

3. 差异对比
   - 对比备份与当前状态
   - 显示新增/删除的关注

4. 自动分类
   - 根据UP主类型自动分组
   - 智能推荐分组

### 性能优化

1. 并发请求
   - 在风控允许范围内并发
   - 提高备份速度

2. 缓存机制
   - 缓存用户信息
   - 减少重复请求

3. 增量还原
   - 跳过已关注的用户
   - 仅还原差异部分

## 开发者指南

### 添加新功能

1. 在 `FollowingService` 中实现业务逻辑
2. 在 `commands/following.rs` 中添加Tauri命令
3. 在 `mod.rs` 中导出新命令
4. 更新文档和类型定义

### 调试技巧

1. 启用日志:
   ```rust
   use tracing_subscriber;
   tracing_subscriber::fmt::init();
   ```

2. 查看请求详情:
   ```rust
   client.get_with_retry(url).await?;
   // 查看日志中的请求和响应
   ```

3. 测试单个功能:
   ```bash
   cargo test test_backup_following -- --nocapture
   ```

## 参考资料

- [B站API文档](https://github.com/SocialSisterYi/bilibili-API-collect)
- [Tauri文档](https://tauri.app/v1/guides/)
- [Rust异步编程](https://rust-lang.github.io/async-book/)

## 更新日志

### v1.0.0 (2025-10-06)

- 初始版本
- 实现关注管理基本功能
- 实现粉丝备份功能
- 实现黑名单管理功能
- 支持分组映射
- 支持批量操作和延迟机制

## 许可证

本项目遵循项目整体许可证。
