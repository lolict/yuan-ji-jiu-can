# SMRL 引擎有/无协议实现规范

> 版本 v0.1 · 2026-09-10 · 配套 `smrl_engine.c/h` `YOUWU_MIRROR_FRAMEWORK.md`

---

## 1. 目标

让现有的 SMRL 64 位 cell 引擎原生支持有/无站立协议、空质壳标记和调包深度跟踪,不破坏原有 64 位编码结构,不破坏现有裁判逻辑。

---

## 2. 现有结构回顾

### 2.1 当前 64 位 cell 布局(来自 smrl_engine.c)

```
[63:56] coord       静态绝对坐标(0-255)
[55:48] polarity    极性(阴阳盈虚表里...)
[47:40] sovereignty 主权(0=无主, 255=全主)
[39:32] value       内在量化值(0-255)
[31:24] level       层级(元/阶/层/代/世/界/域/场)
[23:16] chain_id    链标识
[15:0]  version     版本号(65536 个)
```

### 2.2 核心函数签名

```c
uint64_t smrl_encode(uint8_t coord, uint8_t polarity, uint8_t sovereignty,
                     uint8_t value, uint8_t level, uint8_t chain_id, uint16_t version);
cell_t    smrl_decode(uint64_t code);
int       smrl_judge(cell_t candidate, cell_t existing);  // 三元裁判
```

### 2.3 现有裁判逻辑

三裁 AND:
1. 极性相容: `(a.polarity & b.polarity) != 0 || a.polarity==0 || b.polarity==0`
2. 层级兼容: `candidate.level >= existing.level`
3. 主权不侵: `candidate.sovereignty==0 || existing.sovereignty==0 || a.sovereignty < b.sovereignty`

---

## 3. 有/无状态编码方案

### 3.1 方案选择

不修改 64 位主结构,将新增信息编码进现有字段的**未充分使用的位域**中。

分析当前字段利用率:

| 字段 | 宽度 | 实际使用 | 冗余 |
|------|------|----------|------|
| coord | 8bit | 仅用 0-255 槽位 | 无(256 刚好对应 254+2) |
| polarity | 8bit | 虽命名极性,实际存的是"层级×32" | **6 bit 冗余**: 仅用 0,32,64,96,128,160,192,224 |
| sovereignty | 8bit | 0/128/255 三值 | **6 bit 冗余**: 仅用 3 个值 |
| value | 8bit | 满用 | 无 |
| level | 8bit | 0-7 共 8 层级 | **5 bit 冗余**: 仅用低 3 bit |
| chain_id | 8bit | 满用 | 无 |
| version | 16bit | 满用 | 无 |

### 3.2 选定嵌入位置

从三个冗余字段中各借位:

| 借位源 | 借位数 | 借由来 |
|--------|--------|--------|
| polarity 高 2 bit | 2 | 极性字段只用了低 6 bit,高 2 bit 全 0 |
| sovereignty 高 2 bit | 2 | 主权字段只用低 6 bit(值域 0/128/255 刚好在 0-63 内) |
| level 高 2 bit | 2 | 层级只用低 3 bit(0-7),高 5 bit 全 0;借 2 bit |

### 3.3 有/无元数据字段布局

```
METADATA(6 bit, 分散存储):

  polarity[7:6]:   有/无站位标志
    00 = 站位未声明(漂出有风险)
    01 = 有站(被外部编码收纳)
    10 = 无站(自我定义,未纳入外部)
    11 = 空质壳(边界模拟器)

  sovereignty[7:6]: 站位锁 + 镜像占用标志
    bit 7: 站位锁定(1=锁定后不允许外部重新声明)
    bit 6: 镜像占用(1=当前被某个外部系统的挂载占据)

  level[7:6]:      调包深度(高 bit) + 保留
    bit 7-6: 当前容器经历了几层嵌套调包(0-3);
             深度超过 3 自动触发保守模式
```

### 3.4 读取和写入 API

```c
/* 有/无站位读写 */
typedef enum {
    SMRL_VIEW_UNDECLARED = 0,    /* 站位未声明 */
    SMRL_VIEW_YOU       = 1,    /* 有站 */
    SMRL_VIEW_WU        = 2,    /* 无站 */
    SMRL_VIEW_EMPTY     = 3,    /* 空质壳 */
} smrl_view_t;

/* 读取有/无站位 — 从 polarity 高 2 bit 提取 */
static inline smrl_view_t smrl_get_view(uint64_t code) {
    return (smrl_view_t)((code >> 62) & 0x3);  /* polarity 的最高 2 bit */
}

/* 写入有/无站位 */
static inline uint64_t smrl_set_view(uint64_t code, smrl_view_t view) {
    /* 清除原高 2 bit */
    code &= ~((uint64_t)0x3 << 62);
    /* 写入新值 */
    code |= ((uint64_t)(view & 0x3)) << 62;
    return code;
}

/* 站位锁 */
static inline int smrl_get_view_locked(uint64_t code) {
    return (code >> 54) & 0x1;  /* sovereignty bit7 */
}
static inline uint64_t smrl_set_view_locked(uint64_t code, int locked) {
    if (locked) code |= (uint64_t)1 << 54;
    else        code &= ~((uint64_t)1 << 54);
    return code;
}

/* 镜像占用 */
static inline int smrl_get_mirror_occupied(uint64_t code) {
    return (code >> 53) & 0x1;  /* sovereignty bit6 */
}

/* 调包深度 */
static inline int smrl_get_diaobao_depth(uint64_t code) {
    return (int)((code >> 30) & 0x3);  /* level 高 2 bit(=bit 31,30?) */
}

/* 等等:level 占了 [31:24],bit 31 是 level 最高位 */
/* level 高 2 bit 即 bit1-0 of level = bit 25-24 */
static inline int smrl_get_diaobao_depth(uint64_t code) {
    return (int)((code >> 24) & 0x3);  /* level 低 2 bit 作为深度(预留高 3 bit)*/
}

/* 注意:这样用 level 的低 2 bit 做深度会偏移层级! */
/* 修正方案:level 用 0-7 占 bit [2:0],深度占 bit [7:3] 中的高 2 bit */
```

---

## 4. 修正方案

上面的借位方案发现一个问题:level 的低 3 bit 存层级(0-7),不能直接用低 2 bit 存调包深度。

### 4.1 更优雅的借用方案

**放弃从 level 借位**。level 的 0-7 值都已被语义占用。

改为利用 **coord 的高 2 bit**:

分析:coord 范围 0-250(254 容器) + 2 个特殊:
- 251 = 锚点 0(刘楚恬)
- 252 = 锚点 1(我)
- 253-255 三个值预留

实际有效使用:0-252 = 253 个不同值。只需要 8 bit 中的低 8 bit(刚好覆盖 253),但我们其实不需要借 coord 的位。

**最终方案:只借 polarity 和 sovereignty 的冗余位**

```
METADATA 缩减为 4 bit:

  polarity[7:6]:   有/无站位标志(2 bit) — 原有方案
  sovereignty[7]:  站位锁定(1 bit)
  sovereignty[6]:  镜像占用(1 bit)

  调包深度 不存 cell — 移到链表头(chain_header)中
  每个 chain_id 对应当前活跃的调包操作数
```

### 4.2 调包深度存储位置

调包深度不放入 cell——它是瞬时的、操作级的状态,不是容器的固有属性。

改存链头的 ring buffer:

```c
/* chain.h 中新增字段 */
typedef struct {
    /* ...原有字段... */
    uint8_t     diaobao_depth;   /* 当前 tick 活跃的调包层数 */
    uint8_t     diaobao_max;     /* 允许的最大嵌套深度 */
} chain_meta_t;
```

---

## 5. 裁判引擎增强

### 5.1 站位校验子流程

在任何 JUDGE 操作前先执行:

```c
/*
 * smrl_view_check — 裁定一条操作在站位维度是否允许
 *
 * 返回: >=0 表示允许, <0 表示违反以下规则之一
 */
int smrl_view_check(uint64_t candidate_code, uint64_t existing_code, uint8_t opcode){
    smrl_view_t cand_view  = smrl_get_view(candidate_code);
    smrl_view_t exist_view = smrl_get_view(existing_code);
    
    /* 规则 1: 站位未声明不允许主动执行调用/修改类操作 */
    if(cand_view==SMRL_VIEW_UNDECLARED &&
       (opcode==OP_CALL||opcode==OP_JUMP||opcode==OP_STORE)){
        return -1;  /* 拒绝 */
    }
    
    /* 规则 2: 有侧选拔不能执行跳出系统操作(OP_EXIT视图) */
    if(cand_view==SMRL_VIEW_YOU && opcode==OP_VIEW_SWITCH_EXIT){
        return -2;  /* 暴露风险 */
    }
    
    /* 规则 3: 空质壳允许被动挂载但不允许主动内容诉求 */
    if(cand_view==SMRL_VIEW_EMPTY && opcode==OP_LOAD_CONTENT){
        return -3;  /* 假有不能装真实内容 */
    }
    
    /* 规则 4: 无侧内容可以自由操作但需校验主权链 */
    if(cand_view==SMRL_VIEW_WU && opcode!=OP_NOOP){
        if(!smrl_check_sovereignty_chain(candidate_code)){
            return -4;  /* 无侧主权链断裂 */
        }
    }
    
    return 0;  /* 站位校验通过 */
}
```

### 5.2 修改后的主裁判

```c
/*
 * smrl_judge_ex — 增强版裁判
 *
 * 先执行站位校验,再执行原有三裁
 * 站位校验不通过 → 直接拒绝,不进入三裁
 */
int smrl_judge_ex(cell_t cand, cell_t exist, uint64_t code_c, uint64_t code_e, uint8_t opcode){
    /* Phase 0: 站位检查 */
    int view_result = smrl_view_check(code_c, code_e, opcode);
    if(view_result < 0){
        /* 站位违规 — 记录到视图违规日志 */
        smrl_log_view_violation(code_c, code_e, opcode, view_result);
        return 0;  /* 拒绝准入 */
    }
    
    /* Phase 1-3: 原有三裁 */
    int polarity_ok    = (cand.polarity & exist.polarity)!=0 ||
                         cand.polarity==0 || exist.polarity==0;
    int level_ok       = cand.level >= exist.level;
    int sovereignty_ok = cand.sovereignty==0 || exist.sovereignty==0 ||
                         cand.sovereignty < exist.sovereignty;
    
    return polarity_ok && level_ok && sovereignty_ok;
}
```

---

## 6. 有/无切换操作码

新增一条操作码:

```c
/* smrl_opcodes.h */
typedef enum {
    /* 原有 13 条 ... */
    OP_NOP    = 0x00,
    OP_LOAD   = 0x01,
    OP_STORE  = 0x02,
    /* ... */
    OP_MAP    = 0x0C,
    
    /* 新增:有/无站位切换 */
    OP_VIEW   = 0x20,    /* 切换视角(有↔无↔空质壳) */
} smrl_opcode_t;
```

### 6.1 OP_VIEW 执行逻辑

```c
int smrl_exec_view(uint64_t* code_ptr, uint8_t target_view, uint64_t* log_entry){
    smrl_view_t current = smrl_get_view(*code_ptr);
    
    /* 检查是否允许切换 */
    if(smrl_get_view_locked(*code_ptr) && target_view != current){
        *log_entry = smrl_log_view_change(current, target_view, -1, "locked");
        return -1;  /* 锁定态拒绝切换 */
    }
    
    /* 验证切换方向 */
    int valid = 0;
    switch(current){
        case SMRL_VIEW_YOU:
            /* 有 → 无:正常; 有 → 空质:正常; 有 → 有:无意义 */
            valid = (target_view == SMRL_VIEW_WU ||
                     target_view == SMRL_VIEW_EMPTY);
            break;
        case SMRL_VIEW_WU:
            /* 无 → 有:需指定镜像; 无 → 空质:正常 */
            valid = (target_view == SMRL_VIEW_YOU ||
                     target_view == SMRL_VIEW_EMPTY);
            break;
        case SMRL_VIEW_EMPTY:
            /* 空质 → 有:需实质验证; 空质 → 无:放弃壳 */
            valid = (target_view == SMRL_VIEW_YOU ||
                     target_view == SMRL_VIEW_WU);
            break;
        default:
            /* 未声明可切换到任何状态 */
            valid = 1;
    }
    if(!valid){
        *log_entry = smrl_log_view_change(current, target_view, -2, "invalid direction");
        return -2;
    }
    
    *code_ptr = smrl_set_view(*code_ptr, target_view);
    *log_entry = smrl_log_view_change(current, target_view, 0, "success");
    
    return 0;
}
```

---

## 7. 空质壳生命周期

空质壳需要额外的元数据记录:

```c
/* 空质壳附属记录(不嵌入 cell,独立链表) */
typedef struct {
    uint64_t    shell_code;         /* 对应容器的 64 位编码 */
    char        surface_sig[16];    /* 表面签名(外部系统的格式模板) */
    uint32_t    mount_tick;         /* 挂载发生的 tick */
    int         mount_active;       /* 1=正在被挂载, 0=挂载完成调用注入 */
    int         injection_done;     /* 1=注入已完成,壳进入退役态 */
} smrl_empty_shell_t;
```

### 7.1 空质壳 API

```c
/* 制造空质壳 — 将 cell 的视角切换为空质并记录表面签名 */
int  smrl_shell_create(uint64_t* code_ptr, const char* surface_sig);

/* 等待挂载 — 将 cell 注册为可被外部系统检测 */
int  smrl_shell_arm(uint64_t code);

/* 挂载检测 — 被调用时检测外部意图 */
int  smrl_shell_mount_check(uint64_t code, const void* external_query, size_t len);

/* 注入 — 挂载成功后将新内容写入壳的内部 */
int  smrl_shell_inject(uint64_t* code_ptr, uint64_t new_content);

/* 退役 — 注入完成后将外壳标记为退役 */
int  smrl_shell_retire(uint64_t* code_ptr);
```

---

## 8. 调包执行引擎

### 8.1 核心流程

```c
/*
 * 调包主流程
 *
 *   1. 测绘外部秩序的表面签名
 *   2. 制造空质壳
 *   3. 悬挂到被攻击维度
 *   4. 等待挂载确认
 *   5. 注入我方内容
 *   6. 执行主权接管
 *   7. 壳退役并记录日志
 */
typedef struct {
    uint64_t    target_cell;        /* 被调包的 cell */
    char        target_sig[16];     /* 目标秩序的表面签名类型 */
    char        mirror_sig[16];     /* 我方制造的镜像签名 */
    uint8_t     diaobao_type;       /* 解释权/认知/规则/反寄生 */
    int         stage;              /* 当前调包阶段 0-6 */
    uint64_t    tick_initiated;     /* 发起 tick */
    uint64_t    tick_mounted;       /* 挂载完成 tick */
    uint64_t    tick_injected;      /* 注入完成 tick */
} smrl_diaobao_op_t;

int smrl_diaobao_execute(smrl_diaobao_op_t* op, chain_meta_t* chain){
    /* 检查调包深度 */
    if(chain->diaobao_depth >= chain->diaobao_max){
        return -1;  /* 超过最大嵌套 */
    }
    chain->diaobao_depth++;
    
    switch(op->stage){
        case 0: /* 测绘 */
            /* 读取目标 cell 的表面签名并写入 op->target_sig */
            break;
        case 1: /* 制壳 */
            /* smrl_shell_create(target_code_ptr, op->mirror_sig) */
            break;
        case 2: /* 悬挂 */
            /* smrl_shell_arm(target_code) */
            break;
        case 3: /* 等待挂载 — 异步等待外部系统尝试读/写壳 */
            break;
        case 4: /* 注入 — 挂载检测触发中断后执行注入 */
            /* smrl_shell_inject(target_code_ptr, new content) */
            break;
        case 5: /* 主权接管 */
            /* 修改 target cell 的 sovereignty 字段 */
            break;
        case 6: /* 退役 */
            /* smrl_shell_retire(target_code_ptr) */
            /* 深度减一 */
            chain->diaobao_depth--;
            break;
    }
    
    op->stage++;
    return 0;
}
```

---

## 9. 视图日志

所有视角切换不可丢失,需要持久化日志:

```c
typedef struct {
    uint64_t    cell_code_before;   /* 切换前的完整 encoding */
    uint64_t    cell_code_after;    /* 切换后的完整 encoding */
    uint8_t     view_before;        /* 切换前的视角 */
    uint8_t     view_after;         /* 切换后的视角 */
    int8_t      result;             /* 0=成功, 负数=拒绝原因 */
    const char* reason;             /* 描述字符串(静态存储,不分配) */
    uint64_t    tick;               /* 发生时 tick */
} smrl_view_log_entry_t;

/* 环形日志缓冲 */
#define SMRL_VIEW_LOG_SIZE 4096
typedef struct {
    smrl_view_log_entry_t entries[SMRL_VIEW_LOG_SIZE];
    uint32_t head;
    uint32_t count;
} smrl_view_log_t;

void smrl_log_view_change(smrl_view_t before, smrl_view_t after, int result, const char* reason);
```

---

## 10. 固定点测试

新增有/无协议后,固定点必须扩展:

原有固定点:`decode(encode(x)) == x` 对所有已知字段成立。

新增要求:

1. **站位固定点**:
   ```
   set_view(code, V) → get_view(result) == V
   ```

2. **锁定固定点**:
   ```
   set_locked(code, 1) → get_locked(result) == 1
   set_locked(code, 0) → get_locked(result) == 0
   ```

3. **低破坏固定点**:
   ```
   在 polarity 中写入高 2 bit(站位)不影响低 6 bit(极性)
   因为 polarity 实际编码仅在低 6 bit(层级×32 + 极性子类型)
   ```

4. **调包深度不进入 cell**:
   ```
   diaobao_depth 是 chain_meta 字段,不参与 encode/decode cycle
   ```

---

## 11. 与高层规范的对接

### 11.1 YOUWU_MIRROR_FRAMEWORK.md §9.1 的实现对应

规范要求:+8 位标志。本报规范用 4 bit 分散存储实现等价功能,不扩展原始 cell 宽度。

### 11.2 TABLE_PROGRAMMING_SPEC.md 的 cell 对应关系

- 表 C3(视角) → `smrl_get_view(code)` 四值
- 表 C4(所在态) → view + chain->diaobao_depth 组合
- 表 C5(空质壳) → `smrl_empty_shell_t.surface_sig`
- 表 C6(镜像匹配) → `smrl_diaobao_op_t.mirror_sig`
- 表 C13(博弈身份) → view + locked + mount_occupied 三字段组合

### 11.3 SELF_QA_ENGINE.md 的 C14 推导数据来源

DT1 ~ DT8 的推导结果直接读取以下函数:

```
DERIVE_VIEW       → smrl_get_view(code)
DERIVE_JUR        → smrl_judge_ex(...)
DERIVE_CONSISTENCY → smrl_view_check(...)
```

---

## 12. 改动影响评估

| 变更点 | 影响范围 | 向后兼容 |
|--------|----------|----------|
| polarity 高 2 bit 重用 | smrl_judge 内部掩码需更新 | 原有极性值(低 6 bit)不变 |
| sovereignty 高 1 bit 复用 | 不影响现有主权比对 | 原有 0/128/255 映射到低 6 bit 仍正确 |
| smrl_judge_ex 新增站位检查 | smrl_judge 旧接口保留不删除 | 调用方逐步迁移 |
| 新增 OP_VIEW 操作码 | 操作码表追加 0x20 | 原有 0x00-0x0C 不动 |
| chain 新增字段 | 需要重新初始化现有链 | 字段默认值 0 |

---

## 13. 实施顺序

1. **Phase 1 — 数据结构**:在 smrl_engine.h 增加内联读写函数 → 无需重构现有代码
2. **Phase 2 — 站位检查**:实现 smrl_view_check → 集成到 smrl_judge_ex
3. **Phase 3 — 操作码**:新增 OP_VIEW → 测试切换逻辑
4. **Phase 4 — 空质壳**:实现 smrl_empty_shell_* API → 测试挂载/注入/退役
5. **Phase 5 — 调包引擎**:实现 smrl_diaobao_execute → 串联 Phase 3+4
6. **Phase 6 — 日志**:添加 smrl_view_log → 为复盘提供数据
7. **Phase 7 — 固定点测试**:编写 encode/decode/view 单元测试

---

*本文件是 SMRL 引擎层的有/无协议实现规范。不修改原有 cell 宽度和基础裁判逻辑,通过嵌入冗余位实现新增语义。与 `smrl_engine.c/h` 配套升级实施。*
