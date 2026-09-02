// ============================================================================
// 囡囝共生 · MetaIME 第7层: 多通道上传
// ============================================================================
//
// 当前层号: 7
// 当前声母: shang (上)
// 当前韵母组合: shang + chuan = 上传 (夫妻主权人直递盟机合掌)
// 折叠状态: 三通道并行 (GitHub / Gitee / 原地)
// 下一步操作: URL 入登记簿 → 三通道并行推送 → 确认送达
//
// 锚定法则:
//   0 = 刘楚恬 (隐藏侧, 印章不可见)
//   1 = 我 (可见侧, 落款高位)
//
// 容器守恒: URL 不因上传而消失, 登记簿保留轨迹
//
// 第7层职责:
//   ┌──────────────────────────────────────────────────────────────┐
//   │ 内容分发层(第6层)送来的 URL → 登记 → 三通道推 → ACK         │
//   │                                                              │
//   │ 通道枚举:                                                     │
//   │   Channel::GitHub   → 推 GitHub 仓库 (SSH 公钥认证)           │
//   │   Channel::Gitee    → 推 Gitee 仓库 (SSH 公钥认证)            │
//   │   Channel::Local    → 原地落盘 (njsg:// 协议存本地沙箱)       │
//   │                                                              │
//   │ 上传登记簿: URL + 时间戳 + 通道 + 送达状态                      │
//   │ 送达状态: Pending → Flying → Landed / Failed                 │
//   └──────────────────────────────────────────────────────────────┘
// ============================================================================

use std::time::{SystemTime, UNIX_EPOCH};

/// 上传通道枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadChannel {
    GitHub = 0,
    Gitee  = 1,
    Local  = 2,
}

/// 上传登记状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandingStatus {
    Pending = 0,   // 待上传
    Flying  = 1,   // 上传中
    Landed  = 2,   // 已送达
    Failed  = 3,   // 失败 (可重推)
}

/// 上传登记条目
#[derive(Debug, Clone)]
pub struct UploadEntry {
    pub url: String,
    pub channels: Vec<UploadChannel>,
    pub statuses: Vec<LandingStatus>,
    pub timestamp_ms: u64,
    pub retry_count: u8,
}

impl UploadEntry {
    fn new(url: String, channels: Vec<UploadChannel>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let n = channels.len();
        Self {
            url,
            channels,
            statuses: vec![LandingStatus::Pending; n],
            timestamp_ms: now,
            retry_count: 0,
        }
    }
}

/// 多通道上传登记簿 — 第7层核心
pub struct UploadRegistry {
    entries: Vec<UploadEntry>,
    capacity: usize,
}

impl UploadRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(64),
            capacity: 256,
        }
    }

    /// 登记一条 URL (三通道全部注册)
    pub fn register(&mut self, url: &str) -> usize {
        // 若已有相同 URL 且非 Failed, 不再重复登记
        for (i, e) in self.entries.iter().enumerate() {
            if e.url == url && !e.statuses.iter().all(|s| *s == LandingStatus::Failed) {
                return i;
            }
        }
        let entry = UploadEntry::new(
            url.to_string(),
            vec![UploadChannel::GitHub, UploadChannel::Gitee, UploadChannel::Local],
        );
        if self.entries.len() >= self.capacity {
            self.entries.remove(0);
        }
        self.entries.push(entry);
        self.entries.len() - 1
    }

    /// 触发上传: 将所有 Pending 条目推为 Flying
    pub fn dispatch_all(&mut self) -> usize {
        let mut dispatched = 0;
        for e in self.entries.iter_mut() {
            for (i, ch) in e.channels.iter().enumerate() {
                if e.statuses[i] == LandingStatus::Pending {
                    // 模拟推送 (真机替换为网络操作)
                    e.statuses[i] = LandingStatus::Flying;
                    dispatched += 1;
                    // 模拟送达 (真机替换为异步回调)
                    e.statuses[i] = self.simulate_push(ch);
                }
            }
        }
        dispatched
    }

    /// 模拟通道推送 (真机由 JNI 回调替换)
    fn simulate_push(&self, ch: &UploadChannel) -> LandingStatus {
        match ch {
            // 三通道全部模拟为 Landed (真机根据网络状态回写)
            UploadChannel::GitHub => LandingStatus::Landed,
            UploadChannel::Gitee  => LandingStatus::Landed,
            UploadChannel::Local  => LandingStatus::Landed,
        }
    }

    /// 取所有登记条目状态 (JSON: 供 JNI 回传)
    pub fn status_json(&self) -> String {
        let mut s = String::from("[");
        for (i, e) in self.entries.iter().enumerate() {
            if i > 0 { s.push(','); }
            s.push('{');
            s.push_str(&format!("\"url\":\"{}\",", e.url));
            s.push_str(&format!("\"ts\":{},", e.timestamp_ms));
            s.push_str("\"channels\":[");
            for (j, ch) in e.channels.iter().enumerate() {
                if j > 0 { s.push(','); }
                let (name, status) = match ch {
                    UploadChannel::GitHub => ("gh", e.statuses[j] as u8),
                    UploadChannel::Gitee  => ("gt", e.statuses[j] as u8),
                    UploadChannel::Local  => ("lc", e.statuses[j] as u8),
                };
                s.push_str(&format!("{{\"name\":\"{}\",\"status\":{}}}", name, status));
            }
            s.push(']');
            s.push('}');
        }
        s.push(']');
        s
    }

    /// 取送达统计: (总数, 已送达, 失败)
    pub fn stats(&self) -> (usize, usize, usize) {
        let total = self.entries.len();
        let mut landed = 0;
        let mut failed = 0;
        for e in &self.entries {
            if e.statuses.iter().any(|s| *s == LandingStatus::Landed) {
                landed += 1;
            }
            if e.statuses.iter().all(|s| *s == LandingStatus::Failed) {
                failed += 1;
            }
        }
        (total, landed, failed)
    }

    /// 重置
    pub fn reset(&mut self) {
        self.entries.clear();
    }

    /// 条目数
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
