/**
 * 认证相关的TypeScript类型定义
 *
 * 与Rust后端的数据结构保持一致
 */

/**
 * 二维码信息
 */
export interface QRCode {
  /** 二维码URL */
  url: string;
  /** 二维码密钥 */
  qrcode_key: string;
}

/**
 * 登录结果
 */
export interface LoginResult {
  /** 状态码 */
  code: number;
  /** 响应消息 */
  message: string;
  /** 跳转URL（可选） */
  url?: string;
  /** 刷新令牌（可选） */
  refresh_token?: string;
  /** 时间戳（可选） */
  timestamp?: number;
}

/**
 * 登录状态码枚举
 */
export enum LoginStatus {
  /** 登录成功 */
  Success = 0,
  /** 二维码已失效 */
  QRExpired = 86038,
  /** 二维码未扫描 */
  QRNotScanned = 86101,
  /** 二维码已扫描，等待确认 */
  QRScanned = 86090,
}

/**
 * WBI签名图片信息
 */
export interface WbiImg {
  /** img_url */
  img_url: string;
  /** sub_url */
  sub_url: string;
}

/**
 * 导航信息
 */
export interface NavInfo {
  /** 用户ID */
  mid?: number;
  /** 用户名 */
  uname?: string;
  /** 头像URL */
  face?: string;
  /** 是否登录 */
  isLogin?: boolean;
  /** WBI图片信息 */
  wbi_img?: WbiImg;
}

/**
 * 认证用户信息
 */
export interface AuthUser {
  /** 用户ID */
  uid: string;
  /** Cookie字符串 */
  cookie: string;
  /** CSRF令牌 */
  bili_jct: string;
  /** 是否已注销账号 */
  is_cancelled_account: boolean;
}

/**
 * 认证服务API
 *
 * 使用Tauri的invoke调用后端命令
 */
export class AuthAPI {
  /**
   * 生成登录二维码
   *
   * @returns Promise<QRCode> 二维码信息
   * @throws Error 如果生成失败
   *
   * @example
   * ```typescript
   * const qrcode = await AuthAPI.generateLoginQRCode();
   * console.log('请扫描二维码:', qrcode.url);
   * ```
   */
  static async generateLoginQRCode(): Promise<QRCode> {
    const { invoke } = await import('@tauri-apps/api/tauri');
    return invoke<QRCode>('generate_login_qrcode');
  }

  /**
   * 轮询登录状态
   *
   * @param qrcodeKey 二维码密钥
   * @returns Promise<LoginResult> 登录结果
   * @throws Error 如果轮询失败
   *
   * @example
   * ```typescript
   * const result = await AuthAPI.pollLoginStatus(qrcodeKey);
   * if (result.code === LoginStatus.Success) {
   *   console.log('登录成功!');
   * }
   * ```
   */
  static async pollLoginStatus(qrcodeKey: string): Promise<LoginResult> {
    const { invoke } = await import('@tauri-apps/api/tauri');
    return invoke<LoginResult>('poll_login_status', { qrcodeKey });
  }

  /**
   * 使用Cookie登录
   *
   * @param cookie Cookie字符串
   * @returns Promise<AuthUser> 登录用户信息
   * @throws Error 如果登录失败
   *
   * @example
   * ```typescript
   * const user = await AuthAPI.loginWithCookie('DedeUserID=123; bili_jct=xxx');
   * console.log('登录成功:', user.uid);
   * ```
   */
  static async loginWithCookie(cookie: string): Promise<AuthUser> {
    const { invoke } = await import('@tauri-apps/api/tauri');
    return invoke<AuthUser>('login_with_cookie', { cookie });
  }

  /**
   * 获取用户信息
   *
   * @returns Promise<NavInfo> 用户导航信息
   * @throws Error 如果获取失败
   *
   * @example
   * ```typescript
   * const navInfo = await AuthAPI.getUserInfo();
   * console.log('用户名:', navInfo.uname);
   * ```
   */
  static async getUserInfo(): Promise<NavInfo> {
    const { invoke } = await import('@tauri-apps/api/tauri');
    return invoke<NavInfo>('get_user_info');
  }

  /**
   * 获取当前用户
   *
   * @returns Promise<AuthUser | null> 当前用户（如果已登录）
   * @throws Error 如果获取失败
   *
   * @example
   * ```typescript
   * const user = await AuthAPI.getCurrentUser();
   * if (user) {
   *   console.log('当前用户:', user.uid);
   * } else {
   *   console.log('未登录');
   * }
   * ```
   */
  static async getCurrentUser(): Promise<AuthUser | null> {
    const { invoke } = await import('@tauri-apps/api/tauri');
    return invoke<AuthUser | null>('get_current_user');
  }

  /**
   * 登出
   *
   * @returns Promise<void>
   * @throws Error 如果登出失败
   *
   * @example
   * ```typescript
   * await AuthAPI.logout();
   * console.log('已登出');
   * ```
   */
  static async logout(): Promise<void> {
    const { invoke } = await import('@tauri-apps/api/tauri');
    return invoke<void>('logout');
  }
}

/**
 * 二维码登录工具类
 *
 * 提供二维码登录的便捷方法
 */
export class QRCodeLogin {
  private qrcodeKey: string | null = null;
  private pollingInterval: number | null = null;

  /**
   * 开始二维码登录流程
   *
   * @param onStatusChange 状态变化回调
   * @param pollIntervalMs 轮询间隔（毫秒），默认2000
   * @returns Promise<QRCode> 二维码信息
   *
   * @example
   * ```typescript
   * const login = new QRCodeLogin();
   * const qrcode = await login.start((status) => {
   *   switch (status.code) {
   *     case LoginStatus.Success:
   *       console.log('登录成功!');
   *       break;
   *     case LoginStatus.QRNotScanned:
   *       console.log('等待扫描...');
   *       break;
   *     case LoginStatus.QRScanned:
   *       console.log('已扫描，请确认...');
   *       break;
   *     case LoginStatus.QRExpired:
   *       console.log('二维码已失效');
   *       break;
   *   }
   * });
   * ```
   */
  async start(
    onStatusChange: (status: LoginResult) => void,
    pollIntervalMs: number = 2000
  ): Promise<QRCode> {
    // 生成二维码
    const qrcode = await AuthAPI.generateLoginQRCode();
    this.qrcodeKey = qrcode.qrcode_key;

    // 开始轮询
    this.pollingInterval = window.setInterval(async () => {
      if (!this.qrcodeKey) return;

      try {
        const result = await AuthAPI.pollLoginStatus(this.qrcodeKey);
        onStatusChange(result);

        // 如果登录成功或二维码失效，停止轮询
        if (
          result.code === LoginStatus.Success ||
          result.code === LoginStatus.QRExpired
        ) {
          this.stop();
        }
      } catch (error) {
        console.error('轮询登录状态失败:', error);
      }
    }, pollIntervalMs);

    return qrcode;
  }

  /**
   * 停止轮询
   */
  stop(): void {
    if (this.pollingInterval !== null) {
      window.clearInterval(this.pollingInterval);
      this.pollingInterval = null;
    }
    this.qrcodeKey = null;
  }
}
