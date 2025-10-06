import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button, Card, Typography, Space } from "antd";
import "./App.css";

const { Title, Paragraph } = Typography;

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [version, setVersion] = useState("");
  const [name, setName] = useState("用户");

  async function greet() {
    // 调用Rust后端命令
    const message = await invoke<string>("greet", { name });
    setGreetMsg(message);
  }

  async function getVersion() {
    const ver = await invoke<string>("get_version");
    setVersion(ver);
  }

  return (
    <div className="container">
      <Title level={1}>哔哩哔哩账号备份</Title>

      <Card title="欢迎使用" style={{ width: 600, margin: "20px auto" }}>
        <Space direction="vertical" style={{ width: "100%" }}>
          <Paragraph>
            这是一个使用 Tauri + Rust + React 构建的B站账号备份工具。
          </Paragraph>

          <div>
            <input
              type="text"
              placeholder="输入你的名字..."
              value={name}
              onChange={(e) => setName(e.target.value)}
              style={{
                padding: "8px 12px",
                marginRight: "8px",
                border: "1px solid #d9d9d9",
                borderRadius: "4px",
              }}
            />
            <Button type="primary" onClick={greet}>
              打招呼
            </Button>
          </div>

          {greetMsg && <Paragraph type="success">{greetMsg}</Paragraph>}

          <div>
            <Button onClick={getVersion}>获取版本信息</Button>
            {version && <Paragraph>当前版本: v{version}</Paragraph>}
          </div>
        </Space>
      </Card>
    </div>
  );
}

export default App;
