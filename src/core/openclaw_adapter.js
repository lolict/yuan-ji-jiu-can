/**
 * ================================================
 * Trae 接入 OpenClaw 完整方案
 * ================================================
 */

const http = require('http');
const https = require('https');

class OpenClawClient {
  constructor(options = {}) {
    this.host = options.host || '127.0.0.1';
    this.port = options.port || 18789;
    this.token = options.token || 'test123456';
  }

  // HTTP POST 请求
  _request(path, method = 'POST', body = {}) {
    return new Promise((resolve, reject) => {
      const options = {
        hostname: this.host,
        port: this.port,
        path: path,
        method: method,
        headers: {
          'Authorization': 'Bearer ' + this.token,
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        }
      };

      const req = http.request(options, (res) => {
        let data = '';
        res.on('data', (chunk) => { data += chunk; });
        res.on('end', () => {
          try {
            resolve(JSON.parse(data));
          } catch (e) {
            resolve({ ok: false, error: data });
          }
        });
      });

      req.on('error', (e) => reject(e));
      if (body) {
        req.write(JSON.stringify(body));
      }
      req.end();
    });
  }

  // 工具调用
  async invokeTool(toolName, args = {}) {
    return this._request('/tools/invoke', 'POST', {
      tool: toolName,
      action: 'json',
      args: args
    });
  }

  // 获取会话列表
  async getSessions() {
    return this.invokeTool('sessions_list');
  }

  // 获取状态
  async getStatus() {
    return this.invokeTool('gateway_status');
  }

  // 创建会话
  async createSession(sessionKey) {
    return this.invokeTool('sessions_spawn', { sessionKey });
  }

  // 发送消息
  async sendMessage(sessionKey, message) {
    return this.invokeTool('sessions_send', {
      sessionKey,
      message: {
        type: 'text',
        text: message
      }
    });
  }

  // 启动 Gateway
  static startGateway() {
    const { exec } = require('child_process');
    return new Promise((resolve) => {
      const proc = exec('openclaw gateway --allow-unconfigured', (error) => {
        if (error) console.error('Gateway error:', error);
      });
      proc.stdout.on('data', (data) => {
        console.log('[Gateway]', data.toString());
        if (data.includes('ready')) resolve(proc);
      });
      proc.stderr.on('data', (data) => console.error('[Gateway]', data.toString()));
    });
  }
}

// 使用示例
async function main() {
  console.log('OpenClaw 接入测试');
  console.log('='.repeat(60));

  const client = new OpenClawClient();

  // 获取会话列表
  const sessions = await client.getSessions();
  console.log('会话列表:', JSON.stringify(sessions, null, 2));

  // 创建会话并发送消息
  const sessionKey = 'agent:dev:xiaolongxia';
  console.log('\n创建会话:', sessionKey);
  
  try {
    const spawnResult = await client.createSession(sessionKey);
    console.log('创建结果:', spawnResult);

    const message = '帮我接入小龙虾，我手机本地的小龙虾开发环境，我这小龙虾开发环境写在 termux 这里面干我处理不了，无法让它正常使用，一直卡我。';
    console.log('\n发送消息:', message);
    
    const sendResult = await client.sendMessage(sessionKey, message);
    console.log('发送结果:', sendResult);
  } catch (e) {
    console.error('操作失败:', e.message);
  }

  console.log('\n' + '='.repeat(60));
}

// 导出
module.exports = { OpenClawClient };

// 执行
if (require.main === module) {
  main();
}
