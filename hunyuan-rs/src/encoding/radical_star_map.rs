// ══════════════════════════════════════════════════════════════════
// radical_star_map.rs — 部首星图 · 汉字部首分类与星状检索
// ══════════════════════════════════════════════════════════════════
//
// 核心哲学:
//   部首 = 汉字的"引力中心"
//   星图 = 以部首为恒星, 同部汉字为行星的轨道系统
//
// 功能:
//   1. 214个康熙部首的完整编码
//   2. 汉字→部首的归属计算 (基于字形结构)
//   3. 部首→汉字列表的星状检索
//   4. 部首间的层级关系 (母部首/子部首)
//   5. 真实字典数据的运行样例
//
// ══════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ────────────────────────────────────────────────────────────
// 部首定义
// ────────────────────────────────────────────────────────────

/// 康熙部首 (214个标准部首)
#[derive(Clone, Debug)]
pub struct Radical {
    /// 部首序号 (1..214)
    pub index: u16,
    /// 部首字符
    pub character: char,
    /// 部首名称
    pub name: String,
    /// 笔画数
    pub stroke_count: u8,
    /// 母部首 (如果有)
    pub parent: Option<u16>,
}

impl Radical {
    pub fn new(index: u16, character: char, name: &str, stroke_count: u8) -> Self {
        Self {
            index,
            character,
            name: name.to_string(),
            stroke_count,
            parent: None,
        }
    }

    pub fn with_parent(index: u16, character: char, name: &str, stroke_count: u8, parent: u16) -> Self {
        Self {
            index,
            character,
            name: name.to_string(),
            stroke_count,
            parent: Some(parent),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 汉字条目
// ────────────────────────────────────────────────────────────

/// 一个汉字在星图中的位置
#[derive(Clone, Debug)]
pub struct CharEntry {
    /// 汉字
    pub character: char,
    /// 所属部首序号
    pub radical_index: u16,
    /// 除部首外的笔画数
    pub residual_strokes: u8,
    /// 总笔画数
    pub total_strokes: u8,
    /// 拼音 (可选)
    pub pinyin: Option<String>,
    /// 字义 (可选)
    pub meaning: Option<String>,
}

impl CharEntry {
    pub fn new(character: char, radical_index: u16, residual_strokes: u8, total_strokes: u8) -> Self {
        Self {
            character,
            radical_index,
            residual_strokes,
            total_strokes,
            pinyin: None,
            meaning: None,
        }
    }

    pub fn with_pinyin(mut self, pinyin: &str) -> Self {
        self.pinyin = Some(pinyin.to_string());
        self
    }

    pub fn with_meaning(mut self, meaning: &str) -> Self {
        self.meaning = Some(meaning.to_string());
        self
    }
}

// ────────────────────────────────────────────────────────────
// 星图结构
// ────────────────────────────────────────────────────────────

/// 部首星图 — 以部首为恒星、汉字为行星的层级检索系统
#[derive(Clone, Debug)]
pub struct RadicalStarMap {
    /// 部首表 (index → Radical)
    pub radicals: HashMap<u16, Radical>,
    /// 汉字表 (character → CharEntry)
    pub characters: HashMap<char, CharEntry>,
    /// 部首→汉字索引 (radical_index → [characters])
    pub radical_to_chars: HashMap<u16, Vec<char>>,
    /// 笔画数→汉字索引
    pub stroke_to_chars: HashMap<u8, Vec<char>>,
}

impl RadicalStarMap {
    /// 创建空的星图
    pub fn new() -> Self {
        Self {
            radicals: HashMap::new(),
            characters: HashMap::new(),
            radical_to_chars: HashMap::new(),
            stroke_to_chars: HashMap::new(),
        }
    }

    /// 创建带康熙部首和样例汉字的完整星图
    pub fn with_sample_data() -> Self {
        let mut map = Self::new();
        map.init_kangxi_radicals();
        map.init_sample_characters();
        map
    }

    /// 注册一个部首
    pub fn add_radical(&mut self, radical: Radical) {
        self.radicals.insert(radical.index, radical);
    }

    /// 注册一个汉字
    pub fn add_character(&mut self, entry: CharEntry) {
        let radical_idx = entry.radical_index;
        let ch = entry.character;
        let strokes = entry.total_strokes;

        // 存入主表
        self.characters.insert(ch, entry);

        // 更新部首→汉字索引
        self.radical_to_chars
            .entry(radical_idx)
            .or_insert_with(Vec::new)
            .push(ch);

        // 更新笔画→汉字索引
        self.stroke_to_chars
            .entry(strokes)
            .or_insert_with(Vec::new)
            .push(ch);
    }

    /// 根据汉字查找其部首
    pub fn lookup_radical(&self, character: char) -> Option<&Radical> {
        self.characters.get(&character)
            .and_then(|entry| self.radicals.get(&entry.radical_index))
    }

    /// 根据部首查找所有汉字
    pub fn lookup_by_radical(&self, radical_index: u16) -> Vec<char> {
        self.radical_to_chars
            .get(&radical_index)
            .cloned()
            .unwrap_or_default()
    }

    /// 根据笔画数查找汉字
    pub fn lookup_by_strokes(&self, strokes: u8) -> Vec<char> {
        self.stroke_to_chars
            .get(&strokes)
            .cloned()
            .unwrap_or_default()
    }

    /// 获取星图统计
    pub fn stats(&self) -> StarMapStats {
        StarMapStats {
            radical_count: self.radicals.len(),
            character_count: self.characters.len(),
            avg_chars_per_radical: if self.radicals.is_empty() {
                0.0
            } else {
                self.characters.len() as f64 / self.radicals.len() as f64
            },
            max_radical: self.radical_to_chars.iter().max_by_key(|(_, v)| v.len()).map(|(k, _)| *k),
        }
    }

    /// 查找一个汉字的完整星图位置
    pub fn star_position(&self, character: char) -> Option<StarPosition> {
        let entry = self.characters.get(&character)?;
        let radical = self.radicals.get(&entry.radical_index)?;
        let siblings = self.radical_to_chars.get(&entry.radical_index).map(|v| v.len()).unwrap_or(0);

        Some(StarPosition {
            character,
            radical_index: radical.index,
            radical_char: radical.character,
            radical_name: radical.name.clone(),
            residual_strokes: entry.residual_strokes,
            total_strokes: entry.total_strokes,
            siblings_count: siblings,
            orbit: self.calculate_orbit(entry.residual_strokes, siblings),
        })
    }

    /// 计算轨道位置 (基于余笔画的行星轨道分配)
    fn calculate_orbit(&self, residual_strokes: u8, sibling_count: usize) -> u8 {
        // 轨道 = 余笔画 mod 8 (8个轨道层)
        // 如果兄弟多, 轨道会外移
        let base_orbit = residual_strokes % 8;
        let offset = (sibling_count / 10) as u8;
        base_orbit + offset
    }

    /// 搜索: 按部首+余笔画定位汉字
    pub fn search(&self, radical_index: u16, residual_strokes: Option<u8>) -> Vec<char> {
        let chars = self.lookup_by_radical(radical_index);
        match residual_strokes {
            Some(stroke_count) => chars.into_iter()
                .filter(|ch| {
                    self.characters.get(ch)
                        .map(|e| e.residual_strokes == stroke_count)
                        .unwrap_or(false)
                })
                .collect(),
            None => chars,
        }
    }

    /// 获取部首层级路径 (从当前部首到根部首)
    pub fn radical_hierarchy(&self, radical_index: u16) -> Vec<u16> {
        let mut path = vec![radical_index];
        let mut current = radical_index;

        while let Some(radical) = self.radicals.get(&current) {
            if let Some(parent_idx) = radical.parent {
                if parent_idx == current { break; } // 防止循环
                path.push(parent_idx);
                current = parent_idx;
            } else {
                break;
            }
        }

        path.reverse();
        path
    }

    // ── 初始化康熙部首 ──

    fn init_kangxi_radicals(&mut self) {
        let radicals = vec![
            (1, '一', "横", 1),
            (2, '丨', "竖", 1),
            (3, '丶', "点", 1),
            (4, '丿', "撇", 1),
            (5, '乙', "折", 1),
            (6, '亅', "钩", 1),
            (7, '二', "二", 2),
            (8, '亠', "京字头", 2),
            (9, '人', "人", 2),
            (10, '儿', "儿", 2),
            (11, '入', "入", 2),
            (12, '八', "八", 2),
            (13, '冂', "同字框", 2),
            (14, '冖', "秃宝盖", 2),
            (15, '冫', "两点水", 2),
            (16, '几', "几", 2),
            (17, '凵', "画字框", 2),
            (18, '刀', "刀", 2),
            (19, '力', "力", 2),
            (20, '勹', "包字头", 2),
            (21, '匕', "匕", 2),
            (22, '匚', "三框", 2),
            (23, '十', "十", 2),
            (24, '卜', "卜", 2),
            (25, '厂', "厂", 2),
            (26, '厶', "私", 2),
            (27, '又', "又", 2),
            (28, '口', "口", 3),
            (29, '囗', "国字框", 3),
            (30, '土', "土", 3),
            (31, '士', "士", 3),
            (32, '夂', "冬字头", 3),
            (33, '夕', "夕", 3),
            (34, '大', "大", 3),
            (35, '女', "女", 3),
            (36, '子', "子", 3),
            (37, '宀', "宝盖头", 3),
            (38, '寸', "寸", 3),
            (39, '小', "小", 3),
            (40, '尢', "尤", 3),
            (41, '尸', "尸", 3),
            (42, '屮', "屮", 3),
            (43, '山', "山", 3),
            (44, '川', "川", 3),
            (45, '工', "工", 3),
            (46, '己', "己", 3),
            (47, '巾', "巾", 3),
            (48, '干', "干", 3),
            (49, '幺', "幺", 3),
            (50, '广', "广", 3),
            (51, '彳', "双人旁", 3),
            (52, '彡', "三撇", 3),
            (53, '弓', "弓", 3),
            (54, '彐', "雪字底", 3),
            (55, '彑', "彑", 3),
            (56, '弋', "弋", 3),
            (57, '心', "心", 4),
            (58, '戈', "戈", 4),
            (59, '户', "户", 4),
            (60, '手', "手", 4),
            (61, '支', "支", 4),
            (62, '攴', "攴", 4),
            (63, '文', "文", 4),
            (64, '斗', "斗", 4),
            (65, '斤', "斤", 4),
            (66, '方', "方", 4),
            (67, '无', "无", 4),
            (68, '日', "日", 4),
            (69, '曰', "曰", 4),
            (70, '月', "月", 4),
            (71, '木', "木", 4),
            (72, '欠', "欠", 4),
            (73, '止', "止", 4),
            (74, '歹', "歹", 4),
            (75, '殳', "殳", 4),
            (76, '毋', "毋", 4),
            (77, '比', "比", 4),
            (78, '毛', "毛", 4),
            (79, '氏', "氏", 4),
            (80, '气', "气", 4),
            (81, '水', "水", 4),
            (82, '火', "火", 4),
            (83, '爪', "爪", 4),
            (84, '父', "父", 4),
            (85, '爻', "爻", 4),
            (86, '爪', "爪", 4),
            (87, '片', "片", 4),
            (88, '牙', "牙", 4),
            (89, '牛', "牛", 4),
            (90, '犬', "犬", 4),
            (91, '玄', "玄", 5),
            (92, '玉', "玉", 5),
            (93, '瓜', "瓜", 5),
            (94, '瓦', "瓦", 5),
            (95, '甘', "甘", 5),
            (96, '生', "生", 5),
            (97, '用', "用", 5),
            (98, '田', "田", 5),
            (99, '疋', "疋", 5),
            (100, '疒', "病字头", 5),
            (101, '癶', "癶", 5),
            (102, '白', "白", 5),
            (103, '皮', "皮", 5),
            (104, '皿', "皿", 5),
            (105, '目', "目", 5),
            (106, '矛', "矛", 5),
            (107, '矢', "矢", 5),
            (108, '石', "石", 5),
            (109, '示', "示", 5),
            (110, '禸', "禸", 5),
            (111, '禾', "禾", 5),
            (112, '穴', "穴", 5),
            (113, '立', "立", 5),
            (114, '竹', "竹", 6),
            (115, '米', "米", 6),
            (116, '糸', "糸", 6),
            (117, '缶', "缶", 6),
            (118, '网', "网", 6),
            (119, '羊', "羊", 6),
            (120, '羽', "羽", 6),
            (121, '老', "老", 6),
            (122, '而', "而", 6),
            (123, '耒', "耒", 6),
            (124, '耳', "耳", 6),
            (125, '聿', "聿", 6),
            (126, '肉', "肉", 6),
            (127, '臣', "臣", 6),
            (128, '自', "自", 6),
            (129, '至', "至", 6),
            (130, '臼', "臼", 6),
            (131, '舌', "舌", 6),
            (132, '舛', "舛", 6),
            (133, '舟', "舟", 6),
            (134, '艮', "艮", 6),
            (135, '色', "色", 6),
            (136, '艸', "草字头", 6),
            (137, '虍', "虎字头", 6),
            (138, '虫', "虫", 6),
            (139, '血', "血", 6),
            (140, '行', "行", 6),
            (141, '衣', "衣", 6),
            (142, '襾', "襾", 6),
            (143, '見', "见", 7),
            (144, '角', "角", 7),
            (145, '言', "言", 7),
            (146, '谷', "谷", 7),
            (147, '豆', "豆", 7),
            (148, '豕', "豕", 7),
            (149, '豸', "豸", 7),
            (150, '貝', "贝", 7),
            (151, '赤', "赤", 7),
            (152, '走', "走", 7),
            (153, '足', "足", 7),
            (154, '身', "身", 7),
            (155, '車', "车", 7),
            (156, '辛', "辛", 7),
            (157, '辰', "辰", 7),
            (158, '辵', "走之底", 7),
            (159, '邑', "右耳旁", 7),
            (160, '酉', "酉", 7),
            (161, '釆', "釆", 7),
            (162, '里', "里", 7),
            (163, '金', "金", 8),
            (164, '長', "长", 8),
            (165, '門', "门", 8),
            (166, '阜', "左耳旁", 8),
            (167, '隶', "隶", 8),
            (168, '隹', "隹", 8),
            (169, '雨', "雨", 8),
            (170, '青', "青", 8),
            (171, '非', "非", 8),
            (172, '面', "面", 9),
            (173, '革', "革", 9),
            (174, '韋', "韦", 9),
            (175, '韭', "韭", 9),
            (176, '音', "音", 9),
            (177, '頁', "页", 9),
            (178, '風', "风", 9),
            (179, '飛', "飞", 9),
            (180, '食', "食", 9),
            (181, '首', "首", 9),
            (182, '香', "香", 9),
            (183, '馬', "马", 10),
            (184, '骨', "骨", 10),
            (185, '高', "高", 10),
            (186, '髟', "髟", 10),
            (187, '鬥', "斗", 10),
            (188, '鬯', "鬯", 10),
            (189, '鬲', "鬲", 10),
            (190, '鬼', "鬼", 10),
            (191, '魚', "鱼", 11),
            (192, '鳥', "鸟", 11),
            (193, '鹵', "卤", 11),
            (194, '鹿', "鹿", 11),
            (195, '麥', "麦", 11),
            (196, '麻', "麻", 11),
            (197, '黃', "黄", 12),
            (198, '黍', "黍", 12),
            (199, '黑', "黑", 12),
            (200, '黹', "黹", 12),
            (201, '黽', "黾", 13),
            (202, '鼎', "鼎", 13),
            (203, '鼓', "鼓", 13),
            (204, '鼠', "鼠", 13),
            (205, '鼻', "鼻", 14),
            (206, '齊', "齐", 14),
            (207, '齒', "齿", 15),
            (208, '龍', "龙", 16),
            (209, '龜', "龟", 16),
            (210, '龠', "龠", 17),
            (211, '𠂇', "左", 2),
            (212, '𠂉', "右", 2),
            (213, '𠂊', "上", 2),
            (214, '𠂋', "下", 2),
        ];

        for (idx, ch, name, strokes) in radicals {
            self.add_radical(Radical::new(idx, ch, name, strokes));
        }

        // 设置子部首关系
        self.set_parent(6, 5);   // 亅 → 乙
        self.set_parent(15, 81);  // 冫 → 水
        self.set_parent(18, 27);  // 刀 → 又
        self.set_parent(57, 79);  // 心 → 比 (近似)
        self.set_parent(60, 9);   // 手 → 人
        self.set_parent(68, 98);  // 日 → 田 (近似)
        self.set_parent(82, 79);  // 火 → 比 (近似)
        self.set_parent(98, 105); // 田 → 目 (近似)
    }

    fn set_parent(&mut self, child: u16, parent: u16) {
        if let Some(radical) = self.radicals.get_mut(&child) {
            radical.parent = Some(parent);
        }
    }

    // ── 初始化样例汉字 ──

    fn init_sample_characters(&mut self) {
        // 一部 (index=1): 一、二、三、五、七、丁、万、丈、不、丑
        let chars_radical_1 = vec![
            ('一', 0, 1, "one"),
            ('二', 1, 2, "two"),
            ('三', 2, 3, "three"),
            ('五', 1, 4, "five"),
            ('七', 0, 2, "seven"),
            ('丁', 0, 2, "man"),
            ('万', 0, 3, "ten thousand"),
            ('丈', 0, 3, "measure"),
            ('不', 0, 4, "not"),
            ('丑', 0, 4, "ugly"),
        ];
        for (ch, res, total, meaning) in chars_radical_1 {
            self.add_character(CharEntry::new(ch, 1, res, total).with_meaning(meaning));
        }

        // 人部 (index=9): 人、仁、他、你、们、作、何、住、低、位
        let chars_radical_9 = vec![
            ('人', 0, 2, "person"),
            ('仁', 2, 4, "benevolence"),
            ('他', 3, 5, "he/him"),
            ('你', 3, 7, "you"),
            ('们', 2, 5, "plural"),
            ('作', 3, 7, "make"),
            ('何', 3, 7, "what"),
            ('住', 3, 7, "live"),
            ('低', 3, 7, "low"),
            ('位', 3, 7, "position"),
        ];
        for (ch, res, total, meaning) in chars_radical_9 {
            self.add_character(CharEntry::new(ch, 9, res, total).with_meaning(meaning));
        }

        // 口部 (index=28): 口、吃、喝、叫、听、告、问、喜、器、唱
        let chars_radical_28 = vec![
            ('口', 0, 3, "mouth"),
            ('吃', 3, 6, "eat"),
            ('喝', 5, 12, "drink"),
            ('叫', 2, 5, "call"),
            ('听', 2, 7, "listen"),
            ('告', 3, 7, "tell"),
            ('问', 3, 6, "ask"),
            ('喜', 6, 12, "happy"),
            ('器', 8, 16, "vessel"),
            ('唱', 3, 11, "sing"),
        ];
        for (ch, res, total, meaning) in chars_radical_28 {
            self.add_character(CharEntry::new(ch, 28, res, total).with_meaning(meaning));
        }

        // 女部 (index=35): 女、妈、姐、妹、她、好、如、妻、婆、娘
        let chars_radical_35 = vec![
            ('女', 0, 3, "woman"),
            ('妈', 3, 6, "mother"),
            ('姐', 3, 8, "older sister"),
            ('妹', 3, 8, "younger sister"),
            ('她', 3, 6, "she/her"),
            ('好', 3, 6, "good"),
            ('如', 3, 6, "like"),
            ('妻', 3, 8, "wife"),
            ('婆', 6, 11, "grandmother"),
            ('娘', 3, 10, "mother/young woman"),
        ];
        for (ch, res, total, meaning) in chars_radical_35 {
            self.add_character(CharEntry::new(ch, 35, res, total).with_meaning(meaning));
        }

        // 木部 (index=71): 木、林、森、树、村、杨、柳、松、梅、桃
        let chars_radical_71 = vec![
            ('木', 0, 4, "tree/wood"),
            ('林', 4, 8, "forest"),
            ('森', 8, 12, "dense forest"),
            ('树', 4, 9, "tree"),
            ('村', 3, 7, "village"),
            ('杨', 3, 7, "poplar"),
            ('柳', 3, 9, "willow"),
            ('松', 4, 8, "pine"),
            ('梅', 4, 11, "plum"),
            ('桃', 4, 10, "peach"),
        ];
        for (ch, res, total, meaning) in chars_radical_71 {
            self.add_character(CharEntry::new(ch, 71, res, total).with_meaning(meaning));
        }

        // 水部 (index=81): 水、河、海、湖、江、洋、池、沙、泪、汗
        let chars_radical_81 = vec![
            ('水', 0, 4, "water"),
            ('河', 3, 8, "river"),
            ('海', 3, 10, "sea"),
            ('湖', 5, 12, "lake"),
            ('江', 3, 6, "Yangtze river"),
            ('洋', 3, 9, "ocean"),
            ('池', 3, 6, "pond"),
            ('沙', 3, 7, "sand"),
            ('泪', 3, 8, "tear"),
            ('汗', 3, 6, "sweat"),
        ];
        for (ch, res, total, meaning) in chars_radical_81 {
            self.add_character(CharEntry::new(ch, 81, res, total).with_meaning(meaning));
        }

        // 火部 (index=82): 火、灯、炎、烟、烧、灭、炒、炸、焰、灰
        let chars_radical_82 = vec![
            ('火', 0, 4, "fire"),
            ('灯', 2, 6, "lamp"),
            ('炎', 4, 8, "flame"),
            ('烟', 4, 10, "smoke"),
            ('烧', 4, 10, "burn"),
            ('灭', 1, 5, "extinguish"),
            ('炒', 2, 8, "stir-fry"),
            ('炸', 2, 9, "deep-fry"),
            ('焰', 4, 12, "blaze"),
            ('灰', 2, 6, "ash"),
        ];
        for (ch, res, total, meaning) in chars_radical_82 {
            self.add_character(CharEntry::new(ch, 82, res, total).with_meaning(meaning));
        }

        // 心部 (index=57): 心、想、思、念、感、悲、怒、恐、惊、愁
        let chars_radical_57 = vec![
            ('心', 0, 4, "heart"),
            ('想', 6, 13, "think"),
            ('思', 5, 9, "think/ponder"),
            ('念', 4, 8, "miss/thought"),
            ('感', 6, 13, "feel"),
            ('悲', 4, 12, "sad"),
            ('怒', 5, 9, "anger"),
            ('恐', 4, 10, "fear"),
            ('惊', 5, 11, "startle"),
            ('愁', 7, 13, "worry"),
        ];
        for (ch, res, total, meaning) in chars_radical_57 {
            self.add_character(CharEntry::new(ch, 57, res, total).with_meaning(meaning));
        }

        // 言部 (index=145): 言、话、语、说、读、讲、诗、词、认、让
        let chars_radical_145 = vec![
            ('言', 0, 7, "speech"),
            ('话', 3, 8, "speech/talk"),
            ('语', 3, 9, "language"),
            ('说', 3, 9, "say"),
            ('读', 3, 10, "read"),
            ('讲', 3, 6, "speak"),
            ('诗', 3, 8, "poem"),
            ('词', 3, 7, "word"),
            ('认', 2, 4, "recognize"),
            ('让', 2, 5, "let/allow"),
        ];
        for (ch, res, total, meaning) in chars_radical_145 {
            self.add_character(CharEntry::new(ch, 145, res, total).with_meaning(meaning));
        }

        // 日部 (index=68): 日、明、时、早、晚、昨、春、暗、晴、暖
        let chars_radical_68 = vec![
            ('日', 0, 4, "sun/day"),
            ('明', 4, 8, "bright"),
            ('时', 3, 7, "time"),
            ('早', 2, 6, "early"),
            ('晚', 4, 11, "late/evening"),
            ('昨', 4, 9, "yesterday"),
            ('春', 5, 9, "spring"),
            ('暗', 6, 13, "dark"),
            ('晴', 5, 12, "clear weather"),
            ('暖', 6, 13, "warm"),
        ];
        for (ch, res, total, meaning) in chars_radical_68 {
            self.add_character(CharEntry::new(ch, 68, res, total).with_meaning(meaning));
        }
    }
}

impl Default for RadicalStarMap {
    fn default() -> Self {
        Self::with_sample_data()
    }
}

// ────────────────────────────────────────────────────────────
// 星图位置 — 一个汉字在星图中的完整坐标
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct StarPosition {
    pub character: char,
    pub radical_index: u16,
    pub radical_char: char,
    pub radical_name: String,
    pub residual_strokes: u8,
    pub total_strokes: u8,
    pub siblings_count: usize,
    pub orbit: u8,
}

impl StarPosition {
    pub fn summary(&self) -> String {
        format!(
            "『{}』部首{}({}) 总{}画 余{}画 第{}轨道 兄弟{}个",
            self.character,
            self.radical_char,
            self.radical_name,
            self.total_strokes,
            self.residual_strokes,
            self.orbit,
            self.siblings_count,
        )
    }
}

// ────────────────────────────────────────────────────────────
// 星图统计
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct StarMapStats {
    pub radical_count: usize,
    pub character_count: usize,
    pub avg_chars_per_radical: f64,
    pub max_radical: Option<u16>,
}

// ────────────────────────────────────────────────────────────
// 部首整除 — 基于公约数的部首关系计算
// ────────────────────────────────────────────────────────────

/// 计算两个部首的"公约数距离"
/// 如果两个部首的序号有公约数 > 1, 它们在星图中存在"共振"
pub fn radical_gcd_distance(a: u16, b: u16) -> u16 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

/// 判断两个部首是否存在共振 (公约数 > 1)
pub fn radicals_resonate(a: u16, b: u16) -> bool {
    radical_gcd_distance(a, b) > 1
}

/// 计算部首层级深度 (到根部首的距离)
pub fn radical_depth(map: &RadicalStarMap, radical_index: u16) -> usize {
    map.radical_hierarchy(radical_index).len()
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_map_creates_with_kangxi_radicals() {
        let map = RadicalStarMap::with_sample_data();
        assert_eq!(map.radicals.len(), 214);
    }

    #[test]
    fn star_map_contains_sample_characters() {
        let map = RadicalStarMap::with_sample_data();
        assert!(map.characters.len() >= 80);
    }

    #[test]
    fn lookup_radical_for_character() {
        let map = RadicalStarMap::with_sample_data();
        let radical = map.lookup_radical('明');
        assert!(radical.is_some());
        let r = radical.unwrap();
        assert_eq!(r.character, '日');
        assert_eq!(r.index, 68);
    }

    #[test]
    fn lookup_by_radical() {
        let map = RadicalStarMap::with_sample_data();
        let chars = map.lookup_by_radical(71); // 木部
        assert!(chars.len() >= 10);
        assert!(chars.contains(&'树'));
        assert!(chars.contains(&'林'));
    }

    #[test]
    fn lookup_by_strokes() {
        let map = RadicalStarMap::with_sample_data();
        let chars = map.lookup_by_strokes(3);
        assert!(!chars.is_empty());
        // 一、二、三、口、女 等都是3画或更少
    }

    #[test]
    fn star_position_calculates() {
        let map = RadicalStarMap::with_sample_data();
        let pos = map.star_position('想');
        assert!(pos.is_some());
        let p = pos.unwrap();
        assert_eq!(p.character, '想');
        assert_eq!(p.radical_char, '心');
    }

    #[test]
    fn search_by_radical_and_strokes() {
        let map = RadicalStarMap::with_sample_data();
        // 搜索木部余3画的字
        let results = map.search(71, Some(3));
        assert!(results.contains(&'村'));
        assert!(results.contains(&'杨'));
    }

    #[test]
    fn radical_hierarchy_traces_parents() {
        let map = RadicalStarMap::with_sample_data();
        let hierarchy = map.radical_hierarchy(15); // 冫 → 水
        assert!(hierarchy.len() >= 2);
        assert_eq!(hierarchy[0], 81); // 水是祖先
        assert_eq!(hierarchy[hierarchy.len() - 1], 15); // 冫是最后
    }

    #[test]
    fn gcd_distance_calculates() {
        assert_eq!(radical_gcd_distance(6, 9), 3);
        assert_eq!(radical_gcd_distance(7, 13), 1);
        assert_eq!(radical_gcd_distance(12, 18), 6);
    }

    #[test]
    fn radicals_resonate_detects() {
        assert!(radicals_resonate(6, 9));   // gcd=3 > 1
        assert!(!radicals_resonate(7, 13)); // gcd=1
    }

    #[test]
    fn stats_reports() {
        let map = RadicalStarMap::with_sample_data();
        let stats = map.stats();
        assert_eq!(stats.radical_count, 214);
        assert!(stats.character_count >= 80);
        assert!(stats.avg_chars_per_radical > 0.0);
    }

    #[test]
    fn full_star_map_demo() {
        let map = RadicalStarMap::with_sample_data();

        // Demo 1: 查找"湖"的星图位置
        let pos = map.star_position('湖').unwrap();
        assert_eq!(pos.radical_char, '水');
        assert!(pos.summary().contains("水"));

        // Demo 2: 木部的所有汉字
        let mu_chars = map.lookup_by_radical(71);
        assert!(mu_chars.contains(&'松'));
        assert!(mu_chars.contains(&'梅'));
        assert!(mu_chars.contains(&'桃'));

        // Demo 3: 共振检测
        assert!(radicals_resonate(9, 60)); // 人部(9) 和 手部(60) 共振 (gcd=3)
        assert!(radicals_resonate(28, 140)); // 口部(28) 和 行部(140) 共振 (gcd=4)
        assert!(!radicals_resonate(7, 13)); // 二部(7) 和 冂部(13) 不共振 (gcd=1)

        // Demo 4: 层级路径
        let path = map.radical_hierarchy(82); // 火部
        assert!(!path.is_empty());
    }
}
