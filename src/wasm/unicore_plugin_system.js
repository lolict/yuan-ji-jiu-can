/**
 * UniCore v2.0 插件指令扩展系统
 * 
 * 核心功能：
 * 1. 插件加载/卸载
 * 2. 自定义指令注册
 * 3. 热插拔支持
 * 4. 插件生命周期管理
 * 5. 插件间通信
 * 
 * 底层优先，上层服从
 */

class UniCorePluginSystem {
    constructor(vm) {
        this.vm = vm;
        this.plugins = new Map();
        this.commands = new Map();
        this.nextPluginId = 0x40; // 从0x40开始，0x00-0x3F为内置指令
        this.loadedPlugins = new Set();
        this.onPluginChange = null;
    }

    /**
     * 注册插件
     */
    registerPlugin(plugin) {
        if (!plugin.id || !plugin.name || !plugin.commands) {
            throw new Error('Invalid plugin structure');
        }

        // 分配ID
        if (plugin.id === undefined) {
            plugin.id = this.nextPluginId++;
            if (plugin.id > 0xFF) {
                throw new Error('Plugin ID limit reached (256)');
            }
        }

        // 检查ID冲突
        if (this.plugins.has(plugin.id)) {
            throw new Error(`Plugin ID ${plugin.id} already exists`);
        }

        // 设置默认值
        plugin.version = plugin.version || '1.0.0';
        plugin.active = plugin.active !== undefined ? plugin.active : true;
        plugin.init = plugin.init || (() => {});
        plugin.destroy = plugin.destroy || (() => {});
        plugin.commands = plugin.commands || [];

        // 注册插件
        this.plugins.set(plugin.id, plugin);

        // 注册指令
        for (const cmd of plugin.commands) {
            this.registerCommand(plugin.id, cmd);
        }

        // 初始化插件
        if (plugin.active) {
            this.loadPlugin(plugin.id);
        }

        this.notifyChange('registered', plugin);
        return plugin.id;
    }

    /**
     * 注册指令
     */
    registerCommand(pluginId, command) {
        if (!command.opcode || !command.name || !command.execute) {
            throw new Error('Invalid command structure');
        }

        const fullOpcode = pluginId << 8 | command.opcode;
        if (this.commands.has(fullOpcode)) {
            throw new Error(`Command 0x${fullOpcode.toString(16)} already exists`);
        }

        this.commands.set(fullOpcode, {
            pluginId,
            opcode: command.opcode,
            name: command.name,
            execute: command.execute,
            description: command.description || ''
        });
    }

    /**
     * 加载插件
     */
    loadPlugin(pluginId) {
        const plugin = this.plugins.get(pluginId);
        if (!plugin) throw new Error(`Plugin ${pluginId} not found`);
        if (this.loadedPlugins.has(pluginId)) return;

        try {
            plugin.init(this.vm);
            this.loadedPlugins.add(pluginId);
            plugin.active = true;
            this.notifyChange('loaded', plugin);
        } catch (error) {
            console.error(`Failed to load plugin ${pluginId}:`, error);
            throw error;
        }
    }

    /**
     * 卸载插件
     */
    unloadPlugin(pluginId) {
        const plugin = this.plugins.get(pluginId);
        if (!plugin) throw new Error(`Plugin ${pluginId} not found`);
        if (!this.loadedPlugins.has(pluginId)) return;

        try {
            plugin.destroy(this.vm);
            this.loadedPlugins.delete(pluginId);
            plugin.active = false;
            this.notifyChange('unloaded', plugin);
        } catch (error) {
            console.error(`Failed to unload plugin ${pluginId}:`, error);
            throw error;
        }
    }

    /**
     * 执行插件指令
     */
    executeCommand(opcode, arg1, arg2, flags) {
        // 检查是否是插件指令 (0x40-0xFF)
        if (opcode < 0x40) {
            return false; // 内置指令，由VM处理
        }

        const pluginId = opcode;
        const cmdOpcode = arg1;
        const fullOpcode = pluginId << 8 | cmdOpcode;

        const command = this.commands.get(fullOpcode);
        if (!command) {
            console.warn(`Command 0x${fullOpcode.toString(16)} not found`);
            return false;
        }

        // 检查插件是否加载
        if (!this.loadedPlugins.has(pluginId)) {
            console.warn(`Plugin ${pluginId} not loaded`);
            return false;
        }

        try {
            const result = command.execute(this.vm, arg1, arg2, flags);
            this.vm.log(`Plugin command executed: ${command.name}`);
            return result !== undefined ? result : true;
        } catch (error) {
            console.error(`Command execution failed:`, error);
            return false;
        }
    }

    /**
     * 获取已注册插件列表
     */
    listPlugins() {
        const result = [];
        this.plugins.forEach((plugin, id) => {
            result.push({
                id,
                name: plugin.name,
                version: plugin.version,
                active: plugin.active,
                loaded: this.loadedPlugins.has(id),
                commandCount: plugin.commands.length
            });
        });
        return result;
    }

    /**
     * 获取已注册指令列表
     */
    listCommands() {
        const result = [];
        this.commands.forEach((cmd, opcode) => {
            result.push({
                opcode: opcode.toString(16).padStart(4, '0'),
                pluginId: cmd.pluginId,
                name: cmd.name,
                description: cmd.description
            });
        });
        return result;
    }

    /**
     * 导入插件配置
     */
    importPlugins(config) {
        for (const pluginConfig of config) {
            this.registerPlugin(pluginConfig);
        }
    }

    /**
     * 导出插件配置
     */
    exportPlugins() {
        const config = [];
        this.plugins.forEach((plugin) => {
            config.push({
                id: plugin.id,
                name: plugin.name,
                version: plugin.version,
                active: plugin.active,
                commands: plugin.commands.map(cmd => ({
                    opcode: cmd.opcode,
                    name: cmd.name,
                    description: cmd.description
                }))
            });
        });
        return config;
    }

    /**
     * 通知插件变化
     */
    notifyChange(event, plugin) {
        if (this.onPluginChange) {
            this.onPluginChange({
                event,
                plugin: {
                    id: plugin.id,
                    name: plugin.name,
                    version: plugin.version,
                    active: plugin.active
                }
            });
        }
    }

    /**
     * 热重载插件
     */
    hotReload(pluginId) {
        if (this.loadedPlugins.has(pluginId)) {
            this.unloadPlugin(pluginId);
        }
        this.loadPlugin(pluginId);
    }
}

/**
 * 插件模板
 */
class UniCorePlugin {
    constructor(options) {
        this.id = options.id;
        this.name = options.name;
        this.version = options.version || '1.0.0';
        this.active = options.active !== undefined ? options.active : true;
        this.commands = options.commands || [];
        this.init = options.init || (() => {});
        this.destroy = options.destroy || (() => {});
    }
}

/**
 * 示例插件：进制转换插件
 */
class RadixPlugin extends UniCorePlugin {
    constructor() {
        super({
            id: 0x40,
            name: 'radix',
            version: '1.0.0',
            commands: [
                {
                    opcode: 0x01,
                    name: 'TO_BASE',
                    description: '转换为指定进制',
                    execute: (vm, base, targetContainer, _flags) => {
                        if (vm.activeContainer === null) return;
                        const value = vm.containers[vm.activeContainer];
                        const result = parseInt(value.toString(base), base);
                        if (targetContainer < 256) {
                            vm.containers[targetContainer] = result & 0xFF;
                        }
                        return true;
                    }
                },
                {
                    opcode: 0x02,
                    name: 'FROM_BASE',
                    description: '从指定进制转换',
                    execute: (vm, base, _arg2, _flags) => {
                        if (vm.activeContainer === null) return;
                        const value = vm.containers[vm.activeContainer];
                        const result = parseInt(value.toString(), base);
                        vm.containers[vm.activeContainer] = result & 0xFF;
                        return true;
                    }
                }
            ]
        });
    }
}

/**
 * 示例插件：字符集插件
 */
class CharsetPlugin extends UniCorePlugin {
    constructor() {
        super({
            id: 0x41,
            name: 'charset',
            version: '1.0.0',
            commands: [
                {
                    opcode: 0x01,
                    name: 'ENCODE',
                    description: '字符编码转换',
                    execute: (vm, _arg1, _arg2, _flags) => {
                        vm.log('Charset encoding...');
                        return true;
                    }
                },
                {
                    opcode: 0x02,
                    name: 'DECODE',
                    description: '字符解码转换',
                    execute: (vm, _arg1, _arg2, _flags) => {
                        vm.log('Charset decoding...');
                        return true;
                    }
                }
            ]
        });
    }
}

/**
 * 示例插件：三维空间插件
 */
class SpatialPlugin extends UniCorePlugin {
    constructor() {
        super({
            id: 0x42,
            name: 'spatial',
            version: '1.0.0',
            commands: [
                {
                    opcode: 0x01,
                    name: 'ROTATE_X',
                    description: '绕X轴旋转',
                    execute: (vm, angle, _arg2, _flags) => {
                        vm.log(`Rotate X: ${angle} degrees`);
                        return true;
                    }
                },
                {
                    opcode: 0x02,
                    name: 'ROTATE_Y',
                    description: '绕Y轴旋转',
                    execute: (vm, angle, _arg2, _flags) => {
                        vm.log(`Rotate Y: ${angle} degrees`);
                        return true;
                    }
                },
                {
                    opcode: 0x03,
                    name: 'ROTATE_Z',
                    description: '绕Z轴旋转',
                    execute: (vm, angle, _arg2, _flags) => {
                        vm.log(`Rotate Z: ${angle} degrees`);
                        return true;
                    }
                },
                {
                    opcode: 0x04,
                    name: 'TRANSLATE',
                    description: '平移',
                    execute: (vm, dx, dy, flags) => {
                        const dz = (flags >> 4) & 0xFF;
                        vm.log(`Translate: (${dx}, ${dy}, ${dz})`);
                        return true;
                    }
                }
            ]
        });
    }
}

// 导出
window.UniCorePluginSystem = UniCorePluginSystem;
window.UniCorePlugin = UniCorePlugin;
window.RadixPlugin = RadixPlugin;
window.CharsetPlugin = CharsetPlugin;
window.SpatialPlugin = SpatialPlugin;

console.log('UniCore Plugin System loaded');
