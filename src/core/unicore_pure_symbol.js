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

class NativeOrder {
  static shengmuFirst(a, b) {
    const idxA = SHENGMU.indexOf(a);
    const idxB = SHENGMU.indexOf(b);
    return (idxA !== -1 && idxB !== -1) ? (idxA < idxB) : null;
  }
  static yunmuFirst(a, b) {
    const idxA = YUNMU.indexOf(a);
    const idxB = YUNMU.indexOf(b);
    return (idxA !== -1 && idxB !== -1) ? (idxA < idxB) : null;
  }
  static shengdiaoFirst(a, b) {
    const idxA = SHENGDIAO.indexOf(a);
    const idxB = SHENGDIAO.indexOf(b);
    return (idxA !== -1 && idxB !== -1) ? (idxA < idxB) : null;
  }
  static nextShengmu(sm) {
    const idx = SHENGMU.indexOf(sm);
    return (idx !== -1 && idx < SHENGMU.length - 1) ? SHENGMU[idx + 1] : null;
  }
  static nextYunmu(ym) {
    const idx = YUNMU.indexOf(ym);
    return (idx !== -1 && idx < YUNMU.length - 1) ? YUNMU[idx + 1] : null;
  }
  static nextShengdiao(sd) {
    const idx = SHENGDIAO.indexOf(sd);
    return (idx !== -1 && idx < SHENGDIAO.length - 1) ? SHENGDIAO[idx + 1] : null;
  }
  static prevShengmu(sm) {
    const idx = SHENGMU.indexOf(sm);
    return (idx !== -1 && idx > 0) ? SHENGMU[idx - 1] : null;
  }
  static prevYunmu(ym) {
    const idx = YUNMU.indexOf(ym);
    return (idx !== -1 && idx > 0) ? YUNMU[idx - 1] : null;
  }
  static prevShengdiao(sd) {
    const idx = SHENGDIAO.indexOf(sd);
    return (idx !== -1 && idx > 0) ? SHENGDIAO[idx - 1] : null;
  }
}

class PureGrid {
  constructor(shengmu, yunmu, shengdiao) {
    this.shengmu = shengmu;
    this.yunmu = yunmu;
    this.shengdiao = shengdiao;
    this.positions = this._initPositions();
    this.connections = [];
  }
  _initPositions() {
    const pos = {};
    pos.center = { yunmu: YUNMU[0], content: null, active: false };
    pos.walls = {
      front: { yunmu: YUNMU[1], content: null, active: false },
      back: { yunmu: YUNMU[2], content: null, active: false },
      left: { yunmu: YUNMU[3], content: null, active: false },
      right: { yunmu: YUNMU[4], content: null, active: false },
      top: { yunmu: YUNMU[5], content: null, active: false },
      bottom: { yunmu: YUNMU[6], content: null, active: false }
    };
    pos.inner_corners = {
      ne_fl: { yunmu: YUNMU[7], content: null, active: false },
      ne_fr: { yunmu: YUNMU[8], content: null, active: false },
      ne_bl: { yunmu: YUNMU[9], content: null, active: false },
      ne_br: { yunmu: YUNMU[10], content: null, active: false },
      ne_tl: { yunmu: YUNMU[11], content: null, active: false },
      ne_tr: { yunmu: YUNMU[12], content: null, active: false },
      ne_bl2: { yunmu: YUNMU[13], content: null, active: false },
      ne_br2: { yunmu: YUNMU[14], content: null, active: false }
    };
    pos.outer_connections = {
      connect1: { yunmu: YUNMU[15], content: null, active: false },
      connect2: { yunmu: YUNMU[16], content: null, active: false },
      connect3: { yunmu: YUNMU[17], content: null, active: false },
      connect4: { yunmu: YUNMU[18], content: null, active: false },
      connect5: { yunmu: YUNMU[19], content: null, active: false },
      connect6: { yunmu: YUNMU[20], content: null, active: false }
    };
    pos.cross_nodes = {};
    for (let i = 21; i < YUNMU.length; i++) {
      pos.cross_nodes['node_' + i] = { yunmu: YUNMU[i], content: null, active: false };
    }
    return pos;
  }
  write(positionPath, content) {
    const parts = positionPath.split('.');
    let current = this.positions;
    for (let i = 0; i < parts.length; i++) {
      if (!current[parts[i]]) return false;
      if (i === parts.length - 1) {
        current[parts[i]].content = content;
        current[parts[i]].active = true;
        return true;
      }
      current = current[parts[i]];
    }
    return false;
  }
  read(positionPath) {
    const parts = positionPath.split('.');
    let current = this.positions;
    for (const part of parts) {
      if (!current[part]) return null;
      current = current[part];
    }
    return current.content;
  }
  activate(positionPath) {
    const parts = positionPath.split('.');
    let current = this.positions;
    for (const part of parts) {
      if (!current[part]) return false;
      current = current[part];
    }
    current.active = true;
    return true;
  }
  connect(grid, shengdiao) {
    this.connections.push({ grid: grid, shengdiao: shengdiao });
    grid.connections.push({ grid: this, shengdiao: shengdiao });
  }
  activateCenter() {
    this.positions.center.active = true;
    for (const key in this.positions.outer_connections) {
      if (this.positions.outer_connections[key].content) {
        this.positions.outer_connections[key].active = true;
      }
    }
  }
}

class PureHuren {
  constructor() { this.registered = new Map(); }
  register(identity, grid) { this.registered.set(identity, grid); }
  huren(identityA, identityB) {
    const gridA = this.registered.get(identityA);
    const gridB = this.registered.get(identityB);
    if (!gridA || !gridB) return false;
    if (NativeOrder.shengmuFirst(gridA.shengmu, gridB.shengmu)) {
      return { result: true, higher: gridA, lower: gridB };
    }
    if (NativeOrder.shengmuFirst(gridB.shengmu, gridA.shengmu)) {
      return { result: true, higher: gridB, lower: gridA };
    }
    if (NativeOrder.yunmuFirst(gridA.yunmu, gridB.yunmu)) {
      return { result: true, higher: gridA, lower: gridB };
    }
    if (NativeOrder.yunmuFirst(gridB.yunmu, gridA.yunmu)) {
      return { result: true, higher: gridB, lower: gridA };
    }
    return { result: true, equal: true, gridA: gridA, gridB: gridB };
  }
}

class PureJie {
  constructor() {
    this.jieState = {};
    const names = ['jie_he', 'jie_jiao', 'jie_chu', 'jie_wei', 'jie_quan', 'jie_shu', 'jie_ji', 'jie_xu', 'jie_cheng', 'jie_cha', 'jie_duan', 'jie_die', 'jie_ni', 'jie_shun', 'jie_jiang', 'jie_sheng', 'jie_jianbian', 'jie_xishu', 'jie_fanchou'];
    for (const n of names) { this.jieState[n] = { active: false, value: null }; }
  }
  activate(jieName, value) {
    if (this.jieState[jieName]) {
      this.jieState[jieName].active = true;
      this.jieState[jieName].value = value;
    }
  }
  jieCheng(shengmu, yunmu, shengdiao) {
    let current = { shengmu: shengmu, yunmu: yunmu, shengdiao: shengdiao };
    const sdIdx = SHENGDIAO.indexOf(shengdiao);
    if (sdIdx === SHENGDIAO.length - 1) {
      const nextYm = NativeOrder.nextYunmu(yunmu);
      if (nextYm) {
        current.yunmu = nextYm;
        current.shengdiao = SHENGDIAO[0];
      } else {
        const nextSm = NativeOrder.nextShengmu(shengmu);
        if (nextSm) {
          current.shengmu = nextSm;
          current.yunmu = YUNMU[0];
          current.shengdiao = SHENGDIAO[0];
        } else {
          current.shengdiao = shengdiao + '_double';
        }
      }
    } else {
      current.shengdiao = NativeOrder.nextShengdiao(shengdiao);
    }
    return current;
  }
  jieShun(shengmu) { return NativeOrder.nextShengmu(shengmu); }
  jieNi(shengmu) { return NativeOrder.prevShengmu(shengmu); }
  jieSheng(currentShengmu) {
    let sm = currentShengmu;
    let nextSm = NativeOrder.nextShengmu(sm);
    while (nextSm) { sm = nextSm; nextSm = NativeOrder.nextShengmu(sm); }
    return sm;
  }
  jieJiang(currentShengmu) {
    let sm = currentShengmu;
    let prevSm = NativeOrder.prevShengmu(sm);
    while (prevSm) { sm = prevSm; prevSm = NativeOrder.prevShengmu(sm); }
    return sm;
  }
}

class PureZhenfa {
  constructor() { this.grids = []; }
  addGrid(grid, positionType) { this.grids.push({ grid: grid, positionType: positionType }); }
  zhuiZhen(centerGrid, layers) {
    const result = [centerGrid];
    let currentLayer = [centerGrid];
    let currentSd = SHENGDIAO[0];
    for (let layer = 0; layer < layers; layer++) {
      const nextLayer = [];
      for (const g of currentLayer) {
        for (let i = 0; i < 3; i++) {
          const newSm = g.shengmu;
          const newYm = NativeOrder.nextYunmu(g.yunmu) || YUNMU[0];
          const newGrid = new PureGrid(newSm, newYm, currentSd);
          g.connect(newGrid, currentSd);
          nextLayer.push(newGrid);
          result.push(newGrid);
          currentSd = NativeOrder.nextShengdiao(currentSd) || SHENGDIAO[0];
        }
      }
      currentLayer = nextLayer;
    }
    return result;
  }
  jingxiangZhen(gridA, gridB) {
    gridA.positions.walls.front.content = gridB;
    gridB.positions.walls.front.content = gridA;
    gridA.connect(gridB, SHENGDIAO[0]);
  }
  menZhen(grid, lockGrid) {
    grid.write('walls.top', lockGrid);
    grid.write('walls.bottom', { locked: true });
  }
}

class PureModules {
  constructor() {
    this.modules = {};
    const names = ['mll', 'll', 'ct', 'gtt', 'xy', 'gy', 'mrror', 'cbc', 'mrror.mll', 'cbc.mll'];
    const descs = ['主聚类底层核心根模块', '层级链递从属子模块', '层级位权定型模块', '立体几何拓扑根节点', '空间相位坐标节点', '格局阴阳对立统一模块', '镜像对立统一模块', '汉字积木编码模块', '自纠错自收敛聚类中枢', '编码块挂靠主聚类'];
    for (let i = 0; i < names.length; i++) {
      this.modules[names[i]] = { desc: descs[i], active: false, bound: [] };
    }
  }
  activate(moduleName) { if (this.modules[moduleName]) { this.modules[moduleName].active = true; } }
  bind(moduleName, target) { if (this.modules[moduleName]) { this.modules[moduleName].bound.push(target); } }
  mllCluster(items, featureExtractor) {
    const clusters = new Map();
    for (const item of items) {
      const feature = featureExtractor(item);
      if (!clusters.has(feature)) { clusters.set(feature, []); }
      clusters.get(feature).push(item);
    }
    return clusters;
  }
}

class PureCBC {
  constructor() { this.characters = new Map(); }
  storeChar(hanzi, shengmu, shengdiao, details) {
    const grid = new PureGrid(shengmu, YUNMU[0], shengdiao);
    grid.write('center', hanzi);
    grid.write('walls.front', shengmu);
    grid.write('walls.back', YUNMU[0]);
    grid.write('walls.left', shengdiao);
    grid.write('walls.right', details.strokes);
    grid.write('walls.top', details.radical);
    grid.write('walls.bottom', details.component);
    grid.write('inner_corners.ne_fl', details.original);
    grid.write('inner_corners.ne_fr', details.evolution);
    grid.write('inner_corners.ne_bl', details.cause);
    grid.write('inner_corners.ne_br', details.right_wrong);
    grid.write('inner_corners.ne_tl', details.yinyang);
    grid.write('inner_corners.ne_tr', details.closeness);
    grid.write('inner_corners.ne_bl2', details.boundary);
    grid.write('outer_connections.connect1', details.synonym);
    grid.write('outer_connections.connect2', details.antonym);
    grid.write('outer_connections.connect3', details.ancient);
    grid.write('outer_connections.connect4', details.related);
    grid.write('outer_connections.connect5', details.derived);
    this.characters.set(hanzi, grid);
    return grid;
  }
  retrieveChar(shengmu, shengdiao) {
    for (const [hanzi, grid] of this.characters) {
      if (grid.shengmu === shengmu && grid.shengdiao === shengdiao) {
        return { hanzi: hanzi, grid: grid };
      }
    }
    return null;
  }
  traceback(hanzi) {
    const grid = this.characters.get(hanzi);
    if (!grid) return null;
    return {
      hanzi: hanzi,
      shengmu: grid.read('walls.front'),
      yunmu: grid.read('walls.back'),
      shengdiao: grid.read('walls.left'),
      strokes: grid.read('walls.right'),
      radical: grid.read('walls.top'),
      component: grid.read('walls.bottom'),
      original: grid.read('inner_corners.ne_fl'),
      evolution: grid.read('inner_corners.ne_fr'),
      cause: grid.read('inner_corners.ne_bl'),
      right_wrong: grid.read('inner_corners.ne_br'),
      yinyang: grid.read('inner_corners.ne_tl'),
      closeness: grid.read('inner_corners.ne_tr'),
      boundary: grid.read('inner_corners.ne_bl2'),
      synonym: grid.read('outer_connections.connect1'),
      antonym: grid.read('outer_connections.connect2'),
      ancient: grid.read('outer_connections.connect3'),
      related: grid.read('outer_connections.connect4'),
      derived: grid.read('outer_connections.connect5')
    };
  }
}

module.exports = { SHENGMU, YUNMU, SHENGDIAO, NativeOrder, PureGrid, PureHuren, PureJie, PureZhenfa, PureModules, PureCBC };
