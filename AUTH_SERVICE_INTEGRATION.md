# AuthService 集成指南

本文档说明如何将 AuthService（认证服务）集成到 Tauri 应用中。

## 目录

- [概述](#概述)
- [后端集成](#后端集成)
- [前端集成](#前端集成)
- [使用示例](#使用示例)
- [API参考](#api参考)

## 概述

AuthService 提供完整的用户认证功能，包括：

- ✅ 二维码登录（生成二维码、轮询扫码状态）
- ✅ Cookie登录
- ✅ 用户信息获取
- ✅ WBI签名器初始化
- ✅ 登出功能

## 后端集成

### 1. 注册 AuthService 状态

在 `src-tauri/src/main.rs` 中，将 `AuthService` 注册为 Tauri 应用的全局状态：

```rust
// src-tauri/src/main.rs
use bilibili_backup_tauri::services::AuthService;

fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with_target(false)
        .init();

    tracing::info!("启动哔哩哔哩账号备份工具 v{}", env!("CARGO_PKG_VERSION"));

    // 创建认证服务实例
    let auth_service = AuthService::new();

    // 启动Tauri应用
    tauri::Builder::default()
        .manage(auth_service)  // 注册全局状态
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_version,
            // 认证相关命令
            commands::generate_login_qrcode,
            commands::poll_login_status,
            commands::login_with_cookie,
            commands::get_user_info,
            commands::get_current_user,
            commands::logout,
        ])
        .run(tauri::generate_context!())
        .expect("启动Tauri应用失败");
}
```

### 2. 注册 Tauri 命令

认证相关的命令已在 `commands/auth.rs` 中定义，通过 `commands/mod.rs` 重新导出。

确保在 `generate_handler!` 宏中添加所有认证命令：

```rust
.invoke_handler(tauri::generate_handler![
    // 现有命令
    commands::greet,
    commands::get_version,

    // 认证命令
    commands::generate_login_qrcode,
    commands::poll_login_status,
    commands::login_with_cookie,
    commands::get_user_info,
    commands::get_current_user,
    commands::logout,
])
```

### 3. Cargo.toml 依赖

AuthService 依赖以下 crate（这些依赖在项目初始化时已添加）：

```toml
[dependencies]
# HTTP客户端
reqwest = { version = "0.11", features = ["json", "cookies"] }

# 异步运行时
tokio = { version = "1", features = ["full"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 错误处理
thiserror = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 加密
md5 = "0.7"

# 时间
chrono = "0.4"

# 随机数
rand = "0.8"
```

## 前端集成

### 1. 导入类型定义

前端 TypeScript 类型已在 `src/types/auth.ts` 中定义。

在你的组件中导入：

```typescript
import { AuthAPI, QRCodeLogin, LoginStatus } from '@/types/auth';
import type { QRCode, LoginResult, NavInfo, AuthUser } from '@/types/auth';
```

### 2. 使用示例

#### 二维码登录（简单方式）

```typescript
import { QRCodeLogin, LoginStatus } from '@/types/auth';

const login = new QRCodeLogin();

// 开始登录流程
const qrcode = await login.start((status) => {
  switch (status.code) {
    case LoginStatus.Success:
      console.log('登录成功!');
      // 刷新页面或跳转到主页
      break;
    case LoginStatus.QRNotScanned:
      console.log('等待扫描...');
      break;
    case LoginStatus.QRScanned:
      console.log('已扫描，请确认...');
      break;
    case LoginStatus.QRExpired:
      console.log('二维码已失效');
      // 生成新的二维码
      break;
  }
});

// 显示二维码
console.log('二维码URL:', qrcode.url);

// 需要时可以停止轮询
// login.stop();
```

#### 二维码登录（手动控制）

```typescript
import { AuthAPI, LoginStatus } from '@/types/auth';

// 1. 生成二维码
const qrcode = await AuthAPI.generateLoginQRCode();
console.log('请扫描二维码:', qrcode.url);

// 2. 轮询登录状态
const pollInterval = setInterval(async () => {
  try {
    const result = await AuthAPI.pollLoginStatus(qrcode.qrcode_key);

    if (result.code === LoginStatus.Success) {
      console.log('登录成功!');
      clearInterval(pollInterval);
    } else if (result.code === LoginStatus.QRExpired) {
      console.log('二维码已失效');
      clearInterval(pollInterval);
    } else if (result.code === LoginStatus.QRScanned) {
      console.log('已扫描，等待确认...');
    } else {
      console.log('等待扫描...');
    }
  } catch (error) {
    console.error('轮询失败:', error);
    clearInterval(pollInterval);
  }
}, 2000);
```

#### Cookie登录

```typescript
import { AuthAPI } from '@/types/auth';

try {
  const cookie = 'DedeUserID=123456; bili_jct=xxx; SESSDATA=yyy';
  const user = await AuthAPI.loginWithCookie(cookie);
  console.log('登录成功:', user.uid);
} catch (error) {
  console.error('登录失败:', error);
}
```

#### 获取用户信息

```typescript
import { AuthAPI } from '@/types/auth';

// 获取当前用户（本地缓存）
const user = await AuthAPI.getCurrentUser();
if (user) {
  console.log('当前用户:', user.uid);
} else {
  console.log('未登录');
}

// 获取导航信息（需要网络请求）
const navInfo = await AuthAPI.getUserInfo();
console.log('用户名:', navInfo.uname);
console.log('用户ID:', navInfo.mid);
```

#### 登出

```typescript
import { AuthAPI } from '@/types/auth';

await AuthAPI.logout();
console.log('已登出');
```

## 使用示例

### React 登录组件示例

```typescript
import React, { useState, useEffect } from 'react';
import { QRCodeLogin, LoginStatus } from '@/types/auth';
import type { QRCode, LoginResult } from '@/types/auth';

const LoginPage: React.FC = () => {
  const [qrcode, setQrcode] = useState<QRCode | null>(null);
  const [status, setStatus] = useState<LoginResult | null>(null);
  const [login] = useState(() => new QRCodeLogin());

  useEffect(() => {
    // 开始登录流程
    const startLogin = async () => {
      try {
        const qr = await login.start((result) => {
          setStatus(result);

          if (result.code === LoginStatus.Success) {
            // 登录成功，跳转到主页
            window.location.href = '/home';
          }
        });
        setQrcode(qr);
      } catch (error) {
        console.error('生成二维码失败:', error);
      }
    };

    startLogin();

    // 清理函数
    return () => {
      login.stop();
    };
  }, [login]);

  const getStatusText = () => {
    if (!status) return '正在生成二维码...';

    switch (status.code) {
      case LoginStatus.QRNotScanned:
        return '请使用哔哩哔哩APP扫描二维码';
      case LoginStatus.QRScanned:
        return '已扫描，请在手机上确认登录';
      case LoginStatus.Success:
        return '登录成功，正在跳转...';
      case LoginStatus.QRExpired:
        return '二维码已失效，请刷新页面';
      default:
        return status.message;
    }
  };

  return (
    <div className="login-page">
      <h1>哔哩哔哩账号备份工具</h1>

      {qrcode && (
        <div className="qrcode-container">
          <img src={qrcode.url} alt="登录二维码" />
          <p>{getStatusText()}</p>
        </div>
      )}
    </div>
  );
};

export default LoginPage;
```

### Vue 登录组件示例

```vue
<template>
  <div class="login-page">
    <h1>哔哩哔哩账号备份工具</h1>

    <div v-if="qrcode" class="qrcode-container">
      <img :src="qrcode.url" alt="登录二维码" />
      <p>{{ statusText }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { QRCodeLogin, LoginStatus } from '@/types/auth';
import type { QRCode, LoginResult } from '@/types/auth';

const qrcode = ref<QRCode | null>(null);
const status = ref<LoginResult | null>(null);
const login = new QRCodeLogin();

const statusText = computed(() => {
  if (!status.value) return '正在生成二维码...';

  switch (status.value.code) {
    case LoginStatus.QRNotScanned:
      return '请使用哔哩哔哩APP扫描二维码';
    case LoginStatus.QRScanned:
      return '已扫描，请在手机上确认登录';
    case LoginStatus.Success:
      return '登录成功，正在跳转...';
    case LoginStatus.QRExpired:
      return '二维码已失效，请刷新页面';
    default:
      return status.value.message;
  }
});

onMounted(async () => {
  try {
    const qr = await login.start((result) => {
      status.value = result;

      if (result.code === LoginStatus.Success) {
        // 登录成功，跳转到主页
        window.location.href = '/home';
      }
    });
    qrcode.value = qr;
  } catch (error) {
    console.error('生成二维码失败:', error);
  }
});

onUnmounted(() => {
  login.stop();
});
</script>

<style scoped>
.login-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}

.qrcode-container {
  text-align: center;
}

.qrcode-container img {
  width: 200px;
  height: 200px;
}
</style>
```

## API参考

### Rust API

#### AuthService

```rust
// 创建新实例
let auth = AuthService::new();

// 生成二维码
let qrcode: QRCode = auth.generate_qrcode().await?;

// 轮询登录状态
let result: LoginResult = auth.poll_qrcode(&qrcode_key).await?;

// Cookie登录
let user: AuthUser = auth.login_with_cookie(&cookie).await?;

// 获取导航信息
let nav_info: NavInfo = auth.get_nav_info().await?;

// 获取当前用户
let user: Option<AuthUser> = auth.get_current_user().await;

// 获取WBI签名器
let signer: Option<WbiSigner> = auth.get_wbi_signer().await;

// 登出
auth.logout().await;
```

### TypeScript API

#### AuthAPI（静态方法）

```typescript
// 生成登录二维码
const qrcode: QRCode = await AuthAPI.generateLoginQRCode();

// 轮询登录状态
const result: LoginResult = await AuthAPI.pollLoginStatus(qrcodeKey);

// Cookie登录
const user: AuthUser = await AuthAPI.loginWithCookie(cookie);

// 获取用户信息
const navInfo: NavInfo = await AuthAPI.getUserInfo();

// 获取当前用户
const user: AuthUser | null = await AuthAPI.getCurrentUser();

// 登出
await AuthAPI.logout();
```

#### QRCodeLogin（工具类）

```typescript
const login = new QRCodeLogin();

// 开始登录流程（自动轮询）
const qrcode: QRCode = await login.start((status: LoginResult) => {
  // 处理状态变化
});

// 停止轮询
login.stop();
```

### 数据结构

#### QRCode

```typescript
interface QRCode {
  url: string;           // 二维码URL
  qrcode_key: string;    // 二维码密钥
}
```

#### LoginResult

```typescript
interface LoginResult {
  code: number;          // 状态码
  message: string;       // 响应消息
  url?: string;          // 跳转URL（可选）
  refresh_token?: string; // 刷新令牌（可选）
  timestamp?: number;    // 时间戳（可选）
}
```

#### LoginStatus（枚举）

```typescript
enum LoginStatus {
  Success = 0,           // 登录成功
  QRExpired = 86038,     // 二维码已失效
  QRNotScanned = 86101,  // 二维码未扫描
  QRScanned = 86090,     // 二维码已扫描，等待确认
}
```

#### NavInfo

```typescript
interface NavInfo {
  mid?: number;          // 用户ID
  uname?: string;        // 用户名
  face?: string;         // 头像URL
  isLogin?: boolean;     // 是否登录
  wbi_img?: WbiImg;      // WBI图片信息
}
```

#### AuthUser

```typescript
interface AuthUser {
  uid: string;                  // 用户ID
  cookie: string;               // Cookie字符串
  bili_jct: string;             // CSRF令牌
  is_cancelled_account: boolean; // 是否已注销账号
}
```

## 测试

运行单元测试：

```bash
cd src-tauri
cargo test services::auth
```

运行所有测试：

```bash
cd src-tauri
cargo test
```

## 故障排查

### 常见问题

1. **二维码生成失败**
   - 检查网络连接
   - 查看日志输出（tracing日志）

2. **Cookie登录失败**
   - 确保Cookie包含 `DedeUserID` 和 `bili_jct` 字段
   - 验证Cookie未过期

3. **轮询登录状态失败**
   - 检查网络连接
   - 确保二维码未过期（通常2分钟有效期）

### 日志调试

在 `main.rs` 中设置日志级别：

```rust
tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"))  // 设置为debug级别
    )
    .with_target(false)
    .init();
```

或通过环境变量：

```bash
RUST_LOG=debug cargo run
```

## 下一步

- 集成其他服务模块（历史记录、收藏夹等）
- 实现持久化存储（保存登录状态）
- 添加自动重新登录功能

## 许可证

MIT License
