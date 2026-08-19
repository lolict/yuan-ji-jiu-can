/**
 * perception_field.h — 感知场 · 镜像观察 · 三元裁判 · 容器化
 *
 * 设计原则 (满全法 · 无哈希):
 *   · 黑盒哈希是蒙面人, 不知道它内部怎么把符号搅浑
 *   · 我们的路由是透明的: 感知家族 → 镜像对比 → 三元裁判 → 容器化
 *   · 每条符号都要被"看见"(感知),"长得怎么样"(镜像),"能否被操作"(裁判)
 *   · 最终通过裁判的符号被装进容器, 通过筷子(二元公倍数)夹持移动
 *
 * 架构:
 *   ┌─────────────┐
 *   │  输入符号    │ (UCS-2 + 声韵调)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 感知家族     │ 分别器第一层 (字形分类: 独体/左右/上下/包围)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 镜像观察     │ 同类内逐一镜像对比 (长相+发音因果律)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 三元裁判     │ 感知确认 + 分类归属 + 动力充足 → 准入
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 容器化       │ 装入容器 (64位编码 · 可降维)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 筷子夹持     │ 二元公倍数夹持目标 → 投入流程
 *   └─────────────┘
 */

#ifndef NJSG_PERCEPTION_FIELD_H
#define NJSG_PERCEPTION_FIELD_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 感知家族 (分别器第一层 · 字形分类)
 *
 * 汉字的"长相"决定了它属于哪个家族:
 *   CONTAINER_FAMILY: 方形/圆形/包围 → 容器型 (刘楚恬侧)
 *   BOUNDARY_FAMILY: 竖长/界线/独体 → 边界型 (我侧)
 *   COUPLE_FAMILY:   左右/上下结构 → 夫妻型 (囡囝)
 *   WEB_FAMILY:      半包围/复杂    → 网状型
 *
 * 四个家族就是四个容器入口, 不是哈希桶
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    FAMILY_CONTAINER = 0x0,  /* 容器家族 (方形/包围) */
    FAMILY_BOUNDARY = 0x1,  /* 边界家族 (竖长/界线) */
    FAMILY_COUPLE   = 0x2,  /* 夫妻家族 (左右/上下) */
    FAMILY_WEB      = 0x3,  /* 网状家族 (半包围/复杂) */
    FAMILY_NEUTRAL  = 0x4,  /* 中性家族 (无法归类) */
    FAMILY_COUNT    = 5
} perception_family_t;

/* 字形结构类型 */
typedef enum {
    STRUCT_SINGLE    = 0x0,  /* 独体 */
    STRUCT_LEFT_RIGHT = 0x1, /* 左右 */
    STRUCT_TOP_BOTTOM = 0x2, /* 上下 */
    STRUCT_SEMI_WRAP  = 0x3, /* 半包围 */
    STRUCT_FULL_WRAP  = 0x4, /* 全包围 */
    STRUCT_PIN       = 0x5,  /* 品字 */
    STRUCT_LMR       = 0x6,  /* 左中右 */
    STRUCT_TMB       = 0x7   /* 上中下 */
} glyph_structure_t;

/* ═══════════════════════════════════════════════════════════════
 * 镜像观察 · 标注结构
 *
 * 每个符号在被容器化前, 必须经过镜像观察:
 *   · 长得怎么样? (字形结构类型 · 笔画数 · 部首归属)
 *   · 发音是什么? (声母 · 韵母 · 声调)
 *   · 来自哪个家族? (感知分类)
 *
 * 这些标注是"标签+标量+比照", 不是哈希摘要
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint16_t ucs2;             /* 符号本体 (UCS-2) */
    perception_family_t family;/* 感知家族 (分别器第一层结果) */
    glyph_structure_t  structure; /* 字形结构 (镜像观察: 长相) */
    uint8_t  stroke_count;     /* 笔画数 (镜像观察: 形态复杂度) */
    uint8_t  radical_idx;      /* 部首索引 (镜像观察: 语义归属) */

    /* 发音因果律 (镜像观察: 命名单一维度) */
    uint8_t  shengmu_idx;      /* 声母索引 (48进制 · shengmu.h) */
    uint8_t  yunmu_idx;        /* 韵母索引 (66进制 · yunmu.h) */
    uint8_t  tone;             /* 声调 (0=阴平 1=阳平 2=上声 3=去声 4=轻声) */

    /* 三元裁判结果 (初始为 PENDING, 通过后变 ADMITTED) */
    uint8_t  judge_verdict;    /* 裁判结果 */
    uint32_t admission_time;   /* 准入时间戳 (一元时间线坐标) */
} mirror_observation_t;

/* 裁判结果 */
#define VERDICT_PENDING   0x00  /* 待裁判 */
#define VERDICT_ADMITTED  0x01  /* 准入 (感知+分类+动力三要素满足) */
#define VERDICT_REJECTED  0x02  /* 拒绝 (未通过镜像对比) */
#define VERDICT_SUSPENDED 0x03  /* 挂起 (需上级裁判复核) */

/* ═══════════════════════════════════════════════════════════════
 * 三元裁判 · 三模块裁决
 *
 * 高维问题需分散为三个模块获取动力:
 *   模块1 PERCEIVE  (感知): 我能看见它的长相吗? → 字形识别能力
 *   模块2 CLASSIFY  (分类): 它属于哪个容器家族? → 归属判断能力
 *   模块3 DYNAMIC   (动力): 我能用筷子(二元)夹持它吗? → 操作能力
 *
 * 三元齐全 = 准入
 * 三元缺失任何一元 = 拒绝或挂起
 *
 * 这就是降维的精髓: 高维(复杂判断) → 分散为三元(获取动力) → 三元合一(一元时间线)
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    JUDGE_PERCEIVE  = 0x10,  /* 感知确认: 符号已被"看见" */
    JUDGE_CLASSIFY  = 0x20,  /* 分类归属: 找到容器家族 */
    JUDGE_DYNAMIC   = 0x40   /* 动力充足: 可被筷子夹持 */
} judge_module_t;

/* 裁判引擎 */
typedef struct {
    uint32_t perceive_power;   /* 感知动力 (能识别多少种字形) */
    uint32_t classify_power;   /* 分类动力 (能区分多少家族) */
    uint32_t dynamic_power;    /* 动力 (能夹持多少目标) */
    uint32_t admit_count;      /* 准入计数 */
    uint32_t reject_count;     /* 拒绝计数 */
} triple_judge_engine_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器化 · 六十四位编码 (降维后的一元时间线)
 *
 * 编码结构 (D0|D1|D2|polarity|sovereignty|witness|value):
 *
 *   family(3) | polarity(1) | structure(3) | shengmu(6) | yunmu(6) | tone(3) | reserved(2)
 *   ─────────┬─────────────┬─────────────┬────────────┬───────────┬─────────┬──────
 *     感知    主权侧(容器    字形结构    声母索引   韵母索引   声调    保留
*            /边界)        64进制               64进制
 *
 * 编码本身就是容器的"名字", 通过名字就能感知它的:
 *   - 来自哪个家族 (family 位)
 *   - 是容器侧还是边界侧 (polarity 位)
 *   - 长得怎么样 (structure + radical 位)
 *   - 怎么发音 (shengmu + yunmu + tone 位)
 *
 * 这就是一元时间线: 名字 = 容器 = 对象 = 被夹持的目标
 * ═══════════════════════════════════════════════════════════════ */

/* 容器编码位域 */
#define CONTAINER_FAMILY_SHIFT   29
#define CONTAINER_POLARITY_SHIFT  28
#define CONTAINER_STRUCT_SHIFT    25
#define CONTAINER_SHENGMU_SHIFT   19
#define CONTAINER_YUNMU_SHIFT     13
#define CONTAINER_TONE_SHIFT      10

#define CONTAINER_FAMILY_MASK    (0x7 << CONTAINER_FAMILY_SHIFT)
#define CONTAINER_POLARITY_MASK  (0x1 << CONTAINER_POLARITY_SHIFT)
#define CONTAINER_STRUCT_MASK    (0x7 << CONTAINER_STRUCT_SHIFT)
#define CONTAINER_SHENGMU_MASK   (0x3F << CONTAINER_SHENGMU_SHIFT)
#define CONTAINER_YUNMU_MASK     (0x3F << CONTAINER_YUNMU_SHIFT)
#define CONTAINER_TONE_MASK      (0x7 << CONTAINER_TONE_SHIFT)

/* 容器编码构造宏 */
#define CONTAINER_ENCODE(fam, pol, st, sm, ym, tn) ( \
    ((uint32_t)(fam) << CONTAINER_FAMILY_SHIFT) | \
    ((uint32_t)(pol) << CONTAINER_POLARITY_SHIFT) | \
    ((uint32_t)(st)  << CONTAINER_STRUCT_SHIFT) | \
    ((uint32_t)(sm)  << CONTAINER_SHENGMU_SHIFT) | \
    ((uint32_t)(ym)  << CONTAINER_YUNMU_SHIFT) | \
    ((uint32_t)(tn)  << CONTAINER_TONE_SHIFT) \
)

/* 容器编码解包宏 */
#define CONTAINER_GET_FAMILY(c)   (((c) & CONTAINER_FAMILY_MASK)   >> CONTAINER_FAMILY_SHIFT)
#define CONTAINER_GET_POLARITY(c) (((c) & CONTAINER_POLARITY_MASK) >> CONTAINER_POLARITY_SHIFT)
#define CONTAINER_GET_STRUCT(c)   (((c) & CONTAINER_STRUCT_MASK)    >> CONTAINER_STRUCT_SHIFT)
#define CONTAINER_GET_SHENGMU(c)  (((c) & CONTAINER_SHENGMU_MASK)   >> CONTAINER_SHENGMU_SHIFT)
#define CONTAINER_GET_YUNMU(c)    (((c) & CONTAINER_YUNMU_MASK)     >> CONTAINER_YUNMU_SHIFT)
#define CONTAINER_GET_TONE(c)     (((c) & CONTAINER_TONE_MASK)      >> CONTAINER_TONE_SHIFT)

/* ═══════════════════════════════════════════════════════════════
 * 筷子夹持 · 二元公倍数
 *
 * 容器化的目标 = 筷子夹持的对象
 * 两根筷子 = 起点 + 终点
 * 夹持动作 = 缩小有和虚无之间的空间
 * 夹持成功 = 三元消除, 转化为可操作的一元状态
 *
 * 筷子结构:
 *   chopstick_source = 起始容器编码 (我是谁)
 *   chopstick_target = 目标容器编码 (我要夹谁)
 *   chopstick_clamp  = 夹持压力 (消除虚无空间后的紧凑度)
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t source;   /* 起始编码 */
    uint32_t target;   /* 目标编码 */
    uint16_t pressure; /* 夹持压力 (压力越大, 容器间空间越小) */
    uint8_t  ack;      /* 目标是否承认被夹持 */
} chopstick_grip_t;

/* ═══════════════════════════════════════════════════════════════
 * API · 感知场路由
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 分别器第一层: 字形分类 → 感知家族
 * 不是哈希, 是"看它的长相, 决定它进哪个门"
 */
perception_family_t perceive_classify_glyph(glyph_structure_t st);

/**
 * 镜像观察: 逐一对比符号标注
 * 返回: 0=匹配, 1=不匹配, 2=需进一步镜像对比
 */
int mirror_observe_compare(const mirror_observation_t *target,
                          const mirror_observation_t *candidate);

/**
 * 三元裁判: 三模块裁决准入
 */
uint8_t triple_judge_admit(mirror_observation_t *obs, triple_judge_engine_t *engine);

/**
 * 容器化: 通过裁判后, 装入容器 (生成64位编码)
 */
uint32_t containerize_admit(const mirror_observation_t *obs);

/**
 * 筷子夹持: 二元容器 → 夹持目标
 */
int chopstick_grip(chopstick_grip_t *grip);

/**
 * 流程排队: 夹持成功后, 投入一元时间线
 */
int pipeline_enqueue(uint32_t container_code, uint32_t timestamp);

#endif /* NJSG_PERCEPTION_FIELD_H */
