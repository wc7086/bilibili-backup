/**
 * 主布局组件
 *
 * 包含侧边栏导航和顶部用户信息
 */

import { useState, useEffect } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { Layout as AntLayout, Menu, Avatar, Dropdown, message, Space } from 'antd';
import type { MenuProps } from 'antd';
import {
  UserOutlined,
  TeamOutlined,
  StarOutlined,
  HistoryOutlined,
  PlayCircleOutlined,
  ClockCircleOutlined,
  LogoutOutlined,
  StopOutlined,
  HeartOutlined,
} from '@ant-design/icons';
import { AuthAPI, NavInfo } from '../types/auth';

const { Header, Sider, Content } = AntLayout;

export default function Layout() {
  const navigate = useNavigate();
  const location = useLocation();
  const [userInfo, setUserInfo] = useState<NavInfo | null>(null);
  const [collapsed, setCollapsed] = useState(false);

  useEffect(() => {
    loadUserInfo();
  }, []);

  const loadUserInfo = async () => {
    try {
      const info = await AuthAPI.getUserInfo();
      setUserInfo(info);
    } catch (error) {
      console.error('获取用户信息失败:', error);
      message.error('获取用户信息失败');
      navigate('/login');
    }
  };

  const handleLogout = async () => {
    try {
      await AuthAPI.logout();
      message.success('已退出登录');
      navigate('/login');
    } catch (error) {
      console.error('退出登录失败:', error);
      message.error('退出登录失败');
    }
  };

  const userMenuItems: MenuProps['items'] = [
    {
      key: 'logout',
      icon: <LogoutOutlined />,
      label: '退出登录',
      onClick: handleLogout,
    },
  ];

  const menuItems: MenuProps['items'] = [
    {
      key: '/dashboard',
      icon: <UserOutlined />,
      label: '概览',
    },
    {
      key: '/following',
      icon: <HeartOutlined />,
      label: '关注管理',
    },
    {
      key: '/followers',
      icon: <TeamOutlined />,
      label: '粉丝管理',
    },
    {
      key: '/blacklist',
      icon: <StopOutlined />,
      label: '黑名单管理',
    },
    {
      key: '/favorites',
      icon: <StarOutlined />,
      label: '收藏管理',
    },
    {
      key: '/history',
      icon: <HistoryOutlined />,
      label: '历史记录',
    },
    {
      key: '/bangumi',
      icon: <PlayCircleOutlined />,
      label: '追番管理',
    },
    {
      key: '/toview',
      icon: <ClockCircleOutlined />,
      label: '稍后再看',
    },
  ];

  const handleMenuClick = ({ key }: { key: string }) => {
    navigate(key);
  };

  return (
    <AntLayout style={{ minHeight: '100vh' }}>
      <Sider collapsible collapsed={collapsed} onCollapse={setCollapsed}>
        <div
          style={{
            height: 32,
            margin: 16,
            background: 'rgba(255, 255, 255, 0.2)',
            borderRadius: 6,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: '#fff',
            fontSize: collapsed ? 14 : 16,
            fontWeight: 'bold',
          }}
        >
          {collapsed ? 'B站' : 'B站备份'}
        </div>
        <Menu
          theme="dark"
          selectedKeys={[location.pathname]}
          mode="inline"
          items={menuItems}
          onClick={handleMenuClick}
        />
      </Sider>
      <AntLayout>
        <Header
          style={{
            padding: '0 24px',
            background: '#fff',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}
        >
          <div style={{ fontSize: 18, fontWeight: 'bold' }}>
            哔哩哔哩账号备份工具
          </div>
          {userInfo && (
            <Dropdown menu={{ items: userMenuItems }} placement="bottomRight">
              <Space style={{ cursor: 'pointer' }}>
                <Avatar src={userInfo.face} icon={<UserOutlined />} />
                <span>{userInfo.uname || '未登录'}</span>
              </Space>
            </Dropdown>
          )}
        </Header>
        <Content style={{ margin: 24, padding: 24, background: '#fff' }}>
          <Outlet />
        </Content>
      </AntLayout>
    </AntLayout>
  );
}
