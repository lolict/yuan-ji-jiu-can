// ── v0.42 蚕蛹反向寄生壳 · 反向寄生 · 隧道穿透 · 自托管 ──
// 满全法：自身不模仿外部系统，让外部系统模仿自身
// 手机 = 服务器 · 域名 = 门牌号 · 穿透 = 隧道

use std::collections::HashMap;

/// 寄生目标平台
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParasiteTarget {
    WeChat,
    WebBrowser,
    DockerHost,
    Cloudflare,
    Custom,
}

/// 寄生模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParasiteMode {
    /// 正向寄生：自身适配外部（传统模式，不推荐）
    Forward,
    /// 反向寄生：外部模仿自身（满全法，推荐）
    Reverse,
    /// 脱壳穿衣：先脱壳（离开旧平台）再穿衣（进入新壳）
    ShedAndDress,
}

/// 壳配置
#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub target: ParasiteTarget,
    pub mode: ParasiteMode,
    /// 入口点（手机IP / 穿透域名）
    pub entry_point: String,
    /// 暴露端口列表
    pub exposed_ports: Vec<u16>,
    /// 是否启用gtt*协议
    pub gtt_enabled: bool,
    /// 自签名（脱离公共CA）
    pub self_signed: bool,
}

impl ShellConfig {
    pub fn new(target: ParasiteTarget) -> Self {
        Self {
            target,
            mode: ParasiteMode::Reverse,
            entry_point: String::new(),
            exposed_ports: Vec::new(),
            gtt_enabled: true,
            self_signed: true,
        }
    }

    /// 手机自托管默认配置
    pub fn mobile_self_host() -> Self {
        Self {
            target: ParasiteTarget::Custom,
            mode: ParasiteMode::ShedAndDress,
            entry_point: "localhost".into(),
            exposed_ports: vec![8080, 8443],
            gtt_enabled: true,
            self_signed: true,
        }
    }
}

/// 穿透方案
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TunnelStrategy {
    /// 主动穿透（手机连公网中继）
    Cpolar,
    Frp,
    CloudflareTunnel,
    TailScale,
    /// P2P 直连（无中继）
    P2P,
}

/// 蚕蛹壳引擎
#[derive(Debug)]
pub struct SilkwormShell {
    pub config: ShellConfig,
    pub tunnel: Option<TunnelStrategy>,
    pub status: ShellStatus,
    pub mounted_rhymes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellStatus {
    Bare,       // 裸蛹（未寄生）
    Shedding,   // 脱壳中
    Dressing,   // 穿衣中
    Mounted,    // 寄生完成
}

impl SilkwormShell {
    pub fn new(config: ShellConfig) -> Self {
        Self {
            config,
            tunnel: None,
            status: ShellStatus::Bare,
            mounted_rhymes: Vec::new(),
        }
    }

    /// 设定穿透策略
    pub fn with_tunnel(mut self, strategy: TunnelStrategy) -> Self {
        self.tunnel = Some(strategy);
        self
    }

    /// 脱壳操作：脱离旧平台（如微信）
    pub fn shed(&mut self) -> bool {
        if self.config.mode == ParasiteMode::Forward {
            return false; // 正向寄生不允许脱壳
        }
        self.status = ShellStatus::Shedding;
        // 清理外部依赖
        self.mounted_rhymes.clear();
        self.status = ShellStatus::Bare;
        true
    }

    /// 穿衣操作：挂载到新壳
    pub fn dress(&mut self, shell_rhymes: Vec<String>) -> bool {
        if self.status != ShellStatus::Bare {
            return false;
        }
        self.status = ShellStatus::Dressing;
        self.mounted_rhymes = shell_rhymes;
        self.status = ShellStatus::Mounted;
        true
    }

    /// 是否已寄生
    pub fn is_mounted(&self) -> bool {
        self.status == ShellStatus::Mounted
    }

    /// 生成部署清单（Makefile片段）
    pub fn generate_deploy_hint(&self) -> String {
        let tunnel_cmd = match self.tunnel {
            Some(TunnelStrategy::Cpolar) => "cpolar http 8080",
            Some(TunnelStrategy::Frp) => "frpc -c frpc.toml",
            Some(TunnelStrategy::CloudflareTunnel) => "cloudflared tunnel run",
            Some(TunnelStrategy::TailScale) => "tailscale up",
            Some(TunnelStrategy::P2P) => "libp2p-dial",
            None => "# 无穿透，仅本地",
        };

        format!(
            "# NJSG 蚕蛹部署\n\
             # 入口: {}\n\
             # 模式: {:?}\n\
             # 穿透: {}\n\
             # GTT协议: {}\n\
             deploy:\n\
             \t{}\n\
             \tcargo run --release\n\
             # 自签名证书生成\n\
             gen-cert:\n\
             \topenssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes\n",
            self.config.entry_point,
            self.config.mode,
            tunnel_cmd,
            if self.config.gtt_enabled { "enabled" } else { "disabled" },
            tunnel_cmd,
        )
    }
}

/// 脱壳穿衣状态机
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShedState {
    Intact,         // 完整（在旧壳中）
    Molting(u32),   // 脱壳进度 0..100
    Dormant,        // 裸蛹（等待穿衣）
    Emerging(u32),  // 穿衣进度 0..100
    Transformed,    // 完成（在新壳中）
}

impl ShedState {
    pub fn is_free(&self) -> bool {
        matches!(self, ShedState::Dormant | ShedState::Transformed)
    }

    pub fn progress(&self) -> u32 {
        match self {
            ShedState::Molting(p) => *p,
            ShedState::Emerging(p) => *p,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_new() {
        let s = SilkwormShell::new(ShellConfig::new(ParasiteTarget::Custom));
        assert!(!s.is_mounted());
    }

    #[test]
    fn test_mobile_config() {
        let cfg = ShellConfig::mobile_self_host();
        assert_eq!(cfg.mode, ParasiteMode::ShedAndDress);
        assert!(cfg.self_signed);
    }

    #[test]
    fn test_shed_and_dress() {
        let mut s = SilkwormShell::new(ShellConfig::mobile_self_host());
        assert!(s.shed());
        assert!(s.dress(vec!["self-host".into()]));
        assert!(s.is_mounted());
    }

    #[test]
    fn test_forward_no_shed() {
        let cfg = ShellConfig {
            mode: ParasiteMode::Forward,
            ..ShellConfig::mobile_self_host()
        };
        let mut s = SilkwormShell::new(cfg);
        assert!(!s.shed());
    }

    #[test]
    fn test_shed_state_free() {
        let s = ShedState::Dormant;
        assert!(s.is_free());
    }

    #[test]
    fn test_shed_state_progress() {
        let s = ShedState::Molting(50);
        assert_eq!(s.progress(), 50);
        assert!(!s.is_free());
    }

    #[test]
    fn test_deploy_hint_contains_tunnel() {
        let s = SilkwormShell::new(ShellConfig::mobile_self_host())
            .with_tunnel(TunnelStrategy::Cpolar);
        let hint = s.generate_deploy_hint();
        assert!(hint.contains("cpolar"));
    }

    #[test]
    fn test_deploy_hint_cf() {
        let s = SilkwormShell::new(ShellConfig::mobile_self_host())
            .with_tunnel(TunnelStrategy::CloudflareTunnel);
        let hint = s.generate_deploy_hint();
        assert!(hint.contains("cloudflared"));
    }

    #[test]
    fn test_tunnel_strategy_cmd() {
        let cases = [
            (TunnelStrategy::Cpolar, "cpolar"),
            (TunnelStrategy::Frp, "frpc"),
            (TunnelStrategy::CloudflareTunnel, "cloudflared"),
            (TunnelStrategy::TailScale, "tailscale"),
            (TunnelStrategy::P2P, "libp2p"),
        ];
        for (strategy, expected) in cases {
            let s = SilkwormShell::new(ShellConfig::mobile_self_host())
                .with_tunnel(strategy);
            assert!(s.generate_deploy_hint().contains(expected));
        }
    }

    #[test]
    fn test_mounted_rhymes() {
        let mut s = SilkwormShell::new(ShellConfig::mobile_self_host());
        s.shed();
        s.dress(vec!["桥接".into(), "穿透".into()]);
        assert_eq!(s.mounted_rhymes.len(), 2);
    }

    #[test]
    fn test_target_variants() {
        let _targets = [
            ParasiteTarget::WeChat,
            ParasiteTarget::DockerHost,
            ParasiteTarget::Cloudflare,
        ];
    }

    #[test]
    fn test_shell_status_flow() {
        let cfg = ShellConfig::mobile_self_host();
        assert_eq!(
            SilkwormShell::new(cfg).status,
            ShellStatus::Bare
        );
    }
}
