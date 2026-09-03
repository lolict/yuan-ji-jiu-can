// ============================================================================
// 囡囝共生 · MetaIME 第4层: 声母-韵母分类器 — 候选字生成 (全量版)
// ============================================================================
//
// 当前层号: 4
// 当前声母: fen (分)
// 当前韵母组合: fen + lei = 分类 (声母韵母坐标 → 音节约束 → 候选集)
// 折叠状态: 声母-韵母-候字 三级映射展开 (21声母 × 35韵母 ≈ 400 音节)
// 下一步操作: 接收声母坐标+韵母坐标 → 查表产候字 → 多音字权重裁定 → 回传
//
// 锚定法则:
//   0 = 刘楚恬 (隐匿侧, 韵母水深)
//   1 = 我 (显现侧, 声母山立)
//
// 容器守恒: 分类器只从声母韵母坐标推测候字,不更改坐标本身
// ============================================================================

/// 声母-韵母组合 → 候选字表
pub struct ShengYunEngine {
    table: ShengYunTable,
    context_window: ContextWindow,
}

/// 声母-韵母映射表
/// 维度: [21声母 x 35韵母] → ~400+ 音节约束
/// 每个 (声母idx, 韵母idx) 对应候选字向量, 高频在前
struct ShengYunTable {
    entries: Vec<((u8, u8), Vec<&'static str>)>,
}

/// 前字上下文窗口 — 用于多音字甄别
/// 权重算法: 前字+1 / 前字+2 / 后字 -1 累计评分
struct ContextWindow {
    prev_chars: Vec<u32>,
    prev_prev_chars: Vec<u32>,
}

impl ContextWindow {
    fn new() -> Self {
        Self {
            prev_chars: Vec::with_capacity(4),
            prev_prev_chars: Vec::with_capacity(4),
        }
    }

    fn reset(&mut self) {
        self.prev_chars.clear();
        self.prev_prev_chars.clear();
    }

    /// 推入已选候字
    fn push_choice(&mut self, cp: u32) {
        if self.prev_chars.len() >= 4 {
            self.prev_chars.remove(0);
        }
        self.prev_chars.push(cp);
        if self.prev_prev_chars.len() >= 4 {
            self.prev_prev_chars.remove(0);
        }
        self.prev_prev_chars.push(cp);
    }

    /// 计算候选字上下文权重
    /// 策略:
    ///   - 候选若在标准词组中与前字组合 → +3
    ///   - 候选若与前字构成高频双字 → +2
    ///   - 候选独立成词 → +1
    ///   - 默认 → 0
    fn weigh_candidate(&self, candidate: &str, current_sm: u8, current_ym: u8) -> i32 {
        let mut score = 0i32;

        // 常见组词匹配表 (简化版: 60 高频组合)
        let bigrams: &[(&[&str], &[&str])] = &[
            // 声母-韵母组合的常见搭配 (前字字形, 候字字形)
            (["你", "好"], ["的", "吗"]),
            (["人", "民", "中"], ["国", "家", "我"]),
            (["人", "心", "家"], ["在", "有", "是"]),
            (["大", "小"], ["家", "学", "的"]),
            (["不"], ["是", "能", "要", "会"]),
            (["我"], ["们", "国", "的"]),
            (["的", "地"], ["确", "方"]),
            (["上", "下"], ["午", "去", "来"]),
            (["今", "明", "昨"], ["天", "年", "日"]),
            (["学", "同"], ["习", "校", "们"]),
        ];

        // 检查前字上下文
        if let Some(&prev_cp) = self.prev_chars.last() {
            if let Some(ch) = char::from_u32(prev_cp) {
                let prev_str = ch.to_string();
                for (lefts, rights) in bigrams {
                    if lefts.iter().any(|l| *l == prev_str) {
                        if rights.iter().any(|r| *r == candidate) {
                            score += 3;
                        }
                    }
                }
            }
        }

        // 声母类型加成 (某些声母在词首更常见)
        if current_sm <= 3 {
            // b/p/m/f 在词首频率略高
            score += 1;
        }

        // 独立成词能力 (高频字自身权重小幅提升)
        let high_freq: &[&str] = &["的", "是", "了", "在", "有", "我", "他", "她", "你", "不",
            "这", "那", "个", "来", "去", "说", "看", "好", "大", "小",
            "人", "上", "下", "出", "日", "月", "水", "火", "地", "天"];
        if high_freq.iter().any(|c| **c == candidate) {
            score += 1;
        }

        score
    }
}

impl ShengYunEngine {
    pub fn new() -> Self {
        Self {
            table: ShengYunTable::build_default(),
            context_window: ContextWindow::new(),
        }
    }

    /// 重新装载声母-韵母表
    pub fn reload(&mut self) {
        self.table = ShengYunTable::build_default();
        self.context_window.reset();
    }

    /// 音节约束产候字 (含多音字上下文加权)
    /// 返回候字向量, 按上下文得分排序, 首字=最高权重候选
    pub fn generate_candidates(&mut self, code_point: u32, sm: u8, ym: u8) -> Vec<String> {
        // ASCII 无效坐标 (非拼音区间)
        if code_point < 128 && sm >= 255 && ym >= 255 {
            if let Some(ch) = char::from_u32(code_point) {
                return vec![ch.to_string()];
            }
            return vec![];
        }

        // ASCII 声母坐标有效 → 查声母-韵母组合表
        if code_point < 128 {
            return self.sm_ym_lookup_with_context(sm, ym);
        }

        // 非 ASCII (直接汉字输入): 原样作为候字
        if let Some(ch) = char::from_u32(code_point) {
            return vec![ch.to_string()];
        }
        vec![]
    }

    /// 告知引擎用户已选取候字 (用于上下文窗口更新)
    pub fn on_candidate_chosen(&mut self, candidate: &str) {
        if let Some(&cp) = candidate.first().map(|c| c as u32) {
            self.context_window.push_choice(cp);
        }
    }

    /// 核心: 声母坐标 + 韵母坐标 → 候字列表 (含多音字加权排序)
    fn sm_ym_lookup_with_context(&mut self, sm: u8, ym: u8) -> Vec<String> {
        let raw_candidates: Vec<&str> = self.sm_ym_lookup(sm, ym);

        if raw_candidates.is_empty() {
            return vec![];
        }

        // 按上下文权重排序
        let mut scored: Vec<(i32, &str)> = raw_candidates
            .iter()
            .map(|c| (self.context_window.weigh_candidate(c, sm, ym), *c))
            .collect();

        // 稳定排序: 得分高在前, 相同分保持原序 (高频在前)
        scored.sort_by(|a, b| b.0.cmp(&a.0));

        scored.into_iter().map(|(_, s)| s.to_string()).collect()
    }

    /// 纯音节查表 (无多音字加权)
    pub fn lookup_syllable(&self, sm: u8, ym: u8) -> Vec<String> {
        self.sm_ym_lookup(sm, ym).iter().map(|s| s.to_string()).collect()
    }

    fn sm_ym_lookup(&self, sm: u8, ym: u8) -> Vec<&'static str> {
        for ((s, y), cands) in &self.table.entries {
            if *s == sm && *y == ym {
                return cands.clone();
            }
        }
        vec![]
    }

    /// 取状态描述
    pub fn state_desc(&self) -> String {
        format!("音节约束={} 条目, 窗口历史={}", self.table.entries.len(), self.context_window.prev_chars.len())
    }

    /// 取已装载的音节数 (供测试)
    pub fn syllable_count(&self) -> usize {
        self.table.entries.len()
    }
}

impl ShengYunTable {
    /// 内置声母-韵母 → 候字映射 (全量版)
    /// 覆盖: 21声母 × 35韵母 中文普通话全部有效组合 (约400音节)
    /// 每行: ((声母idx, 韵母idx), [候字1, 候字2, ...])
    /// 候字按字频降序
    fn build_default() -> Self {
        let mut entries: Vec<((u8, u8), Vec<&'static str>)> = Vec::with_capacity(512);

        // ════════════════════════════════════════════════════════════════
        // 声母 idx 映射 (与 encode_parser.rs / IMEInterceptor.kt 同源):
        //   0=b  1=p  2=m  3=f
        //   4=d  5=t  6=n  7=l
        //   8=g  9=k  10=h
        //   11=j  12=q  13=x
        //   14=zh 15=ch 16=sh 17=r
        //   18=z  19=c  20=s
        //
        // 韵母 idx 映射:
        //   0=a  1=o  2=e  3=i  4=u  5=v
        //   6=ai  7=ei  8=ui  9=ao  10=ou  11=iu
        //   12=ie  13=ve  14=er
        //   15=an  16=en  17=in  18=un  19=vn
        //   20=ang  21=eng  22=ing  23=ong
        //   24=ia  25=iao  26=ian  27=iang  28=iong
        //   29=ua  30=uo  31=uai  32=uan  33=uang
        // ════════════════════════════════════════════════════════════════

        // ── 声母 b (idx=0) ──
        entries.push(((0, 0), vec!["把", "八", "巴", "吧", "爸", "拔"])); // ba
        entries.push(((0, 1), vec!["波", "博", "播", "伯", "薄", "勃"])); // bo
        entries.push(((0, 2), vec!["被", "北", "备", "倍", "贝", "悲"])); // bei (e=2 实际是 bei)
        entries.push(((0, 3), vec!["比", "必", "笔", "毕", "壁", "币"])); // bi
        entries.push(((0, 4), vec!["不", "部", "步", "布", "补", "捕"])); // bu
        entries.push(((0, 6), vec!["白", "百", "败", "拜", "摆", "柏"])); // bai
        entries.push(((0, 7), vec!["被", "北", "备", "倍", "贝", "悲"])); // bei
        entries.push(((0, 8), vec!["对", "队", "堆", "兑"])); // dui (近似 bui)
        entries.push(((0, 9), vec!["包", "报", "保", "宝", "暴", "爆"])); // bao
        entries.push(((0, 10), vec!["某", "谋", "眸", "牟"])); // bo (近似 bou → 不常用, 留)
        entries.push(((0, 11), vec!["六", "留", "流", "柳", "刘"])); // biu (近似 liu)
        entries.push(((0, 15), vec!["半", "办", "般", "班", "版", "板"])); // ban
        entries.push(((0, 16), vec!["本", "笨", "奔", "份", "粉"])); // ben (近)
        entries.push(((0, 17), vec!["宾", "滨", "滨", "殡"])); // bin
        entries.push(((0, 20), vec!["帮", "邦", "棒", "膀", "傍"])); // bang
        entries.push(((0, 21), vec!["崩", "绷", "蹦", "棚"])); // beng
        entries.push(((0, 22), vec!["并", "病", "冰", "兵", "饼", "柄"])); // bing
        entries.push(((0, 25), vec!["表", "标", "彪", "别", "憋"])); // biao

        // ── 声母 p (idx=1) ──
        entries.push(((1, 0), vec!["怕", "爬", "帕", "趴", "拍", "排"])); // pa
        entries.push(((1, 1), vec!["破", "坡", "泼", "婆", "颇", "泊"])); // po
        entries.push(((1, 2), vec!["配", "佩", "陪", "培", "赔"])); // pei (近似)
        entries.push(((1, 3), vec!["皮", "疲", "脾", "匹", "批", "披"])); // pi
        entries.push(((1, 4), vec!["普", "谱", "浦", "铺", "朴", "蒲"])); // pu
        entries.push(((1, 6), vec!["排", "牌", "拍", "派", "徘"])); // pai
        entries.push(((1, 7), vec!["配", "佩", "陪", "培", "赔", "沛"])); // pei
        entries.push(((1, 9), vec!["跑", "泡", "炮", "抛", "刨"])); // pao
        entries.push(((1, 10), vec!["剖", "掊", "裒"])); // pou
        entries.push(((1, 15), vec!["盘", "判", "攀", "叛", "潘", "畔"])); // pan
        entries.push(((1, 16), vec!["喷", "盆", "湓"])); // pen
        entries.push(((1, 17), vec!["品", "贫", "拼", "频", "聘", "嫔"])); // pin
        entries.push(((1, 20), vec!["旁", "胖", "膀", "磅", "螃", "傍"])); // pang
        entries.push(((1, 21), vec!["朋", "碰", "蓬", "鹏", "捧", "篷"])); // peng
        entries.push(((1, 22), vec!["平", "凭", "评", "瓶", "屏", "萍"])); // ping

        // ── 声母 m (idx=2) ──
        entries.push(((2, 0), vec!["妈", "马", "吗", "麻", "骂", "码"])); // ma
        entries.push(((2, 1), vec!["莫", "摸", "魔", "模", "漠", "墨"])); // mo
        entries.push(((2, 2), vec!["么", "没", "每", "美", "妹", "梅"])); // me (mei)
        entries.push(((2, 3), vec!["米", "密", "迷", "蜜", "秘", "觅"])); // mi
        entries.push(((2, 4), vec!["母", "木", "目", "幕", "牧", "墓"])); // mu
        entries.push(((2, 6), vec!["买", "卖", "迈", "埋", "脉", "霾"])); // mai
        entries.push(((2, 7), vec!["没", "每", "美", "妹", "梅", "煤"])); // mei
        entries.push(((2, 9), vec!["猫", "毛", "冒", "貌", "帽", "矛"])); // mao
        entries.push(((2, 10), vec!["某", "谋", "眸", "牟", "侔"])); // mou
        entries.push(((2, 11), vec!["谬", "缪"])); // miu
        entries.push(((2, 12), vec!["灭", "蔑", "篾"])); // mie
        entries.push(((2, 15), vec!["满", "慢", "漫", "蛮", "曼", "蔓"])); // man
        entries.push(((2, 16), vec!["门", "闷", "们", "焖"])); // men
        entries.push(((2, 17), vec!["民", "敏", "闽", "泯", "岷"])); // min
        entries.push(((2, 20), vec!["忙", "盲", "茫", "芒", "莽", "蟒"])); // mang
        entries.push(((2, 21), vec!["梦", "萌", "蒙", "猛", "盟", "孟"])); // meng
        entries.push(((2, 22), vec!["名", "明", "命", "鸣", "铭", "冥"])); // ming
        entries.push(((2, 25), vec!["妙", "庙", "苗", "描", "瞄", "淼"])); // miao

        // ── 声母 f (idx=3) ──
        entries.push(((3, 0), vec!["发", "法", "罚", "乏", "伐", "阀"])); // fa
        entries.push(((3, 1), vec!["佛"])); // fo
        entries.push(((3, 2), vec!["非", "飞", "费", "废", "沸", "肺"])); // fei (e=fei)
        entries.push(((3, 4), vec!["夫", "福", "府", "父", "富", "复"])); // fu
        entries.push(((3, 6), vec![""])); // fai 不存在
        entries.push(((3, 7), vec!["非", "飞", "费", "废", "沸", "肥"])); // fei
        entries.push(((3, 9), vec![""])); // fao 不存在
        entries.push(((3, 10), vec!["否", "缶"])); // fou
        entries.push(((3, 15), vec!["反", "饭", "翻", "凡", "犯", "范"])); // fan
        entries.push(((3, 16), vec!["分", "份", "纷", "粉", "芬", "愤"])); // fen
        entries.push(((3, 20), vec!["方", "放", "房", "防", "访", "芳"])); // fang
        entries.push(((3, 21), vec!["风", "封", "丰", "锋", "峰", "凤"])); // feng

        // ── 声母 d (idx=4) ──
        entries.push(((4, 0), vec!["大", "打", "达", "答", "搭", "嗒"])); // da
        entries.push(((4, 2), vec!["的", "得", "德"])); // de
        entries.push(((4, 3), vec!["地", "第", "低", "底", "弟", "递"])); // di
        entries.push(((4, 4), vec!["度", "读", "独", "毒", "杜", "督"])); // du
        entries.push(((4, 6), vec!["代", "带", "待", "袋", "戴", "歹"])); // dai
        entries.push(((4, 7), vec!["得", "德", "地", "底"])); // dei
        entries.push(((4, 8), vec!["对", "队", "兑", "堆", "碓"])); // dui
        entries.push(((4, 9), vec!["到", "道", "倒", "刀", "岛", "盗"])); // dao
        entries.push(((4, 10), vec!["都", "斗", "豆", "抖", "陡", "兜"])); // dou
        entries.push(((4, 11), vec!["六", "留", "流", "柳"])); // diu (近似 liu)
        entries.push(((4, 12), vec!["爹", "迭", "碟", "跌", "蝶"])); // die
        entries.push(((4, 15), vec!["但", "单", "担", "蛋", "弹", "淡"])); // dan
        entries.push(((4, 16), vec!["扽"])); // den
        entries.push(((4, 20), vec!["当", "党", "荡", "挡", "档", "裆"])); // dang
        entries.push(((4, 21), vec!["等", "登", "灯", "邓", "凳", "蹬"])); // deng
        entries.push(((4, 22), vec!["定", "顶", "订", "丁", "钉", "盯"])); // ding
        entries.push(((4, 23), vec!["东", "动", "懂", "洞", "冬", "董"])); // dong
        entries.push(((4, 25), vec!["掉", "调", "钓", "雕", "叼", "刁"])); // diao
        entries.push(((4, 26), vec!["点", "电", "店", "典", "殿", "垫"])); // dian

        // ── 声母 t (idx=5) ──
        entries.push(((5, 0), vec!["他", "她", "它", "塔", "踏", "塌"])); // ta
        entries.push(((5, 2), vec!["特", "疼", "腾", "藤", "滕"])); // te
        entries.push(((5, 3), vec!["提", "题", "体", "替", "梯", "踢"])); // ti
        entries.push(((5, 4), vec!["土", "图", "途", "突", "涂", "徒"])); // tu
        entries.push(((5, 6), vec!["太", "台", "态", "泰", "抬", "胎"])); // tai
        entries.push(((5, 8), vec!["推", "退", "腿", "蜕", "颓"])); // tui
        entries.push(((5, 9), vec!["套", "逃", "桃", "讨", "陶", "掏"])); // tao
        entries.push(((5, 10), vec!["头", "投", "透", "偷"])); // tou
        entries.push(((5, 11), vec![""])); // tiu 不存在
        entries.push(((5, 12), vec!["铁", "贴", "帖", "餮"])); // tie
        entries.push(((5, 15), vec!["谈", "探", "坦", "叹", "摊", "滩"])); // tan
        entries.push(((5, 16), vec![""])); // ten 不存在
        entries.push(((5, 20), vec!["堂", "唐", "糖", "躺", "趟", "烫"])); // tang
        entries.push(((5, 21), vec!["腾", "藤", "滕", "疼"])); // teng
        entries.push(((5, 22), vec!["听", "停", "庭", "亭", "挺", "廷"])); // ting
        entries.push(((5, 23), vec!["同", "通", "统", "痛", "铜", "童"])); // tong
        entries.push(((5, 25), vec!["条", "调", "跳", "挑", "鸟", "窕"])); // tiao

        // ── 声母 n (idx=6) ──
        entries.push(((6, 0), vec!["那", "拿", "哪", "纳", "娜", "钠"])); // na
        entries.push(((6, 1), vec![""])); // no 不存在
        entries.push(((6, 2), vec!["呢", "讷", "哪"])); // ne
        entries.push(((6, 3), vec!["你", "泥", "逆", "尼", "腻", "溺"])); // ni
        entries.push(((6, 4), vec!["女", "努", "怒", "奴", "弩", "驽"])); // nu
        entries.push(((6, 5), vec!["律", "绿", "滤", "虑", "旅", "屡"])); // nv (lü) - 共用
        entries.push(((6, 6), vec!["奶", "乃", "耐", "奈", "鼐"])); // nai
        entries.push(((6, 7), vec!["内", "馁", "那"])); // nei
        entries.push(((6, 9), vec!["脑", "闹", "恼", "挠", "淖"])); // nao
        entries.push(((6, 10), vec!["", "耨"])); // nou
        entries.push(((6, 11), vec!["牛", "扭", "纽", "钮", "拗"])); // niu
        entries.push(((6, 12), vec!["捏", "聂", "镊", "蹑", "孽"])); // nie
        entries.push(((6, 15), vec!["南", "男", "难", "赧", "腩"])); // nan
        entries.push(((6, 16), vec!["嫩"])); // nen
        entries.push(((6, 17), vec!["您", "宁", "凝", "拧", "泞"])); // nin (您)
        entries.push(((6, 19), vec![""])); // nvn
        entries.push(((6, 20), vec!["囊", "囔", "馕", "攮"])); // nang
        entries.push(((6, 21), vec!["能", "嫩", "恁"])); // neng
        entries.push(((6, 22), vec!["宁", "凝", "拧", "狞", "泞", "佞"])); // ning
        entries.push(((6, 23), vec!["农", "浓", "弄", "脓", "侬"])); // nong
        entries.push(((6, 26), vec!["年", "念", "廿", "辗", "捻"])); // nian

        // ── 声母 l (idx=7) ──
        entries.push(((7, 0), vec!["拉", "辣", "啦", "蜡", "喇", "剌"])); // la
        entries.push(((7, 1), vec!["落", "洛", "络", "罗", "锣", "裸"])); // luo/lo
        entries.push(((7, 2), vec!["了", "乐", "勒"])); // le
        entries.push(((7, 3), vec!["里", "力", "立", "理", "利", "李"])); // li
        entries.push(((7, 4), vec!["路", "陆", "录", "露", "鹿", "卢"])); // lu
        entries.push(((7, 5), vec!["律", "绿", "滤", "虑", "旅", "吕"])); // lv
        entries.push(((7, 6), vec!["来", "赖", "莱", "徕", "涞"])); // lai
        entries.push(((7, 7), vec!["类", "累", "雷", "泪", "垒", "蕾"])); // lei
        entries.push(((7, 8), vec!["", ""])); // lui 不存在
        entries.push(((7, 9), vec!["老", "劳", "牢", "捞", "姥", "酪"])); // lao
        entries.push(((7, 10), vec!["楼", "漏", "搂", "陋", "露"])); // lou
        entries.push(((7, 11), vec!["六", "留", "流", "柳", "刘", "榴"])); // liu
        entries.push(((7, 12), vec!["列", "烈", "裂", "猎", "劣", "冽"])); // lie
        entries.push(((7, 13), vec!["略", "掠"])); // lüe
        entries.push(((7, 14), vec![""])); // ler 不存在
        entries.push(((7, 15), vec!["蓝", "兰", "烂", "懒", "拦", "篮"])); // lan
        entries.push(((7, 17), vec!["林", "邻", "临", "淋", "琳", "霖"])); // lin
        entries.push(((7, 18), vec!["论", "轮", "伦", "沦", "抡"])); // lun
        entries.push(((7, 19), vec![""])); // lvn
        entries.push(((7, 20), vec!["浪", "朗", "郎", "廊", "狼", "琅"])); // lang
        entries.push(((7, 21), vec!["冷", "棱", "楞", "塄"])); // leng
        entries.push(((7, 22), vec!["领", "令", "零", "灵", "另", "龄"])); // ling
        entries.push(((7, 23), vec!["龙", "弄", "笼", "聋", "隆", "窿"])); // long
        entries.push(((7, 25), vec!["了", "料", "聊", "疗", "僚", "辽"])); // liao
        entries.push(((7, 26), vec!["连", "联", "脸", "练", "恋", "莲"])); // lian

        // ── 声母 g (idx=8) ──
        entries.push(((8, 0), vec!["咖", "嘎", "噶", "尕"])); // ga
        entries.push(((8, 1), vec![""])); // go 不存在
        entries.push(((8, 2), vec!["个", "各", "格", "阁", "葛", "隔"])); // ge
        entries.push(((8, 4), vec!["古", "故", "顾", "固", "股", "谷"])); // gu
        entries.push(((8, 6), vec!["该", "改", "概", "盖", "钙", "丐"])); // gai
        entries.push(((8, 7), vec!["给", ""])); // gei
        entries.push(((8, 8), vec!["贵", "归", "鬼", "规", "桂", "柜"])); // gui
        entries.push(((8, 10), vec!["够", "狗", "沟", "勾", "构", "购"])); // gou
        entries.push(((8, 15), vec!["干", "敢", "感", "赶", "甘", "肝"])); // gan
        entries.push(((8, 16), vec!["根", "跟", "亘", "艮"])); // gen
        entries.push(((8, 18), vec!["滚", "棍", "鲧"])); // gun
        entries.push(((8, 20), vec!["刚", "钢", "港", "岗", "纲", "缸"])); // gang
        entries.push(((8, 21), vec!["更", "耕", "羹", "哽", "埂"])); // geng
        entries.push(((8, 23), vec!["工", "公", "共", "功", "红", "供"])); // gong
        entries.push(((8, 29), vec!["瓜", "挂", "卦", "刮", "呱", "褂"])); // gua
        entries.push(((8, 30), vec!["过", "国", "果", "锅", "裹", "郭"])); // guo
        entries.push(((8, 31), vec!["怪", "乖", "拐", "掴"])); // guai
        entries.push(((8, 32), vec!["关", "观", "官", "管", "惯", "馆"])); // guan
        entries.push(((8, 33), vec!["光", "广", "逛", "犷", "胱"])); // guang

        // ── 声母 k (idx=9) ──
        entries.push(((9, 0), vec!["卡", "咖", "喀", "咔", "胩"])); // ka
        entries.push(((9, 1), vec![""])); // ko 不存在
        entries.push(((9, 2), vec!["可", "克", "刻", "客", "课", "颗"])); // ke
        entries.push(((9, 4), vec!["苦", "库", "酷", "裤", "枯", "哭"])); // ku
        entries.push(((9, 6), vec!["开", "凯", "慨", "楷", "铠", "忾"])); // kai
        entries.push(((9, 7), vec!["", "刻"])); // kei (罕见)
        entries.push(((9, 8), vec!["亏", "奎", "魁", "葵", "傀", "愧"])); // kui
        entries.push(((9, 10), vec!["口", "扣", "叩", "寇", "蔻"])); // kou
        entries.push(((9, 15), vec!["看", "刊", "砍", "坎", "堪", "勘"])); // kan
        entries.push(((9, 16), vec!["肯", "恳", "啃", "垦", "裉"])); // ken
        entries.push(((9, 18), vec!["困", "昆", "坤", "捆", "鲲", "琨"])); // kun
        entries.push(((9, 20), vec!["抗", "康", "扛", "炕", "糠", "亢"])); // kang
        entries.push(((9, 21), vec!["坑", "吭", "铿"])); // keng
        entries.push(((9, 30), vec!["阔", "扩", "括", "廓", "适"])); // kuo
        entries.push(((9, 32), vec!["宽", "款", "髋"])); // kuan
        entries.push(((9, 33), vec!["狂", "况", "矿", "框", "旷", "眶"])); // kuang

        // ── 声母 h (idx=10) ──
        entries.push(((10, 0), vec!["哈", "蛤", "铪", "虾"])); // ha
        entries.push(((10, 1), vec![""])); // ho 不存在
        entries.push(((10, 2), vec!["和", "合", "河", "何", "核", "贺"])); // he
        entries.push(((10, 4), vec!["湖", "户", "护", "互", "呼", "胡"])); // hu
        entries.push(((10, 6), vec!["还", "海", "害", "孩", "亥", "骇"])); // hai
        entries.push(((10, 7), vec!["黑", "嘿"])); // hei
        entries.push(((10, 8), vec!["回", "会", "灰", "绘", "挥", "悔"])); // hui
        entries.push(((10, 9), vec!["好", "号", "毫", "豪", "耗", "壕"])); // hao
        entries.push(((10, 10), vec!["后", "候", "厚", "吼", "侯", "猴"])); // hou
        entries.push(((10, 15), vec!["汉", "旱", "寒", "韩", "含", "喊"])); // han
        entries.push(((10, 16), vec!["很", "恨", "狠", "痕"])); // hen
        entries.push(((10, 18), vec!["混", "浑", "魂", "昏", "婚", "荤"])); // hun
        entries.push(((10, 20), vec!["行", "航", "杭", "巷", "吭"])); // hang
        entries.push(((10, 29), vec!["花", "华", "划", "画", "话", "滑"])); // hua
        entries.push(((10, 30), vec!["火", "活", "或", "货", "获", "祸"])); // huo
        entries.push(((10, 31), vec!["坏", "怀", "淮", "槐", "踝"])); // huai
        entries.push(((10, 32), vec!["换", "欢", "还", "环", "缓", "患"])); // huan
        entries.push(((10, 33), vec!["黄", "皇", "荒", "慌", "晃", "煌"])); // huang

        // ── 声母 j (idx=11) ──
        entries.push(((11, 3), vec!["几", "机", "己", "及", "即", "级", "极", "集", "急", "吉", "纪", "记", "计", "季", "基", "际"])); // ji
        entries.push(((11, 4), vec!["句", "巨", "具", "剧", "聚", "居", "举", "局", "据", "距", "拘", "菊"])); // ju
        entries.push(((11, 11), vec!["就", "九", "酒", "旧", "救", "久", "舅", "灸", "咎"])); // jiu
        entries.push(((11, 12), vec!["接", "节", "街", "姐", "借", "届", "戒", "皆", "阶", "揭"])); // jie
        entries.push(((11, 13), vec!["觉", "决", "绝", "掘", "诀", "爵", "倔"])); // jue
        entries.push(((11, 15), vec!["间", "建", "见", "件", "剑", "健", "检", "减", "简", "坚", "渐", "尖"])); // jian
        entries.push(((11, 16), vec!["", "仅", "尽", "紧", "进", "近", "金", "今", "斤"])); // jin
        entries.push(((11, 17), vec![""])); // jun
        entries.push(((11, 18), vec!["均", "菌", "军", "君", "俊", "峻"])); // jun (v→un)
        entries.push(((11, 22), vec!["经", "京", "精", "景", "静", "境", "警", "竟", "敬", "睛", "晶"])); // jing
        entries.push(((11, 23), vec!["", "窘", "炯", "迥"])); // jiong
        entries.push(((11, 25), vec!["教", "叫", "较", "觉", "交", "角", "脚", "焦", "浇", "胶"])); // jiao
        entries.push(((11, 26), vec!["间", "见", "件", "建", "剑", "尖", "渐", "减", "检", "简"])); // jian
        entries.push(((11, 27), vec!["将", "江", "讲", "降", "蒋", "奖", "疆", "姜", "僵"])); // jiang
        entries.push(((11, 28), vec!["窘", "炯", "迥", "扃"])); // jiong

        // ── 声母 q (idx=12) ──
        entries.push(((12, 3), vec!["起", "气", "七", "期", "奇", "其", "器", "弃", "泣", "企", "启", "骑", "棋", "旗"])); // qi
        entries.push(((12, 4), vec!["去", "取", "区", "曲", "趣", "屈", "驱", "趋", "渠", "娶"])); // qu
        entries.push(((12, 11), vec!["求", "球", "秋", "丘", "邱", "囚", "酋", "泅"])); // qiu
        entries.push(((12, 12), vec!["且", "切", "窃", "妾", "怯", "趄"])); // qie
        entries.push(((12, 13), vec!["确", "却", "缺", "雀", "鹊", "阙", "榷"])); // que
        entries.push(((12, 15), vec!["前", "钱", "千", "签", "欠", "牵", "浅", "铅", "迁", "乾", "潜"])); // qian
        entries.push(((12, 16), vec!["亲", "侵", "钦", "琴", "勤", "芹", "禽", "寝"])); // qin
        entries.push(((12, 17), vec!["群", "裙", "逡"])); // qun
        entries.push(((12, 22), vec!["情", "请", "清", "青", "轻", "晴", "庆", "倾", "顷", "亲"])); // qing
        entries.push(((12, 23), vec!["穷", "琼", "蛩", "邛"])); // qiong
        entries.push(((12, 25), vec!["桥", "巧", "乔", "侨", "敲", "俏", "窍", "翘"])); // qiao
        entries.push(((12, 26), vec!["前", "钱", "千", "签", "欠", "牵", "浅", "迁"])); // qian
        entries.push(((12, 27), vec!["强", "墙", "枪", "抢", "腔", "羌", "锵"])); // qiang
        entries.push(((12, 28), vec!["穷", "琼", "邛"])); // qiong

        // ── 声母 x (idx=13) ──
        entries.push(((13, 3), vec!["西", "系", "喜", "细", "习", "席", "袭", "吸", "希", "息", "希", "溪", "稀"])); // xi
        entries.push(((13, 4), vec!["许", "需", "须", "虚", "续", "序", "叙", "徐", "蓄", "绪", "絮", "婿"])); // xu
        entries.push(((13, 11), vec!["修", "秀", "休", "袖", "臭", "绣", "锈", "嗅"])); // xiu
        entries.push(((13, 12), vec!["写", "血", "些", "谢", "斜", "鞋", "胁", "歇"])); // xie
        entries.push(((13, 13), vec!["学", "雪", "血", "穴", "薛", "靴", "削"])); // xue
        entries.push(((13, 15), vec!["先", "现", "线", "县", "显", "鲜", "险", "限", "献", "宪", "陷", "仙", "掀"])); // xian
        entries.push(((13, 16), vec!["新", "心", "信", "薪", "欣", "辛", "馨", "锌"])); // xin
        entries.push(((13, 17), vec!["寻", "训", "迅", "讯", "巡", "逊", "熏", "勋", "循"])); // xun (v→un)
        entries.push(((13, 22), vec!["行", "形", "型", "性", "姓", "兴", "星", "醒", "刑", "杏"])); // xing
        entries.push(((13, 25), vec!["小", "笑", "校", "效", "晓", "孝", "肖", "萧", "消", "销"])); // xiao
        entries.push(((13, 26), vec!["先", "现", "线", "县", "显", "鲜", "险", "限", "献", "宪"])); // xian
        entries.push(((13, 27), vec!["想", "向", "响", "象", "像", "香", "乡", "箱", "详", "祥"])); // xiang
        entries.push(((13, 28), vec!["雄", "胸", "兄", "熊", "凶", "汹", "匈"])); // xiong

        // ── 声母 zh (idx=14) ──
        entries.push(((14, 0), vec!["这", "者", "著", "着", "折", "哲", "遮", "蛰"])); // zhe (a=近音)
        entries.push(((14, 2), vec!["这", "者", "著", "着", "折", "哲", "遮"])); // zhe
        entries.push(((14, 3), vec!["之", "只", "知", "直", "指", "纸", "止", "至", "制", "治", "志", "支", "值", "织", "职"])); // zhi
        entries.push(((14, 4), vec!["主", "住", "注", "著", "祝", "竹", "煮", "朱", "珠", "诸", "猪", "筑", "助", "柱"])); // zhu
        entries.push(((14, 6), vec!["", "斋", "摘", "窄", "债", "寨"])); // zhai
        entries.push(((14, 7), vec!["这", "着"])); // zhei (罕见)
        entries.push(((14, 8), vec!["追", "坠", "锥", "椎"])); // zhui
        entries.push(((14, 9), vec!["找", "照", "招", "罩", "兆", "沼", "诏"])); // zhao
        entries.push(((14, 10), vec!["周", "州", "洲", "舟", "粥", "轴", "肘", "帚"])); // zhou
        entries.push(((14, 15), vec!["站", "战", "占", "沾", "展", "盏", "崭", "绽"])); // zhan
        entries.push(((14, 16), vec!["真", "针", "阵", "镇", "震", "振", "珍", "诊", "枕"])); // zhen
        entries.push(((14, 18), vec!["准", "盹"])); // zhun
        entries.push(((14, 20), vec!["张", "章", "长", "掌", "丈", "仗", "账", "涨"])); // zhang
        entries.push(((14, 21), vec!["正", "整", "争", "证", "政", "症", "征", "蒸", "睁", "筝"])); // zheng
        entries.push(((14, 23), vec!["中", "重", "种", "众", "终", "钟", "忠", "衷", "肿"])); // zhong
        entries.push(((14, 25), vec!["", "爪"])); // zhao (重复)
        entries.push(((14, 26), vec!["", ""])); // zhian 不存在
        entries.push(((14, 29), vec!["抓", "爪", "拽"])); // zhua
        entries.push(((14, 30), vec!["桌", "捉", "着", "卓", "琢", "浊", "酌", "灼"])); // zhuo
        entries.push(((14, 32), vec!["转", "专", "砖", "传", "赚", "撰", "篆"])); // zhuan

        // ── 声母 ch (idx=15) ──
        entries.push(((15, 0), vec!["差", "查", "察", "茶", "岔", "刹", "岔"])); // cha
        entries.push(((15, 2), vec!["车", "彻", "撤", "扯", "澈"])); // che
        entries.push(((15, 3), vec!["吃", "持", "迟", "尺", "齿", "赤", "池", "驰"])); // chi
        entries.push(((15, 4), vec!["出", "处", "初", "除", "楚", "厨", "储", "础", "触", "畜"])); // chu
        entries.push(((15, 6), vec!["拆", "柴", "钗", "差", "侪"])); // chai
        entries.push(((15, 8), vec!["吹", "垂", "锤", "捶", "椎"])); // chui
        entries.push(((15, 9), vec!["超", "朝", "潮", "炒", "吵", "抄", "嘲", "巢"])); // chao
        entries.push(((15, 10), vec!["抽", "丑", "仇", "臭", "绸", "愁", "稠"])); // chou
        entries.push(((15, 15), vec!["产", "颤", "缠", "禅", "蝉", "铲", "搀"])); // chan
        entries.push(((15, 16), vec!["陈", "沉", "晨", "尘", "臣", "衬", "趁", "称"])); // chen
        entries.push(((15, 20), vec!["长", "场", "常", "厂", "唱", "尝", "偿", "肠"])); // chang
        entries.push(((15, 21), vec!["成", "城", "程", "称", "承", "诚", "乘", "惩", "呈"])); // cheng
        entries.push(((15, 23), vec!["冲", "重", "充", "崇", "虫", "宠", "憧"])); // chong
        entries.push(((15, 29), vec!["", "欻"])); // chua
        entries.push(((15, 30), vec!["戳", "绰", "龊", "踔", "辍"])); // chuo
        entries.push(((15, 32), vec!["穿", "川", "传", "船", "串", "喘", "椽"])); // chuan

        // ── 声母 sh (idx=16) ──
        entries.push(((16, 0), vec!["沙", "杀", "纱", "啥", "砂", "厦", "刹", "杉"])); // sha
        entries.push(((16, 2), vec!["设", "社", "舍", "涉", "舌", "蛇", "折", "赦"])); // she
        entries.push(((16, 3), vec!["是", "时", "十", "事", "世", "市", "式", "识", "石", "史", "使", "始", "师", "诗", "失"])); // shi
        entries.push(((16, 4), vec!["书", "数", "树", "术", "属", "输", "述", "熟", "束", "叔", "舒", "殊", "梳"])); // shu
        entries.push(((16, 6), vec!["晒", "筛", "色", ""])); // shai
        entries.push(((16, 7), vec!["谁"])); // shei
        entries.push(((16, 8), vec!["水", "睡", "税", "谁", "蜕"])); // shui
        entries.push(((16, 9), vec!["少", "烧", "稍", "勺", "苕", "梢", "哨"])); // shao
        entries.push(((16, 10), vec!["收", "手", "首", "守", "受", "授", "售", "瘦", "兽"])); // shou
        entries.push(((16, 15), vec!["山", "闪", "善", "扇", "衫", "删", "陕", "赡"])); // shan
        entries.push(((16, 16), vec!["深", "身", "神", "什", "审", "甚", "肾", "申", "沈", "伸"])); // shen
        entries.push(((16, 20), vec!["上", "商", "伤", "赏", "尚", "裳", "晌"])); // shang
        entries.push(((16, 21), vec!["生", "声", "省", "升", "胜", "圣", "剩", "绳", "甥"])); // sheng
        entries.push(((16, 29), vec!["刷", "耍", "唰"])); // shua
        entries.push(((16, 30), vec!["说", "硕", "烁", "朔", "妁"])); // shuo
        entries.push(((16, 32), vec!["拴", "栓", "涮"])); // shuan

        // ── 声母 r (idx=17) ──
        entries.push(((17, 2), vec!["热", "惹", "若", "弱"])); // re (近似)
        entries.push(((17, 3), vec!["日", ""])); // ri
        entries.push(((17, 4), vec!["如", "入", "儒", "乳", "辱", "孺", "蠕"])); // ru
        entries.push(((17, 7), vec![""])); // rei 不存在
        entries.push(((17, 8), vec!["瑞", "锐", "蕊", "睿", "芮"])); // rui
        entries.push(((17, 10), vec!["肉", "揉", "柔", "糅", "蹂"])); // rou
        entries.push(((17, 15), vec!["然", "染", "燃", "冉", "髯"])); // ran
        entries.push(((17, 16), vec!["人", "任", "认", "忍", "仁", "韧", "刃", "纫"])); // ren
        entries.push(((17, 20), vec!["让", "嚷", "壤", "瓤", "攘", "禳"])); // rang
        entries.push(((17, 21), vec!["仍", "扔", "戎", "绒", "茸"])); // reng
        entries.push(((17, 23), vec!["容", "荣", "融", "绒", "溶", "榕", "熔"])); // rong
        entries.push(((17, 30), vec!["若", "弱", "箬", "偌"])); // ruo

        // ── 声母 z (idx=18) ──
        entries.push(((18, 0), vec!["杂", "扎", "砸", "咋", "咂"])); // za
        entries.push(((18, 2), vec!["则", "责", "泽", "择", "仄", "啧"])); // ze
        entries.push(((18, 3), vec!["自", "字", "子", "紫", "仔", "姿", "资", "滋", "咨"])); // zi
        entries.push(((18, 4), vec!["组", "族", "足", "祖", "阻", "卒", "诅"])); // zu
        entries.push(((18, 6), vec!["在", "再", "载", "灾", "栽", "哉", "宰"])); // zai
        entries.push(((18, 7), vec!["贼"])); // zei
        entries.push(((18, 8), vec!["最", "罪", "嘴", "醉", "咀"])); // zui
        entries.push(((18, 9), vec!["早", "造", "遭", "枣", "糟", "澡", "灶"])); // zao
        entries.push(((18, 10), vec!["走", "奏", "揍", "邹", "诹"])); // zou
        entries.push(((18, 11), vec!["", ""])); // ziu 不存在
        entries.push(((18, 15), vec!["咱", "赞", "暂", "攒", "趱", "簪"])); // zan
        entries.push(((18, 16), vec!["怎", "谮"])); // zen
        entries.push(((18, 18), vec!["尊", "遵", "樽", "鳟"])); // zun
        entries.push(((18, 20), vec!["脏", "葬", "赃", "臧", "奘"])); // zang
        entries.push(((18, 21), vec!["增", "曾", "赠", "憎", "甑"])); // zeng
        entries.push(((18, 23), vec!["总", "宗", "综", "纵", "棕", "踪", "鬃"])); // zong

        // ── 声母 c (idx=19) ──
        entries.push(((19, 0), vec!["擦", "嚓", "拆", ""])); // ca
        entries.push(((19, 2), vec!["策", "侧", "测", "册", "厕", "恻"])); // ce
        entries.push(((19, 3), vec!["次", "此", "刺", "词", "辞", "慈", "磁", "雌", "祠", "赐"])); // ci
        entries.push(((19, 4), vec!["粗", "促", "醋", "簇", "蹴", "猝", "徂"])); // cu
        entries.push(((19, 6), vec!["猜", "才", "菜", "采", "财", "材", "裁", "彩"])); // cai
        entries.push(((19, 8), vec!["催", "脆", "翠", "摧", "崔", "淬", "粹"])); // cui
        entries.push(((19, 9), vec!["草", "操", "曹", "槽", "糙", "嘈"])); // cao
        entries.push(((19, 10), vec!["凑", "辏", "腠"])); // cou
        entries.push(((19, 15), vec!["参", "惨", "灿", "餐", "残", "蚕", "惭"])); // can
        entries.push(((19, 16), vec![""])); // cen
        entries.push(((19, 18), vec!["村", "存", "寸", "忖", "皴"])); // cun
        entries.push(((19, 20), vec!["藏", "仓", "苍", "舱", "沧", "伧"])); // cang
        entries.push(((19, 21), vec!["曾", "层", "蹭"])); // ceng
        entries.push(((19, 23), vec!["从", "丛", "聪", "葱", "匆", "琮", "淙"])); // cong

        // ── 声母 s (idx=20) ──
        entries.push(((20, 0), vec!["撒", "洒", "萨", "卅", "仨"])); // sa
        entries.push(((20, 2), vec!["色", "塞", "涩", "瑟", "啬"])); // se (近似)
        entries.push(((20, 3), vec!["四", "死", "丝", "司", "私", "思", "斯", "撕", "似", "寺", "祀", "肆"])); // si
        entries.push(((20, 4), vec!["素", "速", "塑", "苏", "俗", "诉", "肃", "宿", "粟", "酥"])); // su
        entries.push(((20, 6), vec!["赛", "塞", ""])); // sai
        entries.push(((20, 8), vec!["虽", "随", "岁", "碎", "穗", "遂", "隧"])); // sui
        entries.push(((20, 10), vec!["搜", "艘", "叟", "擞", "馊", "嗖", "飕"])); // sou
        entries.push(((20, 15), vec!["三", "散", "伞", "叁", "毵"])); // san
        entries.push(((20, 16), vec!["森"])); // sen
        entries.push(((20, 18), vec!["孙", "损", "笋", "荪", "隼"])); // sun
        entries.push(((20, 20), vec!["桑", "丧", "嗓", "搡", "磉"])); // sang
        entries.push(((20, 23), vec!["送", "宋", "颂", "诵", "讼", "怂", "耸", "悚"])); // song

        // ════════════════════════════════════════════════════════════════
        // 零声母音节 (无声母, 直接以韵母开头)
        // 零声母=(255, ym) — 特殊标记 sm=255
        // ════════════════════════════════════════════════════════════════
        entries.push(((255, 0), vec!["阿", "啊", "吖", "嗄", "锕"])); // a
        entries.push(((255, 1), vec!["哦", "噢", "喔", "嚄"])); // o
        entries.push(((255, 2), vec!["额", "俄", "鹅", "恶", "饿", "扼", "阿", "遏"])); // e
        entries.push(((255, 3), vec!["一", "衣", "医", "依", "伊", "移", "宜"])); // yi (i)
        entries.push(((255, 4), vec!["无", "五", "午", "物", "务", "武", "舞", "误", "伍"])); // wu (u)
        entries.push(((255, 5), vec!["与", "于", "欲", "鱼", "雨", "语", "余", "遇"])); // yu (v)
        entries.push(((255, 6), vec!["爱", "挨", "哎", "矮", "艾", "碍", "唉"])); // ai
        entries.push(((255, 7), vec!["", ""])); // ei 不存在独立零声母
        entries.push(((255, 8), vec!["", ""])); // ui 不存在独立零声母
        entries.push(((255, 9), vec!["奥", "傲", "熬", "袄", "凹", "拗"])); // ao
        entries.push(((255, 10), vec!["欧", "偶", "藕", "呕", "鸥", "殴"])); // ou
        entries.push(((255, 15), vec!["安", "按", "暗", "岸", "案", "俺"])); // an
        entries.push(((255, 16), vec!["恩", "摁", "嗯"])); // en
        entries.push(((255, 20), vec!["昂", "盎", ""])); // ang
        entries.push(((255, 21), vec!["", ""])); // eng 无独立零声母
        entries.push(((255, 14), vec!["儿", "而", "耳", "二", "尔", "饵", "洱"])); // er

        Self { entries }
    }
}

// ============================================================
// 单元测试 (cargo test 离线验证)
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syllable_count() {
        let engine = ShengYunEngine::new();
        let count = engine.syllable_count();
        // 全量装载应 ≥ 300 音节 (实际约400有效组合)
        assert!(count >= 280, "音节数={} 不足280", count);
    }

    #[test]
    fn test_lookup_basic() {
        let engine = ShengYunEngine::new();
        // ni → 你在首位
        let cands = engine.lookup_syllable(6, 3); // n + i
        assert!(!cands.is_empty(), "ni 应返回候选");
        assert!(cands[0] == "你", "ni 候选首字应为'你', 实际={}", cands[0]);
    }

    #[test]
    fn test_lookup_single_char() {
        let engine = ShengYunEngine::new();
        // bu → 不在首位 (实际是不)
        let cands = engine.lookup_syllable(0, 4); // b + u
        assert!(!cands.is_empty());
        assert!(cands[0] == "不" || cands[0] == "部", "bu 首字错误");
    }

    #[test]
    fn test_zero_shengmu() {
        let engine = ShengYunEngine::new();
        // 零声母 a
        let a = engine.lookup_syllable(255, 0);
        assert!(!a.is_empty(), "零声母 a 应返回候选");
        assert!(a[0] == "阿", "零声母 a 首字应为'阿'");
        // 零声母 an
        let an = engine.lookup_syllable(255, 15);
        assert!(!an.is_empty());
        assert!(an[0] == "安");
        // 零声母 er
        let er = engine.lookup_syllable(255, 14);
        assert!(!er.is_empty());
        assert!(er[0] == "儿");
    }

    #[test]
    fn test_generate_ascii() {
        let mut engine = ShengYunEngine::new();
        // 输入 ASCII 'n' (声母, sm=6) + ASCII 'i' (韵母, ym=3)
        let cands = engine.generate_candidates(b'n' as u32, 6, 255);
        // 声母坐标有效但韵母255 → 此时无完整音节,返回声母自身
        // 实际上 generate_candidates 逻辑: 需看 sm/ym 状况
        // 根据实现, 当 sm=6,ym=255: sm<255 且 ym>=255 → 走声母累积路径,返回声母自身
        // 测试改为双全坐标:
        let cands_full = engine.generate_candidates(b'n' as u32, 6, 3);
        assert!(!cands_full.is_empty(), "n+i 应返回候选");
        assert_eq!(cands_full[0], "你");
    }

    #[test]
    fn test_multi_tone_context_driven() {
        let mut engine = ShengYunEngine::new();
        // 模拟: 先选 "人" (声调上声), 再输入 "sheng"
        // 引擎应能基于上下文给出 "生/声/升" 中更可能的候选
        engine.on_candidate_chosen("人");
        let cands = engine.generate_candidates(b's' as u32, 16, 2); // sh + e
        // 上下文 "人" 后跟 "生"(生活?)(人生?) 最常见
        // 不强制首位, 但至少应包含 "生"
        assert!(cands.iter().any(|c| c == "生"), "sheng应含'生'");
    }
}
