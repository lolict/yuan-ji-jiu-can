/**
 * ================================================
 * CocoonLang 自举感应器 (BootstrapSensor)
 * ================================================
 *
 * "感应器"是用户独创概念：它是蛹茧"看见"世界的能力。
 * 蛹茧（Cocoon）要破茧成蝶，必须先感知：
 *   - 自己身处何种环境（平台/CPU/资源）
 *   - 蛹茧文件本身的结构与内容
 *   - 哪些寄生壳可用、适配度如何
 *   - 宿主系统的能力（GPU/网络/存储/传感器）
 *   - 网络状态与安全环境
 *   - 感应器自身的状态（自举感应）
 *
 * 自举含义：感应器既能由 JavaScript 调用，也能用 CocoonLang 自身源码书写。
 *
 * 方法：
 *   - senseEnvironment()  : 感知运行环境（平台/CPU架构/可用资源）
 *   - senseCocoon(buffer) : 感知蛹茧文件的内容和结构
 *   - senseShell(shellId) : 感知某个寄生壳的适配度
 *   - senseSystem()       : 感知宿主系统能力（GPU/网络/存储/传感器）
 *   - senseNetwork()      : 感知网络状态
 *   - senseSecurity()     : 感知安全环境
 *   - selfSense()         : 自举感应（感应器感知自己的状态）
 *   - recommend()         : 根据感知结果推荐最佳壳和配置
 */

'use strict';

const os = require('os');

class BootstrapSensor {
  constructor() {
    this.version = '0.1.0-bootstrap';
    // 感知结果缓存
    this._cache = {};
    // 已知寄生壳注册表
    this.shells = this._initShellRegistry();
    // 感应器自身状态
    this.selfState = {
      createdAt: Date.now(),
      sensesPerformed: 0,
      lastSense: null,
      health: 'nominal',
    };
  }

  // ===============
  // 1. senseEnvironment()
  // ===============
  // 感知运行环境：平台/CPU架构/内存/可用资源
  senseEnvironment() {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'environment';
    const env = {
      hostname: os.hostname(),
      platform: os.platform(),
      osType: os.type(),
      osRelease: os.release(),
      arch: os.arch(),
      cpuCount: os.cpus().length,
      cpuModel: os.cpus()[0] ? os.cpus()[0].model : 'unknown',
      cpuSpeed: os.cpus()[0] ? os.cpus()[0].speed : 0,
      totalMemory: os.totalmem(),
      freeMemory: os.freemem(),
      memoryUsage: process.memoryUsage(),
      uptime: os.uptime(),
      loadavg: os.loadavg(),
      nodeVersion: process.version,
      pid: process.pid,
      cwd: process.cwd(),
      endianness: os.endianness(),
      // 资源充裕度评估
      resourceLevel: this._assessResources(os.totalmem(), os.freemem(), os.cpus().length),
    };
    this._cache.environment = env;
    return env;
  }

  _assessResources(total, free, cpus) {
    const freeRatio = free / total;
    if (free > 1e9 && cpus >= 4 && freeRatio > 0.3) return 'abundant';
    if (free > 256e6 && cpus >= 2 && freeRatio > 0.1) return 'sufficient';
    return 'constrained';
  }

  // ===============
  // 2. senseCocoon(buffer)
  // ===============
  // 感知蛹茧文件的内容和结构
  // 蛹茧文件可以是 Buffer / 字符串 / 字节阵列
  senseCocoon(buffer) {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'cocoon';

    if (!buffer) {
      return { error: '空蛹茧', size: 0 };
    }

    const bytes = this._toBytes(buffer);
    const analysis = {
      size: bytes.length,
      // 魔数检测（前 4 字节）
      magic: this._detectMagic(bytes),
      // 文件类型推断
      fileType: this._inferFileType(bytes),
      // 熵分析（判断是否压缩/加密）
      entropy: this._entropy(bytes),
      compression: null,
      // 字节分布
      byteDistribution: this._byteDistribution(bytes),
      // 结构特征
      hasNullBytes: bytes.includes(0),
      nullByteRatio: bytes.filter(b => b === 0).length / Math.max(bytes.length, 1),
      // 可读性
      printableRatio: bytes.filter(b => b >= 0x20 && b <= 0x7e).length / Math.max(bytes.length, 1),
      // 蛹茧完整度评估
      integrity: this._assessIntegrity(bytes),
    };
    analysis.compression = analysis.entropy > 7.5 ? 'likely-compressed' :
                           analysis.entropy < 2.0 ? 'likely-sparse' : 'none';
    this._cache.cocoon = analysis;
    return analysis;
  }

  _toBytes(buffer) {
    if (Buffer.isBuffer(buffer)) return Array.from(buffer);
    if (Array.isArray(buffer)) return buffer;
    if (typeof buffer === 'string') {
      // 尝试十六进制解码
      if (/^[0-9a-fA-F\s]+$/.test(buffer) && buffer.replace(/\s/g, '').length % 2 === 0) {
        const hex = buffer.replace(/\s/g, '');
        const bytes = [];
        for (let i = 0; i + 1 < hex.length; i += 2) {
          bytes.push(parseInt(hex.substr(i, 2), 16));
        }
        return bytes;
      }
      return Array.from(Buffer.from(buffer, 'utf8'));
    }
    if (typeof buffer === 'object') {
      // 视为字节码对象
      if (buffer.code) return this._toBytes(JSON.stringify(buffer));
    }
    return [];
  }

  _detectMagic(bytes) {
    if (bytes.length < 4) return 'unknown';
    const hex = bytes.slice(0, 4).map(b => b.toString(16).padStart(2, '0')).join('');
    const magics = {
      '0061736d': 'wasm',
      '7f454c46': 'elf',
      '4d5a':     'pe/exe',
      'cafebabe': 'java-class',
      '504b0304': 'zip',
      '1f8b':     'gzip',
      '25504446': 'pdf',
      '89504e47': 'png',
      'cocoon':   'cocoon-lang',
    };
    // 检查完整 4 字节
    if (magics[hex]) return magics[hex];
    // 检查前 2 字节
    const short = hex.substr(0, 4);
    if (magics[short]) return magics[short];
    // CocoonLang 字节码特征：首字节为 PUSH(0x01)
    if (bytes[0] === 0x01) return 'cocoon-bytecode';
    return hex;
  }

  _inferFileType(bytes) {
    const magic = this._detectMagic(bytes);
    const typeMap = {
      'wasm': 'WebAssembly 模块',
      'elf': 'ELF 可执行文件',
      'pe/exe': 'Windows 可执行文件',
      'java-class': 'Java 类文件',
      'zip': 'ZIP 压缩包',
      'gzip': 'GZIP 压缩流',
      'pdf': 'PDF 文档',
      'png': 'PNG 图像',
      'cocoon-bytecode': 'CocoonVM 字节码',
      'cocoon-lang': 'CocoonLang 源码',
    };
    // 文本特征：大量可打印字符
    const printable = bytes.filter(b => b >= 0x20 && b <= 0x7e || b === 0x0a || b === 0x0d).length;
    if (printable / Math.max(bytes.length, 1) > 0.85) {
      return '文本/源码';
    }
    return typeMap[magic] || '二进制数据';
  }

  _entropy(bytes) {
    if (bytes.length === 0) return 0;
    const freq = new Array(256).fill(0);
    for (const b of bytes) freq[b]++;
    let entropy = 0;
    for (const f of freq) {
      if (f === 0) continue;
      const p = f / bytes.length;
      entropy -= p * Math.log2(p);
    }
    return Math.round(entropy * 100) / 100;
  }

  _byteDistribution(bytes) {
    const dist = { low: 0, mid: 0, high: 0, zero: 0 };
    for (const b of bytes) {
      if (b === 0) dist.zero++;
      else if (b < 0x40) dist.low++;
      else if (b < 0xc0) dist.mid++;
      else dist.high++;
    }
    const total = Math.max(bytes.length, 1);
    return {
      low: Math.round(dist.low / total * 100) + '%',
      mid: Math.round(dist.mid / total * 100) + '%',
      high: Math.round(dist.high / total * 100) + '%',
      zero: Math.round(dist.zero / total * 100) + '%',
    };
  }

  _assessIntegrity(bytes) {
    if (bytes.length === 0) return 'empty';
    if (bytes.length < 16) return 'fragment';
    if (bytes.length < 256) return 'minimal';
    if (bytes.length < 65536) return 'complete';
    return 'large';
  }

  // ===============
  // 3. senseShell(shellId)
  // ===============
  // 感知某个寄生壳的适配度
  // 寄生壳 = 宿主运行环境（Node.js / 浏览器 / Deno / 嵌入式）
  senseShell(shellId) {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'shell';

    const shell = this.shells[shellId];
    if (!shell) {
      return { error: `未知壳: ${shellId}`, available: this.listShells() };
    }

    const result = {
      id: shellId,
      name: shell.name,
      description: shell.description,
      detected: this._detectShell(shellId),
      capabilities: shell.capabilities,
      // 适配度评分 0-100
      compatibility: this._scoreCompatibility(shellId, shell),
      // 性能预期
      performance: shell.performance,
      // 限制
      limitations: shell.limitations,
      // 推荐
      recommended: false,
    };
    result.recommended = result.compatibility >= 70 && result.detected;
    return result;
  }

  _initShellRegistry() {
    return {
      'node': {
        name: 'Node.js',
        description: '服务端 JavaScript 运行时',
        capabilities: ['fs', 'net', 'crypto', 'child_process', 'wasm', 'worker_threads'],
        performance: 'high',
        limitations: ['单线程主循环'],
      },
      'browser': {
        name: '浏览器',
        description: 'Web 浏览器沙箱环境',
        capabilities: ['dom', 'wasm', 'webgl', 'websocket', 'fetch', 'canvas'],
        performance: 'medium',
        limitations: ['无直接文件系统', '无原生进程', '沙箱限制'],
      },
      'deno': {
        name: 'Deno',
        description: '安全 TypeScript 运行时',
        capabilities: ['fs', 'net', 'wasm', 'permissions', 'web_api'],
        performance: 'high',
        limitations: ['权限模型'],
      },
      'bun': {
        name: 'Bun',
        description: '高性能 JavaScript 运行时',
        capabilities: ['fs', 'net', 'wasm', 'fast_start', 'ffi'],
        performance: 'very-high',
        limitations: ['生态较新'],
      },
      'embedded': {
        name: '嵌入式',
        description: '资源受限嵌入式设备',
        capabilities: ['gpio', 'i2c', 'spi', 'uart', 'low_power'],
        performance: 'low',
        limitations: ['内存受限', '无动态加载', '精简指令集'],
      },
      'wasm-standalone': {
        name: 'WASM 独立运行时',
        description: 'WebAssembly 独立运行时（wasmtime/wasmer）',
        capabilities: ['wasm', 'sandbox', 'cross-platform', 'aot'],
        performance: 'high',
        limitations: ['无直接系统调用', '需宿主导入'],
      },
    };
  }

  _detectShell(shellId) {
    switch (shellId) {
      case 'node':
        return typeof process !== 'undefined' && process.versions && process.versions.node;
      case 'browser':
        return typeof window !== 'undefined' && typeof document !== 'undefined';
      case 'deno':
        return typeof Deno !== 'undefined';
      case 'bun':
        return typeof Bun !== 'undefined';
      case 'embedded':
        return typeof process !== 'undefined' && process.platform === 'linux' && os.totalmem() < 256 * 1024 * 1024;
      case 'wasm-standalone':
        return typeof WebAssembly !== 'undefined' && typeof window === 'undefined' && !process.versions.node;
      default:
        return false;
    }
  }

  _scoreCompatibility(shellId, shell) {
    let score = 50;
    const detected = this._detectShell(shellId);
    if (detected) score += 30;
    // 环境匹配
    const env = this._cache.environment || this.senseEnvironment();
    if (env.resourceLevel === 'abundant' && shell.performance !== 'low') score += 15;
    if (env.resourceLevel === 'constrained' && shell.performance === 'low') score += 15;
    if (env.resourceLevel === 'constrained' && shell.performance === 'very-high') score -= 10;
    // 能力匹配
    if (shell.capabilities.includes('wasm')) score += 5;
    return Math.max(0, Math.min(100, score));
  }

  listShells() {
    return Object.keys(this.shells);
  }

  // ===============
  // 4. senseSystem()
  // ===============
  // 感知宿主系统的能力（GPU/网络/存储/传感器）
  senseSystem() {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'system';

    const env = this._cache.environment || this.senseEnvironment();
    const system = {
      gpu: this._senseGPU(),
      network: this._senseNetworkInterfaces(),
      storage: this._senseStorage(),
      sensors: this._senseSensors(),
      cpu: {
        cores: env.cpuCount,
        model: env.cpuModel,
        speed: env.cpuSpeed,
        features: this._cpuFeatures(),
      },
      memory: {
        total: env.totalMemory,
        free: env.freeMemory,
        usage: env.memoryUsage,
      },
      power: this._sensePower(),
    };
    this._cache.system = system;
    return system;
  }

  _senseGPU() {
    // GPU 检测：通过环境变量和运行时特征
    const gpu = { available: false, type: 'unknown', vendor: 'unknown' };
    // WebGL/WebGPU 检测（浏览器环境）
    if (typeof navigator !== 'undefined' && navigator.gpu) {
      gpu.available = true;
      gpu.type = 'webgpu';
    }
    // Node.js 环境下通过进程特征推断
    if (process.env.DISPLAY || process.env.WAYLAND_DISPLAY) {
      gpu.available = true;
      gpu.type = 'display-server';
    }
    // 检测 NVIDIA CUDA
    if (process.env.CUDA_HOME || process.env.PATH && process.env.PATH.includes('cuda')) {
      gpu.available = true;
      gpu.vendor = 'nvidia';
      gpu.type = 'cuda';
    }
    return gpu;
  }

  _senseNetworkInterfaces() {
    const ifaces = os.networkInterfaces();
    const result = [];
    for (const [name, addrs] of Object.entries(ifaces)) {
      if (!addrs) continue;
      for (const addr of addrs) {
        result.push({
          name,
          family: addr.family,
          address: addr.address,
          internal: addr.internal,
        });
      }
    }
    return { interfaces: result, count: result.length };
  }

  _senseStorage() {
    // 存储感知：当前工作目录可写性 + 临时目录
    const fs = require('fs');
    const path = require('path');
    const storage = {
      tmpdir: os.tmpdir(),
      cwdWritable: false,
      availableBytes: 0,
    };
    try {
      fs.accessSync(process.cwd(), fs.constants.W_OK);
      storage.cwdWritable = true;
    } catch (e) {
      storage.cwdWritable = false;
    }
    // 磁盘空间估算（通过 statvfs 不可用，用内存近似）
    storage.availableBytes = os.freemem();
    return storage;
  }

  _senseSensors() {
    // 传感器感知：嵌入式/移动设备上的传感器
    const sensors = {
      accelerometer: false,
      gyroscope: false,
      temperature: this._readTempSensor(),
      light: false,
      proximity: false,
    };
    // 浏览器环境检测
    if (typeof window !== 'undefined' && window.DeviceOrientationEvent) {
      sensors.accelerometer = true;
      sensors.gyroscope = true;
    }
    return sensors;
  }

  _readTempSensor() {
    // 尝试读取 Linux 温度传感器
    try {
      const fs = require('fs');
      const temp = fs.readFileSync('/sys/class/thermal/thermal_zone0/temp', 'utf8');
      return parseInt(temp) / 1000; // 毫摄氏度 → 摄氏度
    } catch (e) {
      return null;
    }
  }

  _cpuFeatures() {
    // CPU 特性推断
    const features = [];
    const arch = os.arch();
    if (arch === 'x64' || arch === 'x86') {
      features.push('sse', 'sse2', 'avx');
    }
    if (arch === 'arm64') {
      features.push('neon', 'crypto');
    }
    if (arch === 'arm') {
      features.push('neon');
    }
    return features;
  }

  _sensePower() {
    // 电源状态
    const power = { source: 'unknown', battery: null };
    if (typeof process !== 'undefined' && process.env.BATTERY_LEVEL) {
      power.battery = parseInt(process.env.BATTERY_LEVEL);
      power.source = 'battery';
    }
    // 嵌入式常为电池供电
    if (os.totalmem() < 256 * 1024 * 1024) {
      power.source = 'battery';
    } else {
      power.source = 'mains';
    }
    return power;
  }

  // ===============
  // 5. senseNetwork()
  // ===============
  // 感知网络状态
  senseNetwork() {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'network';

    const net = {
      interfaces: this._senseNetworkInterfaces(),
      online: typeof navigator !== 'undefined' ? navigator.onLine : true,
      latency: null,
      bandwidth: null,
      connectionType: 'unknown',
      dns: this._senseDNS(),
      proxy: this._senseProxy(),
    };
    // 连接类型推断
    if (net.interfaces.count === 0) {
      net.connectionType = 'offline';
    } else if (net.interfaces.interfaces.some(i => i.internal)) {
      net.connectionType = net.interfaces.interfaces.some(i => !i.internal) ? 'connected' : 'loopback-only';
    } else {
      net.connectionType = 'connected';
    }
    this._cache.network = net;
    return net;
  }

  _senseDNS() {
    // DNS 配置感知
    const dns = { servers: [] };
    try {
      const fs = require('fs');
      const resolv = fs.readFileSync('/etc/resolv.conf', 'utf8');
      const matches = resolv.match(/nameserver\s+(\S+)/g);
      if (matches) {
        dns.servers = matches.map(m => m.split(/\s+/)[1]);
      }
    } catch (e) {
      // 非 Linux 或无权限
    }
    return dns;
  }

  _senseProxy() {
    const proxy = { configured: false, http: null, https: null };
    if (process.env.HTTP_PROXY || process.env.http_proxy) {
      proxy.configured = true;
      proxy.http = process.env.HTTP_PROXY || process.env.http_proxy;
    }
    if (process.env.HTTPS_PROXY || process.env.https_proxy) {
      proxy.configured = true;
      proxy.https = process.env.HTTPS_PROXY || process.env.https_proxy;
    }
    return proxy;
  }

  // ===============
  // 6. senseSecurity()
  // ===============
  // 感知安全环境
  senseSecurity() {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'security';

    const env = this._cache.environment || this.senseEnvironment();
    const security = {
      platform: env.platform,
      isRoot: this._isRoot(),
      privileges: this._assessPrivileges(),
      sandbox: this._detectSandbox(),
      tls: this._assessTLS(),
      cryptoAvailable: typeof require('crypto') !== 'undefined',
      secureContext: process.env.NODE_ENV === 'production',
      threats: [],
      riskLevel: 'low',
      recommendations: [],
    };

    // 威胁评估
    if (security.isRoot) {
      security.threats.push('以 root 权限运行');
      security.riskLevel = 'medium';
    }
    if (env.platform === 'linux' && !security.sandbox) {
      security.threats.push('无沙箱隔离');
    }
    if (!security.cryptoAvailable) {
      security.threats.push('无加密能力');
      security.riskLevel = 'high';
    }
    if (process.env.NODE_ENV !== 'production') {
      security.threats.push('非生产环境');
    }

    // 风险级别调整
    if (security.threats.length >= 3) security.riskLevel = 'high';
    else if (security.threats.length >= 1 && security.riskLevel === 'low') security.riskLevel = 'medium';

    // 建议
    if (security.isRoot) security.recommendations.push('降权至非 root 用户运行');
    if (!security.sandbox) security.recommendations.push('启用沙箱或容器隔离');
    if (security.cryptoAvailable) security.recommendations.push('已具备加密能力，建议启用 TLS');

    this._cache.security = security;
    return security;
  }

  _isRoot() {
    try {
      return process.getuid && process.getuid() === 0;
    } catch (e) {
      return false;
    }
  }

  _assessPrivileges() {
    try {
      if (process.getuid) {
        const uid = process.getuid();
        if (uid === 0) return 'root';
        if (uid < 1000) return 'system';
        return 'user';
      }
    } catch (e) {}
    return 'unknown';
  }

  _detectSandbox() {
    // 沙箱检测
    if (process.env.SNAP) return 'snap';
    if (process.env.FLATPAK_ID) return 'flatpak';
    if (process.env.container) return 'container';
    // Docker 检测
    try {
      const fs = require('fs');
      if (fs.existsSync('/.dockerenv')) return 'docker';
      const cgroup = fs.readFileSync('/proc/1/cgroup', 'utf8');
      if (cgroup.includes('docker') || cgroup.includes('containerd')) return 'container';
    } catch (e) {}
    return null;
  }

  _assessTLS() {
    // TLS 能力评估
    return {
      available: typeof require('tls') !== 'undefined',
      minVersion: 'TLSv1.2',
      defaultCiphers: 10,
    };
  }

  // ===============
  // 7. selfSense()
  // ===============
  // 自举感应：感应器感知自己的状态
  selfSense() {
    this.selfState.sensesPerformed++;
    this.selfState.lastSense = 'self';

    const self = {
      version: this.version,
      name: 'BootstrapSensor',
      // 感应器自身元数据
      identity: {
        type: 'sensor',
        bootstrap: true,
        selfHosting: true,
      },
      // 运行状态
      runtime: {
        createdAt: this.selfState.createdAt,
        uptime: Date.now() - this.selfState.createdAt,
        sensesPerformed: this.selfState.sensesPerformed,
        lastSense: this.selfState.lastSense,
        health: this.selfState.health,
      },
      // 缓存状态
      cache: {
        keys: Object.keys(this._cache),
        size: Object.keys(this._cache).length,
        populated: Object.keys(this._cache).length > 0,
      },
      // 已知壳
      knownShells: this.listShells(),
      // 感应能力清单
      capabilities: [
        'environment', 'cocoon', 'shell', 'system',
        'network', 'security', 'self',
      ],
      // 自举验证：感应器能感知自己
      selfAware: true,
      // 反射：感知自己的方法
      methods: [
        'senseEnvironment', 'senseCocoon', 'senseShell',
        'senseSystem', 'senseNetwork', 'senseSecurity',
        'selfSense', 'recommend',
      ],
    };
    return self;
  }

  // ===============
  // 8. recommend()
  // ===============
  // 根据感知结果推荐最佳壳和配置
  recommend() {
    // 确保所有感知已完成
    const env = this._cache.environment || this.senseEnvironment();
    const system = this._cache.system || this.senseSystem();
    const security = this._cache.security || this.senseSecurity();
    const network = this._cache.network || this.senseNetwork();

    // 评分所有壳
    const scores = [];
    for (const shellId of this.listShells()) {
      const shellInfo = this.senseShell(shellId);
      scores.push({
        shell: shellId,
        name: shellInfo.name,
        score: shellInfo.compatibility,
        detected: shellInfo.detected,
        recommended: shellInfo.recommended,
      });
    }
    scores.sort((a, b) => b.score - a.score);

    // 选出最佳壳
    const best = scores[0] || null;

    // 生成配置建议
    const config = {
      recommendedShell: best ? best.shell : null,
      recommendedShellName: best ? best.name : null,
      reason: this._recommendReason(best, env),
      // 编译目标推荐
      compileTarget: this._recommendCompileTarget(env, system),
      // 优化级别推荐
      optLevel: this._recommendOptLevel(env),
      // 内存配置
      memoryConfig: this._recommendMemory(env),
      // 网络配置
      networkConfig: this._recommendNetwork(network),
      // 安全配置
      securityConfig: this._recommendSecurity(security),
      // 全部壳评分
      shellRanking: scores,
    };

    return config;
  }

  _recommendReason(best, env) {
    if (!best) return '无可用壳';
    if (best.detected) return `${best.name} 已检测到且兼容性最高 (${best.score}分)`;
    if (env.resourceLevel === 'constrained') return `资源受限，推荐轻量壳 (${best.name}, ${best.score}分)`;
    return `综合评分最高 (${best.name}, ${best.score}分)`;
  }

  _recommendCompileTarget(env, system) {
    if (system.gpu && system.gpu.type === 'cuda') return 'wasm+gpu';
    if (env.resourceLevel === 'constrained') return 'cocoon-vm';
    if (env.platform === 'linux') return 'wasm';
    return 'cocoon-vm';
  }

  _recommendOptLevel(env) {
    if (env.resourceLevel === 'abundant') return 'O2';
    if (env.resourceLevel === 'sufficient') return 'O1';
    return 'O0';
  }

  _recommendMemory(env) {
    return {
      maxStackSize: Math.min(1024, Math.floor(env.freeMemory / 1024 / 1024)),
      heapLimit: Math.floor(env.freeMemory * 0.5),
      gcEnabled: env.resourceLevel !== 'constrained',
    };
  }

  _recommendNetwork(network) {
    return {
      enableNetworking: network.connectionType === 'connected',
      useProxy: network.proxy.configured,
      tlsRequired: true,
      timeout: network.connectionType === 'connected' ? 30000 : 0,
    };
  }

  _recommendSecurity(security) {
    return {
      sandbox: !security.sandbox,
      dropPrivileges: security.isRoot,
      enforceTLS: true,
      riskLevel: security.riskLevel,
      actions: security.recommendations,
    };
  }

  // 全量感知：一次性感知所有
  senseAll() {
    return {
      environment: this.senseEnvironment(),
      cocoon: this._cache.cocoon || null,
      system: this.senseSystem(),
      network: this.senseNetwork(),
      security: this.senseSecurity(),
      self: this.selfSense(),
      recommendation: this.recommend(),
      timestamp: Date.now(),
    };
  }
}

module.exports = {
  BootstrapSensor,
};
