/**
 * FusionEngine.js — 一体化协议融合引擎
 * 
 * 理论根基:
 *   融合论 (Fusion Theory)     : 协议不是转换,是本体论层面的共存
 *   共同体论 (Community Theory): 公约数即共同体边界
 *   造字论 (Creation Theory)   : 新协议如造字,部首即进制,笔画即层级
 *   一体成型同构论 (Isomorphism): 三角见证裁定融合稳定性
 * 
 * 核心原则: 进制编码优先,代数编码交给汉字偏旁部首范畴论
 * 行脚本体: 几何即阅读方式,位置协议 = 体验协议
 * 
 * 协议本体 (含行脚/体验语义):
 *   G/GI      = 2进制   (基底·阴阳开合)
 *   HTTP/HTTPS = 16进制 (目部·行角段落,以行业为行列的段落体验)
 *   GTT/GTTX = 64进制  (⬡部·球形包裹螺旋,陈漫以星球为圈层的曲线行脚)
 *   GITT/GTIT = 64进制 (⬡部·意向螺旋)
 *   TCP/CP/FTP = 256进制(◉部·字节行列流)
 */

const PROTOCOL_ONTOLOGY = new Map();

const BASE = {
  BINARY: 2,
  HEX: 16,
  B64: 64,
  BYTE: 256,
};

const POLARITY = {
  YANG: +1,
  YIN: -1,
};

function defineProtocol(name, base, nature, polarity, layers, sovereignty = {}, experience = {}) {
  const entity = {
    name,
    base,
    nature,
    polarity,
    layers,
    sovereignty,
    experience,             // ★ 行脚/体验语义 (新增)
    founded: Date.now(),
    glyph: generateGlyph(name, base),
  };
  PROTOCOL_ONTOLOGY.set(name, entity);
  return entity;
}

function generateGlyph(name, base) {
  const radicalMap = {
    2:   '⚊',
    16:  '目',
    64:  '⬡',
    256: '◉',
  };
  const strokeMap = {
    2:   1,
    16:  4,
    64:  6,
    256: 8,
  };
  return {
    radical: radicalMap[base] || '●',
    strokes: strokeMap[base] || 4,
    body: `[${name}]`,
  };
}

// ═══════════════════════════════════════════════════════════
// 行脚几何函数 — 每种协议的"阅读方式"即"运动轨迹"
// ═══════════════════════════════════════════════════════════

/**
 * GTTX 球形包裹螺旋 (陈漫体验)
 * 数据读取: 从核心以螺旋向外展开,每一圈包裹上一层
 * 行脚 = 曲线螺旋,中心视角逐层扩大
 */
export function gttxFoldPosition(index, total, cx, cy, maxRadius) {
  const turns = Math.max(2, Math.ceil(total / 4));     // 每圈约4个点
  const goldenAngle = Math.PI * (3 - Math.sqrt(5));     // 黄金角 ( Phyllotaxis )
  const t = index / total;                              // 归一化进度 0..1
  
  const r = maxRadius * Math.sqrt(t);                   // 平方根散布(中心密外围疏)
  const theta = index * goldenAngle + t * turns * Math.PI * 2; // 黄金角 + 旋转
  
  return {
    x: cx + Math.cos(theta) * r,
    y: cy + Math.sin(theta) * r,
    radius: r,
    angle: theta,
    fold: Math.floor(index / 4),                         // 第几圈包裹
    description: `包裹螺旋 第${Math.floor(index/4)+1}层 视角展开`,
  };
}

/**
 * HTTP 行列段落体验
 * 数据读取: 以行业为行角,一行一行的段落,对齐的行列
 * 行脚 = 从上到下,一行读完换一行
 */
export function httpParagraphPosition(index, total, startX, startY, lineWidth, lineHeight) {
  const perRow = Math.max(1, Math.ceil(Math.sqrt(total)));
  const col = index % perRow;
  const row = Math.floor(index / perRow);
  
  return {
    x: startX + col * (lineWidth / perRow),
    y: startY + row * lineHeight,
    row,
    col,
    indent: row * 2,                                     // 段落缩进
    description: `段落 第${row+1}行 第${col+1}列 (行角读取)`,
  };
}

/**
 * 通用协议空间位置 — 根据协议的行脚语义选择布局
 */
export function protocolSpatialPosition(proto, index, total, centerX, centerY, mode = 'orbit') {
  const base = typeof proto === 'string' ? PROTOCOL_ONTOLOGY.get(proto) : proto;
  if (!base) return null;

  // ★ GTTX (及 GTT) → 球形包裹螺旋布局
  if (mode === 'gttxFold' || (mode === 'auto' && (base.name === 'GTTX' || base.name === 'GTT' || base.name === 'GITT' || base.name === 'GTIT'))) {
    return { name: base.name, base: base.base, nature: base.nature, polarity: base.polarity, glyph: base.glyph, ...gttxFoldPosition(index, total, centerX, centerY, 200), readingMode: 'spiral-fold' };
  }

  // ★ HTTP/HTTPS → 行列段落布局
  if (mode === 'paragraph' || (mode === 'auto' && (base.name === 'HTTP' || base.name === 'HTTPS'))) {
    return { name: base.name, base: base.base, nature: base.nature, polarity: base.polarity, glyph: base.glyph, ...httpParagraphPosition(index, total, 50, 100, 500, 30), readingMode: 'paragraph-row' };
  }

  // ★ TCP/CP/FTP → 字节行列流布局
  if (mode === 'stream' || (mode === 'auto' && base.base === 256 && base.name !== 'GTT' && base.name !== 'GTTX')) {
    return { name: base.name, base: base.base, nature: base.nature, polarity: base.polarity, glyph: base.glyph, ...tcpStreamPosition(index, total, centerX, centerY), readingMode: 'stream-flow' };
  }

  // ★ 默认: 轨道圆形
  const radiusMap = { 2: 50, 16: 170, 64: 110, 256: 230 };
  const radius = radiusMap[base.base] || 150;
  const hash = base.name.split('').reduce((a, c) => a + c.charCodeAt(0), 0);
  const angle = (hash % 360) * Math.PI / 180 + (index / total) * Math.PI * 2;
  return {
    name: base.name, base: base.base, nature: base.nature, polarity: base.polarity, glyph: base.glyph,
    x: centerX + Math.cos(angle) * radius, y: centerY + Math.sin(angle) * radius,
    radius: 14 + Math.log2(base.base) * 4, orbitRadius: radius, readingMode: 'orbit',
  };
}

/**
 * TCP 字节行列流 — 从左到右,字节逐行流过
 */
function tcpStreamPosition(index, total, cx, cy) {
  const bytesPerRow = 16;
  const row = Math.floor(index / bytesPerRow);
  const col = index % bytesPerRow;
  return {
    x: cx - 200 + col * 25,
    y: cy - 100 + row * 20,
    row, col, byteIndex: index,
    description: `流 B[${index}] = 行${row} 列${col} (逐字节顺序读)`,
  };
}

// ═══════════════════════════════════════════════════════════
// 行脚编码类型 — Canvas 绘制时如何选择
// ═══════════════════════════════════════════════════════════

const EXPERIENCE_ENCODINGS = {
  'spiral-fold': {
    name: '螺旋包裹',
    description: '从核心曲线螺旋向外展开 (陈漫 GTTX)',
    draw: (ctx, x, y, size, t) => {
      ctx.beginPath();
      const turns = 3;
      for (let i = 0; i < turns * Math.PI * 2; i += 0.08) {
        const r = (i / (turns * Math.PI * 2)) * size;
        const px = x + Math.cos(i + t * 0.01) * r;
        const py = y + Math.sin(i + t * 0.01) * r;
        i === 0 ? ctx.moveTo(px, py) : ctx.lineTo(px, py);
      }
      ctx.stroke();
      ctx.save();
      ctx.globalAlpha = 0.15;
      ctx.beginPath();
      ctx.arc(x, y, size, 0, Math.PI * 2);
      ctx.stroke();
      ctx.restore();
    },
  },
  'paragraph-row': {
    name: '行列段落',
    description: '以行业为行角的段落对齐 (HTTP)',
    draw: (ctx, x, y, size) => {
      // 主行
      ctx.fillRect(x - size, y - 1, size * 2, 2);
      // 次行 (缩进)
      ctx.fillRect(x - size * 0.8, y + 5, size * 1.6, 2);
      // 第三行
      ctx.fillRect(x - size * 0.6, y + 10, size * 1.2, 2);
    },
  },
  'stream-flow': {
    name: '字节行列流',
    description: 'TCP 逐字节顺序流过',
    draw: (ctx, x, y, size) => {
      ctx.fillRect(x - size / 2, y - size / 2, size * 0.8, size * 0.8);
      // 流方向箭头
      ctx.beginPath();
      ctx.moveTo(x + size * 0.4, y);
      ctx.lineTo(x + size * 0.7, y);
      ctx.moveTo(x + size * 0.6, y - 3);
      ctx.lineTo(x + size * 0.7, y);
      ctx.lineTo(x + size * 0.6, y + 3);
      ctx.stroke();
    },
  },
  'orbit': {
    name: '轨道圆周',
    description: '默认轨道布局',
    draw: (ctx, x, y, size) => {
      ctx.beginPath();
      ctx.arc(x, y, size, 0, Math.PI * 2);
      ctx.stroke();
    },
  },
};

// ── 进制转换器 ──
export class RadixEncoder {
  static convert(input, fromBase, toBase) {
    if (fromBase === toBase) return input;
    const bits = this.toBinary(input, fromBase);
    return this.fromBinary(bits, toBase);
  }

  static toBinary(input, fromBase) {
    if (typeof input === 'string') {
      return input.split('').map(ch => {
        const val = parseInt(ch, fromBase);
        return isNaN(val) ? '' : val.toString(2).padStart(Math.log2(fromBase), '0');
      }).join('');
    }
    return input.toString(2);
  }

  static fromBinary(bits, toBase) {
    const perChar = Math.log2(toBase);
    const chars = [];
    for (let i = 0; i < bits.length; i += perChar) {
      const slice = bits.slice(i, i + perChar).padEnd(perChar, '0');
      const val = parseInt(slice, 2);
      chars.push(val.toString(toBase));
    }
    return chars.join('');
  }

  static gcdBase(baseA, baseB) {
    const gcd = (a, b) => b === 0 ? a : gcd(b, a % b);
    return gcd(baseA, baseB);
  }

  static lcmBase(baseA, baseB) {
    const gcd = (a, b) => b === 0 ? a : gcd(b, a % b);
    return (baseA * baseB) / gcd(baseA, baseB);
  }
}

// ── 本体论操作 ──
export function getProtocol(name) { return PROTOCOL_ONTOLOGY.get(name); }
export function listProtocols() { return Array.from(PROTOCOL_ONTOLOGY.values()); }

export function ontologicalDistance(protoA, protoB) {
  const a = typeof protoA === 'string' ? PROTOCOL_ONTOLOGY.get(protoA) : protoA;
  const b = typeof protoB === 'string' ? PROTOCOL_ONTOLOGY.get(protoB) : protoB;
  if (!a || !b) return Infinity;
  const baseDist = Math.abs(Math.log2(a.base) - Math.log2(b.base));
  const layerDist = Math.abs(a.layers.length - b.layers.length);
  const polDist = a.polarity === b.polarity ? 0 : 1;
  return baseDist + layerDist + polDist;
}

// ── 融合核心 ──
export function fuseProtocols(nameA, nameB) {
  const protoA = typeof nameA === 'string' ? PROTOCOL_ONTOLOGY.get(nameA) : nameA;
  const protoB = typeof nameB === 'string' ? PROTOCOL_ONTOLOGY.get(nameB) : nameB;
  if (!protoA || !protoB) return null;

  const commonBase = RadixEncoder.gcdBase(protoA.base, protoB.base);
  const fullBase = RadixEncoder.lcmBase(protoA.base, protoB.base);
  const fusedPolarity = protoA.polarity ^ protoB.polarity;
  const fusedLayers = [...new Set([...protoA.layers, ...protoB.layers])];
  const fusedSovereignty = { ...protoA.sovereignty, ...protoB.sovereignty, fused: true, fusionTime: Date.now() };

  // ★ 融合的行脚 = 两种阅读方式的复合
  const readingMode = inferFusedReading(protoA, protoB);

  return {
    name: `${protoA.name}×${protoB.name}`,
    base: commonBase, fullBase,
    nature: `${protoA.nature} ∩ ${protoB.nature}`,
    polarity: fusedPolarity, layers: fusedLayers, sovereignty: fusedSovereignty,
    parents: [protoA.name, protoB.name],
    isCouple: protoA.polarity !== protoB.polarity,
    readingMode,
    stability: judgeFusionStability(protoA, protoB),
  };
}

/**
 * 融合后的行脚推断:
 *   GTTX + HTTP → 螺旋包裹 + 行列对齐 = "段落螺旋曲线"
 *   TCP + CP   → 字节流 + 纠缠对 = "并行双向流"
 */
function inferFusedReading(a, b) {
  const modes = [a.experience?.readingMode || 'orbit', b.experience?.readingMode || 'orbit'];
  if (modes.includes('spiral-fold') && modes.includes('paragraph-row')) return 'spiral-paragraph';
  if (modes.includes('spiral-fold') && modes.includes('stream-flow')) return 'spiral-stream';
  if (modes.includes('paragraph-row') && modes.includes('stream-flow')) return 'row-stream';
  if (modes.includes('spiral-fold')) return 'spiral-fold';
  if (modes.includes('paragraph-row')) return 'paragraph-row';
  if (modes.includes('stream-flow')) return 'stream-flow';
  return 'orbit';
}

export function computeCommunity(protocolNames) {
  const protos = protocolNames.map(n => PROTOCOL_ONTOLOGY.get(n)).filter(Boolean);
  if (protos.length === 0) return null;

  const commonBase = protos.reduce((acc, p) => RadixEncoder.gcdBase(acc, p.base), protos[0].base);
  const fullBase = protos.reduce((acc, p) => RadixEncoder.lcmBase(acc, p.base), protos[0].base);
  const allLayers = [...new Set(protos.flatMap(p => p.layers))];
  const yangCount = protos.filter(p => p.polarity === POLARITY.YANG).length;
  const yinCount = protos.length - yangCount;

  // ★ 共同体行脚 = 所有行脚的并集
  const allReadingModes = [...new Set(protos.flatMap(p => p.experience?.readingMode ? [p.experience.readingMode] : []))];

  return {
    name: `[${protocolNames.join('+')}]共同体`,
    base: commonBase, fullBase, layers: allLayers,
    yangCount, yinCount,
    polarityBalance: (yangCount - yinCount) / protos.length,
    memberCount: protos.length, protocolNames,
    capacity: Math.log2(fullBase) * allLayers.length,
    readingModes: allReadingModes,
    health: judgeCommunityHealth(protos),
  };
}

// ── 同构裁判 ──
function judgeFusionStability(a, b, anchor = null) {
  const distAB = ontologicalDistance(a, b);
  const distAAnchor = anchor ? ontologicalDistance(a, anchor) : distAB * 0.8;
  const distBAnchor = anchor ? ontologicalDistance(b, anchor) : distAB * 0.9;

  const angle = (x, y, z) => {
    const cos = (x * x + y * y - z * z) / (2 * x * y);
    if (cos >= 1) return 0;
    if (cos <= -1) return Math.PI;
    return Math.acos(cos);
  };

  const angleA = angle(distAAnchor, distAB, distBAnchor);
  const angleB = angle(distAB, distBAnchor, distAAnchor);
  const angleC = angle(distBAnchor, distAAnchor, distAB);
  const sum = angleA + angleB + angleC;
  const deviation = Math.abs(sum - Math.PI);

  return {
    stable: deviation < 0.3,
    angles: [angleA, angleB, angleC], sum, deviation,
    verdict: deviation < 0.1 ? '完美融合' : deviation < 0.3 ? '稳定' : deviation < 0.6 ? '临界' : '不稳定',
  };
}

function judgeCommunityHealth(protos) {
  if (protos.length < 2) return { healthy: true, score: 1.0 };
  let totalDist = 0, pairs = 0;
  for (let i = 0; i < protos.length; i++) {
    for (let j = i + 1; j < protos.length; j++) {
      totalDist += ontologicalDistance(protos[i], protos[j]); pairs++;
    }
  }
  const avgDist = totalDist / pairs;
  const healthy = avgDist < 3.0;
  const score = Math.max(0, 1 - avgDist / 5);
  return { healthy, score: score.toFixed(2), avgDistance: avgDist.toFixed(3) };
}

// ═══════════════════════════════════════════════════════════
// 初始化 — ★ 含行脚体验语义
// ═══════════════════════════════════════════════════════════

function initProtocolOntology() {
  // ── G/GI — 基底 ──
  defineProtocol('G', BASE.BINARY, '基底:存在与虚无', POLARITY.YIN, ['ground'],
    { type: 'foundation' }, { readingMode: 'origin-point', dataReading: '点状感知·顿悟' });
  
  defineProtocol('GI', BASE.BINARY, '通用意图:感知源头', POLARITY.YANG, ['sense', 'intent'],
    { origin: true }, { readingMode: 'origin-point', dataReading: '感知涌出·源头' });

  // ★ HTTP — 行列段落行脚
  defineProtocol('HTTP', BASE.HEX, '超文本传输:以行业为行角的段落体验 · 逐行读取', POLARITY.YANG,
    ['request-line', 'headers', 'body'],
    { method: true },
    {
      readingMode: 'paragraph-row',
      dataReading: '以行业为行列,一行一行的段落 · 从上到下 · 对齐的行脚',
      geometry: '行角矩形段落',
      traversal: 'row-by-row, left-to-right, top-to-bottom',
      source: '以行业为行角的一个段落体验',
    }
  );

  defineProtocol('HTTPS', BASE.HEX, '加密表象层:行角段落·缠绕加密', POLARITY.YANG,
    ['tls-handshake', 'request-line', 'headers', 'body'],
    { method: true, encrypted: true },
    {
      readingMode: 'paragraph-row',
      dataReading: '加密的行角段落 · TLS 握手后逐行读',
      geometry: '加密矩形段落',
    }
  );

  // ★ TCP — 字节行列流
  defineProtocol('TCP', BASE.BYTE, '传输控制:字节行列流 · 逐字节顺序读', POLARITY.YANG,
    ['syn', 'stream', 'ack', 'fin'],
    { reliable: true },
    {
      readingMode: 'stream-flow',
      dataReading: '字节行列流 · SYN 建立 → 逐字节流过 → ACK 确认 → FIN 关闭',
      geometry: '行列字节流 (每行 16 字节)',
      traversal: 'sequential byte stream',
    }
  );

  defineProtocol('CP', BASE.BYTE, '伴侣协议:纠缠双流 · 并行双向读', POLARITY.YANG,
    ['couple-bind', 'heart-beat', 'sync'],
    { xorMask: 0xFF, entangled: true },
    {
      readingMode: 'stream-flow',
      dataReading: '并行双向流 · 夫妻各写各的通道 · XOR 0xFF 互补',
      geometry: '双螺旋并行流',
      traversal: 'bidirectional entangled',
    }
  );

  defineProtocol('FTP', BASE.BYTE, '文件传输:控制+数据双行列通道', POLARITY.YANG,
    ['control', 'data'],
    { dualChannel: true },
    {
      readingMode: 'stream-flow',
      dataReading: '双行列通道 · 控制行(命令) + 数据行(内容)',
      geometry: '双行列流',
    }
  );

  // ★★ GTTX — 球形包裹螺旋行脚 (陈漫体验)
  defineProtocol('GTTX', BASE.B64, '扩展通用传输:以星球为圈层的包裹螺旋 · 曲线行脚向外展开', POLARITY.YIN,
    ['target', 'transport', 'extension'],
    { generalized: true, extended: true },
    {
      readingMode: 'spiral-fold',
      dataReading: '陈漫以球形为圈层关系 · 从核心以曲线螺旋逐层包裹向外展开',
      geometry: '球形包裹螺旋 · Phyllotaxis 黄金角散布',
      traversal: 'spiral outward from core, each fold wraps the previous',
      curve: '从中心视角向外展开的一套运动轨迹',
      description: '以球星为圈层关系的体验, 包裹的宣传曲线螺旋为中心视角, 向外展开',
    }
  );

  defineProtocol('GTT', BASE.B64, '通用目标传输层:球形包裹螺旋 · 圈层体验', POLARITY.YIN,
    ['target', 'transport'],
    { generalized: true },
    {
      readingMode: 'spiral-fold',
      dataReading: '球形包裹螺旋 · 从核心逐层向外展开',
      geometry: '螺旋包裹',
    }
  );

  // GTITX = GTTX + Intent 扩展
  defineProtocol('GTITX', BASE.B64, '扩展目标意图传输:意向螺旋 · 以意图为中心视角向外包裹展开', POLARITY.YANG,
    ['target', 'transport', 'intent', 'extension'],
    { intent: true, extended: true },
    {
      readingMode: 'spiral-fold',
      dataReading: '以意图为核心的球形包裹螺旋 · 曲线向外 · 逐层包裹',
      geometry: '意向中心螺旋 · 曲线行脚',
      traversal: 'intent-centered spiral outward',
    }
  );

  defineProtocol('GITT', BASE.B64, '通用意图目标传输:意向螺旋 · 读法=先意向后寻目标', POLARITY.YANG,
    ['intent', 'target', 'transport'],
    { intent: true },
    {
      readingMode: 'spiral-fold',
      dataReading: '意向 → 目标 → 传输 · 螺旋寻的',
      geometry: '意向螺旋',
    }
  );

  defineProtocol('GTIT', BASE.B64, '通用目标意图传输:目标寻的螺旋 · 读法=先目的地形后意图', POLARITY.YANG,
    ['target', 'transport', 'intent'],
    { intent: true },
    {
      readingMode: 'spiral-fold',
      dataReading: '目标 → 传输 → 意图 · 螺旋展开',
      geometry: '目的地形螺旋',
    }
  );
}

initProtocolOntology();

export default {
  BASE, POLARITY, RadixEncoder, fuseProtocols, computeCommunity,
  getProtocol, listProtocols, ontologicalDistance, protocolSpatialPosition,
  gttxFoldPosition, httpParagraphPosition,
  EXPERIENCE_ENCODINGS,
  defineProtocol,
};
