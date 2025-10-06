/**
 * 粉丝管理页面
 *
 * 支持备份粉丝列表
 */

import { useState } from 'react';
import {
  Card,
  Button,
  Table,
  Space,
  message,
  Statistic,
  Row,
  Col,
  Input,
  Avatar,
} from 'antd';
import {
  DownloadOutlined,
  ExportOutlined,
  ImportOutlined,
  SearchOutlined,
  UserOutlined,
} from '@ant-design/icons';
import type { Follower } from '../types/api';
import { backupFollowers, selectJsonFile, saveJsonFile } from '../utils/api';
import { writeTextFile, readTextFile } from '@tauri-apps/api/fs';

export default function Followers() {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<Follower[]>([]);
  const [filteredData, setFilteredData] = useState<Follower[]>([]);
  const [searchText, setSearchText] = useState('');

  const handleBackup = async () => {
    setLoading(true);
    try {
      const result = await backupFollowers();
      setData(result);
      setFilteredData(result);
      message.success(`备份成功！共 ${result.length} 个粉丝`);
    } catch (error) {
      message.error(`备份失败: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleExport = async () => {
    if (data.length === 0) {
      message.warning('没有数据可导出');
      return;
    }

    try {
      const filePath = await saveJsonFile();
      if (filePath) {
        await writeTextFile(filePath, JSON.stringify(data, null, 2));
        message.success('导出成功！');
      }
    } catch (error) {
      message.error(`导出失败: ${error}`);
    }
  };

  const handleImport = async () => {
    try {
      const filePath = await selectJsonFile();
      if (filePath) {
        const content = await readTextFile(filePath);
        const imported = JSON.parse(content) as Follower[];
        setData(imported);
        setFilteredData(imported);
        message.success(`导入成功！共 ${imported.length} 条数据`);
      }
    } catch (error) {
      message.error(`导入失败: ${error}`);
    }
  };

  const handleSearch = (value: string) => {
    setSearchText(value);
    if (!value) {
      setFilteredData(data);
    } else {
      const filtered = data.filter((item) =>
        item.uname.toLowerCase().includes(value.toLowerCase())
      );
      setFilteredData(filtered);
    }
  };

  const columns = [
    {
      title: '头像',
      dataIndex: 'face',
      key: 'face',
      width: 80,
      render: (face: string) => <Avatar src={face} icon={<UserOutlined />} />,
    },
    {
      title: '用户名',
      dataIndex: 'uname',
      key: 'uname',
    },
    {
      title: 'UID',
      dataIndex: 'mid',
      key: 'mid',
    },
    {
      title: '签名',
      dataIndex: 'sign',
      key: 'sign',
      ellipsis: true,
    },
    {
      title: '关注时间',
      dataIndex: 'mtime',
      key: 'mtime',
      render: (time: number) => new Date(time * 1000).toLocaleString('zh-CN'),
    },
  ];

  return (
    <div>
      <Card
        title="粉丝管理"
        extra={
          <Space>
            <Button
              type="primary"
              icon={<DownloadOutlined />}
              onClick={handleBackup}
              loading={loading}
            >
              备份
            </Button>
            <Button icon={<ExportOutlined />} onClick={handleExport}>
              导出
            </Button>
            <Button icon={<ImportOutlined />} onClick={handleImport}>
              导入
            </Button>
          </Space>
        }
      >
        <Row gutter={16} style={{ marginBottom: 16 }}>
          <Col span={12}>
            <Card>
              <Statistic title="粉丝总数" value={data.length} />
            </Card>
          </Col>
          <Col span={12}>
            <Card>
              <Statistic
                title="显示条数"
                value={filteredData.length}
                suffix={`/ ${data.length}`}
              />
            </Card>
          </Col>
        </Row>

        <Input
          placeholder="搜索用户名"
          prefix={<SearchOutlined />}
          value={searchText}
          onChange={(e) => handleSearch(e.target.value)}
          style={{ marginBottom: 16 }}
          allowClear
        />

        <Table
          dataSource={filteredData}
          columns={columns}
          rowKey="mid"
          loading={loading}
          pagination={{ pageSize: 20 }}
        />
      </Card>
    </div>
  );
}
