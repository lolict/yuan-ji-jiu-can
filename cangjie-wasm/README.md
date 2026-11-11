# cangjie-wasm

CHIR（Cangjie High-level IR）→ wasm32 后端，M0–M2 整数子集。

- 输入：`cjc --dump-chir --dump-to-screen`，或 `--emit-chir=opt` + `chir-dis`
- 输出：`.wasm`，导出 `main`（同时导出其余 `_CN7default*` 函数，便于单函数探测）
- 验收：11 个样例覆盖 `if`、递归、`while`、`while(true)+break`、`for-in`、`continue`、嵌套循环、`Unit`/`Bool` 返回

## 构建与运行

```bash
# 构建 CHIR→wasm 后端
make

# 一条命令：Cangjie 源 → CHIR → wasm → wasmtime
./cjwasm run tests/fact.cj

# 调用任意导出函数
./cjwasm invoke _CN7default5isPosHl tests/boolret.cj 7

# 只生成 wasm，不运行
./cjwasm build tests/loop.cj

# 从 Cangjie 源完整重跑（期望 7 / 0 / 120 / 45 / 10 / 6 / 42 / 55 / 25 / 1 / 9）
make test
```

`make test` 会调用 `tests/run.sh`：先用 `cjc` 生成 CHIR，再转 wasm，最后用
`wasmtime` 断言结果。

## 已支持

| 类别 | CHIR op |
|---|---|
| 存储 | `Allocate`（`[ret]` 与可变槽）、`Store`、`Load` |
| 常量/算术 | `Constant`、`Add`、`Sub`、`Mul`、`Div`、`Mod` |
| 比较 | `EQ`、`NE`、`LT`、`GT`、`LE`、`GE`（i64 有符号） |
| 调用 | `Apply`（`_CN7default*` 内部调用） |
| 控制流 | `Branch`（if/else）、`GoTo`、`while`、`while(true)+break`、`for-in`、`continue`、嵌套循环 |
| 返回 | `Int64`、`Bool`、`Unit`（`noop(): Unit` 不返回值） |
| 其他 | `Debug`（丢弃）、`TypeCast`（等宽恒等） |

## 两种控制流发射模式

默认是通用 CFG 调度器：每个块放进 `loop` + `block` 嵌套，用 `br_table` 按状态跳转，
因此 `for-in` 降级出的多前驱块也能跑。

```bash
# 结构化模式（简单 if/else、while、自环），用于对照
CHIR_STRUCTURED=1 ./chir_wasm build/loop.chirtxt build/loop.wasm
```

## while 的结构化落法（CHIR_STRUCTURED=1）

```
block $exit
  loop $head
    A...                  ; 头块，跳过末尾 Branch
    cond; i32.eqz; br_if 1
    B...                  ; 循环体，GoTo $head 不单独发射
    br 0
  end
end
C...                      ; 出口块必须在 block 之后
```

出口代码放在 `block` 之后：`br_if 1` 跳到 `block` 的 end，若把 C 放在
`block` 内、`end` 之前，会被整段跳过（M1 踩过的坑）。

## 已知边界

- `Overflow: THROWING` 仍按 `i64.add` 回绕，溢出语义待 kndoyle 定。
- 通用调度器支持任意 CFG，但每块都经 `br_table` 调度，性能不是 M2 目标。
- `Constant(null)` 暂按 0 处理；`for-in` 样例不依赖它的哨兵语义。
- 浮点、字符串、GC、`addrspace(1)`、异常表不在范围内。
- 只解析 `default` 包整数子集，不做完整 CHIR 语法校验。
