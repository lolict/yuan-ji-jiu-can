/**
 * UniCore v4.0 - 稀疏数代数体系
 * 
 * 核心原则：
 * 1. 声母、韵母、声调本身就是进制
 * 2. 优先级：声调(170) > 韵母(64) > 声母(48) > 数字(10)
 * 3. 稀疏数代数学：后面数量少的代表前面数量多的集合
 * 4. 数字是代数，代表前面所有集合
 * 
 * 稀疏数规则：
 * - 数量越少，权位越高
 * - 后面的数代表前面的数
 * - 每个符号都是自己体系的集装箱
 */

class SparseNumberSystem {
    constructor(options = {}) {
        // 用户定义的符号序列（按优先级从低到高排列）
        this.shengmu = options.shengmu || [];    // 48个，第3优先级
        this.yunmu = options.yunmu || [];         // 64个，第2优先级
        this.shengdiao = options.shengdiao || []; // 170个，第1优先级
        this.numbers = options.numbers || [];     // 10个，第4优先级（代数）
        
        // 符号到索引的映射
        this.shengmuIndex = new Map();
        this.yunmuIndex = new Map();
        this.shengdiaoIndex = new Map();
        this.numberIndex = new Map();
        
        // 稀疏数映射表（后面的数 → 前面的集合）
        this.sparseMapping = new Map();
        
        this.init();
    }

    init() {
        // 建立索引映射
        this.shengmu.forEach((s, i) => this.shengmuIndex.set(s, i));
        this.yunmu.forEach((y, i) => this.yunmuIndex.set(y, i));
        this.shengdiao.forEach((d, i) => this.shengdiaoIndex.set(d, i));
        this.numbers.forEach((n, i) => this.numberIndex.set(n, i));
        
        // 构建稀疏数映射：数字→声母→韵母→声调
        this.buildSparseMapping();
    }

    /**
     * 构建稀疏数映射表
     * 核心逻辑：数量少的代表数量多的
     * 数字(10) → 声母(48) → 韵母(64) → 声调(170)
     */
    buildSparseMapping() {
        // 数字代表声母：1个数字代表多个声母
        const shengmuPerNumber = Math.ceil(this.shengmu.length / this.numbers.length);
        this.numbers.forEach((num, numIdx) => {
            const startIdx = numIdx * shengmuPerNumber;
            const endIdx = Math.min(startIdx + shengmuPerNumber, this.shengmu.length);
            const shengmuSet = this.shengmu.slice(startIdx, endIdx);
            this.sparseMapping.set(`num_${num}`, { type: 'shengmu', symbols: shengmuSet });
        });

        // 声母代表韵母：1个声母代表多个韵母
        const yunmuPerShengmu = Math.ceil(this.yunmu.length / this.shengmu.length);
        this.shengmu.forEach((sm, smIdx) => {
            const startIdx = smIdx * yunmuPerShengmu;
            const endIdx = Math.min(startIdx + yunmuPerShengmu, this.yunmu.length);
            const yunmuSet = this.yunmu.slice(startIdx, endIdx);
            this.sparseMapping.set(`sm_${sm}`, { type: 'yunmu', symbols: yunmuSet });
        });

        // 韵母代表声调：1个韵母代表多个声调
        const shengdiaoPerYunmu = Math.ceil(this.shengdiao.length / this.yunmu.length);
        this.yunmu.forEach((ym, ymIdx) => {
            const startIdx = ymIdx * shengdiaoPerYunmu;
            const endIdx = Math.min(startIdx + shengdiaoPerYunmu, this.shengdiao.length);
            const shengdiaoSet = this.shengdiao.slice(startIdx, endIdx);
            this.sparseMapping.set(`ym_${ym}`, { type: 'shengdiao', symbols: shengdiaoSet });
        });
    }

    /**
     * 获取符号的权位值
     * 权位：声调 > 韵母 > 声母 > 数字
     */
    getWeight(symbol, type) {
        const total = this.shengdiao.length + this.yunmu.length + this.shengmu.length + this.numbers.length;
        
        switch(type) {
            case 'shengdiao':
                return total - this.shengdiaoIndex.get(symbol);
            case 'yunmu':
                return total - this.shengmu.length - this.numbers.length - this.yunmuIndex.get(symbol);
            case 'shengmu':
                return total - this.numbers.length - this.shengmuIndex.get(symbol);
            case 'number':
                return this.numberIndex.get(symbol) + 1;
            default:
                return 0;
        }
    }

    /**
     * 获取符号代表的集合
     */
    getRepresentedSet(symbol, type) {
        const key = `${type.charAt(0)}m_${symbol}`;
        return this.sparseMapping.get(key) || null;
    }

    /**
     * 获取符号的完整信息
     */
    getSymbolInfo(symbol, type) {
        const info = {
            symbol,
            type,
            weight: this.getWeight(symbol, type),
            index: this[`${type}Index`].get(symbol),
            represents: null
        };
        
        const represented = this.getRepresentedSet(symbol, type);
        if (represented) {
            info.represents = {
                type: represented.type,
                count: represented.symbols.length,
                symbols: represented.symbols.slice(0, 5)
            };
        }
        
        return info;
    }

    // ==================== 格子系统 ====================

    /**
     * 创建格子（集装箱）
     * 每个格子由一个完整的稀疏数组合构成
     */
    createGrid(shengmu, yunmu, shengdiao) {
        return {
            shengmu,      // 声母集装箱
            yunmu,        // 韵母集装箱
            shengdiao,    // 声调集装箱
            content: null,// 内容
            connections: {} // 与其他格子的连接
        };
    }

    /**
     * 获取格子的完整权位值
     */
    getGridWeight(grid) {
        const sdWeight = this.getWeight(grid.shengdiao, 'shengdiao');
        const ymWeight = this.getWeight(grid.yunmu, 'yunmu');
        const smWeight = this.getWeight(grid.shengmu, 'shengmu');
        return sdWeight * 10000 + ymWeight * 100 + smWeight;
    }

    // ==================== 互认机制 ====================

    /**
     * 两个格子是否互认（通过共同代表的集合）
     */
    recognize(grid1, grid2) {
        // 检查是否共享相同的符号
        if (grid1.shengmu === grid2.shengmu) return true;
        if (grid1.yunmu === grid2.yunmu) return true;
        if (grid1.shengdiao === grid2.shengdiao) return true;
        
        // 检查是否通过稀疏数映射关联
        const grid1Sets = [
            this.getRepresentedSet(grid1.shengmu, 'shengmu'),
            this.getRepresentedSet(grid1.yunmu, 'yunmu')
        ];
        const grid2Sets = [
            this.getRepresentedSet(grid2.shengmu, 'shengmu'),
            this.getRepresentedSet(grid2.yunmu, 'yunmu')
        ];
        
        return grid1Sets.some(set1 => 
            grid2Sets.some(set2 => 
                set1 && set2 && set1.type === set2.type
            )
        );
    }

    // ==================== 稀疏数运算 ====================

    /**
     * 稀疏数加法：用数量少的代表数量多的
     */
    sparseAdd(symbol1, type1, symbol2, type2) {
        const info1 = this.getSymbolInfo(symbol1, type1);
        const info2 = this.getSymbolInfo(symbol2, type2);
        
        // 返回权位更高的符号（数量更少的）
        if (info1.weight > info2.weight) {
            return { result: symbol1, type: type1, reason: '权位更高' };
        } else if (info2.weight > info1.weight) {
            return { result: symbol2, type: type2, reason: '权位更高' };
        } else {
            return { result: symbol1, type: type1, reason: '权位相等，取第一个' };
        }
    }

    /**
     * 获取符号代表的所有符号链
     */
    getSymbolChain(symbol, type) {
        const chain = [{ symbol, type, weight: this.getWeight(symbol, type) }];
        let current = this.getRepresentedSet(symbol, type);
        
        while (current) {
            const nextType = current.type;
            const nextSymbol = current.symbols[0];
            chain.push({
                symbol: nextSymbol,
                type: nextType,
                weight: this.getWeight(nextSymbol, nextType),
                representsCount: current.symbols.length
            });
            current = this.getRepresentedSet(nextSymbol, nextType);
        }
        
        return chain;
    }

    // ==================== 汉字系统 ====================

    /**
     * 创建汉字（完整的稀疏数组合）
     */
    createCharacter(shengmu, yunmu, shengdiao) {
        return {
            sparseCode: `${shengmu}${yunmu}${shengdiao}`,
            shengmu,
            yunmu,
            shengdiao,
            weight: this.getGridWeight({ shengmu, yunmu, shengdiao }),
            meaning: null,      // 本义
            evolution: [],      // 演化历史
            relations: {}       // 关联关系
        };
    }

    // ==================== 通信系统 ====================

    /**
     * 通过稀疏数传递信息
     */
    transmit(sourceGrid, targetGrid, data) {
        if (!this.recognize(sourceGrid, targetGrid)) {
            return { success: false, reason: '未互认' };
        }
        
        // 通过最高权位的符号传递
        const sourceChain = this.getSymbolChain(sourceGrid.shengdiao, 'shengdiao');
        const targetChain = this.getSymbolChain(targetGrid.shengdiao, 'shengdiao');
        
        // 找到共同的最高权位符号
        const commonSymbol = sourceChain.find(s => 
            targetChain.some(t => t.symbol === s.symbol)
        );
        
        if (commonSymbol) {
            targetGrid.content = {
                from: sourceGrid.sparseCode,
                data,
                via: commonSymbol.symbol
            };
            return { success: true, via: commonSymbol.symbol };
        }
        
        return { success: false, reason: '无共同通道' };
    }
}

// ==================== 用户定义的符号序列 ====================

// 用户定义的声母序列（48个，第3优先级）
const USER_SHENGMU = [
    'B', 'P', 'M', 'F', 'D', 'T', 'N', 'L',
    'G', 'K', 'H', 'J', 'Q', 'X', 'ZH', 'CH',
    'SH', 'R', 'Z', 'C', 'S', 'Y', 'W', 'A',
    'O', 'E', 'I', 'U', 'V', 'AI', 'EI', 'AO',
    'OU', 'AN', 'EN', 'ANG', 'ENG', 'ER', 'AR',
    'OR', 'IR', 'UR', 'YR', 'WR', 'BR', 'PR',
    'MR', 'FR'
];

// 用户定义的韵母序列（64个，第2优先级）
const USER_YUNMU = [
    'A', 'O', 'E', 'I', 'U', 'V',
    'AI', 'EI', 'AO', 'OU', 'AN', 'EN',
    'ANG', 'ENG', 'ER', 'IA', 'IE', 'IO',
    'IU', 'IV', 'IAI', 'IEI', 'IAO', 'IOU',
    'IAN', 'IEN', 'IANG', 'IENG', 'UA', 'UO',
    'UE', 'UI', 'UV', 'UAI', 'UEI', 'UAO',
    'UOU', 'UAN', 'UEN', 'UANG', 'UENG', 'VA',
    'VO', 'VE', 'VI', 'VU', 'VAI', 'VEI',
    'VAO', 'VOU', 'VAN', 'VEN', 'VANG', 'VENG',
    'YA', 'YO', 'YE', 'YI', 'YU', 'YAI',
    'YEI', 'YAO', 'YOU', 'YAN', 'YEN', 'YANG',
    'YENG'
];

// 用户定义的声调序列（170个，第1优先级）
const USER_SHENGDIAO = Array.from({length: 170}, (_, i) => {
    const group = Math.floor(i / 10);
    const pos = i % 10;
    return `${String.fromCharCode(65 + group)}${pos}`;
});

// 用户定义的数字序列（10个，第4优先级，代数）
const USER_NUMBERS = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

// 创建系统实例
const SPARSE_SYSTEM = new SparseNumberSystem({
    shengmu: USER_SHENGMU,
    yunmu: USER_YUNMU,
    shengdiao: USER_SHENGDIAO,
    numbers: USER_NUMBERS
});

// 导出
window.SparseNumberSystem = SparseNumberSystem;
window.SPARSE_SYSTEM = SPARSE_SYSTEM;
window.USER_SHENGMU = USER_SHENGMU;
window.USER_YUNMU = USER_YUNMU;
window.USER_SHENGDIAO = USER_SHENGDIAO;
window.USER_NUMBERS = USER_NUMBERS;

console.log('Sparse Number System loaded - 稀疏数代数体系');
