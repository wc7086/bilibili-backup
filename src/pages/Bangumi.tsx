/**
 * 追番管理页面
 *
 * 支持备份、还原、清空、导入导出追番列表
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
  Tag,
  Progress,
  Select,
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
import type { Bangumi, RestoreResult } from '../types/api';
import {
  backupBangumi,
  restoreBangumi,
  clearBangumi,
  exportBangumi,
  importBangumi,
  selectJsonFile,
  saveJsonFile,
} from '../utils/api';

const { Option } = Select;

export default function BangumiPage() {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<Bangumi[]>([]);
  const [filteredData, setFilteredData] = useState<Bangumi[]>([]);
  const [searchText, setSearchText] = useState('');
  const [type, setType] = useState<number>(1);
  const [restoreResult, setRestoreResult] = useState<RestoreResult | null>(null);

  const handleBackup = async () => {
    setLoading(true);
    try {
      const result = await backupBangumi(type);
      setData(result);
      setFilteredData(result);
      message.success(`备份成功！共 ${result.length} 个追番`);
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
      title: '还原追番列表',
      content: `确认还原 ${data.length} 个追番吗？`,
      okText: '开始还原',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const result = await restoreBangumi(data);
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
      content: `此操作将清空您的所有${type === 1 ? '追番' : '追剧'}，是否继续？`,
      okText: '确认',
      okType: 'danger',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const result = await clearBangumi(type);
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
        await exportBangumi(data, filePath);
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
        const imported = await importBangumi(filePath);
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
      render: (cover: string) => (
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
      title: '类型',
      dataIndex: 'seasonTypeName',
      key: 'seasonTypeName',
      render: (typeName?: string) => (typeName ? <Tag>{typeName}</Tag> : '-'),
    },
    {
      title: '集数',
      dataIndex: 'totalCount',
      key: 'totalCount',
      render: (count?: number) => (count ? `${count} 集` : '-'),
    },
    {
      title: '状态',
      dataIndex: 'badge',
      key: 'badge',
      render: (badge?: string) => (badge ? <Tag color="blue">{badge}</Tag> : '-'),
    },
    {
      title: 'Season ID',
      dataIndex: 'seasonId',
      key: 'seasonId',
    },
  ];

  return (
    <div>
      <Card
        title={
          <Space>
            <span>追番管理</span>
            <Select value={type} onChange={setType} style={{ width: 120 }}>
              <Option value={1}>追番</Option>
              <Option value={2}>追剧</Option>
            </Select>
          </Space>
        }
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
              <Statistic
                title={type === 1 ? '追番总数' : '追剧总数'}
                value={data.length}
              />
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
          rowKey="seasonId"
          loading={loading}
          pagination={{ pageSize: 20 }}
        />
      </Card>
    </div>
  );
}
