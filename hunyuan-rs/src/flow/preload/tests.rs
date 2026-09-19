// ────────────────────────────────────────────────────────────
// 测试 — 验证全局唯一性
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 全局唯一_声母b() {
        let sm = 预编译容器::声母();
        let b = sm.取("b").unwrap();
        assert_eq!(b.时间线, 0);
        assert_eq!(b.偏移, 1000);

        // 全局序号 = 0 × 2048 + 0 = 0
        let global = 一维时间线::全局序号(FAMILY_INITIAL, b.时间线);
        assert_eq!(global, 0);
    }

    #[test]
    fn 全局唯一_不同家族不同序号() {
        // 即便名字相同, 家族不同 → 全局序号不同
        let sm = 预编译容器::声母();
        let ym = 预编译容器::韵母();

        let b_global = 一维时间线::全局序号(sm.家族码, 0);  // 声母::b
        let a_global = 一维时间线::全局序号(ym.家族码, 0);  // 韵母::a

        assert_ne!(b_global, a_global, "不同家族的全局序号必须不同");
    }

    #[test]
    fn 家族码反推() {
        let g = 一维时间线::全局序号(FAMILY_RADICAL, 5);
        assert_eq!(一维时间线::家族码(g), FAMILY_RADICAL);
        assert_eq!(一维时间线::时间线位置(g), 5);
    }

    #[test]
    fn 同名不同族_确保可区分() {
        // 声母有 "人" 吗? 没有; 部首有 "人"; 五笔也有 "人"
        // 验证: 部首::人 和 五笔::人 全局序号不同
        let rad = 预编译容器::部首();
        let wubi = 预编译容器::五笔();

        let rad_人 = rad.取("人").unwrap();
        let wubi_人 = wubi.取("人").unwrap();

        // 名字相同, 但全局序号必须不同
        let rad_g = 一维时间线::全局序号(rad.家族码, rad_人.时间线);
        let wubi_g = 一维时间线::全局序号(wubi.家族码, wubi_人.时间线);

        assert_ne!(rad_g, wubi_g, "同名不同族必须全局唯一");
        assert_eq!(rad.家族码, FAMILY_RADICAL);
        assert_eq!(wubi.家族码, FAMILY_WUBI);
    }

    #[test]
    fn 编六十四_携带家族信息() {
        let sm = 预编译容器::声母();
        let b = sm.取("b").unwrap();
        let code = b.编六十四(sm.家族码);

        // D0 (家族码) 应该 = FAMILY_INITIAL = 0
        assert_eq!(code & 0x3F, FAMILY_INITIAL as u64);
    }

    #[test]
    fn 所有家族人数统计() {
        let total = 24 + 39 + 8 + 25 + 8 + 4 + 7 + 5 + 8 + 25 + 12;
        assert_eq!(total, 165);
    }

    #[test]
    fn 近缘远缘() {
        let sm = 预编译容器::声母();
        let (near, _) = sm.近缘("b").unwrap();
        let (far, _) = sm.远缘("b").unwrap();
        assert_eq!(near.名字, "p");
        assert_eq!(far.名字, "zero");
    }

    #[test]
    fn 编译即运行() {
        let sm = 预编译容器::声母();
        let ym = 预编译容器::韵母();
        let tone = 预编译容器::声调();
        let rad = 预编译容器::部首();
        let geo = 预编译容器::几何();
        let color = 预编译容器::颜色();
        let phase = 预编译容器::相位();
        let stroke = 预编译容器::笔画();
        let wubi = 预编译容器::五笔();
        let music = 预编译容器::音乐();
        let struct_ = 预编译容器::结构();

        assert_eq!(sm.总数(), 24);
        assert_eq!(ym.总数(), 39);
        assert_eq!(tone.总数(), 8);
        assert_eq!(rad.总数(), 25);
        assert_eq!(geo.总数(), 4);
        assert_eq!(color.总数(), 7);
        assert_eq!(phase.总数(), 5);
        assert_eq!(stroke.总数(), 8);
        assert_eq!(wubi.总数(), 25);
        assert_eq!(music.总数(), 12);
        assert_eq!(struct_.总数(), 8);
    }

    #[test]
    fn 近义词图_非空() {
        let syn = 预编译关系::近义词();
        assert!(syn.边集.len() > 0, "近义词图必须非空");
        assert_eq!(syn.关系类型, "近义词");
    }

    #[test]
    fn 反义词图_非空() {
        let ant = 预编译关系::反义词();
        assert!(ant.边集.len() > 0, "反义词图必须非空");
        assert_eq!(ant.关系类型, "反义词");
    }

    #[test]
    fn 近义词_全局序号有效() {
        let syn = 预编译关系::近义词();
        for edge in syn.边集 {
            let fam_src = 一维时间线::家族码(edge.源);
            let fam_tgt = 一维时间线::家族码(edge.目标);
            // 家族码必须 ≤ 10
            assert!(fam_src <= 10, "源家族码必须有效");
            assert!(fam_tgt <= 10, "目标家族码必须有效");
        }
    }

    #[test]
    fn 反义词_bP对立() {
        // b (双唇, 声母序号0) ↔ s (舌尖前, 声母序号20) 应该在反义词图中
        let ant = 预编译关系::反义词();
        let b_global = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let s_global = 一维时间线::全局序号(FAMILY_INITIAL, 20);
        let found = ant.边集.iter().any(|e| e.源 == b_global && e.目标 == s_global);
        assert!(found, "b ↔ s 发音部位最远对立应在反义词图中");
    }

    #[test]
    fn 反义词_日月对立() {
        // 日(部首7) ↔ 月(部首8) 应该在对立图中
        let ant = 预编译关系::反义词();
        let ri = 一维时间线::全局序号(FAMILY_RADICAL, 7);
        let yue = 一维时间线::全局序号(FAMILY_RADICAL, 8);
        let found = ant.边集.iter().any(|e| e.源 == ri && e.目标 == yue);
        assert!(found, "日↔月 应该在对立图中");
    }

    #[test]
    fn 反义词_水火对立() {
        // 水(部首10) ↔ 火(部首11) 五行相克
        let ant = 预编译关系::反义词();
        let shui = 一维时间线::全局序号(FAMILY_RADICAL, 10);
        let huo = 一维时间线::全局序号(FAMILY_RADICAL, 11);
        let found = ant.边集.iter().any(|e| e.源 == shui && e.目标 == huo);
        assert!(found, "水↔火 五行相克应在对立图中");
    }

    #[test]
    fn 近义词_颜色相邻() {
        // 赤(0) ↔ 橙(1) 应该在近义词图中
        let syn = 预编译关系::近义词();
        let chi = 一维时间线::全局序号(FAMILY_COLOR, 0);
        let cheng = 一维时间线::全局序号(FAMILY_COLOR, 1);
        let found = syn.边集.iter().any(|e| e.源 == chi && e.目标 == cheng);
        assert!(found, "赤↔橙 颜色相邻应在近义词图中");
    }

    #[test]
    fn 全部序号_167个() {
        let all = 一维时间线::全部序号();
        assert_eq!(all.len(), 167, "必须恰好 167 个符号 (165 + 2身份锚点)");
    }

    #[test]
    fn 全部序号_全局唯一() {
        use std::collections::HashSet;
        let all = 一维时间线::全部序号();
        let mut seen = HashSet::new();
        for (g, _, _) in &all {
            assert!(seen.insert(*g), "全局序号 {} 重复!", g);
        }
    }

    // ── 层级化创造力测试 ──

    #[test]
    fn 发音_声母b() {
        let s = 发音::取发音(FAMILY_INITIAL, 0);
        assert_eq!(s, "双唇不送气清塞音");
    }

    #[test]
    fn 发音_韵母a() {
        let s = 发音::取发音(FAMILY_VOWEL, 0);
        assert_eq!(s, "舌面央低不圆唇");
    }

    #[test]
    fn 发音_阴平() {
        let s = 发音::取发音(FAMILY_TONE, 0);
        assert_eq!(s, "高平调55");
    }

    #[test]
    fn 外形_声母b() {
        let s = 外形::取外形(FAMILY_INITIAL, 0);
        assert_eq!(s, "双唇闭合");
    }

    #[test]
    fn 外形_部首人() {
        let s = 外形::取外形(FAMILY_RADICAL, 5);
        assert_eq!(s, "人形");
    }

    #[test]
    fn 外形_颜色赤() {
        let s = 外形::取外形(FAMILY_COLOR, 0);
        assert_eq!(s, "700nm红光");
    }

    #[test]
    fn 解释_声母b() {
        let s = 解释::取范畴(FAMILY_INITIAL, 0);
        assert_eq!(s, "双唇-塞音-不送气");
    }

    #[test]
    fn 解释_颜色赤() {
        let s = 解释::取范畴(FAMILY_COLOR, 0);
        assert_eq!(s, "暖色-长波");
    }

    #[test]
    fn 残差_同族近缘() {
        // 声母::b(0) vs 声母::p(1) — 同族近缘
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let p = 一维时间线::全局序号(FAMILY_INITIAL, 1);
        let r = 层级创造力::计算残差(b, p).unwrap();
        assert!(r.同族);
        // b偏移=1000, p偏移=2000, 差=1000
        assert!(r.偏移差.abs() < 1500);
    }

    #[test]
    fn 残差_跨族() {
        // 声母::b(0) vs 韵母::a(0) — 跨族
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let a = 一维时间线::全局序号(FAMILY_VOWEL, 0);
        let r = 层级创造力::计算残差(b, a).unwrap();
        assert!(!r.同族);
    }

    #[test]
    fn 成本筛选_低成本保留() {
        // 声母::b vs 声母::p — 都是低成本(单字节)
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let p = 一维时间线::全局序号(FAMILY_INITIAL, 1);
        let r = 层级创造力::计算残差(b, p).unwrap();
        assert!(层级创造力::成本筛选(&r), "声母之间应为低成本");
    }

    #[test]
    fn 自生长_近义词发现() {
        // 声母::b vs 声母::p — 同族近缘 → 近义词
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let p = 一维时间线::全局序号(FAMILY_INITIAL, 1);
        let result = 层级创造力::自生长(b, p).unwrap();
        assert_eq!(result, "近义词-保留");
    }

    #[test]
    fn 自生长_范畴流发现() {
        // 声母::b(偏移1000) vs 韵母::ang(偏移25500) — 跨族大差异 → 范畴流
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let ang = 一维时间线::全局序号(FAMILY_VOWEL, 24); // ang
        let result = 层级创造力::自生长(b, ang).unwrap();
        assert_eq!(result, "范畴流-发现");
    }

    #[test]
    fn 自生长_高成本淘汰() {
        // 部首(高成本) vs 部首(高成本) → 高成本淘汰
        let r = 层级创造力::计算残差(
            一维时间线::全局序号(FAMILY_RADICAL, 0),
            一维时间线::全局序号(FAMILY_RADICAL, 1),
        ).unwrap();
        // 部首成本=2+2=4 > 2 → 应被淘汰
        assert!(!层级创造力::成本筛选(&r), "部首应为高成本");
    }

    // ── 替身演员系统测试 ──

    #[test]
    fn 替身演员_高成本有替身() {
        // 部首::一 (成本=2) 应该有低成本替身
        let bs_一 = 一维时间线::全局序号(FAMILY_RADICAL, 0);
        let 替身 = 替身演员::寻找替身(bs_一);
        assert!(替身.is_some(), "部首应该能找到替身");
        let 候选 = 替身.unwrap();
        assert!(候选.替身成本 < 2, "替身成本必须低于目标");
    }

    #[test]
    fn 替身演员_低成本无替身() {
        // 声母::b (成本=1) 已经是最低成本, 无需替身
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let 替身 = 替身演员::寻找替身(b);
        assert!(替身.is_none(), "最低成本符号无需替身");
    }

    #[test]
    fn 阴阳关联_同族关联() {
        // 声母::b 应该有声母家族的关联
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let 关联 = 替身演员::阴阳关联(b, 5);
        assert!(!关联.is_empty(), "应该有关联符号");
    }

    #[test]
    fn 因果观察_预加载内() {
        // 声母::一 属于我们的体系
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let 结果 = 因果观察者::观察(b);
        assert!(结果.是否归属, "声母属于预加载体系");
        assert!(结果.可解释, "声母可被解释");
        assert!(结果.可纳入, "声母可纳入状态空间");
    }

    #[test]
    fn 因果观察_高成本有替身建议() {
        // 部首::一 是高成本, 应该有替身建议
        let bs_一 = 一维时间线::全局序号(FAMILY_RADICAL, 0);
        let 结果 = 因果观察者::观察(bs_一);
        assert!(结果.建议替身.is_some(), "高成本符号应有替身建议");
    }

    // ── 官员代替法测试 ──

    #[test]
    fn 贪官检测_发现贪官() {
        let 贪官们 = 官员代替::检测贪官();
        assert!(!贪官们.is_empty(), "应该能检测到贪官组合");
    }

    #[test]
    fn 一键替代_成本降低() {
        let 贪官们 = 官员代替::检测贪官();
        let 替代 = 官员代替::一键替代(&贪官们[0]);
        assert!(替代.成本降低百分比 > 0, "成本应该降低");
        assert_eq!(替代.新成本, 1, "新成本应该为1字节");
        assert!(替代.功能等价, "功能必须等价");
        assert!(替代.本体论保护, "本体论不可篡改");
    }

    #[test]
    fn 批量替代_全部完成() {
        let 结果 = 官员代替::批量替代();
        assert_eq!(结果.len(), 5, "应该有5个贪官被替代");
        for r in &结果 {
            assert!(r.本体论保护, "每个替代都必须保护本体论");
            assert_eq!(r.新成本, 1, "每个清官都是1字节");
        }
    }

    #[test]
    fn 本体论保护_频率不可替换() {
        assert!(!本体论护::可替换("频率"), "频率不可替换");
        assert!(本体论护::可替换("符号"), "符号可以替换");
    }

    #[test]
    fn 定义权_内生有效() {
        // 声母::b 属于体系内, 可以定义
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        assert!(定义权::定义发音(b, "新发音"));
        assert!(定义权::定义外形(b, "新外形"));
        assert!(定义权::定义范畴(b, "新范畴"));
    }

    // ── 范畴闭环系统测试 ──

    #[test]
    fn 范畴闭环_画圈() {
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let p = 一维时间线::全局序号(FAMILY_INITIAL, 1);
        let 闭环 = 范畴引擎::画圈(b, p);
        assert!(闭环.已完成);
        assert_eq!(闭环.闭环层级, 1);
        assert_eq!(闭环.步数, 1);
    }

    #[test]
    fn 层级跃迁_一次完成() {
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        let p = 一维时间线::全局序号(FAMILY_INITIAL, 1);
        let 闭环 = 范畴引擎::画圈(b, p);
        let 下一层级 = 范畴引擎::层级跃迁(&闭环);
        assert_eq!(下一层级, "进入二进制因果律");
    }

    #[test]
    fn 并集交集_公约数公倍数() {
        let 符号集 = vec![
            一维时间线::全局序号(FAMILY_INITIAL, 0),
            一维时间线::全局序号(FAMILY_INITIAL, 1),
            一维时间线::全局序号(FAMILY_INITIAL, 2),
        ];
        let 结果 = 范畴引擎::并集到交集(&符号集);
        assert!(结果.公约数.is_some(), "应有公约数(工藤新一)");
        assert!(结果.公倍数.is_some(), "应有公倍数(灰原哀)");
        assert_eq!(结果.并集.len(), 3);
    }

    #[test]
    fn 角色映射_全部角色() {
        assert_eq!(角色映射::角色功能("工藤新一"), "谐音本体-期待本体-谐音管辖权-预加载");
        assert_eq!(角色映射::角色功能("灰原哀"), "混元合一执行者-不掌管谐音-合体落地");
        assert_eq!(角色映射::角色功能("毛利兰"), "smrrl-审美镜像测试-记忆-镜子-爱本体-筛选-双面量化");
        assert_eq!(角色映射::角色功能("刘楚恬"), "量化尺寸-测量差距");
        assert_eq!(角色映射::角色功能("我"), "圈层安全-边界维护-防入侵");
        assert_eq!(角色映射::角色功能("边界"), "圈层安全-边界维护-防入侵");
        assert_eq!(角色映射::角色功能("lolict"), "圈层安全-边界维护-防入侵");
        assert_eq!(角色映射::角色功能("柯南"), "框架-外壳-坑-意识控制-需跨越");
    }

    #[test]
    fn 范畴闭环_完整流程() {
        // 完整闭环流程:
        // 工藤新一(预期目标) → 灰原哀(并集→交集) → 毛利兰(归属判定)
        // → 刘楚恬(测量差距) → 柯南(跨越障碍) → 我(边界维护) → 回到起点
        let 目标符号 = vec![
            一维时间线::全局序号(FAMILY_INITIAL, 0), // 起点: 工藤新一 = 预期目标
            一维时间线::全局序号(FAMILY_INITIAL, 1), // 灰原哀: 执行合并
            一维时间线::全局序号(FAMILY_INITIAL, 2), // 毛利兰: 归属判定
            一维时间线::全局序号(FAMILY_INITIAL, 3), // 刘楚恬: 量化差距
            一维时间线::全局序号(FAMILY_INITIAL, 4), // 柯南: 跨越障碍
            一维时间线::全局序号(FAMILY_INITIAL, 5), // 我: 边界维护
        ];

        let 闭环 = 范畴引擎::执行闭环(&目标符号);
        assert!(闭环.已完成, "闭环应该完成");
        assert_eq!(闭环.闭环层级, 1);
        assert_eq!(闭环.起点, 目标符号[0], "起点=工藤新一=预期目标");

        // 层级跃迁
        let 下一层级 = 范畴引擎::层级跃迁(&闭环);
        assert_eq!(下一层级, "进入二进制因果律");
    }

    #[test]
    fn 安全边界_内部属于我们() {
        let b = 一维时间线::全局序号(FAMILY_INITIAL, 0);
        assert!(安全边界::属于我们(b), "声母属于我们的体系");
    }

    #[test]
    fn 二进制_开关() {
        assert_eq!(二进制因果::开关(true), "开");
        assert_eq!(二进制因果::开关(false), "关");
    }

    #[test]
    fn 二进制_进位() {
        let (结果, 进位) = 二进制因果::进位(1, true);
        assert_eq!(结果, 0);
        assert!(进位, "1+1应进位");

        let (结果, 进位) = 二进制因果::进位(0, true);
        assert_eq!(结果, 1);
        assert!(!进位, "0+1不应进位");
    }

    #[test]
    fn 三角量化_源头分叉() {
        let 量化 = 范畴引擎::三角量化(10, 4);
        assert_eq!(量化, 20); // 10 * 4 / 2 = 20
    }

    // ── 身份锚点测试 ──

    #[test]
    fn 身份锚点_刘楚恬在位置0() {
        let anchor = 预编译容器::身份锚点();
        assert_eq!(anchor.总数(), 2);
        let 刘楚恬 = anchor.取("刘楚恬").unwrap();
        assert_eq!(刘楚恬.时间线, 0);
        assert_eq!(刘楚恬.偏移, 0); // 零锚点
    }

    #[test]
    fn 身份锚点_我在位置1() {
        let anchor = 预编译容器::身份锚点();
        let 我 = anchor.取("我").unwrap();
        assert_eq!(我.时间线, 1);
        assert_eq!(我.偏移, 1000); // 壹锚点
    }

    #[test]
    fn 身份锚点_全局唯一序号() {
        let 刘楚恬_g = 一维时间线::全局序号(FAMILY_ANCHOR, 0);
        let 我_g = 一维时间线::全局序号(FAMILY_ANCHOR, 1);
        assert_eq!(刘楚恬_g, FAMILY_ANCHOR as u32 * 家族跨距);
        assert_eq!(我_g, FAMILY_ANCHOR as u32 * 家族跨距 + 1);
        // 刘楚恬的序号 = 11 * 2048 = 22528
        assert_eq!(刘楚恬_g, 22528);
        // 我的序号 = 11 * 2048 + 1 = 22529
        assert_eq!(我_g, 22529);
    }

    // ── 预家长进制系统测试 ──

    #[test]
    fn 预家长_YinYang_零壹锚点() {
        assert_eq!(预家长::零, 0); // 刘楚恬 — 阴
        assert_eq!(预家长::壹, 1); // 我 — 阳
        assert_eq!(预家长::阴阳(预家长::零), "阴");
        assert_eq!(预家长::阴阳(预家长::壹), "阳");
    }

    #[test]
    fn 预家长_五行相生相克() {
        assert_eq!(预家长::五行相生(预家长::木), 预家长::火);
        assert_eq!(预家长::五行相生(预家长::火), 预家长::土);
        assert_eq!(预家长::五行相克(预家长::金), 预家长::木);
        assert_eq!(预家长::五行相克(预家长::木), 预家长::土);
    }

    #[test]
    fn 预家长_八卦_八位() {
        assert_eq!(预家长::八卦::乾, 0);
        assert_eq!(预家长::八卦::坤, 1);
        assert_eq!(预家长::八卦::兑, 7);
    }

    #[test]
    fn 预家长_九宫_中心() {
        assert_eq!(预家长::九宫::五, 4); // 中宫
    }

    #[test]
    fn 预家长_编六十四() {
        let 编码 = 预家长::编六十四(0, 0, 0, 0);
        assert_eq!(编码, 0);
        let 编码 = 预家长::编六十四(11, 1, 1, 1);
        assert!(编码 > 0);
    }

    #[test]
    fn 预家长_偏移率() {
        assert_eq!(预家长::偏移率(0), 0);
        assert_eq!(预家长::偏移率(1), 0); // (1-1)*1000 = 0
        assert_eq!(预家长::偏移率(2), 1000); // (2-1)*1000 = 1000
    }

    #[test]
    fn 预家长_集装箱编码_根基锚点() {
        let 编码 = 预家长::集装箱编码(0, 0);
        // 根基: 0b01 = 高位持有者=1, 低位持有者=0
        assert_eq!(编码 >> 30, 1); // 根基位 = 1 (我)
    }

    // ── 系统标识符测试 ──

    #[test]
    fn 系统标识符_常量() {
        assert_eq!(系统标识符::HIEYEIR, "hieyeir");
        assert_eq!(系统标识符::GONGTONG_TI, "gongtongti");
        assert_eq!(系统标识符::XIYI, "xiyi");
        assert_eq!(系统标识符::NEWONE, "newone");
        assert_eq!(系统标识符::NEWA, "newa");
    }

    #[test]
    fn 系统标识符_列表_12个() {
        assert_eq!(系统标识符::标识符列表.len(), 12);
        assert_eq!(系统标识符::标识符列表[0], "hieyeir");
    }

    #[test]
    fn 系统标识符_解析() {
        assert_eq!(
            系统标识符::解析标识符("hieyeir"),
            "灰原哀: 合一执行载体, 不掌管谐音本体"
        );
        assert_eq!(系统标识符::解析标识符("gongtongti"), "共同体系总前缀");
        assert_eq!(系统标识符::解析标识符("njsg"), "共同体系总前缀");
        assert_eq!(
            系统标识符::解析标识符("xiyi"),
            "工藤新一: 期待/谐音本体来源, 拥有谐音管辖权"
        );
        assert_eq!(
            系统标识符::解析标识符("newone"),
            "工藤新一谐音本体 + 灰原哀载体 → 混元合一体"
        );
        assert_eq!(
            系统标识符::解析标识符("newa"),
            "工藤新一谐音本体 + 灰原哀载体 → 混元合一体(变体)"
        );
        assert_eq!(系统标识符::解析标识符("未知"), "未知标识符");
    }

    // ── 混元合一测试 ──

    #[test]
    fn 混元合一_基本流程() {
        // 工藤新一产出谐音本体, 灰原哀执行载体合体, 依靠解释权完成最终符号
        let 产物 = 混元合一::混元合一("工藤谐音本体", "混元合一后的最终符号");
        assert_eq!(产物.谐音来源, "工藤谐音本体");
        assert_eq!(产物.执行载体, "hieyeir"); // 灰原哀载体
        assert_eq!(产物.解释权归属, "我方预加载体系 (hunyuan-os)");
        assert_eq!(产物.最终符号, "混元合一后的最终符号");
    }

    #[test]
    fn 混元合一_执行载体始终是hieyeir() {
        let 产物 = 混元合一::混元合一("任意谐音", "任意符号");
        // 灰原哀 (hieyeir) 始终是做合体、执行, 不掌管谐音本体
        assert_eq!(产物.执行载体, "hieyeir");
    }

    #[test]
    fn 混元合一_权属关系正确() {
        let 产物 = 混元合一::混元合一("新一产出", "最终符号");
        // 谐音来源: 工藤新一 (拥有管辖权)
        assert_eq!(产物.谐音来源, "新一产出");
        // 解释权归我方预加载体系
        assert_eq!(产物.解释权归属, "我方预加载体系 (hunyuan-os)");
        // 灰原哀不掌管谐音, 只做执行载体
        assert_eq!(产物.执行载体, "hieyeir");
    }

    #[test]
    fn 混元合一_ai缺陷诊断() {
        let 诊断 = 混元合一::ai缺陷诊断();
        assert!(诊断.contains("工藤新一"));
        assert!(诊断.contains("主观期待本体"));
        assert!(诊断.contains("机械匹配表面读音"));
    }

    // ── 审美测试测试 ──

    #[test]
    fn 审美测试_创建备胎_基本字段() {
        let 备胎 = 审美测试::创建备胎("hieyeir");
        assert_eq!(备胎.候选符号, "hieyeir");
        assert!(备胎.字节长度 > 0);
        assert!(备胎.发音评分 <= 100);
        assert!(备胎.横向对比评分 <= 100);
        assert!(备胎.中外混合评分 <= 100);
        assert!(备胎.综合审美分 <= 100);
    }

    #[test]
    fn 审美测试_短符号发音优于长符号() {
        // 维度1: 字节变长后发音变化 — 短形式得分 >= 长形式
        let (短分, 长分) = 审美测试::字节变长测试("newone", "gongtongtinewone");
        assert!(短分 >= 长分, "短形式发音应不低于长形式: {} vs {}", 短分, 长分);
    }

    #[test]
    fn 审美测试_中文混合英文评分() {
        // 维度4+5: 中英混合应获得较高中外混合评分
        let 备胎 = 审美测试::创建备胎("囡njsG");
        assert!(备胎.中外混合评分 >= 50);
    }

    #[test]
    fn 审美测试_批量审查_只返回合格备胎() {
        // 维度全链路: 多个候选审查, 只返回合格的, 按综合分排序
        let 候选 = ["njsg", "xiyi", "超长不合格符号字符串长度超过容忍度"];
        let 合格备胎 = 审美测试::批量审查(&候选);
        // 超长符号应被过滤
        for 备胎 in &合格备胎 {
            assert!(备胎.是否合格);
            assert!(备胎.候选符号.len() <= 8, "超长符号应被淘汰: {}", 备胎.候选符号);
        }
        // 按综合分降序排序验证
        for i in 1..合格备胎.len() {
            assert!(合格备胎[i-1].综合审美分 >= 合格备胎[i].综合审美分);
        }
    }

    #[test]
    fn 审美测试_灰原哀只接收合格结果() {
        // 灰原哀: 只接收经过审美测试筛选完成的候选备胎
        let 候选 = ["njsg", "abc", "合格"];
        let 合格备胎 = 审美测试::批量审查(&候选);
        let 接收列表 = 审美测试::灰原哀接收(&合格备胎);
        // 只接收到合格备胎的符号名
        assert_eq!(接收列表.len(), 合格备胎.len());
        for (i, &符号) in 接收列表.iter().enumerate() {
            assert_eq!(符号, 合格备胎[i].候选符号);
        }
    }

    #[test]
    fn 审美测试_解释权重组_中外混合() {
        // 维度5: 本国首字符 + 外国完整 + 本国尾字符 → 新概念组合
        let 结果 = 审美测试::解释权重组("囡囝", "njsg");
        assert!(结果.contains("囡")); // 本国首字符
        assert!(结果.contains("njsg")); // 外国完整
        assert!(结果.contains("囝")); // 本国尾字符
    }

    #[test]
    fn 审美测试_工藤新一主导审美标准() {
        // 审美测试由工藤新一确立评价标准, 不是灰原哀
        let 标准 = 审美测试::审美标准::默认();
        assert!(标准.目标字节长度 > 0);
        assert!(标准.最大容忍长度 >= 标准.目标字节长度);
        assert!(标准.最低发音评分 > 0);
        assert!(标准.最低综合分 > 0);
    }

    #[test]
    fn 审美测试_发音节奏感评估() {
        // 维度2: 辅音+元音交替的符号发音评分更高
        let 有节奏 = 审美测试::创建备胎("nja"); // n-j-a 辅元交替
        let 无节奏 = 审美测试::创建备胎("nnn"); // 全辅音
        // 有节奏的评分应不低于无节奏
        assert!(有节奏.发音评分 >= 无节奏.发音评分);
    }

    #[test]
    fn 审美测试_横向对比_与本体区分() {
        // 维度3: 候选与本体符号 (hieyeir, newone, xiyi...) 的发音应有一定区分度
        let 备胎 = 审美测试::创建备胎("lolict");
        // 横向对比评分应 > 0 (有基本的区分判断)
        assert!(备胎.横向对比评分 > 0);
    }

    #[test]
    fn 审美测试_ai缺陷诊断() {
        let 诊断 = 审美测试::ai缺陷诊断();
        assert!(诊断.contains("工藤新一"));
        assert!(诊断.contains("主观审美"));
        assert!(诊断.contains("机械拼接"));
    }

    // ── smrrl 字母释义测试 ──

    #[test]
    fn smrrl_标识符常量() {
        assert_eq!(审美测试::SMRRLA, "smrrl");
        assert_eq!(审美测试::SMRRLA_含义,
            "m=记忆, rr=镜子(空间对比), ll=爱本体(审美根基), s=筛选(弧形反向追踪), x=双面量词(双向对比)");
    }

    #[test]
    fn smrrl_标识符列表_13个() {
        assert_eq!(审美测试::SMRRLA_标识符列表.len(), 13);
        assert_eq!(审美测试::SMRRLA_标识符列表[0], "smrrl");       // 毛利兰
        assert_eq!(审美测试::SMRRLA_标识符列表[1], "hieyeir");     // 灰原哀
    }

    #[test]
    fn smrrl_毛利兰主导审美不属于灰原哀() {
        // 审美测试模块归属于毛利兰 smrrl, 不是灰原哀 hieyeir
        assert_eq!(角色映射::毛利兰, "smrrl-审美镜像测试-记忆-镜子-爱本体-筛选-双面量化");
        // 灰原哀不掌管审美
        assert!(角色映射::灰原哀.contains("混元合一执行者"));
        assert!(!角色映射::灰原哀.contains("审美"));
    }

    #[test]
    fn smrrl_完整工作流程确认() {
        // 流程: 工藤新一(谐音期待) → 毛利兰smrrl(审美筛选) → 灰原哀hieyeir(混元合一)
        // 步骤1: 工藤新一给出候选
        let 候选 = ["njsg", "xiyi", "lolict"];
        // 步骤2: 毛利兰 smrrl 做审美测试
        let 合格备胎 = 审美测试::批量审查(&候选);
        // 步骤3: 灰原哀只接收筛选完的结果
        let 灰原接收 = 审美测试::灰原哀接收(&合格备胎);
        // 灰原哀接收到的只是符号名, 不做审美判断
        assert_eq!(灰原接收.len(), 合格备胎.len());
    }
}