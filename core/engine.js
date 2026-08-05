/*
 * 囡囝共生 · 核心逻辑引擎 v0.1
 * ---------------------------
 * 编码系统 + 天平裁判 + 关系代数
 * 纯 JS 模块，无 UI、无网络、无暴露
 * 
 * 导出:
 *   MQF.encode     编码系统
 *   MQF.scale      天平裁判
 *   MQF.algebra    关系代数
 *   MQF.situation  态势逻辑
 */

const MQF = {};

/* ============================================================
 * 0. 基础数据表
 * ============================================================ */
const INITS = ["b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w","@"];
const FINALS = ["a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er","an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao","iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong"];
const TONES = ["1","2","3","4","0"];
const TONE_MARKS = {1:"̄",2:"́",3:"̌",4:"̀",0:""};

// 通用汉字拼音表（最少覆盖常用 500 字，实际部署可扩展）
const PINYIN_DB = {
  "的":["de","d","e","0"], "一":["yi","y","i","1"], "是":["shi","sh","i","4"], "不":["bu","b","u","4"], "了":["le","l","e","0"],
  "在":["zai","z","ai","4"], "人":["ren","r","en","2"], "有":["you","y","ou","3"], "我":["wo","w","o","3"], "他":["ta","t","a","1"], "她":["ta","t","a","1"],
  "们":["men","m","en","0"], "这":["zhe","zh","e","4"], "中":["zhong","zh","ong","1"], "大":["da","d","a","4"], "来":["lai","l","ai","2"],
  "上":["shang","sh","ang","4"], "个":["ge","g","e","4"], "到":["dao","d","ao","4"], "说":["shuo","sh","uo","1"], "时":["shi","sh","i","2"],
  "和":["he","h","e","2"], "地":["de","d","e","0"], "出":["chu","ch","u","1"], "会":["hui","h","ui","4"], "也":["ye","y","e","3"],
  "你":["ni","n","i","3"], "对":["dui","d","ui","4"], "她":["ta","t","a","1"], "里":["li","l","i","3"], "后":["hou","h","ou","4"],
  "小":["xiao","x","iao","3"], "么":["me","m","e","0"], "心":["xin","x","in","1"], "多":["duo","d","uo","1"], "天":["tian","t","ian","1"],
  "而":["er","@","er","2"], "能":["neng","n","eng","2"], "好":["hao","h","ao","3"], "都":["dou","d","ou","1"], "没":["mei","m","ei","2"],
  "日":["ri","r","i","4"], "去":["qu","q","u","4"], "过":["guo","g","uo","4"], "家":["jia","j","ia","1"], "年":["nian","n","ian","2"],
  "说":["shuo","sh","uo","1"], "看":["kan","k","an","4"], "自":["zi","z","i","3"], "得":["de","d","e","2"], "知":["zhi","zh","i","1"],
  "行":["xing","x","ing","2"], "方":["fang","f","ang","1"], "所":["suo","s","uo","3"], "an":["an","@","an","1"],
  "成":["cheng","ch","eng","2"], "要":["yao","y","ao","4"] , "当":["dang","d","ang","1"],
  "想":["xiang","x","iang","3"], "只":["zhi","zh","i","1"], "如":["ru","r","u","2"], "事":["shi","sh","i","4"],
  "把":["ba","b","a","3"], "被":["bei","b","ei","4"], "从":["cong","c","ong","2"], "当":["dang","d","ang","1"],
  "等":["deng","d","eng","3"], "各":["ge","g","e","4"], "己":["ji","j","i","3"], "其":["qi","q","i","2"],
  "入":["ru","r","u","4"], "若":["ruo","r","uo","4"], "使":["shi","sh","i","3"], "同":["tong","t","ong","2"],
  "无":["wu","w","u","2"], "但":["dan","d","an","4"], "何":["he","h","e","2"], "仍":["reng","r","eng","2"],
  "认":["ren","r","en","4"], "让":["rang","r","ang","4"], "所":["suo","s","uo","3"], "因":["yin","y","in","1"],
  "与":["yu","y","u","3"], "在":["zai","z","ai","4"], "则":["ze","z","e","2"], "zhe":["zhe","zh","e","4"],
  "真":["zhen","zh","en","1"], "正":["zheng","zh","eng","4"]
};

// 笔画数（常用字）
const STROKES_DB = {
  "的":8,"一":1,"是":9,"不":4,"了":2,"在":6,"人":2,"有":6,"我":7,"他":5,"她":6,"们":5,"这":7,"中":4,"大":3,
  "来":5,"上":3,"个":3,"到":8,"说":9,"时":7,"和":8,"地":6,"出":5,"会":6,"也":3,"你":7,"对":5,"她":6,
  "里":7,"后":6,"小":3,"么":3,"心":4,"多":6,"天":4,"而":6,"能":10,"好":6,"都":10,"没":7,"日":4,
  "去":5,"过":6,"家":10,"年":6,"看":9,"自":6,"得":11,"知":8,"行":6,"方":4,"所":8,"成":6,"要":9,
  "当":6,"想":13,"只":5,"如":6,"事":8,"把":7,"被":10,"从":4,"等":12,"各":6,"己":3,"其":8,"入":2,
  "若":8,"使":8,"同":6,"无":4,"但":7,"何":7,"仍":4,"认":4,"让":5,"因":6,"与":3,"则":6,"真":10,
  "正":5
};

/* ============================================================
 * 1. 编码系统 — 汉字 → 多层向量
 * ============================================================ */
MQF.encode = {
  // 单字编码：{ 声母, 韵母, 声调, 笔画, 声母号, 韵母号, 声调号, 笔画号 }
  char(ch) {
    const r = PINYIN_DB[ch];
    if (!r) {
      const cp = ch.codePointAt(0);
      return {
        ch,
        init: INITS[cp % INITS.length],
        final: FINALS[(cp / INITS.length | 0) % FINALS.length],
        tone: "" + (cp % 5),
        strokes: (cp % 30) + 1,
        init_idx: cp % INITS.length,
        final_idx: (cp / INITS.length | 0) % FINALS.length,
        tone_idx: cp % 5,
        stroke_idx: cp % 30,
        source: "fallback"
      };
    }
    return {
      ch,
      init: r[1],
      final: r[2],
      tone: r[3],
      strokes: STROKES_DB[ch] || ((ch.codePointAt(0) % 30) + 1),
      init_idx: INITS.indexOf(r[1]),
      final_idx: FINALS.indexOf(r[2]),
      tone_idx: TONES.indexOf(r[3]),
      stroke_idx: STROKES_DB[ch] || ((ch.codePointAt(0) % 30) + 1),
      source: "db"
    };
  },

  // 文本编码：逐字编码后汇总
  text(s) {
    const chars = Array.from(s).filter(c => /[一-龥]/.test(c));
    const encoded = chars.map(c => this.char(c));
    return { text: s, chars: encoded, length: chars.length };
  },

  // 三维进制编码：每个字 → [声母号, 韵母号, 声调号] 三维坐标
  encode3D(s) {
    const chars = Array.from(s).filter(c => /[一-龥]/.test(c));
    return chars.map(c => {
      const e = this.char(c);
      return [e.init_idx, e.final_idx, e.tone_idx];
    });
  },

  // 七维神龙链：声母,韵母,声调,笔画,偏旁,外语近似,空
  dragonChain(s) {
    const chars = Array.from(s).filter(c => /[一-龥]/.test(c));
    return chars.map(c => {
      const e = this.char(c);
      const cp = c.codePointAt(0);
      return {
        init: e.init_idx,
        final: e.final_idx,
        tone: e.tone_idx,
        strokes: e.strokes % 32,
        radical: cp % 64,
        foreign: cp % 128,
        harmonic: (e.init_idx * 7 + e.final_idx * 3 + e.tone_idx) % 64
      };
    });
  },

  // 编码距离：两字之间的编码距离（多维欧氏）
  distance(ch1, ch2) {
    const a = this.char(ch1), b = this.char(ch2);
    const d1 = a.init_idx - b.init_idx;
    const d2 = a.final_idx - b.final_idx;
    const d3 = a.tone_idx - b.tone_idx;
    const d4 = a.strokes - b.strokes;
    return Math.sqrt(d1*d1 + d2*d2 + d3*d3 + d4*d4);
  }
};

/* ============================================================
 * 2. 天平裁判 — 对偶观测与裁决
 * ============================================================ */
MQF.scale = {
  // 单维度评分：观测量 vs 被观测量 → 偏差
  weigh(observed, observer, axis = "tone") {
    const a = MQF.encode.char(observed);
    const b = MQF.encode.char(observer);
    const axes = {
      tone: [a.tone_idx, b.tone_idx],
      init: [a.init_idx, b.init_idx],
      final: [a.final_idx, b.final_idx],
      strokes: [a.strokes, b.strokes]
    };
    const [va, vb] = axes[axis] || [0, 0];
    const diff = va - vb;
    const balance = diff === 0;
    const direction = diff > 0 ? "observed_stronger" : diff < 0 ? "observer_stronger" : "balanced";
    return { axis, observed_val: va, observer_val: vb, diff, balance, direction };
  },

  // 全维度裁决
  judge(textA, textB) {
    const encA = MQF.encode.text(textA).chars;
    const encB = MQF.encode.text(textB).chars;
    const len = Math.min(encA.length, encB.length);
    const verdicts = [];
    let totalScore = 0;
    for (let i = 0; i < len; i++) {
      const toneW = this.weigh(encA[i].ch, encB[i].ch, "tone");
      const initW = this.weigh(encA[i].ch, encB[i].ch, "init");
      const finalW = this.weigh(encA[i].ch, encB[i].ch, "final");
      const localScore = (toneW.balance ? 1 : 0) + (initW.balance ? 1 : 0) + (finalW.balance ? 1 : 0);
      totalScore += localScore;
      verdicts.push({ 
        pos: i, charA: encA[i].ch, charB: encB[i].ch,
        tone: toneW, init: initW, final: finalW,
        score: localScore
      });
    }
    const maxScore = len * 3;
    const ratio = maxScore > 0 ? totalScore / maxScore : 0;
    const overall = ratio >= 0.8 ? "吉" : ratio >= 0.5 ? "平" : ratio >= 0.3 ? "争" : "冲";
    return { textA, textB, verdicts, totalScore, maxScore, ratio, overall };
  },

  // 统计裁判：对一组观测做整体评估
  statistics(observations) {
    const n = observations.length;
    if (n === 0) return { count: 0, mean: 0, stable: true };
    const scores = observations.map(o => o.score || 0);
    const mean = scores.reduce((a,b) => a+b, 0) / n;
    const variance = scores.reduce((a,b) => a + (b-mean)*(b-mean), 0) / n;
    const stdDev = Math.sqrt(variance);
    const stable = stdDev < 0.5;
    return { count: n, mean, variance, stdDev, stable, min: Math.min(...scores), max: Math.max(...scores) };
  }
};

/* ============================================================
 * 3. 关系代数 — 对象 × 关系 × 交互 × 迭代
 * ============================================================ */
MQF.algebra = {
  // 对象: { id, encoding, relations: [] }
  createObject(id, text) {
    return {
      id,
      encoding: MQF.encode.text(text),
      dragon: MQF.encode.dragonChain(text),
      createdAt: Date.now(),
      relations: [],
      state: "being" // being / non-being
    };
  },

  // 关系: { from, to, type, weight, bidirectional }
  createRelation(fromId, toId, type, weight = 1) {
    return { from: fromId, to: toId, type, weight, createdAt: Date.now() };
  },

  // 对象+关系 → 关系图快照
  addRelation(obj, relation) {
    obj.relations.push(relation);
    return obj;
  },

  // 迭代：所有对象的关系进行一次步进
  step(objects, iterations = 1) {
    for (let iter = 0; iter < iterations; iter++) {
      for (const obj of objects) {
        for (const rel of obj.relations) {
          if (rel.type === "observe" || rel.type === "bind") {
            const target = objects.find(o => o.id === rel.to);
            if (target) {
              // 观测/绑定 → 关系权重按编码距离调整
              const dist = obj.encoding.chars.length > 0 && target.encoding.chars.length > 0
                ? MQF.encode.distance(obj.encoding.chars[0].ch, target.encoding.chars[0].ch)
                : 10;
              rel.weight = rel.weight * 0.9 + (1 / (1 + dist)) * 0.1;
            }
          } else if (rel.type === "transform") {
            // 变换 → 对象编码按关系方向转移
            if (obj.encoding.chars.length > 0) {
              obj.state = obj.state === "being" ? "non-being" : "being";
            }
          }
        }
      }
    }
    return objects;
  },

  // 全量关系快照
  snapshot(objects) {
    const edges = [];
    for (const obj of objects) {
      for (const rel of obj.relations) {
        edges.push({ from: rel.from, to: rel.to, type: rel.type, weight: rel.weight });
      }
    }
    return {
      objects: objects.map(o => ({ id: o.id, state: o.state, encoding_count: o.encoding.chars.length })),
      edges,
      timestamp: Date.now()
    };
  },

  // 对象路径：A → B 是否可达
  reachable(objects, fromId, toId) {
    const visited = new Set();
    const queue = [fromId];
    while (queue.length > 0) {
      const id = queue.shift();
      if (id === toId) return true;
      if (visited.has(id)) continue;
      visited.add(id);
      const obj = objects.find(o => o.id === id);
      if (obj) for (const r of obj.relations) queue.push(r.to);
    }
    return false;
  },

  // 关系交集：两个对象共有的关系对象
  commonTargets(objA, objB) {
    const setA = new Set(objA.relations.map(r => r.to));
    const setB = new Set(objB.relations.map(r => r.to));
    return [...setA].filter(x => setB.has(x));
  }
};

/* ============================================================
 * 4. 态势逻辑 — 有无/间接传递/空间感应
 * ============================================================ */
MQF.situation = {
  // 有↔无 中介状态：两对象之间的间接连接强度
  indirectForce(objA, objB, objects, hops = 1) {
    let strength = 0;
    for (const r1 of objA.relations) {
      const mid = objects.find(o => o.id === r1.to);
      if (!mid) continue;
      for (const r2 of mid.relations) {
        if (r2.to === objB.id) {
          strength += (r1.weight || 0.5) * (r2.weight || 0.5);
        }
      }
    }
    return { from: objA.id, to: objB.id, hops, strength, exist: strength > 0 };
  },

  // 空间镜像：对象 A 的"内空间"——所有间接可达对象的编码特征向量
  innerMirror(objId, objects, depth = 2) {
    const obj = objects.find(o => o.id === objId);
    if (!obj) return null;
    const visited = new Set();
    const layers = [[obj]];
    visited.add(objId);
    for (let d = 0; d < depth; d++) {
      const next = [];
      for (const o of layers[layers.length-1]) {
        for (const r of o.relations) {
          if (!visited.has(r.to)) {
            const target = objects.find(x => x.id === r.to);
            if (target) { next.push(target); visited.add(r.to); }
          }
        }
      }
      if (next.length === 0) break;
      layers.push(next);
    }
    return { center: objId, layers: layers.map((layer, i) => ({ depth: i, count: layer.length, nodes: layer.map(n => ({ id: n.id, state: n.state })) })) };
  },

  // 态势判断：给定一组当前关系，判断整体处于什么态势
  assess(objects) {
    const snap = MQF.algebra.snapshot(objects);
    const edges = snap.edges;
    const positive = edges.filter(e => e.type === "observe" || e.type === "bind").length;
    const transform = edges.filter(e => e.type === "transform").length;
    const totalEdges = edges.length;
    const observeRatio = totalEdges > 0 ? positive / totalEdges : 0;
    const situation = observeRatio >= 0.7 ? "稳态" : observeRatio >= 0.4 ? "渐变态" : "逆变态";
    return { situation, observeRatio, totalEdges, positiveEdges: positive, transformEdges: transform };
  }
};

/* ============================================================
 * 5. 全量入口 — 对一个文本世界做完整推演
 * ============================================================ */
MQF.world = {
  // 从一个文本库构建完整对象世界
  build(texts) {
    // texts: [{ id, content }]
    const objects = texts.map(t => MQF.algebra.createObject(t.id, t.content));
    
    // 互相关系：每个对象观测其他所有对象
    for (let i = 0; i < objects.length; i++) {
      for (let j = 0; j < objects.length; j++) {
        if (i === j) continue;
        const verdict = MQF.scale.judge(
          objects[i].encoding.text, 
          objects[j].encoding.text
        );
        MQF.algebra.addRelation(objects[i], 
          MQF.algebra.createRelation(objects[i].id, objects[j].id, "observe", verdict.ratio)
        );
      }
    }
    
    // 迭代三次
    MQF.algebra.step(objects, 3);
    
    return {
      objects: objects.map(o => ({
        id: o.id, state: o.state,
        text_len: o.encoding.length,
        relations_count: o.relations.length,
        avg_weight: o.relations.length > 0 
          ? o.relations.reduce((a,b) => a+b.weight, 0) / o.relations.length 
          : 0
      })),
      assessment: MQF.situation.assess(objects),
      relations: MQF.algebra.snapshot(objects)
    };
  }
};

/* ============================================================
 * EXPORT
 * ============================================================ */
if (typeof module !== 'undefined') module.exports = MQF;
if (typeof window !== 'undefined') window.MQF = MQF;
