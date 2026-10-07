# RISC-V 压缩指令编码器/解码器(ISA16) — 囡囝共生 · 16-bit C扩展 · 三象限 · 编码和解码 · 断言保护

## 1. 定位

将原始机器码(32位 RISC-V)压缩为 16 位 C 扩展指令, 部署到资源受限设备(手机/嵌入式). 编码和解码双向实现. assert!确保无效编码被拦截.

对应 Rust 模块: encoding/isa16.rs.

## 2. 压缩指令格式 (RISC-V Compressed ISA v2.2)

```
bit[1:0] = 象限标识(Q0/Q1/Q2)
Q0 (00): funct3=bits[15:13], bit12未使用
Q1 (01): bit15=imm/flag, funct3=bits[14:12], bits[11:7]=rd, bits[6:2]=imm
Q2 (10): funct3=bits[15:13], bit12=flag, bits[11:7]=rd/rs1, bits[6:2]=rs2/shamt
```

## 3. 指令编码方法

### 3.1 Q0 (运算类)

- lw(rd, rs1, uimm) — C.LW: 加载字. rd/rs1 限 8-15(压缩). 6-bit 立即数位交织(bit5/4/3→bits12/11/10, bit2→bit6, bit6→bit5)
- sw(rs2, rs1, uimm) — C.SW: 存字. 字段位置类似 lw

### 3.2 Q1 (立即数/跳转类)

- addi(rd, nzimm) — C.ADDI: rd 加立即数(5位 imm + bit15 sign → 6-bit)
- li(rd, imm) — C.LI: 加载立即数到 rd
- lui(rd, nzimm) — C.LUI: 加载到高 20 位
- slli(rd, shamt) — C.SLLI: 逻辑左移(6位移位量拆 bit15+bits6:2)
- j(offset) — C.J: 跳转(直接硬编码 0xa001)
- jal(offset) — C.JAL: 跳转+链接(0x2001)
- beqz(rs1, offset) — C.BEQZ: 条件跳转(0xc001)
- bnez(rs1, offset) — C.BNEZ: 条件跳转(0xe001)
- addi16sp / lwsp / swsp — placeholder(nop 替代)

### 3.3 Q2 (寄存器类)

- jr(rs1) — C.JR: 寄存器跳转
- jalr(rs1) — C.JALR: 跳转+链接到寄存器
- ebreak() — C.EBREAK: 断点
- mv(rd, rs2) — C.MV: 寄存器复制
- add(rd, rs2) — C.ADD: 寄存器加法

### 3.4 硬编码常量

- nop() = 0x0001 — 4 位操作码全 0 后 bit0=1
- j/jal/beqz/bnez/addi16sp = 硬编码占位, 实际部署时替换为真正的位拼接

## 4. 解码器

decode(self) → String: 三位分支(Q0/Q1/Q2):

### 4.1 Q0 – funct3=2 → C.LW, 6 → C.SW

rd = bits[2:]+8; rs1 = bits[9:]+8; 6位立即数重组.

### 4.2 Q1 – funct3=0/2/3/4/5/6/7

- 0: c.addi(符号扩展立即数)
- 2: c.li(字面立即数, 可能与 addi 同编码不同 mnemonic)
- 3: c.lui 或 c.addi16sp(rd==2)
- 4: CB/CR(slli/srai/andi/sub/xor/or/and — 根据 funct2 + 子操作码解码)
- 5/6/7: c.j / c.beqz / c.bnez

### 4.3 Q2 – funct3=0/2/4/6

- 0: c.slli(6位移位量)
- 2: c.lwsp(立即数重组)
- 4: c.jr / c.mv / c.ebreak / c.jalr / c.add(flag/rs1/rs2 三维匹配)
- 6: c.swsp

## 5. 断言保护

```rust
assert!(rd != 0 && shamt < 32);       // SLLI: 移位量 < 32, rd 不 = 0
assert!(rd >= 8 && rd <= 15 && ...);  // rd 在压缩区间
```

无效编码在编译期/运行期就会被拦截, 不会产生不可预测行为.

## 6. 实现约束

- 硬编码跳转常量(0xa001/0x2001/0xc001/0xe001/nop=0x0001)当前用作占位, 正式部署需替换为按 offset/rs1 动态计算
- lw/sw 的 uimm 位交织布局来自 RISC-V 规范, 不动
- placeholder 方法(lwsp/swsp/addi16sp)标记为未完整实现
- 解码器的位运算从 Spec v2.2 标准派生, 不偏离
- no_std: u16 位运算天然支持 no_std; 唯一 alloc 是 String(format!)用于调试输出

// hunyuan-rs/src/encoding/isa16.rs
