/**
 * 登录页面
 *
 * 支持二维码扫码登录和Cookie登录
 */

import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card, Spin, message, Tabs, Input, Button, Space, Typography } from 'antd';
import { QRCodeSVG } from 'qrcode.react';
import { AuthAPI, LoginStatus, QRCodeLogin } from '../types/auth';

const { TabPane } = Tabs;
const { TextArea } = Input;
const { Title, Paragraph } = Typography;

export default function Login() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [qrUrl, setQrUrl] = useState<string>('');
  const [qrStatus, setQrStatus] = useState<string>('请扫描二维码登录');
  const [cookie, setCookie] = useState<string>('');
  const [qrLogin] = useState(() => new QRCodeLogin());

  useEffect(() => {
    checkLoginStatus();
  }, []);

  const checkLoginStatus = async () => {
    try {
      const user = await AuthAPI.getCurrentUser();
      if (user) {
        message.success('已登录');
        navigate('/dashboard');
      }
    } catch (error) {
      console.log('未登录，显示登录页面');
    }
  };

  const handleQRCodeLogin = async () => {
    setLoading(true);
    try {
      const qrcode = await qrLogin.start((status) => {
        switch (status.code) {
          case LoginStatus.Success:
            message.success('登录成功！');
            qrLogin.stop();
            navigate('/dashboard');
            break;
          case LoginStatus.QRNotScanned:
            setQrStatus('等待扫描...');
            break;
          case LoginStatus.QRScanned:
            setQrStatus('已扫描，请在手机上确认登录');
            break;
          case LoginStatus.QRExpired:
            message.warning('二维码已失效，请重新获取');
            setQrStatus('二维码已失效');
            qrLogin.stop();
            setLoading(false);
            break;
          default:
            setQrStatus(status.message);
        }
      });

      setQrUrl(qrcode.url);
      setQrStatus('请使用B站APP扫描二维码');
    } catch (error) {
      message.error(`生成二维码失败: ${error}`);
      setLoading(false);
    }
  };

  const handleCookieLogin = async () => {
    if (!cookie.trim()) {
      message.warning('请输入Cookie');
      return;
    }

    setLoading(true);
    try {
      await AuthAPI.loginWithCookie(cookie);
      message.success('登录成功！');
      navigate('/dashboard');
    } catch (error) {
      message.error(`登录失败: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        minHeight: '100vh',
        background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      }}
    >
      <Card
        style={{
          width: 500,
          boxShadow: '0 4px 12px rgba(0,0,0,0.15)',
        }}
      >
        <div style={{ textAlign: 'center', marginBottom: 24 }}>
          <Title level={2}>哔哩哔哩账号备份</Title>
          <Paragraph type="secondary">请先登录您的B站账号</Paragraph>
        </div>

        <Tabs defaultActiveKey="qrcode">
          <TabPane tab="扫码登录" key="qrcode">
            <div
              style={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                padding: '20px 0',
              }}
            >
              {qrUrl ? (
                <div style={{ textAlign: 'center' }}>
                  <QRCodeSVG value={qrUrl} size={200} />
                  <Paragraph style={{ marginTop: 16 }}>{qrStatus}</Paragraph>
                  <Button
                    onClick={() => {
                      qrLogin.stop();
                      setQrUrl('');
                      handleQRCodeLogin();
                    }}
                  >
                    刷新二维码
                  </Button>
                </div>
              ) : (
                <Space direction="vertical" align="center">
                  {loading ? (
                    <Spin tip="加载中..." />
                  ) : (
                    <>
                      <Paragraph>点击下方按钮获取登录二维码</Paragraph>
                      <Button type="primary" size="large" onClick={handleQRCodeLogin}>
                        获取二维码
                      </Button>
                    </>
                  )}
                </Space>
              )}
            </div>
          </TabPane>

          <TabPane tab="Cookie登录" key="cookie">
            <Space direction="vertical" style={{ width: '100%' }} size="large">
              <div>
                <Paragraph>
                  请从浏览器中复制B站的Cookie并粘贴到下方：
                </Paragraph>
                <TextArea
                  rows={6}
                  placeholder="粘贴Cookie内容..."
                  value={cookie}
                  onChange={(e) => setCookie(e.target.value)}
                />
              </div>
              <Button
                type="primary"
                block
                size="large"
                loading={loading}
                onClick={handleCookieLogin}
              >
                登录
              </Button>
            </Space>
          </TabPane>
        </Tabs>
      </Card>
    </div>
  );
}
