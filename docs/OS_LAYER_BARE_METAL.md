# OS Layer & Bare Metal · OS抽象层与裸机目标

> 版本 v1.0 · 2026-09-19 · 核心逻辑与 OS 的分离 · 最终目标是裸机
> 关键词:OS 抽象层 · Bare Metal · 主机 OS 剥离 · 内存分配 · 网络 · 文件系统 · 多态后端

---

## 1. 定位

NJSG 核心逻辑**不应该依赖任何特定操作系统**。
Linux / Windows / macOS 只是当前可用的"宿主",最终目标是 **裸机(Bare Metal)**运行。

```
核心逻辑(纯逻辑,无 OS 依赖)
      ↓
OS 抽象层(OsLayer trait)
      ↓
┌──────────────────────────────────────┐
│  Linux 后端  │  Win 后端  │  macOS 后端  │ ← 当前阶段
└──────────────────────────────────────┘
      ↓
┌──────────────────────────────────────┐
│        裸机 BareMetal 后端           │ ← 最终目标
└──────────────────────────────────────┘
```

---

## 2. 核心接口(OsLayer Trait)

```rust
pub trait OsLayer: Send + Sync + Debug {
    // ── 内存 ──
    fn alloc(&self, size: usize) -> *mut u8;
    fn dealloc(&self, ptr: *mut u8, size: usize);

    // ── 文件 ──
    fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError>;
    fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError>;
    fn file_delete(&self, path: &str) -> Result<(), OsError>;
    fn file_exists(&self, path: &str) -> bool;

    // ── 时间 ──
    fn time_millis(&self) -> u64;
    fn perf_counter_ns(&self) -> u64;   // 纳秒性能计数器(wall-clock 不保证对齐)

    // ── 网络 ──
    fn net_send(&self, host: &str, port: u16, data: &[u8]) -> Result<(), OsError>;
    fn net_recv(&self, host: &str, port: u16, buf: &mut [u8]) -> Result<usize, OsError>;

    // ── 控制台 ──
    fn print(&self, msg: &str);
    fn platform(&self) -> &str;
}
```

### 2.1 为什么用 trait 而不是 #[cfg(feature)]

feature 一次性决定后端,trait 允许:
- **运行时多态**:同一编译文件可支持不同宿主,根据运行环境切换
- **测试**:单元测试注入 MockOsLayer,无需真实系统调用
- **裸机迁移**:BareMetalOs 实现 trait = 零修改替换 Linux 后端

### 2.2 错误类型

```
OsError:
  FileNotFound(String)      — 文件不存在
  PermissionDenied(String)  — 无权限
  NetworkError(String)      — 网络错误
  AllocFailed               — 内存分配失败
  Other(String)             — 其他
```

错误转换为: 上层模块不直接处理 OsError,而是转为容器间呼唤的 CallResponse错码。

---

## 3. 多后端设计

### 3.1 Linux 后端

```
LinuxOs {
    · file_read/write → std::fs
    · net_send/recv → TCP stream
    · time_millis → SystemTime
    · perf_counter_ns → libc clock_gettime
    · platform → "linux"
}
```

### 3.2 Windows 后端(计划中)

```
WindowsOs {
    · file_read/write → WinAPI CreateFile/ReadFile/WriteFile
    · net_send/recv → WinSock
    · time_millis → GetTickCount64
    · platform → "windows"
}
```

### 3.3 macOS 后端(计划中)

```
MacOs {
    · 同 Linux(BSD 子系统)
    · Mach 内核可直接用作裸机接口的前身
    · platform → "macos"
}
```

### 3.4 裸机后端(最终目标)

```
BareMetalOs {
    · alloc/dealloc → 自实现分配器(buddy system / slab)
    · file → 直接读取磁盘扇区(FAT32/自研 FS)
    · net → 直接驱动 NIC(Realtek/E1000 兼容)
    · time → APIC 定时器 / HPET
    · print → VGA 文本模式帧缓冲
    · platform → "bare_metal"
}
```

**裸机关键点**:无 libc,无 std,全 `#[no_std]` 代码。这就是为什么 WASM_COMMON_DIVISOR 强调 NJ_ 前缀(无 libc 依赖),也是为什么 CROSS_ISA_EXECUTION_LAYER 要求所有 MDA 指令用固定宽度类型。

---

## 4. 与内核的关系

### 4.1 调用链

```
业务模块(MetaIME / 裁定引擎 / 容器本体)
    ↓ 不直接调用 OS API
OS 抽象层(OsLayer trait)
    ↓ 具体后端
LinuxOs / WindowsOs / MacOs / BareMetalOs
    ↓ 系统调用 / 直接硬件
Linux 内核 / 裸机硬件
```

好处:
- 换操作系统 = 换后端,不改业务代码
- 测试 = MockOsLayer 注入
- 裸机迁移 = 实现 trait 一次,处处可用

### 4.2 文件系统的容器化

文件操作不只是 I/O — 文件=容器在宿主的投影:
```
NJSG 容器体系 → 在场域登记
                ↓ 需要持久化?
OS Layer file_write → FQMT 容器条目 → 写磁盘

读取路径:
OS Layer file_read → FQMT 字节流
                  → BOOTSTRAP_SELF_COMPILER 域加载
                  → 内存中的容器状态
```

### 4.3 网络操作盟机合化

节点间通信不直接裸 TCP,经过盟机合协议:
```
NJSG 节点A → Call::Arbitrate / Call::Witness
           → 序列化为 FQMT 容器条目(Protocol, Kind=0x7)
           → OS Layer net_send

对端 NJSG 节点B → OS Layer net_recv
                → FQMT 解析
                → Protocol 处理
```

---

## 5. 内存模型

### 5.1 域加载(一次分配 + 不再增长)

```
启动:
  1. 从 FQMT 容器条目 0xFC 读取在场域快照
  2. 统计总符号数 N
  3. OS Layer.alloc(N * SIZE_PER_SYMBOL) → 一次性得到 Domain 区域
  4. 读取所有符号定义到 Domain
  5. RwLock::new(Domain) → 全局只读引用

运行期:
  不调用 alloc! 不调用 dealloc!
  域内所有数据 = 'static 不可变引用
```

### 5.2 Fallback 路径

alloc 失败(OsError::AllocFailed):
```
Step 1: 尝试降域加载(仅加载 ·global 文件中的核心符号,跳过扩展)
Step 2: 仍失败 → 降为极小内核模式(64 容器核心符号 + 21 声母 + 35 韵母)
Step 3: 仍失败 → 最小应急模式(不加载域,直接 PASS 所有,仅做应急用途)
  
每次降级 = 输出日志(print),允许用户看到问题。
```

### 5.3 裸机内存策略

无 libc = 无 malloc/free。

```
自定义分配器策略:
  · 启动时一次性获取全部可用内存(链接脚本确定)
  · 把内存切成 Buddy System(2^n 块)
  · 容器 = 从 buddy 中获取 2^n 大小的块
  · 释放 = 合并回 buddy
  · 无泄漏检查 = 运行期全部静态预分配,死亡才释放
```

---

## 6. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 域加载 · 运行期零内存增长 | BOOTSTRAP_SELF_COMPILER §8 |
| MCCP 字节码在 VM 内运行 | MCCP_ISA_VM §4 |
| WASM 无 libc 依赖 | WASM_COMMON_DIVISOR(阶梯制双向公约) |
| 时间脉冲 · 纳秒计数 | TEMPORAL_PULSE_LIFECYCLE §2 |
| 盟机合节点间通信 | MENGJI_IDENTITY_PROTOCOL §8.1 |
| 负样本 · FQMT 持久化 | JUDGEMENT_APPARATUS §3 |
| 容器在文件系统投影 | CONTAINER_REPERTOIRE |
| FQMT 格式存储 | FQMT_BINARY_FORMAT |

---

## 7. 完整示例(Linux后端起步)

```rust
#[cfg(target_os = "linux")]
pub use linux::LinuxOs;

// 核心启动时:选择后端
fn create_os() -> Box<dyn OsLayer> {
    Box::new(LinuxOs::new())
}

// 上层代码:完全不感知是 Linux 还是 裸机
fn bootstrap(os: &dyn OsLayer) {
    let fqmt_data = os.file_read(".global").expect("·global missing");
    let domain = Domain::load(&fqmt_data);
    
    loop {    // RUNTIME_BOOTSTRAP for(;;) 主循环
        let input = poll_input(os);
        let frame = perception.process(input);
        let verdict = judge.apply(frame);
        if verdict == PASS {
            let output = soul_kernel.execute(verdict);
            os.print(&output);
        }
        os.dealloc(input.ptr, input.size);    // 可选: 立即释放或复用
    }
}
```

---

## 8. 已知限制

1. **Windows / macOS 后端未实现**:当前仅 Linux 后端完成(依赖 std::fs 和 BSD socket)。Windows 和 macOS 需另外实现。

2. **bare_metal 是目标不是现实**:当前 boot 到裸机需要自定义 bootloader(UEFI/BIOS),本文件不定义 bootloader,仅定义裸机运行时。

3. **无 libc 依赖的具体表现**:OsLayer 内的 alloc/dealloc 接口不允许调用 libc malloc。当前 Linux 后端使用 std → 依赖 libc,需逐步替换为 mmap/自管理分配器。

4. **网络 send/recv 非异步**:net_send 是同步 blocking 调用。在高频场景(tick 60Hz + 每 tick 网络心跳)下阻塞 tick。未来应升级为 io_uring(Linux) / IOCP(Windows)/ kqueue(BSD)。

5. **file_delete 在容器语义下的歧义**:删除 = 从在场域删除?还是从磁盘删除文件?本规范以"在场域 tombstone + 磁盘保留 FQMT 容器条目"为默认行为 — 不物理删除数据。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/OsLayer 模块*
