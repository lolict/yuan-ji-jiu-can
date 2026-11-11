# M0 归属表：CHIR → wasm32 整数子集

每条 op 的主责层只有一个。主层定结论；辅层只能在自己的边界内否决。

| CHIR op | gtxone（骨架） | kndoyle（推理） | mlil（表面） | heyair（横向） | gttcp（接口） | gkgndl（感知） |
|---|---|---|---|---|---|---|
| `Allocate(T)` + `[ret]` | 定槽宽与对齐；`Int64&` 只是引用，落成 i64 局部 | 决定返回槽是否可提升为局部 | 保留 `%N` 到局部名的可追溯映射 | — | — | 槽缺失时报错 |
| `Constant(v)` | 定常量类型宽度 | — | 记录 `%N` 名字 | `i64.const` / `i32.const`；`i` 后缀剥离 | 有符号 LEB128 | 解析失败即停 |
| `Add(a,b)` | 定 `Int64` 结果宽度 | `Overflow: THROWING` 的语义归这里（M0 暂用 `i64.add` 回绕，见下方缺口） | — | `0x7C` | — | — |
| `GT(a,b)` | 结果类型 `Bool` | 比较是有符号还是无符号归这里 | — | `i64.gt_s` → `0x55` | — | — |
| `Store(v, slot)` | 槽与值类型必须一致 | 存储语义（覆盖/初始化）归这里 | 槽名解析 | `local.set` | — | 槽不存在时报错 |
| `Apply(f, args)` | 调用约定：参数顺序、返回槽 | 内联/不内联由 kndoyle 决定 | 符号 `@_CN7default*` → 函数索引 | `call` | 函数索引编码 | 未定义 callee 时报错 |
| `Branch(c, T, E)` | 基本块与后继关系 | 分支合并、不可达块判定 | 块号 `#N` → 位置 | `if/else/end` | 块类型 `0x40` | 缺块时报错 |
| `Exit()` | 出口唯一 | 返回槽必须已定义 | 返回槽名解析 | 取返回值后 `return` | `0x0F` | 无返回槽时报错 |
| `Debug(...)` | — | — | 调试名只留在表面层 | 不发射 | — | 不参与验收 |
| `TypeCast(x)` | 源/目标同宽才允许 | 隐式转换策略归这里 | — | M0 按恒等处理 | — | 宽度不一致时报错 |

## 一条 `Add` 的完整走法

源：`func add(a: Int64, b: Int64): Int64 { return a + b }`

1. gtxone 确认两个 `Int64` 参数 + 一个 `Int64` 返回槽，块结构只有 `#0`。
2. kndoyle 看到 `Overflow: THROWING`，M0 记录为缺口，不假装已实现。
3. mlil 把参数落成 `%0`、`%1`，返回槽落成 `%4`，结果落成 `%5`。
4. heyair 把 `Add` 映射成 `i64.add`。
5. gttcp 写 Type/Function/Export/Code 四段，`main` 导出为 `"main"`。
6. gkgndl 只在 op 不认识、槽/块缺失、callee 未定义时终止。

## 验收

| 层 | 验收命令 | 通过条件 |
|---|---|---|
| gtxone | `wasmtime run --invoke main add.wasm` | 校验器不报类型错误 |
| kndoyle | 两个样例：`add(3,4)`、`add(1,2)` | 分别输出 `7`、`0` |
| mlil | `wasmtime run --invoke main` | 能找到导出 `main` |
| heyair | 反汇编/字节检查 | `Add→0x7C`、`GT→0x55`、`Branch→0x04/0x05` |
| gttcp | `ls -l add.wasm` | 模块可解析；M0 产物 122 字节 |
| gkgndl | 喂一个含未支持 op 的 CHIR | 非零退出并打印 op 名 |

## 已记录缺口

- `Overflow: THROWING`：M0 用 `i64.add` 回绕，溢出语义待 kndoyle 定（trap 还是 Cangjie 异常）。
- `Unit` 返回值函数：已支持声明，但尚未用样例验收。
- GC、`addrspace(1)`、异常表、字符串：不属于 M0，留给 M1+。
- `chir-dis` 的完整语法未覆盖，只解析 `default` 包内整数子集。

## M1 增补

| CHIR op / 结构 | 主责层 | 落法 | 验收 |
|---|---|---|---|
| `Load(%slot)` | heyair | `local.get` | `loop2` 返回 10 |
| `GoTo(#N)` | gtxone | 前向跳转内联；回边由 `emit_while` 发射 | `loop` 返回 45 |
| `LT/LE/GE/EQ/NE` | heyair | `i64.lt_s` 等有符号比较 | `loop` 条件成立 |
| `Sub/Mul/Div/Mod` | heyair | `0x7D/0x7E/0x7F/0x81` | 编译期 `-Wall` 无告警 |
| 可变 `Allocate`（`%3[s]`） | gtxone | 槽名去掉 `[s]` 后缀，落成独立 i64 局部 | `s`/`i` 读写正确 |
| 可归约 `while` | gtxone + gttcp | `block/loop` + `i32.eqz` + `br_if` | `loop`=45、`loop2`=10 |

### M1 边界教训

`br_if 1` 指向 `block` 的 end。出口块若写在 `block` 内、`end` 之前，会被整个跳过，
函数落到兜底 `i64.const 0`。这属于 gtxone 的块序问题，gttcp 只负责按块序编码，
gkgndl 用两个循环样例把它抓出来。

## M2 增补：通用 CFG 调度

`for-in` 会降级成多前驱块的 CFG，结构化发射不再够用。默认改为：

```
loop $dispatch
  block $b0 ... block $bN
    state; br_table $b0 ... $bN
  end $bN; <块 N 代码>; br (loop 深度 N)
  ...
  end $b0; <块 0 代码>; br 0
end
```

| 关注点 | 归属 | 结论 |
|---|---|---|
| 块序与 `br_table` 深度 | gtxone | 按块数组顺序嵌套，深度 = `n-1-index` |
| 状态局部变量 | mlil | 每函数追加一个 i32 `%state`，不暴露给 CHIR |
| 条件分支 | heyair | `if` 两臂各设 state 后 `br (k+1)`，随后 `unreachable` |
| `Unit` 返回 | gtxone | 函数类型无结果，`Exit` 只发 `return` |
| 空调用返回值 | gttcp | 被丢弃的 `Unit` 调用不补 `drop`，否则栈下溢 |

验收：8 个样例全部通过，其中 `sumrange`=55 证明 `for-in` 的多前驱 CFG 可执行。
