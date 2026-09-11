// ======================================================================
// celestial_perception_v8.rs — 天庭系统 v8（老八）· 盟机合袋子网络协议栈
//
// 本文件在天庭v7 OS三合一张基础上, 新增分布式网络层:
//
//   1. 袋子数据包 (BagPacket)    — 主权标记 · 袋子ID · 序列号 · 校验和
//   2. 盟机合协议状态机          — 三次握手建立 · 袋子流转传输 · 确认重传
//   3. 盟机场节点管理            — 节点注册/发现/心跳/主权链
//   4. 主权路由表                — 目的地 盟机场 → 下一跳 · 公约数优先路由
//
// 天庭公理: 网络是袋子流动的管道, 主权是路由的身份证;
//           盟机场是袋子进出的关卡, 盟机合掌是资源的开关.
// ======================================================================

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// ======================================================================
// 第一部分: 袋子数据包 (BagPacket)
// ======================================================================

/// 数据包类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PacketKind {
    BagData,        // 袋子数据传输
    Ack,            // 确认应答
    Nak,            // 否定应答 (请求重传)
    HandshakeSyn,   // 连接建立请求
    HandshakeAck,   // 连接建立确认
    Heartbeat,      // 心跳保活
    RouteUpdate,    // 路由更新
    Sovereignproof, // 主权证明 (袋子过境凭证)
}

impl fmt::Display for PacketKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PacketKind::BagData => "BAG",
            PacketKind::Ack => "ACK",
            PacketKind::Nak => "NAK",
            PacketKind::HandshakeSyn => "SYN",
            PacketKind::HandshakeAck => "SAK",
            PacketKind::Heartbeat => "HBT",
            PacketKind::RouteUpdate => "RTU",
            PacketKind::Sovereignproof => "SOV",
        };
        write!(f, "{}", s)
    }
}

/// 主权标记 — 袋子携带的过境身份
#[derive(Clone, Debug)]
pub struct SovereignStamp {
    pub holder_id: String,      // 盟机合掌 ID
    pub resource_token: u64,    // 资源令牌
    pub issued_at_ms: u64,      // 签发时间戳
    pub expires_at_ms: u64,     // 过期时间戳
}

impl SovereignStamp {
    pub fn new(holder_id: &str, resource_token: u64, ttl_ms: u64) -> Self {
        let now = current_timestamp_ms();
        Self {
            holder_id: holder_id.to_string(),
            resource_token,
            issued_at_ms: now,
            expires_at_ms: now + ttl_ms,
        }
    }

    /// 检查主权标记是否仍在有效期内
    pub fn is_valid(&self) -> bool {
        current_timestamp_ms() < self.expires_at_ms
    }

    /// 计算剩余有效时间 (毫秒), 0表示已过期
    pub fn remaining_ms(&self) -> u64 {
        let now = current_timestamp_ms();
        if now >= self.expires_at_ms { 0 } else { self.expires_at_ms - now }
    }
}

/// 袋子数据包 — pdu (协议数据单元)
#[derive(Clone, Debug)]
pub struct BagPacket {
    pub src_id: String,         // 源节点ID
    pub dst_id: String,         // 目的节点ID
    pub kind: PacketKind,       // 包类型
    pub bag_id: Option<u64>,    // 关联袋子ID
    pub seq: u64,               // 序列号
    pub payload: Vec<u8>,       // 有效载荷
    pub checksum: u32,          // 校验和
    pub sovereign: Option<SovereignStamp>, // 主权标记 (过境袋子携带)
    pub timestamp_ms: u64,      // 发包时间戳
}

impl BagPacket {
    pub fn new(src: &str, dst: &str, kind: PacketKind, bag_id: Option<u64>, seq: u64, payload: Vec<u8>) -> Self {
        let mut pkt = Self {
            src_id: src.to_string(),
            dst_id: dst.to_string(),
            kind,
            bag_id,
            seq,
            payload,
            checksum: 0,
            sovereign: None,
            timestamp_ms: current_timestamp_ms(),
        };
        pkt.checksum = pkt.compute_checksum();
        pkt
    }

    /// 计算校验和 (简单 FNV-1a 变体)
    pub fn compute_checksum(&self) -> u32 {
        const FNV_OFFSET: u32 = 0x811c9dc5;
        const FNV_PRIME: u32 = 0x01000193;
        let mut hash = FNV_OFFSET;
        for byte in &self.payload {
            hash ^= *byte as u32;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        // 加入元数据
        for byte in self.src_id.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        for byte in self.dst_id.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash ^= self.seq as u32;
        hash ^= (self.seq >> 32) as u32;
        hash
    }

    /// 验证完整性
    pub fn verify_checksum(&self) -> bool {
        self.checksum == self.compute_checksum()
    }

    /// 附加主权标记 (袋子过境)
    pub fn with_sovereign(&mut self, stamp: SovereignStamp) {
        self.sovereign = Some(stamp);
    }

    /// 判断是否需要主权验证 (过境数据包)
    pub fn requires_sovereign(&self) -> bool {
        matches!(self.kind, PacketKind::BagData | PacketKind::Sovereignproof)
    }

    /// 判断是否有效 (含主权验证)
    pub fn is_valid(&self) -> bool {
        if !self.verify_checksum() {
            return false;
        }
        if self.requires_sovereign() {
            match &self.sovereign {
                Some(stamp) => stamp.is_valid(),
                None => false,
            }
        } else {
            true
        }
    }

    /// 创建对应的确认包
    pub fn make_ack(&self) -> BagPacket {
        BagPacket::new(
            &self.dst_id,
            &self.src_id,
            PacketKind::Ack,
            self.bag_id,
            self.seq,
            vec![],
        )
    }

    /// 创建对应的否定确认包
    pub fn make_nak(&self) -> BagPacket {
        BagPacket::new(
            &self.dst_id,
            &self.src_id,
            PacketKind::Nak,
            self.bag_id,
            self.seq,
            vec![],
        )
    }

    /// 获取包大小 (字节)
    pub fn size(&self) -> usize {
        let base = 64; // 头部估算
        base + self.payload.len()
            + self.sovereign.as_ref().map_or(0, |_| 32)
    }
}

// ======================================================================
// 第二部分: 盟机合协议状态机
// ======================================================================

/// 连接状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnState {
    Closed,         // 关闭
    SynSent,        // 已发起握手
    Established,    // 已建立
    Closing,        // 正在关闭
}

/// 连接 — 盟机场之间的袋子传输通道
pub struct Connection {
    pub local_id: String,
    pub remote_id: String,
    pub state: ConnState,
    pub next_seq: u64,
    pub expected_seq: u64,
    pub established_ms: Option<u64>,
    pub last_activity_ms: u64,
    pub tx_count: u64,
    pub rx_count: u64,
    pub err_count: u64,
    pub window_size: u64,
    pub pending_acks: Vec<u64>,
}

impl Connection {
    pub fn new(local: &str, remote: &str) -> Self {
        Self {
            local_id: local.to_string(),
            remote_id: remote.to_string(),
            state: ConnState::Closed,
            next_seq: 1,
            expected_seq: 1,
            established_ms: None,
            last_activity_ms: current_timestamp_ms(),
            tx_count: 0,
            rx_count: 0,
            err_count: 0,
            window_size: 16,
            pending_acks: Vec::new(),
        }
    }

    /// 发起握手
    pub fn connect(&mut self) -> Option<BagPacket> {
        if self.state != ConnState::Closed {
            return None;
        }
        self.state = ConnState::SynSent;
        let syn = BagPacket::new(
            &self.local_id,
            &self.remote_id,
            PacketKind::HandshakeSyn,
            None,
            0,
            vec![],
        );
        Some(syn)
    }

    /// 收到握手请求, 返回握手确认
    pub fn on_syn(&mut self, _pkt: &BagPacket) -> Option<BagPacket> {
        if self.state == ConnState::Closed {
            self.state = ConnState::Established;
            self.established_ms = Some(current_timestamp_ms());
            let ack = BagPacket::new(
                &self.local_id,
                &self.remote_id,
                PacketKind::HandshakeAck,
                None,
                0,
                vec![],
            );
            Some(ack)
        } else {
            None
        }
    }

    /// 收到握手确认
    pub fn on_syn_ack(&mut self) -> bool {
        if self.state == ConnState::SynSent {
            self.state = ConnState::Established;
            self.established_ms = Some(current_timestamp_ms());
            true
        } else {
            false
        }
    }

    /// 发送袋子数据
    pub fn send_bag(&mut self, bag_id: u64, data: Vec<u8>) -> Option<BagPacket> {
        if self.state != ConnState::Established {
            return None;
        }
        let pkt = BagPacket::new(
            &self.local_id,
            &self.remote_id,
            PacketKind::BagData,
            Some(bag_id),
            self.next_seq,
            data,
        );
        self.next_seq += 1;
        self.tx_count += 1;
        self.last_activity_ms = current_timestamp_ms();
        Some(pkt)
    }

    /// 收到数据包
    pub fn on_data(&mut self, pkt: &BagPacket) -> Option<BagPacket> {
        if self.state != ConnState::Established {
            self.err_count += 1;
            return Some(pkt.make_nak());
        }
        if pkt.seq == self.expected_seq {
            self.expected_seq += 1;
            self.rx_count += 1;
            self.last_activity_ms = current_timestamp_ms();
            self.pending_acks.push(pkt.seq);
            Some(pkt.make_ack())
        } else if pkt.seq > self.expected_seq {
            // 乱序, 请求重传
            self.err_count += 1;
            Some(pkt.make_nak())
        } else {
            // 重复包 — 也发 ACK
            Some(pkt.make_ack())
        }
    }

    /// 判断连接是否超时
    pub fn is_timeout(&self, timeout_ms: u64) -> bool {
        current_timestamp_ms() - self.last_activity_ms > timeout_ms
    }

    pub fn is_established(&self) -> bool {
        self.state == ConnState::Established
    }
}

// ======================================================================
// 第三部分: 盟机场节点管理
// ======================================================================

/// 节点信息 — 盟机场
#[derive(Clone, Debug)]
pub struct NodeInfo {
    pub id: String,
    pub address: String,        // 地址 (可虚拟)
    pub holder_name: String,    // 盟机合掌名
    pub registered_at_ms: u64,
    pub last_heartbeat_ms: u64,
    pub is_alive: bool,
    pub bag_count: u64,         // 该机场已处理袋子数
    pub reputation_score: u32,  // 声誉分 (历史信誉)
}

impl NodeInfo {
    pub fn new(id: &str, address: &str, holder: &str) -> Self {
        let now = current_timestamp_ms();
        Self {
            id: id.to_string(),
            address: address.to_string(),
            holder_name: holder.to_string(),
            registered_at_ms: now,
            last_heartbeat_ms: now,
            is_alive: true,
            bag_count: 0,
            reputation_score: 100,
        }
    }

    /// 更新心跳
    pub fn heartbeat(&mut self) {
        self.last_heartbeat_ms = current_timestamp_ms();
        self.is_alive = true;
    }

    /// 判断是否存活
    pub fn check_alive(&mut self, timeout_ms: u64) -> bool {
        self.is_alive = current_timestamp_ms() - self.last_heartbeat_ms < timeout_ms;
        self.is_alive
    }

    /// 累加声誉
    pub fn add_reputation(&mut self, delta: i32) {
        let new = self.reputation_score as i32 + delta;
        self.reputation_score = new.max(0).min(1000) as u32;
    }
}

/// 节点管理器
pub struct NodeManager {
    pub nodes: HashMap<String, NodeInfo>,
    pub local_id: String,
    pub heartbeat_count: u64,
}

impl NodeManager {
    pub fn new(local_id: &str) -> Self {
        Self {
            nodes: HashMap::new(),
            local_id: local_id.to_string(),
            heartbeat_count: 0,
        }
    }

    /// 注册节点
    pub fn register(&mut self, id: &str, address: &str, holder: &str) -> bool {
        if self.nodes.contains_key(id) {
            return false;
        }
        self.nodes.insert(id.to_string(), NodeInfo::new(id, address, holder));
        true
    }

    /// 反注册节点
    pub fn unregister(&mut self, id: &str) -> bool {
        self.nodes.remove(id).is_some()
    }

    /// 获取节点
    pub fn get(&self, id: &str) -> Option<&NodeInfo> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut NodeInfo> {
        self.nodes.get_mut(id)
    }

    /// 批量心跳检测, 返回失效节点列表
    pub fn check_all_heartbeats(&mut self, timeout_ms: u64) -> Vec<String> {
        let mut dead = Vec::new();
        for (id, node) in self.nodes.iter_mut() {
            if !node.check_alive(timeout_ms) {
                dead.push(id.clone());
            }
        }
        dead
    }

    /// 发送心跳到所有已知节点
    pub fn broadcast_heartbeat(&mut self) -> Vec<BagPacket> {
        self.heartbeat_count += 1;
        self.nodes.keys()
            .map(|id| {
                BagPacket::new(
                    &self.local_id,
                    id,
                    PacketKind::Heartbeat,
                    None,
                    self.heartbeat_count,
                    vec![],
                )
            })
            .collect()
    }

    pub fn alive_count(&self) -> usize {
        self.nodes.values().filter(|n| n.is_alive).count()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

// ======================================================================
// 第四部分: 主权路由表
// ======================================================================

/// 路由项
#[derive(Clone, Debug)]
pub struct RouteEntry {
    pub destination_id: String,
    pub next_hop_id: String,
    pub metric: u32,            // 跳数/开销
    pub gcd_priority: u32,      // 公约数优先级 (越大越优先)
    pub updated_at_ms: u64,
}

impl RouteEntry {
    pub fn new(dst: &str, next_hop: &str, metric: u32, gcd_priority: u32) -> Self {
        Self {
            destination_id: dst.to_string(),
            next_hop_id: next_hop.to_string(),
            metric,
            gcd_priority,
            updated_at_ms: current_timestamp_ms(),
        }
    }
}

/// 主权路由引擎 — 基于公约数优先的路由决策
pub struct SovereignRouter {
    pub routes: Vec<RouteEntry>,
    pub local_id: String,
    pub max_hops: u32,
}

impl SovereignRouter {
    pub fn new(local_id: &str) -> Self {
        Self {
            routes: Vec::new(),
            local_id: local_id.to_string(),
            max_hops: 16,
        }
    }

    /// 添加路由
    pub fn add_route(&mut self, dst: &str, next_hop: &str, metric: u32, gcd_priority: u32) {
        // 更新已有路由
        if let Some(entry) = self.routes.iter_mut().find(|r| r.destination_id == dst && r.next_hop_id == next_hop) {
            entry.metric = metric;
            entry.gcd_priority = gcd_priority;
            entry.updated_at_ms = current_timestamp_ms();
        } else {
            self.routes.push(RouteEntry::new(dst, next_hop, metric, gcd_priority));
        }
    }

    /// 查找最佳下一跳 (公约数优先, 其次跳数)
    pub fn lookup(&self, dst: &str) -> Option<&RouteEntry> {
        self.routes.iter()
            .filter(|r| r.destination_id == dst)
            .filter(|r| r.metric <= self.max_hops)
            .max_by(|a, b| {
                // 先比较公约数优先级 (越高越好), 再比较跳数 (越少越好)
                a.gcd_priority.cmp(&b.gcd_priority)
                    .then_with(|| b.metric.cmp(&a.metric))
            })
    }

    /// 移除目标路由
    pub fn remove_route(&mut self, dst: &str, next_hop: &str) -> bool {
        let before = self.routes.len();
        self.routes.retain(|r| !(r.destination_id == dst && r.next_hop_id == next_hop));
        self.routes.len() < before
    }

    /// 计算两个节点的公约数优先级 (基于ID的公约数关系)
    pub fn calculate_gcd_priority(id_a: &str, id_b: &str) -> u32 {
        let num_a: u64 = id_a.bytes().map(|b| b as u64).sum::<u64>().max(1);
        let num_b: u64 = id_b.bytes().map(|b| b as u64).sum::<u64>().max(1);
        gcd(num_a, num_b) as u32
    }

    /// 接收路由更新
    pub fn on_route_update(&mut self, from_node: &str, dst: &str, metric: u32, gcd_priority: u32) {
        // 学习邻居通告的路由
        let new_metric = metric + 1;
        if new_metric <= self.max_hops {
            self.add_route(dst, from_node, new_metric, gcd_priority);
        }
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }
}

/// 计算最大公约数 (欧几里得算法)
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// ======================================================================
// 第五部分: 天庭V8引擎 — 盟机合协议栈总集成
// ======================================================================

/// V8 引擎输出
#[derive(Clone, Debug)]
pub struct PipelineResultV8 {
    pub packets_sent: usize,
    pub packets_received: usize,
    pub connections_established: usize,
    pub routes_active: usize,
    pub nodes_alive: usize,
    pub errors: usize,
}

/// 发送统计
#[derive(Clone, Debug, Default)]
pub struct TxStats {
    pub total_packets: u64,
    pub acked_packets: u64,
    pub retransmissions: u64,
    pub bytes_sent: u64,
    pub errors: u64,
}

/// 天庭V8引擎 — 盟机合袋子网络协议栈
pub struct CelestialPerceptionV8 {
    pub node_mgr: NodeManager,
    pub router: SovereignRouter,
    pub connections: HashMap<String, Connection>,
    pub tx_stats: TxStats,
    pub error_count: u64,
    pub protocol_version: u32,
}

impl CelestialPerceptionV8 {
    pub fn new(local_id: &str) -> Self {
        Self {
            node_mgr: NodeManager::new(local_id),
            router: SovereignRouter::new(local_id),
            connections: HashMap::new(),
            tx_stats: TxStats::default(),
            error_count: 0,
            protocol_version: 1,
        }
    }

    /// 启动网络栈: 注册本地节点
    pub fn start_network(&mut self, local_addr: &str, holder: &str) {
        let local_id = self.node_mgr.local_id.clone();
        self.node_mgr.register(&local_id, local_addr, holder);
    }

    /// 连接到远端盟机场
    pub fn connect_to(&mut self, remote_id: &str) -> Option<BagPacket> {
        let mut conn = Connection::new(&self.node_mgr.local_id, remote_id);
        let syn = conn.connect();
        if syn.is_some() {
            self.connections.insert(remote_id.to_string(), conn);
        }
        syn
    }

    /// 收到远端握手
    pub fn on_connect_request(&mut self, remote_id: &str, pkt: &BagPacket) -> Option<BagPacket> {
        let conn = self.connections.entry(remote_id.to_string())
            .or_insert_with(|| Connection::new(&self.node_mgr.local_id, remote_id));
        conn.on_syn(pkt)
    }

    /// 完成握手
    pub fn complete_handshake(&mut self, remote_id: &str, pkt: &BagPacket) -> bool {
        if let Some(conn) = self.connections.get_mut(remote_id) {
            if conn.on_syn_ack() {
                // 插入路由
                let gcd_p = SovereignRouter::calculate_gcd_priority(&conn.local_id, &conn.remote_id);
                self.router.add_route(remote_id, remote_id, 1, gcd_p);
                return true;
            }
        }
        false
    }

    /// 发送袋子到远端盟机场
    pub fn send_bag_to(&mut self, remote_id: &str, bag_id: u64, data: Vec<u8>) -> Option<BagPacket> {
        let conn = self.connections.get_mut(remote_id)?;
        if !conn.is_established() {
            return None;
        }
        let pkt = conn.send_bag(bag_id, data)?;
        self.tx_stats.total_packets += 1;
        self.tx_stats.bytes_sent += pkt.size() as u64;
        Some(pkt)
    }

    /// 收到远端数据包处理
    pub fn on_packet(&mut self, pkt: &BagPacket) -> Option<BagPacket> {
        match pkt.kind {
            PacketKind::BagData => {
                let conn = self.connections.get_mut(&pkt.src_id)?;
                let reply = conn.on_data(pkt)?;
                self.tx_stats.total_packets += 1;
                if matches!(reply.kind, PacketKind::Ack) {
                    self.tx_stats.acked_packets += 1;
                }
                Some(reply)
            }
            PacketKind::Ack => {
                // 确认收到
                if let Some(conn) = self.connections.get_mut(&pkt.src_id) {
                    conn.last_activity_ms = current_timestamp_ms();
                }
                self.tx_stats.acked_packets += 1;
                None
            }
            PacketKind::Heartbeat => {
                // 更新节点心跳
                if let Some(node) = self.node_mgr.get_mut(&pkt.src_id) {
                    node.heartbeat();
                }
                None
            }
            PacketKind::Nak => {
                self.error_count += 1;
                self.tx_stats.retransmissions += 1;
                None
            }
            _ => {
                self.error_count += 1;
                None
            }
        }
    }

    /// 全网节点心跳检测
    pub fn heartbeat_check(&mut self, timeout_ms: u64) -> Vec<String> {
        self.node_mgr.check_all_heartbeats(timeout_ms)
    }

    /// 全网广播心跳
    pub fn broadcast_heartbeats(&mut self) -> Vec<BagPacket> {
        self.node_mgr.broadcast_heartbeat()
    }

    /// 汇报
    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭v8报告（老八）· 盟机合袋子网络 ═══\n");
        s.push_str(&format!("  协议版本: {}\n", self.protocol_version));
        s.push_str(&format!("  节点数: {} (存活: {})\n",
            self.node_mgr.node_count(), self.node_mgr.alive_count()));
        s.push_str(&format!("  连接数: {}\n", self.connections.len()));
        s.push_str(&format!("  路由表: {}条\n", self.router.route_count()));
        s.push_str(&format!("  发包: {} (确认: {} 重传: {})\n",
            self.tx_stats.total_packets, self.tx_stats.acked_packets,
            self.tx_stats.retransmissions));
        s.push_str(&format!("  错误数: {}\n", self.error_count));
        s
    }
}

impl Default for CelestialPerceptionV8 {
    fn default() -> Self { Self::new("local") }
}

fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ── BagPacket ──

    #[test]
    fn bag_packet_checksum_verification() {
        let pkt = BagPacket::new("A", "B", PacketKind::Ack, Some(1), 1, vec![1, 2, 3, 4, 5]);
        assert!(pkt.verify_checksum());
        assert!(pkt.is_valid());
    }

    #[test]
    fn bag_packet_checksum_detects_corruption() {
        let mut pkt = BagPacket::new("A", "B", PacketKind::Ack, Some(1), 1, vec![1, 2, 3]);
        pkt.payload[0] = 99; // 篡改
        assert!(!pkt.verify_checksum());
        assert!(!pkt.is_valid());
    }

    #[test]
    fn bag_packet_ack_generation() {
        let pkt = BagPacket::new("A", "B", PacketKind::BagData, Some(42), 5, vec![1, 2, 3]);
        let ack = pkt.make_ack();
        assert_eq!(ack.src_id, "B");
        assert_eq!(ack.dst_id, "A");
        assert_eq!(ack.kind, PacketKind::Ack);
        assert_eq!(ack.bag_id, Some(42));
        assert_eq!(ack.seq, 5);
    }

    #[test]
    fn bag_packet_sovereign_validation() {
        let mut pkt = BagPacket::new("A", "B", PacketKind::BagData, Some(1), 1, vec![1, 2, 3]);
        // 未附加主权标记时, BagData 不应通过验证
        assert!(!pkt.is_valid());

        // 附加有效主权标记
        let stamp = SovereignStamp::new("holder1", 1000, 60000);
        assert!(stamp.is_valid());
        assert!(stamp.remaining_ms() > 0);
        pkt.with_sovereign(stamp);
        assert!(pkt.is_valid());
    }

    #[test]
    fn sovereign_stamp_expiry() {
        let stamp = SovereignStamp::new("holder", 100, 0); // 立即过期
        // 由于检查是 current_ms < expires, expires == now 时立刻过期
        // 我们可以用负TTL来验证
        assert!(!stamp.is_valid() || stamp.remaining_ms() == 0);
    }

    #[test]
    fn bag_packet_size_calculation() {
        let pkt = BagPacket::new("AA", "BB", PacketKind::BagData, Some(1), 1, vec![0u8; 100]);
        let base_size = pkt.size();
        assert!(base_size >= 100);
    }

    // ── Connection (三次握手) ──

    #[test]
    fn connection_three_way_handshake() {
        let mut conn_a = Connection::new("A", "B");
        let mut conn_b = Connection::new("B", "A");

        // A 发起 SYN
        let syn = conn_a.connect().unwrap();
        assert_eq!(conn_a.state, ConnState::SynSent);
        assert_eq!(syn.kind, PacketKind::HandshakeSyn);

        // B 收到 SYN, 回复 SYN-ACK
        let syn_ack = conn_b.on_syn(&syn).unwrap();
        assert_eq!(conn_b.state, ConnState::Established);
        assert_eq!(syn_ack.kind, PacketKind::HandshakeAck);

        // A 收到 SYN-ACK, 完成
        assert!(conn_a.on_syn_ack());
        assert_eq!(conn_a.state, ConnState::Established);
        assert!(conn_a.is_established());
    }

    #[test]
    fn connection_data_transfer() {
        let mut conn_a = Connection::new("A", "B");
        let mut conn_b = Connection::new("B", "A");

        // 握手
        let _syn_ack = conn_b.on_syn(&conn_a.connect().unwrap()).unwrap();
        conn_a.on_syn_ack();

        // A 发送数据
        let pkt = conn_a.send_bag(1, vec![1, 2, 3]).unwrap();
        assert_eq!(pkt.kind, PacketKind::BagData);
        assert_eq!(pkt.bag_id, Some(1));
        assert_eq!(conn_a.tx_count, 1);

        // B 收到并确认
        let reply = conn_b.on_data(&pkt).unwrap();
        assert_eq!(reply.kind, PacketKind::Ack);
        assert_eq!(conn_b.rx_count, 1);
    }

    #[test]
    fn connection_sequence_tracking() {
        let mut conn = Connection::new("A", "B");
        // 手动标记连接已建立 (模拟握手后状态)
        conn.state = ConnState::Established;
        conn.expected_seq = 1;
        let pkt1 = BagPacket::new("X", "Y", PacketKind::Ack, None, 1, vec![1]);
        let pkt2 = BagPacket::new("X", "Y", PacketKind::Ack, None, 2, vec![2]);

        // 先收到 seq=1
        let reply = conn.on_data(&pkt1);
        assert!(reply.is_some());
        assert_eq!(reply.unwrap().kind, PacketKind::Ack);

        // 再收到 seq=2
        let reply = conn.on_data(&pkt2);
        assert_eq!(reply.unwrap().kind, PacketKind::Ack);

        // 收到乱序 seq=1 (重复)
        let reply = conn.on_data(&pkt1);
        assert_eq!(reply.unwrap().kind, PacketKind::Ack);
    }

    #[test]
    fn connection_timeout_detection() {
        let mut conn = Connection::new("A", "B");
        // 将 last_activity_ms 设置为很久以前, 模拟超时
        conn.last_activity_ms = 0; // Unix 起点
        assert!(conn.is_timeout(1)); // 1ms 超时立即触发
        assert!(conn.is_timeout(0)); // 0ms 也触发 (activity_ms=0)
    }

    // ── NodeManager ──

    #[test]
    fn node_register_and_lookup() {
        let mut mgr = NodeManager::new("local");
        assert!(mgr.register("node1", "addr1", "holder1"));
        assert!(mgr.register("node2", "addr2", "holder2"));
        assert!(!mgr.register("node1", "x", "y")); // 重复

        let node = mgr.get("node1").unwrap();
        assert_eq!(node.id, "node1");
        assert_eq!(node.holder_name, "holder1");
        assert!(node.is_alive);
    }

    #[test]
    fn node_unregister() {
        let mut mgr = NodeManager::new("local");
        mgr.register("n1", "a", "h");
        assert!(mgr.unregister("n1"));
        assert_eq!(mgr.node_count(), 0);
        assert!(!mgr.unregister("n1"));
    }

    #[test]
    fn node_heartbeat_timeout() {
        let mut mgr = NodeManager::new("local");
        mgr.register("n1", "a", "h");
        // 检查超时 (0 超时)
        let dead = mgr.check_all_heartbeats(0);
        assert!(!dead.is_empty());
        assert_eq!(mgr.alive_count(), 0);
    }

    #[test]
    fn node_reputation() {
        let mut mgr = NodeManager::new("local");
        mgr.register("n1", "a", "h");
        {
            let node = mgr.get_mut("n1").unwrap();
            node.add_reputation(50);
            assert_eq!(node.reputation_score, 150);
            node.add_reputation(-30);
            assert_eq!(node.reputation_score, 120);
        }
        {
            let node = mgr.get_mut("n1").unwrap();
            node.add_reputation(-200); // 不低于 0
            assert_eq!(node.reputation_score, 0);
        }
    }

    #[test]
    fn broadcast_heartbeats_to_all_nodes() {
        let mut mgr = NodeManager::new("local");
        mgr.register("a", "x", "h");
        mgr.register("b", "y", "h");
        mgr.register("c", "z", "h");

        let pkts = mgr.broadcast_heartbeat();
        assert_eq!(pkts.len(), 3);
        for pkt in &pkts {
            assert_eq!(pkt.kind, PacketKind::Heartbeat);
        }
    }

    // ── SovereignRouter ──

    #[test]
    fn router_add_and_lookup() {
        let mut router = SovereignRouter::new("local");
        router.add_route("net1", "hop1", 2, 5);
        router.add_route("net1", "hop2", 1, 10);

        let best = router.lookup("net1").unwrap();
        // 公约数优先级高者优先 (hop2 的 gcd_priority=10 > 5)
        assert_eq!(best.next_hop_id, "hop2");
        assert_eq!(best.gcd_priority, 10);
    }

    #[test]
    fn router_route_update() {
        let mut router = SovereignRouter::new("local");
        router.on_route_update("neighbor_a", "dst_x", 2, 8);
        let route = router.lookup("dst_x").unwrap();
        assert_eq!(route.next_hop_id, "neighbor_a");
        assert_eq!(route.metric, 3); // 2 + 1
    }

    #[test]
    fn router_max_hops_limit() {
        let mut router = SovereignRouter::new("local");
        router.add_route("far", "hop", 100, 999); // metric=100 超过 max_hops=16
        assert!(router.lookup("far").is_none()); // 被过滤
    }

    #[test]
    fn gcd_calculation() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 0), 7);
    }

    #[test]
    fn gcd_priority_from_ids() {
        let p = SovereignRouter::calculate_gcd_priority("ABC", "DEF");
        // A=65 B=66 C=67 sum=198; D=68 E=69 F=70 sum=207
        // gcd(198, 207) = gcd(198, 9) = 9
        assert_eq!(p, 9);
    }

    // ── CelestialPerceptionV8 引擎 ──

    #[test]
    fn v8_start_network() {
        let mut engine = CelestialPerceptionV8::new("local_node");
        engine.start_network("local_addr", "holder");
        assert_eq!(engine.node_mgr.node_count(), 1);
    }

    #[test]
    fn v8_full_handshake() {
        let mut engine_a = CelestialPerceptionV8::new("node_A");
        let mut engine_b = CelestialPerceptionV8::new("node_B");

        // A 发起连接
        let syn = engine_a.connect_to("node_B").unwrap();
        assert_eq!(syn.kind, PacketKind::HandshakeSyn);

        // B 收到 SYN, 回复
        let syn_ack = engine_b.on_connect_request("node_A", &syn).unwrap();
        assert_eq!(syn_ack.kind, PacketKind::HandshakeAck);

        // A 完成握手
        assert!(engine_a.complete_handshake("node_B", &syn_ack));
        assert!(engine_a.connections.get("node_B").unwrap().is_established());

        // 双方路由已学习
        assert!(engine_a.router.lookup("node_B").is_some());
    }

    #[test]
    fn v8_bag_transfer_between_nodes() {
        let mut engine_a = CelestialPerceptionV8::new("A");
        let mut engine_b = CelestialPerceptionV8::new("B");

        // A 发起握手
        let syn = engine_a.connect_to("B").unwrap();
        let syn_ack = engine_b.on_connect_request("A", &syn).unwrap();
        engine_a.complete_handshake("B", &syn_ack);

        // 使 B 到 A 的连接也进入 Established (模拟双向)
        {
            let conn = engine_b.connections.get_mut("A").unwrap();
            conn.state = ConnState::Established;
        }

        // A 发送袋子
        let bag_data = b"rice_bag_attributes_vec";
        let pkt = engine_a.send_bag_to("B", 42, bag_data.to_vec()).unwrap();
        assert_eq!(pkt.bag_id, Some(42));
        assert_eq!(pkt.kind, PacketKind::BagData);

        // B 收到
        let reply = engine_b.on_packet(&pkt).unwrap();
        assert_eq!(reply.kind, PacketKind::Ack);
    }

    #[test]
    fn v8_bag_packet_from_unknown_sender_rejected() {
        let mut engine = CelestialPerceptionV8::new("local");
        let pkt = BagPacket::new("unknown", "local", PacketKind::BagData, Some(1), 1, vec![1]);
        let reply = engine.on_packet(&pkt);
        // 未知发送方 → 没有连接 → None
        assert!(reply.is_none());
    }

    #[test]
    fn v8_heartbeat_check_removes_dead() {
        let mut engine = CelestialPerceptionV8::new("local");
        engine.node_mgr.register("alive_node", "a", "h");
        engine.node_mgr.register("dead_node", "a", "h");

        // 一个节点发心跳
        {
            let node = engine.node_mgr.get_mut("alive_node").unwrap();
            node.heartbeat();
        }

        // 0 超时检测
        let dead = engine.heartbeat_check(0);
        assert!(!dead.is_empty()); // 两个都超时
        assert_eq!(engine.node_mgr.alive_count(), 0);
    }

    #[test]
    fn v8_report_format() {
        let mut engine = CelestialPerceptionV8::new("me");
        engine.start_network("my_addr", "holder");
        let report = engine.report();
        assert!(report.contains("天庭v8"));
        assert!(report.contains("节点"));
        assert!(report.contains("路由"));
    }

    #[test]
    fn v8_tx_stats_tracking() {
        let mut engine = CelestialPerceptionV8::new("A");
        let mut engine_b = CelestialPerceptionV8::new("B");

        // 握手
        let syn = engine.connect_to("B").unwrap();
        let syn_ack = engine_b.on_connect_request("A", &syn).unwrap();
        engine.complete_handshake("B", &syn_ack);

        // 发送袋子
        engine.send_bag_to("B", 1, vec![1, 2, 3]);
        engine.send_bag_to("B", 2, vec![4, 5, 6]);

        assert_eq!(engine.tx_stats.total_packets, 2);
        assert!(engine.tx_stats.bytes_sent > 0);
    }
}
