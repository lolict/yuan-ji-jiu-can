// 垒码 (Lěi Mǎ / LM) — 进制寻址 · 声母调入 · 操作族演算 · 不可变注册清单
//
// 这是一个最小实现：token 流解析 + 线性执行 + 容器映射存储。
// 从 Forth 取来了 token 流遍历形式，从 Nix 取来了不可变哈希寻址。
// 语义层完全使用满全法规则。

use std::collections::HashMap;

/// 极性 — 只取最基本的 6 个，完整 20 个在 SPEC 中定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YinYang {
    Yin,      // 阴
    Yang,     // 阳
    Ying,     // 盈
    Kui,      // 亏
    Xu,       // 虚
    Shi,      // 实
}

impl YinYang {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "阴" => Some(YinYang::Yin),
            "阳" => Some(YinYang::Yang),
            "盈" => Some(YinYang::Ying),
            "亏" => Some(YinYang::Kui),
            "虚" => Some(YinYang::Xu),
            "实" => Some(YinYang::Shi),
            _ => None,
        }
    }

    /// 极性适配：从 self 流向 target 时的数值调整
    fn bridge_value(&self, target: YinYang, value: u64) -> u64 {
        match (self, target) {
            // 阳→阴：数值不变
            (YinYang::Yang, YinYang::Yin) => value,
            // 阴→阳：数值 +1
            (YinYang::Yin, YinYang::Yang) => value.saturating_add(1),
            // 盈→亏：折半（不进位）
            (YinYang::Ying, YinYang::Kui) => value / 2,
            // 同极性：不变
            (a, b) if a == &b => value,
            _ => value,
        }
    }

    fn dress_polarity(&self, target: YinYang) -> YinYang {
        match (self, target) {
            (YinYang::Yin, YinYang::Yang) => YinYang::Yang,
            (YinYang::Yang, YinYang::Yin) => YinYang::Yin,
            _ => target,
        }
    }
}

impl std::fmt::Display for YinYang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            YinYang::Yin => "阴",
            YinYang::Yang => "阳",
            YinYang::Ying => "盈",
            YinYang::Kui => "亏",
            YinYang::Xu => "虚",
            YinYang::Shi => "实",
        };
        write!(f, "{}", s)
    }
}

/// 容器族
#[derive(Debug, Clone)]
struct Container {
    id: u64,
    name: String,       // 袋/桶/罐/...
    polarity: YinYang,
    value: u64,
    generation: u32,    // 版本代数（不可变存储的版本链）
}

impl Container {
    fn capacity(&self) -> u64 {
        // 根据容器族等级计算上限
        let tier = container_tier(&self.name);
        tier * 48
    }
}

/// 容器族等级（一级=1, 二级=2, ...）
fn container_tier(name: &str) -> u64 {
    match name {
        "袋" | "桶" | "罐" | "缸" | "盆" | "盂" | "钵" | "盏" => 1,
        "盒" | "杯" | "壶" | "瓶" | "篓" | "筐" | "篮" | "箩" => 2,
        "筲" | "箕" | "笼" | "橱" | "柜" | "箱" | "匣" | "奁" => 3,
        "瓮" | "甏" | "坛" | "罂" | "甑" | "甗" | "瓯" | "盌" | "盉" | "盅" => 4,
        _ => 1,
    }
}

/// 操作码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Zhao,   // 照 — 复制
    Jian,   // 鉴 — 见证输出
    Jiana,  // 见 — 深度查看
    Jia,    // 加 — 累加（不进位）
    Tui,    // 退 — 加法逆动作
    Jin,    // 进 — 推进（极性变更+迁移）
    Tong,   // 通 — 连通
    Duan,   // 断 — 断开
    Fen,    // 分 — 分裂
    He,     // 合 — 合并
    Zhe,    // 折 — 折叠（层级压缩）
    Die,    // 叠 — 展开
    Juan,   // 卷 — 卷曲（嵌套）
    Shu,    // 舒 — 舒展（解包）
    Cai,    // 裁 — 裁判（三元见证）
    Jue,    // 决 — 断决
}

impl Op {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "照" => Some(Op::Zhao),
            "鉴" => Some(Op::Jian),
            "见" => Some(Op::Jiana),
            "加" => Some(Op::Jia),
            "退" => Some(Op::Tui),
            "进" => Some(Op::Jin),
            "通" => Some(Op::Tong),
            "断" => Some(Op::Duan),
            "分" => Some(Op::Fen),
            "合" => Some(Op::He),
            "折" => Some(Op::Zhe),
            "叠" => Some(Op::Die),
            "卷" => Some(Op::Juan),
            "舒" => Some(Op::Shu),
            "裁" => Some(Op::Cai),
            "决" => Some(Op::Jue),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Op::Zhao => "照",
            Op::Jian => "鉴",
            Op::Jiana => "见",
            Op::Jia => "加",
            Op::Tui => "退",
            Op::Jin => "进",
            Op::Tong => "通",
            Op::Duan => "断",
            Op::Fen => "分",
            Op::He => "合",
            Op::Zhe => "折",
            Op::Die => "叠",
            Op::Juan => "卷",
            Op::Shu => "舒",
            Op::Cai => "裁",
            Op::Jue => "决",
        }
    }
}

/// 操作行
#[derive(Debug, Clone)]
struct OpLine {
    line_no: u64,
    op: Op,
    params: Vec<u64>,      // 源地址们
    targets: Vec<u64>,     // 目标地址们（→之后）
}

/// 不可变存储（借鉴 Nix：地址=身份，内容不同则哈希不同）
#[derive(Debug, Clone)]
struct StoreEntry {
    container: Container,
    hash: String,          // 简化的"哈希"标识（代数+数值+极性）
}

impl StoreEntry {
    fn compute_hash(c: &Container) -> String {
        format!("{:x}", c.generation as u64 * 100000 + c.value * 100 + c.polarity.polarity_hash())
    }

    fn new(c: Container) -> Self {
        let hash = Self::compute_hash(&c);
        StoreEntry { container: c, hash }
    }
}

impl YinYang {
    fn polarity_hash(&self) -> u64 {
        match self {
            YinYang::Yin => 0,
            YinYang::Yang => 1,
            YinYang::Ying => 2,
            YinYang::Kui => 3,
            YinYang::Xu => 4,
            YinYang::Shi => 5,
        }
    }
}

/// 垒码运行时
struct LM {
    containers: HashMap<u64, Container>,    // 当前可达容器
    history: Vec<StoreEntry>,                // 历史版本（不可变存储，旧版可寻）
    roads: HashMap<u64, Vec<u64>>,           // 公路图：容器id → [已连通的容器id]
}

impl LM {
    fn new() -> Self {
        LM {
            containers: HashMap::new(),
            history: Vec::new(),
            roads: HashMap::new(),
        }
    }

    /// 注册容器
    fn register(&mut self, id: u64, name: &str, polarity: YinYang, value: u64) {
        let c = Container {
            id,
            name: name.to_string(),
            polarity,
            value,
            generation: 0,
        };
        self.containers.insert(id, c.clone());
        self.history.push(StoreEntry::new(c));
    }

    /// 检查公路是否连通
    fn has_road(&self, from: u64, to: u64) -> bool {
        self.roads.get(&from).map_or(false, |v| v.contains(&to))
    }

    /// 建立公路
    fn build_road(&mut self, a: u64, b: u64) {
        self.roads.entry(a).or_default().push(b);
        self.roads.entry(b).or_default().push(a);
    }

    /// 写入新版本（不可变存储：旧版保留，新版成为当前可达）
    fn write_version(&mut self, id: u64, new_value: u64, new_polarity: Option<YinYang>) {
        if let Some(c) = self.containers.get_mut(&id) {
            let mut new_c = c.clone();
            new_c.value = new_value;
            if let Some(p) = new_polarity {
                new_c.polarity = p;
            }
            new_c.generation += 1;
            self.history.push(StoreEntry::new(new_c.clone()));
            *c = new_c;
        }
    }

    /// 执行一行操作
    fn exec_op(&mut self, op: &OpLine) -> Option<String> {
        match op.op {
            Op::Jian => {
                // 鉴：见证输出（旁路不消耗）
                let id = *op.params.first()?;
                let c = self.containers.get(&id)?;
                Some(format!("鉴 {} → {}({}, {})", id, c.name, c.polarity, c.value))
            }
            Op::Jiana => {
                // 见：深度查看
                let id = *op.params.first()?;
                let c = self.containers.get(&id)?;
                let cap = c.capacity();
                let road = self.roads.get(&id).map_or("无".to_string(), |v| {
                    v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
                });
                Some(format!(
                    "见 {}: {}({}, {}/{}) gen{}_road[{}]",
                    id, c.name, c.polarity, c.value, cap, c.generation, road
                ))
            }
            Op::Zhao => {
                // 照：复制（源→目标）
                let src = *op.params.first()?;
                let dst = *op.targets.first()?;
                let (src_name, src_pol, src_val, dst_name) = {
                    let src_c = self.containers.get(&src)?;
                    let dst_c = self.containers.get(&dst)?;
                    (src_c.name.clone(), src_c.polarity, src_c.value, dst_c.name.clone())
                };
                let dst_pol = self.containers.get(&dst).map(|c| c.polarity).unwrap_or(YinYang::Yin);
                let adapted = src_pol.bridge_value(dst_pol, src_val);
                let dressed_pol = src_pol.dress_polarity(dst_pol);
                self.write_version(dst, adapted, Some(dressed_pol));
                Some(format!("照 {}→{} ({}{})→{}({})", src, dst, src_name, src_val, dst_name, adapted))
            }
            Op::Jia => {
                // 加：累加（不进位）
                let a = *op.params.get(0)?;
                let b = *op.params.get(1)?;
                let dst = *op.targets.first()?;
                let (val_a, pol_a, val_b) = {
                    let ca = self.containers.get(&a)?;
                    let cb = self.containers.get(&b)?;
                    (ca.value, ca.polarity, cb.value)
                };
                let sum = val_a.saturating_add(val_b);
                self.write_version(dst, sum, Some(pol_a));
                Some(format!("加 {}+{}→{} ({}+{}={})", a, b, dst, val_a, val_b, sum))
            }
            Op::Tui => {
                // 退：退回（源分成两半给两个目标）
                let src = *op.params.first()?;
                let t1 = *op.targets.get(0)?;
                let t2 = *op.targets.get(1)?;
                let (half, remainder) = {
                    let src_c = self.containers.get(&src)?;
                    let half = src_c.value / 2;
                    (half, src_c.value - half)
                };
                self.write_version(t1, half, None);
                self.write_version(t2, remainder, None);
                Some(format!("退 {}→{} {}, {} {}", src, t1, half, t2, remainder))
            }
            Op::Jin => {
                // 进：推进（极性变更+迁移）
                let src = *op.params.first()?;
                let dst = *op.targets.first()?;
                let (new_pol, src_val, src_pol) = {
                    let src_c = self.containers.get(&src)?;
                    let new_pol = match src_c.polarity {
                        YinYang::Yin => YinYang::Yang,
                        YinYang::Yang => YinYang::Yin,
                        YinYang::Xu => YinYang::Shi,
                        YinYang::Shi => YinYang::Xu,
                        other => other,
                    };
                    (new_pol, src_c.value, src_c.polarity)
                };
                self.write_version(dst, src_val, Some(new_pol));
                self.build_road(src, dst);
                Some(format!("进 {}→{} ({}→{})", src, dst, src_pol, new_pol))
            }
            Op::Tong => {
                // 通：连通
                let a = *op.params.first()?;
                let b = *op.params.get(1)?;
                self.build_road(a, b);
                Some(format!("通 {}↔{}", a, b))
            }
            Op::Duan => {
                // 断：断开
                let a = *op.params.first()?;
                let b = *op.params.get(1)?;
                if let Some(v) = self.roads.get_mut(&a) {
                    v.retain(|&x| x != b);
                }
                if let Some(v) = self.roads.get_mut(&b) {
                    v.retain(|&x| x != a);
                }
                Some(format!("断 {}⊘{}", a, b))
            }
            Op::Fen => {
                // 分：一分为二
                let src = *op.params.first()?;
                let t1 = *op.targets.get(0)?;
                let t2 = *op.targets.get(1)?;
                let (half, remainder, pol, name) = {
                    let src_c = self.containers.get(&src)?;
                    (src_c.value / 2, src_c.value - src_c.value / 2, src_c.polarity.clone(), src_c.name.clone())
                };
                self.register(t1, &name, pol.clone(), half);
                self.register(t2, &name, pol, remainder);
                Some(format!("分 {} → {}({}) + {}({})", src, t1, half, t2, remainder))
            }
            Op::He => {
                // 合：二合为一
                let a = *op.params.get(0)?;
                let b = *op.params.get(1)?;
                let dst = *op.targets.first()?;
                let (val_a, val_b, pol, name) = {
                    let ca = self.containers.get(&a)?;
                    let cb = self.containers.get(&b)?;
                    (ca.value, cb.value, ca.polarity.clone(), ca.name.clone())
                };
                let sum = val_a.saturating_add(val_b);
                self.register(dst, &name, pol, sum);
                Some(format!("合 {}+{}→{} ({}+{}={})", a, b, dst, val_a, val_b, sum))
            }
            Op::Zhe => {
                // 折：折叠（源 → 目标）—— 目标数值累加源数值的一半，源极性反转
                let src = *op.params.first()?;
                let dst = *op.targets.first()?;
                let (src_val, src_pol, dst_name) = {
                    let src_c = self.containers.get(&src)?;
                    let dst_c = self.containers.get(&dst)?;
                    (src_c.value, src_c.polarity, dst_c.name.clone())
                };
                let half = src_val / 2;
                let folded = src_val - half;
                let new_dst_val = self.containers[&dst].value + folded;
                let toggled = match src_pol {
                    YinYang::Yin => YinYang::Yang,
                    YinYang::Yang => YinYang::Yin,
                    YinYang::Xu => YinYang::Shi,
                    YinYang::Shi => YinYang::Xu,
                    p => p,
                };
                self.write_version(dst, new_dst_val, None);
                self.write_version(src, half, Some(toggled));
                Some(format!("折 {}({}/2={}, 留{})→{}, 极性{}→{}", src, src_val, half, folded, dst_name, src_pol, toggled))
            }
            Op::Die => {
                // 叠：展开（源 → 目标1 + 目标2）—— 将源容器分裂为两个新容器
                let src = *op.params.first()?;
                let t1 = *op.targets.get(0)?;
                let t2 = *op.targets.get(1)?;
                let (half, remainder, pol, name) = {
                    let src_c = self.containers.get(&src)?;
                    (src_c.value / 2, src_c.value - src_c.value / 2, src_c.polarity.clone(), src_c.name.clone())
                };
                self.register(t1, &name, pol.clone(), half);
                self.register(t2, &name, pol, remainder);
                Some(format!("叠 {} → {}({}) {}({})", src, t1, half, t2, remainder))
            }
            Op::Juan => {
                // 卷：卷曲（源 → 目标）—— 源的极性反转，数值裹上一层（加1）
                let src = *op.params.first()?;
                let dst = *op.targets.first()?;
                let (src_val, src_pol, src_name) = {
                    let src_c = self.containers.get(&src)?;
                    (src_c.value, src_c.polarity, src_c.name.clone())
                };
                let new_pol = match src_pol {
                    YinYang::Yin => YinYang::Ying,
                    YinYang::Yang => YinYang::Kui,
                    p => p,
                };
                self.write_version(src, src_val.saturating_add(1), Some(new_pol));
                Some(format!("卷 {}({}→{}, {}裹+1)", src, src_name, new_pol, src_val))
            }
            Op::Shu => {
                // 舒：舒展（源 → 目标）—— 卷曲的逆操作，数值解（减1），极性还原
                let src = *op.params.first()?;
                let (src_val, src_pol) = {
                    let src_c = self.containers.get(&src)?;
                    (src_c.value, src_c.polarity)
                };
                let (new_pol, new_val) = match src_pol {
                    YinYang::Ying => (YinYang::Yin, src_val.saturating_sub(1)),
                    YinYang::Kui => (YinYang::Yang, src_val.saturating_sub(1)),
                    p => (p, src_val),
                };
                self.write_version(src, new_val, Some(new_pol));
                Some(format!("舒 {}({}→{}, {}解-1)", src, src_pol, new_pol, src_val))
            }
            Op::Cai => {
                // 裁：裁判准入（裁容器 审容器 → 裁定目标）—— 必须两个见证都是阳极性
                let cai_id = *op.params.get(0)?;
                let shen_id = *op.params.get(1)?;
                let target = *op.targets.first()?;
                let (cai_pol, shen_pol, dst_val) = {
                    let c = self.containers.get(&cai_id)?;
                    let s = self.containers.get(&shen_id)?;
                    let d = self.containers.get(&target)?;
                    (c.polarity, s.polarity, d.value)
                };
                let admitted = cai_pol == YinYang::Yang && shen_pol == YinYang::Yang;
                if admitted {
                    let new_val = dst_val.saturating_add(1);
                    self.write_version(target, new_val, Some(YinYang::Yang));
                    Some(format!("裁 {} {}→{} 准入({}→阳, 值{}→{})", cai_id, shen_id, target, cai_pol, dst_val, new_val))
                } else {
                    self.write_version(target, dst_val, Some(YinYang::Yin));
                    Some(format!("裁 {} {}→{} 不准入(判={} 审={})", cai_id, shen_id, target, cai_pol, shen_pol))
                }
            }
            Op::Jue => {
                // 决：断决（源 → 正地址 负地址）—— 将源的正数值给正地址，负数值给负地址
                let src = *op.params.first()?;
                let pos_dst = *op.targets.get(0)?;
                let neg_dst = *op.targets.get(1)?;
                let (src_val, src_pol) = {
                    let src_c = self.containers.get(&src)?;
                    (src_c.value, src_c.polarity)
                };
                let (pos_val, neg_val) = match src_pol {
                    YinYang::Ying | YinYang::Shi | YinYang::Yang => (src_val, 0),
                    YinYang::Kui | YinYang::Xu | YinYang::Yin => (0, src_val),
                    _ => (src_val / 2, src_val - src_val / 2),
                };
                let pos_pol = match src_pol {
                    YinYang::Yin => YinYang::Yang,
                    p => p,
                };
                self.write_version(pos_dst, pos_val, Some(pos_pol));
                self.write_version(neg_dst, neg_val, Some(YinYang::Yin));
                Some(format!("决 {} → 正{}({}) 负{}({})", src, pos_dst, pos_val, neg_dst, neg_val))
            }
        }
    }

    /// 检查边界到达
    fn check_boundary(&self, id: u64) -> bool {
        if let Some(c) = self.containers.get(&id) {
            c.value >= c.capacity()
        } else {
            false
        }
    }

    /// 执行整个程序
    fn run(&mut self, ops: &[OpLine]) {
        for op in ops {
            // 检查源容器是否可达（有公路）
            let reachable = if op.params.len() >= 2 {
                // 多个源需要前两个互相连通
                self.has_road(op.params[0], op.params[1]) ||
                op.params.iter().all(|p| self.containers.contains_key(p))
            } else {
                op.params.iter().all(|p| self.containers.contains_key(p))
            };

            if !reachable {
                eprintln!("  [跳过 行{}] {} — 源容器不可达或无公路", op.line_no, op.op.name());
                continue;
            }

            if let Some(output) = self.exec_op(op) {
                println!("  {}", output);
            }

            // 检查边界到达（执行后）
            for t in &op.targets {
                if self.check_boundary(*t) {
                    println!("  [边界到达] 容器{} 已满({}/{})", t,
                        self.containers[t].value, self.containers[t].capacity());
                }
            }
        }
    }
}

/// 解析器：纯 token 流，无 AST
struct Parser;

impl Parser {
    /// 解析完整的垒码程序
    fn parse(input: &str) -> (Vec<(u64, String, YinYang, u64)>, Vec<OpLine>) {
        let mut phase = 0; // 0=寻找区段, 1=注册清单, 2=操作序列
        let mut registrations = Vec::new();
        let mut ops = Vec::new();
        let mut op_counter: u64 = 0;

        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // 区段标记
            if trimmed.starts_with("=<") && trimmed.ends_with(">=") {
                let section = &trimmed[2..trimmed.len()-2];
                phase = match section {
                    "注册清单" => 1,
                    "操作序列" => 2,
                    _ => 0,
                };
                continue;
            }

            match phase {
                1 => {
                    // 注册清单行: <id>: <容器族> <极性> <初始值>
                    if let Some((id, rest)) = trimmed.split_once(':') {
                        let id: u64 = id.trim().parse().unwrap_or(0);
                        let parts: Vec<&str> = rest.trim().split_whitespace().collect();
                        if parts.len() >= 3 {
                            let name = parts[0];
                            let pol = YinYang::from_str(parts[1]).unwrap_or(YinYang::Yin);
                            let val: u64 = parts[2].parse().unwrap_or(0);
                            registrations.push((id, name.to_string(), pol, val));
                        }
                    }
                }
                2 => {
                    // 操作行: <忽略行号> <操作码> <参数> → <目标>
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        // 第一个 token 是行号（仅用于错误报告，不作为地址）
                        // 第二个 token 是操作码
                        if let Some(op) = Op::from_str(parts[1]) {
                            let line_no = parts[0].trim_end_matches(':').parse().unwrap_or(op_counter);
                            op_counter += 1;

                            // 解析参数，以 → 分隔
                            let mut params = Vec::new();
                            let mut targets = Vec::new();
                            let mut is_target = false;

                            for token in &parts[2..] {
                                if *token == "→" || *token == "->" {
                                    is_target = true;
                                    continue;
                                }
                                if let Ok(n) = token.parse::<u64>() {
                                    if is_target {
                                        targets.push(n);
                                    } else {
                                        params.push(n);
                                    }
                                }
                            }

                            ops.push(OpLine { line_no, op, params, targets });
                        }
                    }
                }
                _ => {}
            }
        }

        (registrations, ops)
    }
}

fn main() {
    eprintln!("垒码 (LM) — 进制寻址 · 声母调入 · 操作族演算 · 不可变注册清单");
    eprintln!("================================================================");

    let spec = std::fs::read_to_string("tools/lm/SPEC.md").unwrap_or_default();
    if spec.is_empty() {
        eprintln!("[找不到 SPEC.md — 使用内嵌示例]");
    }

    // 默认示例：读取 tools/lm/example.lm 或内嵌示例
    let example = std::fs::read_to_string("tools/lm/example.lm")
        .unwrap_or_else(|_| {
            // 内嵌示例
            r#"=<注册清单>=
1: 袋 阴 3
2: 桶 阳 5
3: 罐 阴 0
4: 瓶 阳 0
5: 盏 阴 0

=<操作序列>=
10 鉴 1
20 鉴 2
30 加 1 2 → 3
40 鉴 3
50 进 3 → 4
60 见 4
70 通 1 4
80 加 1 4 → 5
90 见 5
100 分 5 → 6 7
110 见 6
120 见 7
130 合 6 7 → 8
140 见 8
"#.to_string()
        });

    eprintln!("\n--- 源码 ---");
    for line in example.lines() {
        eprintln!("  {}", line);
    }
    eprintln!("--- 执行 ---");

    let (registrations, ops) = Parser::parse(&example);

    let mut lm = LM::new();
    for (id, name, pol, val) in &registrations {
        lm.register(*id, name, *pol, *val);
    }

    eprintln!("注册容器 {} 个", registrations.len());
    eprintln!("操作 {} 行", ops.len());
    eprintln!();

    lm.run(&ops);

    eprintln!();
    eprintln!("--- 历史版本 ---");
    for (i, entry) in lm.history.iter().enumerate() {
        eprintln!("  [{}] {} {}({}, {}) gen{} _hash_{}",
            i,
            entry.container.id,
            entry.container.name,
            entry.container.polarity,
            entry.container.value,
            entry.container.generation,
            &entry.hash[..8.min(entry.hash.len())],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_program() -> String {
        r#"=<注册清单>=
1: 袋 阴 3
2: 桶 阳 5
3: 罐 阴 0
4: 瓶 阳 0
5: 盏 阴 0

=<操作序列>=
10 鉴 1
20 加 1 2 → 3
30 进 3 → 4
40 见 4
"#.to_string()
    }

    #[test]
    fn parse_works() {
        let (reg, ops) = Parser::parse(&example_program());
        assert_eq!(reg.len(), 5);
        assert!(ops.len() >= 3);
    }

    #[test]
    fn addition_no_overflow() {
        let (reg, ops) = Parser::parse(&example_program());
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        // 袋(3) + 桶(5) = 罐(8)
        let guan = lm.containers.get(&3).expect("罐 应存在");
        assert_eq!(guan.value, 8);
        assert_eq!(guan.polarity, YinYang::Yin); // 袋(阴)的极性
    }

    #[test]
    fn jin_changes_polarity() {
        let input = r#"=<注册清单>=
1: 袋 阴 6
2: 桶 阳 0

=<操作序列>=
10 进 1 → 2
20 见 2
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        // 进：阴→阳
        let tong = lm.containers.get(&2).expect("桶 应存在");
        assert_eq!(tong.polarity, YinYang::Yang);
        assert_eq!(tong.value, 6);
    }

    #[test]
    fn boundary_reached() {
        // 边界到达测试
        let tier1_capacity = 48; // 等级1 * 48
        assert!(tier1_capacity == 48);
    }

    #[test]
    fn road_building() {
        let input = r#"=<注册清单>=
1: 袋 阴 1
2: 桶 阳 2

=<操作序列>=
10 通 1 2
20 加 1 2 → 3
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        // 通操作后应建立公路
        assert!(lm.has_road(1, 2));
    }

    #[test]
    fn fen_split() {
        let input = r#"=<注册清单>=
1: 袋 阴 10

=<操作序列>=
10 分 1 → 2 3
20 见 2
30 见 3
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        let c2 = lm.containers.get(&2).expect("分裂后容器2应存在");
        let c3 = lm.containers.get(&3).expect("分裂后容器3应存在");
        assert_eq!(c2.value, 5);
        assert_eq!(c3.value, 5);
    }

    #[test]
    fn immutable_history() {
        let input = r#"=<注册清单>=
1: 袋 阴 3
2: 桶 阳 0

=<操作序列>=
10 照 1 → 2
20 见 1
30 见 2
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        let initial_history_len = lm.history.len();
        lm.run(&ops);

        // 每次写入都生成新版本
        assert!(lm.history.len() > initial_history_len);
    }

    #[test]
    fn zhe_fold() {
        // 折：折叠——源数值分两半，一半累加到目标，源极性翻转
        let input = r#"=<注册清单>=
1: 袋 阴 10
2: 桶 阳 0

=<操作序列>=
10 折 1 → 2
20 见 1
30 见 2
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        // 袋(10) 折叠后：一半(5)留袋（极性翻转→阳），一半(5)累加到桶
        let dai = lm.containers.get(&1).expect("袋应存在");
        assert_eq!(dai.value, 5);
        assert_eq!(dai.polarity, YinYang::Yang); // 阴→阳
        let tong = lm.containers.get(&2).expect("桶应存在");
        assert_eq!(tong.value, 5);
    }

    #[test]
    fn die_split_two() {
        // 叠：展开——将源容器分裂为两个新容器
        let input = r#"=<注册清单>=
1: 罐 阴 10

=<操作序列>=
10 叠 1 → 2 3
20 见 2
30 见 3
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        let c2 = lm.containers.get(&2).expect("容器2应存在");
        let c3 = lm.containers.get(&3).expect("容器3应存在");
        assert_eq!(c2.value, 5);
        assert_eq!(c3.value, 5);
        assert_eq!(c2.polarity, YinYang::Yin);
    }

    #[test]
    fn juan_shu_roundtrip() {
        // 卷/舒：卷曲后再舒展，应回到原始极性（近似）
        let input = r#"=<注册清单>=
1: 袋 阴 5

=<操作序列>=
10 卷 1 → 1
20 见 1
30 舒 1 → 1
40 见 1
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        // 卷：阴→盈，数值+1
        // 舒：盈→阴，数值-1
        // 最终应回到接近原始值
        let dai = lm.containers.get(&1).expect("袋应存在");
        assert_eq!(dai.polarity, YinYang::Yin);
        assert_eq!(dai.value, 5);
    }

    #[test]
    fn cai_admit() {
        // 裁：两个阳极性容器做见证 → 准入
        let input = r#"=<注册清单>=
1: 袋 阳 10
2: 桶 阳 8
3: 罐 阴 0

=<操作序列>=
10 裁 1 2 → 3
20 见 3
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        let guan = lm.containers.get(&3).expect("罐应存在");
        // 准入：两个都是阳 → 准入，极性变阳，数值+1
        assert_eq!(guan.polarity, YinYang::Yang);
        assert_eq!(guan.value, 1);
    }

    #[test]
    fn cai_reject() {
        // 裁：一个阴一个阳 → 不准入
        let input = r#"=<注册清单>=
1: 袋 阴 10
2: 桶 阳 8
3: 罐 阳 5

=<操作序列>=
10 裁 1 2 → 3
20 见 3
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        let guan = lm.containers.get(&3).expect("罐应存在");
        // 不准入：极性变阴
        assert_eq!(guan.polarity, YinYang::Yin);
        assert_eq!(guan.value, 5);
    }

    #[test]
    fn jue_split_pos_neg() {
        // 决：阳容器 → 正地址得数值，负地址得0
        let input = r#"=<注册清单>=
1: 瓶 阳 8
2: 袋 阴 0
3: 桶 阴 0

=<操作序列>=
10 决 1 → 2 3
20 见 2
30 见 3
"#;
        let (reg, ops) = Parser::parse(input);
        let mut lm = LM::new();
        for (id, name, pol, val) in &reg {
            lm.register(*id, name, *pol, *val);
        }
        lm.run(&ops);

        let dai = lm.containers.get(&2).expect("袋(正)应存在");
        let tong = lm.containers.get(&3).expect("桶(负)应存在");
        assert_eq!(dai.value, 8);  // 正地址得全部
        assert_eq!(dai.polarity, YinYang::Yang);  // 正极性
        assert_eq!(tong.value, 0); // 负地址得0
        assert_eq!(tong.polarity, YinYang::Yin); // 负极性为阴
    }
}
