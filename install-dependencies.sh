#!/bin/bash
# bilibili-backup-tauri 系统依赖安装脚本
# 用途：安装 Tauri 在 Linux 系统上所需的开发库

set -e

echo "========================================="
echo "bilibili-backup-tauri 依赖安装脚本"
echo "========================================="
echo ""

# 检测操作系统
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$NAME
    echo "检测到操作系统: $OS"
else
    echo "无法检测操作系统类型"
    exit 1
fi

echo ""
echo "正在安装 Tauri 所需的系统库..."
echo ""

# Ubuntu/Debian 系列
if [[ "$OS" == *"Ubuntu"* ]] || [[ "$OS" == *"Debian"* ]]; then
    echo "使用 apt-get 安装依赖..."
    sudo apt-get update
    sudo apt-get install -y \
        libwebkit2gtk-4.0-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libsoup2.4-dev \
        build-essential \
        curl \
        wget \
        file \
        libssl-dev \
        pkg-config

# Fedora/RHEL 系列
elif [[ "$OS" == *"Fedora"* ]] || [[ "$OS" == *"Red Hat"* ]]; then
    echo "使用 dnf 安装依赖..."
    sudo dnf install -y \
        webkit2gtk4.0-devel \
        gtk3-devel \
        libayatana-appindicator-gtk3-devel \
        librsvg2-devel \
        libsoup-devel \
        openssl-devel \
        curl \
        wget \
        file

# Arch Linux
elif [[ "$OS" == *"Arch"* ]] || [[ "$OS" == *"Manjaro"* ]]; then
    echo "使用 pacman 安装依赖..."
    sudo pacman -Syu --needed \
        webkit2gtk \
        gtk3 \
        libayatana-appindicator \
        librsvg \
        libsoup \
        base-devel \
        curl \
        wget \
        file \
        openssl \
        appmenu-gtk-module

else
    echo "不支持的操作系统: $OS"
    echo "请参考 Tauri 官方文档手动安装依赖:"
    echo "https://tauri.app/v1/guides/getting-started/prerequisites"
    exit 1
fi

echo ""
echo "========================================="
echo "依赖安装完成！"
echo "========================================="
echo ""
echo "下一步："
echo "1. 运行 'cargo check' 验证编译环境"
echo "2. 运行 'npm install' 安装前端依赖"
echo "3. 运行 'npm run tauri dev' 启动开发服务器"
echo "4. 运行 'npm run tauri build' 构建生产版本"
echo ""
