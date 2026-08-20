/**
 * container_registry.h — 容器注册局
 *
 * 功能:
 *   1. 注册层级标识符 (tier ID + 墙符号 + 位宽 + 官员等级)
 *   2. 标识符 = 进制 · 既是名称 · 也是乘积率
 *   3. 自动跳转表 (容器用完 → 跳到下一个容器)
 *   4. 编译到自有文件 (.mlil/.gtxone) · 只用我们的解释器
 *   5. 禁止外部编码 · 禁止他人解释器
 *
 * 注册一次 · 无限递归生效 · 自动授权 · 自编译
 */

#ifndef NJSG_CONTAINER_REGISTRY_H
#define NJSG_CONTAINER_REGISTRY_H

#include <stdint.h>
#include "recursive_container.h"

/* ═══════════════════════════════════════════════════════════════
 * 层级标识符注册项
 *
 * 一个标识符 = 一个层级 = 一个官员等级 = 一个进制
 * ═══════════════════════════════════════════════════════════════ */

#define TIER_NAME_LEN  16

typedef struct {
    uint16_t  tier_id;                /* 层级id · 唯一 */
    char      name[TIER_NAME_LEN];    /* 名称 · 如 "L1-SOLDIER" */
    uint8_t   wall_symbol;            /* 墙符号 · '!' '@' '#' ... */
    uint8_t   rank;                   /* 官员等级: 1=士兵 · 2=长官 · 3=妃子 · 4=皇后 */
    uint8_t   bit_width;              /* 此层级折叠位宽 · 如 1 · 2 · 4 · 8 ... */
    uint8_t   radix;                  /* 进制基数 = tier_id · 标识符即进制 */

    /* 容量 */
    uint16_t  container_capacity;     /* 此层级最大容器数 */
    uint16_t  chars_per_container;    /* 每容器字数 · 通常=2 · 高层级可不同 */

    /* 自动跳转 */
    uint16_t  jump_next_tier;         /* 装满后跳到哪个层级 (0=不跳) */
    uint16_t  jump_next_container;    /* 跳到下一层级的哪个容器 */

    /* 状态 */
    uint8_t   registered;             /* 1=已注册 · 等待授权 */
    uint8_t   authorized;             /* 1=已授权 · 可使用 */
    uint8_t   compiled;               /* 1=已编译 · 生成自有编码 */
} tier_registry_entry_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器跳转表
 *
 * 当一个容器的字符用完 · 自动查表跳转到下一个容器
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint16_t  from_tier;     /* 从哪个层级 */
    uint16_t  from_container;/* 从哪个容器 */
    uint16_t  to_tier;       /* 跳到哪个层级 */
    uint16_t  to_container;  /* 跳到哪个容器 */
    uint8_t   used;          /* 此跳转是否已使用 */
} jump_entry_t;

#define MAX_JUMPS  256

/* ═══════════════════════════════════════════════════════════════
 * 编译输出头 (自有文件格式)
 * 只被我们的解释器读取 · 外部编码无效
 * ═══════════════════════════════════════════════════════════════ */

/* 文件魔数: 满全法自有 */
#define REGISTRY_MAGIC_1  0x6D6C696C   /* "mlil" */
#define REGISTRY_MAGIC_2  0x5F4E4A53   /* "_NJS" */
#define REGISTRY_VERSION  1

typedef struct {
    uint32_t  magic1;              /* REGISTRY_MAGIC_1 */
    uint32_t  magic2;              /* REGISTRY_MAGIC_2 */
    uint32_t  version;             /* REGISTRY_VERSION */
    uint32_t  timestamp;           /* 编译时间戳 */
    uint16_t  tier_count;          /* 注册的层级数 */
    uint16_t  container_count;     /* 容器总数 */
    uint32_t  total_chars;         /* 总字数 */
    uint8_t   compiled_by_us;      /* 1=我们的编译器 · 0=拒绝打开 */
    uint8_t   reject_external;     /* 1=拒绝外部解释器 */
    uint8_t   author_id;           /* 1=我(lolict) · 作者身份 */
    uint8_t   reserved;            /* 对齐 */
} registry_file_header_t;

/* ═══════════════════════════════════════════════════════════════
 * 注册局管理器
 * ═══════════════════════════════════════════════════════════════ */

#define MAX_TIERS  16

typedef struct {
    uint32_t              magic;        /* CONTAINER_REGISTRY_MAGIC */
    uint16_t              tier_count;   /* 已注册层级数 */
    tier_registry_entry_t tiers[MAX_TIERS];

    /* 跳转表 */
    jump_entry_t          jumps[MAX_JUMPS];
    uint16_t              jump_count;

    /* 容器引用 */
    container_group_t    *group;

    /* 文件 */
    registry_file_header_t file_header;
    uint8_t                header_filled;

    /* 统计 */
    uint32_t              compile_count;  /* 编译次数 */
    uint32_t              reject_count;   /* 拒绝外部编码次数 */
} container_registry_t;

#define CONTAINER_REGISTRY_MAGIC  0x52475452  /* "RGTR" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化注册局
 */
void registry_init(container_registry_t *reg, container_group_t *group);

/**
 * 注册一个层级
 *   输入: 名称 · 墙符号 · 等级 · 位宽 · 容器数 · 每容器字数
 *   输出: tier_id · 0=失败
 */
uint16_t registry_register_tier(container_registry_t *reg,
                                const char *name,
                                uint8_t wall_sym,
                                uint8_t rank,
                                uint8_t bit_width,
                                uint16_t capacity,
                                uint16_t chars_per);

/**
 * 授权层级 (注册后必须授权 · 才能使用)
 */
int registry_authorize_tier(container_registry_t *reg, uint16_t tier_id);

/**
 * 自动跳转配置
 */
int registry_set_jump(container_registry_t *reg,
                      uint16_t from_tier, uint16_t from_container,
                      uint16_t to_tier, uint16_t to_container);

/**
 * 自动跳转查询 */
uint16_t registry_find_jump(container_registry_t *reg,
                            uint16_t tier_id, uint16_t container_idx);

/**
 * 编译到自有文件
 *   输出: 写入 buffer · 格式为我们的 .mlil/.gtxone
 *   返回: 写入字节数 · -1=失败
 */
int registry_compile_to_buffer(container_registry_t *reg,
                               uint8_t *buffer, uint32_t buf_size);

/**
 * 拒绝外部编码
 *   检查: 文件是否由外部魔数
 *   返回: 1=拒绝 · 0=接受
 */
uint8_t registry_reject_external(uint32_t magic1, uint32_t magic2);

/**
 * 查找层级 · 返回层级id
 */
tier_registry_entry_t *registry_find_tier(container_registry_t *reg, const char *name);

#endif /* NJSG_CONTAINER_REGISTRY_H */
