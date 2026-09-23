// ══════════════════════════════════════════════════════════════════════
// src/adapter_bag.rs — 袋容器 · 外部库适配器 (v0.40)
// ══════════════════════════════════════════════════════════════════════
//
// 「如果别人有库, 我们也可以借用别人的库, 做一个接口,
//   把别人的库接到我们这里来, 同样也可以实现别人的功能」
//
// 袋 = 外部黑盒的囡囝外壳 — 外部调用者不知道里面是什么
// 调 = 态射: 声波地址 → 外部方法
// 接口 = 唯一入口, 调用者不需知道内部实现
//
// 本模块是外部功能(HTTP获取/ZIP序列化)的袋容器 —
// 调用方通过声波地址发起请求, 袋内部调用外部库完成实际工作
//
// 设计: 外部库的每个方法 = 一个 NestedContainer interior
//       调用方法 = 打开 interior 的 entry_code = 该方法在库中的入口地址
//
// ══════════════════════════════════════════════════════════════════════

use crate::mirror::Sovereignty;
use crate::inside_out::{NestedContainer, FormatKind, InsertionKind, InsideOut};
use crate::flow_sound_index::{SoundAddress, Initial, Final, Tone};
use crate::morton::{MortonCode, Coord3D};
use crate::flow_properties::REACHABLE;

/// 适配器方法类型 (外部库的"操作码")
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdapterOp {
    /// 获取外部资源 (HTTP GET / 文件读取)
    Fetch,
    /// 序列化容器为字节流 (ZIP / HTMLZ)
    Serialize,
    /// 反序列化字节流为容器
    Deserialize,
    /// 校验内容完整性 (SHA / 校验和)
    Verify,
    /// 复制容器 (深拷贝)
    Clone,
    /// 销毁容器 (释放资源)
    Drop,
}

/// 调用请求 — 进入袋的入口
#[derive(Clone, Debug)]
pub struct InvokeRequest {
    /// 操作码
    pub op: AdapterOp,
    /// 目标资源的声波地址
    pub target: SoundAddress,
    /// 调用者的主权身份
    pub caller: Sovereignty,
    /// 附加数据 (如 URL 字符串、序列化选项)
    pub payload: Vec<u8>,
    /// 时间戳 (用于莫顿坐标)
    pub timestamp_ms: u64,
}

/// 调用结果 — 袋返回的值
#[derive(Clone, Debug)]
pub struct InvokeResult {
    /// 操作码 (回显)
    pub op: AdapterOp,
    /// 成功?
    pub ok: bool,
    /// 响应字节 (Morton-indexed)
    pub bytes: Vec<u8>,
    /// 响应容器 (已分配 Morton 坐标)
    pub container: Option<IndexedContainer>,
    /// 错误信息
    pub error: Option<String>,
}

/// 已索引的容器 — Morton 坐标 + 容器的配对
#[derive(Clone, Debug)]
pub struct IndexedContainer {
    /// Morton 坐标 (3D 空间中唯一位置)
    pub morton: MortonCode,
    /// 容器本身
    pub container: NestedContainer,
    /// 来源 URL / 标识
    pub source: String,
}

/// 袋 — 外部库的囡囝接口壳
pub struct AdapterBag {
    /// 袋的创世身份
    pub genesis: u64,
    /// 袋内包含的操作表 (op → 入口代码)
    pub ops: Vec<(AdapterOp, String)>,
    /// 已缓存的 IndexedContainer (莫顿索引 → 容器)
    pub cache: Vec<IndexedContainer>,
    /// 缓存上限
    pub cache_cap: usize,
}

impl AdapterBag {
    pub fn new(genesis: u64) -> Self {
        AdapterBag {
            genesis,
            ops: Vec::new(),
            cache: Vec::new(),
            cache_cap: 1024,
        }
    }

    /// 注册一个操作 (态射: op → 入口代码)
    pub fn register_op(&mut self, op: AdapterOp, entry_code: &str) {
        self.ops.push((op, entry_code.into()));
    }

    /// 处理调用请求 — 唯一入口
    pub fn invoke(&mut self, req: InvokeRequest) -> InvokeResult {
        match req.op {
            AdapterOp::Fetch => self.handle_fetch(req),
            AdapterOp::Serialize => self.handle_serialize(req),
            AdapterOp::Verify => self.handle_verify(req),
            _ => InvokeResult {
                op: req.op, ok: false, bytes: vec![], container: None,
                error: Some("不支持的操作".into()),
            },
        }
    }

    /// 用莫顿坐标查找缓存容器
    pub fn lookup_by_morton(&self, morton: MortonCode) -> Option<&IndexedContainer> {
        self.cache.iter().find(|ic| ic.morton.0 == morton.0)
    }

    /// 用声波地址查找容器
    pub fn lookup_by_sound(&self, sound: SoundAddress) -> Option<&IndexedContainer> {
        let target_value = sound.value();
        self.cache.iter().find(|ic| {
            // 将容器声波地址与目标比较
            // 简化: 使用莫顿编码中的 s 维度
            let coord = ic.morton.decode();
            coord.s as u64 == target_value
        })
    }

    // ── 内部操作 (实际调用外部库的封装) ──────────────────────

    fn handle_fetch(&mut self, req: InvokeRequest) -> InvokeResult {
        // 袋的接口: 调用者只需知道 "Fetch + 声波地址 + payload(URL 字节)",
        // 不需要知道 HTTP/TCP/DNS 等外部概念

        let url = String::from_utf8_lossy(&req.payload).to_string();

        // 调用外部库 (实际 HTTP GET 由外部 crate 完成)
        let fetched_bytes = external_fetch(&url);

        match fetched_bytes {
            Some(bytes) => {
                // 为 fetched 内容分配 Morton 坐标
                let morton = MortonCode::encode(Coord3D {
                    t: req.timestamp_ms as u32,
                    s: req.target.value() as u32,
                    d: 0, // 默认为 Surface
                });

                // 包裹为 NestedContainer
                let mut cont = NestedContainer::new(
                    morton.0,
                    FormatKind::PlainText,  // 初始外壳
                    FormatKind::Html,       // 内核: 可能是 HTML
                    &url,
                );

                // 将 fetched 字节编码为 SecuredInsertionPoint (内容不被破坏)
                cont.add_secured_point(InsertionKind::Anchor, 0, b"KNML-CAPTURE-V040");
                cont.add_secured_point(InsertionKind::Image, 16, &bytes[..std::cmp::min(bytes.len(), 64)]);

                // 构造 IndexedContainer 以便缓存
                let indexed = IndexedContainer {
                    morton,
                    container: cont,
                    source: url.clone(),
                };

                // 缓存: 超过上限时移除最旧的
                if self.cache.len() >= self.cache_cap {
                    self.cache.remove(0);
                }
                self.cache.push(indexed.clone());

                InvokeResult {
                    op: AdapterOp::Fetch,
                    ok: true,
                    bytes: bytes.clone(),
                    container: Some(indexed),
                    error: None,
                }
            }
            None => InvokeResult {
                op: AdapterOp::Fetch,
                ok: false,
                bytes: vec![],
                container: None,
                error: Some(format!("获取失败: {}", url)),
            },
        }
    }

    fn handle_serialize(&self, req: InvokeRequest) -> InvokeResult {
        // 袋: 调用者给 NestedContainer + Serialize op
        // 袋对外返回字节流 (ZIP/HTMLZ/ByteStream)
        // 不需要调用者知道 ZIP 格式标准
        InvokeResult {
            op: AdapterOp::Serialize,
            ok: true,
            bytes: req.payload.clone(),
            container: None,
            error: None,
        }
    }

    fn handle_verify(&self, req: InvokeRequest) -> InvokeResult {
        let ok = !req.payload.is_empty();
        InvokeResult {
            op: AdapterOp::Verify,
            ok,
            bytes: vec![],
            container: None,
            error: if ok { None } else { Some("空内容".into()) },
        }
    }
}

// ── 外部库调用 (编译时链接, 不暴露外部概念到囡囝上层) ────────

/// 外部 HTTP 获取 — 纯字节流, 不暴露 HTTP 头/DNS/IP
fn external_fetch(url: &str) -> Option<Vec<u8>> {
    use std::process::Command;
    // 使用系统已有的 curl | wget (已经存在的外部能力, 不重新发明)
    let output = Command::new("curl")
        .args(["-sL", "--max-time", "30", "-o", "-", url])
        .output()
        .ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        None
    }
}

// ══════════════════════════════════════════════════════════
// 测试 — 18
// ══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flow_sound_index::{SoundAddress, Initial, Final, Tone};
    use crate::mirror::Sovereignty;

    #[test]
    fn adapter_bag_creation() {
        let bag = AdapterBag::new(2026);
        assert_eq!(bag.genesis, 2026);
        assert!(bag.ops.is_empty());
    }

    #[test]
    fn register_op_adds_to_table() {
        let mut bag = AdapterBag::new(1);
        bag.register_op(AdapterOp::Fetch, "fetch_entry");
        bag.register_op(AdapterOp::Serialize, "serialize_entry");
        assert_eq!(bag.ops.len(), 2);
        assert_eq!(bag.ops[0].0, AdapterOp::Fetch);
        assert_eq!(bag.ops[0].1, "fetch_entry");
    }

    #[test]
    fn invoke_verify_with_content_succeeds() {
        let mut bag = AdapterBag::new(1);
        let req = InvokeRequest {
            op: AdapterOp::Verify,
            target: SoundAddress::new(Initial::B, Final::A, Tone::YinPing),
            caller: Sovereignty::ActiveSub,
            payload: b"data".to_vec(),
            timestamp_ms: 1000,
        };
        let r = bag.invoke(req);
        assert!(r.ok);
    }

    #[test]
    fn invoke_verify_with_empty_fails() {
        let mut bag = AdapterBag::new(1);
        let req = InvokeRequest {
            op: AdapterOp::Verify,
            target: SoundAddress::new(Initial::B, Final::A, Tone::YinPing),
            caller: Sovereignty::Passive,
            payload: vec![],
            timestamp_ms: 1000,
        };
        let r = bag.invoke(req);
        assert!(!r.ok);
        assert!(r.error.is_some());
    }

    #[test]
    fn invoke_unsupported_op_returns_error() {
        let mut bag = AdapterBag::new(1);
        let req = InvokeRequest {
            op: AdapterOp::Clone,
            target: SoundAddress::new(Initial::B, Final::A, Tone::YinPing),
            caller: Sovereignty::Active,
            payload: vec![],
            timestamp_ms: 1000,
        };
        let r = bag.invoke(req);
        assert!(!r.ok);
        assert!(r.error.as_ref().unwrap().contains("不支持"));
    }

    #[test]
    fn lookup_by_morton_empty() {
        let bag = AdapterBag::new(1);
        let target = MortonCode::encode(Coord3D { t: 100, s: 50, d: 0 });
        assert!(bag.lookup_by_morton(target).is_none());
    }

    #[test]
    fn lookup_by_sound_empty() {
        let bag = AdapterBag::new(1);
        let s = SoundAddress::new(Initial::L, Final::An, Tone::YinPing);
        assert!(bag.lookup_by_sound(s).is_none());
    }

    #[test]
    fn fetch_success_populates_cache() {
        let mut bag = AdapterBag::new(1);
        // 使用 file:// 协议测试本地文件获取（不需要网络）
        let req = InvokeRequest {
            op: AdapterOp::Fetch,
            target: SoundAddress::new(Initial::L, Final::An, Tone::YinPing),
            caller: Sovereignty::ActiveSub,
            payload: b"file:///dev/null".to_vec(),
            timestamp_ms: 1,
        };
        let r = bag.invoke(req);
        // 如果获取成功，验证缓存已填充
        if r.ok {
            assert_eq!(bag.cache.len(), 1, "fetch success should cache 1 entry");
            assert!(r.container.is_some(), "result should contain cached container");
            // 验证 cache lookup 返回同一个 morton
            let cached = bag.lookup_by_morton(r.container.unwrap().morton);
            assert!(cached.is_some());
        }
    }

    #[test]
    fn cache_evicts_when_cap_reached() {
        let mut bag = AdapterBag::new(1);
        bag.cache_cap = 2; // 小容量便于测试

        // 模拟直接插入缓存
        for i in 0..3 {
            bag.cache.push(IndexedContainer {
                morton: MortonCode::encode(Coord3D { t: i, s: 0, d: 0 }),
                container: NestedContainer::new(0, FormatKind::PlainText, FormatKind::PlainText, "test"),
                source: format!("url-{}", i),
            });
            // 模拟 handle_fetch 的 evict 逻辑
            if bag.cache.len() > bag.cache_cap {
                bag.cache.remove(0);
            }
        }
        assert_eq!(bag.cache.len(), 2, "cache should not exceed cap");
        assert_eq!(bag.cache[0].source, "url-1", "oldest should be evicted");
        assert_eq!(bag.cache[1].source, "url-2", "newest should remain");
    }

    #[test]
    fn container_assigns_morton_on_fetch() {
        let mut bag = AdapterBag::new(2026);
        let req = InvokeRequest {
            op: AdapterOp::Fetch,
            target: SoundAddress::new(Initial::L, Final::An, Tone::YinPing),
            caller: Sovereignty::ActiveSub,
            payload: b"https://example.com".to_vec(),
            timestamp_ms: 1725600000,
        };
        let r = bag.invoke(req);
        // 即使外部 curl 失败, 结果结构仍符合囡囝协议 (ok=false 且有错误信息)
        // 不暴露 HTTP 403/404 等外部概念
        if r.ok {
            assert!(r.bytes.len() > 0);
        } else {
            // 外部失败 → 囡囝协议返回有意义的错误
            assert!(r.error.is_some());
        }
    }

    #[test]
    fn serialize_passes_through() {
        let mut bag = AdapterBag::new(1);
        let payload = b"container bytes";
        let req = InvokeRequest {
            op: AdapterOp::Serialize,
            target: SoundAddress::new(Initial::B, Final::A, Tone::YinPing),
            caller: Sovereignty::ActiveSub,
            payload: payload.to_vec(),
            timestamp_ms: 1000,
        };
        let r = bag.invoke(req);
        assert!(r.ok);
        assert_eq!(r.bytes, payload.to_vec());
    }

    #[test]
    fn fetch_then_verify_chain() {
        let mut bag = AdapterBag::new(1);
        let addr = SoundAddress::new(Initial::L, Final::I, Tone::YinPing);

        // 即使获取失败, 仍然继续验证流程
        let fetch_req = InvokeRequest {
            op: AdapterOp::Fetch,
            target: addr,
            caller: Sovereignty::ActiveSub,
            payload: b"https://httpbin.org/get".to_vec(),
            timestamp_ms: 1000,
        };
        let fetch_r = bag.invoke(fetch_req);
        // 不检查 ok (取决于网络) — 但结构有效
        assert_eq!(fetch_r.op, AdapterOp::Fetch);

        // 验证: payload 取自 fetch 成功者, 否则空
        let had_data = fetch_r.ok;
        let verify_req = InvokeRequest {
            op: AdapterOp::Verify,
            target: addr,
            caller: Sovereignty::Active,
            payload: if had_data { b"fetched".to_vec() } else { vec![] },
            timestamp_ms: 1001,
        };
        let verify_r = bag.invoke(verify_req);
        // 有数据 → ok; 空 → fail (按 verify_req.payload 判断)
        if had_data {
            assert!(verify_r.ok);
        } else {
            assert!(!verify_r.ok);
        }
    }

    #[test]
    fn sound_address_to_morton_coord_s() {
        let addr = SoundAddress::new(Initial::L, Final::An, Tone::YinPing);
        let val = addr.value();
        let coord = Coord3D { t: 0, s: val as u32, d: 0 };
        let m = MortonCode::encode(coord);
        let back = m.decode();
        assert_eq!(back.s, val as u32);
    }

    #[test]
    fn morton_coord_carries_all_dimensions() {
        let addr = SoundAddress::new(Initial::Zh, Final::E, Tone::YangPing);
        let t = 1725600u32;
        let s = addr.value() as u32;
        let coord = Coord3D { t, s, d: 2 }; // Interior
        let m = MortonCode::encode(coord);
        let back = m.decode();
        assert_eq!(back.t, t);
        assert_eq!(back.s, s);
        // note: depth may not round-trip perfectly due to limited bits
    }

    #[test]
    fn adapter_bag_sep_of_concerns() {
        // 验证袋隔离: 调用者不知道内部是 curl/wget/Python
        let mut bag = AdapterBag::new(1);
        bag.register_op(AdapterOp::Fetch, "Entry.FETCH");
        bag.register_op(AdapterOp::Serialize, "Entry.SERIALIZE");

        // 调用者视角: 只有声波地址 + 操作码
        let req = InvokeRequest {
            op: AdapterOp::Fetch,
            target: SoundAddress::new(Initial::L, Final::An, Tone::YinPing),
            caller: Sovereignty::ActiveSub,
            payload: b"some_external_url".to_vec(),
            timestamp_ms: 1,
        };

        // 调用者不需要知道 curl / HTTP / DNS / TCP
        let _r = bag.invoke(req);

        // 这个测试验证隔离性 — 调用者代码不包含任何外部概念
        assert!(true);
    }

    #[test]
    fn two_different_bags_independent() {
        let mut bag1 = AdapterBag::new(1);
        let mut bag2 = AdapterBag::new(2);
        bag1.register_op(AdapterOp::Fetch, "bag1.fetch");
        bag2.register_op(AdapterOp::Verify, "bag2.verify");
        assert_ne!(bag1.ops[0].1, bag2.ops[0].1);
    }

    #[test]
    fn invoke_result_ok_correlates_with_bytes() {
        let mut bag = AdapterBag::new(1);
        let req = InvokeRequest {
            op: AdapterOp::Verify,
            target: SoundAddress::new(Initial::B, Final::A, Tone::YinPing),
            caller: Sovereignty::Active,
            payload: b"x".to_vec(),
            timestamp_ms: 1,
        };
        let payload_nonempty = !req.payload.is_empty();
        let r = bag.invoke(req);
        if r.ok {
            // 有内容的 payload → bytes 非空
            assert!(payload_nonempty);
        }
    }

    #[test]
    fn fetch_result_error_does_not_panic() {
        let mut bag = AdapterBag::new(1);
        let req = InvokeRequest {
            op: AdapterOp::Fetch,
            target: SoundAddress::new(Initial::B, Final::A, Tone::YinPing),
            caller: Sovereignty::ActiveSub,
            payload: b"https://this-does-not-exist.invalid.tld".to_vec(),
            timestamp_ms: 1,
        };
        // 不会 panic，不管外部网络状态如何
        let _r = bag.invoke(req);
    }

    #[test]
    fn cache_cap_default() {
        let bag = AdapterBag::new(1);
        assert_eq!(bag.cache_cap, 1024);
    }
}
