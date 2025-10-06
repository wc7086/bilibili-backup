# 📚 项目文档索引

> 哔哩哔哩账号备份工具 - 完整文档导航

**最后更新**: 2025-10-06

---

## 🗂️ 文档分类

### 📋 核心集成文档（优先阅读）

| 文档 | 大小 | 说明 | 优先级 |
|------|------|------|--------|
| **[INTEGRATION_SUMMARY.md](./INTEGRATION_SUMMARY.md)** | 9.8KB | 集成工程师总结报告 | ⭐⭐⭐ |
| **[INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md)** | 20.5KB | 完整集成报告（架构、模块、命令映射） | ⭐⭐⭐ |
| **[QUICKSTART.md](./QUICKSTART.md)** | 9.6KB | 快速启动指南（环境、安装、运行） | ⭐⭐⭐ |
| **[FEATURE_MATRIX.md](./FEATURE_MATRIX.md)** | 15.3KB | 功能覆盖矩阵（详细功能清单） | ⭐⭐ |
| **[INTEGRATION_CHECKLIST.md](./INTEGRATION_CHECKLIST.md)** | 9.7KB | 集成验证清单 | ⭐⭐ |

### 🔧 模块开发文档

| 文档 | 模块 | Agent | 说明 |
|------|------|-------|------|
| [AGENT2_COMPLETION_REPORT.md](./AGENT2_COMPLETION_REPORT.md) | API核心 | Agent 2 | API层、WBI签名、数据模型 |
| [AUTH_SERVICE_INTEGRATION.md](./AUTH_SERVICE_INTEGRATION.md) | 用户认证 | Agent 3 | 二维码登录、Cookie登录 |
| [FOLLOWING_SERVICE_INTEGRATION.md](./FOLLOWING_SERVICE_INTEGRATION.md) | 关注管理 | Agent 4 | 关注、粉丝、黑名单 |
| [HISTORY_SERVICE_INTEGRATION.md](./HISTORY_SERVICE_INTEGRATION.md) | 观看历史 | Agent 6 | 历史、追番、稍后再看 |
| [AGENT6_COMPLETION_REPORT.md](./AGENT6_COMPLETION_REPORT.md) | 观看历史 | Agent 6 | 完成报告 |
| [HISTORY_SERVICES_QUICKREF.md](./HISTORY_SERVICES_QUICKREF.md) | 观看历史 | Agent 6 | 快速参考 |

### 📖 项目基础文档

| 文档 | 说明 |
|------|------|
| [README.md](./README.md) | 项目介绍和基础说明 |

---

## 🚀 快速导航

### 我想了解...

#### 项目整体情况
→ 阅读 **[INTEGRATION_SUMMARY.md](./INTEGRATION_SUMMARY.md)**
- 集成成果概览
- 完成度统计
- 下一步建议

#### 如何快速启动项目
→ 阅读 **[QUICKSTART.md](./QUICKSTART.md)**
- 环境要求
- 安装步骤
- 开发模式运行
- 常见问题

#### 项目架构和设计
→ 阅读 **[INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md)**
- 三层架构设计
- 模块清单
- 数据流向图
- 依赖关系图

#### 有哪些功能
→ 阅读 **[FEATURE_MATRIX.md](./FEATURE_MATRIX.md)**
- 完整功能列表
- API端点对照
- 数据模型清单
- 测试覆盖计划

#### 如何验证集成
→ 阅读 **[INTEGRATION_CHECKLIST.md](./INTEGRATION_CHECKLIST.md)**
- 模块交付验证
- 命令完整性检查
- 代码质量验证

---

## 📂 按功能模块查找

### API核心模块
- **文档**: [AGENT2_COMPLETION_REPORT.md](./AGENT2_COMPLETION_REPORT.md)
- **包含**:
  - WBI签名算法
  - HTTP客户端封装
  - 27个数据结构
  - 45个API端点
  - 分页数据获取

### 用户认证模块
- **文档**: [AUTH_SERVICE_INTEGRATION.md](./AUTH_SERVICE_INTEGRATION.md)
- **包含**:
  - 二维码登录（生成、轮询）
  - Cookie登录
  - 用户信息管理
  - 6个认证命令

### 关注管理模块
- **文档**: [FOLLOWING_SERVICE_INTEGRATION.md](./FOLLOWING_SERVICE_INTEGRATION.md)
- **包含**:
  - 关注列表管理（备份、还原、清空）
  - 粉丝列表管理（备份）
  - 黑名单管理（备份、还原、清空）
  - 分组管理
  - 9个管理命令

### 收藏管理模块
- **文档**: 包含在集成报告中
- **包含**:
  - 收藏夹管理（备份、还原、清空）
  - 自动创建缺失收藏夹
  - 3个收藏命令

### 观看历史模块
- **文档**: [HISTORY_SERVICE_INTEGRATION.md](./HISTORY_SERVICE_INTEGRATION.md)
- **包含**:
  - 历史记录（备份、清空、导出、导入）
  - 追番追剧（备份、还原、清空、导出、导入）
  - 稍后再看（备份、还原、清空、导出、导入）
  - 14个历史命令

---

## 🔍 按读者角色查找

### 新开发者入门
**推荐阅读顺序**:
1. [README.md](./README.md) - 了解项目背景
2. [QUICKSTART.md](./QUICKSTART.md) - 搭建开发环境
3. [INTEGRATION_SUMMARY.md](./INTEGRATION_SUMMARY.md) - 快速了解整体
4. [INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md) - 深入架构设计

### 前端开发者
**推荐阅读**:
1. [INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md) → 命令映射表
2. [FEATURE_MATRIX.md](./FEATURE_MATRIX.md) → 前端类型定义
3. [AUTH_SERVICE_INTEGRATION.md](./AUTH_SERVICE_INTEGRATION.md) → 认证流程
4. 各模块文档 → 前端调用示例

### 后端开发者
**推荐阅读**:
1. [INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md) → 架构设计
2. [AGENT2_COMPLETION_REPORT.md](./AGENT2_COMPLETION_REPORT.md) → API层
3. 各服务集成文档 → 业务逻辑
4. [FEATURE_MATRIX.md](./FEATURE_MATRIX.md) → 数据模型

### 测试工程师
**推荐阅读**:
1. [FEATURE_MATRIX.md](./FEATURE_MATRIX.md) → 功能清单和测试计划
2. [INTEGRATION_CHECKLIST.md](./INTEGRATION_CHECKLIST.md) → 验证清单
3. [INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md) → 数据流向
4. 各模块文档 → 功能详细说明

### 项目负责人
**推荐阅读**:
1. [INTEGRATION_SUMMARY.md](./INTEGRATION_SUMMARY.md) → 集成总结
2. [INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md) → 完整报告
3. [INTEGRATION_CHECKLIST.md](./INTEGRATION_CHECKLIST.md) → 验证结果
4. [FEATURE_MATRIX.md](./FEATURE_MATRIX.md) → 完成度统计

---

## 📊 文档统计

### 文档数量
- **核心集成文档**: 5份
- **模块开发文档**: 6份
- **项目基础文档**: 1份
- **总计**: 12份

### 文档大小
- **总大小**: ~130KB
- **最大文档**: INTEGRATION_REPORT.md (20.5KB)
- **最小文档**: HISTORY_SERVICES_QUICKREF.md (4.3KB)

### 文档覆盖
- ✅ 架构设计: 100%
- ✅ 功能说明: 100%
- ✅ API文档: 100%
- ✅ 使用指南: 100%
- ⏳ 用户手册: 0% (待编写)

---

## 🔗 相关链接

### 代码位置
- **Rust后端**: `/home/test/bl/bilibili-backup-tauri/src-tauri/src/`
- **Vue前端**: `/home/test/bl/bilibili-backup-tauri/src/`
- **配置文件**: `/home/test/bl/bilibili-backup-tauri/src-tauri/Cargo.toml`

### 外部资源
- [Tauri 官方文档](https://tauri.app/zh-cn/)
- [Vue 3 文档](https://cn.vuejs.org/)
- [Rust 程序设计语言](https://rustwiki.org/)

---

## 📝 文档维护

### 更新记录
- **2025-10-06**: 创建文档索引，集成所有文档

### 待完成文档
- [ ] 用户使用手册
- [ ] API参考文档（Rust doc）
- [ ] 常见问题整理
- [ ] 贡献指南
- [ ] 变更日志

### 文档规范
- 所有文档使用简体中文
- Markdown格式，UTF-8编码
- 包含目录和导航链接
- 定期更新最后修改时间

---

## 💡 使用建议

### 第一次接触项目？
→ 从 **[INTEGRATION_SUMMARY.md](./INTEGRATION_SUMMARY.md)** 开始

### 想要快速运行？
→ 参考 **[QUICKSTART.md](./QUICKSTART.md)**

### 需要开发新功能？
→ 查阅 **[INTEGRATION_REPORT.md](./INTEGRATION_REPORT.md)** 和相关模块文档

### 遇到问题？
→ 先看 **[QUICKSTART.md](./QUICKSTART.md)** 的常见问题
→ 再查 **[INTEGRATION_CHECKLIST.md](./INTEGRATION_CHECKLIST.md)** 的已知问题

---

**文档维护**: Claude Code - 集成工程师
**最后更新**: 2025-10-06
