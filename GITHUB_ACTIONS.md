# GitHub Actions 自动构建指南

**项目**：bilibili-backup-tauri
**版本**：v2.2.0
**日期**：2025-10-06

---

## 📋 概览

本项目配置了完整的 GitHub Actions 工作流，支持：
- ✅ **自动构建** Linux/Windows/macOS 三平台版本
- ✅ **自动发布** 创建 Release 并上传构建产物
- ✅ **快速检查** 代码质量和编译验证
- ✅ **增量构建** 利用缓存加速构建过程

---

## 🔧 工作流文件

### 1. `build.yml` - 完整构建工作流（主要）

**触发条件**：
- 推送到 `main` 或 `master` 分支
- 推送标签（如 `v1.0.0`）
- 创建或更新 Pull Request
- 手动触发（workflow_dispatch）

**构建矩阵**：
| 平台 | 运行环境 | 输出产物 |
|------|---------|---------|
| **Linux** | `ubuntu-latest` | 可执行文件、AppImage、DEB |
| **Windows** | `windows-latest` | EXE、MSI、NSIS 安装程序 |
| **macOS** | `macos-latest` | APP、DMG（x86_64 + Apple Silicon） |

**构建步骤**：
1. 检出代码
2. 安装系统依赖
3. 设置 Node.js 20 + npm 缓存
4. 设置 Rust 工具链 + Rust 缓存
5. 安装 npm 依赖
6. 构建前端（Vite）
7. 构建 Tauri 应用
8. 上传构建产物

**构建产物**：
- ✅ **Linux**：
  - `bilibili-backup-tauri`（可执行文件，~6MB）
  - `*.AppImage`（便携版）
  - `*.deb`（Debian/Ubuntu 安装包）

- ✅ **Windows**：
  - `bilibili-backup-tauri.exe`（可执行文件，~8-10MB）
  - `*.msi`（MSI 安装包）
  - `*-setup.exe`（NSIS 安装程序）

- ✅ **macOS**：
  - `*.app`（应用包，x86_64 + ARM64）
  - `*.dmg`（磁盘映像，x86_64 + ARM64）

### 2. `quick-build.yml` - 快速构建检查

**触发条件**：
- 推送到任何分支
- 创建或更新 Pull Request
- 手动触发

**检查内容**：
- ✅ 前端代码 Lint
- ✅ 前端构建（Vite）
- ✅ Rust 代码检查（cargo check）
- ✅ Rust 单元测试（cargo test）
- ✅ Rust 代码格式（cargo fmt）
- ✅ Rust 代码质量（cargo clippy）

**用途**：
- 快速验证代码可编译
- 在 PR 中自动运行质量检查
- 不生成完整的构建产物（节省时间）

---

## 🚀 使用方法

### 方法1：推送代码自动构建

```bash
# 1. 推送到主分支触发构建
git add .
git commit -m "feat: 添加新功能"
git push origin main

# 2. GitHub Actions 会自动运行 build.yml
# 3. 在 Actions 页面查看构建进度
# 4. 构建完成后下载 Artifacts
```

### 方法2：打标签创建 Release

```bash
# 1. 创建版本标签
git tag v1.0.0
git push origin v1.0.0

# 2. GitHub Actions 会：
#    - 构建所有平台版本
#    - 自动创建 Release
#    - 上传所有构建产物到 Release

# 3. 在 Releases 页面查看发布
```

### 方法3：手动触发构建

1. 访问 GitHub 仓库
2. 点击 **Actions** 标签
3. 选择 **构建多平台版本** 或 **快速构建测试**
4. 点击 **Run workflow**
5. 选择分支
6. 点击 **Run workflow** 按钮

---

## 📦 下载构建产物

### 从 Actions 页面下载

**适用于**：开发版本、测试构建

1. 进入 **Actions** 页面
2. 点击对应的 workflow 运行记录
3. 滚动到底部的 **Artifacts** 区域
4. 下载所需平台的构建产物

**Artifacts 列表**：
```
bilibili-backup-tauri-linux-x86_64          # Linux 可执行文件
bilibili-backup-tauri-linux-appimage        # Linux AppImage
bilibili-backup-tauri-linux-deb             # Linux DEB 包
bilibili-backup-tauri-windows-x86_64        # Windows EXE
bilibili-backup-tauri-windows-msi           # Windows MSI
bilibili-backup-tauri-windows-nsis          # Windows NSIS 安装程序
bilibili-backup-tauri-macos-x86_64          # macOS APP (Intel)
bilibili-backup-tauri-macos-aarch64         # macOS APP (Apple Silicon)
bilibili-backup-tauri-macos-x86_64-dmg      # macOS DMG (Intel)
bilibili-backup-tauri-macos-aarch64-dmg     # macOS DMG (Apple Silicon)
```

### 从 Releases 页面下载

**适用于**：正式发布版本

1. 进入 **Releases** 页面
2. 选择对应的版本（如 `v1.0.0`）
3. 在 **Assets** 区域下载所需平台的文件

**Release 包含**：
- 所有平台的可执行文件
- 所有平台的安装包
- 自动生成的 Release Notes

---

## ⚙️ 配置说明

### 系统依赖

**Linux（ubuntu-latest）**：
```bash
libwebkit2gtk-4.1-dev       # WebView 渲染引擎
libgtk-3-dev                # GTK 图形库
libayatana-appindicator3-dev # 系统托盘支持
librsvg2-dev                # SVG 支持
patchelf                    # ELF 修补工具
libsoup-3.0-dev             # HTTP 库
libjavascriptcoregtk-4.1-dev # JavaScript 引擎
```

**Windows（windows-latest）**：
- 预装 WebView2（Windows 10/11 自带）
- 预装 Visual C++ 构建工具

**macOS（macos-latest）**：
- 预装 WebKit（系统自带）
- 预装 Xcode 命令行工具

### Rust 缓存优化

使用 `Swatinem/rust-cache@v2` 缓存：
- 编译依赖（`~/.cargo/registry`）
- 构建产物（`target/`）
- 加速构建时间（首次 ~15 分钟，后续 ~5 分钟）

### Node.js 缓存优化

使用 `actions/setup-node@v4` 的 `cache: 'npm'` 缓存：
- npm 依赖（`node_modules/`）
- 加速 npm install（首次 ~2 分钟，后续 ~30 秒）

---

## 🔍 构建时间预估

| 平台 | 首次构建 | 缓存构建 | 说明 |
|------|---------|---------|------|
| **Linux** | ~15 分钟 | ~5 分钟 | 最快 |
| **Windows** | ~20 分钟 | ~7 分钟 | 中等 |
| **macOS** | ~25 分钟 | ~10 分钟 | 最慢（需构建两个架构） |
| **快速检查** | ~8 分钟 | ~3 分钟 | 仅检查，不生成产物 |

**总构建时间**（全平台）：
- 首次：~60 分钟（并行运行）
- 缓存：~10-15 分钟（并行运行）

---

## 📊 构建产物大小

| 平台 | 可执行文件 | 安装包 | 说明 |
|------|-----------|-------|------|
| **Linux** | ~6 MB | ~20 MB (DEB/AppImage) | 已剥离符号 |
| **Windows** | ~8-10 MB | ~10-12 MB (MSI/NSIS) | 已优化 |
| **macOS** | ~8-10 MB | ~15-20 MB (DMG) | 通用二进制或单架构 |

**注意**：
- 可执行文件包含完整的前后端代码
- 前端（React）打包为资源嵌入到可执行文件中
- 依赖系统 WebView（不包含 Chromium）

---

## 🐛 常见问题

### Q1: Actions 构建失败怎么办？

**A**: 检查构建日志：
1. 进入 **Actions** 页面
2. 点击失败的 workflow
3. 查看红色 ❌ 的步骤
4. 展开日志查看错误信息

**常见错误**：
- **系统依赖缺失**：检查 `安装系统依赖` 步骤
- **Rust 编译失败**：检查 `src-tauri/` 代码
- **前端构建失败**：检查 `npm run build` 输出
- **缓存损坏**：清除 Actions 缓存重新构建

### Q2: 如何清除 Actions 缓存？

**A**:
1. 进入 **Settings** → **Actions** → **Caches**
2. 找到对应的缓存条目
3. 点击删除按钮
4. 重新触发构建

### Q3: 如何禁用某个平台的构建？

**A**: 编辑 `.github/workflows/build.yml`：
```yaml
jobs:
  # 保留需要的平台
  build-linux:
    # ...

  # 注释掉不需要的平台
  # build-windows:
  #   # ...

  # build-macos:
  #   # ...
```

### Q4: 如何添加代码签名？

**Linux**：
```yaml
# 不需要签名
```

**Windows**：
```yaml
- name: 签名 EXE
  run: |
    signtool sign /f cert.pfx /p ${{ secrets.CERT_PASSWORD }} \
      src-tauri/target/release/bilibili-backup-tauri.exe
```

**macOS**：
```yaml
- name: 签名和公证
  run: |
    codesign --deep --force --verify --verbose \
      --sign "${{ secrets.APPLE_CERTIFICATE }}" \
      "src-tauri/target/release/bundle/macos/*.app"
```

### Q5: 如何自定义 Release Notes？

**A**: 修改 `create-release` job：
```yaml
- name: 创建 Release
  uses: softprops/action-gh-release@v1
  with:
    body_path: RELEASE_NOTES.md  # 使用自定义文件
    # 或者直接指定内容
    body: |
      ## 🎉 版本更新

      ### 新功能
      - 添加 XXX 功能

      ### 修复
      - 修复 XXX 问题
```

---

## 🔐 安全性

### Secrets 配置

如果需要使用敏感信息（如代码签名证书），需要配置 GitHub Secrets：

1. 进入 **Settings** → **Secrets and variables** → **Actions**
2. 点击 **New repository secret**
3. 添加以下 Secrets（按需）：

| Secret 名称 | 用途 | 平台 |
|------------|------|------|
| `CERT_PASSWORD` | Windows 代码签名证书密码 | Windows |
| `APPLE_CERTIFICATE` | macOS 代码签名证书 | macOS |
| `APPLE_TEAM_ID` | Apple 开发者团队 ID | macOS |
| `NPM_TOKEN` | npm 发布令牌 | 全部 |

### 权限配置

`create-release` job 需要写入权限：
```yaml
permissions:
  contents: write  # 允许创建 Release
```

---

## 📚 参考资料

### GitHub Actions
- **官方文档**: https://docs.github.com/actions
- **Workflow 语法**: https://docs.github.com/actions/reference/workflow-syntax-for-github-actions
- **缓存依赖**: https://docs.github.com/actions/using-workflows/caching-dependencies

### Tauri
- **构建指南**: https://tauri.app/v1/guides/building/
- **GitHub Actions**: https://tauri.app/v1/guides/building/github-actions
- **多平台构建**: https://tauri.app/v1/guides/building/cross-platform

### Actions Marketplace
- **actions/checkout**: https://github.com/actions/checkout
- **actions/setup-node**: https://github.com/actions/setup-node
- **dtolnay/rust-toolchain**: https://github.com/dtolnay/rust-toolchain
- **Swatinem/rust-cache**: https://github.com/Swatinem/rust-cache
- **softprops/action-gh-release**: https://github.com/softprops/action-gh-release

---

## 🎯 最佳实践

### 1. 版本号管理
```bash
# 使用语义化版本
git tag v1.0.0  # 主版本.次版本.修订版本
git tag v1.1.0  # 新增功能
git tag v1.1.1  # 修复 bug
```

### 2. 分支策略
```
main/master    → 稳定版本，触发完整构建和发布
develop        → 开发版本，触发快速检查
feature/*      → 功能分支，触发快速检查
```

### 3. 构建优化
- ✅ 使用缓存加速构建
- ✅ 并行构建多平台
- ✅ 仅在标签推送时创建 Release
- ✅ 使用 `npm ci` 代替 `npm install`（更快更可靠）

### 4. 质量保证
- ✅ 每次 PR 自动运行快速检查
- ✅ 主分支推送触发完整构建
- ✅ 使用 `cargo clippy` 检查代码质量
- ✅ 使用 `cargo fmt` 统一代码格式

---

## 🎉 总结

### 自动化优势
- ✅ **多平台构建**：一次推送，三平台同时构建
- ✅ **自动发布**：打标签即可自动创建 Release
- ✅ **质量保证**：自动运行测试和代码检查
- ✅ **节省时间**：无需手动配置构建环境
- ✅ **可重复性**：每次构建环境完全一致

### 快速上手
1. ✅ 配置文件已完成（`.github/workflows/*.yml`）
2. ✅ 推送代码到 GitHub 即可自动构建
3. ✅ 打标签（`v1.0.0`）即可自动发布
4. ✅ 从 Actions 或 Releases 页面下载构建产物

---

**创建时间**：2025-10-06
**作者**：Claude Code (Sonnet 4.5)
**项目**：bilibili-backup-tauri v2.2.0
