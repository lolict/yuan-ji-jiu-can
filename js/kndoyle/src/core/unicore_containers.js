/**
 * ================================================
 * UniCore 集装箱编码系统
 * ================================================
 * 
 * 集装箱1: 声母 (48个)
 * 集装箱2: 韵母 (64个)
 * 集装箱3: 声调 (170个)
 * 集装箱4: 数字 (17个)
 * 
 * 优先级: 集装箱3 > 集装箱2 > 集装箱1 > 集装箱4 (税率效率优先)
 */

const { SHENGMU, YUNMU, SHENGDIAO, NUMBERS, PRIORITY, RADIX } = require('./unicore_symbols.js');

// ========================
// 集装箱定义
// ========================

const CONTAINERS = {
  1: {
    name: '声母',
    symbols: SHENGMU,
    count: RADIX.shengmu,
    index: 0
  },
  2: {
    name: '韵母',
    symbols: YUNMU,
    count: RADIX.yunmu,
    index: 1
  },
  3: {
    name: '声调',
    symbols: SHENGDIAO,
    count: RADIX.shengdiao,
    index: 2
  },
  4: {
    name: '数字',
    symbols: NUMBERS,
    count: RADIX.number,
    index: 3
  }
};

// ========================
// 编码/解码函数
// ========================

class ContainerEncoder {
  constructor() {
    this.containers = CONTAINERS;
    this.priority = PRIORITY.byCount; // 默认税率效率优先
  }

  // 编码: 索引 → 符号
  encode(containerId, index) {
    const container = this.containers[containerId];
    if (index < 0 || index >= container.count) {
      return null;
    }
    return container.symbols[index];
  }

  // 解码: 符号 → 索引
  decode(containerId, symbol) {
    const container = this.containers[containerId];
    const index = container.symbols.indexOf(symbol);
    return index >= 0 ? index : null;
  }

  // 递增进位（模拟进制加法）
  increment(carrier) {
    // carrier 格式: [声调索引, 韵母索引, 声母索引, 数字索引]
    const result = [...carrier];
    
    // 优先级: 集装箱3(声调) → 集装箱2(韵母) → 集装箱1(声母) → 集装箱4(数字)
    // 按优先级顺序从最后开始递增
    
    for (let i = this.priority.length - 1; i >= 0; i--) {
      const containerName = this.priority[i];
      const containerId = this._nameToId(containerName);
      
      result[i] = (result[i] || 0) + 1;
      
      if (result[i] < this.containers[containerId].count) {
        break;
      } else {
        result[i] = 0; // 进位
      }
    }
    
    return result;
  }

  // 递减
  decrement(carrier) {
    const result = [...carrier];
    
    for (let i = this.priority.length - 1; i >= 0; i--) {
      const containerName = this.priority[i];
      const containerId = this._nameToId(containerName);
      
      result[i] = (result[i] || 0) - 1;
      
      if (result[i] >= 0) {
        break;
      } else {
        result[i] = this.containers[containerId].count - 1; // 退位
      }
    }
    
    return result;
  }

  // 索引 → 符号
  indicesToSymbols(indices) {
    const result = [];
    for (let i = 0; i < this.priority.length; i++) {
      const containerName = this.priority[i];
      const containerId = this._nameToId(containerName);
      result.push(this.encode(containerId, indices[i]));
    }
    return result;
  }

  // 符号 → 索引
  symbolsToIndices(symbols) {
    const result = [];
    for (let i = 0; i < this.priority.length; i++) {
      const containerName = this.priority[i];
      const containerId = this._nameToId(containerName);
      result.push(this.decode(containerId, symbols[i]));
    }
    return result;
  }

  _nameToId(name) {
    switch(name) {
      case 'shengmu': return 1;
      case 'yunmu': return 2;
      case 'shengdiao': return 3;
      case 'number': return 4;
      default: return null;
    }
  }

  // 获取符号数量
  getCount(containerId) {
    return this.containers[containerId].count;
  }
}

// ========================
// 导出
// ========================

module.exports = {
  CONTAINERS,
  ContainerEncoder
};
