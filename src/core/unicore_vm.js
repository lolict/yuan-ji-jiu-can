/**
 * ================================================
 * UniCore 完整虚拟机
 * ================================================
 * 
 * 整合:
 * - 符号体系
 * - 集装箱编码
 * - 指令集
 * - 3D格子
 */

const { SHENGMU_LOWER, SHENGMU_UPPER, YUNMU_LOWER, YUNMU_UPPER, SHENGDIAO, NUMBERS } = require('./unicore_symbols.js');
const { CONTAINERS, ContainerEncoder } = require('./unicore_containers.js');

// ========================
// 指令集定义
// ========================

const INSTRUCTIONS = {
  // 小写声母 → 运算指令
  LOAD: 'b',
  STORE: 'p',
  MOVE: 'm',
  FETCH: 'f',
  DROP: 'd',
  TEST: 't',
  NOT: 'n',
  LOOP: 'l',
  
  // 小写韵母 → 流程控制
  IF: 'ꞟ',
  ELSE: 'ɘ',
  ENDIF: '⊱',
  SWITCH: 'ɒ',
  CASE: '౪',
  BREAK: 'ÿ',
  
  // 大写韵母 → 阵法
  ZHUI: 'Ꞟ',
  QIU: 'ɚ',
  FANG: 'ʚ',
  SAN: 'Ɒ',
  FENG: 'ȣ',
  MEN: 'Ÿ',
  SUO: 'ɣ',
  JING: 'Ʊ',
  TOUYI: 'Ə',
  
  // 大写声母 → 聚类
  CLUSTER: 'B',
  PARTITION: 'P',
  MERGE: 'M'
};

// ========================
// 3D格子
// ========================

class PureGrid {
  constructor() {
    this.center = null;
    this.walls = {
      front: null,
      back: null,
      left: null,
      right: null,
      up: null,
      down: null
    };
    this.innerCorners = Array(8).fill(null);
    this.outerConnections = Array(6).fill(null);
    this.crossNodes = Array(64 - 1 - 6 - 8 - 6).fill(null);
  }
  
  setWall(direction, value) {
    this.walls[direction] = value;
  }
  
  setCenter(value) {
    this.center = value;
  }
  
  setInnerCorner(index, value) {
    this.innerCorners[index] = value;
  }
  
  setOuterConnection(index, value) {
    this.outerConnections[index] = value;
  }
}

// ========================
// 完整虚拟机
// ========================

class UniCoreVM {
  constructor() {
    this.encoder = new ContainerEncoder();
    this.memory = [];
    this.registers = {};
    this.stack = [];
    this.pc = 0;
    this.grid = new PureGrid();
    
    this._initRegisters();
  }
  
  _initRegisters() {
    for (let i = 0; i < 4; i++) {
      this.registers['r' + i] = [0, 0, 0, 0];
    }
  }
  
  // 执行指令
  execute(program) {
    this.pc = 0;
    this.stack = [];
    
    while (this.pc < program.length) {
      const instruction = program[this.pc];
      this.pc++;
      
      this._executeInstruction(instruction);
    }
  }
  
  _executeInstruction(instr) {
    switch (instr) {
      case INSTRUCTIONS.LOAD:
        this._load();
        break;
      case INSTRUCTIONS.STORE:
        this._store();
        break;
      case INSTRUCTIONS.MOVE:
        this._move();
        break;
      case INSTRUCTIONS.FETCH:
        this._fetch();
        break;
      case INSTRUCTIONS.DROP:
        this._drop();
        break;
      case INSTRUCTIONS.CLUSTER:
        this._cluster();
        break;
      case INSTRUCTIONS.ZHUI:
        this._zhenfa('锥阵');
        break;
      case INSTRUCTIONS.QIU:
        this._zhenfa('球阵');
        break;
      case INSTRUCTIONS.FANG:
        this._zhenfa('方块阵');
        break;
      case INSTRUCTIONS.SAN:
        this._zhenfa('三角阵');
        break;
      case INSTRUCTIONS.FENG:
        this._zhenfa('蜂窝阵');
        break;
      case INSTRUCTIONS.JING:
        this._zhenfa('镜像阵');
        break;
      case INSTRUCTIONS.TOUYI:
        this._zhenfa('对立统一阵');
        break;
      default:
        // 如果是符号，压入堆栈
        this.stack.push(instr);
    }
  }
  
  _load() {
    if (this.stack.length > 0) {
      const value = this.stack.pop();
      this.memory.push(value);
      console.log('[LOAD] 从堆栈加载到内存: ' + value);
    }
  }
  
  _store() {
    if (this.memory.length > 0) {
      const value = this.memory.pop();
      this.stack.push(value);
      console.log('[STORE] 从内存存回堆栈: ' + value);
    }
  }
  
  _move() {
    if (this.stack.length > 0) {
      const value = this.stack.pop();
      this.grid.setCenter(value);
      console.log('[MOVE] 移动到格子中心: ' + value);
    }
  }
  
  _fetch() {
    const value = this.grid.center;
    if (value !== null) {
      this.stack.push(value);
      console.log('[FETCH] 从格子中心取回: ' + value);
    }
  }
  
  _drop() {
    if (this.stack.length > 0) {
      const dropped = this.stack.pop();
      console.log('[DROP] 丢弃值: ' + dropped);
    }
  }
  
  _cluster() {
    console.log('[CLUSTER] 聚类操作');
  }
  
  _zhenfa(name) {
    console.log('[ZHENFA] 激活阵法: ' + name);
  }
  
  // 获取当前堆栈
  getStack() {
    return [...this.stack];
  }
  
  // 获取当前内存
  getMemory() {
    return [...this.memory];
  }
  
  // 获取当前格子
  getGrid() {
    return this.grid;
  }
}

// ========================
// 导出
// ========================

module.exports = {
  UniCoreVM,
  INSTRUCTIONS,
  PureGrid
};
