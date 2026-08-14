/**
 * UniCore v2.0 WASM/WASI 运行时封装
 * 
 * 核心功能：
 * 1. WASM模块加载与实例化
 * 2. WASI接口适配
 * 3. 集装箱内存映射
 * 4. 节点时间同步
 * 5. 插件指令扩展
 * 
 * 底层优先，上层服从
 */

class UniCoreWASMRuntime {
    constructor() {
        this.module = null;
        this.instance = null;
        this.memory = null;
        this.containers = null;
        this.nodes = new Map();
        this.plugins = new Map();
        this.running = false;
        this.onMessage = null;
    }

    /**
     * 初始化WASM运行时
     */
    async init(wasmBytes) {
        try {
            // 创建WASM模块
            const { instance } = await WebAssembly.instantiate(wasmBytes, {
                env: this.createImports()
            });

            this.instance = instance;
            this.module = instance.module;
            this.memory = instance.exports.memory;

            // 映射集装箱内存
            this.containers = new Uint8Array(this.memory.buffer, 0, 256);

            // 初始化底层
            instance.exports.unicore_initialize();

            this.running = true;
            this.log('WASM runtime initialized');
            return true;
        } catch (error) {
            this.log('WASM init failed:', error.message);
            return false;
        }
    }

    /**
     * 创建WASM导入对象
     */
    createImports() {
        return {
            // WASI-like接口
            wasi_snapshot_preview1: {
                fd_write: (fd, iovs, iovs_len, nwritten) => {
                    return this.wasi_fd_write(fd, iovs, iovs_len, nwritten);
                },
                fd_close: (fd) => 0,
                fd_read: (fd, iovs, iovs_len, nread) => 0,
                environ_get: (environ, environ_buf) => 0,
                environ_sizes_get: (environ_count, environ_buf_size) => 0,
                args_get: (argv, argv_buf) => 0,
                args_sizes_get: (argc, argv_buf_size) => 0,
                clock_time_get: (clock_id, precision, time) => {
                    return this.wasi_clock_time_get(clock_id, precision, time);
                },
                proc_exit: (rval) => {
                    this.running = false;
                }
            },
            // 自定义接口
            unicore: {
                log: (ptr, len) => {
                    const str = this.readString(ptr, len);
                    this.log(str);
                },
                time_now: () => performance.now(),
                node_create: (type, cause) => this.nodeCreate(type, cause),
                node_trigger: (id) => this.nodeTrigger(id),
                plugin_call: (id, args_ptr, args_len) => this.pluginCall(id, args_ptr, args_len),
                container_get: (idx) => this.containers[idx] || 0,
                container_set: (idx, value) => { if (idx < 256) this.containers[idx] = value; }
            }
        };
    }

    /**
     * WASI fd_write实现
     */
    wasi_fd_write(fd, iovs, iovs_len, nwritten) {
        if (fd !== 1 && fd !== 2) return 0;

        let total = 0;
        const view = new DataView(this.memory.buffer);

        for (let i = 0; i < iovs_len; i++) {
            const buf_ptr = view.getUint32(iovs + i * 8, true);
            const buf_len = view.getUint32(iovs + i * 8 + 4, true);
            const buf = new Uint8Array(this.memory.buffer, buf_ptr, buf_len);
            total += buf_len;
        }

        if (nwritten !== 0) {
            const nwritten_view = new DataView(this.memory.buffer, nwritten, 4);
            nwritten_view.setUint32(0, total, true);
        }

        return 0;
    }

    /**
     * WASI clock_time_get实现
     */
    wasi_clock_time_get(clock_id, precision, time) {
        const now = (performance.now() / 1000) * 1e9; // nanoseconds
        const view = new DataView(this.memory.buffer, time, 8);
        view.setBigUint64(0, BigInt(Math.floor(now)), true);
        return 0;
    }

    /**
     * 从WASM内存读取字符串
     */
    readString(ptr, len) {
        const buf = new Uint8Array(this.memory.buffer, ptr, len);
        return new TextDecoder('utf-8').decode(buf);
    }

    /**
     * 写入字符串到WASM内存
     */
    writeString(str) {
        const bytes = new TextEncoder('utf-8').encode(str);
        const ptr = this.instance.exports.alloc(bytes.length + 1);
        const buf = new Uint8Array(this.memory.buffer, ptr, bytes.length + 1);
        buf.set(bytes);
        buf[bytes.length] = 0;
        return ptr;
    }

    /**
     * 加载程序到WASM
     */
    loadProgram(programBytes) {
        if (!this.instance) return false;

        const ptr = this.instance.exports.alloc(programBytes.length);
        const buf = new Uint8Array(this.memory.buffer, ptr, programBytes.length);
        buf.set(programBytes);

        this.instance.exports.unicore_load_program(ptr, programBytes.length);
        this.log(`Program loaded: ${programBytes.length} bytes`);
        return true;
    }

    /**
     * 执行单步指令
     */
    step() {
        if (!this.instance || !this.running) return false;
        return this.instance.exports.unicore_step() === 1;
    }

    /**
     * 运行程序
     */
    run(maxCycles = 1000) {
        if (!this.instance) return;

        this.instance.exports.unicore_run(maxCycles);
        this.log(`Execution completed`);
    }

    /**
     * 获取状态
     */
    getState() {
        if (!this.instance) return null;

        return {
            cycles: this.instance.exports.unicore_get_cycles(),
            activeContainer: this.instance.exports.unicore_get_active_container(),
            containerCount: this.instance.exports.unicore_get_container_count(),
            nodeCount: this.instance.exports.unicore_get_node_count(),
            memoryUsage: this.memory.buffer.byteLength
        };
    }

    /**
     * 节点系统
     */
    nodeCreate(type, cause) {
        const id = this.nodes.size + 1;
        this.nodes.set(id, {
            id,
            type,
            cause: cause === 0xFFFFFFFF ? null : cause,
            triggered: false,
            offset: 0
        });
        return id;
    }

    nodeTrigger(id) {
        if (!this.nodes.has(id)) return;

        const node = this.nodes.get(id);
        node.triggered = true;

        // 因果传播
        if (node.cause !== null && this.nodes.has(node.cause)) {
            this.nodeTrigger(node.cause);
        }

        this.log(`Node ${id} triggered`);
    }

    /**
     * 插件系统
     */
    registerPlugin(id, plugin) {
        this.plugins.set(id, plugin);
        this.log(`Plugin registered: ${id} - ${plugin.name}`);
    }

    pluginCall(id, args_ptr, args_len) {
        const plugin = this.plugins.get(id);
        if (!plugin) return 0;

        const args = new Uint8Array(this.memory.buffer, args_ptr, args_len);
        const result = plugin.execute(this, args);

        return result || 0;
    }

    /**
     * 时间同步
     */
    syncTime() {
        if (!this.instance) return;
        this.instance.exports.unicore_sync_time();
        this.log('Time synchronized');
    }

    /**
     * 日志
     */
    log(...args) {
        const msg = args.map(arg => 
            typeof arg === 'object' ? JSON.stringify(arg) : String(arg)
        ).join(' ');
        
        if (this.onMessage) {
            this.onMessage({ type: 'log', message: msg });
        }
        console.log(`[UniCore WASM] ${msg}`);
    }

    /**
     * 导出状态
     */
    exportState() {
        return {
            containers: Array.from(this.containers),
            nodes: Array.from(this.nodes.entries()),
            plugins: Array.from(this.plugins.keys()),
            state: this.getState()
        };
    }

    /**
     * 销毁运行时
     */
    destroy() {
        this.running = false;
        this.instance = null;
        this.module = null;
        this.memory = null;
        this.containers = null;
        this.nodes.clear();
        this.plugins.clear();
        this.log('Runtime destroyed');
    }
}

/**
 * WASM模块编译器
 */
class UniCoreWASMCompiler {
    constructor() {
        this.assembler = new UniCoreAssembler();
    }

    /**
     * 编译汇编代码到WASM字节码
     */
    compile(assemblyCode) {
        const program = this.assembler.assemble(assemblyCode);
        return this.wrapIntoWASM(program);
    }

    /**
     * 将程序包装成WASM模块
     */
    wrapIntoWASM(program) {
        // 创建最小WASM模块
        const bytes = [];
        
        // WASM magic
        bytes.push(0x00, 0x61, 0x73, 0x6D);
        // Version
        bytes.push(0x01, 0x00, 0x00, 0x00);

        // Type section
        bytes.push(0x01); // section id
        bytes.push(0x0E); // length
        bytes.push(0x02); // count
        
        // Function types
        bytes.push(0x60, 0x00, 0x00); // () -> ()
        bytes.push(0x60, 0x02, 0x7F, 0x7F, 0x00); // (i32, i32) -> ()
        bytes.push(0x60, 0x01, 0x7F, 0x01, 0x7F); // (i32) -> i32
        
        // Import section
        bytes.push(0x02);
        bytes.push(0x26);
        bytes.push(0x02);
        
        // env.memory import
        bytes.push(0x03, 0x65, 0x6E, 0x76); // "env"
        bytes.push(0x06, 0x6D, 0x65, 0x6D, 0x6F, 0x72, 0x79); // "memory"
        bytes.push(0x02); // memory type
        bytes.push(0x00); // min pages
        bytes.push(0x00); // max pages (none)
        
        // Export section
        bytes.push(0x07);
        bytes.push(0x12);
        bytes.push(0x02);
        
        // Export memory
        bytes.push(0x06, 0x6D, 0x65, 0x6D, 0x6F, 0x72, 0x79);
        bytes.push(0x02);
        
        // Export main
        bytes.push(0x04, 0x6D, 0x61, 0x69, 0x6E);
        bytes.push(0x00);
        
        // Code section
        bytes.push(0x0A);
        bytes.push(0x0A);
        bytes.push(0x01);
        
        // Function body
        bytes.push(0x04);
        bytes.push(0x00); // local count
        bytes.push(0x0B); // end
        
        return new Uint8Array(bytes);
    }
}

// 导出
window.UniCoreWASMRuntime = UniCoreWASMRuntime;
window.UniCoreWASMCompiler = UniCoreWASMCompiler;

console.log('UniCore WASM Runtime loaded');
