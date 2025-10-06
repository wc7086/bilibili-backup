/**
 * Dashboard 主页面
 *
 * 显示各功能模块的概览信息
 */

import { Card, Row, Col, Typography, Space } from 'antd';
import {
  HeartOutlined,
  TeamOutlined,
  StarOutlined,
  HistoryOutlined,
  PlayCircleOutlined,
  ClockCircleOutlined,
  StopOutlined,
} from '@ant-design/icons';

const { Title, Paragraph } = Typography;

export default function Dashboard() {
  const features = [
    {
      title: '关注管理',
      icon: <HeartOutlined style={{ fontSize: 32, color: '#1890ff' }} />,
      description: '备份、还原和管理您的关注列表',
      path: '/following',
    },
    {
      title: '粉丝管理',
      icon: <TeamOutlined style={{ fontSize: 32, color: '#52c41a' }} />,
      description: '备份和查看您的粉丝列表',
      path: '/followers',
    },
    {
      title: '黑名单管理',
      icon: <StopOutlined style={{ fontSize: 32, color: '#ff4d4f' }} />,
      description: '备份、还原和管理黑名单',
      path: '/blacklist',
    },
    {
      title: '收藏管理',
      icon: <StarOutlined style={{ fontSize: 32, color: '#faad14' }} />,
      description: '备份、还原和管理收藏夹',
      path: '/favorites',
    },
    {
      title: '历史记录',
      icon: <HistoryOutlined style={{ fontSize: 32, color: '#722ed1' }} />,
      description: '备份和管理观看历史',
      path: '/history',
    },
    {
      title: '追番管理',
      icon: <PlayCircleOutlined style={{ fontSize: 32, color: '#eb2f96' }} />,
      description: '备份、还原和管理追番列表',
      path: '/bangumi',
    },
    {
      title: '稍后再看',
      icon: <ClockCircleOutlined style={{ fontSize: 32, color: '#13c2c2' }} />,
      description: '备份、还原和管理稍后再看',
      path: '/toview',
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: 24 }}>
        <Title level={2}>欢迎使用B站账号备份工具</Title>
        <Paragraph type="secondary">
          这是一个功能完整的B站账号数据备份工具，支持关注、粉丝、黑名单、收藏、历史记录、追番和稍后再看的备份与还原。
        </Paragraph>
      </div>

      <Row gutter={[16, 16]}>
        {features.map((feature, index) => (
          <Col xs={24} sm={12} md={8} key={index}>
            <Card
              hoverable
              onClick={() => (window.location.href = `#${feature.path}`)}
              style={{ height: '100%' }}
            >
              <Space direction="vertical" size="middle" style={{ width: '100%' }}>
                <div style={{ textAlign: 'center' }}>{feature.icon}</div>
                <div>
                  <Title level={4} style={{ marginBottom: 8 }}>
                    {feature.title}
                  </Title>
                  <Paragraph type="secondary" style={{ marginBottom: 0 }}>
                    {feature.description}
                  </Paragraph>
                </div>
              </Space>
            </Card>
          </Col>
        ))}
      </Row>

      <Card style={{ marginTop: 24 }}>
        <Title level={4}>使用说明</Title>
        <Space direction="vertical">
          <Paragraph>
            1. <strong>备份</strong>：点击对应模块的"备份"按钮，将当前数据保存到本地
          </Paragraph>
          <Paragraph>
            2. <strong>还原</strong>：上传之前备份的数据文件，恢复到您的账号中
          </Paragraph>
          <Paragraph>
            3. <strong>清空</strong>：清除账号中对应模块的所有数据（请谨慎操作）
          </Paragraph>
          <Paragraph>
            4. <strong>导出/导入</strong>：将数据导出为JSON文件或从文件导入数据
          </Paragraph>
        </Space>
      </Card>
    </div>
  );
}
