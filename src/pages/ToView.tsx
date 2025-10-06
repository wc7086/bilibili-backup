/**
 * 稍后再看页面
 *
 * 支持备份、还原、清空、导入导出稍后再看列表
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
  Progress,
} from 'antd';
import {
  DownloadOutlined,
  UploadOutlined,
  DeleteOutlined,
  ExportOutlined,
  ImportOutlined,
  SearchOutlined,
  PlayCircleOutlined,
} from '@ant-design/icons';
import type { ToView, RestoreResult } from '../types/api';
import {
  backupToview,
  restoreToview,
  clearToview,
  exportToview,
  importToview,
  selectJsonFile,
  saveJsonFile,
} from '../utils/api';

export default function ToViewPage() {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<ToView[]>([]);
  const [filteredData, setFilteredData] = useState<ToView[]>([]);
  const [searchText, setSearchText] = useState('');
  const [restoreResult, setRestoreResult] = useState<RestoreResult | null>(null);

  const handleBackup = async () => {
    setLoading(true);
    try {
      const result = await backupToview();
      setData(result);
      setFilteredData(result);
      message.success(`备份成功！共 ${result.length} 个视频`);
    } catch (error) {
      message.error(`备份失败: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleRestore = async () => {
    if (data.length === 0) {
      message.warning('请先备份或导入数据');
      return;
    }

    Modal.confirm({
      title: '还原稍后再看',
      content: `确认还原 ${data.length} 个视频吗？`,
      okText: '开始还原',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const result = await restoreToview(data);
          setRestoreResult(result);
          message.success(result.message);
        } catch (error) {
          message.error(`还原失败: ${error}`);
        } finally {
          setLoading(false);
        }
      },
    });
  };

  const handleClear = async () => {
    Modal.confirm({
      title: '确认清空',
      content: '此操作将清空您的所有稍后再看，是否继续？',
      okText: '确认',
      okType: 'danger',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const result = await clearToview();
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
        await exportToview(data, filePath);
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
        const imported = await importToview(filePath);
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

  const formatDuration = (seconds?: number): string => {
    if (!seconds) return '-';
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    if (h > 0) {
      return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    }
    return `${m}:${s.toString().padStart(2, '0')}`;
  };

  const columns = [
    {
      title: '封面',
      dataIndex: 'pic',
      key: 'pic',
      width: 100,
      render: (pic: string) => (
        <Avatar
          shape="square"
          size={64}
          src={pic}
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
      dataIndex: 'owner',
      key: 'owner',
      render: (owner?: { name: string }) => owner?.name || '-',
    },
    {
      title: 'BVID',
      dataIndex: 'bvid',
      key: 'bvid',
    },
    {
      title: '时长',
      dataIndex: 'duration',
      key: 'duration',
      render: (duration?: number) => formatDuration(duration),
    },
    {
      title: '添加时间',
      dataIndex: 'add_at',
      key: 'add_at',
      render: (time?: number) =>
        time ? new Date(time * 1000).toLocaleString('zh-CN') : '-',
    },
  ];

  return (
    <div>
      <Card
        title="稍后再看"
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
              icon={<UploadOutlined />}
              onClick={handleRestore}
              disabled={data.length === 0}
              loading={loading}
            >
              还原
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
              <Statistic title="视频总数" value={data.length} />
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

        {restoreResult && (
          <Card style={{ marginBottom: 16 }}>
            <Row gutter={16}>
              <Col span={8}>
                <Statistic
                  title="成功"
                  value={restoreResult.success_count}
                  valueStyle={{ color: '#3f8600' }}
                />
              </Col>
              <Col span={8}>
                <Statistic
                  title="失败"
                  value={restoreResult.failed_count}
                  valueStyle={{ color: '#cf1322' }}
                />
              </Col>
              <Col span={8}>
                <Statistic title="总数" value={restoreResult.total_count} />
              </Col>
            </Row>
            {restoreResult.success_count > 0 && (
              <Progress
                percent={Math.round(
                  (restoreResult.success_count / restoreResult.total_count) * 100
                )}
                status={restoreResult.failed_count > 0 ? 'exception' : 'success'}
                style={{ marginTop: 16 }}
              />
            )}
          </Card>
        )}

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
          rowKey="aid"
          loading={loading}
          pagination={{ pageSize: 20 }}
        />
      </Card>
    </div>
  );
}
