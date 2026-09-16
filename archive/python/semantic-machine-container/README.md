# semantic-machine-code — 语义机器码容器

## 容器定位

**父容器**：`tools/`
**容器家族**：编码族 → 语义机器码容器
**公约数**：零维静态点 = 单一语义（如"加"、"存"、"取"、"跳"）
**公倍数**：一维动态线 = 同一语义在不同架构（ARM/x86/R/MIPS）上的投影

**容器职责**：
- 为各架构（ARM64/x86-64/RV32I/RV64I/MIPS32）的"同一功能"建立唯一的语义 ID
- 将语义 ID 与具体机器码双向映射
- 把映射关系封装为可被程序读取的自定义字体文件（MCOD 表）
- 所有数据预加载存储，查询时无运行时计算（冰箱模型）

## 容器结构

```
semantic-machine-code/
├── README.md              # 本文件（容器自我声明）
├── semantic_ids/          # 语义 ID 字典（容器族）
│   ├── arithmetic.json    # 算术语义族
│   ├── logic.json         # 逻辑语义族
│   ├── memory.json        # 访存语义族
│   ├── branch.json        # 分支语义族
│   └── move.json          # 数据移动语义族
├── mappings/              # 架构投影（公倍数）
│   ├── rv32i.json         # RISC-V 32 位投影
│   ├── arm64.json         # ARM64 投影
│   ├── x86_64.json        # x86-64 投影
│   └── mips32.json        # MIPS32 投影
├── fonts/                 # 封装输出
│   └── semantic_code.ttf  # 含 MCOD 表的字体容器
├── tools/                 # 转换工具
│   ├── encode.py          # 语义 → 机器码
│   └── decode.py          # 机器码 → 语义
└── tests/                 # 闭合验证
    ├── test_encode.py
    └── test_decode.py
```

## 设计原则（满全法约束）

| 原则 | 实现 |
|------|------|
| 无运行时 | 所有映射预生成 JSON/二进制，启动即加载 |
| 无外部依赖 | 纯 Python 标准库 + fontTools |
| 预加载冰箱 | 字体文件即数据库，打开即查询 |
| 三元裁判 | encode/decode 双向可逆（正、反、裁判 = 双向验证） |
| 产权归属 | 每条记录标注来源手册与校验状态 |
| 容器封装 | 字体文件 = 容器，字形 = 符号，MCOD 表 = 内涵 |

## 使用方法（预加载式）

```python
from semantic_code import SemanticMachineCode

smc = SemanticMachineCode.load("fonts/semantic_code.ttf")

# 正向：语义 → RV32I 机器码
code = smc.encode("ADD", "x1", "x2", "x3")
# → 0x003100B3

# 反向：机器码 → 语义
inst = smc.decode("rv32i", 0x003100B3)
# → {"semantic": "ADD", "operands": ["x1","x2","x3"]}
```

无编译等待，无运行时计算，加载即使用——这就是冰箱：菜已炒好，点菜直接端。

*容器落款：刘楚恬和我 · 私有编码系统*
