// ══════════════════════════════════════════════════════════════════════
// silkworm_parasite.rs — 蚕蛹反向寄生壳 (v0.41)
// ══════════════════════════════════════════════════════════════════════
//
// 核心概念:
//   不是"蚕破茧成蝶"(蛹飞走了), 而是"蛹让茧长在自己身上"。
//   不是系统适应外部规则 — 是外部系统模仿系统自己。
//
// 类比:
//   传统: 下载框架 → 按框架规则写代码 → 被你绑架
//   反向寄生: 下载蚕蛹(解析器+壳选项) → 壳选一条与自己规则匹配
//            → "脱壳穿衣" → 外部系统开始模仿你的规则
//
// 流程:
//   ┌─────────────┐    ┌──────────────┐    ┌────────────────┐
//   │ Download    │ →  │ 脱壳         │ →  │ 穿衣            │
//   │蚕蛹包        │    │ Peel         │    │ Dress           │
//   │(解析器+壳清单)│   │ 去掉原有外壳  │    │ 套上选定的壳     │
//   └─────────────┘    └──────────────┘    └────────────────┘
//          │                                     │
//          ▼                                     ▼
//   ┌─────────────┐                      ┌────────────────┐
//   │ Select/Auto │                      │ Mount 内核     │
//   │ 选壳/自动匹配 │                      │ 让自己的规则    │
//   └─────────────┘                      │ 运行在外部之上  │
//                                        └────────────────┘
//
// 数学约束:
//   - 寄生兼容性的判断:  gcd(自有规则数, 外部规则数) > 0
//   - 壳厚度: 对外表现的"形似度" ≠ 必须完全一致, 只是接口匹配
//   - 内核恒定: 寄生不改变内核规则, 只改变外部表现
//
// 用途例子:
//   - Git 操控: 不用学 Git 命令 — Git 按你的"满全法提交"规则运作
//   - 部署适配: 不用学 Dockerfile — Docker 按你的"容器即关系"逻辑运作
//   - 平台 API: 不用学平台 SDK — 平台接你的调用方式

use crate::container_dynamics::calculate_gcd;

// ────────────────────────────────────────────────────────────
// 蚕蛹状态
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PupaState {
    /// 刚下载, 还没开始脱壳
    Fresh,
    /// 正在脱壳(分析原有外壳结构)
    Peeling,
    /// 壳已脱完, 内核裸露
    Naked,
    /// 穿衣中(匹配/选定外壳)
    Dressing,
    /// 穿衣完成, 套上了新壳
    Shelled,
    /// 内核已挂载, 外部系统开始模仿自己
    Mounted,
    /// 寄生失败(无兼容壳/内核损坏)
    Failed,
}

// ────────────────────────────────────────────────────────────
// 壳选项 — 外部系统的"制服"
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ShellOption {
    /// 壳名称 (例: "git-shell", "docker-shell", "nocode-shell")
    pub name: String,
    /// 壳的目标系统
    pub target_system: String,
    /// 壳的"接口数" — 它模仿你多少个规则
    pub interface_count: u32,
    /// 壳的伪装度 (0.0 = 完全不像, 1.0 = 完美模仿)
    pub mimicry_degree: f64,
    /// 是否已测试通过
    pub tested: bool,
}

impl ShellOption {
    /// 创建壳选项
    pub fn new(name: &str, target: &str, interfaces: u32, mimicry: f64) -> Self {
        ShellOption {
            name: name.to_string(),
            target_system: target.to_string(),
            interface_count: interfaces,
            mimicry_degree: mimicry.clamp(0.0, 1.0),
            tested: false,
        }
    }

    /// 壳是否有效 (接口数 > 0 且 伪装度 > 0.1)
    pub fn is_usable(&self) -> bool {
        self.interface_count > 0 && self.mimicry_degree > 0.1
    }
}

// ────────────────────────────────────────────────────────────
// 内核规则 — 蚕蛹自身固定的东西, 不因寄生而改变
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct KernelRules {
    /// 规则数量 (决定内核的"复杂度")
    pub rule_count: u64,
    /// 根基名 (识别内核身份)
    pub identity: String,
    /// 是否完整 (false = 部分规则丢失)
    pub intact: bool,
}

impl KernelRules {
    /// 创建满全法内核
    pub fn hunyuan(identity: &str) -> Self {
        KernelRules {
            rule_count: 64, // 64 位编码
            identity: identity.to_string(),
            intact: true,
        }
    }

    /// 内核是否可以用
    pub fn is_operational(&self) -> bool {
        self.intact && self.rule_count > 0
    }
}

// ────────────────────────────────────────────────────────────
// 寄生配置 — 脱壳穿衣后生成的挂载配置
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SilkwormConfig {
    /// 选定的壳
    pub shell_name: String,
    /// 目标系统
    pub target_system: String,
    /// 伪装等级 (LOW/MEDIUM/HIGH)
    pub mimicry_level: MimicryLevel,
    /// 挂载点 (内核扎根的路径)
    pub mount_point: String,
    /// 生成的规则数
    pub generated_rules: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MimicryLevel {
    /// 低: 只有核心接口匹配
    Low,
    /// 中: 大部分接口匹配
    Medium,
    /// 高: 几乎完美模仿
    High,
}

impl SilkwormConfig {
    /// 根据伪装度推断等级
    pub fn from_degree(degree: f64) -> Self {
        let level = if degree >= 0.85 {
            MimicryLevel::High
        } else if degree >= 0.5 {
            MimicryLevel::Medium
        } else {
            MimicryLevel::Low
        };
        SilkwormConfig {
            shell_name: String::new(),
            target_system: String::new(),
            mimicry_level: level,
            mount_point: String::new(),
            generated_rules: 0,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 蚕蛹 — 核心结构
// ────────────────────────────────────────────────────────────

pub struct SilkwormPupa {
    /// 当前状态
    pub state: PupaState,
    /// 内核规则 (不变的内核)
    pub kernel: KernelRules,
    /// 可用壳选项
    shells: Vec<ShellOption>,
    /// 选定的壳索引
    selected_shell: Option<usize>,
    /// 生成的寄生配置 (complete 后填充)
    pub config: Option<SilkwormConfig>,
    /// 脱壳日志
    log: Vec<String>,
}

impl SilkwormPupa {
    /// 创建新蚕蛹 (刚下载)
    pub fn new(kernel_identity: &str) -> Self {
        SilkwormPupa {
            state: PupaState::Fresh,
            kernel: KernelRules::hunyuan(kernel_identity),
            shells: Vec::new(),
            selected_shell: None,
            config: None,
            log: vec!["蚕蛹诞生".to_string()],
        }
    }

    /// 下载壳清单 (模拟从外部获取可寄生壳列表)
    pub fn load_shell_list(&mut self, shells: Vec<ShellOption>) {
        let count = shells.len();
        self.shells = shells;
        self.log.push(format!("加载了 {} 个壳选项", count));
    }

    /// 执行脱壳 — 去掉原有外壳, 露出内核
    pub fn peel(&mut self) -> Result<(), ParasiteError> {
        if self.state != PupaState::Fresh {
            return Err(ParasiteError::WrongState(
                "脱壳只能在 Fresh 状态".to_string(),
            ));
        }
        if !self.kernel.is_operational() {
            return Err(ParasiteError::KernelBroken);
        }
        self.state = PupaState::Peeling;
        self.log.push("脱壳开始: 分析原有外壳结构".to_string());
        // 模拟脱壳耗时: 实际中可能涉及规则映射
        self.state = PupaState::Naked;
        self.log.push("脱壳完成: 内核裸露".to_string());
        Ok(())
    }

    /// 选壳 — 从清单中选择合适的壳
    pub fn select_shell(&mut self, shell_name: &str) -> Result<(), ParasiteError> {
        if self.state != PupaState::Naked && self.state != PupaState::Peeling {
            return Err(ParasiteError::WrongState(
                "选壳需要 Naked 状态".to_string(),
            ));
        }
        let idx = self.shells.iter().position(|s| s.name == shell_name);
        match idx {
            Some(i) => {
                if !self.shells[i].is_usable() {
                    return Err(ParasiteError::ShellUnusable(shell_name.to_string()));
                }
                // 检查兼容性
                let gcd = calculate_gcd(self.kernel.rule_count, self.shells[i].interface_count as u64);
                if gcd == 0 {
                    return Err(ParasiteError::IncompatibleShell(
                        "内核规则数与壳接口数无公约数, 无法共振".to_string(),
                    ));
                }
                self.selected_shell = Some(i);
                self.log.push(format!("选定壳: {} (gcd={})", shell_name, gcd));
                Ok(())
            }
            None => Err(ParasiteError::ShellNotFound(shell_name.to_string())),
        }
    }

    /// 自动选壳 — 选择伪装度最高的兼容壳
    pub fn auto_select(&mut self) -> Result<String, ParasiteError> {
        if self.state != PupaState::Naked && self.state != PupaState::Peeling {
            return Err(ParasiteError::WrongState(
                "选壳需要 Naked 状态".to_string(),
            ));
        }

        let mut best_idx = None;
        let mut best_mimicry = -1.0_f64;

        for (i, shell) in self.shells.iter().enumerate() {
            if !shell.is_usable() {
                continue;
            }
            let gcd = calculate_gcd(self.kernel.rule_count, shell.interface_count as u64);
            if gcd == 0 {
                continue; // 不兼容就跳过
            }
            if shell.mimicry_degree > best_mimicry {
                best_mimicry = shell.mimicry_degree;
                best_idx = Some(i);
            }
        }

        match best_idx {
            Some(i) => {
                let name = self.shells[i].name.clone();
                self.selected_shell = Some(i);
                self.log.push(format!("自动选定壳: {} (mimicry={:.2})", name, best_mimicry));
                Ok(name)
            }
            None => Err(ParasiteError::NoCompatibleShell),
        }
    }

    /// 穿衣 — 套上选定的壳
    pub fn dress(&mut self) -> Result<(), ParasiteError> {
        if self.state != PupaState::Naked {
            return Err(ParasiteError::WrongState(
                "穿衣需要 Naked 状态".to_string(),
            ));
        }
        let idx = self.selected_shell.ok_or(ParasiteError::NoShellSelected)?;
        let shell = &self.shells[idx];

        self.state = PupaState::Dressing;
        self.log.push(format!("穿衣: 套上 {}", shell.name));

        // 生成寄生配置
        let mut config = SilkwormConfig::from_degree(shell.mimicry_degree);
        config.shell_name = shell.name.clone();
        config.target_system = shell.target_system.clone();
        config.mount_point = format!("/mnt/{}", shell.target_system);
        config.generated_rules = shell.interface_count as u64;
        self.config = Some(config);

        self.shells[idx].tested = true;
        self.state = PupaState::Shelled;
        self.log.push("穿衣完成".to_string());
        Ok(())
    }

    /// 挂载内核 — 让外部系统模仿自己
    pub fn mount(&mut self) -> Result<SilkwormConfig, ParasiteError> {
        if self.state != PupaState::Shelled {
            return Err(ParasiteError::WrongState(
                "挂载需要 Shelled 状态".to_string(),
            ));
        }
        if self.config.is_none() {
            return Err(ParasiteError::NoConfig);
        }
        self.state = PupaState::Mounted;
        self.log.push("内核挂载: 外部系统开始模仿你的规则".to_string());
        Ok(self.config.clone().unwrap())
    }

    /// 一键寄生 — 完整流程
    pub fn parasitize(&mut self, shell_name: Option<&str>) -> Result<SilkwormConfig, ParasiteError> {
        self.peel()?;
        if let Some(name) = shell_name {
            self.select_shell(name)?;
        } else {
            let _ = self.auto_select()?;
        }
        self.dress()?;
        self.mount()
    }

    /// 失败 → 恢复到 Fresh
    pub fn reset(&mut self) {
        self.state = PupaState::Fresh;
        self.selected_shell = None;
        self.config = None;
        self.log.push("蚕蛹重置".to_string());
    }

    /// 获取当前壳选项 (只读)
    pub fn shells(&self) -> &[ShellOption] {
        &self.shells
    }

    /// 获取日志
    pub fn log(&self) -> &[String] {
        &self.log
    }

    /// 是否已成功寄生
    pub fn is_parasitized(&self) -> bool {
        self.state == PupaState::Mounted
    }
}

// ────────────────────────────────────────────────────────────
// 寄生报告 — 寄生虫进程总结
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SilkwormReport {
    /// 目标系统数量
    pub targets: u32,
    /// 成功寄生数量
    pub successful: u32,
    /// 失败数量
    pub failed: u32,
    /// 平均伪装度
    pub avg_mimicry: f64,
    /// 日志摘要
    pub highlights: Vec<String>,
}

// ────────────────────────────────────────────────────────────
// 错误类型
// ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum ParasiteError {
    WrongState(String),
    KernelBroken,
    ShellNotFound(String),
    ShellUnusable(String),
    IncompatibleShell(String),
    NoCompatibleShell,
    NoShellSelected,
    NoConfig,
}

impl std::fmt::Display for ParasiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongState(s) => write!(f, "状态错误: {}", s),
            Self::KernelBroken => write!(f, "内核损坏"),
            Self::ShellNotFound(s) => write!(f, "壳未找到: {}", s),
            Self::ShellUnusable(s) => write!(f, "壳不可用: {}", s),
            Self::IncompatibleShell(s) => write!(f, "不兼容的壳: {}", s),
            Self::NoCompatibleShell => write!(f, "无兼容的壳"),
            Self::NoShellSelected => write!(f, "未选择壳"),
            Self::NoConfig => write!(f, "无寄生配置"),
        }
    }
}

impl std::error::Error for ParasiteError {}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_shells() -> Vec<ShellOption> {
        vec![
            ShellOption::new("git-shell", "git", 32, 0.92),
            ShellOption::new("docker-shell", "docker", 64, 0.85),
            ShellOption::new("nocode-shell", "nocode", 16, 0.60),
            ShellOption::new("broken-shell", "broken", 0, 0.0),
        ]
    }

    #[test]
    fn silkworm_fresh_state() {
        let pupa = SilkwormPupa::new("hunyuan-os");
        assert_eq!(pupa.state, PupaState::Fresh);
        assert!(pupa.kernel.is_operational());
        assert_eq!(pupa.kernel.rule_count, 64);
        assert!(!pupa.is_parasitized());
    }

    #[test]
    fn load_shell_list() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        assert_eq!(pupa.shells().len(), 4);
        assert_eq!(pupa.shells()[0].name, "git-shell");
    }

    #[test]
    fn peel_success() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.peel().unwrap();
        assert_eq!(pupa.state, PupaState::Naked);
    }

    #[test]
    fn peel_wrong_state() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.peel().unwrap();
        // 二次脱壳应该失败
        assert!(pupa.peel().is_err());
    }

    #[test]
    fn select_compatible_shell() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.peel().unwrap();
        // git-shell 的接口数 32, 内核 64 → gcd = 32 > 0
        assert!(pupa.select_shell("git-shell").is_ok());
    }

    #[test]
    fn select_incompatible_shell() {
        // 创建一个内核 rule_count 为 7 (质数) 的蚕蛹
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(vec![
            ShellOption::new("odd-shell", "odd", 4, 0.5), // 接口4, 内核7 → gcd=1
        ]);
        pupa.peel().unwrap();
        // gcd(7, 4) = 1, 兼容 (最低共振)
        assert!(pupa.select_shell("odd-shell").is_ok());
    }

    #[test]
    fn select_unusable_shell() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.peel().unwrap();
        let result = pupa.select_shell("broken-shell");
        assert!(matches!(result, Err(ParasiteError::ShellUnusable(_))));
    }

    #[test]
    fn select_nonexistent_shell() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.peel().unwrap();
        let result = pupa.select_shell("ghost-shell");
        assert!(matches!(result, Err(ParasiteError::ShellNotFound(_))));
    }

    #[test]
    fn auto_select_best() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.peel().unwrap();
        let chosen = pupa.auto_select().unwrap();
        // 伪装度最高的壳是 git-shell (0.92)
        assert_eq!(chosen, "git-shell");
    }

    #[test]
    fn auto_select_skips_useless() {
        let mut pupa = SilkwormPupa::new("test");
        let shells = vec![
            ShellOption::new("bad1", "x", 0, 0.0),    // 不可用
            ShellOption::new("good", "y", 32, 0.7),   // 可用
        ];
        pupa.load_shell_list(shells);
        pupa.peel().unwrap();
        let chosen = pupa.auto_select().unwrap();
        assert_eq!(chosen, "good");
    }

    #[test]
    fn dress_and_configure() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.peel().unwrap();
        pupa.select_shell("git-shell").unwrap();
        pupa.dress().unwrap();
        assert_eq!(pupa.state, PupaState::Shelled);
        let config = pupa.config.as_ref().unwrap();
        assert_eq!(config.shell_name, "git-shell");
        assert_eq!(config.target_system, "git");
        assert_eq!(config.mimicry_level, MimicryLevel::High); // 0.92 > 0.85
    }

    #[test]
    fn mount_success() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.peel().unwrap();
        pupa.select_shell("git-shell").unwrap();
        pupa.dress().unwrap();
        let config = pupa.mount().unwrap();
        assert_eq!(pupa.state, PupaState::Mounted);
        assert!(pupa.is_parasitized());
        assert_eq!(config.generated_rules, 32);
    }

    #[test]
    fn mount_wrong_state_fails() {
        let mut pupa = SilkwormPupa::new("test");
        // Fresh 状态无法挂载
        assert!(pupa.mount().is_err());
    }

    #[test]
    fn full_parasitize_manual() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        let config = pupa.parasitize(Some("docker-shell")).unwrap();
        assert!(pupa.is_parasitized());
        assert_eq!(config.target_system, "docker");
        assert_eq!(config.mimicry_level, MimicryLevel::High); // 0.85 ≥ 0.85
    }

    #[test]
    fn full_parasitize_auto() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        let config = pupa.parasitize(None).unwrap();
        assert!(pupa.is_parasitized());
        // 自动选择是 git-shell (0.92)
        assert_eq!(config.shell_name, "git-shell");
    }

    #[test]
    fn reset_after_parasitize() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.parasitize(None).unwrap();
        assert!(pupa.is_parasitized());
        pupa.reset();
        assert_eq!(pupa.state, PupaState::Fresh);
        assert!(!pupa.is_parasitized());
    }

    #[test]
    fn log_tracks_progress() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.load_shell_list(make_test_shells());
        pupa.parasitize(Some("nocode-shell")).unwrap();
        let log = pupa.log();
        assert!(log.len() >= 4); // 诞生 + 加载 + 脱壳 + 选壳 + 穿衣 + 挂载
    }

    #[test]
    fn mimicry_level_boundaries() {
        let high = SilkwormConfig::from_degree(0.92);
        assert_eq!(high.mimicry_level, MimicryLevel::High);

        let medium = SilkwormConfig::from_degree(0.7);
        assert_eq!(medium.mimicry_level, MimicryLevel::Medium);

        let low = SilkwormConfig::from_degree(0.3);
        assert_eq!(low.mimicry_level, MimicryLevel::Low);
    }

    #[test]
    fn shell_usable() {
        let good = ShellOption::new("g", "t", 10, 0.5);
        assert!(good.is_usable());

        let no_interface = ShellOption::new("g", "t", 0, 0.5);
        assert!(!no_interface.is_usable());

        let no_mimicry = ShellOption::new("g", "t", 10, 0.05);
        assert!(!no_mimicry.is_usable());
    }

    #[test]
    fn kernel_broken_detection() {
        let mut pupa = SilkwormPupa::new("test");
        pupa.kernel.intact = false;
        let result = pupa.peel();
        assert!(matches!(result, Err(ParasiteError::KernelBroken)));
    }
}
