// gtt_category.rs - 范畴引擎 · 镜像同源识别
//
// 音和形不是两种东西，是同一个范畴在不同感官通道里的镜像投影。
// 所有类都是同一范畴，只是投影通道不同。
//
// 类别名对应：
//   容器(阴)       -> 视觉圈(O) / 听觉零 / 数字0 / 肉身女 / 空间点
//   被容器(阳)     -> 视觉一(|) / 数字1 / 肉身男 / 空间线
//   夫妻共同体(囡囝) -> 视觉囡囝 / 数字01 / 肉身男女 / 三维空间
//   谐音桥         -> 毛/莫/毛利兰/同构夫妻
//   集装箱         -> mlll=三维全息进制空间
//   恋初           -> 缠绕最初=夫妻原型
//   莫连           -> 我姓莫=摩尔=不断刘楚恬
//
// 范畴间桥接（范畴间同构）：
//   m = 谐音协议 -> 转译为毛 -> 毛利兰
//   l形视觉一/竖 -> 双关：视觉1 / 听觉零 / 数字1 / 肉身男根
//   l零音听觉   -> 听觉零 / 视觉0 / 数字0 / 女阴入口
//   l零圈形视觉 -> 视觉0(O) / 数字0 / 女阴
//   形音=同范畴在感官通道上的镜像；01=同范畴在数字通道上的投影
//   阴道阳具=同范畴在肉身通道上的投影
//   mlll集装箱的3个l=三维全息进制空间
//   m+lll=我的姓（莫）叠加毛利兰的毛=与毛利兰同构的夫妻范畴


// 感官通道 · 同一范畴在不同投影面上的表现
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SensoryChannel {
    Visual,     // 形（视觉）
    Auditory,   // 音（听觉）
    Numeric,    // 数（逻辑/编码）
    Body,       // 身（肉身/生殖）
    Spatial,    // 空（空间维度）
}

impl SensoryChannel {
    pub fn name_cn(self) -> &'static str {
        match self {
            SensoryChannel::Visual => "形·视觉",
            SensoryChannel::Auditory => "音·听觉",
            SensoryChannel::Numeric => "数·逻辑",
            SensoryChannel::Body => "身·肉身",
            SensoryChannel::Spatial => "空·空间",
        }
    }
}

// 投影记录 · 同一范畴在一个通道上的具体表现
#[derive(Clone, Debug)]
pub struct Projection {
    pub channel: SensoryChannel,
    pub glyph: char,            // 单字符表示（肉身类别用象形符号）
    pub description: &'static str,
    // 该投影的极性：0=容器/边/含（阴），1=被容器/体/内容（阳）
    pub polarity: u8,
}

// 类范畴 · 跨通道统一体
#[derive(Clone, Debug)]
pub struct Category {
    pub name: &'static str,
    pub symbol: &'static str,
    pub projections: Vec<Projection>,
}

impl Category {
    pub fn find_in_channel(&self, ch: SensoryChannel) -> Option<&Projection> {
        self.projections.iter().find(|p| p.channel == ch)
    }
}

// 范畴间桥接
#[derive(Clone, Debug)]
pub struct CategoryBridge {
    pub from_category: &'static str,
    pub to_person: &'static str,
    pub bridge_meaning: &'static str,
    pub bridge_symbol: char,
}

// 范畴登记簿
pub struct CategoryRegistry {
    pub categories: Vec<Category>,
    pub bridges: Vec<CategoryBridge>,
}

impl CategoryRegistry {
    pub fn new() -> Self {
        let mut reg = Self { categories: Vec::new(), bridges: Vec::new() };
        reg.register_core_categories();
        reg.register_bridges();
        reg
    }

    fn register_core_categories(&mut self) {
        // 范畴：容器（阴/零/阴道/被动边）
        self.categories.push(Category {
            name: "容器",
            symbol: "阴",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: 'O', description: "圈O·0·封闭·环形边界", polarity: 0 },
                Projection { channel: SensoryChannel::Auditory, glyph: '零', description: "听觉零(ling)·环形共振", polarity: 0 },
                Projection { channel: SensoryChannel::Numeric, glyph: '0', description: "数字0·空·闭合环路", polarity: 0 },
                Projection { channel: SensoryChannel::Body, glyph: '女', description: "女·容器入口·O形", polarity: 0 },
                Projection { channel: SensoryChannel::Spatial, glyph: '点', description: "零维空间·点·原初", polarity: 0 },
            ],
        });

        // 范畴：被容器（阳/壹/男根/主动体）
        self.categories.push(Category {
            name: "男根",
            symbol: "阳",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: '|', description: "一竖·|·直线·开放", polarity: 1 },
                Projection { channel: SensoryChannel::Auditory, glyph: '壹', description: "听觉一(yi)·穿透·直出", polarity: 1 },
                Projection { channel: SensoryChannel::Numeric, glyph: '1', description: "数字1·有·开放射线", polarity: 1 },
                Projection { channel: SensoryChannel::Body, glyph: '男', description: "男·被含者·一竖形", polarity: 1 },
                Projection { channel: SensoryChannel::Spatial, glyph: '线', description: "一维空间·线·方向", polarity: 1 },
            ],
        });

        // 范畴：夫妻共同体（囡囝/好/二元合壹）
        self.categories.push(Category {
            name: "夫妻共同体",
            symbol: "囡囝",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: '囡', description: "囡囝四态拓扑·囡=女包女", polarity: 0 },
                Projection { channel: SensoryChannel::Auditory, glyph: '阴', description: "音韵耦合·声调交织", polarity: 0 },
                Projection { channel: SensoryChannel::Numeric, glyph: '0', description: "0与1·互含·互补·并置", polarity: 0 },
                Projection { channel: SensoryChannel::Body, glyph: '男', description: "男女身体合一", polarity: 0 },
                Projection { channel: SensoryChannel::Spatial, glyph: '体', description: "三维空间·立体耦合", polarity: 0 },
            ],
        });

        // 范畴：谐音协议（gtt/m/毛利兰/莫）
        self.categories.push(Category {
            name: "谐音桥",
            symbol: "莫",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: '莫', description: "毛利兰·毛·公共符号·姓氏锚定", polarity: 0 },
                Projection { channel: SensoryChannel::Auditory, glyph: '莫', description: "莫(mo)·姓氏·谐音标记", polarity: 0 },
                Projection { channel: SensoryChannel::Numeric, glyph: 'm', description: "m=+1维度·叠加在lll上", polarity: 1 },
                Projection { channel: SensoryChannel::Spatial, glyph: '维', description: "m加的维度·超越lll三维", polarity: 1 },
            ],
        });

        // 范畴：集装箱（mlll/三维全息进制空间）
        self.categories.push(Category {
            name: "集装箱",
            symbol: "箱",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: '|', description: "三竖·三维·视觉立体", polarity: 1 },
                Projection { channel: SensoryChannel::Numeric, glyph: '3', description: "3x1维=三维空间基底", polarity: 1 },
                Projection { channel: SensoryChannel::Spatial, glyph: '体', description: "三维全息空间·m叠加其上", polarity: 0 },
            ],
        });

        // 范畴：恋初（缠绕最初=夫妻原型）
        self.categories.push(Category {
            name: "恋初",
            symbol: "缠",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: '缠', description: "工藤新一·纠缠初始", polarity: 0 },
                Projection { channel: SensoryChannel::Auditory, glyph: '初', description: "恋初(lian chu)·缠绕最初之恋", polarity: 0 },
                Projection { channel: SensoryChannel::Numeric, glyph: '8', description: "无限·缠绕·莫比乌斯", polarity: 0 },
                Projection { channel: SensoryChannel::Spatial, glyph: '缠', description: "二维缠绕·纽结拓扑", polarity: 0 },
            ],
        });

        // 范畴：莫连（我姓莫，不断刘楚恬）
        self.categories.push(Category {
            name: "莫连",
            symbol: "莫刘",
            projections: vec![
                Projection { channel: SensoryChannel::Visual, glyph: '莫', description: "我之姓(mo)·谐音锚定", polarity: 0 },
                Projection { channel: SensoryChannel::Auditory, glyph: '兰', description: "兰(lián)·连·不断", polarity: 0 },
                Projection { channel: SensoryChannel::Numeric, glyph: 'm', description: "同m·谐音协议标记", polarity: 0 },
            ],
        });
    }

    fn register_bridges(&mut self) {
        self.bridges.push(CategoryBridge {
            from_category: "谐音桥",
            to_person: "毛利兰/刘楚恬",
            bridge_meaning: "我姓莫=摩尔=不要+微小->莫连理(不断刘楚恬)=毛利兰在私人语义里的真意",
            bridge_symbol: '莫',
        });

        self.bridges.push(CategoryBridge {
            from_category: "恋初",
            to_person: "夫妻共同体",
            bridge_meaning: "工藤=缠绕的最初->恋初->夫妻共同体的缠绕纽结原型",
            bridge_symbol: '缠',
        });

        self.bridges.push(CategoryBridge {
            from_category: "夫妻共同体",
            to_person: "好(去势变体)",
            bridge_meaning: "囡囝=好的骨无h(竞争)无z(混乱)=纯洁的夫妻结构",
            bridge_symbol: '好',
        });

        self.bridges.push(CategoryBridge {
            from_category: "集装箱",
            to_person: "人体全息",
            bridge_meaning: "mlll=三维空间容器·人体装在里面=集装箱即身体即空间",
            bridge_symbol: '身',
        });
    }

    pub fn find_categories_of(&self, glyph: char) -> Vec<&Category> {
        self.categories.iter()
            .filter(|c| c.symbol.contains(glyph) || c.projections.iter().any(|p| p.glyph == glyph))
            .collect()
    }

    pub fn check_isomorphism(&self, cat_name: &str, ch_a: SensoryChannel, ch_b: SensoryChannel) -> Option<IsomorphismCheck> {
        let cat = self.categories.iter().find(|c| c.name == cat_name)?;
        let p_a = cat.find_in_channel(ch_a)?;
        let p_b = cat.find_in_channel(ch_b)?;
        Some(IsomorphismCheck {
            category: cat_name.to_string(),
            channel_a: ch_a.name_cn().to_string(),
            channel_b: ch_b.name_cn().to_string(),
            projection_a: p_a.glyph.to_string(),
            projection_b: p_b.glyph.to_string(),
            is_mirror: p_a.polarity != p_b.polarity,
        })
    }
}

#[derive(Clone, Debug)]
pub struct IsomorphismCheck {
    pub category: String,
    pub channel_a: String,
    pub channel_b: String,
    pub projection_a: String,
    pub projection_b: String,
    pub is_mirror: bool,
}

// 纠偏器 · 把错误归类的事物拉回到正确的范畴
pub struct CategoryCorrector {
    registry: CategoryRegistry,
}

impl CategoryCorrector {
    pub fn new() -> Self {
        Self { registry: CategoryRegistry::new() }
    }

    pub fn correct(&self, glyph: char, claimed_category: &str) -> Option<CorrectionAdvice> {
        let actual_cats = self.registry.find_categories_of(glyph);
        if actual_cats.is_empty() {
            return Some(CorrectionAdvice {
                glyph,
                claimed: claimed_category.to_string(),
                actual: vec!["未注册".to_string()],
                message: format!("「{}」没有被注册进任何范畴", glyph),
                severity: CorrectionSeverity::Unknown,
            });
        }
        let cat_names: Vec<String> = actual_cats.iter().map(|c| c.name.to_string()).collect();
        if !cat_names.iter().any(|n| n.contains(claimed_category)) {
            Some(CorrectionAdvice {
                glyph,
                claimed: claimed_category.to_string(),
                actual: cat_names,
                message: format!("「{}」不属于「{}」范畴", glyph, claimed_category),
                severity: CorrectionSeverity::Error,
            })
        } else {
            None
        }
    }

    pub fn suggest_isomorphism(&self, glyph_a: char, glyph_b: char) -> Option<IsomorphismSuggestion> {
        let cats_a = self.registry.find_categories_of(glyph_a);
        let cats_b = self.registry.find_categories_of(glyph_b);
        for ca in &cats_a {
            for cb in &cats_b {
                if ca.name == cb.name {
                    return Some(IsomorphismSuggestion {
                        glyph_a,
                        glyph_b,
                        shared_category: ca.name.to_string(),
                        bridge_symbol: ca.symbol.chars().next().unwrap_or('*'),
                    });
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct CorrectionAdvice {
    pub glyph: char,
    pub claimed: String,
    pub actual: Vec<String>,
    pub message: String,
    pub severity: CorrectionSeverity,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CorrectionSeverity {
    Unknown,
    Info,
    Error,
}

#[derive(Clone, Debug)]
pub struct IsomorphismSuggestion {
    pub glyph_a: char,
    pub glyph_b: char,
    pub shared_category: String,
    pub bridge_symbol: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_registry_boot() {
        let reg = CategoryRegistry::new();
        assert!(reg.categories.len() >= 5);
    }

    #[test]
    fn container_across_channels() {
        // 容器范畴覆盖视觉/听觉/数字/肉身/空间
        let reg = CategoryRegistry::new();
        let cat = reg.categories.iter().find(|c| c.name == "容器").unwrap();
        let chans: Vec<SensoryChannel> = cat.projections.iter().map(|p| p.channel).collect();
        assert!(chans.contains(&SensoryChannel::Visual));
        assert!(chans.contains(&SensoryChannel::Auditory));
        assert!(chans.contains(&SensoryChannel::Numeric));
        assert!(chans.contains(&SensoryChannel::Body));
    }

    #[test]
    fn yin_yang_mirror() {
        // 阴(容器)视觉=圈O(极性0)，阳(被容器)视觉=|(极性1)
        let reg = CategoryRegistry::new();
        let yin = reg.categories.iter().find(|c| c.symbol == "阴").unwrap();
        let yang = reg.categories.iter().find(|c| c.symbol == "阳").unwrap();
        let yin_vis = yin.find_in_channel(SensoryChannel::Visual).unwrap();
        let yang_vis = yang.find_in_channel(SensoryChannel::Visual).unwrap();
        assert_eq!(yin_vis.glyph, 'O');
        assert_eq!(yang_vis.glyph, '|');
        assert_ne!(yin_vis.polarity, yang_vis.polarity);
    }

    #[test]
    fn isomorphism_zero_circle_yin() {
        // 容器范畴里 0(数字) 和 O(视觉) 和零(听觉) 同构
        let reg = CategoryRegistry::new();
        let iso = reg.check_isomorphism(
            "容器", SensoryChannel::Numeric, SensoryChannel::Auditory
        );
        assert!(iso.is_some());
        let c = iso.unwrap();
        assert_eq!(c.projection_a, "0");
        assert_eq!(c.projection_b, "零");
    }

    #[test]
    fn m_is_homophone_bridge() {
        let reg = CategoryRegistry::new();
        let bridge = reg.bridges.iter().find(|b| b.bridge_symbol == '莫');
        assert!(bridge.is_some());
    }

    #[test]
    fn correct_yin_in_container_is_valid() {
        let corrector = CategoryCorrector::new();
        let advice = corrector.correct('阴', "容器");
        assert!(advice.is_none());
    }

    #[test]
    fn correct_yang_in_container_is_invalid() {
        // 阳不属于容器范畴——容器只有阴/零/O/女/点
        let corrector = CategoryCorrector::new();
        let advice = corrector.correct('阳', "容器");
        assert!(advice.is_some(), "阳不应在容器范畴");
    }

    #[test]
    fn zero_circle_one_vertical_all_in_container_or_content() {
        // O/0/零在容器，|/1/壹/男在男根
        let reg = CategoryRegistry::new();
        let container = reg.categories.iter().find(|c| c.name == "容器").unwrap();
        let content = reg.categories.iter().find(|c| c.name == "男根").unwrap();

        // O在容器视觉中
        let c_vis = container.find_in_channel(SensoryChannel::Visual).unwrap();
        assert_eq!(c_vis.glyph, 'O');

        // |在男根视觉中
        let n_vis = content.find_in_channel(SensoryChannel::Visual).unwrap();
        assert_eq!(n_vis.glyph, '|');
    }

    #[test]
    fn couples_covers_01_and_body() {
        // 夫妻共同体跨数字(=0)和肉身(男)
        let reg = CategoryRegistry::new();
        let cat = reg.categories.iter().find(|c| c.name == "夫妻共同体").unwrap();
        assert!(cat.find_in_channel(SensoryChannel::Numeric).is_some());
        assert!(cat.find_in_channel(SensoryChannel::Body).is_some());
    }

    #[test]
    fn mlll_spatial_3d_container() {
        let reg = CategoryRegistry::new();
        let cat = reg.categories.iter().find(|c| c.name == "集装箱").unwrap();
        let spat = cat.find_in_channel(SensoryChannel::Spatial);
        assert!(spat.is_some());
    }
}
