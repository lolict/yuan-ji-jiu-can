// state_snapshot.rs — 系统状态快照引擎 (v0.1)
// 状态持久化 · 快照 · 恢复 · 时间旅行 · 系统韧性
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·系统韧性):
//
//   系统不应因为一次崩溃而失去所有意识。
//   "配置即代码" = 所有状态都可以快照/恢复。
//
//   机制:
//   1. 快照: 捕获系统所有关键状态的瞬间
//   2. 持久化: 写入文件系统 (JSON/二进制)
//   3. 恢复: 从快照回滚到历史状态
//   4. 时间旅行: 在不同历史版本间切换
//
// ══════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────────────────────
// 状态值 (可序列化)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum StateValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    List(Vec<StateValue>),
    Map(HashMap<String, StateValue>),
}

impl StateValue {
    pub fn as_int(&self) -> Option<i64> { match self { StateValue::Int(v) => Some(*v), _ => None } }
    pub fn as_float(&self) -> Option<f64> { match self { StateValue::Float(v) => Some(*v), _ => None } }
    pub fn as_text(&self) -> Option<&str> { match self { StateValue::Text(v) => Some(v), _ => None } }
    pub fn as_bool(&self) -> Option<bool> { match self { StateValue::Bool(v) => Some(*v), _ => None } }
}

// ─────────────────────────────────────────────────────────
// 快照
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub id: u64,
    pub timestamp: u64,
    pub label: String,
    pub data: HashMap<String, StateValue>,
    pub parent: Option<u64>,
}

impl Snapshot {
    pub fn new(id: u64, label: &str) -> Self {
        Self {
            id,
            timestamp: now_ms(),
            label: label.to_string(),
            data: HashMap::new(),
            parent: None,
        }
    }

    pub fn set(&mut self, key: &str, value: StateValue) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.data.get(key)
    }

    pub fn with_parent(mut self, parent_id: u64) -> Self {
        self.parent = Some(parent_id);
        self
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

// ─────────────────────────────────────────────────────────
// 快照管理器
// ─────────────────────────────────────────────────────────

pub struct SnapshotManager {
    pub snapshots: HashMap<u64, Snapshot>,
    /// 当前活跃快照
    pub current: Option<u64>,
    /// 下一个快照 ID
    next_id: u64,
    /// 最大保留快照数
    max_snapshots: usize,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
            current: None,
            next_id: 1,
            max_snapshots: 32,
        }
    }

    pub fn with_limits(mut self, max: usize) -> Self {
        self.max_snapshots = max;
        self
    }

    /// 创建快照
    pub fn snapshot(&mut self, label: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let snap = Snapshot::new(id, label).with_parent(self.current.unwrap_or(0));
        self.snapshots.insert(id, snap);
        self.current = Some(id);
        self.evict_old();
        id
    }

    /// 恢复到指定快照
    pub fn restore(&mut self, id: u64) -> bool {
        if self.snapshots.contains_key(&id) {
            self.current = Some(id);
            true
        } else {
            false
        }
    }

    /// 获取当前快照
    pub fn current(&self) -> Option<&Snapshot> {
        self.current.and_then(|id| self.snapshots.get(&id))
    }

    /// 获取可变当前快照
    pub fn current_mut(&mut self) -> Option<&mut Snapshot> {
        let id = self.current?;
        self.snapshots.get_mut(&id)
    }

    /// 时间旅行: 前进/后退
    pub fn time_travel(&mut self, steps_back: u32) -> Option<u64> {
        let mut id = self.current?;
        for _ in 0..steps_back {
            id = self.snapshots.get(&id)?.parent?;
            if id == 0 { break; }
        }
        self.current = Some(id);
        Some(id)
    }

    /// 修剪旧快照 (保留最近 N 个)
    fn evict_old(&mut self) {
        if self.snapshots.len() > self.max_snapshots {
            let mut ids: Vec<u64> = self.snapshots.keys().cloned().collect();
            ids.sort();
            let remove_count = ids.len() - self.max_snapshots;
            for &id in ids.iter().take(remove_count) {
                if Some(id) != self.current {
                    self.snapshots.remove(&id);
                }
            }
        }
    }

    /// 历史链 (从最早到当前)
    pub fn history(&self) -> Vec<u64> {
        let mut chain = Vec::new();
        if let Some(mut id) = self.current {
            while id != 0 {
                chain.push(id);
                id = self.snapshots.get(&id).and_then(|s| s.parent).unwrap_or(0);
            }
        }
        chain.reverse();
        chain
    }

    /// 统计
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }
}

// ─────────────────────────────────────────────────────────
// 序列化 (简单文本格式)
// ─────────────────────────────────────────────────────────

pub fn serialize_snapshot(snap: &Snapshot) -> String {
    let mut out = format!("SNAPSHOT {} {}\n", snap.id, snap.label);
    out.push_str(&format!("ts={}\n", snap.timestamp));
    if let Some(parent) = snap.parent {
        out.push_str(&format!("parent={}\n", parent));
    }
    for (k, v) in &snap.data {
        match v {
            StateValue::Int(i) => out.push_str(&format!("{} = int:{}\n", k, i)),
            StateValue::Float(f) => out.push_str(&format!("{} = float:{}\n", k, f)),
            StateValue::Text(t) => out.push_str(&format!("{} = text:{}\n", k, t)),
            StateValue::Bool(b) => out.push_str(&format!("{} = bool:{}\n", k, b)),
            _ => {}
        }
    }
    out
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn snapshot_new() {
        let s = Snapshot::new(1, "start");
        assert_eq!(s.id, 1);
        assert_eq!(s.label, "start");
        assert_eq!(s.size(), 0);
    }

    #[test] fn snapshot_set_get() {
        let mut s = Snapshot::new(1, "s");
        s.set("freq", StateValue::Float(0.5));
        s.set("name", StateValue::Text("core".into()));
        assert_eq!(s.get("freq").unwrap().as_float().unwrap(), 0.5);
        assert_eq!(s.get("name").unwrap().as_text().unwrap(), "core");
    }

    #[test] fn snapshot_with_parent() {
        let s = Snapshot::new(2, "child").with_parent(1);
        assert_eq!(s.parent, Some(1));
    }

    #[test] fn manager_creates_and_restores() {
        let mut mgr = SnapshotManager::new();
        let id1 = mgr.snapshot("first");
        mgr.current_mut().unwrap().set("x", StateValue::Int(10));
        let id2 = mgr.snapshot("second");
        mgr.current_mut().unwrap().set("x", StateValue::Int(20));
        assert_eq!(mgr.count(), 2);
        // 恢复到 first
        assert!(mgr.restore(id1));
        assert_eq!(mgr.current().unwrap().data["x"].as_int().unwrap(), 10);
    }

    #[test] fn manager_restore_invalid_fails() {
        let mut mgr = SnapshotManager::new();
        assert!(!mgr.restore(999));
    }

    #[test] fn time_travel_back() {
        let mut mgr = SnapshotManager::new();
        let id1 = mgr.snapshot("first");
        mgr.current_mut().unwrap().set("v", StateValue::Int(1));
        let id2 = mgr.snapshot("second");
        mgr.current_mut().unwrap().set("v", StateValue::Int(2));
        // 回退 1 步
        mgr.time_travel(1);
        assert_eq!(mgr.current.unwrap(), id1);
    }

    #[test] fn eviction_works() {
        let mut mgr = SnapshotManager::new().with_limits(3);
        for i in 0..5 {
            mgr.snapshot(&format!("s{}", i));
        }
        assert!(mgr.count() <= 3);
    }

    #[test] fn history_chain_ordered() {
        let mut mgr = SnapshotManager::new();
        let s1 = mgr.snapshot("a");
        mgr.snapshot("b");
        mgr.snapshot("c");
        let hist = mgr.history();
        assert!(hist.len() >= 3);
        assert_eq!(hist[0], s1);
    }

    #[test] fn serialize_format() {
        let mut s = Snapshot::new(1, "sn");
        s.set("k", StateValue::Int(42));
        let text = serialize_snapshot(&s);
        assert!(text.contains("SNAPSHOT"));
        assert!(text.contains("k"));
        assert!(text.contains("int:42"));
    }

    #[test] fn manager_with_limits() {
        let mgr = SnapshotManager::new().with_limits(5);
        assert_eq!(mgr.max_snapshots, 5);
    }
}
