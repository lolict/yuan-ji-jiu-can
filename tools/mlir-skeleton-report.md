# MLIR 18.1.3 Skeleton — API 结构速查

> 来源: $HOME/mlir-18.1.3/ (apt-get download from Ubuntu 24.04 noble-updates)
> 提取: libmlir-18, libmlir-18-dev, llvm-18-dev, llvm-18-tools

---

## 1. IR 核心子系统 (IR/, 65 headers)

| 子系统 | 著名文件 | 用途 |
|---|---|---|
| 基础定义 | Operation.h, Block.h, Region.h, Value.h, Unit.h | Op/Block/Region/Value 树 |
| 类型系统 | Types.h, TypeSupport.h, BuiltinTypes.h | Integer/Index/Float/Tensor/Vector/MemRef |
| 属性系统 | Attributes.h, AttributeSupport.h, BuiltinAttributes.h | AffineMap/IntegerSet/String/Array/Dictionary |
| Dialect 注册 | Dialect.h, DialectRegistry.h, DialectInterface.h | 动态注册 Dialect |
| 上下文 | MLIRContext.h, Threading.h | 全局上下文 + 线程安全 |
| 构建器 | Builders.h, ImplicitLocOpBuilder.h | Op/Type/Attr 工厂 |
| Op 定义 | OpDefinition.h, OpDefinitionGen.h, ODSSupport.h | ODS/TableGen 支持 |
| 模式匹配 | PatternMatch.h | 声明式 rewrite |
| IR 映射 | IRMapping.h | Op/Value 替换映射 |
| 符号表 | SymbolTable.h | FuncOp 内 Symbol 查找 |
| 访问者 | Visitors.h, AffineExprVisitor.h | 类型-driven 遍历 |
| 诊断 | Diagnostics.h | Diag/Note/Remark |
| 验证 | Verifier.h | Op/Attr/Type 约束 verify |
| 迭代器 | Iterators.h, UseDefLists.h, ValueRange.h, TypeRange.h | SSA 链 |
| 内置 Dialect | BuiltinDialect.h, BuiltinOps.h | Module/Func/Block/Terminator |

### 关键类层级
```
MLIRContext (全局)
  └── DialectRegistry
        └── Dialect (AbstractOperation for each registered dialect)
              └── Op (OpState: name, attrs, operands, regions, results)
                    ├── Region (Block list)
                    │     └── Block (Op list)
                    │           └── Op (circular)
                    ├── Type (AbstractType)
                    ├── Value (SSA: OpResult or BlockArg)
                    └── Attribute (AbstractAttr)
```

---

## 2. Dialect 清单 (Dialect/, 39 registrations)

生产级 20 个, 实验级 19 个:

### 核心 Control Flow
- **Affine** — 仿射循环嵌套优化 (多面体模型)
- **SCF** — Structured Control Flow (for/if/while)
- **Func** (func) — 函数定义/调用
- **ControlFlow** — cf.br/cf.cond_br/cf.switch

### 数据类型
- **Arith** — 整数/浮点算术
- **Math** — 超越函数 (sin/log/exp/pow)
- **Complex** — 复数
- **Index** — target-independent index 类型
- **Quant** — 量化运算 (uniform/non-uniform)
- **MLProgram** — 顶层 unknown op 容器 (外部函数调用)

### Memory
- **MemRef** — 连续内存的 view (多维)
- **Tensor** — 不可值张量
- **SparseTensor** — 稀疏格式 (level-based)
- **GPU** — kernel launch/thread/block hierarchy
- **LLVMIR** — LLVM 方言桥接

### ML 高层
- **Linalg** — named ops to loops (matmul/conv/reduce)
- **Vector** — SIMD 向量操作
- **Shape** — tensor shape 计算 (编译时)

### 硬件/后端
- **SPIR-V** — Vulkan/OpenCL 计算着色器
- **NVGPU** — NVIDIA GPU 指令
- **AMDGPU** — AMD GPU 指令
- **AMX** — Intel AMX 矩阵加速
- **armNeon / ArmSME / ArmSVE** — ARM SIMD/SVE/SME
- **X86Vector** — x86 SSE/AVX/AVX-512

### Transform/Special
- **Transform** — transform dialect (schedule transformations)
- **Bufferization** — tensor→memref 降级
- **PDL / PDLInterp** — pattern description language
- **IRDL** — IR definition language (verifier 生成)
- **Mesh** — 分布式 tensor mesh
- **DLTI** — Device Level Transformation Interface
- **EmitC** — C emitter

### 领域专用
- **OpenMP** — OpenMP pragma ops
- **OpenACC / OpenACCMP** — accelerator directives
- **Tosa** — TensorFlow Operators for Standard Architecture
- **Async** — async/fiber ops
- **Mesh** — distributed mesh

---

## 3. Pass 系统 (Transforms/, Pass/, Rewrite/)

| Pass 类型 | 文件 |
|---|---|
| Greedy Rewriter | Transforms/GreedyPatternRewriteDriver.h |
| CSE | Transforms/CSE.h |
| Inlining | Transforms/InliningUtils.h |
| Mem2Reg 风格 | Transforms/Mem2Reg.h |
| SROA | Transforms/SROA.h |
| Dialect Conversion | Transforms/DialectConversion.h |
| Fold Utils | Transforms/FoldUtils.h |
| Canonicalization | (集中式 per-dialect) |
| Loop Hoist | Transforms/LoopInvariantCodeMotionUtils.h |
| CFG→SCF | Transforms/CFGToSCF.h |
| All Pass Registration | Transforms/Passes.h |
| 运行时 pass 反射 | Query/PassQuery.h |

核心入口模式:
```cpp
// 1. 建 context + registry
MLIRContext ctx;
ctx.loadDialect<mlir::func::FuncDialect>();

// 2. 建 PassManager
PassManager pm(&ctx);
pm.addPass(mlir::createCanonicalizerPass());
pm.addPass(convertLinalgToLoopsPass());

// 3. 跑
pm.run(module);
```

---

## 4. Conversion/ 目录结构 — Dialect 间转换通道

```
Conversion/
├── AffineToStandard/        Affine → SCF/MemRef
├── ArithToLLVM/             Arith → LLVM
├── ArithToSPIRV/            Arith → SPIRV
├── Bufferization/           Tensor → MemRef buffer
├── ComplexToLibm/           → libm calls
├── ControlFlowToSPIRV/      → SPIRV branch
├── FuncToLLVM/              → LLVM function
├── GPUToNVVM/               → NVVM (CUDA)
├── GPUToROCDL/              → ROCm
├── GPUToSPIRV/              → Vulkan/OpenCL
├── IndexToLLVM/             Index → LLVM
├── LinalgToLLVM/            → LLVM loops
├── LinalgToSPIRV/           → SPIRV loops
├── MathToLLVM/              → LLVM math intrin
├── MathToSPIRV/             → SPIRV math
├── MemRefToLLVM/            → LLVM ptr
├── MemRefToSPIRV/           → SPIRV variable
├── OneToNTypeConversion/    → generic 1:N
├── PDLToPDLInterp/          pattern → interpreter
├── ReconcileUnrealizedCasts/<-> unrealized_conversion_cast
├── SCFToGPU/                loops → GPU kernels
├── SCFToSPIRV/              loops → SPIRV
├── ShapeToStandard/         Shape → Index/Math
├── SparseTensorToLLVM/      → 降成 Sparse runtime calls
├── TensorToLinalg/          high-level named
├── TensorToSPIRV/           → SPIRV
├── TosaToArith/             TOSA → Arith
├── TosaToLinalg/            TOSA → Linalg
├── TosaToSCF/               TOSA → SCF
├── VectorToGPU/             → GPU SIMD
├── VectorToLLVM/            → LLVM <N x T>
├── VectorToSCF/             → SCF for loops
├── X86VectorToLLVM/         → LLVM x86 SSE/AVX
└── convert*.h (disabled/aux)
```

---

## 5. Interface 系统 (Interfaces/)

Dialect-independent 行为约定，用宏声明, TableGen 展开:

| Interface | 作用 |
|---|---|
| CallInterfaces.h | Call/Callable op 规范 |
| CastInterfaces.h | Unrealized cast |
| ControlFlowInterfaces.h | branch/return/region terminator |
| CopyOpInterfaces.h | copy semantic |
| DataLayoutInterfaces.h | type ↔ width 查询 |
| DestinationStyleOpInterface.h | copy dest |
| FoldInterfaces.h | constant fold |
| InferIntRangeInterface.h | 整数范围推断 |
| InferTypeOpInterface.h | 结果类型推断 |
| LoopLikeInterface.h | for/while 通用契约 |
| ParallelCombiningOpInterface.h | work reduction |
| ShapeInterfaces.h | shape 计算 |
| ShapedOpInterfaces.h | ranked shape |
| SideEffectInterfaces.h | read/write/allocation effects |
| TilingInterface.h | loop tiling |
| VectorInterfaces.h | vector 拆分 |
| ViewLikeInterface.h | subview/collapse |

典型 pattern:
```cpp
struct MyOpInterface : OpInterface::Trait<OpTy> {
  // 这里定义 interface method
  void myMethod();
};
```

---

## 6. C Binding CAPI/)

C API 完整封装，每子系统一个 .h:
- CAPI/IR.h — Context/Dialect/Module/Op register
- CAPI/Pass.h — Pass manager
- CAPI/Interfaces.h
- CAPI/AffineExpr.h, AffineMap.h, IntegerSet.h
- CAPI/ExecutionEngine.h — JIT
- CAPI/AffineMap.h, AffineExpr.h — Polyhedral
- CAPI/Wrap.h — C++→C 桥

用途: Python binding / SWIG / FFI 入口

---

## 7. ExecutionEngine/ — JIT 路径

ExecutionEngine.h + Wrapper 将已 register 的动态库链接进运行时代码，通过 `main`/`_mlir_ciface_main` entry 调用

---

## 8. 二进制裁乱 .so 清单

/usr/lib/llvm-18/lib/
├── libMLIR.so.18.1              ← 主库, 4728 symbols
├── libmlir_async_runtime.so.18.1
├── libmlir_c_runner_utils.so.18.1
├── libmlir_float16_utils.so.18.1
├── libmlir_arm_runner_utils.so.18.1
├── libmlir_arm_sme_abi_stubs.so.18.1
├── libmlir_runner_utils.so.18.1
└── libRemarks.so.18.1           ← 可独立静态链接

---

## 9. 与满全法容器系统的对照

| 满全法概念 | MLIR 映射 | 备注 |
|---|---|---|
| 64位单元格坐标 | Index/Location | MLIR Index 随 target 变宽 |
| 极性 (阴/阳) | SideEffectInterfaces | read/write/none 三态 |
| 主权 | Ownership SSA | value 一遍定一生 |
| 价值 (去壳量化) | Arith 纯语义 | strip away literal suffix |
| 三元 witness | Op+Region+Block | Op 以 region 作 witness 嵌套 |
| 仲裁准入 | Verifier | Op/Type/Attr verify 函数 |
| Dialect | Dialect | 满每法每"进制"即一个 Dialect |
| Conversion Pass | DialectConversion | 容器→容器的合法映射 |
| PassManager 裁/判流水线 | PassManager | Pass = 满全法一"判"动作 |
| Rewrite Driver | GreedyPatternRewriteDriver | Pattern Match + 阴→阳替换 |
| DialectResourceBlobManager | Value≅符号资源流 | 去重 + 延迟加载 |

---

*Generated by skeleton scan 2026-09-27*
