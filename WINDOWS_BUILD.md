# Windows 版本构建指南

**项目**：bilibili-backup-tauri
**版本**：v2.2.0
**日期**：2025-10-06

---

## ⚠️ 重要说明

由于 Tauri 应用依赖平台特定的 WebView 组件：
- **Linux**: webkit2gtk
- **Windows**: WebView2 (EdgeHTML)

**从 Linux 交叉编译到 Windows 非常困难且不推荐**。

---

## ✅ 推荐方案：在 Windows 上构建

### 方法1：直接在 Windows 上构建（推荐）⭐

#### 前置要求

1. **安装 Rust**
   ```powershell
   # 下载并运行 rustup-init.exe
   https://rustup.rs/
   ```

2. **安装 Node.js**
   ```powershell
   # 下载并安装 Node.js LTS
   https://nodejs.org/
   ```

3. **安装 WebView2**
   - Windows 10/11 已预装
   - 如需手动安装：https://developer.microsoft.com/microsoft-edge/webview2/

4. **安装 Visual Studio Build Tools**
   ```powershell
   # 下载 Visual Studio Installer
   https://visualstudio.microsoft.com/downloads/

   # 安装 "Desktop development with C++" 工作负载
   ```

#### 构建步骤

1. **克隆或传输项目到 Windows**
   ```powershell
   # 从 Linux 复制整个项目目录到 Windows
   # 或使用 Git 克隆
   ```

2. **安装依赖**
   ```powershell
   cd bilibili-backup-tauri
   npm install
   ```

3. **构建 Debug 版本**
   ```powershell
   npm run tauri:dev
   ```

4. **构建 Release 版本**
   ```powershell
   npm run tauri:build
   ```

5. **获取可执行文件**
   - **MSI 安装包**：`src-tauri/target/release/bundle/msi/bilibili-backup-tauri_2.2.0_x64_en-US.msi`
   - **EXE 可执行文件**：`src-tauri/target/release/bilibili-backup-tauri.exe`

#### 输出文件说明

| 文件类型 | 路径 | 说明 |
|---------|------|------|
| **EXE** | `src-tauri/target/release/bilibili-backup-tauri.exe` | 独立可执行文件（前后端都在此文件中）|
| **MSI** | `src-tauri/target/release/bundle/msi/*.msi` | Windows 安装包 |
| **NSIS** | `src-tauri/target/release/bundle/nsis/*.exe` | NSIS 安装程序（可选）|

✅ **EXE 文件包含完整的前后端**：
- Rust 后端（所有业务逻辑）
- React 前端（打包为资源）
- WebView2 渲染引擎调用

---

## 🔧 方法2：GitHub Actions 自动构建

创建 `.github/workflows/build-windows.yml`：

```yaml
name: Build Windows Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-windows:
    runs-on: windows-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc

      - name: Install dependencies
        run: npm install

      - name: Build Tauri App
        run: npm run tauri:build

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: bilibili-backup-tauri-windows
          path: |
            src-tauri/target/release/bilibili-backup-tauri.exe
            src-tauri/target/release/bundle/msi/*.msi
```

**使用方法**：
1. 将代码推送到 GitHub
2. 手动触发 workflow 或打 tag
3. 从 Artifacts 下载构建产物

---

## 📦 方法3：使用 Docker（复杂，不推荐）

虽然可以使用 Docker + Wine 来尝试交叉编译，但由于以下原因不推荐：
- 需要 Wine 环境模拟 Windows
- WebView2 依赖难以处理
- 构建时间长且不稳定
- 调试困难

---

## 🚀 快速传输方案

### 如何将 Linux 源码传输到 Windows

#### 方案A：使用 Git

```bash
# Linux 上
cd /home/test/bl/bilibili-backup-tauri
git init
git add .
git commit -m "Initial commit"
git remote add origin <your-repo-url>
git push -u origin main

# Windows 上
git clone <your-repo-url>
```

#### 方案B：使用 SCP/SFTP

```powershell
# Windows 上使用 WinSCP 或 FileZilla
# 连接到 Linux 服务器
# 下载整个 /home/test/bl/bilibili-backup-tauri 目录
```

#### 方案C：压缩传输

```bash
# Linux 上
cd /home/test/bl
tar -czf bilibili-backup-tauri.tar.gz bilibili-backup-tauri/

# 传输到 Windows 后解压
```

---

## 📝 构建配置说明

### tauri.conf.json

项目已配置为支持 Windows：

```json
{
  "bundle": {
    "identifier": "com.bilibili.backup",
    "targets": ["msi", "nsis"],  // Windows 安装包格式
    "windows": {
      "certificateThumbprint": null,  // 可选：代码签名
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

### Cargo.toml

已配置 Windows 优化：

```toml
[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

---

## 🎯 预期输出

### Windows Release 构建产物

```
src-tauri/target/release/
├── bilibili-backup-tauri.exe          # 主可执行文件（~8MB）⭐
└── bundle/
    ├── msi/
    │   └── bilibili-backup-tauri_2.2.0_x64_en-US.msi  # MSI 安装包
    └── nsis/
        └── bilibili-backup-tauri_2.2.0_x64-setup.exe  # NSIS 安装程序
```

### 文件说明

**bilibili-backup-tauri.exe**：
- ✅ 包含完整的 Rust 后端
- ✅ 包含打包的 React 前端（dist/）
- ✅ 独立运行，无需额外文件
- ✅ 大小约 8-10 MB
- ✅ 依赖 WebView2（Windows 10/11 自带）

**MSI 安装包**：
- ✅ 包含 .exe 文件
- ✅ 创建开始菜单快捷方式
- ✅ 支持静默安装
- ✅ 可通过"程序和功能"卸载

---

## ⚡ 性能预期

在 Windows 上的性能指标（预计）：

| 指标 | 数值 |
|------|------|
| EXE 大小 | ~8-10 MB |
| 启动时间 | <0.5 秒 |
| 内存占用 | ~50-60 MB |
| CPU 占用（空闲）| <1% |

---

## 🐛 常见问题

### Q1: 构建失败提示缺少 WebView2

**A**: Windows 10/11 已预装。如需手动安装：
```powershell
# 下载 Evergreen Bootstrapper
https://go.microsoft.com/fwlink/p/?LinkId=2124703
```

### Q2: 构建失败提示缺少 MSVC

**A**: 安装 Visual Studio Build Tools：
```powershell
# 下载并安装 "Desktop development with C++"
https://visualstudio.microsoft.com/downloads/
```

### Q3: npm install 失败

**A**: 使用国内镜像：
```powershell
npm config set registry https://registry.npmmirror.com
npm install
```

### Q4: Rust 编译慢

**A**: 启用并行编译：
```powershell
# 在 ~/.cargo/config.toml 添加
[build]
jobs = 4  # 根据 CPU 核心数调整
```

---

## 📚 相关文档

- **Tauri Windows 构建指南**: https://tauri.app/v1/guides/building/windows
- **WebView2 文档**: https://developer.microsoft.com/microsoft-edge/webview2/
- **Rust Windows 工具链**: https://rust-lang.github.io/rustup/installation/windows.html

---

## ✅ 验证构建

构建完成后，验证步骤：

1. **检查文件存在**：
   ```powershell
   ls src-tauri/target/release/bilibili-backup-tauri.exe
   ls src-tauri/target/release/bundle/msi/*.msi
   ```

2. **运行 EXE**：
   ```powershell
   .\src-tauri\target\release\bilibili-backup-tauri.exe
   ```

3. **测试基本功能**：
   - 应用启动
   - UI 渲染正常
   - 网络请求正常

4. **安装 MSI**：
   ```powershell
   # 双击 MSI 文件或
   msiexec /i bilibili-backup-tauri_2.2.0_x64_en-US.msi
   ```

---

## 🎉 总结

### 推荐流程

1. ✅ **将项目传输到 Windows 机器**
2. ✅ **安装 Rust + Node.js + Visual Studio Build Tools**
3. ✅ **运行 `npm run tauri:build`**
4. ✅ **获取 `bilibili-backup-tauri.exe`**（前后端都在此文件中）

### 为什么 EXE 包含前后端

Tauri 在构建时会：
1. 编译 Rust 后端为原生代码
2. 打包 React 前端（dist/）为资源文件
3. 将前端资源嵌入到 EXE 文件中
4. 运行时使用 WebView2 渲染前端

**结果**：单个 EXE 文件包含：
- ✅ Rust 后端（业务逻辑）
- ✅ React 前端（UI）
- ✅ 资源文件（图标、配置等）

---

**创建时间**：2025-10-06
**作者**：Claude Code (Sonnet 4.5)
