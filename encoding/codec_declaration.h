/**
 * codec_declaration.h — 声明式编码系统
 *
 * 核心: 活动的 · 可切换的 · 多协议编码框架
 *
 * 三种定位模式:
 *   CODEC_MODE_FULL     全量绝对定位  — 所有容器是一个整体 · 一键制全局
 *   CODEC_MODE_SELECT   相对定位      — 选择容器 · 优先级排列 · 定制云
 *   CODEC_MODE_INERTIA  最佳态惯性    — 固定某几个最佳容器 + 补充 · 体验惯性
 *
 * 协议可以互相切换 · 绑定全新编码规则
 * 容器可以注册/注销 · 未来可增长 · 不死板
 *
 * 声明表头 (codec_banner) 是全局唯一的入口:
 *   修改 mode/protocol_id 即切换整个系统的行为
 *   所有下游容器随声明而变 · 不是写死
 */

#ifndef NJSG_CODEC_DECLARATION_H
#define NJSG_CODEC_DECLARATION_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 配置
 * ═══════════════════════════════════════════════════════════════ */

#define CODEC_MAX_CONTAINERS       256   /* 最大注册容器数 */
#define CODEC_MAX_PROTOCOLS        16    /* 最大协议数 */
#define CODEC_MAX_PRIORITY         255   /* 最高优先级 */
#define CODEC_NAME_MAX             16    /* 容器名长度 */

#define CODEC_MAGIC                0x34 47 53 4A  /* "NJSG" */

/* ═══════════════════════════════════════════════════════════════
 * 模式
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    CODEC_MODE_FULL     = 0,   /* 全量绝对定位 · 所有容器 */
    CODEC_MODE_SELECT   = 1,   /* 相对定位 · 选择 · 优先级 */
    CODEC_MODE_INERTIA  = 2,   /* 最佳态惯性 · 固定 + 补充 */
} codec_mode_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器类型 (什么罐头装什么内容)
 *
 * 容器 = 一个可命名的编码单元
 * 可以有无限种类型 · 通过注册表动态新增
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    CONTAINER_TYPE_HANZI    = 0x01,  /* 汉字容器 (perception家族) */
    CONTAINER_TYPE_PINYIN   = 0x02,  /* 拼音容器 (声韵调) */
    CONTAINER_TYPE_RADICAL  = 0x03,  /* 部首容器 */
    CONTAINER_TYPE_MCCP     = 0x04,  /* MCCP token 容器 */
    CONTAINER_TYPE_BINARY   = 0x05,  /* 纯二进制的壳 */
    CONTAINER_TYPE_CUSTOM   = 0x80,  /* 自定义类型起点 */
} container_type_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器槽位 (注册表的每一行)
 *
 * 一个容器 = 一个罐头:
 *   id        — 罐头编号
 *   type      — 装的什么
 *   priority  — 优先级 (选择模式下 · 先选谁 · 后选谁)
 *   active    — 是否激活
 *   immutable — 是否固定不动 (最佳态 · 这几个永远是底)
 *   name[16]  — 罐头名字
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t          id;          /* 容器唯一id */
    container_type_t  type;        /* 容器类型 */
    uint8_t           priority;    /* 优先级 · 数字越小越优先 */
    uint8_t           active;      /* 1=在当前选择集内 */
    uint8_t           immutable;   /* 1=最佳态永远不动的底 */
    uint8_t           reserved;    /* 对齐 */
    char              name[CODEC_NAME_MAX];  /* 罐头名 */
} codec_slot_t;

/* ═══════════════════════════════════════════════════════════════
 * 声明表头 (全局的 · 唯一的入口)
 *
 * 修改它就能改变整个编码行为:
 *   mode         — 全量 / 选择 / 最佳态
 *   protocol_id  — 当前用哪套转译规则
 *   container_count — 当前注册了多少罐头
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t       magic;          /* "NJSG" 校验 */
    uint32_t       version;        /* 协议版本号 */
    codec_mode_t   mode;           /* 当前定位模式 */
    uint32_t       protocol_id;    /* 当前协议id */
    uint32_t       container_count; /* 注册的容器数 */
    uint32_t       active_count;   /* 当前激活数 */
    uint32_t       auto_balance;   /* 自动再平衡 (增减时) */
} codec_banner_t;

/* ═══════════════════════════════════════════════════════════════
 * 转译协议 (一套规则: 告诉系统容器间怎么互转)
 *
 * 协议是可替换的声明 · 不同协议 = 不同的"语言"
 * 例如:
 *   协议0: 满全法容器 → 汉字 (直接)
 *   协议1: 满全法容器 → 拼音 (投影)
 *   协议2: MCCP → 容器 (反向)
 *   协议3: 二进制 → 容器 (重新编排)
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t  id;                /* 协议id */
    uint32_t  version;           /* 协议版本 */
    char      name[CODEC_NAME_MAX]; /* 协议名 */
    uint32_t  source_type;       /* 源容器类型 */
    uint32_t  target_type;       /* 目标容器类型 */
    uint32_t  slot_ref;          /* 关联的选择槽位 (可选) */
} codec_protocol_t;

/* ═══════════════════════════════════════════════════════════════
 * 活跃集 (当前"使用哪些容器"的状态)
 *
 * 三种模式产出不同的活跃集:
 *   FULL     → 全部注册容器
 *   SELECT   → 按优先级 + active=1 过滤 · 不超过 max_active
 *   INERTIA  → immutable=1 的 + 优先级最高的补到 max_active
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t  count;                         /* 活跃容器数 */
    uint32_t  ids[CODEC_MAX_CONTAINERS];     /* 容器id列表 (按优先级排序) */
    uint32_t  max_active;                    /* 活跃集最大容量 */
    codec_mode_t mode;                       /* 生成时用的模式 */
} codec_active_set_t;

/* ═══════════════════════════════════════════════════════════════
 * 编码结果
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    CODEC_OK           = 0x00,
    CODEC_ERR_FULL     = 0x01,  /* 容器注册表满 */
    CODEC_ERR_NOTFOUND = 0x02,  /* 容器不存在 */
    CODEC_ERR_MISMATCH = 0x03,  /* 类型不匹配 */
    CODEC_ERR_INACTIVE = 0x04,  /* 容器未激活 */
    CODEC_ERR_MAGIC    = 0xFF,  /* 表头magic错 */
} codec_error_t;

typedef struct {
    codec_error_t  error;
    uint32_t       container_id;      /* 用了哪个容器 */
    uint32_t       encoded_value;     /* 编码后的值 */
    uint8_t        protocol_used;     /* 用了哪个协议 */
} codec_result_t;

/* ═══════════════════════════════════════════════════════════════
 * API · 声明式编码系统
 * ═══════════════════════════════════════════════════════════════ */

/* ── 生命周期 ── */

/**
 * 初始化声明表头
 */
codec_error_t codec_init(codec_banner_t *banner);

/**
 * 加载已注册的容器表
 */
codec_error_t codec_load_slots(const codec_slot_t *slots, uint32_t count);

/* ── 容器注册 (活动增减) ── */

/**
 * 注册一个新容器
 *   若 auto_balance 开启, 自动调整优先级
 */
codec_error_t codec_register(const codec_slot_t *slot);

/**
 * 注销一个容器
 *   若 auto_balance 开启, 活跃集自动重排
 */
codec_error_t codec_unregister(uint32_t container_id);

/**
 * 激活/停用一个容器
 *   只在 SELECT 模式下生效
 */
codec_error_t codec_set_active(uint32_t container_id, int active);

/**
 * 设置容器优先级
 */
codec_error_t codec_set_priority(uint32_t container_id, uint8_t priority);

/* ── 模式切换 ── */

/**
 * 切换定位模式
 *   FULL / SELECT / INERTIA
 *   切换后 · 活跃集自动重算
 */
codec_error_t codec_set_mode(codec_mode_t mode);

/**
 * 获取当前活跃集
 */
codec_active_set_t codec_get_active_set(void);

/* ── 协议切换 ── */

/**
 * 注册一个转译协议
 */
codec_error_t codec_register_protocol(const codec_protocol_t *protocol);

/**
 * 切换到某协议
 */
codec_error_t codec_switch_protocol(uint32_t protocol_id);

/* ── 编码/解码 ── */

/**
 * 用当前模式下的活跃容器编码
 *   自动选择最匹配的容器 (优先级最高 + 类型匹配)
 */
codec_result_t codec_encode(uint32_t input, uint32_t input_type);

/**
 * 解码: 容器id + 编码值 → 原始值
 */
codec_result_t codec_decode(uint32_t container_id, uint32_t encoded_value);

/**
 * 转译: 把某值从一种容器格式转到另一种
 */
codec_result_t codec_translate(uint32_t input, uint32_t from_type, uint32_t to_type);

#endif /* NJSG_CODEC_DECLARATION_H */
