# WASM · 双向公约数策略规范

> 版本 v0.1 · 2026-09-10 · 配套 `FQMT_BINARY_FORMAT.md` `WASM_CORE_v17.md` `cross_platform_machine_code_alignment_v3.md`

---

## 1. 定义

双向公约数不是"跨平台编译",不是"抽象层",不是"虚拟机妥协"。

**双向公约数 = 一套 C 源码同时编译 wasm32 和 wasm64 两个目标文件,使用固定宽度类型前缀宏抹平架构差异,从而让任何裁决算法在 32 位 WASM 和 64 位 WASM 上结果完全一致。**

系统在两种运行模式下都被支持:
- **WASM 模式**: 在浏览器、WASM 运行时等沙盒环境中运行
- **原生模式**: 直接在裸机 CPU 上运行,使用 FQMT 二进制格式

关键在于——同样一份 C 源码编译产物不同但裁决算法可验证一致。

---

## 2. 目标

```
           同一份 C 源码
               │
      ┌────────┴─────────┐
      │                  │
  wasm32 目标          wasm64 目标
  (浏览器/沙盒)        (服务器/边缘)
      │                  │
  32 位裁决结果       64 位裁决结果
      │                  │
      └────────┬─────────┘
               │
      验证:结果比特位完全相同
```

一次编写,多种架构展开——裁决算法的行为跨架构保持**领域无关**(`hunyuan-methodology` 开发红线第 6 条:裁决引擎的领域无关性)。

---

## 3. 固定宽度类型系统

### 3.1 自有 typedef

```c
/*
 * njsg_types.h — 禁止直接使用 short/int/long
 *
 * 双向公约数系统的自有类型前缀:
 *   NJ  = NJSG = 满全法/NJSG 体系
 *   数字 = 位宽
 *   后缀 = 类型角色
 */

#include <stdint.h>

/* 基础类型 */
typedef uint8_t   NJu8;     /* 无符号 8 位(容器索引/极性子级) */
typedef int8_t    NJi8;     /* 有符号 8 位 */
typedef uint16_t  NJu16;    /* 无符号 16 位(版本号/容器块计数) */
typedef int16_t   NJi16;    /* 有符号 16 位 */
typedef uint32_t  NJu32;    /* 无符号 32 位(SMRL cell 编码的一部分) */
typedef int32_t   NJi32;    /* 有符号 32 位 */
typedef uint64_t  NJu64;    /* 无符号 64 位(完整 SMRL cell / 完整裁决状态) */
typedef int64_t   NJi64;    /* 有符号 64 位 */

/* 地址宽度:在 wasm32 上是 32 位,在 wasm64 上 / 裸机上由宏切换 */
#ifdef __wasm32__
  typedef NJu32 NJuaddr;
  typedef NJi32 NJiaddr;
#elif defined(__wasm64__)
  typedef NJu64 NJuaddr;
  typedef NJi64 NJiaddr;
#else
  /* 裸机:使用编译时可配置的 NJ_ADDR_WIDTH */
  #if NJ_ADDR_WIDTH == 32
    typedef NJu32 NJuaddr;
  #elif NJ_ADDR_WIDTH == 64
    typedef NJu64 NJuaddr;
  #endif
#endif

/* 布尔:裁判引擎用的布尔不是 C99 _Bool,是我们自己的枚举 */
typedef enum {
  NJ_FALSE = 0,
  NJ_TRUE  = 1
} NJBool;

/* 三值与四值逻辑扩展 */
typedef enum {
  NJ_NO       = 0,    /* 禁 */
  NJ_MAYBE    = 1,    /* 待/未决 */
  NJ_YES      = 2     /* 许可 */
} NJTri;

typedef enum {
  NJ_VOID       = 0,  /* 无 */
  NJ_PARTIAL    = 1,  /* 部分 */
  NJ_FULL       = 2,  /* 完整 */
  NJ_OVERFLOW   = 3   /* 溢出 */
} NJQuad;
```

### 3.2 宏前缀体系

```c
/*
 * 双向公约数宏前缀:
 *   NJ_API    = 导出给外部的函数(对 wasm32/wasm64 各有不同 ABI,但同名)
 *   NJ_INLINE = 内联辅助函数
 *   NJ_EXPORT = 需要导出到 wasm 模块外的宏
 *   NJ_IMPORT = 需要从 wasm 模块外导入的宏(是 NJ_EXPORT 的反面)
 *   NJ_PACKED = 紧凑结构体(__attribute__((packed)) 的跨平台一致)
 */

#ifdef __wasm__
  #define NJ_EXPORT __attribute__((export_name))
  #define NJ_IMPORT __attribute__((import_module))
#else
  #define NJ_EXPORT __declspec(dllexport)  /* 或 __attribute__((visibility("default"))) */
  #define NJ_IMPORT
#endif

#define NJ_INLINE static inline
#define NJ_PACKED __attribute__((packed))
```

---

## 4. 同一份裁决源码示例

```c
/*
 * njsg_verdict.c — 七元裁决算法
 *
 * 这个文件同时被编译为:
 *   · njsg_verdict.wasm32.wasm (32 位浏览器沙盒)
 *   · njsg_verdict.wasm64.wasm (64 位服务器 WASM 运行时)
 *   · njsg_verdict_native.o    (原生平台目标文件)
 *
 * 三种目标的编译产物不同,但对相同输入必定输出相同裁决。
 */

#include "njsg_types.h"

/* 五元推进 */
NJ_EXPORT NJTri njsg_five_yuan_push(
    NJu32 id,
    NJi32 angle_fixed,    /* 角度 × 1000,定点小数避免浮点不跨架构一致 */
    NJi32 force_fixed     /* 力 × 1000 */
){
    if(id >= 254){
        return NJ_NO;     /* id 越界:禁行 */
    }
    /* 五元算法核心:使用固定位宽运算,避免 32/64 位隐式提升的差异 */
    NJi64 torque = ((NJi64)angle_fixed * (NJi64)force_fixed) / 1000;
    NJu32 slot = (NJu32)(torque % 251);
    if(slot < 254){
        return NJ_YES;    /* 找到有效容器槽 → 许可 */
    }
    return NJ_NO;
}

/* 七元蜂窝套壳 */
NJ_EXPORT NJBool njsg_shell_nest_wrap(
    const NJu8 *cell_encoded,  /* 指向 8 字节 SMRL cell(大端序 NJu64) */
    NJu32 wrap_level           /* 包裹层级:0 = 不包裹,6 = 最大 */
){
    /* NJu64 在 32 位和 64 位 WASM 上都是 8 字节。 */
    /* 大端序统一,不受主机 CPU 字节序影响。 */
    NJu64 cell = 0;
    for(int i = 0; i < 8; i++){
        cell |= ((NJu64)cell_encoded[i]) << (56 - i*8);
    }
    
    /* 七元运算:cell 的层级决定是否还能被进一步包裹 */
    NJu8 level = (NJu8)((cell >> 32) & 0xFF);
    if(level > 7){
        return NJ_FALSE;  /* 超出层级上限 */
    }
    
    return (level + wrap_level <= 7) ? NJ_TRUE : NJ_FALSE;
}
```

关键点:**没有浮点数,只用定点小数(× 1000 的整数)**——这确保 32 位 WASM 的 32 位运算和 64 位 WASM 的 64 位运算在裁决算法中产出比特位相同的结果。

---

## 5. 两目标同时编译

### 5.1 构建系统

```makefile
# Makefile — 双目标同时编译同一份 C 源码

CC_WASM32 = clang --target=wasm32 -mbulk-memory
CC_WASM64 = clang --target=wasm64 -mbulk-memory
CC_NATIVE = cc  # 主机编译器

SRCS = njsg_verdict.c njsg_types.c njsg_judge.c

# wasm32 目标 (浏览器沙盒)
njsg32.wasm: $(SRCS)
	$(CC_WASM32) -O2 -DNJ_ADDR_WIDTH=32 -o $@ $(SRCS)

# wasm64 目标 (服务器运行时)
njsg64.wasm: $(SRCS)
	$(CC_WASM64) -O2 -DNJ_ADDR_WIDTH=64 -o $@ $(SRCS)

# 原生目标 (裸机)
njsg_native.o: $(SRCS)
	$(CC_NATIVE) -O2 -DNJ_ADDR_WIDTH=64 -o $@ $(SRCS)

all: njsg32.wasm njsg64.wasm njsg_native.o
```

### 5.2 一致性的验证方式

```bash
# 验证:对同一输入,32 位和 64 位 WASM 产出比特位相同
echo '{"id":127,"angle":45000,"force":1000}' | wasmtime run njsg32.wasm > out32.json
echo '{"id":127,"angle":45000,"force":1000}' | wasmtime run njsg64.wasm > out64.json
diff out32.json out64.json
# 期望:无差异(比特位完全相同)

# 如果有浮点数参与运算,这一步通常会失败;使用定点小数是设计上的保证
```

---

## 6. WASM 原生适配对照

### 6.1 裁决对照表

| 裁决操作 | wasm32 | wasm64 | 原生 x86-64 | 原生 AArch64 | 原生 RISC-V |
|----------|--------|--------|-------------|-------------|------------|
| SMRL cell 编码 | 8 字节手动组合 | 直接 64 位 | 直接 64 位 | 直接 64 位 | 直接 64 位 |
| 浮点运算 | 禁用 | 禁用 | 禁用 | 禁用 | 禁用 |
| 定点运算(int64) | OK:(i64) | OK:native | OK:native | OK:native | OK:native |
| 运算结果 | 强制 64 位 | 强制 64 位 | 强制 64 位 | 强制 64 位 | 强制 64 位 |

裁决算法不使用任何浮点数——所有坐标、角度、力都定点化——这是跨架构一致性的根本保证。

### 6.2 边界处:WASM 与 JS 交互

浏览器中 WASM 模块与 JS 之间的数据传递:

```
JS 侧传参:
  const input = new Uint8Array([0x46, 0x51, 0x4D, 0x54, ...])  // FQMT 二进制片段
  const ptr = wasmModule._malloc(input.length)
  wasmModule.heapU8.set(input, ptr)
  const result = wasmModule.njsg_judge_wrap(ptr, input.length)
  wasmModule._free(ptr)

WASM 侧导出:
  NJ_EXPORT int njsg_judge_wrap(NJuaddr ptr, NJu32 len){
    const NJu8 *bytes = (const NJu8*)(uintptr_t)ptr;
    return njsg_judge(bytes, len);
  }
```

FQMT 字节流在边界上永远以 **Uint8Array / 字节指针** 形式传递——不在 WASM/JS 边界做 JSON 解析。JSON 解析交给外表面层的蚕蛹壳内核,不在裁决算法内部。

---

## 7. 运行模式选择

### 7.1 根据环境选择

```
运行时选择逻辑:

  if 环境支持直接运行原生二进制:
      使用 NJ qcom_native.o 编译的产物
      (直接 EXEC,无 WASM 性能开销)
      
  else if 环境支持 wasmtime / wasmer / WasmEdge:
      使用 njsg64.wasm (64 位 WASM 运行时)
      
  else if 环境是浏览器:
      使用 njsg32.wasm (32 位 WASM,浏览器标准)
      
  else if 环境是嵌入式 MCU (无 WASM, 无完整 libc):
      提供 njsg_core_minimal.c 最简子集
      (仅含 SMRL 编码 + 五元裁定, 无 WASM 依赖)
      
  else:
      报错:不支持此环境
```

### 7.2 嵌入式 MCU 最简模式

```c
/*
 * njsg_core_minimal.c — 裁定的最小可运行子集
 *
 * 功能限制:
 *   · 不做完整的七元裁决(复杂度太高)
 *   · 只做五元推进 + 三裁判(极性 / 层级 / 主权)
 *   · 不做 FQMT 文件读写(内存中直接操作)
 *   · 用静态数组替代 malloc
 *   · 无文件 I/O,无打印,纯算法
 */

#include <stdint.h>

#define NJ_MIN_SLOTS 254

typedef struct {
    uint64_t slot[NJ_MIN_SLOTS];
    uint8_t  view[NJ_MIN_SLOTS];  /* 0=未声明,1=有,2=无,3=空质 */
} nj_min_box_t;

void nj_min_init(nj_min_box_t *box){ /* ... */ }
int  nj_min_judge(nj_min_box_t *box, uint32_t candidate){ /* ... */ }
```

这套最小子集(约 400 行 C)可以在 ARM Cortex-M0+ / ESP32 / RISC-V MCU 上运行,没有 libc 依赖,没有 OS 依赖。

---

## 8. 跨架构一致性的技术原理

### 8.1 定点化消灭浮点差异

C 标准规定 `float` / `double` 的舍入模式可以通过 `#pragma STDC FENV_ACCESS ON` 改变;不同编译器、不同优化级别、不同架构的浮点结果可能末位不同。

抉择:**满全法体系不使用任何浮点数。**

所有需要小数参与的场景用定点:
- 角度: × 1000 (0.001° 精度)
- 力/权重: × 1000
- surplus: × (整数部分 + 3 位小数位)
- CRC 校验和: 整数

### 8.2 统一大端序

所有跨 WASM-JS、跨 WASM-FQMT、跨 32-64 位边界的数据都用大端序存储。主机是小端序的 x86 / ARM 则在写入和读取时做一次字节序转换——转换函数在 NJu64 读写内联函数中封装。

### 8.3 显式禁止隐式整型提升

```c
/* 禁止: */
int a = force * angle / 1000;  /* force/angle 在 16 位平台可能 int 提升为 unsigned */

/* 强制: */
NJi64 torque = ((NJi64)force_fixed) * ((NJi64)angle_fixed) / 1000;
```

---

## 9. 不变量

1. **定点化**:整个裁决算法不使用 float / double。所有定点系数 × 1000(保留 3 位小数),以 NJi64 承载中间结果。
2. **大端序存储**:所有跨边界(SMRL cell、FQMT 文件、WASM/JS 边界)的多字节整数用大端序。主机的小端序由 NJu_read / NJu_write 内联函数透明转换。
3. **显式位宽**:禁止使用 short/int/long / size_t——这些类型的位宽跨架构不同。NJ_ 前缀替代所有原生 C 类型。
4. **三字宏前缀一致**:所有导出给外部的 API 函数同名同语义,编译到 wasm32 和 wasm64 后名字相同。
5. **跨架构裁决一致性**:相同输入在 wasm32、wasm64、native x86-64、native AArch64、native RISC-V 上的裁决结果**确定性匹配**——不是概率一致,是比特位完全一致。
6. **WASM 模式与 native 模式的裁决路径相同**:不走两套算法——同一份 .c 文件,同一套函数体。唯一的差异是编译目标的字节码格式,不是算法语义。

---

*本文件定义 WASM 双向公约数策略的规范。配合 FQMT_BINARY_FORMAT.md(文件层的跨平台一致性)、WASM_CORE_v17.md(WASM 内核的历史实现)、cross_platform_machine_code_alignment_v3.md(跨架构机器码对齐的底层机制)。核心实践:同一份 C 源码 + 两套编译 → 同裁决结果 → 验证系统确保实现不偏离。*
