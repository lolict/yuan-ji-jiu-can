// ========================
// 用户定义的纯符号序列
// ========================

const SHENGMU = [
  'B', 'P', 'M', 'F', 'D', 'T', 'N', 'L',
  'G', 'K', 'H', 'J', 'Q', 'X', 'ZH', 'CH',
  'SH', 'R', 'Z', 'C', 'S', 'Y', 'W', 'A',
  'O', 'E', 'I', 'U', 'V', 'AI', 'EI', 'AO',
  'OU', 'AN', 'EN', 'ANG', 'ENG', 'ER', 'AR',
  'OR', 'IR', 'UR', 'YR', 'WR', 'BR', 'PR',
  'MR', 'FR'
];

const YUNMU = [
  'C', 'W1', 'W2', 'W3', 'W4', 'W5', 'W6',
  'J1', 'J2', 'J3', 'J4', 'J5', 'J6', 'J7', 'J8',
  'L1', 'L2', 'L3', 'L4', 'L5', 'L6',
  'E1', 'E2', 'E3', 'E4', 'E5', 'E6', 'E7', 'E8',
  'E9', 'E10', 'E11', 'E12', 'E13', 'E14', 'E15', 'E16',
  'E17', 'E18', 'E19', 'E20', 'E21', 'E22', 'E23', 'E24',
  'E25', 'E26', 'E27', 'E28', 'E29', 'E30', 'E31', 'E32',
  'E33', 'E34', 'E35', 'E36', 'E37', 'E38', 'E39', 'E40',
  'E41', 'E42', 'E43', 'E44', 'E45', 'E46', 'E47', 'E48'
].slice(0, 64);

const SHENGDIAO = [];
for (let i = 0; i < 170; i++) {
  const letter = String.fromCharCode(65 + (i % 26));
  const num = Math.floor(i / 26) + 1;
  SHENGDIAO.push('D' + letter + num);
}

const JIE_SYSTEM = {
  jie_he: { desc: '层级互融对位、节律和合匹配', bind: ['互认对齐', '阶和'] },
  jie_jiao: { desc: '三维立体夹角、格子墙角、阵形角度基准', bind: ['阶位', 'gtt.xy'] },
  jie_chu: { desc: '层级过滤、剔除异类、清边界杂质', bind: ['互认门锁', '阶除'] },
  jie_wei: { desc: '固定空间选址、层级固有位置', bind: ['互认格子', '互认墙面'] },
  jie_quan: { desc: '层级先天权级高低、统领从属秩序', bind: ['互认权位', 'ct'] },
  jie_shu: { desc: '层级自带属性、归类特征、亲疏标签', bind: ['互认身份', 'mll'] },
  jie_ji: { desc: '同层级聚合、集装箱归集同类单元', bind: ['互认集装箱', '阶集'] },
  jie_xu: { desc: '层级先后顺位、时序排布、先天次序', bind: ['ll', '阶序'] },
  jie_cheng: { desc: '声韵调叠合、函子代数层级叠加爬升', bind: ['阶乘', '阶乘'] },
  jie_cha: { desc: '层级分叉衍生、多脉络分支延伸', bind: ['阶岔', 'gtt'] },
  jie_duan: { desc: '演化分段、规则生效分段、生长分段', bind: ['阶段', '阶段'] },
  jie_die: { desc: '里外嵌套、多层堆叠、格子套格子', bind: ['阶叠', '阶叠'] },
  jie_ni: { desc: '反向回溯、镜像反转、逆向推演', bind: ['阶逆', 'mrror'] },
  jie_shun: { desc: '正向流转、顺位运行、顺时序推演', bind: ['阶顺', '阶顺'] },
  jie_jiang: { desc: '权级下沉、层级收敛向内', bind: ['阶降', '阶降'] },
  jie_sheng: { desc: '权级攀升、层级扩张向外', bind: ['阶升', '阶升'] },
  jie_jianbian: { desc: '层级平滑过渡、大小属性特征渐变', bind: ['阶渐变', '阶渐变'] },
  jie_xishu: { desc: '节点疏密排布、阵形聚散留白固有规律', bind: ['阶稀疏数律', '阶稀疏数律'] },
  jie_fanchou: { desc: '层级管辖边界、是否范畴、不可越界规制', bind: ['阶范畴', '阶范畴'] }
};

const CATEGORIES = {
  '集合论范畴': { desc: '集合的定义、归属、操作规则' },
  '函数范畴': { desc: '函数的定义、调用、变换规则' },
  '阶范畴': { desc: '阶系的管辖、边界、约束规则' }
};

const HUREN_SYSTEM = {
  '互认接口': { desc: '互认的入口和通道', bind: '互认接口' },
  '互认权位': { desc: '互认的权级高低判定', bind: 'ct' },
  '互认格子': { desc: '格子之间的互相识别', bind: '互认格子' },
  '互认节点': { desc: '节点之间的互相识别', bind: '互认节点' },
  '互认墙面': { desc: '墙面之间的互相识别', bind: '互认墙面' },
  '互认集装箱': { desc: '集装箱之间的互相识别', bind: '互认集装箱' },
  '互认阴阳': { desc: '阴阳属性的互认', bind: 'gy' },
  '互认关系': { desc: '关系的互认判定', bind: '互认关系' },
  '互认进制': { desc: '进制的互认对齐', bind: '互认进制' },
  '互认对齐': { desc: '对齐的互认', bind: '互认对齐' },
  '互认身份': { desc: '身份的互认', bind: '互认身份' },
  '互认问题': { desc: '问题的互认', bind: '互认问题' },
  '互认目标': { desc: '目标的互认', bind: '互认目标' },
  '互认基因': { desc: '基因的互认', bind: '互认基因' },
  '互认钥匙': { desc: '钥匙的互认', bind: '互认钥匙' },
  '互认门锁': { desc: '门锁的互认', bind: '互认门锁' },
  '互认因果律': { desc: '因果律的互认', bind: '互认因果律' }
};

const ZHENFA_SYSTEM = {
  '锥阵': { desc: '锥形空间结构', bind: 'gtt.xy' },
  '球阵': { desc: '球形空间结构', bind: 'gtt.xy' },
  '方块阵': { desc: '方块空间结构', bind: 'gtt.xy' },
  '三角阵': { desc: '三角形空间结构', bind: 'gtt.xy' },
  '蜂窝阵': { desc: '六边形蜂窝结构', bind: 'gtt.xy' },
  '门阵': { desc: '边界门阵结构', bind: '互认门锁' },
  '锁阵': { desc: '安全锁阵结构', bind: '互认门锁' },
  '镜像阵': { desc: '镜像对称结构', bind: 'mrror' },
  '对立统一阵': { desc: '对立统一结构', bind: 'gy' },
  '聚类认知阵': { desc: '聚类认知结构', bind: 'mll' },
  '空间认知阵': { desc: '空间认知结构', bind: 'gtt.xy' },
  '向量排布阵': { desc: '节点拓扑脉络', bind: 'gtt.xy' },
  '标量本位阵': { desc: '本位锚点结构', bind: 'gtt.xy' },
  '张量叠合阵': { desc: '多层叠合脉络', bind: 'gtt.xy' },
  '矩阵拓扑阵': { desc: '矩阵拓扑脉络', bind: 'gtt.xy' }
};

const MODULE_ANCHORS = {
  'mll': { desc: '主聚类底层核心根模块', bind: ['聚类认知阵', '阶属', '互认身份'] },
  'll': { desc: '层级链递从属子模块', bind: ['阶序', '阶升', '阶降'] },
  'ct': { desc: '层级位权定型模块', bind: ['阶权', '互认权位'] },
  'gtt': { desc: '立体几何拓扑根节点', bind: ['锥阵', '球阵', '方块阵', '阶角'] },
  'xy': { desc: '空间相位坐标节点', bind: ['gtt', '空间认知阵'] },
  'gy': { desc: '格局阴阳对立统一模块', bind: ['互认阴阳', '对立统一阵'] },
  'mrror': { desc: '镜像对立统一模块', bind: ['镜像阵', '阶逆'] },
  'cbc': { desc: '汉字积木编码模块', bind: ['互认集装箱', '阶集'] },
  'mrror.mll': { desc: '自纠错自收敛聚类中枢', bind: ['mrror', 'mll'] },
  'cbc.mll': { desc: '编码块挂靠主聚类', bind: ['cbc', 'mll'] }
};

const GRID_POSITIONS = {
  center: { yunmu: YUNMU[0], desc: '中心位：指令入口、汉字本体、函数核心' },
  walls: {
    front: { yunmu: YUNMU[1], desc: '前墙' },
    back: { yunmu: YUNMU[2], desc: '后墙' },
    left: { yunmu: YUNMU[3], desc: '左墙' },
    right: { yunmu: YUNMU[4], desc: '右墙' },
    top: { yunmu: YUNMU[5], desc: '上墙' },
    bottom: { yunmu: YUNMU[6], desc: '下墙' }
  },
  inner_corners: {
    ne_fl: { yunmu: YUNMU[7], desc: '内角1' },
    ne_fr: { yunmu: YUNMU[8], desc: '内角2' },
    ne_bl: { yunmu: YUNMU[9], desc: '内角3' },
    ne_br: { yunmu: YUNMU[10], desc: '内角4' },
    ne_tl: { yunmu: YUNMU[11], desc: '内角5' },
    ne_tr: { yunmu: YUNMU[12], desc: '内角6' },
    ne_bl2: { yunmu: YUNMU[13], desc: '内角7' },
    ne_br2: { yunmu: YUNMU[14], desc: '内角8' }
  },
  outer_connections: {
    connect1: { yunmu: YUNMU[15], desc: '外连1' },
    connect2: { yunmu: YUNMU[16], desc: '外连2' },
    connect3: { yunmu: YUNMU[17], desc: '外连3' },
    connect4: { yunmu: YUNMU[18], desc: '外连4' },
    connect5: { yunmu: YUNMU[19], desc: '外连5' },
    connect6: { yunmu: YUNMU[20], desc: '外连6' }
  },
  cross_nodes: { nodes: [] }
};

for (let i = 21; i < YUNMU.length; i++) {
  GRID_POSITIONS.cross_nodes.nodes.push({ yunmu: YUNMU[i], desc: '节点' + i });
}

const ARCHITECTURE_TREE = {
  root: 'UniCore',
  level1: {
    '底层符号集': [SHENGMU, YUNMU, SHENGDIAO],
    '阶系体系': Object.keys(JIE_SYSTEM),
    '互认体系': Object.keys(HUREN_SYSTEM),
    '阵法体系': Object.keys(ZHENFA_SYSTEM),
    '模块锚点': Object.keys(MODULE_ANCHORS),
    '三大范畴': Object.keys(CATEGORIES),
    '三维格子': Object.keys(GRID_POSITIONS)
  }
};

module.exports = {
  SHENGMU,
  YUNMU,
  SHENGDIAO,
  JIE_SYSTEM,
  CATEGORIES,
  HUREN_SYSTEM,
  ZHENFA_SYSTEM,
  MODULE_ANCHORS,
  GRID_POSITIONS,
  ARCHITECTURE_TREE
};
