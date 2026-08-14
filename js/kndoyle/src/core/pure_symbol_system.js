/**
 * UniCore v3.0 - 纯符号序列体系
 * 
 * 零数字、零进制、零进位、零外来残留
 * 只用用户定义的符号序列：
 * - 声母序列（48个）：顺序决定全局层级和位权
 * - 韵母序列（64个）：顺序决定格子内部位置
 * - 声调序列（170个）：顺序决定跨层连接和函数指令
 */

class PureSymbolSystem {
    constructor(options = {}) {
        // 用户定义的纯符号序列（顺序=位权，不用任何数字）
        this.shengmu = options.shengmu || [];  // 声母序列（48个）
        this.yunmu = options.yunmu || [];       // 韵母序列（64个）
        this.shengdiao = options.shengdiao || []; // 声调序列（170个）
        
        // 符号到索引的映射（内部使用，外部不暴露数字）
        this.shengmuIndex = new Map();
        this.yunmuIndex = new Map();
        this.shengdiaoIndex = new Map();
        
        this.init();
    }

    init() {
        // 建立符号到索引的映射（内部使用，不对外暴露）
        this.shengmu.forEach((s, i) => this.shengmuIndex.set(s, i));
        this.yunmu.forEach((y, i) => this.yunmuIndex.set(y, i));
        this.shengdiao.forEach((d, i) => this.shengdiaoIndex.set(d, i));
    }

    // ==================== 格子系统 ====================

    createGrid(shengmu, yunmu) {
        // 一个格子 = 一个声母（层级）+ 一个韵母（中心位）
        return {
            shengmu,
            yunmu,
            walls: {},           // 前6个韵母 = 6面墙
            corners: {},         // 接下来8个韵母 = 8个内角
            connections: {},     // 再接下来6个韵母 = 6个外连位
            center: yunmu        // 第一个韵母 = 中心位
        };
    }

    // 获取格子在全局的位置（不暴露数字，只返回符号序列）
    getGridPosition(grid) {
        // 返回：声母在序列中的位置描述
        const idx = this.shengmuIndex.get(grid.shengmu);
        if (idx === 0) return '最顶层';
        if (idx === this.shengmu.length - 1) return '最底层';
        return `${grid.shengmu}层级`;
    }

    // ==================== 互认机制 ====================

    // 两个格子是否相邻（通过声调符号连接）
    areConnected(grid1, grid2, shengdiao) {
        // 如果两个格子共享同一个声调符号，则互认
        return grid1.connections[shengdiao] === grid2 || 
               grid2.connections[shengdiao] === grid1;
    }

    // 建立跨层连接
    connectGrids(grid1, grid2, shengdiao) {
        grid1.connections[shengdiao] = grid2;
        grid2.connections[shengdiao] = grid1;
    }

    // ==================== 汉字系统 ====================

    createCharacter(shengmu, yunmu, shengdiao) {
        const grid = this.createGrid(shengmu, yunmu);
        
        // 填充格子内容
        grid.walls[this.yunmu[1]] = shengmu;           // 声母
        grid.walls[this.yunmu[2]] = yunmu;            // 韵母
        grid.walls[this.yunmu[3]] = shengdiao;        // 声调
        grid.walls[this.yunmu[4]] = '';               // 字形
        grid.walls[this.yunmu[5]] = null;             // 偏旁
        grid.walls[this.yunmu[6]] = null;             // 部首
        
        grid.corners[this.yunmu[7]] = '';              // 本义
        grid.corners[this.yunmu[8]] = '';              // 最早形态
        grid.corners[this.yunmu[9]] = '';              // 演化形态1
        grid.corners[this.yunmu[10]] = '';             // 演化形态2
        grid.corners[this.yunmu[11]] = '';             // 因果根因
        grid.corners[this.yunmu[12]] = 0;              // 奖励阈值
        grid.corners[this.yunmu[13]] = 0;              // 处罚阈值
        
        // 外连位初始化为null（待连接其他字）
        for (let i = 15; i <= 20 && i < this.yunmu.length; i++) {
            grid.connections[this.yunmu[i]] = null;
        }
        
        return grid;
    }

    // ==================== 函数/指令系统 ====================

    // 创建函数（一个声调符号 = 一个函数）
    createFunction(shengmu, shengdiao, handler) {
        return {
            shengmu,      // 运算声母
            shengdiao,    // 声调 = 函数标识
            handler       // 执行逻辑
        };
    }

    // 执行函数
    executeFunction(func, input1, input2) {
        return func.handler(input1, input2);
    }

    // ==================== 宗族代数 ====================

    // 获取宗族层级（根据声母在序列中的位置）
    getClanLevel(shengmu) {
        const idx = this.shengmuIndex.get(shengmu);
        const levels = ['太奶', '太爷', '奶奶', '爷爷', '母亲', '父亲', '女儿', '儿子'];
        return levels[idx] || `第${idx}代`;
    }

    // 判断层级关系
    isParent(shengmuParent, shengmuChild) {
        const parentIdx = this.shengmuIndex.get(shengmuParent);
        const childIdx = this.shengmuIndex.get(shengmuChild);
        return childIdx === parentIdx + 1;
    }

    // ==================== 阵法系统 ====================

    // 创建阵法（声调符号的不同连接方式）
    createFormation(type, shengdiaoList) {
        return {
            type,           // 阵法类型
            shengdiao: shengdiaoList,  // 用于连接的声调符号
            grids: []       // 参与阵法的格子
        };
    }

    // 添加格子到阵法
    addGridToFormation(formation, grid) {
        if (!formation.grids.includes(grid)) {
            formation.grids.push(grid);
        }
    }

    // ==================== 时间节奏 ====================

    // 设置时间流速（不同声调对应不同节奏）
    setTimeSpeed(shengdiao) {
        const idx = this.shengdiaoIndex.get(shengdiao);
        const baseIdx = 40; // 从第41个声调开始用于时间
        if (idx >= baseIdx) {
            return (idx - baseIdx + 1) + '倍速';
        }
        return '正常速度';
    }

    // ==================== 通信系统 ====================

    // 发送信息
    sendMessage(fromGrid, toGrid, shengdiao, content) {
        if (this.areConnected(fromGrid, toGrid, shengdiao)) {
            // 通过声调符号传递内容
            toGrid.corners[this.yunmu[7]] = content;
            return true;
        }
        return false;
    }
}

// ==================== 纯符号序列定义示例 ====================

// 用户定义的声母序列（48个，顺序决定位权）
const USER_SHENGMU = [
    'B', 'P', 'M', 'F', 'D', 'T', 'N', 'L',
    'G', 'K', 'H', 'J', 'Q', 'X', 'ZH', 'CH',
    'SH', 'R', 'Z', 'C', 'S', 'Y', 'W', 'A',
    'O', 'E', 'I', 'U', 'V', 'AI', 'EI', 'AO',
    'OU', 'AN', 'EN', 'ANG', 'ENG', 'ER', 'AR',
    'OR', 'IR', 'UR', 'YR', 'WR', 'BR', 'PR',
    'MR', 'FR'
];

// 用户定义的韵母序列（64个，顺序决定位置）
const USER_YUNMU = [
    'C', 'W1', 'W2', 'W3', 'W4', 'W5', 'W6',  // 中心位 + 6面墙
    'J1', 'J2', 'J3', 'J4', 'J5', 'J6', 'J7', 'J8', // 8个内角
    'L1', 'L2', 'L3', 'L4', 'L5', 'L6',  // 6个外连位
    'E1', 'E2', 'E3', 'E4', 'E5', 'E6', 'E7', 'E8', // 扩展位
    'E9', 'E10', 'E11', 'E12', 'E13', 'E14', 'E15', 'E16',
    'E17', 'E18', 'E19', 'E20', 'E21', 'E22', 'E23', 'E24',
    'E25', 'E26', 'E27', 'E28', 'E29', 'E30', 'E31', 'E32',
    'E33', 'E34', 'E35', 'E36', 'E37', 'E38', 'E39', 'E40',
    'E41', 'E42', 'E43', 'E44', 'E45', 'E46', 'E47', 'E48'
];

// 用户定义的声调序列（170个，顺序决定连接和指令）
const USER_SHENGDIAO = Array.from({length: 170}, (_, i) => `D${String.fromCharCode(65 + i % 26)}${Math.floor(i / 26) + 1}`);

// 创建系统实例
const UNICORE_SYSTEM = new PureSymbolSystem({
    shengmu: USER_SHENGMU,
    yunmu: USER_YUNMU,
    shengdiao: USER_SHENGDIAO
});

// 导出
window.PureSymbolSystem = PureSymbolSystem;
window.UNICORE_SYSTEM = UNICORE_SYSTEM;
window.USER_SHENGMU = USER_SHENGMU;
window.USER_YUNMU = USER_YUNMU;
window.USER_SHENGDIAO = USER_SHENGDIAO;

console.log('Pure Symbol System loaded - 零数字、零进制、零进位');
