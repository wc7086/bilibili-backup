/**
 * 关注管理页面
 *
 * 支持备份、还原、清空关注列表
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
  Form,
  InputNumber,
  Switch,
} from 'antd';
import {
  DownloadOutlined,
  UploadOutlined,
  DeleteOutlined,
  ExportOutlined,
  ImportOutlined,
  SearchOutlined,
  UserOutlined,
} from '@ant-design/icons';
import type { Relation, RestoreOptions, RestoreResult } from '../types/api';
import {
  backupFollowing,
  restoreFollowing,
  clearFollowing,
} from '../utils/api';
import { selectJsonFile, saveJsonFile } from '../utils/api';
import { writeTextFile, readTextFile } from '@tauri-apps/api/fs';

export default function Following() {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<Relation[]>([]);
  const [filteredData, setFilteredData] = useState<Relation[]>([]);
  const [searchText, setSearchText] = useState('');
  const [restoreResult, setRestoreResult] = useState<RestoreResult | null>(null);

  const handleBackup = async () => {
    setLoading(true);
    try {
      const result = await backupFollowing();
      setData(result);
      setFilteredData(result);
      message.success(`备份成功！共 ${result.length} 个关注`);
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
      title: '还原关注列表',
      content: (
        <Form
          layout="vertical"
          initialValues={{
            create_missing_tags: true,
            continue_on_error: true,
            batch_size: 10,
            delay_ms: 1000,
          }}
        >
          <Form.Item
            label="创建缺失的分组"
            name="create_missing_tags"
            valuePropName="checked"
          >
            <Switch />
          </Form.Item>
          <Form.Item
            label="遇到错误继续执行"
            name="continue_on_error"
            valuePropName="checked"
          >
            <Switch />
          </Form.Item>
          <Form.Item label="批量大小" name="batch_size">
            <InputNumber min={1} max={50} style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item label="延迟时间(毫秒)" name="delay_ms">
            <InputNumber min={0} max={5000} style={{ width: '100%' }} />
          </Form.Item>
        </Form>
      ),
      okText: '开始还原',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const options: RestoreOptions = {
            create_missing_tags: true,
            continue_on_error: true,
            batch_size: 10,
            delay_ms: 1000,
          };
          const result = await restoreFollowing(data, options);
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
      content: '此操作将清空您的所有关注，是否继续？',
      okText: '确认',
      okType: 'danger',
      cancelText: '取消',
      onOk: async () => {
        setLoading(true);
        try {
          const result = await clearFollowing();
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
        const imported = JSON.parse(content) as Relation[];
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
      render: (face: string) => (
        <Avatar src={face} icon={<UserOutlined />} />
      ),
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
      title: '特别关注',
      dataIndex: 'special',
      key: 'special',
      render: (special: number) => (special === 1 ? <Tag color="red">是</Tag> : '-'),
    },
    {
      title: '分组',
      dataIndex: 'tag',
      key: 'tag',
      render: (tags: number[]) =>
        tags && tags.length > 0 ? tags.join(', ') : '默认分组',
    },
  ];

  return (
    <div>
      <Card
        title="关注管理"
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
          <Col span={8}>
            <Card>
              <Statistic title="关注总数" value={data.length} />
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic
                title="特别关注"
                value={data.filter((d) => d.special === 1).length}
              />
            </Card>
          </Col>
          <Col span={8}>
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
