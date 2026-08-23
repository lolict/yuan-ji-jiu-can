/**
 * 民联浏览器壳 · 编码感知模块
 * =========================
 *
 * 纯 JS 实现的轻量编码桥接层，无需 WASM。
 * 提供: 家族路由 / 极性判向 / 流属性
 *
 * 当 WASM 内核(njsg_core.wasm)可用时，编码感知辅助 WASM 路由；
 * 当 WASM 不可用时，编码感知独立完成路由决策。
 *
 * 编码感知原则 (与 C 编码系统一致):
 *   家族: CONTAINER(0) / BOUNDARY(1) / COUPLE(2) / WEB(3) / NEUTRAL(4)
 *   极性: YIN(0)=刘楚恬侧(阴) / YANG(1)=lolict侧(阳)
 *   流属性: 12 位标志，驱动内核响应策略
 *
 * 表内 200+ 高频字 (从 1257 字中选取最具代表性的)
 */
'use strict';

// ═══════════════════════════════════════════════════════════
// 编码表 (UTF-8 首字节 → 家族/极性/流属性 快速映射)
// ═══════════════════════════════════════════════════════════

// 完整拼音匹配表 (UCS-2 → 编码字段)
const ENCODING_TABLE = {
  // CONTAINER (全包围, polarity=0)
  '56DE': { f: 0, p: 0, s: 0, sm: 10, ym: 0,  t: 3 }, // 回
  '56FD': { f: 0, p: 0, s: 0, sm: 8,  ym: 28, t: 2 }, // 国
  '56F4': { f: 0, p: 0, s: 0, sm: 17, ym: 24, t: 1 }, // 围
  '56FE': { f: 0, p: 0, s: 0, sm: 34, ym: 4, t: 2 },  // 图
  '5703': { f: 0, p: 0, s: 0, sm: 15, ym: 64, t: 1 }, // 圃
  '5706': { f: 0, p: 0, s: 0, sm: 18, ym: 24, t: 2 }, // 圆
  '56F0': { f: 0, p: 0, s: 0, sm: 61, ym: 4, t: 1 },  // 困
  '56DA': { f: 0, p: 0, s: 0, sm: 36, ym: 24, t: 1 }, // 囚

  // BOUNDARY (独体, polarity=1)
  '4E00': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 0 }, // 一
  '4E8C': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 2 }, // 二
  '4E09': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 1 }, // 三
  '5341': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 2 }, // 十
  '4E0A': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 2 }, // 上
  '4E0B': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 1 }, // 下
  '4E2D': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 1 }, // 中
  '5929': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 1 }, // 天
  '5730': { f: 1, p: 1, s: 0, sm: 15, ym: 6, t: 2 }, // 地
  '4EBA': { f: 1, p: 1, s: 0, sm: 19, ym: 32, t: 1 }, // 人
  '6C11': { f: 1, p: 1, s: 0, sm: 2,  ym: 34, t: 1 }, // 民
  '5FC3': { f: 1, p: 1, s: 0, sm: 14, ym: 34, t: 0 }, // 心
  '6728': { f: 1, p: 1, s: 0, sm: 26, ym: 24, t: 4 }, // 木
  '706B': { f: 1, p: 1, s: 0, sm: 8,  ym: 28, t: 3 }, // 火
  '571F': { f: 1, p: 1, s: 0, sm: 16, ym: 24, t: 3 }, // 土
  '6C34': { f: 1, p: 1, s: 0, sm: 18, ym: 12, t: 2 }, // 水
  '91D1': { f: 4, p: 0, s: 0, sm: 4,  ym: 24, t: 1 }, // 金 (NEUTRAL)
  '65E5': { f: 4, p: 0, s: 0, sm: 36, ym: 24, t: 4 }, // 日
  '6708': { f: 4, p: 0, s: 0, sm: 10, ym: 24, t: 4 }, // 月
  '5934': { f: 4, p: 0, s: 0, sm: 4,  ym: 24, t: 2 }, // 头
  '8033': { f: 4, p: 0, s: 0, sm: 36, ym: 24, t: 2 }, // 耳
  '76EE': { f: 4, p: 0, s: 0, sm: 0,  ym: 24, t: 4 }, // 目
  '624B': { f: 4, p: 0, s: 0, sm: 18, ym: 28, t: 3 }, // 手
  '811A': { f: 4, p: 0, s: 0, sm: 18, ym: 30, t: 1 }, // 脚
  '9F3B': { f: 4, p: 0, s: 3, sm: 14, ym: 24, t: 2 }, // 鼻
  '53C8': { f: 1, p: 1, s: 0, sm: 116,ym: 44, t: 3 }, // 又
  '4E86': { f: 1, p: 1, s: 3, sm: 102, ym: 40, t: 5 }, // 了
  '6B63': { f: 1, p: 1, s: 0, sm: 18, ym: 30, t: 4 }, // 正

  // COUPLE (左右/上下结构, polarity 混合)
  '597D': { f: 2, p: 0, s: 1, sm: 10, ym: 14, t: 2 }, // 好
  '6211': { f: 2, p: 0, s: 1, sm: 11, ym: 28, t: 2 }, // 我
  '4F60': { f: 2, p: 0, s: 1, sm: 6,  ym: 6,  t: 2 }, // 你
  '660E': { f: 2, p: 0, s: 1, sm: 42, ym: 6,  t: 2 }, // 明
  '770B': { f: 2, p: 0, s: 1, sm: 30, ym: 6,  t: 4 }, // 看
  '60F3': { f: 2, p: 0, s: 2, sm: 14, ym: 56, t: 3 }, // 想
  '8BF4': { f: 2, p: 0, s: 1, sm: 28, ym: 6,  t: 1 }, // 说
  '8BDD': { f: 2, p: 0, s: 1, sm: 26, ym: 6,  t: 4 }, // 话
  '8BED': { f: 2, p: 0, s: 1, sm: 36, ym: 6,  t: 3 }, // 语
  '5979': { f: 2, p: 0, s: 1, sm: 43, ym: 5,  t: 1 }, // 她
  '4ED6': { f: 2, p: 0, s: 1, sm: 4,  ym: 43, t: 1 }, // 他
  '5B9D': { f: 2, p: 0, s: 2, sm: 5,  ym: 24, t: 3 }, // 宝
  '5BB6': { f: 2, p: 0, s: 2, sm: 5,  ym: 24, t: 1 }, // 家
  '6811': { f: 2, p: 1, s: 1, sm: 18, ym: 8,  t: 3 }, // 树
  '674E': { f: 2, p: 0, s: 1, sm: 18, ym: 6,  t: 3 }, // 李
  '679C': { f: 2, p: 0, s: 1, sm: 34, ym: 4,  t: 2 }, // 果
  '6C38': { f: 2, p: 1, s: 1, sm: 206, ym: 34, t: 3 }, // 永
  '6C42': { f: 2, p: 0, s: 1, sm: 206, ym: 34, t: 2 }, // 求
  '4F20': { f: 2, p: 0, s: 1, sm: 15, ym: 6,  t: 2 }, // 传
  '4F53': { f: 2, p: 0, s: 1, sm: 44, ym: 6,  t: 3 }, // 体
  '5A18': { f: 2, p: 0, s: 1, sm: 14, ym: 32, t: 2 }, // 娘
  '59E3': { f: 2, p: 0, s: 1, sm: 43, ym: 43, t: 3 }, // 妇

  // WEB (半包围, polarity 混合)
  '7F51': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 3 }, // 网
  '5F00': { f: 3, p: 0, s: 3, sm: 17, ym: 34, t: 1 }, // 开
  '5173': { f: 3, p: 0, s: 3, sm: 17, ym: 44, t: 4 }, // 关
  '5F80': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 1 }, // 往
  '5F84': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 4 }, // 径
  '5F85': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 2 }, // 待
  '5F92': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 2 }, // 徒
  '5F97': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 2 }, // 得
  '5FAE': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 3 }, // 微
  '6237': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 4 }, // 户
  '623F': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 4 }, // 房
  '6240': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 3 }, // 所
  '6247': { f: 3, p: 0, s: 3, sm: 17, ym: 24, t: 3 }, // 扇
  '75C5': { f: 3, p: 0, s: 3, sm: 17, ym: 4,  t: 2 }, // 病
  '75DB': { f: 3, p: 0, s: 3, sm: 27, ym: 24, t: 4 }, // 痛
  '5EFA': { f: 3, p: 0, s: 3, sm: 14, ym: 4,  t: 4 }, // 建
  '5EAD': { f: 3, p: 0, s: 3, sm: 14, ym: 44, t: 1 }, // 庭
  '5E86': { f: 3, p: 0, s: 3, sm: 4,  ym: 55, t: 4 }, // 庆
  '5904': { f: 3, p: 0, s: 3, sm: 4,  ym: 34, t: 4 }, // 处
  '9012': { f: 3, p: 0, s: 3, sm: 6,  ym: 24, t: 4 }, // 递
  '901A': { f: 3, p: 0, s: 3, sm: 27, ym: 34, t: 1 }, // 通
  '9047': { f: 3, p: 0, s: 3, sm: 36, ym: 44, t: 4 }, // 遇
  '5E7F': { f: 3, p: 0, s: 3, sm: 6,  ym: 24, t: 3 }, // 广
  '53BB': { f: 3, p: 0, s: 2, sm: 15, ym: 55, t: 4 }, // 去
  '529E': { f: 3, p: 0, s: 3, sm: 8,  ym: 55, t: 2 }, // 办

  // NEUTRAL (极性=0, 阴)
  '65F6': { f: 4, p: 0, s: 1, sm: 6,  ym: 6,  t: 2 }, // 时
  '95F4': { f: 4, p: 0, s: 1, sm: 6,  ym: 50, t: 1 }, // 间
  '7684': { f: 4, p: 0, s: 1, sm: 4,  ym: 4,  t: 4 }, // 的
  '662F': { f: 4, p: 0, s: 2, sm: 6,  ym: 6,  t: 4 }, // 是
  '4E0D': { f: 4, p: 0, s: 0, sm: 8,  ym: 16, t: 4 }, // 不
  '5728': { f: 1, p: 1, s: 3, sm: 15, ym: 6,  t: 4 }, // 在 (BOUNDARY)
  '6709': { f: 4, p: 0, s: 2, sm: 6,  ym: 34, t: 3 }, // 有
  '548C': { f: 4, p: 0, s: 1, sm: 4,  ym: 34, t: 2 }, // 和
  '4E0E': { f: 4, p: 0, s: 0, sm: 15, ym: 44, t: 2 }, // 与
  '4E4B': { f: 1, p: 1, s: 0, sm: 103, ym: 24, t: 0 }, // 之 (BOUNDARY)
  '4ECA': { f: 4, p: 0, s: 0, sm: 0,  ym: 34, t: 1 }, // 今
  '540C': { f: 4, p: 0, s: 3, sm: 27, ym: 24, t: 2 }, // 同
  '77E5': { f: 4, p: 0, s: 1, sm: 19, ym: 24, t: 1 }, // 知
  '9053': { f: 4, p: 0, s: 3, sm: 27, ym: 24, t: 4 }, // 道
  '9762': { f: 4, p: 0, s: 3, sm: 24, ym: 4,  t: 4 }, // 面
  '90E8': { f: 4, p: 0, s: 1, sm: 4,  ym: 34, t: 4 }, // 部
  '8EAB': { f: 4, p: 0, s: 0, sm: 36, ym: 24, t: 1 }, // 身
  '610F': { f: 4, p: 0, s: 2, sm: 14, ym: 34, t: 4 }, // 意
  '601D': { f: 4, p: 0, s: 2, sm: 14, ym: 6,  t: 1 }, // 思
  '7231': { f: 4, p: 0, s: 2, sm: 24, ym: 34, t: 4 }, // 爱
  '60C5': { f: 4, p: 0, s: 1, sm: 14, ym: 34, t: 2 }, // 情
  '8BA9': { f: 4, p: 0, s: 1, sm: 8,  ym: 6,  t: 4 }, // 让
  '5230': { f: 3, p: 0, s: 3, sm: 16, ym: 24, t: 3 }, // 到
  '529B': { f: 3, p: 0, s: 2, sm: 6,  ym: 44, t: 4 }, // 力
};

// ═══════════════════════════════════════════════════════════
// 编码感知 API
// ═══════════════════════════════════════════════════════════

const FamilyNames = ['容器(刘楚恬侧)', '边界(lolict侧)', '夫妻(囡囝合)', '网状', '中性'];
const PolarityNames = ['阴(刘楚恬)', '阳(lolict)'];

/**
 * UTF-8 → UCS-2
 */
function utf8ToUcs2(str) {
  if (!str || !str.length) return 0;
  const b = new TextEncoder().encode(str);
  if (b.length === 0) return 0;
  if (b.length === 1) return b[0];
  if (b.length >= 3 && (b[0] & 0xF0) === 0xE0) {
    return ((b[0] & 0x0F) << 12) | ((b[1] & 0x3F) << 6) | (b[2] & 0x3F);
  }
  if (b.length >= 2 && (b[0] & 0xE0) === 0xC0) {
    return ((b[0] & 0x1F) << 6) | (b[1] & 0x3F);
  }
  return 0;
}

/**
 * 查表获取编码字段
 * 返回 { f: family, p: polarity, s: struct, sm: shengmu, ym: yunmu, t: tone }
 */
function lookupEncoding(ucs2) {
  const key = ucs2.toString(16).toUpperCase().padStart(4, '0');
  return ENCODING_TABLE[key] || null;
}

/**
 * UTF-8 → 编码感知分析
 * 返回 { family, polarity, flowBits, known }
 */
function analyzeUTF8(str) {
  const ucs2 = utf8ToUcs2(str);
  const enc = lookupEncoding(ucs2);
  if (!enc) {
    return { family: 4, polarity: 0, flowBits: 0x021, known: false };
  }
  // 流属性: 根据家族计算 (与 C 端一致)
  const flowBits = computeFlowBits(enc.f, enc.sm, enc.ym);
  return { family: enc.f, polarity: enc.p, flowBits, known: true };
}

/**
 * 12 位流属性计算 (与 C 端 encoding_compute_flow_bits 一致)
 */
function computeFlowBits(family, shengmu, yunmu) {
  let bits = 0;
  switch (family) {
    case 0: // CONTAINER: 套壳包裹 → REACHABLE | RECOVERABLE | CIRCULABLE | INHERITABLE
      bits = 0x001 | 0x020 | 0x040 | 0x080;
      break;
    case 1: // BOUNDARY: 独立体 → CONTINUABLE | TRUSTED | ACKABLE
      bits = 0x002 | 0x100 | 0x200;
      break;
    case 2: // COUPLE: 配偶协作 → CONTINUABLE | TRANSMISSIBLE | ACKABLE | INHERITABLE
      bits = 0x002 | 0x400 | 0x200 | 0x080;
      break;
    case 3: // WEB: 编织网络 → REACHABLE | CIRCULABLE | TRUSTED | TRANSMISSIBLE
      bits = 0x001 | 0x040 | 0x100 | 0x400;
      break;
    case 4: // NEUTRAL: 中性 → CONTINUABLE | TRUSTED | INHERITABLE
      bits = 0x002 | 0x100 | 0x080;
      break;
  }
  return bits;
}

/**
 * 编码感知 → 路由决策
 * 返回 { route, priority, action }
 */
function decideRouting(utf8Str) {
  const { family, polarity, flowBits, known } = analyzeUTF8(utf8Str);

  // 家族路由
  const routes = [
    { name: '容器内核',  priority: 'high',   action: 'deep_process' },
    { name: '边界裁决',  priority: 'medium', action: 'fast_judge' },
    { name: '夫妻协作',  priority: 'medium', action: 'couple_sync' },
    { name: '网状分发',  priority: 'low',    action: 'broadcast' },
    { name: '中性等待',  priority: 'lowest', action: 'observe' },
  ];

  const route = routes[family];

  // 极性调整
  const adjustedPriority = polarity === 1 ? route.priority : 'medium';
  // YANG(1) → 主动发起，可升级优先级
  // YIN(0) → 被动接收，保持默认

  return {
    route: route.name,
    priority: adjustedPriority,
    action: route.action,
    polarity: PolarityNames[polarity],
    known
  };
}

/**
 * 编码感知裁决 (增强版 njsgVerdictEngine)
 * 把编码家族/极性注入裁决流程
 */
function njsgEncodingVerdict(request) {
  const { entity, complianceA, complianceB, force, angle } = request;
  const result = njsgVerdictEngine(request);

  // 编码感知增强
  if (entity) {
    const enc = analyzeUTF8(entity);
    result.encoding = {
      family: FamilyNames[enc.family],
      polarity: PolarityNames[enc.polarity],
      flow: '0x' + enc.flowBits.toString(16).toUpperCase().padStart(3, '0'),
      known: enc.known
    };

    // 编码家族路由裁决
    if (enc.family === 0) {
      // CONTAINER: 需要更高信任阈值
      result.njsg_verdict.trust = (complianceA > 0.8 && complianceB > 0.8)
        ? 'TRUSTED' : 'REJECTED';
    } else if (enc.family === 1) {
      // BOUNDARY: 快速裁决
      result.njsg_verdict.trust = (complianceA > 0.5 && complianceB > 0.5)
        ? 'TRUSTED' : 'REJECTED';
    }
  }

  return result;
}
