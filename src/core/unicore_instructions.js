/**
 * ================================================
 * UniCore 指令集 - 基于用户定义的符号
 * ================================================
 * 
 * 不分先后顺序，声母、韵母、声调随意组合
 * 所有符号都是指令
 */

const {
  SHENGMU_LOWER, SHENGMU_UPPER, SHENGMU,
  YUNMU_LOWER, YUNMU_UPPER, YUNMU,
  SHENGDIAO, NUMBERS
} = require('./unicore_symbols.js');

// ========================
// 指令集定义（基于符号）
// ========================

const INSTRUCTIONS = {
  // 基础运算（用声母）
  'b': { op: 'LOAD', desc: '加载' },
  'p': { op: 'STORE', desc: '存储' },
  'm': { op: 'MOVE', desc: '移动' },
  'f': { op: 'FETCH', desc: '取回' },
  'd': { op: 'DROP', desc: '丢弃' },
  't': { op: 'TEST', desc: '测试' },
  'n': { op: 'NOT', desc: '非运算' },
  'l': { op: 'LOOP', desc: '循环' },
  
  // 流程控制（用韵母）
  'ə': { op: 'IF', desc: '条件判断' },
  'ä': { op: 'ELSE', desc: '否则' },
  'ɘ': { op: 'ENDIF', desc: '结束条件' },
  'ö': { op: 'SWITCH', desc: '开关' },
  'ѧ': { op: 'CASE', desc: '分支' },
  'ʉ': { op: 'BREAK', desc: '跳出' },
  
  // 通信（用声调）
  'aa1': { op: 'SEND', desc: '发送' },
  'aa2': { op: 'RECV', desc: '接收' },
  'aa3': { op: 'SYNC', desc: '同步' },
  
  // 聚类（用大写声母）
  'B': { op: 'CLUSTER', desc: '聚类' },
  'P': { op: 'PARTITION', desc: '分区' },
  'M': { op: 'MERGE', desc: '合并' },
  
  // 阵法（用大写韵母）
  'Ə': { op: 'ZHUI', desc: '锥阵' },
  'Ä': { op: 'QIU', desc: '球阵' },
  'Ǝ': { op: 'FANG', desc: '方块阵' },
  'Ɯ': { op: 'SAN', desc: '三角阵' },
  'Ö': { op: 'FENG', desc: '蜂窝阵' },
  'Я': { op: 'MEN', desc: '门阵' },
  'Ж': { op: 'SUO', desc: '锁阵' },
  'ꙮ': { op: 'JING', desc: '镜像阵' },
  'М': { op: 'TOUYI', desc: '对立统一阵' }
};

// ========================
// 指令编码器
// ========================

class InstructionEncoder {
  constructor() {
    this.symbolMap = new Map();
    this._buildMap();
  }
  
  _buildMap() {
    // 声母 -> 指令
    for (const sm of SHENGMU) {
      const lower = sm.toLowerCase();
      if (INSTRUCTIONS[lower]) {
        this.symbolMap.set(sm, INSTRUCTIONS[lower]);
        this.symbolMap.set(lower, INSTRUCTIONS[lower]);
      }
    }
    // 韵母 -> 指令
    for (const ym of YUNMU) {
      if (INSTRUCTIONS[ym]) {
        this.symbolMap.set(ym, INSTRUCTIONS[ym]);
      }
    }
    // 声调 -> 指令
    for (const sd of SHENGDIAO) {
      if (INSTRUCTIONS[sd]) {
        this.symbolMap.set(sd, INSTRUCTIONS[sd]);
      }
    }
  }
  
  // 编码：符号 -> 指令
  encode(symbol) {
    return this.symbolMap.get(symbol) || { op: 'NOP', desc: '空操作' };
  }
  
  // 解码：指令 -> 符号
  decode(instruction) {
    for (const [symbol, inst] of this.symbolMap) {
      if (inst.op === instruction) {
        return symbol;
      }
    }
    return null;
  }
  
  // 组合指令：任意符号串 -> 指令序列
  compose(symbols) {
    const result = [];
    for (const sym of symbols) {
      const inst = this.encode(sym);
      if (inst.op !== 'NOP') {
        result.push(inst);
      }
    }
    return result;
  }
}

// ========================
// 虚拟机
// ========================

class SymbolVM {
  constructor() {
    this.encoder = new InstructionEncoder();
    this.memory = new Map();
    this.pc = 0;
    this.registers = {};
  }
  
  // 执行符号序列
  execute(symbols) {
    const instructions = this.encoder.compose(symbols);
    for (const inst of instructions) {
      this._executeInst(inst);
    }
    return this.registers;
  }
  
  _executeInst(inst) {
    switch (inst.op) {
      case 'LOAD':
        this.registers.LOAD = true;
        break;
      case 'STORE':
        this.registers.STORE = true;
        break;
      case 'MOVE':
        this.registers.MOVE = true;
        break;
      case 'CLUSTER':
        this.registers.CLUSTER = true;
        break;
      case 'ZHUI':
        this.registers.ZHUI = true;
        break;
      case 'JING':
        this.registers.JING = true;
        break;
      default:
        break;
    }
  }
  
  // 状态
  status() {
    return {
      registers: this.registers,
      memory: this.memory.size,
      pc: this.pc
    };
  }
}

// ========================
// 导出
// ========================

module.exports = {
  INSTRUCTIONS,
  InstructionEncoder,
  SymbolVM
};
