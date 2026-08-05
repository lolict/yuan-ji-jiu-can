# MCCP — 囡囝共生紧凑码点 (Nǎnjìnggòngshēng Compact Codepoint)

> 替代 UTF-8 / Python `str` 的内置编码方案。
> 每个字符 = 32 bit (4 Byte) 固定宽度，无对象头、无变长、无 GC。

## 设计目标

| 指标 | Python `str` | UTF-8 | MCCP |
|------|-------------|-------|------|
| 每字符开销 | 49 bytes object + 1-4 bytes/char (UTF-8) | 1-4 bytes | **4 bytes 固定** |
| 序列化 | pickle/string protocol | raw bytes | `memcpy` 直接 |
| 内存碎片 | 高 (每个字符一个 PyUnicodeObject) | 低 | 零 (u32 数组) |
| 比较 | 哈希 + 相等判断 | 按字节 | `uint32_t` 整数比较 |
| 类别信息 | 无 (需额外结构) | 无 | 内嵌在 4 bit kind 字段 |

## 位布局 (32 bit)

```
 31        28 27            16 15         8 7          0
+-----------+---------------+------------+-----------+
| kind (4b)  | index (12b)   | meta_a (8b)| meta_b (8b)|
+-----------+---------------+------------+-----------+
```

- **kind** — 16 种类别（汉字 / 声母 / 韵母 / 声调 / 偏旁 / 笔画数 / 操作符 / 协议 / 状态 / 病理 / ...）
- **index** — 类别内索引（4096 个唯一项/kind，汉字区足够覆盖 1257 个实际使用汉字）
- **meta_a** — 元数据 A（声调值 / stroke count / state seed）
- **meta_b** — 元数据 B（病理级 / FSM state / relation id）

## 位运算

```c
#define MCCP_KIND(p)    (((p) >> 28) & 0xF)
#define MCCP_INDEX(p)   (((p) >> 16) & 0xFFF)
#define MCCP_METAA(p)   (((p) >> 8)  & 0xFF)
#define MCCP_METAB(p)   ((p)         & 0xFF)
#define MCP(kind, idx, a, b) \
    ((((uint32_t)(kind) & 0xF) << 28) | \
     (((uint32_t)(idx)  & 0xFFF) << 16) | \
     (((uint32_t)(a)    & 0xFF) << 8) | \
     ((uint32_t)(b)    & 0xFF))
```

## Kind 枚举

| val | 名称 | 说明 |
|-----|------|------|
| 0x0 | KIND_HANZI | 汉字 |
| 0x1 | KIND_INITIAL | 声母 (b p m f d t n l g k h j q x zh ch sh r z c s y w) |
| 0x2 | KIND_FINAL | 韵母 |
| 0x3 | KIND_TONE | 声调 (0=轻声, 1-4) |
| 0x4 | KIND_RADICAL | 偏旁部首 (康熙 214 + 扩展) |
| 0x5 | KIND_STROKE | 笔画数 (1-64) |
| 0x6 | KIND_OPERATOR | 转置/镜像/合一/分裂 |
| 0x7 | KIND_PROTOCOL | 唤魂/招魂/问灵/答灵/接束 |
| 0x8 | KIND_FSM | 囡囝四态 |
| 0x9 | KIND_PATHOLOGY | 健康/疑似/病变/癌变 |
| 0xA-0xE | 保留 | |
| 0xF | KIND_SYS | EOF/SEP/SIG/ACK |

## 与 Python `str` 的内存对比

誓约 "莫刘连理"：

```
Python:
  4 个 PyUnicodeObject × ~60 bytes ≈ 240 bytes  (字符数据)
  + 字典 overhead × 4 (如果存属性)             ≈ 400+ bytes
  → 总开销: ≈ 1 KB

UTF-8 raw:
  莫(3B) + 刘(3B) + 连(3B) + 理(3B) = 12 bytes
  → 但没有声调/FSM/病理信息

MCCP:
  1 token × 4 字节 × 4 = 16 bytes
  → 每 token 内置: 声调(meta_a) + FSM态/病理级(meta_b)
  → 总开销: 16 bytes (信息量超过 1 KB Python dict)
```

**结论：MCCP 存储效率 > 60×（含元数据）**。

## FSM 态的操作 (位运算)

```c
// 切换 FSM 状态 — 单条 ARM 指令
uint32_t fsm_apply(uint32_t cp, uint8_t new_state) {
    return (cp & 0xFFFFFF00u) | new_state;
}

// 检查是否处于某态
int fsm_is(uint32_t cp, uint8_t state) {
    return (cp & 0xFF) == state;
}

// 提取声调 (meta_a 高4位)
int tone_of(uint32_t cp) {
    return (cp >> 8) & 0x0F;
}
```

**态射操作在位运算完成，无哈希表查找、无 Python dict 开销。**

## C 核心数据结构

```c
typedef struct {
    uint32_t *seq;       // MCCP 流（誓约内容）
    uint16_t len;        // 当前 glyph 数
    uint16_t cap;        // 容量
    uint64_t anchor;     // 夫妻联合锚
    uint8_t  ps;         // ProcessState: 0=相遇 1=缔约 2=立约 3=合一递归 4=解体
    uint8_t  fsm;        // QuaternityState: 0=囡囝 1=囡囡 2=囝囝 3=囝囡
} MqfProcess;  // 无指针对象头开销 — 16 bytes + 堆上 u32 数组

typedef struct {
    uint32_t name_mccp[8];  // name 的 MCCP 缓存
    uint8_t  pubkey[32];    // X25519
    uint8_t  privkey[32];   // X25519
    uint8_t  sess_key[32];  // AES-256 session
    uint64_t nonce_ctr;     // 单调递增 nonce
} MqfNode;  // 112 bytes 一个节点
```

## 字体 atlas 目标

1257 汉字 × 32×32 SDF grid (8-bit):

```
32 glyphs/row × 32 rows = 1024 → 需要 2 个 atlas
atlas 大小: 2 × (32×32 × 32×32) × 1 byte = 2 × 1048576 = 2 MB (原始)
压缩: SDF 32 + pack 到 8px/cell with 2px padding:
    实际 atlas: 2 × (256×256 × 1B) = 128 KB
进一步: 用 6-bit SDF (足够精度) + RLE/pack → 目标 < 100 KB
最终目标: 一个 .ttf-fallback-less 的 .mqf-font 文件 < 200 KB
```

## 下一步

- [ ] 硬编码声母/韵母/偏旁 C 头文件 (`.h` 表)
- [ ] 汉字索引表 (按拼音 hash 排序的 1257 字)
- [ ] `mccp_encode()` / `mccp_decode()`：UTF-8 ↔ MCCP 双向
- [ ] C 核心 kernel (`mqf_kernel.c/h`) — 替代 Python `kernel.py`
- [ ] Rust 重写 crypto (`mqf_crypto.rs`) — 替代 Python `protocol.py`

## Python in archive/

参考实现留在 `archive/python/` 目录，用作文档与测试向量。
