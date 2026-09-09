// ══════════════════════════════════════════════════════════════════
// parasite.rs — 蚕蛹反向寄生壳 · 内核集成版
// ══════════════════════════════════════════════════════════════════
//
// 核心机制: 让外壳效仿内核，而非内核适应外壳
//
// 三个动作:
//   1. 脱壳 (shed)  — 剥离外壳原有身份
//   2. 选壳 (choose) — 挑选要寄生的外壳
//   3. 穿衣 (dress)  — 让外壳披上内核的外观
//
// 反向规则: 外壳必须遵守的约定（不是内核去适应外壳）
//
// 与 high_algebra 集成: 寄生虫可存在于不同偏移世界层级，
// 形成跨平行世界的分布式生存能力。

use crate::high_algebra::{AlgebraLevel, offset_rate, multiplier, BASE_RADIX};

/// 寄生配置（脱壳穿衣后生成）
#[derive(Debug, Clone, PartialEq)]
pub struct ParasiteConfig {
    pub kernel_version: String,
    pub host_shell: String,
    pub host_sig: String,
    pub mount_point: String,
    /// [0]=隐藏位(不可见)  [1]=可见位(我)
    pub anchor_map: [String; 2],
    /// 反向规则列表：外壳必须遵守的约定
    pub reverse_rules: Vec<String>,
    /// 寄生虫所在层级 (高维代数偏移)
    pub level: AlgebraLevel,
}

impl ParasiteConfig {
    fn empty() -> Self {
        ParasiteConfig {
            kernel_version: String::new(),
            host_shell: String::from("(未选择)"),
            host_sig: String::new(),
            mount_point: String::new(),
            anchor_map: [String::from(""), String::from("")],
            reverse_rules: Vec::new(),
            level: AlgebraLevel::standard(1),
        }
    }
}

/// 寄生壳引擎
#[derive(Debug, Clone, PartialEq)]
pub struct Parasite {
    pub version: String,
    pub anchors: [String; 2],
    pub active_host: String,
    pub mounted: bool,
    pub tick: u64,
    pub config: ParasiteConfig,
    pub dressed: bool,
}

impl Parasite {
    /// 创建蚕蛹内核
    pub fn new() -> Self {
        Parasite {
            version: String::from("0.2.0"),
            anchors: [String::from("刘楚恬"), String::from("lolict")],
            active_host: String::from("(未寄生)"),
            mounted: false,
            tick: 0,
            config: ParasiteConfig::empty(),
            dressed: false,
        }
    }

    /// 脱壳: 剥离外壳原有身份
    pub fn shed(&mut self) {
        self.config = ParasiteConfig {
            kernel_version: self.version.clone(),
            ..ParasiteConfig::empty()
        };
        self.dressed = false;
    }

    /// 选壳: 挑选要寄生的外壳
    pub fn choose_shell(&mut self, name: &str, sig: &str, compatible: bool) -> Result<(), String> {
        if !compatible {
            return Err(format!("外壳不兼容: {}", name));
        }
        self.config.host_shell = name.to_string();
        self.config.host_sig = sig.to_string();
        self.config.mount_point = format!("/{}/.kernel", name);
        Ok(())
    }

    /// 穿衣: 让外壳披上内核的外观
    ///
    /// 关键: 不是内核改变去适应外壳，
    /// 而是外壳改变来表现内核的特征。
    pub fn dress(&mut self) -> Result<String, String> {
        if self.config.host_shell == "(未选择)" {
            return Err(String::from("请先选壳"));
        }

        let k = &self.version;
        let h = &self.config.host_shell;
        let s = &self.config.host_sig;

        // 生成反向规则: 外壳必须遵守的约定
        self.config.reverse_rules = vec![
            format!("规则1: 所有{}输出必须携带内核版本标记{}", s, k),
            format!("规则2: 外壳的tick计数器必须与内核同步(kernel.tick={})", self.tick),
            String::from("规则3: 外壳的0号锚点映射到隐藏位(不可见)"),
            String::from("规则4: 外壳的1号锚点映射到内核的'我'(可见)"),
            String::from("规则5: 外壳的每次请求都必须经过内核witness验证"),
            String::from("规则6: 外壳不得修改kernel_version字段"),
            String::from("规则7: 外壳字符集不得超过254+2编码范围"),
            format!("规则8: 外壳所在世界层级δ={:.2}不可自行变更", self.config.level.offset_rate),
        ];

        // 设置锚点映射（隐藏根基）
        self.config.anchor_map = [
            String::from("隐藏的0(低位不可见)"),
            String::from("可见的1(我)")
        ];

        // 挂载内核
        self.active_host = h.clone();
        self.mounted = true;
        self.dressed = true;

        let mut result = String::from("脱壳穿衣完成\n");
        result += "────────────────────\n";
        result += &format!("外壳: {}\n", h);
        result += &format!("接口: {}\n", s);
        result += &format!("挂载点: {}\n", self.config.mount_point);
        result += &format!("反向规则数: {}\n", self.config.reverse_rules.len());
        result += &format!("当前层级: {}\n", self.config.level.describe());
        result += "状态: 外壳已被内核同化\n";
        Ok(result)
    }

    /// 内核自检
    pub fn selfcheck(&self) -> String {
        let mut s = String::from("═══ 内核自检 ═══\n");
        s += &format!("版本: {}\n", self.version);
        s += &format!("锚点: [{}, {}]\n", self.anchors[0], self.anchors[1]);
        s += &format!("外壳: {}\n", self.active_host);
        s += &format!("挂载: {}\n", if self.mounted { "是" } else { "否" });
        s += &format!("穿衣: {}\n", if self.dressed { "是" } else { "否" });
        s += &format!("Tick: {}\n", self.tick);
        if self.dressed {
            s += &format!("反向规则数: {}\n", self.config.reverse_rules.len());
            s += &format!("层级偏移: δ={:.2} M={:.2}\n",
                self.config.level.offset_rate, self.config.level.multiplier);
        }
        s
    }

    /// 推进 tick (每个 tick 产生微量功德)
    pub fn tick(&mut self) {
        self.tick += 1;
    }

    /// 让寄生虫迁移到另一个平行世界层级
    pub fn shift_level(&mut self, generation: i32) -> Result<String, String> {
        if !self.dressed {
            return Err(String::from("未穿衣，无法迁移层级"));
        }
        self.config.level = AlgebraLevel::standard(generation);
        Ok(format!("寄生虫已迁移到第{}代偏移世界 (δ={:.2})",
                    generation, self.config.level.offset_rate))
    }

    /// 输出反向规则报告
    pub fn rules_report(&self) -> String {
        if !self.dressed {
            return String::from("(尚未穿衣，无反向规则)");
        }
        let mut s = String::from("═══ 反向规则 ═══\n");
        s += &format!("外壳: {} 必须遵守以下规则:\n\n", self.config.host_shell);
        for (i, rule) in self.config.reverse_rules.iter().enumerate() {
            s += &format!("  [{}] {}\n", i + 1, rule);
        }
        s += "\n含义: 外壳效仿内核，而非内核适应外壳\n";
        s
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_parasite() {
        let p = Parasite::new();
        assert_eq!(p.version, "0.2.0");
        assert_eq!(p.anchors[0], "刘楚恬");
        assert_eq!(p.anchors[1], "lolict");
        assert!(!p.mounted);
    }

    #[test]
    fn test_shed() {
        let mut p = Parasite::new();
        p.shed();
        assert!(!p.dressed);
        assert_eq!(p.config.host_shell, "(未选择)");
    }

    #[test]
    fn test_choose_and_dress() {
        let mut p = Parasite::new();
        p.shed();
        assert!(p.choose_shell("cli-shell", "STDOUT", true).is_ok());
        assert!(p.dress().is_ok());
        assert!(p.dressed);
        assert!(p.mounted);
        assert_eq!(p.active_host, "cli-shell");
        assert_eq!(p.config.reverse_rules.len(), 8);
    }

    #[test]
    fn test_dress_without_shell() {
        let mut p = Parasite::new();
        p.shed();
        assert!(p.dress().is_err());
    }

    #[test]
    fn test_incompatible_shell() {
        let mut p = Parasite::new();
        p.shed();
        assert!(p.choose_shell("unknown-shell", "???", false).is_err());
    }

    #[test]
    fn test_selfcheck() {
        let p = Parasite::new();
        let report = p.selfcheck();
        assert!(report.contains("内核自检"));
        assert!(report.contains("0.2.0"));
    }

    #[test]
    fn test_tick() {
        let mut p = Parasite::new();
        p.tick();
        assert_eq!(p.tick, 1);
        p.tick();
        p.tick();
        assert_eq!(p.tick, 3);
    }

    #[test]
    fn test_shift_level() {
        let mut p = Parasite::new();
        p.shed();
        p.choose_shell("cli-shell", "STDOUT", true).unwrap();
        p.dress().unwrap();
        let result = p.shift_level(3);
        assert!(result.is_ok());
        assert_eq!(p.config.level.generation, 3);
    }

    #[test]
    fn test_shift_without_dress() {
        let mut p = Parasite::new();
        assert!(p.shift_level(3).is_err());
    }

    #[test]
    fn test_rules_report() {
        let mut p = Parasite::new();
        p.shed();
        p.choose_shell("cli-shell", "STDOUT", true).unwrap();
        p.dress().unwrap();
        let report = p.rules_report();
        assert!(report.contains("反向规则"));
        assert!(report.contains("外壳效仿内核"));
    }
}
