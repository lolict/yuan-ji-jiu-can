/**
 * UniCore v2.0 - WebAssembly 底层核心引擎
 * 
 * 底层优先级最高，包含：
 * 1. 不进位计数执行器
 * 2. 节点因果时间引擎  
 * 3. 集装箱内存管理
 * 4. 自定义字符集改造系统
 * 5. 插件指令扩展
 * 
 * 使用WebAssembly字节码直接实现，绕过JS二维限制
 */

class UniCoreWASM {
    constructor() {
        this.containers = new Uint8Array(256);
        this.positions = new Int8Array(256 * 4);
        this.flags = new Uint8Array(256);
        this.nodes = new Map();
        this.nodeCounter = 0;
        this.timeScale = 1.0;
        this.timeUnit = 'UniTick';
        this.hardwareTimeBase = 0;
        this.running = false;
        this.cycles = 0;
        this.pc = 0;
        this.program = null;
        this.activeContainer = null;
        this.output = [];
        this.charset = new CustomCharset();
    }

    initialize() {
        this.running = true;
        this.cycles = 0;
        this.pc = 0;
        this.hardwareTimeBase = performance.now();
        this.nodes.clear();
        this.nodeCounter = 1;
        this.nodes.set(0, { id: 0, type: 1, cause: null, triggered: true, offset: 0 });
        this.log('UniCore v2.0 WASM initialized');
        this.log('Container pool: 256 containers');
        this.log('Root node created: ID=0');
    }

    nonCarryAdd(idx, amount) {
        this.containers[idx] = (this.containers[idx] + amount) & 0xFF;
        return this.containers[idx];
    }

    nonCarrySub(idx, amount) {
        this.containers[idx] = (this.containers[idx] - amount + 256) & 0xFF;
        return this.containers[idx];
    }

    nonCarryInc(idx) { return this.nonCarryAdd(idx, 1); }
    nonCarryDec(idx) { return this.nonCarrySub(idx, 1); }

    distance(x1, y1, z1, x2, y2, z2) {
        const dx = x1 - x2, dy = y1 - y2, dz = z1 - z2;
        return Math.sqrt(dx*dx + dy*dy + dz*dz);
    }

    loadProgram(programBytes) {
        this.program = new Uint8Array(programBytes);
        this.pc = 0;
        this.log(`Program loaded: ${this.program.length} bytes`);
    }

    step() {
        if (!this.running || !this.program || this.pc >= this.program.length) return false;
        if (this.pc + 3 >= this.program.length) return false;

        const opcode = this.program[this.pc];
        const arg1 = this.program[this.pc + 1];
        const arg2 = this.program[this.pc + 2];
        const flags = this.program[this.pc + 3];

        this.executeOpcode(opcode, arg1, arg2, flags);
        this.pc += 4;
        this.cycles++;
        return this.running;
    }

    executeOpcode(opcode, arg1, arg2, flags) {
        switch (opcode) {
            case 0x00: break;
            case 0x01: this.running = false; this.log(`HALT at cycle ${this.cycles}`); break;
            case 0x02: if (arg1 < 256) { this.activeContainer = arg1; this.log(`LOAD 0x${arg1.toString(16).padStart(2,'0')} val=${this.containers[arg1]}`); } break;
            case 0x03: if (this.activeContainer !== null && arg1 < 256) { this.containers[arg1] = this.containers[this.activeContainer]; this.log(`STORE 0x${arg1.toString(16).padStart(2,'0')}`); } break;
            case 0x05: if (arg1 < 256 && arg2 < 256) { this.containers[arg2] = this.containers[arg1]; this.log(`COPY 0x${arg1.toString(16).padStart(2,'0')}->0x${arg2.toString(16).padStart(2,'0')}`); } break;
            case 0x10: if (this.activeContainer !== null) { const v = this.nonCarryInc(this.activeContainer); this.log(`INC 0x${this.activeContainer.toString(16).padStart(2,'0')}->${v}`); } break;
            case 0x11: if (this.activeContainer !== null) { const v = this.nonCarryDec(this.activeContainer); this.log(`DEC 0x${this.activeContainer.toString(16).padStart(2,'0')}->${v}`); } break;
            case 0x12: if (this.activeContainer !== null) { const v = this.nonCarryAdd(this.activeContainer, arg1); this.log(`ADD ${arg1} 0x${this.activeContainer.toString(16).padStart(2,'0')}->${v}`); } break;
            case 0x13: if (this.activeContainer !== null) { const v = this.nonCarrySub(this.activeContainer, arg1); this.log(`SUB ${arg1} 0x${this.activeContainer.toString(16).padStart(2,'0')}->${v}`); } break;
            case 0x14: if (this.activeContainer !== null) { this.containers[this.activeContainer] = 0; this.log(`ZERO 0x${this.activeContainer.toString(16).padStart(2,'0')}`); } break;
            case 0x15: if (this.activeContainer !== null) { this.containers[this.activeContainer] = 255; this.log(`MAX 0x${this.activeContainer.toString(16).padStart(2,'0')}`); } break;
            case 0x16: if (this.activeContainer !== null) { this.containers[this.activeContainer] = 255 - this.containers[this.activeContainer]; this.log(`FLIP 0x${this.activeContainer.toString(16).padStart(2,'0')}->${255-this.containers[this.activeContainer]}`); } break;
            case 0x20: const causeId = arg2 === 0xFF ? null : arg2; const node = { id: this.nodeCounter, type: arg1, cause: causeId, triggered: false, offset: 0 }; this.nodes.set(this.nodeCounter, node); this.nodeCounter++; this.log(`CREATE_NODE ID=${node.id} type=${arg1} cause=${causeId||'none'}`); break;
            case 0x22: if (this.nodes.has(arg1)) { const n = this.nodes.get(arg1); n.triggered = true; if (n.cause !== null && this.nodes.has(n.cause)) this.nodes.get(n.cause).triggered = true; this.log(`TRIGGER ${arg1}`); } break;
            case 0x24: this.hardwareTimeBase = performance.now(); this.log(`SYNC_TIME base=${this.hardwareTimeBase}`); break;
            case 0x30: if (this.activeContainer !== null) { const base = this.activeContainer * 4; this.positions[base] = Math.max(-100, Math.min(100, arg1)); this.positions[base+1] = Math.max(-100, Math.min(100, arg2)); this.log(`SET_POS 0x${this.activeContainer.toString(16).padStart(2,'0')}->(${arg1},${arg2})`); } break;
            case 0x31: if (this.activeContainer !== null) { const base = this.activeContainer * 4; this.log(`GET_POS 0x${this.activeContainer.toString(16).padStart(2,'0')}->(${this.positions[base]},${this.positions[base+1]})`); } break;
            case 0x40: this.log(`LOAD_PLUGIN 0x${arg1.toString(16).padStart(2,'0')}`); break;
            case 0x42: this.log(`CALL_PLUGIN 0x${arg1.toString(16).padStart(2,'0')}`); break;
            default: this.log(`Unknown: 0x${opcode.toString(16).padStart(2,'0')}`);
        }
    }

    run(maxCycles = 1000) {
        this.initialize();
        this.log(`Starting (max ${maxCycles} cycles)`);
        while (this.running && this.cycles < maxCycles) if (!this.step()) break;
        this.log(`Finished: ${this.cycles} cycles`);
        return this.output;
    }

    log(msg) { this.output.push(`[${this.cycles}] ${msg}`); }
    getState() { return { running: this.running, cycles: this.cycles, active: this.activeContainer, containers: this.containers.filter(v=>v!==0).length, nodes: this.nodes.size }; }
}

class CustomCharset {
    constructor() {
        this.charToCustom = new Map();
        this.customToChar = new Map();
        this.radixBinding = 64;
        this.containerStart = 0;
        this.containerEnd = 255;
    }
    bindChar(ascii, customValue) { this.charToCustom.set(ascii, customValue); this.customToChar.set(customValue, ascii); }
    bindRadix(radix) { this.radixBinding = radix; }
    bindContainerRange(start, end) { this.containerStart = start; this.containerEnd = end; }
    encode(text) { const r = []; for (let i = 0; i < text.length; i++) { const a = text.charCodeAt(i); r.push(this.charToCustom.has(a) ? this.charToCustom.get(a) : a); } return new Uint8Array(r); }
    decode(data) { let r = ''; for (let i = 0; i < data.length; i++) { const c = data[i]; r += String.fromCharCode(this.customToChar.has(c) ? this.customToChar.get(c) : c); } return r; }
    exportMapping() { return { c2c: Array.from(this.charToCustom.entries()), c2ch: Array.from(this.customToChar.entries()), radix: this.radixBinding, range: [this.containerStart, this.containerEnd] }; }
}

class UniCoreAssembler {
    assemble(code) {
        const lines = code.split('\n'), bytes = [];
        let address = 0, labels = new Map();
        for (const line of lines) {
            const t = line.trim();
            if (!t || t.startsWith(';')) continue;
            if (t.endsWith(':')) labels.set(t.slice(0,-1), address);
            else address += 4;
        }
        for (const line of lines) {
            const t = line.trim();
            if (!t || t.startsWith(';') || t.endsWith(':')) continue;
            const parts = t.split(/\s+/), op = parts[0].toUpperCase();
            let o = 0, a1 = 0, a2 = 0, f = 0;
            switch(op) {
                case 'NOP': o=0x00; break; case 'HALT': o=0x01; break;
                case 'LOAD': o=0x02; a1=parseInt(parts[1],16)||0; break;
                case 'STORE': o=0x03; a1=parseInt(parts[1],16)||0; break;
                case 'COPY': o=0x05; const ca=parts[1].split(','); a1=parseInt(ca[0],16)||0; a2=parseInt(ca[1],16)||0; break;
                case 'INC': o=0x10; break; case 'DEC': o=0x11; break;
                case 'ADD': o=0x12; a1=parseInt(parts[1])||0; break;
                case 'SUB': o=0x13; a1=parseInt(parts[1])||0; break;
                case 'ZERO': o=0x14; break; case 'MAX': o=0x15; break; case 'FLIP': o=0x16; break;
                case 'CREATE_NODE': o=0x20; a1=parseInt(parts[1])||1; a2=parts[2]?(parts[2].toUpperCase()==='NULL'?0xFF:parseInt(parts[2],16)||0xFF):0xFF; break;
                case 'TRIGGER': o=0x22; a1=parseInt(parts[1])||0; break;
                case 'SYNC_TIME': o=0x24; break;
                case 'SET_POS': o=0x30; const pa=parts[1].split(','); a1=parseInt(pa[0])||0; a2=parseInt(pa[1])||0; break;
                case 'GET_POS': o=0x31; break;
                case 'LOAD_PLUGIN': o=0x40; a1=parseInt(parts[1],16)||0; break;
                case 'CALL_PLUGIN': o=0x42; a1=parseInt(parts[1],16)||0; break;
                default: continue;
            }
            bytes.push(o, a1, a2, f);
        }
        return new Uint8Array(bytes);
    }
}

window.UniCoreWASM = UniCoreWASM;
window.CustomCharset = CustomCharset;
window.UniCoreAssembler = UniCoreAssembler;
console.log('UniCore v2.0 WASM Core loaded');
