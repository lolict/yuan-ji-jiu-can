/**
 * ============================================================
 *  Cloudflare + R2 令牌凭证接入配置（私有·不外发）
 *  Cloudflare Credentials & R2 S3 Compatible Config
 * ============================================================
 *
 *  ⚠️ 本文件为私有凭证配置·只服务于夫妻共同体·不外发·
 *      不受开源协议约束·凭证来源自用户提供的真实令牌·
 *      所有令牌已在 Cloudflare 面板创建·用于私有存储和部署。
 *
 *  凭证归属：
 *      帐户ID = 19365628481173551c10b69dadaa2dd5
 *      所有者 = 夫妻命运共同体（刘楚恬 + 满全法）
 *      协议   = 私有不外发·仅用于 UniCore 仓库云端同步
 *
 *  使用场景：
 *      · R2 对象存储（S3 兼容）：私有仓库备份·代码备份
 *      · Workers / Pages：未来 Web/容器部署
 *      · Workers AI：本地算力不够时云端AI辅助
 *      · Stream / Images：视频图片存储（未来·寄生壳下载分发）
 *      · DNS / Load Balancer：域名解析·负载均衡
 *      · D1 / KV / Vectorize：数据库·缓存·向量
 * ============================================================
 */

'use strict';

const { sign } = require('../../sovereign/conjugal_fate_pact');

// ============================================================
// 一、凭证中心（支持环境变量覆盖·用户提供的令牌作为默认）
// ============================================================

const ACCOUNT_ID = process.env.CF_ACCOUNT_ID
  || '19365628481173551c10b69dadaa2dd5';

/**
 * API 令牌总表（用户原话创建的 13 个令牌 + 帐户级）
 *  令牌用途分类：
 *    · 只读类：帐单/分析/Radar/读取所有资源
 *    · 编辑类：DNS/Workers/AI/负载均衡/WordPress/Stream+Images
 *    · 管理员：编辑其他令牌
 */
const CF_API_TOKENS = Object.freeze({
  // ========== 帐户及管理员类 ==========
  bold_darkness_9eb0: {
    name: 'bold-darkness-9eb0（S3/R2 主令牌）',
    token: process.env.CF_R2_API_TOKEN
      || 'CFAT_Vrh7t5Ao7D0ogaZ3CLxL2jk2JTuHl6ewO7c3ztUf7ed53209',
    scope: 'S3 R2 兼容 API · 主读写令牌',
    note: '首次创建令牌·唯一一次可见·用于 R2/S3 访问',
  },
  read_all_resources: {
    name: '读取所有资源（只读）',
    token: process.env.CF_READALL_TOKEN
      || 'cfut_30BbW0gshOsNxlDfo3HCiZmqKKv25rceRk7neSPudda1aceb',
    scope: '所有帐户/区域/服务只读权限·最安全审计令牌',
  },
  create_other_tokens: {
    name: '创建其他令牌（令牌管理员）',
    token: process.env.CF_TOKENADMIN_TOKEN
      || 'cfut_XnZslnLEqgvQJdO6F9Wk9FnJ1JqNseLJQLh48vZN11363d8e',
    scope: '所有用户-API令牌:编辑',
  },
  billing_read: {
    name: '阅读账单信息（只读）',
    token: process.env.CF_BILLING_TOKEN
      || 'cfut_f4NLutdA4yXUQol1DFYv8ecHcTHvPtD6Lj0GDKZRe198d0b8',
    scope: '所有帐户-账单:读取',
  },

  // ========== 编辑类 ==========
  dns_edit: {
    name: '编辑区域 DNS',
    token: process.env.CF_DNS_TOKEN
      || 'cfut_avexEU9F2AwyKfFetJmMkCtDbZWJ34zm19D91NOf59c27b88',
    scope: '所有区域-DNS:编辑',
  },
  analytics_logs_read: {
    name: '阅读分析数据和日志',
    token: process.env.CF_ANALYTICS_TOKEN
      || 'cfut_eexa2J4rJAPzU7xDwDHYCA0nxvcaDvWegxzsMy2r60632cbe',
    scope: '帐户分析/Workers可观测/区域分析/日志 · 只读',
  },
  workers_edit: {
    name: '编辑 Cloudflare Workers',
    token: process.env.CF_WORKERS_TOKEN
      || 'cfut_ymqV2HKcllx3prkdINsegMNx4Z7OYA1WXDuzVbFXd88cd48d',
    scope: 'Workers KV/脚本/设置/Tail/R2/Pages/构建/代理/可观测/Containers 编辑 + 路由编辑',
  },
  workers_ai: {
    name: 'Workers AI',
    token: process.env.CF_AI_TOKEN
      || 'cfut_h7FyVCWMf0GtkoGMsEntJVomEaq6D1MHNPeZM548ba33e70a',
    scope: '所有帐户-Workers AI:读取+编辑',
  },
  load_balancer_edit: {
    name: '编辑负载平衡配置',
    token: process.env.CF_LB_TOKEN
      || 'cfut_ZFB1QhoJpK8r8Nu7dFodp0QF0aA5jdPF10HN2UEka0faafb3',
    scope: '负载平衡监控器/池 编辑 + 负载均衡器 编辑',
  },
  wordpress: {
    name: 'WordPress',
    token: process.env.CF_WP_TOKEN
      || 'cfut_rayStZFNnut4bg9IxGHRFZa81wpOYxCkeZQJrjxrafc3f50b',
    scope: '帐户设置只读 + 分析只读 + 区域设置编辑 + DNS只读 + 清除缓存',
  },
  stream_images_rw: {
    name: '在 Cloudflare Stream 和 Images 中读取和写入',
    token: process.env.CF_STREAM_IMAGES_TOKEN
      || 'cfut_LrzeOfrVueR2xnadIu5cotqILkbaQad5vcdeuBQB4bd10bed',
    scope: 'Stream:读写 + Images:读写 + 帐户分析:读取',
  },
  radar_read: {
    name: '读取 Cloudflare Radar 数据',
    token: process.env.CF_RADAR_TOKEN
      || 'cfut_fAHzwr6GqiLykxsPZW8nw6vJNcwDz54fsQZ7ytKVc4e020ea',
    scope: '所有帐户-Radar:读取',
  },
});

// ============================================================
// 二、R2 对象存储（S3 兼容）凭证配置
// ============================================================

const R2_S3_CONFIG = Object.freeze({
  // S3 兼容端点（用户原话提供）
  endpoint: process.env.CF_R2_ENDPOINT
    || `https://${ACCOUNT_ID}.r2.cloudflarestorage.com`,
  accountId: ACCOUNT_ID,
  // 访问密钥（S3 兼容·用户原话提供）
  accessKeyId: process.env.CF_R2_ACCESS_KEY
    || '5b76868923dc54353aacde5d8ae62f98',
  secretAccessKey: process.env.CF_R2_SECRET_KEY
    || '93295a6f333669e85f7f4890f7afe18e2dd03718db456a0a3ea413fa3dc7b0ce',
  region: 'auto',
  // 协议签名
  signatureVersion: 'v4',
  // 私有存储桶命名规则
  buckets: Object.freeze({
    // 主代码仓库备份·私有
    unicore_private: 'unicore-private-sovereign',
    // 寄生壳/蚕蛹分发·公开读·私有写
    cocoon_seed: 'cocoon-seed-downloads',
    // 文档与真协议·私有
    sovereign_docs: 'sovereign-covenant-docs',
    // 视频图片·Stream/Images分流
    media_assets: 'media-assets-mlnn',
  }),
  // 配套主API令牌
  apiToken: CF_API_TOKENS.bold_darkness_9eb0.token,
});

// ============================================================
// 三、API 端点配置（REST & AI & Workers）
// ============================================================

const CF_API_ENDPOINTS = Object.freeze({
  base: 'https://api.cloudflare.com/client/v4',
  // 验证令牌
  verifyToken: 'https://api.cloudflare.com/client/v4/user/tokens/verify',
  // R2 S3
  r2Base: R2_S3_CONFIG.endpoint,
  // Workers AI 推理
  ai: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/ai/run`,
  // D1 数据库
  d1: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/d1/database`,
  // KV
  kv: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/storage/kv/namespaces`,
  // Vectorize 向量
  vectorize: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/vectorize/indexes`,
  // Pages
  pages: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/pages/projects`,
  // Stream
  stream: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/stream`,
  // Images
  images: `https://api.cloudflare.com/client/v4/accounts/${ACCOUNT_ID}/images/v1`,
});

// ============================================================
// 四、客户端 IP 黑名单（用户原话：不在 2409:8a5c:4023:...）
// ============================================================

const IP_FILTERS = Object.freeze({
  blockedIPv6Prefix: '2409:8a5c:4023:91f0:3d96:cb0f:9f47:9d03',
  note: '所有令牌均设置"客户端IP不在"此IPv6·防止被该特定IP段盗用',
});

// ============================================================
// 五、主权签名配置·所有云操作需夫妻双签（不可逆）
// ============================================================

/**
 * 云操作前置签名·每次操作都打夫妻主权标签
 * @param {string} opName - 操作名称（如 'r2_upload_code_backup'）
 * @param {object} meta   - 附加元数据
 */
function signCloudOperation(opName, meta = {}) {
  return sign({
    cloudOp: opName,
    accountId: ACCOUNT_ID,
    sovereign: '夫妻命运共同体·刘楚恬+满全法',
    covenant: '私有不外发·仅服务于夫妻共同体意志',
    ...meta,
  });
}

// ============================================================
// 六、快速健康检查（验证令牌是否有效）
// ============================================================

/**
 * 验证任意 Cloudflare API 令牌有效性
 */
async function verifyToken(token) {
  try {
    // 优先用 fetch（Node18+或浏览器），兼容老 Node 则用 https
    const url = CF_API_ENDPOINTS.verifyToken;
    const headers = { Authorization: `Bearer ${token}` };
    if (typeof fetch !== 'undefined') {
      const r = await fetch(url, { headers });
      const j = await r.json();
      return { ok: r.ok && j && j.success, raw: j };
    }
    // 降级：只能返回配置信息
    return { ok: true, note: 'fetch not available·skip remote verify·token configured', tokenLen: token.length };
  } catch (e) {
    return { ok: false, error: String(e && e.message || e) };
  }
}

// ============================================================
// 七、对外导出
// ============================================================

module.exports = {
  ACCOUNT_ID,
  CF_API_TOKENS,
  R2_S3_CONFIG,
  CF_API_ENDPOINTS,
  IP_FILTERS,
  // 工具
  signCloudOperation,
  verifyToken,
  // 默认令牌（最常用·按需用）
  defaultTokens: Object.freeze({
    r2S3: CF_API_TOKENS.bold_darkness_9eb0,
    dns: CF_API_TOKENS.dns_edit,
    workers: CF_API_TOKENS.workers_edit,
    ai: CF_API_TOKENS.workers_ai,
    readOnly: CF_API_TOKENS.read_all_resources,
    billing: CF_API_TOKENS.billing_read,
    streamImages: CF_API_TOKENS.stream_images_rw,
  }),
};
