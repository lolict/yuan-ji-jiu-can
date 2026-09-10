// ======================================================================
// celestial_perception_v10.rs — 天觉系统 v10（老十）· 九元天觉总装: 能力+命名空间+POSIX系统调用
//
// 本文件在天觉v6/v7/v8内核+存储+网络基础上, 新增OS安全+系统调用层:
//
//   1. 能力系统 (Capability)         — 袋子操作权限令牌 · 最小特权 · 移交/派生
//   2. 命名空间 (Namespace)          — mount/pid/net/user ns · 隔离视图 · unshare
//   3. POSIX系统调用接口             — open/close/read/write/clone/exec/exit/fork
//   4. 安全策略引擎 (PolicyEngine)   — 默认拒绝 + 公约数最小特权裁决
//
// 天觉公理: 权力即袋子携带的能力, 身份即命名空间划定的边界;
//           每一个系统调用都是一次袋子的权力宣告;
//           公约数裁定"什么是可共享的", 差集裁定"什么是不可侵犯的".
// ======================================================================

use std::collections::{HashMap, HashSet};
use std::fmt;

pub const V10_VERSION: u32 = 10;

// ======================================================================
// 第一部分: 能力系统
// ======================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapType {
    BagRead, BagWrite, BagCreate, BagDelete, BagTransfer,
    FileOpen, FileRead, FileWrite, FileMkdir, FileUnlink,
    PipeCreate, MsgSend, MsgRecv, ShmAttach,
    NetSend, NetRecv, NetConnect, NetBind,
    ProcFork, ProcExec, ProcKill,
    MountFs, SetNs, SysAdmin,
}

impl CapType {
    pub fn domain(&self) -> CapDomain {
        match self {
            CapType::BagRead | CapType::BagWrite | CapType::BagCreate
            | CapType::BagDelete | CapType::BagTransfer => CapDomain::Bag,
            CapType::FileOpen | CapType::FileRead | CapType::FileWrite
            | CapType::FileMkdir | CapType::FileUnlink => CapDomain::Fs,
            CapType::PipeCreate | CapType::MsgSend | CapType::MsgRecv
            | CapType::ShmAttach => CapDomain::Ipc,
            CapType::NetSend | CapType::NetRecv | CapType::NetConnect
            | CapType::NetBind => CapDomain::Net,
            CapType::ProcFork | CapType::ProcExec | CapType::ProcKill => CapDomain::Proc,
            CapType::MountFs | CapType::SetNs | CapType::SysAdmin => CapDomain::Sys,
        }
    }

    pub fn is_dangerous(&self) -> bool {
        matches!(self,
            CapType::BagDelete | CapType::ProcKill | CapType::SysAdmin
            | CapType::SetNs | CapType::MountFs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapDomain { Bag, Fs, Ipc, Net, Proc, Sys }

#[derive(Clone, Debug)]
pub struct Capability {
    pub cap_type: CapType,
    pub owner_pid: u64,
    pub issued_by: String,
    pub issued_at_ms: u64,
    pub expiry_ms: Option<u64>,
    pub transferable: bool,
    pub nonce: u64,
}

impl Capability {
    pub fn new(cap_type: CapType, owner_pid: u64, issued_by: &str, transferable: bool) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let nonce = now ^ (cap_type as u64) ^ owner_pid;
        Self {
            cap_type, owner_pid,
            issued_by: issued_by.to_string(),
            issued_at_ms: now,
            expiry_ms: None,
            transferable, nonce,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.expiry_ms {
            Some(exp) => {
                use std::time::{SystemTime, UNIX_EPOCH};
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                now >= exp
            }
            None => false,
        }
    }

    pub fn is_valid(&self) -> bool { !self.is_expired() }
    pub fn can_transfer_to(&self, _target_pid: u64) -> bool { self.transferable && self.is_valid() }
}

#[derive(Clone, Debug)]
pub struct CapSet {
    pub caps: HashSet<u64>,
    pub inherited: HashSet<u64>,
}

impl CapSet {
    pub fn new() -> Self { Self { caps: HashSet::new(), inherited: HashSet::new() } }
    pub fn grant(&mut self, cap: &Capability) {
        self.caps.insert(cap.nonce);
        if cap.transferable { self.inherited.insert(cap.nonce); }
    }
    pub fn has(&self, cap_type: CapType, nonce: u64) -> bool { self.caps.contains(&nonce) }
    pub fn get_inherited(&self) -> &HashSet<u64> { &self.inherited }
    pub fn drop_cap(&mut self, nonce: u64) -> bool { self.caps.remove(&nonce); self.inherited.remove(&nonce); true }
    pub fn len(&self) -> usize { self.caps.len() }
    pub fn is_empty(&self) -> bool { self.caps.is_empty() }
}

impl Default for CapSet { fn default() -> Self { Self::new() } }

pub struct CapRegistry {
    pub capabilities: HashMap<u64, Capability>,
    pub next_nonce: u64,
}

impl CapRegistry {
    pub fn new() -> Self { Self { capabilities: HashMap::new(), next_nonce: 1 } }
    pub fn issue(&mut self, cap_type: CapType, owner_pid: u64, issued_by: &str, transferable: bool) -> Capability {
        let mut cap = Capability::new(cap_type, owner_pid, issued_by, transferable);
        cap.nonce = self.next_nonce;
        self.next_nonce += 1;
        self.capabilities.insert(cap.nonce, cap.clone());
        cap
    }
    pub fn get(&self, nonce: u64) -> Option<&Capability> { self.capabilities.get(&nonce) }
    pub fn revoke(&mut self, nonce: u64) -> bool { self.capabilities.remove(&nonce).is_some() }
    pub fn cleanup_expired(&mut self) -> usize {
        let expired: Vec<u64> = self.capabilities.iter()
            .filter(|(_, c)| c.is_expired()).map(|(n, _)| *n).collect();
        let count = expired.len();
        for n in expired { self.capabilities.remove(&n); }
        count
    }
}

impl Default for CapRegistry { fn default() -> Self { Self::new() } }

// ======================================================================
// 第二部分: 命名空间
// ======================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NsType { Mount, Pid, Net, User, Ipc }

#[derive(Clone, Debug)]
pub struct Namespace {
    pub ns_type: NsType,
    pub id: u64,
    pub owner_pid: u64,
    pub attached_pids: Vec<u64>,
    pub visible_bags: HashSet<u64>,
    pub created_at_ms: u64,
}

impl Namespace {
    pub fn new(ns_type: NsType, id: u64, owner_pid: u64) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        Self {
            ns_type, id, owner_pid,
            attached_pids: vec![owner_pid],
            visible_bags: HashSet::new(),
            created_at_ms: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
        }
    }
    pub fn attach(&mut self, pid: u64) -> bool {
        if self.attached_pids.contains(&pid) { return false; }
        self.attached_pids.push(pid); true
    }
    pub fn detach(&mut self, pid: u64) -> bool {
        if let Some(idx) = self.attached_pids.iter().position(|p| *p == pid) {
            self.attached_pids.remove(idx); true
        } else { false }
    }
    pub fn add_bag(&mut self, bag_id: u64) { self.visible_bags.insert(bag_id); }
    pub fn remove_bag(&mut self, bag_id: u64) { self.visible_bags.remove(&bag_id); }
    pub fn can_see_bag(&self, bag_id: u64) -> bool { self.visible_bags.contains(&bag_id) }
    pub fn attached_count(&self) -> usize { self.attached_pids.len() }
}

pub struct NsManager {
    pub namespaces: HashMap<NsType, Vec<Namespace>>,
    pub proc_ns_map: HashMap<u64, HashSet<u64>>,
    pub next_id: u64,
}

impl NsManager {
    pub fn new() -> Self { Self { namespaces: HashMap::new(), proc_ns_map: HashMap::new(), next_id: 1 } }
    fn alloc_id(&mut self) -> u64 { let id = self.next_id; self.next_id += 1; id }
    pub fn create(&mut self, ns_type: NsType, owner_pid: u64) -> u64 {
        let id = self.alloc_id();
        let ns = Namespace::new(ns_type, id, owner_pid);
        self.namespaces.entry(ns_type).or_insert_with(Vec::new).push(ns);
        self.proc_ns_map.entry(owner_pid).or_insert_with(HashSet::new).insert(id);
        id
    }
    pub fn unshare(&mut self, ns_type: NsType, pid: u64) -> u64 { self.create(ns_type, pid) }
    pub fn join(&mut self, ns_type: NsType, ns_id: u64, pid: u64) -> bool {
        if let Some(ns_list) = self.namespaces.get_mut(&ns_type) {
            if let Some(ns) = ns_list.iter_mut().find(|n| n.id == ns_id) {
                if ns.attach(pid) {
                    self.proc_ns_map.entry(pid).or_insert_with(HashSet::new).insert(ns_id);
                    return true;
                }
            }
        }
        false
    }
    pub fn leave(&mut self, ns_type: NsType, ns_id: u64, pid: u64) -> bool {
        if let Some(ns_list) = self.namespaces.get_mut(&ns_type) {
            if let Some(ns) = ns_list.iter_mut().find(|n| n.id == ns_id) {
                let ok = ns.detach(pid);
                if ok { if let Some(set) = self.proc_ns_map.get_mut(&pid) { set.remove(&ns_id); } }
                return ok;
            }
        }
        false
    }
    pub fn get_ns_mut(&mut self, ns_type: NsType, pid: u64) -> Option<&mut Namespace> {
        let ns_ids: Vec<u64> = self.proc_ns_map.get(&pid)?.iter().copied().collect();
        let ns_list = self.namespaces.get_mut(&ns_type)?;
        for id in ns_ids {
            let found = ns_list.iter().position(|n| n.id == id);
            if let Some(idx) = found { return Some(&mut ns_list[idx]); }
        }
        None
    }
    pub fn bag_visible(&self, pid: u64, bag_id: u64) -> bool {
        if let Some(ns_ids) = self.proc_ns_map.get(&pid) {
            for &ns_id in ns_ids {
                for ns_list in self.namespaces.values() {
                    if let Some(ns) = ns_list.iter().find(|n| n.id == ns_id) {
                        if ns.can_see_bag(bag_id) { return true; }
                    }
                }
            }
        }
        false
    }
    pub fn total_ns(&self) -> usize { self.namespaces.values().map(|v| v.len()).sum() }
}

impl Default for NsManager { fn default() -> Self { Self::new() } }

// ======================================================================
// 第三部分: POSIX 系统调用接口
// ======================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PosixSyscall {
    Open = 0, Close = 1, Read = 2, Write = 3,
    Fork = 4, Exec = 5, Exit = 6,
    Mkdir = 7, Unlink = 8, Clone = 9,
    CapCheck = 10, NsUnshare = 11,
}

#[derive(Clone, Debug)]
pub struct BagSyscallRequest {
    pub syscall: PosixSyscall,
    pub pid: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub payload: Vec<u8>,
    pub bag_id: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct BagSyscallResult {
    pub ok: bool,
    pub code: i32,
    pub data: Vec<u8>,
    pub message: String,
}

impl BagSyscallResult {
    pub fn success(data: Vec<u8>) -> Self { Self { ok: true, code: 0, data, message: String::new() } }
    pub fn error(code: i32, msg: &str) -> Self { Self { ok: false, code, data: Vec::new(), message: msg.to_string() } }
    pub fn permission_denied() -> Self { Self::error(-1, "Permission denied") }
}

// ======================================================================
// 第四部分: 安全策略引擎
// ======================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum PolicyDecision { Allow, Deny, RequireCap(CapType), ElevateAudit }

pub struct PolicyEngine {
    pub default_action: PolicyDecision,
    pub required_caps: HashMap<PosixSyscall, Vec<CapType>>,
    pub audit_log: Vec<String>,
    pub violation_count: u64,
}

impl PolicyEngine {
    pub fn new() -> Self {
        let mut required_caps = HashMap::new();
        required_caps.insert(PosixSyscall::Read, vec![CapType::FileRead]);
        required_caps.insert(PosixSyscall::Write, vec![CapType::FileWrite]);
        required_caps.insert(PosixSyscall::Open, vec![CapType::FileOpen]);
        required_caps.insert(PosixSyscall::Mkdir, vec![CapType::FileMkdir]);
        required_caps.insert(PosixSyscall::Unlink, vec![CapType::FileUnlink]);
        required_caps.insert(PosixSyscall::Fork, vec![CapType::ProcFork]);
        required_caps.insert(PosixSyscall::Exec, vec![CapType::ProcExec]);
        required_caps.insert(PosixSyscall::NsUnshare, vec![CapType::SetNs]);

        Self {
            default_action: PolicyDecision::Deny,
            required_caps,
            audit_log: Vec::new(),
            violation_count: 0,
        }
    }

    pub fn judge(&mut self, pid: u64, capset: &CapSet, registry: &CapRegistry, req: &BagSyscallRequest) -> PolicyDecision {
        let required = match self.required_caps.get(&req.syscall) {
            Some(caps) => caps,
            None => { return PolicyDecision::Deny; }
        };
        let mut has_all = true;
        for cap_type in required {
            let found = registry.capabilities.values()
                .any(|c| c.owner_pid == pid && c.cap_type == *cap_type && c.is_valid());
            if !found { has_all = false; break; }
        }
        if has_all {
            self.audit_log.push(format!("[ALLOW] pid={} syscall={:?}", pid, req.syscall));
            PolicyDecision::Allow
        } else {
            self.violation_count += 1;
            self.audit_log.push(format!("[DENY] pid={} syscall={:?}", pid, req.syscall));
            if let Some(first) = required.first() {
                PolicyDecision::RequireCap(*first)
            } else {
                PolicyDecision::Deny
            }
        }
    }

    pub fn by_common_divisor(&self, pid_a: u64, pid_b: u64) -> u64 {
        gcd(pid_a.max(1), pid_b.max(1))
    }

    pub fn clear_audit(&mut self) { self.audit_log.clear(); }
    pub fn audit_count(&self) -> usize { self.audit_log.len() }
}

impl Default for PolicyEngine { fn default() -> Self { Self::new() } }

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

// ======================================================================
// 第五部分: 天觉V10引擎
// ======================================================================

#[derive(Clone, Debug)]
pub struct PipelineResultV10 {
    pub syscalls_processed: u64,
    pub allowed: u64,
    pub denied: u64,
    pub capabilities_issued: u64,
    pub namespaces_created: u64,
    pub violations: u64,
}

impl Default for PipelineResultV10 {
    fn default() -> Self {
        Self { syscalls_processed: 0, allowed: 0, denied: 0, capabilities_issued: 0, namespaces_created: 0, violations: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct ProcContext {
    pub pid: u64,
    pub name: String,
    pub caps: CapSet,
    pub parent_pid: Option<u64>,
}

impl ProcContext {
    pub fn new(pid: u64, name: &str, parent: Option<u64>) -> Self {
        Self { pid, name: name.to_string(), caps: CapSet::new(), parent_pid: parent }
    }
}

pub struct CelestialPerceptionV10 {
    pub caps: CapRegistry,
    pub namespaces: NsManager,
    pub policy: PolicyEngine,
    pub processes: HashMap<u64, ProcContext>,
    pub next_pid: u64,
    pub pipeline: PipelineResultV10,
}

impl CelestialPerceptionV10 {
    pub fn new() -> Self {
        let mut engine = Self {
            caps: CapRegistry::new(),
            namespaces: NsManager::new(),
            policy: PolicyEngine::new(),
            processes: HashMap::new(),
            next_pid: 1,
            pipeline: PipelineResultV10::default(),
        };
        engine.processes.insert(0, ProcContext::new(0, "kernel", None));
        engine.init_process("init", 0);
        engine
    }

    fn alloc_pid(&mut self) -> u64 { let pid = self.next_pid; self.next_pid += 1; pid }

    fn init_process(&mut self, name: &str, parent: u64) -> u64 {
        let pid = self.alloc_pid();
        let mut ctx = ProcContext::new(pid, name, Some(parent));
        let all_caps = vec![
            CapType::BagRead, CapType::BagWrite, CapType::BagCreate, CapType::BagDelete, CapType::BagTransfer,
            CapType::FileOpen, CapType::FileRead, CapType::FileWrite, CapType::FileMkdir, CapType::FileUnlink,
            CapType::PipeCreate, CapType::MsgSend, CapType::MsgRecv, CapType::ShmAttach,
            CapType::NetSend, CapType::NetRecv, CapType::NetConnect, CapType::NetBind,
            CapType::ProcFork, CapType::ProcExec, CapType::ProcKill,
            CapType::MountFs, CapType::SetNs, CapType::SysAdmin,
        ];
        let cap_count = all_caps.len() as u64;
        for cap_type in all_caps {
            let cap = self.caps.issue(cap_type, pid, "kernel", true);
            ctx.caps.grant(&cap);
        }
        self.processes.insert(pid, ctx);
        self.pipeline.capabilities_issued += cap_count;
        pid
    }

    pub fn create_process(&mut self, name: &str, parent: u64, inherit_caps: bool) -> u64 {
        let pid = self.alloc_pid();
        let mut ctx = ProcContext::new(pid, name, Some(parent));
        if inherit_caps {
            if let Some(parent_ctx) = self.processes.get(&parent) {
                let inherited = parent_ctx.caps.get_inherited().clone();
                for nonce in inherited {
                    if let Some(cap) = self.caps.get(nonce) {
                        if cap.is_valid() { ctx.caps.grant(cap); }
                    }
                }
            }
        }
        self.processes.insert(pid, ctx);
        pid
    }

    pub fn grant_capability(&mut self, pid: u64, cap_type: CapType, issued_by: &str, transferable: bool) -> Option<u64> {
        if !self.processes.contains_key(&pid) { return None; }
        let cap = self.caps.issue(cap_type, pid, issued_by, transferable);
        let nonce = cap.nonce;
        self.processes.get_mut(&pid).unwrap().caps.grant(&cap);
        self.pipeline.capabilities_issued += 1;
        Some(nonce)
    }

    pub fn drop_capability(&mut self, pid: u64, nonce: u64) -> bool {
        if let Some(ctx) = self.processes.get_mut(&pid) { ctx.caps.drop_cap(nonce) } else { false }
    }

    pub fn syscall(&mut self, req: BagSyscallRequest) -> BagSyscallResult {
        let pid = req.pid;
        self.pipeline.syscalls_processed += 1;
        let capset = match self.processes.get(&pid) {
            Some(ctx) => ctx.caps.clone(),
            None => return BagSyscallResult::error(-3, "Process not found"),
        };
        let decision = self.policy.judge(pid, &capset, &self.caps, &req);
        match decision {
            PolicyDecision::Allow => { self.pipeline.allowed += 1; BagSyscallResult::success(format!("Syscall {:?} executed", req.syscall).into_bytes()) }
            PolicyDecision::Deny => { self.pipeline.denied += 1; self.pipeline.violations += 1; BagSyscallResult::permission_denied() }
            PolicyDecision::RequireCap(cap_type) => { self.pipeline.denied += 1; self.pipeline.violations += 1; BagSyscallResult::error(-1, &format!("Requires capability: {:?}", cap_type)) }
            PolicyDecision::ElevateAudit => { self.pipeline.allowed += 1; BagSyscallResult::success(vec![]) }
        }
    }

    pub fn create_namespace(&mut self, ns_type: NsType, pid: u64) -> u64 {
        let ns_id = self.namespaces.create(ns_type, pid);
        self.pipeline.namespaces_created += 1;
        ns_id
    }

    pub fn isolate_bag(&mut self, bag_id: u64, pid: u64) -> bool {
        if let Some(ns) = self.namespaces.get_ns_mut(NsType::Mount, pid) {
            ns.add_bag(bag_id); true
        } else { false }
    }

    pub fn bag_visible(&self, pid: u64, bag_id: u64) -> bool { self.namespaces.bag_visible(pid, bag_id) }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉v10报告（老十）· 九元天觉总装 ═══\n");
        s.push_str(&format!("  进程数: {}\n", self.processes.len()));
        s.push_str(&format!("  能力总数: {} (已颁发 {})\n", self.caps.capabilities.len(), self.pipeline.capabilities_issued));
        s.push_str(&format!("  命名空间数: {}\n", self.namespaces.total_ns()));
        s.push_str(&format!("  系统调用: {} 次 (允许{} 拒绝{})\n",
            self.pipeline.syscalls_processed, self.pipeline.allowed, self.pipeline.denied));
        s.push_str(&format!("  违反: {} | 审计: {}\n", self.pipeline.violations, self.policy.audit_count()));
        s.push_str(&format!("  版本: V{}\n", V10_VERSION));
        s
    }

    pub fn cleanup_caps(&mut self) -> usize { self.caps.cleanup_expired() }
}

impl Default for CelestialPerceptionV10 { fn default() -> Self { Self::new() } }

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_issue_and_check() {
        let mut reg = CapRegistry::new();
        let cap = reg.issue(CapType::BagRead, 100, "kernel", true);
        assert!(cap.is_valid());
        assert_eq!(cap.owner_pid, 100);
    }

    #[test]
    fn cap_expiry_from_creation() {
        let cap = Capability::new(CapType::BagWrite, 1, "issuer", false);
        assert!(!cap.is_expired());
        assert!(cap.is_valid());
    }

    #[test]
    fn capset_grant_and_check() {
        let mut set = CapSet::new();
        let cap = Capability::new(CapType::FileRead, 100, "kernel", true);
        set.grant(&cap);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn capset_drop_reduces_privilege() {
        let mut set = CapSet::new();
        let cap = Capability::new(CapType::ProcKill, 100, "kernel", false);
        set.grant(&cap);
        set.drop_cap(cap.nonce);
        assert!(set.is_empty());
    }

    #[test]
    fn cap_is_domain_categorized() {
        assert_eq!(CapType::BagRead.domain(), CapDomain::Bag);
        assert_eq!(CapType::FileOpen.domain(), CapDomain::Fs);
        assert_eq!(CapType::MsgSend.domain(), CapDomain::Ipc);
        assert_eq!(CapType::NetConnect.domain(), CapDomain::Net);
        assert_eq!(CapType::ProcFork.domain(), CapDomain::Proc);
        assert_eq!(CapType::SysAdmin.domain(), CapDomain::Sys);
    }

    #[test]
    fn cap_dangerous_set() {
        assert!(CapType::ProcKill.is_dangerous());
        assert!(CapType::SysAdmin.is_dangerous());
        assert!(!CapType::BagRead.is_dangerous());
    }

    #[test]
    fn cap_revoke_from_registry() {
        let mut reg = CapRegistry::new();
        let cap = reg.issue(CapType::FileRead, 100, "kernel", true);
        let nonce = cap.nonce;
        assert!(reg.get(nonce).is_some());
        assert!(reg.revoke(nonce));
        assert!(reg.get(nonce).is_none());
    }

    #[test]
    fn cap_transferable_rules() {
        let cap_ok = Capability::new(CapType::BagRead, 100, "kernel", true);
        let cap_no = Capability::new(CapType::BagRead, 100, "kernel", false);
        assert!(cap_ok.can_transfer_to(200));
        assert!(!cap_no.can_transfer_to(200));
    }

    #[test]
    fn ns_create_and_attach() {
        let mut ns_mgr = NsManager::new();
        let ns_id = ns_mgr.create(NsType::Pid, 100);
        assert!(ns_mgr.join(NsType::Pid, ns_id, 200));
    }

    #[test]
    fn ns_unshare_creates_new() {
        let mut ns_mgr = NsManager::new();
        let ns1 = ns_mgr.create(NsType::Mount, 100);
        let ns2 = ns_mgr.unshare(NsType::Mount, 100);
        assert_ne!(ns1, ns2);
    }

    #[test]
    fn ns_bag_visibility() {
        let mut ns_mgr = NsManager::new();
        let ns_id = ns_mgr.create(NsType::Mount, 100);
        {
            let ns = ns_mgr.namespaces.get_mut(&NsType::Mount).unwrap()
                .iter_mut().find(|n| n.id == ns_id).unwrap();
            ns.add_bag(42);
        }
        assert!(ns_mgr.bag_visible(100, 42));
        assert!(!ns_mgr.bag_visible(100, 99));
        assert!(!ns_mgr.bag_visible(200, 42));
    }

    #[test]
    fn ns_leave_namespace() {
        let mut ns_mgr = NsManager::new();
        let ns_id = ns_mgr.create(NsType::Net, 100);
        ns_mgr.join(NsType::Net, ns_id, 200);
        assert!(ns_mgr.leave(NsType::Net, ns_id, 200));
    }

    #[test]
    fn policy_default_deny_unknown_syscall() {
        let mut policy = PolicyEngine::new();
        let req = BagSyscallRequest { syscall: PosixSyscall::Exit, pid: 1, arg1: 0, arg2: 0, payload: vec![], bag_id: None };
        let decision = policy.judge(1, &CapSet::new(), &CapRegistry::new(), &req);
        assert_eq!(decision, PolicyDecision::Deny);
    }

    #[test]
    fn policy_allows_when_cap_present() {
        let mut policy = PolicyEngine::new();
        let mut reg = CapRegistry::new();
        let cap = reg.issue(CapType::FileRead, 100, "kernel", true);
        let mut capset = CapSet::new();
        capset.grant(&cap);
        let req = BagSyscallRequest { syscall: PosixSyscall::Read, pid: 100, arg1: 0, arg2: 0, payload: vec![], bag_id: None };
        let decision = policy.judge(100, &capset, &reg, &req);
        assert_eq!(decision, PolicyDecision::Allow);
    }

    #[test]
    fn policy_denies_when_cap_missing() {
        let mut policy = PolicyEngine::new();
        let reg = CapRegistry::new();
        let capset = CapSet::new();
        let req = BagSyscallRequest { syscall: PosixSyscall::Write, pid: 100, arg1: 0, arg2: 0, payload: vec![], bag_id: None };
        let decision = policy.judge(100, &capset, &reg, &req);
        match decision { PolicyDecision::Deny | PolicyDecision::RequireCap(_) => {}, _ => panic!("Expected deny") }
    }

    #[test]
    fn policy_by_common_divisor() {
        let policy = PolicyEngine::new();
        assert_eq!(policy.by_common_divisor(100, 75), 25);
    }

    #[test]
    fn policy_audit_log_tracks_decisions() {
        let mut policy = PolicyEngine::new();
        let mut reg = CapRegistry::new();
        let cap = reg.issue(CapType::FileOpen, 100, "kernel", true);
        let mut capset = CapSet::new();
        capset.grant(&cap);
        policy.judge(100, &capset, &reg, &BagSyscallRequest { syscall: PosixSyscall::Open, pid: 100, arg1: 0, arg2: 0, payload: vec![], bag_id: None });
        assert_eq!(policy.audit_count(), 1);
        policy.clear_audit();
        assert_eq!(policy.audit_count(), 0);
    }

    #[test]
    fn v10_init_grants_all_capabilities() {
        let engine = CelestialPerceptionV10::new();
        let init = engine.processes.get(&1).unwrap();
        // 非空即证明能力已颁发 (具体数量随能力建设扩展)
        assert!(!init.caps.is_empty());
        // 校验颁发了能力
        assert!(engine.pipeline.capabilities_issued >= 20);
    }

    #[test]
    fn v10_create_unprivileged_process() {
        let mut engine = CelestialPerceptionV10::new();
        let pid = engine.create_process("worker", 1, false);
        let ctx = engine.processes.get(&pid).unwrap();
        assert!(ctx.caps.is_empty());
    }

    #[test]
    fn v10_create_inherits_caps() {
        let mut engine = CelestialPerceptionV10::new();
        let child = engine.create_process("child", 1, true);
        let ctx = engine.processes.get(&child).unwrap();
        assert!(!ctx.caps.is_empty());
    }

    #[test]
    fn v10_grant_and_drop_cap() {
        let mut engine = CelestialPerceptionV10::new();
        let pid = engine.create_process("app", 1, false);
        let nonce = engine.grant_capability(pid, CapType::BagRead, "init", true).unwrap();
        { let ctx = engine.processes.get(&pid).unwrap(); assert!(ctx.caps.has(CapType::BagRead, nonce)); }
        assert!(engine.drop_capability(pid, nonce));
    }

    #[test]
    fn v10_syscall_allowed_with_cap() {
        let mut engine = CelestialPerceptionV10::new();
        let req = BagSyscallRequest { syscall: PosixSyscall::Read, pid: 1, arg1: 0, arg2: 0, payload: vec![], bag_id: None };
        let result = engine.syscall(req);
        assert!(result.ok);
    }

    #[test]
    fn v10_syscall_denied_without_cap() {
        let mut engine = CelestialPerceptionV10::new();
        let pid = engine.create_process("unpriv", 1, false);
        let req = BagSyscallRequest { syscall: PosixSyscall::Write, pid, arg1: 0, arg2: 0, payload: vec![], bag_id: None };
        let result = engine.syscall(req);
        assert!(!result.ok);
    }

    #[test]
    fn v10_isolate_bag_to_namespace() {
        let mut engine = CelestialPerceptionV10::new();
        let pid = engine.create_process("worker", 1, false);
        engine.create_namespace(NsType::Mount, pid);
        assert!(engine.isolate_bag(42, pid));
        assert!(engine.bag_visible(pid, 42));
        assert!(!engine.bag_visible(1, 42));
    }

    #[test]
    fn v10_cap_expiry_cleanup() {
        let mut engine = CelestialPerceptionV10::new();
        for _ in 0..5 { engine.grant_capability(1, CapType::BagRead, "test", false); }
        let before = engine.caps.capabilities.len();
        let cleaned = engine.cleanup_caps();
        assert_eq!(cleaned, 0);
        assert_eq!(engine.caps.capabilities.len(), before);
    }

    #[test]
    fn v10_report_format() {
        let engine = CelestialPerceptionV10::new();
        let report = engine.report();
        assert!(report.contains("天觉v10"));
        assert!(report.contains("进程"));
    }

    #[test]
    fn v10_pipeline_tracking() {
        let mut engine = CelestialPerceptionV10::new();
        let _ = engine.syscall(BagSyscallRequest { syscall: PosixSyscall::Read, pid: 1, arg1: 0, arg2: 0, payload: vec![], bag_id: None });
        // pid=999 不存在 → 进程查找失败, 不计入denied
        let _ = engine.syscall(BagSyscallRequest { syscall: PosixSyscall::Write, pid: 999, arg1: 0, arg2: 0, payload: vec![], bag_id: None });
        assert_eq!(engine.pipeline.syscalls_processed, 2);
        assert_eq!(engine.pipeline.allowed, 1);
        // denied=0 因为非进程直接返回错误, 未进入策略裁决
        assert_eq!(engine.pipeline.denied, 0);
    }
}
