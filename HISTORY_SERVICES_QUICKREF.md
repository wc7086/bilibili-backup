# 历史记录服务快速参考

## 服务对照表

| 服务 | 文件 | 功能 | 支持还原 |
|------|------|------|----------|
| HistoryService | `services/history.rs` | 观看历史 | ❌ (API限制) |
| BangumiService | `services/bangumi.rs` | 追番追剧 | ✅ |
| ToViewService | `services/toview.rs` | 稍后再看 | ✅ |

## 命令速查

### 历史记录
```typescript
await invoke('backup_history')              // 备份
await invoke('clear_history')               // 清空
await invoke('export_history', { ... })     // 导出
await invoke('import_history', { ... })     // 导入
```

### 追番追剧（type: 1=番剧, 2=电影, 3=纪录片, 4=国创, 5=电视剧, 7=综艺）
```typescript
await invoke('backup_bangumi', { type_: 1 })   // 备份
await invoke('restore_bangumi', { ... })       // 还原
await invoke('clear_bangumi', { type_: 1 })    // 清空
await invoke('export_bangumi', { ... })        // 导出
await invoke('import_bangumi', { ... })        // 导入
```

### 稍后再看
```typescript
await invoke('backup_toview')                  // 备份
await invoke('restore_toview', { ... })        // 还原
await invoke('clear_toview')                   // 清空
await invoke('export_toview', { ... })         // 导出
await invoke('import_toview', { ... })         // 导入
```

## 分页方式

- **历史记录**: 游标分页 (max + view_at)
- **追番追剧**: 普通分页 (pn + ps)
- **稍后再看**: 一次性获取

## API端点

| 功能 | 端点 | 方法 |
|------|------|------|
| 获取历史 | `/x/web-interface/history/cursor` | GET |
| 清空历史 | `/x/v2/history/clear` | POST |
| 获取追番 | `/x/space/bangumi/follow/list` | GET |
| 追番 | `/pgc/web/follow/add` | POST |
| 取消追番 | `/pgc/web/follow/del` | POST |
| 获取稍后再看 | `/x/v2/history/toview` | GET |
| 添加稍后再看 | `/x/v2/history/toview/add` | POST |
| 删除稍后再看 | `/x/v2/history/toview/del` | POST |
| 清空稍后再看 | `/x/v2/history/toview/clear` | POST |

## 防风控

所有批量操作都包含1-3秒随机延迟：
```rust
client.delay_random().await;
```

## 文件位置

```
src-tauri/src/
├── services/
│   ├── history.rs      # 历史记录服务
│   ├── bangumi.rs      # 追番追剧服务
│   └── toview.rs       # 稍后再看服务
├── commands/
│   └── history.rs      # Tauri命令（15个）
└── api/
    ├── models.rs       # 数据模型（新增4个）
    └── endpoints.rs    # API端点定义
```

## 数据模型

```rust
// 清除结果
struct ClearResult {
    cleared_count: usize,
    message: String,
}

// 还原结果
struct RestoreResult {
    success_count: usize,
    failed_count: usize,
    total_count: usize,
    failed_items: Option<Vec<String>>,
    message: String,
}
```

## 典型工作流

### 备份流程
```typescript
// 1. 备份数据
const history = await invoke('backup_history');
const bangumi = await invoke('backup_bangumi', { type_: 1 });
const toview = await invoke('backup_toview');

// 2. 导出到文件
await invoke('export_history', { history, filePath: 'history.json' });
await invoke('export_bangumi', { bangumiList: bangumi, filePath: 'bangumi.json' });
await invoke('export_toview', { videos: toview, filePath: 'toview.json' });
```

### 还原流程
```typescript
// 1. 从文件导入
const bangumi = await invoke('import_bangumi', { filePath: 'bangumi.json' });
const toview = await invoke('import_toview', { filePath: 'toview.json' });

// 2. 还原数据
const result1 = await invoke('restore_bangumi', { bangumiList: bangumi });
const result2 = await invoke('restore_toview', { videos: toview });

// 3. 检查结果
console.log(result1.message);
console.log(`成功: ${result1.success_count}, 失败: ${result1.failed_count}`);
```

## 注意事项

⚠️ **历史记录无法还原** - B站API限制，只能备份和清空
⚠️ **清空操作不可逆** - 执行前请确认
⚠️ **批量操作需要时间** - 大量数据会自动添加延迟
⚠️ **需要登录状态** - 所有操作都需要有效的Cookie

## 更多信息

详见：
- `HISTORY_SERVICE_INTEGRATION.md` - 完整集成文档
- `AGENT6_COMPLETION_REPORT.md` - 完成报告
