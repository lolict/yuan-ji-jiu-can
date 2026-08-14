/**
 * ================================================
 * UniCore 四维不进位体系 + 等效维度映射
 * ================================================
 * 
 * 核心规则：
 * 1. 四个维度 = 四个集装箱
 *    (维度1, 维度2, 维度3, 维度4)
 * 
 * 2. 前三个维度：静态框架，永不进位
 *    - 专门对接机器码01
 *    - 保持 (0, 0, 0) 基准形态
 *    - 不自动增长，不膨胀
 * 
 * 3. 第四个维度：动态可增长，可自由递增
 *    - 负责数值计数
 *    - 可以无限变大
 *    - 不干扰前三维框架
 * 
 * 4. 等效维度映射：
 *    - 第四维每涨1个，对应前三维等效层级占位
 *    - 数值增长，但维度层级权重等效对齐
 *    - 全程不破坏前三维 (0, 0, 0) 的底层结构
 */

const { CONTAINERS, ContainerEncoder } = require('./unicore_containers.js');

// ========================
// 四维不进位体系核心
// ========================

class FourDimNonCarrySystem {
  constructor() {
    // 前三个维度：静态框架，永远是 [0, 0, 0]
    this.staticFrame = [0, 0, 0];
    
    // 第四个维度：动态数值，从0开始
    this.dynamicValue = 0;
    
    this.encoder = new ContainerEncoder();
  }
  
  // 核心规则1：获取四维状态（前三维永远0，第四维动态）
  getState() {
    return [...this.staticFrame, this.dynamicValue];
  }
  
  // 核心规则2：仅递增第四维（永不干扰前三维）
  increment() {
    this.dynamicValue++;
    return this.getState();
  }
  
  // 核心规则3：仅递减第四维
  decrement() {
    if (this.dynamicValue > 0) {
      this.dynamicValue--;
    }
    return this.getState();
  }
  
  // 核心规则4：等效维度映射
  // 第四维的数值 → 对应前三维等效层级占位
  getEquivalentMapping() {
    const value = this.dynamicValue;
    
    // 等效映射规则：
    // 第四维每一个增量，对应前三维里一个等效机器码占位
    // 前三维保持0，但通过等效映射得到层级权重
    return {
      state: this.getState(),
      value: value,
      equivalentLayer: Math.floor(value / 17),  // 每17个数字为一层
      equivalentContainer: value % 4 + 1,  // 对应集装箱1-4
      equivalentBit: value % 2  // 对应机器码0/1
    };
  }
  
  // 核心规则5：机器码01映射
  // 前三维框架 → 机器码二进制形式
  getMachineCodeMapping() {
    // 前三维 (0, 0, 0) 对应机器码
    // 动态数值通过等效映射得到机器码位
    const mapping = this.getEquivalentMapping();
    
    return {
      staticFrameBits: '000',  // 前三维永远000
      dynamicValueBits: this._toBits(mapping.equivalentBit, 1),
      fullBits: '000' + this._toBits(mapping.value, 8),
      mapping: mapping
    };
  }
  
  _toBits(num, length) {
    let bits = num.toString(2);
    while (bits.length < length) {
      bits = '0' + bits;
    }
    return bits;
  }
  
  // 重置
  reset() {
    this.dynamicValue = 0;
    return this.getState();
  }
}

// ========================
// 集装箱机器码对接器
// ========================

class ContainerMachineCodeAdapter {
  constructor() {
    this.fourDimSystem = new FourDimNonCarrySystem();
  }
  
  // 机器码 → 集装箱映射
  machineCodeToContainer(bitValue) {
    // bitValue 0或1 → 对应集装箱
    return bitValue === 0 ? CONTAINERS[1] : CONTAINERS[2];
  }
  
  // 集装箱 → 机器码映射
  containerToMachineCode(containerId) {
    // 集装箱1→0，集装箱2→1，集装箱3→0，集装箱4→1
    if (containerId === 1 || containerId === 3) {
      return 0;
    } else {
      return 1;
    }
  }
  
  // 等效机器码数值
  getEquivalentMachineCodeValue() {
    const mapping = this.fourDimSystem.getEquivalentMapping();
    return {
      decimal: mapping.value,
      binary: mapping.equivalentBit,
      containerEquivalent: mapping.equivalentContainer,
      layerEquivalent: mapping.equivalentLayer
    };
  }
  
  // 递增（仅第四维）
  increment() {
    return this.fourDimSystem.increment();
  }
  
  decrement() {
    return this.fourDimSystem.decrement();
  }
  
  getState() {
    return this.fourDimSystem.getState();
  }
  
  getMachineCodeMapping() {
    return this.fourDimSystem.getMachineCodeMapping();
  }
}

// ========================
// 导出
// ========================

module.exports = {
  FourDimNonCarrySystem,
  ContainerMachineCodeAdapter
};
