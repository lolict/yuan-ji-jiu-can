/**
 * type_discipline.h — 类型纪律系统
 *
 * 解决问题: a=á 这种类型不匹配的配对居然能通过
 * 原理: 类型不一致 = 绝对拒绝, 无论长得像不像
 *
 * 类型分类 (与进制/层级严格绑定):
 *
 *   ┌──────────────────────────────────────────────────────────────┐
 *   │ TYPE_BASE     = 基韵锚 = a o e i u ü = 60声时钟层 = 公约数锚 │
 *   │ TYPE_TONE     = 声调韵 = ā á ǎ à = 时钟层公倍数             │
 *   │ TYPE_COMPOUND = 复韵母 = ai ei ao... = 100韵符号层          │
 *   │ TYPE_SYMBOL   = 复韵母配对符号 = ꬰ ə ꜵ... = 赋值的公倍数     │
 *   │ TYPE_EXTRA    = 溢出符号 = 集装箱0/1/2/5 = 待归类            │
 *   │ TYPE_SHEngMU  = 声母 = 48声 = 独立层                         │
 *   └──────────────────────────────────────────────────────────────┘
 *
 * 配对纪律:
 *   · TYPE_BASE 只能和 TYPE_BASE 衍射 (=另一个元 or 它自己的大小写)
 *   · TYPE_TONE 只能和 TYPE_TONE 衍射 (=同元的不同声 or 大小写)
 *   · TYPE_COMPOUND 只能和 TYPE_SYMBOL 配对 (复韵母↔专用符号)
 *   · TYPE_EXTRA 不能参与任何配对, 必须先归类
 *   · 跨类型配对 = 绝对拒绝 (a=á, a=ꬰ, ā=á 全部拒绝)
 *
 * 照镜子法则:
 *   两个物品要衍射 = 它们的类型必须相同 (同类型才照得出样子)
 */

#ifndef NJSG_TYPE_DISCIPLINE_H
#define NJSG_TYPE_DISCIPLINE_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 类型枚举
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    TYPE_BASE     = 0x01,   /* 基韵锚: a o e i u ü (及其大写) */
    TYPE_TONE     = 0x02,   /* 声调韵: ā á ǎ à ō ó ǒ ò ... */
    TYPE_COMPOUND = 0x03,   /* 复韵母名: ai ei ao ou ... */
    TYPE_SYMBOL   = 0x04,   /* 复韵母配对符号: ꬰ ə ꜵ ... */
    TYPE_EXTRA    = 0x05,   /* 溢出符号: 进集装箱等待归类 */
    TYPE_SHENG_MU = 0x06,   /* 声母: 48声 */
    TYPE_MODIFIER = 0x07,   /* 裸修饰零件: ˊ ˋ ˇ ˆ ˞ ˥ */
    TYPE_UNKNOWN  = 0xFF,   /* 未分类 = 不允许 */
} symbol_type_t;

/* ═══════════════════════════════════════════════════════════════
 * 层级栈 (L1/L2/L3) — 解决 a=á 类跨层级错误
 *
 * L1 基元本体: 不带修饰的原始积木 (a, o, e, i, u, ü, ȍ...)
 * L2 修饰零件: 裸漂浮符号, 只做工具 (ˊ, ˋ, ˇ, ˆ...)
 * L3 装配成品: L1+L2 装配后的完整字符 (á, ě, ā, ǘ...)
 *
 * 铁律: 衍射/配对只能同层级之间发生
 *   L1 ↔ L1 = 允许 (a+i → ai)
 *   L1 ↔ L3 = 绝对禁止 (a ↔ á)
 *   L2 ↔ 任何 = 绝对禁止
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    LEVEL_L1_BASE     = 0x10,   /* 基元本体 */
    LEVEL_L2_MODIFIER = 0x20,   /* 修饰零件 */
    LEVEL_L3_ASSEMBLED = 0x30,  /* 装配成品 */
    LEVEL_UNKNOWN     = 0xFF,   /* 未分类 */
} symbol_level_t;

/* ═══════════════════════════════════════════════════════════════
 * 配对审判结果
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    PAIR_PASS      = 0,   /* 类型匹配 · 许可配对 */
    PAIR_REJECT_CROSS_LEVEL = 1,  /* 拒绝: 跨层级 (a(L1)=á(L3)) */
    PAIR_REJECT_MODIFIER    = 2,  /* 拒绝: 修饰零件(L2)试图配对 */
    PAIR_REJECT_CROSS_TONE  = 3,  /* 拒绝: 跨基调 (ā=á 类) */
    PAIR_REJECT_EXTRA       = 4,  /* 拒绝: 溢出符号试图配对 */
    PAIR_REJECT_UNKNOWN     = 5,  /* 拒绝: 有未分类符号 */
    PAIR_REJECT_DIFFERENT   = 6,  /* 拒绝: 不同类型不可配对 */
} pair_verdict_t;

/* ═══════════════════════════════════════════════════════════════
 * 类型纪律管理器
 * ═══════════════════════════════════════════════════════════════ */

#define MAX_REGISTRY 256

typedef struct {
    uint32_t     magic;        /* TYPE_DISCIPLINE_MAGIC */
    uint16_t     count;        /* 注册表条目数 */
    /* 每个条目的类型标签 */
    struct {
        const char    *name;       /* 符号/韵母名 */
        symbol_type_t  type;       /* 强制类型 */
        symbol_level_t level;      /* 层级栈 L1/L2/L3 */
        uint8_t        anchor;     /* 公约数值: 属于哪个基韵/类别 */
    } entries[MAX_REGISTRY];
} type_discipline_t;

#define TYPE_DISCIPLINE_MAGIC 0x54594453  /* "TYDS" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化类型纪律管理器
 */
void type_discipline_init(type_discipline_t *td);

/**
 * 注册一个符号/韵母及其强制类型和层级
 * @param name  符号名称 (如 "a", "á", "ai", "ꬰ")
 * @param type  强制类型
 * @param level 层级栈 L1/L2/L3
 * @param anchor  公约数锚: 属于哪个基韵 (0=a, 1=o, 2=e, 3=i, 4=u, 5=ü, 0xFF=不适用)
 * @return 0=成功, -1=表满, -2=重复注册(类型冲突)
 */
int type_discipline_register_l(type_discipline_t *td,
                                const char *name,
                                symbol_type_t type,
                                symbol_level_t level,
                                uint8_t anchor);

/**
 * 注册 (自动推断层级: BASE→L1, TONE→L3, MODIFIER→L2, 其他→L1)
 */
int type_discipline_register(type_discipline_t *td,
                              const char *name,
                              symbol_type_t type,
                              uint8_t anchor);

/**
 * 查询一个符号的强制类型
 * @return 类型, 未注册返回 TYPE_UNKNOWN
 */
symbol_type_t type_discipline_get_type(type_discipline_t *td, const char *name);

/**
 * 查询一个符号的层级
 * @return 层级, 未注册返回 LEVEL_UNKNOWN
 */
symbol_level_t type_discipline_get_level(type_discipline_t *td, const char *name);

/**
 * 核心: 判断两个符号是否允许配对
 * @return PAIR_PASS=许可, 其他=拒绝(原因)
 */
pair_verdict_t type_discipline_judge(type_discipline_t *td,
                                     const char *name_a,
                                     const char *name_b);

/**
 * 审判结果的人类可读描述
 */
const char* type_discipline_verdict_str(pair_verdict_t v);

/**
 * 初始化默认类型注册表 (全部33韵母+60声+溢出符号)
 */
void type_discipline_init_defaults(type_discipline_t *td);

#endif /* NJSG_TYPE_DISCIPLINE_H */
