# 快速启动指南

> 哔哩哔哩账号备份工具 - Tauri版
>
> 本指南帮助您快速搭建开发环境并运行项目

---

## 📋 目录

- [环境要求](#环境要求)
- [安装步骤](#安装步骤)
- [开发模式](#开发模式)
- [生产构建](#生产构建)
- [项目结构](#项目结构)
- [常见问题](#常见问题)

---

## 🔧 环境要求

### 必需工具

| 工具 | 版本要求 | 说明 |
|------|---------|------|
| **Node.js** | ≥ 18.0.0 | JavaScript运行时 |
| **pnpm** | ≥ 8.0.0 | 包管理器 |
| **Rust** | ≥ 1.70.0 | Rust工具链 |
| **Tauri CLI** | 1.5.x | Tauri命令行工具 |

### 系统依赖

#### Windows

- 安装 [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- 安装 [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (通常已预装)

#### macOS

```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Linux (Fedora)

```bash
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

---

## 📦 安装步骤

### 1. 克隆项目

```bash
git clone <repository-url>
cd bilibili-backup-tauri
```

### 2. 安装 Rust

```bash
# 使用 rustup 安装（推荐）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 3. 安装 Node.js 依赖

```bash
# 安装 pnpm (如果未安装)
npm install -g pnpm

# 安装项目依赖
pnpm install
```

### 4. 安装 Tauri CLI (可选)

```bash
# 方式1: 通过 cargo 安装（全局）
cargo install tauri-cli

# 方式2: 通过 pnpm 使用（本地，推荐）
# 已在 package.json 中配置，无需额外安装
```

---

## 🚀 开发模式

### 启动开发服务器

```bash
# 方式1: 使用 pnpm (推荐)
pnpm tauri dev

# 方式2: 使用 cargo
cd src-tauri
cargo tauri dev
```

这将：
1. 启动前端开发服务器（Vite）
2. 编译 Rust 后端代码
3. 打开应用窗口（支持热重载）

### 开发工具

- **前端热重载**: 修改 Vue 文件自动刷新
- **后端热重载**: 修改 Rust 文件需重新编译（约5-10秒）
- **DevTools**: 右键 → 检查元素（或 `Ctrl+Shift+I` / `Cmd+Opt+I`）

### 开发端口

- **前端**: `http://localhost:1420` (Vite)
- **后端**: Tauri IPC 通信（无端口）

---

## 📦 生产构建

### 构建应用

```bash
# 构建所有平台的安装包
pnpm tauri build

# 仅构建当前平台
pnpm tauri build --target current
```

### 构建产物

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
├── deb/               # Linux .deb 包
├── appimage/          # Linux AppImage
├── msi/               # Windows .msi 安装器
├── nsis/              # Windows .exe 安装器
└── dmg/               # macOS .dmg 磁盘镜像
```

### 构建选项

```bash
# Debug 构建（更快，包含调试符号）
pnpm tauri build --debug

# 指定目标平台
pnpm tauri build --target x86_64-pc-windows-msvc
pnpm tauri build --target x86_64-apple-darwin
pnpm tauri build --target x86_64-unknown-linux-gnu

# 构建前清理
pnpm tauri build --clean
```

---

## 📁 项目结构

```
bilibili-backup-tauri/
├── src/                      # 前端源码（Vue 3 + TypeScript）
│   ├── assets/              # 静态资源
│   ├── components/          # Vue组件
│   ├── types/               # TypeScript类型定义
│   ├── App.vue              # 根组件
│   └── main.ts              # 前端入口
│
├── src-tauri/               # 后端源码（Rust）
│   ├── src/
│   │   ├── api/            # API层
│   │   │   ├── client.rs   # HTTP客户端
│   │   │   ├── models.rs   # 数据模型
│   │   │   ├── sign.rs     # WBI签名
│   │   │   └── endpoints.rs # API端点
│   │   │
│   │   ├── services/       # 业务逻辑层
│   │   │   ├── auth.rs     # 认证服务
│   │   │   ├── following.rs # 关注管理
│   │   │   ├── favorites.rs # 收藏管理
│   │   │   └── history.rs  # 历史记录
│   │   │
│   │   ├── commands/       # Tauri命令层
│   │   │   ├── auth.rs     # 认证命令
│   │   │   ├── following.rs # 关注命令
│   │   │   ├── favorites.rs # 收藏命令
│   │   │   └── history.rs  # 历史命令
│   │   │
│   │   ├── lib.rs          # 库根模块
│   │   └── main.rs         # 应用入口
│   │
│   ├── Cargo.toml          # Rust依赖配置
│   └── tauri.conf.json     # Tauri配置
│
├── public/                  # 公共资源
├── package.json            # Node.js配置
├── vite.config.ts          # Vite配置
└── tsconfig.json           # TypeScript配置
```

---

## 🔍 常见问题

### Q1: Rust 编译失败

**问题**: `cargo build` 报错

**解决方案**:

```bash
# 更新 Rust 工具链
rustup update

# 清理并重新编译
cd src-tauri
cargo clean
cargo build
```

### Q2: 前端依赖安装失败

**问题**: `pnpm install` 报错

**解决方案**:

```bash
# 清理缓存
pnpm store prune

# 删除 node_modules 和 lock 文件
rm -rf node_modules pnpm-lock.yaml

# 重新安装
pnpm install
```

### Q3: Tauri 开发模式启动失败

**问题**: `pnpm tauri dev` 无法启动

**检查清单**:

1. 确认 Rust 已安装: `rustc --version`
2. 确认系统依赖已安装（见[系统依赖](#系统依赖)）
3. 检查端口占用: `lsof -i :1420` (Unix) 或 `netstat -ano | findstr :1420` (Windows)
4. 查看详细错误日志: `pnpm tauri dev --verbose`

### Q4: WebView2 缺失 (Windows)

**问题**: Windows 启动报错 "WebView2 未安装"

**解决方案**:

下载并安装 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section)

### Q5: 编译速度慢

**优化方案**:

```bash
# 1. 使用 sccache 加速编译
cargo install sccache
export RUSTC_WRAPPER=sccache

# 2. 使用 mold 链接器 (Linux)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# 3. 增加并行编译数（根据CPU核心数）
export CARGO_BUILD_JOBS=8
```

### Q6: Cookie登录失败

**问题**: 使用Cookie登录提示"认证失败"

**解决方案**:

1. 确认Cookie格式正确，必须包含：
   - `DedeUserID=xxx`
   - `SESSDATA=xxx`
   - `bili_jct=xxx`

2. Cookie获取方式：
   - 浏览器登录B站
   - 打开开发者工具（F12）
   - Application → Cookies → https://www.bilibili.com
   - 复制以上三个字段

3. Cookie拼接格式：
   ```
   DedeUserID=123456; SESSDATA=abc123...; bili_jct=xyz789...
   ```

### Q7: API请求被风控

**问题**: 批量操作时提示"请求过快"

**解决方案**:

调整还原选项中的延迟参数：

```typescript
const options = {
  continue_on_error: true,
  batch_size: 10,        // 减小批次大小
  delay_ms: 1000         // 增加延迟到1秒
};

await invoke('restore_following', { relations, options });
```

### Q8: 追番列表类型说明

**问题**: 不知道 `type_` 参数的值

**类型对照表**:

| type_ | 说明 |
|-------|------|
| 1 | 番剧 |
| 2 | 电影 |
| 3 | 纪录片 |
| 4 | 国创 |
| 5 | 电视剧 |
| 7 | 综艺 |

示例：
```typescript
// 备份番剧
await invoke('backup_bangumi', { type_: 1 });

// 备份电影
await invoke('backup_bangumi', { type_: 2 });
```

---

## 🛠️ 开发工具推荐

### IDE

- **VS Code** (推荐)
  - 插件: `rust-analyzer`, `Vue Language Features (Volar)`, `Tauri`

- **IntelliJ IDEA / WebStorm**
  - 插件: `Rust`, `Vue.js`

### 调试工具

- **Rust调试**: 使用 `tracing` 日志
  ```bash
  RUST_LOG=debug pnpm tauri dev
  ```

- **前端调试**: Chrome DevTools
  - 右键 → 检查元素
  - Console 查看日志
  - Network 监控请求

### 测试工具

```bash
# Rust 单元测试
cd src-tauri
cargo test

# Rust 文档测试
cargo test --doc

# 前端测试（需配置）
pnpm test
```

---

## 📚 学习资源

### 官方文档

- [Tauri 文档](https://tauri.app/zh-cn/)
- [Vue 3 文档](https://cn.vuejs.org/)
- [Rust 程序设计语言](https://rustwiki.org/)

### 相关技术

- [Vite 构建工具](https://cn.vitejs.dev/)
- [TypeScript 手册](https://www.typescriptlang.org/zh/)
- [Tokio 异步运行时](https://tokio.rs/)

### 项目文档

- `INTEGRATION_REPORT.md` - 集成报告
- `src-tauri/src/**/*.rs` - 代码文档注释
- `src/types/**/*.ts` - TypeScript类型定义

---

## 🚀 下一步

1. **阅读集成报告**: 查看 `INTEGRATION_REPORT.md` 了解项目架构
2. **运行开发模式**: `pnpm tauri dev` 启动应用
3. **查看API文档**: 浏览 `src-tauri/src/commands/` 了解可用命令
4. **开发前端UI**: 在 `src/` 目录开发Vue组件
5. **测试功能**: 使用二维码登录测试认证流程

---

## 📞 获取帮助

- **GitHub Issues**: 提交问题和建议
- **集成报告**: 查看技术细节和已知问题
- **代码注释**: 阅读源码中的文档注释
- **社区讨论**: Tauri Discord / Vue 论坛

---

**祝您开发愉快！** 🎉
