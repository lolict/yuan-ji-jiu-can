/**
 * UniCore v2.0 完整系统核心
 * 
 * 整合：
 * - WASM底层核心 (WASM层)
 * - hacsjs_3d.js 三维引擎 (界面层)
 * - CubeTable 表格系统 (逻辑层)
 * - .3d.cbc.mll 文件格式 (存储层)
 * 
 * 指令集：
 * - 0x00-0x3F: 内置指令
 * - 0x40-0xFF: 插件指令
 * - 0xA5: 特殊系统指令
 */

class UniCoreSystem {
    constructor() {
        // 核心组件
        this.vm = null;
        this.engine3d = null;
        this.tableSystem = null;
        this.pluginSystem = null;
        this.fileHandler = null;
        
        // 状态
        this.initialized = false;
        this.running = false;
        
        // 初始化
        this.init();
    }

    init() {
        console.log('UniCore v2.0 System initializing...');
        
        // 初始化VM
        this.vm = new UniCoreWASM();
        
        // 初始化插件系统
        this.pluginSystem = new UniCorePluginSystem(this.vm);
        this.registerBuiltInPlugins();
        
        // 初始化文件处理
        this.fileHandler = new CBCFileHandler();
        
        this.initialized = true;
        console.log('UniCore v2.0 System initialized');
    }

    registerBuiltInPlugins() {
        // 表格插件
        this.pluginSystem.registerPlugin({
            id: 0xA5,
            name: 'SYS',
            version: '2.0',
            commands: [
                { opcode: 0x01, name: 'INIT_3D', execute: (vm, arg1, arg2) => this.init3D() },
                { opcode: 0x02, name: 'INIT_TABLE', execute: (vm, arg1, arg2) => this.initTable() },
                { opcode: 0x03, name: 'SYNC', execute: (vm, arg1, arg2) => this.syncAll() },
                { opcode: 0x04, name: 'EXPORT', execute: (vm, arg1, arg2) => this.exportData() },
                { opcode: 0x05, name: 'IMPORT', execute: (vm, arg1, arg2) => this.importData() }
            ]
        });

        // Radix插件
        this.pluginSystem.registerPlugin(new RadixPlugin());
        // Charset插件
        this.pluginSystem.registerPlugin(new CharsetPlugin());
        // Spatial插件
        this.pluginSystem.registerPlugin(new SpatialPlugin());
    }

    // ==================== 3D引擎 ====================

    init3D(containerId = 'container') {
        if (this.engine3d) {
            this.engine3d.destroy();
        }
        this.engine3d = initHACSJS3D(containerId);
        return this.engine3d !== null;
    }

    createContainers(count = 256) {
        if (!this.engine3d) return false;
        this.engine3d.createAllContainers(count);
        return true;
    }

    createSpatialArray(type) {
        if (!this.engine3d) return false;
        this.engine3d.createSpatialArray(type);
        return true;
    }

    createNodeChain(nodes) {
        if (!this.engine3d) return false;
        this.engine3d.createNodeChain(nodes);
        return true;
    }

    // ==================== 表格系统 ====================

    initTable() {
        if (!this.tableSystem) {
            this.tableSystem = {
                containers: [],
                mapper: new InsideOutsideMapper()
            };
        }
        return true;
    }

    createTable(options) {
        if (!this.tableSystem) this.initTable();
        const table = new CubeTable(options);
        this.tableSystem.containers.push(table);
        return table;
    }

    // ==================== 数据同步 ====================

    syncAll() {
        const state = {
            timestamp: Date.now(),
            vm: {
                cycles: this.vm.cycles,
                containers: this.vm.containers ? Array.from(this.vm.containers) : [],
                nodes: this.vm.nodes ? Array.from(this.vm.nodes.entries()) : [],
                activeContainer: this.vm.activeContainer
            },
            tables: this.tableSystem ? this.tableSystem.containers.map(t => t.export()) : [],
            plugins: this.pluginSystem.listPlugins()
        };
        return state;
    }

    // ==================== 文件导入导出 ====================

    exportData() {
        const state = this.syncAll();
        return JSON.stringify(state, null, 2);
    }

    importData(jsonString) {
        try {
            const state = JSON.parse(jsonString);
            
            // 恢复VM状态
            if (state.vm) {
                if (state.vm.containers) {
                    this.vm.containers = new Uint8Array(state.vm.containers);
                }
                if (state.vm.nodes) {
                    this.vm.nodes = new Map(state.vm.nodes);
                }
            }
            
            // 恢复表格
            if (state.tables) {
                if (!this.tableSystem) this.initTable();
                this.tableSystem.containers = state.tables.map(t => CubeTable.import(t));
            }
            
            return true;
        } catch (e) {
            console.error('Import failed:', e);
            return false;
        }
    }

    // ==================== 执行控制 ====================

    run(cycles = 1000) {
        if (!this.initialized) return false;
        this.running = true;
        this.vm.initialize();
        this.vm.run(cycles);
        return this.vm.output;
    }

    step() {
        if (!this.running) {
            this.vm.initialize();
            this.running = true;
        }
        return this.vm.step();
    }

    reset() {
        this.vm = new UniCoreWASM();
        this.running = false;
        if (this.engine3d) {
            this.engine3d.clearAll();
        }
    }

    // ==================== 指令执行 ====================

    executeInstruction(opcode, arg1 = 0, arg2 = 0, flags = 0) {
        // 内置指令
        if (opcode < 0x40) {
            return this.executeBuiltIn(opcode, arg1, arg2, flags);
        }
        
        // 插件指令
        return this.pluginSystem.executeCommand(opcode, arg1, arg2, flags);
    }

    executeBuiltIn(opcode, arg1, arg2, flags) {
        switch (opcode) {
            case 0x00: return true; // NOP
            case 0x01: this.running = false; return true; // HALT
            case 0x02: this.vm.activeContainer = arg1; return true; // LOAD
            case 0x10: if (this.vm.activeContainer !== null) this.vm.nonCarryInc(this.vm.activeContainer); return true; // INC
            case 0x11: if (this.vm.activeContainer !== null) this.vm.nonCarryDec(this.vm.activeContainer); return true; // DEC
            case 0xA5: return this.executeSysCommand(arg1, arg2); // SYS
            default: return false;
        }
    }

    executeSysCommand(subCmd, arg) {
        switch (subCmd) {
            case 0x01: return this.init3D();
            case 0x02: return this.initTable();
            case 0x03: return this.syncAll();
            case 0x04: return this.exportData();
            case 0x05: return true;
            default: return false;
        }
    }

    // ==================== 状态查询 ====================

    getStatus() {
        return {
            initialized: this.initialized,
            running: this.running,
            cycles: this.vm.cycles,
            containerCount: this.vm.containers ? this.vm.containers.filter(v => v !== 0).length : 0,
            nodeCount: this.vm.nodes ? this.vm.nodes.size : 0,
            tableCount: this.tableSystem ? this.tableSystem.containers.length : 0,
            pluginCount: this.pluginSystem.listPlugins().length
        };
    }
}

/**
 * CBC文件处理器
 */
class CBCFileHandler {
    constructor() {
        this.reader = new CBCFileReader();
        this.writer = new CBCFileWriter();
    }

    save(data, filename = 'data.3d.cbc.mll') {
        const buffer = this.writer.write(data);
        const blob = new Blob([buffer], { type: 'application/octet-stream' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        a.click();
        URL.revokeObjectURL(url);
    }

    load() {
        return new Promise((resolve, reject) => {
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = '.3d.cbc.mll,.cbc,.json';
            input.onchange = (e) => {
                const file = e.target.files[0];
                if (!file) {
                    reject(new Error('No file selected'));
                    return;
                }
                const reader = new FileReader();
                reader.onload = (e) => resolve(e.target.result);
                reader.onerror = (e) => reject(e.error);
                reader.readAsArrayBuffer(file);
            };
            input.click();
        });
    }

    parse(data) {
        try {
            return this.reader.read(data);
        } catch (e) {
            console.error('Parse error:', e);
            return null;
        }
    }
}

// ==================== 0xA5特殊指令系统 ====================

const SYS_COMMANDS = {
    0x00: 'PING',
    0x01: 'INIT_3D',
    0x02: 'INIT_TABLE',
    0x03: 'SYNC',
    0x04: 'EXPORT',
    0x05: 'IMPORT',
    0x10: 'CREATE_CONTAINER',
    0x11: 'DELETE_CONTAINER',
    0x12: 'CREATE_NODE',
    0x13: 'CREATE_TABLE',
    0x14: 'SET_CONTENT',
    0x15: 'AUTO_FILL_ZERO'
};

function execute0xA5(system, subCmd, arg1 = 0, arg2 = 0) {
    const cmd = SYS_COMMANDS[subCmd] || 'UNKNOWN';
    console.log(`[0xA5] Executing: ${cmd}`);

    switch (subCmd) {
        case 0x00: return { ok: true, cmd: 'PING' };
        case 0x01: return { ok: system.init3D(), cmd: 'INIT_3D' };
        case 0x02: return { ok: system.initTable(), cmd: 'INIT_TABLE' };
        case 0x03: return { ok: true, data: system.syncAll(), cmd: 'SYNC' };
        case 0x04: return { ok: true, data: system.exportData(), cmd: 'EXPORT' };
        case 0x10: return { ok: system.createContainers(arg1 || 256), cmd: 'CREATE_CONTAINER' };
        case 0x12: return { ok: system.createNodeChain(arg1 ? [{ id: 1, type: 1, cause: null }] : []), cmd: 'CREATE_NODE' };
        case 0x13: return { ok: true, table: system.createTable({ title: 'New Table' }), cmd: 'CREATE_TABLE' };
        default: return { ok: false, cmd: 'UNKNOWN' };
    }
}

// 导出
window.UniCoreSystem = UniCoreSystem;
window.CBCFileHandler = CBCFileHandler;
window.SYS_COMMANDS = SYS_COMMANDS;
window.execute0xA5 = execute0xA5;

console.log('UniCore System Core loaded');
