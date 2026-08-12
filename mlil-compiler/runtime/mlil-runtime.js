// mlil-runtime.js — mlil WASM 宿主运行时
// 浏览器端加载和执行编译后的 .mlil 程序
//
// 用法:
//   const runtime = new MLILRuntime();
//   const module = await runtime.load('math.wasm');
//   const result = module.call('double', 210);  // 返回 420

const SYMBOL_TABLE = [
  'ymba','ceiq','kndo','nvfm','bujq','qidi','shuf','zujm',
  'yita','sita','jiqu','guax','fuhr','xubi','wuhr','tudo',
  'lido','qion','liam','duen','shou','mian','ling','yige',
  'yige','yier','yisan','yisi','yiwu','yiliu','yiqiy ba','yijiu',
  'ershi','ershi','ershiy','ershi','ershiy','ershis','ershiw','ershily',
  'qiy ba','qijiu','sanshi','sanshiy','sanshi','sanshiy','sanshis','sanshiw',
  'sishily','sishiji','sishi','sishi','sishiy','sishi','sishis','sushiw',
  'wushily','wushiji','wushiba','wushiji','liushiy','liushiy','liushis','liushiw',
  'qishily','qishiji','qishiba','bashiy','bashiy','bashis','bashiw','jiushily',
  'jiushiji','jiushiji','yibai','yibaiy','yibais','yibaiw','yibaily','yibaiq',
];

const ANGLE_FULL = 384;  // 64进制: 完整圆周 384°

class MLILRuntime {
  constructor() {
    this.modules = new Map();
    this.importObject = this._buildImports();
  }

  // ── 导入对象: WASM 模块可调用的宿主函数 ──
  _buildImports() {
    const self = this;
    return {
      env: {
        // 打印整数 (调试用)
        mlil_print_i32: (val) => {
          console.log('[mlil]', val);
        },
        // 打印角度值 (自动转为 64进制表示)
        mlil_print_angle: (val) => {
          const base64 = self.toBase64(val);
          console.log('[mlil:angle]', val + '° (' + base64 + ')');
        },
        // 打印符号名
        mlil_print_symbol: (val) => {
          const name = self.symbolName(val);
          console.log('[mlil:symbol]', name);
        },
        // 内存分配
        mlil_alloc: (size) => {
          return self._allocMemory(size);
        },
        // 内存释放
        mlil_free: (ptr) => {
          self._freeMemory(ptr);
        },
        // 幂运算
        mlil_pow: (base, exp) => {
          return Math.pow(base, exp);
        },
        // 开方
        mlil_sqrt: (val) => {
          return Math.floor(Math.sqrt(val));
        },
      },
      // 直接函数导出 (简化调用)
      mlil: {
        print: (val) => console.log('[mlil]', val),
        print_angle: (val) => console.log('[mlil:angle]', val + '°'),
        base64_encode: (val) => self.toBase64(val),
        symbol_to_name: (val) => self.symbolName(val),
        name_to_symbol: (name) => self.symbolValue(name),
      }
    };
  }

  // ── 内存管理 (简化) ──
  _allocMemory(size) {
    if (!this._memory) {
      this._memory = new WebAssembly.Memory({ initial: 1, maximum: 10 });
    }
    const ptr = this._nextPtr || 0;
    this._nextPtr = ptr + size;
    return ptr;
  }

  _freeMemory(ptr) {
    // 简化: 暂不实现真正的释放
  }

  get memory() {
    if (!this._memory) {
      this._memory = new WebAssembly.Memory({ initial: 1, maximum: 10 });
    }
    return this._memory;
  }

  // ── 加载 WASM 模块 ──
  async load(urlOrBytes, name) {
    let bytes;
    if (typeof urlOrBytes === 'string') {
      const response = await fetch(urlOrBytes);
      bytes = new Uint8Array(await response.arrayBuffer());
    } else {
      bytes = urlOrBytes;
    }

    const module = await WebAssembly.compile(bytes);
    const instance = await WebAssembly.instantiate(module, {
      ...this.importObject,
      env: {
        ...this.importObject.env,
        memory: this.memory,
      }
    });

    const moduleName = name || urlOrBytes;
    const wrapper = new MLILModule(instance, moduleName, this);
    this.modules.set(moduleName, wrapper);
    return wrapper;
  }

  // ── 从 mlil 源码直接编译加载 (使用内嵌编译器或预编译) ──
  async loadFromSource(wasmBytes, name) {
    return this.load(wasmBytes, name);
  }

  // ── 64进制编码 ──
  toBase64(value) {
    if (value === 0) return SYMBOL_TABLE[0];
    let result = '';
    let v = value;
    while (v > 0) {
      const idx = v % 64;
      result = SYMBOL_TABLE[idx] + (result ? ':'' + result : '');
      v = Math.floor(v / 64);
    }
    return result;
  }

  // ── 符号名查找 ──
  symbolName(value) {
    if (value >= 0 && value < SYMBOL_TABLE.length) {
      return SYMBOL_TABLE[value];
    }
    return 'unk' + value;
  }

  symbolValue(name) {
    const idx = SYMBOL_TABLE.indexOf(name);
    return idx >= 0 ? idx : null;
  }

  // ── 角度换算 ──
  angleToDegrees(val) {
    return val * 6;  // 每单位 = 6°
  }

  degreesToAngle(deg) {
    return Math.floor(deg / 6);
  }
}

// ── 单个 WASM 模块的包装 ──
class MLILModule {
  constructor(instance, name, runtime) {
    this.instance = instance;
    this.name = name;
    this.runtime = runtime;
    this.exports = instance.exports;
  }

  // 调用导出函数
  call(fnName, ...args) {
    const fn = this.exports[fnName];
    if (!fn) {
      throw new Error(`函数 '${fnName}' 不存在于模块 '${this.name}'`);
    }
    return fn(...args);
  }

  // 获取所有导出函数名
  getExportedFunctions() {
    return Object.keys(this.exports).filter(k => typeof this.exports[k] === 'function');
  }

  // 获取导出表
  getExportTable() {
    const table = {};
    for (const key of Object.keys(this.exports)) {
      const val = this.exports[key];
      table[key] = typeof val === 'function' ? '[function]' : val;
    }
    return table;
  }
}

// ── 导出 (同时也作为 ES Module) ──
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { MLILRuntime, MLILModule };
}

// 浏览器全局暴露
if (typeof window !== 'undefined') {
  window.MLILRuntime = MLILRuntime;
  window.MLILModule = MLILModule;
  window.SYMBOL_TABLE = SYMBOL_TABLE;
  window.ANGLE_FULL = ANGLE_FULL;
}
