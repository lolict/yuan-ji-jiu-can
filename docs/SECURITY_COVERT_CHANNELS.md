# Security & Covert Channels · 安全与隐写通道

> 版本 v1.0 · 2026-09-19 · 五层安全:身份→传输→隐写→容器→物理
> 关键词:五彩石隐写 · X25519 · AES‑256‑GCM · 零宽字符 · WASM加密 · 物理隔离 · 双向公约加密

---

## 1. 定位

上层规范已覆盖功能(容器/裁定/路由),本规范覆盖 **安全**:
```
安全 = 五个独立层,任何一层可被攻破不影响其他层:
  1. 身份层(谁在对谁说话)
  2. 传输层(加密通道)
  3. 隐写层(看不见的编码)
  4. 容器层(文件/容器不变性)
  5. 物理层(WASM执行环境隔离)
```

---

## 2. 身份层:X25519 密钥交换

### 2.1 密钥生成

```
用 libsodium 的 X25519:
  crypto_scalarmult_curve25519_base(pub, priv)
  私钥 = 32 字节随机(randombytes_buf)
  公钥 = 从私钥派生
```

### 2.2 会话建立

```
节点A:
  · 生成临时密钥对(A_pub, A_priv)
  · 发送 A_pub

节点B:
  · 生成临时密钥对(B_pub, B_priv)
  · 发送 B_pub

双方:
  shared_secret = crypto_scalarmult(A_priv, B_pub)
                = crypto_scalarmult(B_priv, A_pub)  (对称)
```

### 2.3 前向安全性

每会话生成**临时密钥对**,不重用长期密钥。
即使长期密钥泄漏,历史会话不会被解密(Forward Secrecy)。

---

## 3. 传输层:AES‑256‑GCM + HKDF‑SHA256

### 3.1 密钥派生

```
X25519 仅提供 shared_secret(32字节),不是直接可用的会话密钥。
用 HKDF‑SHA256 派生:
  session_key = HKDF_SHA256(
    ikm: shared_secret,
    salt: 会话ID(双方公钥异或),
    info: "囡囝共生-v1",
    length: 32
  )
```

### 3.2 加密/解密

```
加密(节点A):
  nonce = 12 字节随机(每个消息唯一!)
  ciphertext + tag = AES_256_GCM.encrypt(key, nonce, plaintext, aad)
  发送: nonce || ciphertext || tag

解密(节点B):
  (nonce, ciphertext, tag) = split(received)
  plaintext = AES_256_GCM.decrypt(key, nonce, ciphertext, tag, aad)
  验证 tag: 不匹配 → 拒绝(CRYPTO_AUTH 错误码)
```

### 3.3 重放防护

```
aae = additional authenticated data(附加认证数据):
  aad = frame_counter(帧计数) || timestamp

帧计数器单调递增:
  发送方: sent_seq++
  接收方: 收到帧的 acked_seq 必须 > 上次 acked_seq
  否则 → 重放攻击 → CRYPTO_REPLAY 错误码

时间戳窗口:
  NONCE_WINDOW_SEC = 300(5分钟)
  收到帧的时间戳超出当前时钟 ± 300s → 重放
```

### 3.4 防内存泄漏

```
sodium_memzero(密钥缓冲区): 使用后立即清零
  · shared_secret 派生 session_key 后 → memzero
  · 会话结束后 session_key → memzero
  · 即使程序崩溃/被调试器 dump,密钥不残留在内存
```

---

## 4. 隐写层:看不见的编码

### 4.1 三种隐写模式

| 模式 | 载体 | 容量 | 检测难度 |
|------|------|------|---------|
| Unicode 零宽字符 | 文本(文章/代码注释) | ~文本长度的16倍 | ★★★★★(几乎看不见) |
| LSB 最低位 | 图片 | ~像素数/8 | ★★★☆☆(统计可检测) |
| HTML 结构化 | 网页标签属性顺序 | ~标签数×bit | ★★★★☆(需要语法解析) |

### 4.2 Unicode 零宽字符隐写

```
四个Unicode控制字符:
  ZWSP  = U+200B  bit 0
  ZWNJ  = U+200C  bit 1
  ZWJ   = U+200D  分隔符
  BOM   = U+FEFF  结束符

编码:
  1. carrier 文本原文保留(不修改)
  2. 追加 4 字节大端长度头
  3. payload 每个 bit → ZWSP(0) 或 ZWNJ(1) + ZWJ(分隔)
  4. 结尾 BOM 标记结束

解码:
  1. 从 carrier 中提取所有 ZWSP/ZWNJ/ZWJ/BOM 序列
  2. BOM 之前 → payload
  3. 前 32 bit(4 字节) = 长度头
  4. 后续 bit = 实际 payload
```

### 4.3 为什么在系统中使用

**法律规避 + 隐蔽传输**:
- 系统配置文件可隐写在看似无害的文档中
- 跨节点传携带的配置(如 FQMT 容器条目)的载体不是加密包而是"文章"
- 网络监控者看到的是普通文本/图片,不知道 payload 存在

---

## 5. 容器层:FQMT 条目不变性

### 5.1 三 CRC 条款

规范见 FQMT_BINARY_FORMAT §1.3。
三个 CRC 保证:header 不被篡改、data 不被篡改、trailer 不丢失。

### 5.2 容器条目级 MAC

敏感条目(如在场域快照)除 CRC 外可追加 MAC:
```
entry_mac = HMAC_SHA256(session_key, entry_data || entry_type || anchor_hash)
```

MAC 校验失败 → 该条目被外部篡改 → 丢弃并使用备份恢复。

---

## 6. 物理层:WASM 沙箱隔离

### 6.1 为什么 WASM

传统进程隔离 = 操作系统级别(setuid/seccomp) = 需要 root。
WASM 沙箱 = 浏览器级别 = 用户态即可:
```
· WASM 不能直接访问文件系统 → 仅通过显式 WASI 调用
· WASM 不能直接访问网络 → 仅通过显式 WASI 调用
· WASM 内存隔离 → 每个模块独立线性内存
· WASM 执行沙箱 → 任何越界访问触发 trap(不是 segfault)
```

### 6.2 加密在 WASM 中运行

```
系统 libsodium + libcrypto 不能用:(WASM 不允许直接 FFI)
  → 用 Rust 纯软件实现:
    X25519 → curve25519-dalek (Rust crate)
    AES-256-GCM → aes-gcm (Rust crate)
    HKDF-SHA256 → hkdf + sha2 (Rust crate)

所有加密 = WASM 内部自包含
  · 不依赖宿主 OS 提供
  · 不暴露密钥到 WASM 模块外
  · 编译后是 wasm32 模块,可用于浏览器(Node.js 也支持)
```

---

## 7. 与本系列规范的关系

| 话题 | 主参考 |
|------|--------|
| 在场域持久化 | BOOTSTRAP_SELF_COMPILER §6 + FQMT_BINARY_FORMAT |
| 跨节点一致性 hash | NJSG_SWARM_COORDINATION §5 |
| 隐私化名(身份不绑定真实姓名) | IDENTITY_PRIVACY_SHIELD §3 |
| 多平台密钥管理 | MULTI_HOST_SOVEREIGNTY §3 |
| 无 libc 依赖(WASM就绪) | WASM_COMMON_DIVISOR |
| FQMT 条目级校验 | FQMT_BINARY_FORMAT §3 |
| OS抽象层(OS无关) | OS_LAYER_BARE_METAL |

---

## 8. 完整通信示例

```
前:
  节点A(long-term pubkey=A_lt_pub)
  节点B(long-term pubkey=B_lt_pub)

握手:
Step1: A → B:  X25519 ephemeral A_ep_pub
Step2: B → A:  X25519 ephemeral B_ep_pub + Noise NK handshake pattern
Step3: A 计算 shared_secret = X25519(A_ep_priv, B_ep_pub)
        B 计算 shared_secret = X25519(B_ep_priv, A_ep_pub)
        (两边结果相同)

密钥派生:
Step4: session_key = HKDF_SHA256(shared_salt=A_ep_pub XOR B_ep_pub,
                                  ikm=shared_secret, info="囡囝-v1")

加密传输:
Step5: A → B:
  frame_counter++
  aad = frame_counter || timestamp
  ciphertext = AES256GCM(session_key, nonce=random12, plaintext, aad)
  隐写编码: unicode_encode(carrier="普通文本", payload=ciphertext)
  网络发送

Step6: B 接收:
  隐写解码: ciphertext = unicode_decode(received_text)
  aad' = frame_counter || timestamp
  plaintext = AES256GCM(session_key, nonce, ciphertext, aad')
  验证 tag + aad + frame_counter 单调 → 通过
```

---

## 9. 已知限制

1. **纯软件加密性能比硬件 AES-NI 慢 5-10 倍**:WASM 无 AES-NI 指令。手机芯片(ARM)有 CE(Crypto Extensions),但 WASM 不一定能利用。实际瓶颈在手机 IME 高频输入时。

2. **零宽字符隐写容量受限**:carrier 文本长度 = 1 时,容量 = ~16 bit(2 字节)。真正传 FQMT 需要长文章载体。不适合文件传输,适合密钥/简短配置项。

3. **临时密钥对生成随机数质量**:randombytes_buf 在 WASM 中依赖浏览器 crypto.getRandomValues,在某些旧浏览器中质量不足。应提醒用户升级到现代浏览器。

4. **长期身份密钥没有实现轮换**:X25519 仅用于会话密钥协商,长期身份密钥(用于证书签名)的 rotation 周期未定义。建议 90 天轮换一次长期密钥。

5. **WASM 模块体积**:curve25519-dalek + aes-gcm + sha2 + hkdf 编译后 ~150KB wasm 字节码。对浏览器加载不算大,但对 MetaIME APK 体积有影响。考虑用 wasm-opt 压缩。

---

*本规范的解释权归 刘楚恬和我 所有 · 归属 hunyuan-os/Security 模块*
