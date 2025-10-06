/**
 * 历史记录页面
 *
 * 支持备份、清空、导入导出历史记录
 */

import { useState } from 'react';
import {
  Card,
  Button,
  Table,
  Space,
  message,
  Modal,
  Statistic,
  Row,
  Col,
  Input,
  Avatar,
} from 'antd';
import {
  DownloadOutlined,
  DeleteOutlined,
  ExportOutlined,
  ImportOutlined,
  SearchOutlined,
  PlayCircleOutlined,
} from '@ant-design/icons';
import type { History } from '../types/api';
import {
  backupHistory,
  clearHistory,
  exportHistory,
  importHistory,
  selectJsonFile,
  saveJsonFile,
} from '../utils/api';

export default function HistoryPage() {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<History[]>([]);
  const [filteredData, setFilteredData] = useState<History[]>([]);
  const [searchText, setSearchText] = useState('');

  const handleBackup = async () => {
    setLoading(true);
    try {
      const result = await backupHistory();
      setData(result);
      setFilteredData(result);
      message.success(`备份成功！共 ${result.length} 条历史记录`);
    } catch (error) {
      message.error(`备份失败: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleClear = async () => {
    Modal.confirm({
      title: '确认清空',
      content: '此操作将清空您的所有历史记录，是否继续？',
      okText: '确认',
      okType: 'danger',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const result = await clearHistory();
          message.success(result.message);
        } catch (error) {
          message.error(`清空失败: ${error}`);
        } finally {
          setLoading(false);
        }
      },
    });
  };

  const handleExport = async () => {
    if (data.length === 0) {
      message.warning('没有数据可导出');
      return;
    }

    try {
      const filePath = await saveJsonFile();
      if (filePath) {
        await exportHistory(data, filePath);
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
        const imported = await importHistory(filePath);
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
        item.title.toLowerCase().includes(value.toLowerCase())
      );
      setFilteredData(filtered);
    }
  };

  const columns = [
    {
      title: '封面',
      dataIndex: 'cover',
      key: 'cover',
      width: 100,
      render: (cover?: string) => (
        <Avatar
          shape="square"
          size={64}
          src={cover}
          icon={<PlayCircleOutlined />}
        />
      ),
    },
    {
      title: '标题',
      dataIndex: 'title',
      key: 'title',
      ellipsis: true,
    },
    {
      title: 'UP主',
      dataIndex: 'authorName',
      key: 'authorName',
    },
    {
      title: '观看时间',
      dataIndex: 'viewAt',
      key: 'viewAt',
      render: (time?: number) =>
        time ? new Date(time * 1000).toLocaleString('zh-CN') : '-',
    },
    {
      title: '进度',
      key: 'progress',
      render: (_: any, record: History) => {
        if (record.progress && record.duration) {
          const percent = Math.round((record.progress / record.duration) * 100);
          return `${percent}%`;
        }
        return '-';
      },
    },
  ];

  return (
    <div>
      <Card
        title="历史记录"
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
            <Button
              danger
              icon={<DeleteOutlined />}
              onClick={handleClear}
              loading={loading}
            >
              清空
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
              <Statistic title="历史记录总数" value={data.length} />
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
          placeholder="搜索标题"
          prefix={<SearchOutlined />}
          value={searchText}
          onChange={(e) => handleSearch(e.target.value)}
          style={{ marginBottom: 16 }}
          allowClear
        />

        <Table
          dataSource={filteredData}
          columns={columns}
          rowKey={(record) => `${record.kid || Math.random()}`}
          loading={loading}
          pagination={{ pageSize: 20 }}
        />
      </Card>
    </div>
  );
}
