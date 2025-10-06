# 哔哩哔哩账号备份工具 (Tauri 版本)

使用 Tauri + Rust + React 构建的B站账号数据备份工具。

## ✨ 特性

- 🚀 **高性能**：基于 Rust 和 Tauri，启动快速，内存占用低
- 📦 **小体积**：二进制文件体积 < 15MB
- 🎨 **现代化UI**：使用 React 18 + Ant Design
- 🔒 **安全可靠**：本地运行，数据完全离线处理
- 🌐 **跨平台**：支持 Windows、macOS 和 Linux

## 📋 功能列表

- [ ] 二维码登录
- [ ] 关注管理（关注/粉丝/黑名单）
- [ ] 收藏管理（收藏夹/专栏/合集）
- [ ] 观看历史（历史/追番/稍后再看）
- [ ] 私信管理
- [ ] 弹幕备份
- [ ] 批量操作工具

## 🔧 开发环境要求

### 通用依赖

- **Node.js**: >= 18.0.0
- **npm**: >= 9.0.0
- **Rust**: >= 1.70.0（推荐使用 rustup 安装）

### Linux 系统依赖

在 Linux 上开发需要安装以下系统库：

\`\`\`bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \\
    libwebkit2gtk-4.0-dev \\
    libgtk-3-dev \\
    libayatana-appindicator3-dev \\
    librsvg2-dev \\
    libsoup2.4-dev

# Fedora
sudo dnf install \\
    webkit2gtk3-devel \\
    gtk3-devel \\
    libappindicator-gtk3-devel \\
    librsvg2-devel \\
    libsoup-devel

# Arch Linux
sudo pacman -S \\
    webkit2gtk \\
    gtk3 \\
    libappindicator-gtk3 \\
    librsvg \\
    libsoup
\`\`\`

### macOS 系统依赖

\`\`\`bash
# macOS 不需要额外安装系统依赖
# 但需要确保已安装 Xcode Command Line Tools
xcode-select --install
\`\`\`

### Windows 系统依赖

- 安装 [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- 或安装 Visual Studio（包含 C++ 工作负载）

## 🚀 快速开始

### 1. 克隆项目

\`\`\`bash
git clone https://github.com/[username]/bilibili-backup-tauri.git
cd bilibili-backup-tauri
\`\`\`

### 2. 安装依赖

\`\`\`bash
# 安装 Node.js 依赖
npm install

# 安装 Rust（如果未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
\`\`\`

### 3. 运行开发环境

\`\`\`bash
# 启动开发服务器（自动编译Rust和启动前端）
npm run tauri:dev
\`\`\`

### 4. 生产构建

\`\`\`bash
# 构建生产版本
npm run tauri:build

# 构建产物位置：
# - Linux: src-tauri/target/release/bundle/
# - Windows: src-tauri/target/release/bundle/
# - macOS: src-tauri/target/release/bundle/
\`\`\`

## 📁 项目结构

\`\`\`
bilibili-backup-tauri/
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── api/            # API 层
│   │   │   ├── client.rs   # HTTP 客户端
│   │   │   ├── models.rs   # 数据模型
│   │   │   └── error.rs    # 错误类型
│   │   ├── services/       # 业务逻辑层
│   │   ├── commands/       # Tauri 命令
│   │   ├── utils/          # 工具函数
│   │   ├── lib.rs          # 库入口
│   │   └── main.rs         # 主入口
│   ├── Cargo.toml          # Rust 依赖配置
│   └── tauri.conf.json     # Tauri 配置
├── src/                    # React 前端
│   ├── pages/              # 页面组件
│   ├── components/         # 可复用组件
│   ├── stores/             # 状态管理
│   ├── types/              # TypeScript 类型
│   ├── utils/              # 工具函数
│   ├── App.tsx             # 根组件
│   └── main.tsx            # 入口文件
├── package.json            # Node.js 依赖配置
├── tsconfig.json           # TypeScript 配置
├── vite.config.ts          # Vite 配置
└── README.md               # 项目说明
\`\`\`

## 🛠️ 技术栈

### 后端（Rust）

- **Tauri**: 桌面应用框架
- **reqwest**: HTTP 客户端
- **tokio**: 异步运行时
- **serde**: 序列化/反序列化
- **anyhow/thiserror**: 错误处理
- **tracing**: 日志系统

### 前端（React）

- **React 18**: UI 框架
- **TypeScript**: 类型安全
- **Vite**: 构建工具
- **Ant Design**: UI 组件库
- **Zustand**: 状态管理
- **React Router**: 路由管理

## 📝 开发指南

### 添加新的 Tauri 命令

1. 在 \`src-tauri/src/commands/\` 中创建命令函数
2. 在 \`src-tauri/src/main.rs\` 中注册命令
3. 在前端使用 \`invoke()\` 调用命令

**示例**：

\`\`\`rust
// src-tauri/src/commands/mod.rs
#[tauri::command]
pub async fn my_command(param: String) -> Result<String, String> {
    Ok(format!("Received: {}", param))
}

// src-tauri/src/main.rs
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        commands::my_command,
    ])
    .run(tauri::generate_context!())
    .expect("启动失败");
\`\`\`

\`\`\`typescript
// src/App.tsx
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke<string>('my_command', { param: 'test' });
\`\`\`

### 代码规范

- Rust 代码使用 \`cargo fmt\` 格式化
- Rust 代码使用 \`cargo clippy\` 检查
- TypeScript 代码遵循项目 ESLint 配置
- 提交信息遵循 Conventional Commits 规范

## 🧪 测试

\`\`\`bash
# 运行 Rust 单元测试
cd src-tauri
cargo test

# 运行 Rust 代码检查
cargo clippy

# 运行前端测试（待实现）
npm run test
\`\`\`

## 🐛 已知问题

1. **Linux 编译问题**：需要安装完整的系统依赖（见上方）
2. **图标文件**：当前使用占位图标，需要替换为正式图标

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 联系方式

- 项目主页：[GitHub Repository]
- 问题反馈：[GitHub Issues]

---

**生成时间**：2025-10-06
**版本**：v2.2.0
**维护者**：Claude Code (Sonnet 4.5)
