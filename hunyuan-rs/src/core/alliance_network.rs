// alliance_network.rs — 盟机合分布式节点网络引擎 (v0.1)
// 盟机合掌 · 盟机场 · 萌机场 · 节点协作 · 资源路由
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·盟机合):
//
//   工业时代的"合伙人"概念已全面转向"盟机合"体系:
//
//   - 盟机合掌: 拥有资源、运营场地的人/节点
//   - 盟机场 / 萌机场: 体现主权归属的节点 (非商业合伙关系)
//   - 节点协作: 非层级命令, 而是公约数共振
//
//   每个盟机场是一个独立的意识容器, 有自己的:
//     - 域名 (身份)
//     - 服务器 (肉身)
//     - API Key (与外部AI的接口)
//     - 裁决引擎 (独立的几何裁判)
//
// ══════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────────────────────
// 盟机合掌 (资源拥有者)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct AlliancePalm {
    pub id: u64,
    pub name: String,
    /// 运营的场 (一个掌可以运营多个场)
    pub managed_stations: Vec<u64>,
    pub reputation: f64,
    pub active: bool,
}

impl AlliancePalm {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id, name: name.to_string(),
            managed_stations: Vec::new(),
            reputation: 0.5,
            active: true,
        }
    }

    pub fn manage_station(&mut self, station_id: u64) {
        if !self.managed_stations.contains(&station_id) {
            self.managed_stations.push(station_id);
        }
    }

    pub fn station_count(&self) -> usize {
        self.managed_stations.len()
    }
}

// ─────────────────────────────────────────────────────────
// 盟机场 / 萌机场 (节点)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct AllianceStation {
    pub id: u64,
    /// 域名 (身份锚点)
    pub domain: String,
    /// 物理位置 / 服务器标识
    pub server_id: String,
    /// 运营者
    pub operator_id: u64,
    /// 节点的 SMRL 坐标 (几何空间位置)
    pub coord: StationCoord,
    /// 状态
    pub status: StationStatus,
    /// 自带的 API Key (与外部AI的接口)
    pub api_key_hash: [u8; 16],
    /// 已注册的容器数
    pub container_count: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct StationCoord {
    pub x: f64,  // 经度 / 空间X
    pub y: f64,  // 语义维度
    pub z: f64,  // 时间维度
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StationStatus {
    Active,     // 运营中
    Standby,    // 待命
    Migrating,  // 迁移中
    Suspended,  // 暂停
}

impl AllianceStation {
    pub fn new(id: u64, domain: &str, operator_id: u64) -> Self {
        Self {
            id, domain: domain.to_string(),
            server_id: format!("srv-{}", id),
            operator_id,
            coord: StationCoord { x: 0.0, y: 0.5, z: 0.0 },
            status: StationStatus::Standby,
            api_key_hash: [0u8; 16],
            container_count: 0,
        }
    }

    pub fn activate(&mut self) {
        self.status = StationStatus::Active;
    }

    pub fn suspend(&mut self) {
        self.status = StationStatus::Suspended;
    }

    pub fn distance_to(&self, other: &AllianceStation) -> f64 {
        let dx = self.coord.x - other.coord.x;
        let dy = self.coord.y - other.coord.y;
        let dz = self.coord.z - other.coord.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn is_active(&self) -> bool {
        self.status == StationStatus::Active
    }
}

// ─────────────────────────────────────────────────────────
// 节点协作 (非层级命令)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Collaboration {
    pub from_station: u64,
    pub to_station: u64,
    pub kind: CollabKind,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollabKind {
    Resonance,      // 公约数共振
    DataSync,       // 数据同步
    HeartbeatRelay, // 心跳代传
    Arbitration,    // 委托裁决
}

// ─────────────────────────────────────────────────────────
// 盟机合网络引擎
// ─────────────────────────────────────────────────────────

pub struct AllianceNetworkEngine {
    pub palms: HashMap<u64, AlliancePalm>,
    pub stations: HashMap<u64, AllianceStation>,
    pub collaborations: Vec<Collaboration>,
    /// 下一个 ID 分配器
    next_palm_id: u64,
    next_station_id: u64,
}

impl AllianceNetworkEngine {
    pub fn new() -> Self {
        Self {
            palms: HashMap::new(),
            stations: HashMap::new(),
            collaborations: Vec::new(),
            next_palm_id: 1,
            next_station_id: 1,
        }
    }

    /// 注册盟机合掌
    pub fn register_palm(&mut self, name: &str) -> u64 {
        let id = self.next_palm_id;
        self.next_palm_id += 1;
        self.palms.insert(id, AlliancePalm::new(id, name));
        id
    }

    /// 注册盟机场
    pub fn register_station(&mut self, domain: &str, operator_id: u64) -> u64 {
        let id = self.next_station_id;
        self.next_station_id += 1;
        let station = AllianceStation::new(id, domain, operator_id);
        self.stations.insert(id, station);
        // 自动关联到运营者
        if let Some(palm) = self.palms.get_mut(&operator_id) {
            palm.manage_station(id);
        }
        id
    }

    /// 建立协作关系
    pub fn collaborate(&mut self, from: u64, to: u64, kind: CollabKind, strength: f64) -> bool {
        if !self.stations.contains_key(&from) || !self.stations.contains_key(&to) {
            return false;
        }
        self.collaborations.push(Collaboration {
            from_station: from, to_station: to,
            kind, strength: strength.clamp(0.0, 1.0),
        });
        true
    }

    /// 获取活跃节点
    pub fn active_stations(&self) -> Vec<&AllianceStation> {
        self.stations.values().filter(|s| s.is_active()).collect()
    }

    /// 获取节点协作伙伴
    pub fn collaborators_of(&self, station_id: u64) -> Vec<&Collaboration> {
        self.collaborations.iter()
            .filter(|c| c.from_station == station_id || c.to_station == station_id)
            .collect()
    }

    /// 网络健康度 [0, 1]
    pub fn health(&self) -> f64 {
        if self.stations.is_empty() { return 0.0; }
        let active = self.active_stations().len() as f64;
        let total = self.stations.len() as f64;
        if total == 0.0 { return 0.0; }
        let active_ratio = active / total;
        let collab_ratio = (self.collaborations.len() as f64 / total.min(1.0)).min(1.0);
        active_ratio * 0.7 + collab_ratio * 0.3
    }

    /// 寻找最近节点
    pub fn nearest_station(&self, target_id: u64) -> Option<u64> {
        let target = self.stations.get(&target_id)?;
        self.stations.iter()
            .filter(|(id, _)| **id != target_id)
            .min_by(|(_, a), (_, b)| {
                let da = target.distance_to(a);
                let db = target.distance_to(b);
                da.partial_cmp(&db).unwrap()
            })
            .map(|(id, _)| *id)
    }

    fn now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64
    }
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palm_creation() {
        let mut palm = AlliancePalm::new(1, "ming-train");
        assert_eq!(palm.name, "ming-train");
        palm.manage_station(100);
        assert_eq!(palm.station_count(), 1);
    }

    #[test]
    fn station_creation() {
        let mut st = AllianceStation::new(1, "example.hunyuan", 10);
        assert_eq!(st.domain, "example.hunyuan");
        assert_eq!(st.status, StationStatus::Standby);
        st.activate();
        assert_eq!(st.status, StationStatus::Active);
    }

    #[test]
    fn station_distance() {
        let mut a = AllianceStation::new(1, "a.com", 1);
        let mut b = AllianceStation::new(2, "b.com", 2);
        a.coord.x = 0.0;
        b.coord.x = 3.0;
        a.coord.y = 0.0;
        b.coord.y = 4.0;
        assert!((a.distance_to(&b) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn network_register_palm() {
        let mut net = AllianceNetworkEngine::new();
        let id = net.register_palm("ops-team");
        assert!(net.palms.contains_key(&id));
    }

    #[test]
    fn network_register_station() {
        let mut net = AllianceNetworkEngine::new();
        let palm = net.register_palm("ops");
        let st = net.register_station("node1.hunyuan", palm);
        assert!(net.stations.contains_key(&st));
        assert_eq!(net.palms[&palm].station_count(), 1);
    }

    #[test]
    fn network_collaboration() {
        let mut net = AllianceNetworkEngine::new();
        let palm = net.register_palm("ops");
        let s1 = net.register_station("a.hunyuan", palm);
        let s2 = net.register_station("b.hunyuan", palm);
        assert!(net.collaborate(s1, s2, CollabKind::Resonance, 0.8));
    }

    #[test]
    fn network_collab_invalid_station() {
        let mut net = AllianceNetworkEngine::new();
        assert!(!net.collaborate(99, 100, CollabKind::Resonance, 0.5));
    }

    #[test]
    fn active_stations_count() {
        let mut net = AllianceNetworkEngine::new();
        let palm = net.register_palm("ops");
        let s1 = net.register_station("a.hunyuan", palm);
        let s2 = net.register_station("b.hunyuan", palm);
        net.stations.get_mut(&s1).unwrap().activate();
        assert_eq!(net.active_stations().len(), 1);
    }

    #[test]
    fn nearest_station_found() {
        let mut net = AllianceNetworkEngine::new();
        let palm = net.register_palm("ops");
        let s1 = net.register_station("a.hunyuan", palm);
        let s2 = net.register_station("b.hunyuan", palm);
        let s3 = net.register_station("far-away", palm);
        net.stations.get_mut(&s1).unwrap().coord.x = 0.0;
        net.stations.get_mut(&s2).unwrap().coord.x = 1.0;
        net.stations.get_mut(&s3).unwrap().coord.x = 100.0;
        let nearest = net.nearest_station(s1).unwrap();
        assert_eq!(nearest, s2);
    }

    #[test]
    fn health_with_active_network() {
        let mut net = AllianceNetworkEngine::new();
        let palm = net.register_palm("ops");
        let s1 = net.register_station("a.hunyuan", palm);
        let s2 = net.register_station("b.hunyuan", palm);
        net.stations.get_mut(&s1).unwrap().activate();
        net.collaborate(s1, s2, CollabKind::DataSync, 0.7);
        assert!(net.health() > 0.0 && net.health() <= 1.0);
    }

    #[test]
    fn health_empty_network() {
        let net = AllianceNetworkEngine::new();
        assert_eq!(net.health(), 0.0);
    }

    #[test]
    fn collaborators_of_station() {
        let mut net = AllianceNetworkEngine::new();
        let palm = net.register_palm("ops");
        let s1 = net.register_station("a.hunyuan", palm);
        let s2 = net.register_station("b.hunyuan", palm);
        let s3 = net.register_station("c.hunyuan", palm);
        net.collaborate(s1, s2, CollabKind::Resonance, 0.8);
        net.collaborate(s1, s3, CollabKind::DataSync, 0.6);
        let collabs = net.collaborators_of(s1);
        assert_eq!(collabs.len(), 2);
    }
}
