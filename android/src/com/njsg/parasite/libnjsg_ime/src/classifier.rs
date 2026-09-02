// ============================================================================
// 囡囝共生 · MetaIME 第4层: 声母-韵母分类器 — 候选字生成
// ============================================================================
//
// 当前层号: 4
// 当前声母: fen (分)
// 当前韵母组合: fen + lei = 分类 (声母韵母坐标 → 音节约束 → 候选集)
// 折叠状态: 声母-韵母-候字 三级映射展开
// 下一步操作: 接收声母坐标+韵母坐标 → 查表产候字 → 回传第3层
//
// 锚定法则:
//   0 = 刘楚恬 (隐匿侧, 韵母水深)
//   1 = 我 (显现侧, 声母山立)
//
// 容器守恒: 分类器只从声母韵母坐标推测候字,不更改坐标本身
// ============================================================================

/// 声母-韵母组合 → 候选字表
/// 采用两级映射: 声母坐标 → (韵母坐标 → [候字])
pub struct ShengYunEngine {
    table: ShengYunTable,
}

/// 声母-韵母映射表
/// 维度: [21声母 x 35韵母] → ~735 音节约束
struct ShengYunTable {
    // 简化: 以 (声母idx, 韵母idx) 为键, 候选字列表为值
    entries: Vec<((u8, u8), Vec<String>)>,
}

impl ShengYunEngine {
    pub fn new() -> Self {
        Self {
            table: ShengYunTable::build_default(),
        }
    }

    /// 重新装载声母-韵母表
    pub fn reload(&mut self) {
        self.table = ShengYunTable::build_default();
    }

    /// 音节约束产候字
    /// 若声母坐标和韵母坐标都可识别, 返回候选集
    /// 否则返回空
    pub fn generate_candidates(&self, code_point: u32, sm: u8, ym: u8) -> Vec<String> {
        // 若码点是 ASCII 字母直接消耗, 不产候字
        if code_point < 128 {
            // 查声母-韵母组合表
            return self.sm_ym_lookup(sm, ym);
        }
        // 非 ASCII (可能是直接汉字输入): 原样作为候字
        if let Some(ch) = char::from_u32(code_point) {
            return vec![ch.to_string()];
        }
        vec![]
    }

    /// 核心: 声母坐标 + 韵母坐标 → 候字列表
    fn sm_ym_lookup(&self, sm: u8, ym: u8) -> Vec<String> {
        for ((s, y), cands) in &self.table.entries {
            if *s == sm && *y == ym {
                return cands.clone();
            }
        }
        vec![]
    }

    /// 取状态描述
    pub fn state_desc(&self) -> String {
        format!("音节约束={} 条目", self.table.entries.len())
    }
}

impl ShengYunTable {
    /// 内置声母-韵母 → 候字映射
    /// 覆盖高频: 约 30 声母韵母组合
    fn build_default() -> Self {
        let mut entries: Vec<((u8, u8), Vec<String>)> = Vec::with_capacity(200);

        // ── 声母 b (idx=0) ──
        entries.push(((0, 0), vec!["巴", "八", "把", "吧"])); // b-a
        entries.push(((0, 1), vec!["波", "博", "拨", "播"])); // b-o
        entries.push(((0, 3), vec!["比", "笔", "必", "壁"])); // b-i
        entries.push(((0, 4), vec!["不", "步", "布", "部"])); // b-u

        // ── 声母 p (idx=1) ──
        entries.push(((1, 0), vec!["怕", "爬", "帕"])); // p-a
        entries.push(((1, 1), vec!["坡", "破", "泼"])); // p-o
        entries.push(((1, 3), vec!["皮", "脾", "疲"])); // p-i
        entries.push(((1, 4), vec!["普", "铺", "浦"])); // p-u

        // ── 声母 m (idx=2) ──
        entries.push(((2, 0), vec!["马", "妈", "麻", "骂"])); // m-a
        entries.push(((2, 1), vec!["莫", "墨", "默", "摸"])); // m-o
        entries.push(((2, 2), vec!["么", "没", "美", "妹"])); // m-e
        entries.push(((2, 3), vec!["米", "密", "迷", "蜜"])); // m-i
        entries.push(((2, 4), vec!["母", "目", "木", "幕"])); // m-u

        // ── 声母 f (idx=3) ──
        entries.push(((3, 0), vec!["发", "法", "罚", "乏"])); // f-a
        entries.push(((3, 1), vec!["佛", "否"])); // f-o
        entries.push(((3, 2), vec!["飞", "非", "废", "费"])); // f-e (近似)
        entries.push(((3, 4), vec!["夫", "福", "府", "父"])); // f-u

        // ── 声母 d (idx=4) ──
        entries.push(((4, 0), vec!["大", "打", "达", "答"])); // d-a
        entries.push(((4, 2), vec!["的", "得", "德"]));       // d-e
        entries.push(((4, 3), vec!["地", "第", "低", "底"])); // d-i
        entries.push(((4, 4), vec!["度", "读", "独", "毒"])); // d-u

        // ── 声母 t (idx=5) ──
        entries.push(((5, 0), vec!["他", "她", "它", "塔"])); // t-a
        entries.push(((5, 2), vec!["特", "腾", "疼"]));       // t-e
        entries.push(((5, 3), vec!["提", "题", "体", "替"])); // t-i
        entries.push(((5, 4), vec!["图", "土", "突", "途"])); // t-u

        // ── 声母 n (idx=6) ──
        entries.push(((6, 0), vec!["那", "拿", "哪", "纳"])); // n-a
        entries.push(((6, 2), vec!["呢", "讷"]));             // n-e
        entries.push(((6, 3), vec!["你", "泥", "逆", "尼"])); // n-i
        entries.push(((6, 4), vec!["女", "努", "怒", "奴"])); // n-u

        // ── 声母 l (idx=7) ──
        entries.push(((7, 0), vec!["拉", "辣", "啦", "腊"])); // l-a
        entries.push(((7, 1), vec!["落", "罗", "洛", "络"])); // l-o
        entries.push(((7, 3), vec!["里", "力", "立", "理"])); // l-i
        entries.push(((7, 4), vec!["路", "陆", "录", "露"])); // l-u
        entries.push(((7, 5), vec!["律", "绿", "滤", "旅"])); // l-v (lü)

        // ── 声母 g (idx=8) ──
        entries.push(((8, 0), vec!["咖", "嘎", "噶"]));       // g-a
        entries.push(((8, 2), vec!["个", "各", "格", "阁"])); // g-e
        entries.push(((8, 4), vec!["古", "故", "顾", "固"])); // g-u

        // ── 声母 k (idx=9) ──
        entries.push(((9, 0), vec!["卡", "咖", "喀", "喀"])); // k-a
        entries.push(((9, 2), vec!["可", "克", "刻", "客"])); // k-e
        entries.push(((9, 4), vec!["苦", "库", "酷", "裤"])); // k-u

        // ── 声母 h (idx=10) ──
        entries.push(((10, 0), vec!["哈", "蛤", "铪"]));       // h-a
        entries.push(((10, 2), vec!["和", "合", "河", "何"])); // h-e
        entries.push(((10, 4), vec!["湖", "户", "护", "互"])); // h-u

        // ── 声母 j (idx=11) ──
        entries.push(((11, 3), vec!["机", "己", "几", "及", "即", "级", "极", "集", "急", "吉"])); // j-i
        entries.push(((11, 4), vec!["句", "巨", "具", "剧", "聚", "居", "举", "局"]));            // j-u (ju)

        // ── 声母 q (idx=12) ──
        entries.push(((12, 3), vec!["起", "气", "七", "期", "奇", "其", "器", "弃", "泣"])); // q-i
        entries.push(((12, 5), vec!["去", "取", "区", "曲", "趣", "屈", "驱"]));              // q-v (qü)

        // ── 声母 x (idx=13) ──
        entries.push(((13, 3), vec!["西", "系", "喜", "细", "习", "席", "袭"])); // x-i
        entries.push(((13, 5), vec!["许", "需", "须", "虚", "续", "序", "叙"])); // x-v (xü)

        // ── 声母 zh (idx=14, 占位) ──
        // z/c/s 占位 idx=18,19,20 此处跳过 zh/ch/sh (简化骨架)

        // ── 声母 z (idx=18) ──
        entries.push(((18, 0), vec!["杂", "扎", "砸", "咋"]));   // z-a
        entries.push(((18, 2), vec!["则", "责", "泽", "择"]));   // z-e
        entries.push(((18, 4), vec!["组", "族", "足", "祖", "阻"])); // z-u

        // ── 声母 c (idx=19) ──
        entries.push(((19, 0), vec!["擦", "嚓", "拆"]));         // c-a
        entries.push(((19, 2), vec!["策", "侧", "测", "册"]));   // c-e
        entries.push(((19, 4), vec!["粗", "促", "醋", "簇"]));   // c-u

        // ── 声母 s (idx=20) ──
        entries.push(((20, 0), vec!["撒", "洒", "萨", "卅"]));   // s-a
        entries.push(((20, 2), vec!["色", "塞", "涩", "瑟"]));   // s-e
        entries.push(((20, 4), vec!["素", "速", "塑", "苏", "俗"])); // s-u

        Self { entries }
    }
}
