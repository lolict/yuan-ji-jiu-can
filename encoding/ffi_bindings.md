# FFI 绑定说明

> 符号表编译为二进制后，给 Zig / Nim / Rust 通过FFI调用。
> 功能固定，符号可换。

## 编译产物

| 文件 | 用途 |
|------|------|
| `libsymbol_table.so` | Linux共享库 |
| `libsymbol_table.a` | 静态库（嵌入到任何程序） |
| `symbol_table.wasm` | WASM模块（给任何语言通过WASI调用） |

## Zig 绑定

```zig
const c = @cImport({
    @cInclude("symbol_table.h");
});

pub fn main() void {
    const 地址 = c.查地址("b");
    std.debug.print("b → 地址{}\n", .{地址});

    const 符号 = c.查符号(64);
    std.debug.print("地址64 → {s}\n", .{符号});

    const 是否存在 = c.存在("袋");
    std.debug.print("袋存在: {}\n", .{是否存在});
}
```

编译命令：
```bash
zig build-lib main.zig -L. -lsymbol_table
```

## Nim 绑定

```nim
{.passL: "-L. -lsymbol_table".}
{.pragma, header: "symbol_table.h".}

proc 查地址(符号: cstring): cint {.importc.}
proc 查符号(地址: cint): cstring {.importc.}
proc 存在(符号: cstring): cint {.importc.}
proc 符号数量(): cint {.importc.}

echo "b → 地址", 查地址("b")
echo "地址64 → ", 查符号(64)
echo "袋存在: ", 存在("袋")
echo "符号总数: ", 符号数量()
```

编译命令：
```bash
nim c --passL:"-L. -lsymbol_table" main.nim
```

## Rust 绑定

```rust
extern "C" {
    fn 查地址(符号: *const i8) -> i32;
    fn 查符号(地址: i32) -> *const i8;
    fn 存在(符号: *const i8) -> i32;
    fn 符号数量() -> i32;
}

fn main() {
    unsafe {
        let 地址 = 查地址("b\0".as_ptr());
        println!("b → 地址{}", 地址);

        let 符号 = 查符号(64);
        let 符号_str = std::ffi::CStr::from_ptr(符号);
        println!("地址64 → {:?}", 符号_str);

        let 是否存在 = 存在("袋\0".as_ptr());
        println!("袋存在: {}", 是否存在);
    }
}
```

编译命令：
```bash
rustc main.rs -L. -lsymbol_table
```

## WASM 调用

任何支持WASI的语言都可以加载 `symbol_table.wasm`：

```javascript
// JavaScript (Node.js)
const fs = require('fs');
const wasm = fs.readFileSync('symbol_table.wasm');
const instance = await WebAssembly.instantiate(wasm, {
    wasi_snapshot_preview1: { proc_exit: () => {} }
});
const { 查地址, 查符号, 存在, 符号数量 } = instance.exports;
```

## 后发编程：替换符号

符号表编译成二进制后，可以通过以下方式替换符号：

1. **重新编译**：修改 `symbol_table.c` 中的符号数组，重新编译
2. **运行时补丁**：通过内存修改工具替换字符串
3. **配置文件**：把符号数据放到外部配置文件，运行时读取

**推荐方式**：把符号数据放到外部JSON文件，运行时读取。这样不需要重新编译。
