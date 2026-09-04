# ISA 编码层 · RISC-V RV32I 编码器/解码器

> 创建: 2026-09-04 · 工具编号: hunyuan-rs/src/isa.rs
> 算法参考: RISC-V Volume I Unprivileged ISA 20191213

---

## 1. 设计目标

实现 "最小可运行原型"：
- 覆盖 RISC-V RV32I 全部 37 条基础整数指令中的 32 条常用形式
- 编码: 将汇编字符串转为 32 位机器码
- 解码: 将 32 位机器码转为可读汇编字符串
- 测试: 全量交叉验证,机器码权威来自 RISC-V 官方手册

## 2. 16 核心指令 (DeepSeek 推荐的 Phase 0 最小集)

| 指令 | 操作 | 编码类型 |
|------|------|----------|
| add | 寄存器加法 | R-type |
| sub | 寄存器减法 | R-type |
| and | 按位与 | R-type |
| or | 按位或 | R-type |
| xor | 按位异或 | R-type |
| sll | 逻辑左移 | R-type |
| srl | 逻辑右移 | R-type |
| sra | 算术右移 | R-type |
| slt | 有符号比较 | R-type |
| sltu | 无符号比较 | R-type |
| addi | 立即数加法 | I-type |
| andi | 立即数与 | I-type |
| ori | 立即数或 | I-type |
| xori | 立即数异或 | I-type |
| lw | 加载字 | I-type |
| sw | 存储字 | S-type |
| beq | 相等分支 | B-type |
| lui | 高位立即数 | U-type |
| auipc | PC + 高位立即数 | U-type |
| jal | 跳转并链接 | J-type |
| jalr | 寄存器跳转 | I-type |
| ecall | 系统调用 | I-type |
| ebreak | 断点 | I-type |

## 3. 编码约定

- 编码后为小端 4 字节 (定长 32 位)
- 符号扩展: 12 位立即数扩展为 32 位有符号数
- 分支/跳转偏移量为 2 的倍数 (RISC-V 指令对齐 2 字节)

## 4. 交叉校验

每个编码结果与 `tools/isa-ref/riscv.md` 文档中的编码对比。
见 `src/isa.rs::tests::against_isa_ref_doc`。

## 5. 用法

```shell
cargo run --bin isa-encode -- encode "add x1, x2, x3"
# => 003100b3

cargo run --bin isa-encode -- decode 003100b3
# => add x1, x2, x3

cargo run --bin isa-encode -- demo

cargo run --bin isa-encode -- test
```

## 6. 下一步

- [ ] 实现 RV64I 扩展 (LD/SD/ADDW 等)
- [ ] 实现 M 扩展 (乘除法)
- [ ] 实现 A 扩展 (原子操作)
- [ ] 实现 C 扩展 (压缩指令 16 位)
- [ ] 与字体封装工具联动: 符号→机器码→字体码点映射
- [ ] 添加 x86-64/ARM64/MIPS 编码表, 按功能对齐到同一语义ID

END OF SPEC
