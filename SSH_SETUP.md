# SSH 密钥设置指南

## 问题诊断

当前 SSH 连接到 GitHub 失败，原因是：
- SSH 密钥文件存在：`/home/test/.ssh/id_ed25519`
- SSH 客户端正常工作
- GitHub 服务器可访问
- **但 GitHub 拒绝了密钥认证** → 说明这个公钥还没有添加到 GitHub 账户

---

## 📋 解决步骤

### 步骤1：复制 SSH 公钥

你的 SSH 公钥（指纹：`SHA256:WlTPxZ5UFExdhe5ztg/HcOtj1AO9vmNXq7VWov0gtZw`）：

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGUu3D9z6lepJldjt4aaCzjFsbZE7b8kpmWTn2HYS30v your_email@example.com
```

**复制上面这一整行**（包括 `ssh-ed25519` 开头到 `your_email@example.com` 结尾）

### 步骤2：添加到 GitHub

1. 打开浏览器，访问：https://github.com/settings/keys
2. 点击右上角的 **New SSH key** 按钮（绿色）
3. 填写表单：
   - **Title**：`bilibili-backup-server`（或任意你喜欢的名称）
   - **Key type**：保持默认 `Authentication Key`
   - **Key**：粘贴步骤1复制的公钥（整行）
4. 点击 **Add SSH key** 按钮
5. 如果要求输入 GitHub 密码，请输入你的密码确认

### 步骤3：验证 SSH 连接

添加完成后，在终端运行：

```bash
ssh -T git@github.com
```

**预期输出**：
```
Hi wc7086! You've successfully authenticated, but GitHub does not provide shell access.
```

如果看到这个消息，说明 SSH 配置成功！

### 步骤4：推送代码

SSH 验证成功后，推送代码：

```bash
cd /home/test/bl/bilibili-backup-tauri
git push -u origin main
```

**预期输出**：
```
Enumerating objects: 90, done.
Counting objects: 100% (90/90), done.
Delta compression using up to 8 threads
Compressing objects: 100% (73/73), done.
Writing objects: 100% (90/90), 234.56 KiB | 5.67 MiB/s, done.
Total 90 (delta 8), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (8/8), done.
To github.com:wc7086/bilibili-backup.git
 * [new branch]      main -> main
Branch 'main' set up to track remote branch 'main' from 'origin'.
```

---

## 🔍 常见问题

### Q: 我添加了公钥，但仍然失败

**A**: 请检查：
1. 公钥是否完整复制（从 `ssh-ed25519` 到 `your_email@example.com`）
2. 公钥中间没有换行或多余空格
3. 添加的是 **Authentication Key**（不是 Signing Key）
4. 等待几秒钟后再测试（GitHub 需要几秒钟同步）

### Q: 如何确认公钥已添加成功

**A**:
1. 访问 https://github.com/settings/keys
2. 查看列表中是否有刚才添加的密钥
3. 密钥指纹应该是：`SHA256:WlTPxZ5UFExdhe5ztg/HcOtj1AO9vmNXq7VWov0gtZw`

### Q: 我有多个 GitHub 账户怎么办

**A**: 确保你登录的是 `wc7086` 账户，或者确认 `wc7086/bilibili-backup` 仓库的访问权限。

---

## 📸 添加 SSH 密钥截图指南

如果不确定如何添加，请按照以下截图位置操作：

1. **Settings 页面**：点击右上角头像 → Settings
2. **SSH Keys 页面**：左侧菜单 → SSH and GPG keys
3. **New SSH key 按钮**：右上角绿色按钮
4. **填写表单**：
   ```
   Title: bilibili-backup-server
   Key type: Authentication Key
   Key: [粘贴整个公钥]
   ```
5. **确认添加**：点击 Add SSH key → 输入密码

---

## ✅ 验证清单

添加 SSH 密钥前：
- [ ] 已复制完整的公钥（整行）
- [ ] 已登录 GitHub 账户（wc7086）
- [ ] 已打开 SSH Keys 设置页面

添加 SSH 密钥后：
- [ ] 在 GitHub 页面看到新添加的密钥
- [ ] `ssh -T git@github.com` 验证成功
- [ ] 看到 `Hi wc7086!` 的欢迎消息

推送代码前：
- [ ] SSH 验证成功
- [ ] 在正确的目录：`/home/test/bl/bilibili-backup-tauri`
- [ ] Git 状态正常：`git status` 显示干净

---

**如果按照上述步骤仍然无法推送，请告诉我具体的错误信息，我会帮你进一步诊断。**
