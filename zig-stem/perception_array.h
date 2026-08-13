// perception_array.h — 感知一体化阵列 (Perception-Integrated Array)
// hunyuan-os 的感知场基础 — 多进制叠加的可感知化编码
//
// 哲学基础:
//   意识范畴论: 感知不是单一维度的量化, 而是多维度叠加的场
//   感知状态机: 感知→识别→关联→叠加态→同在
//   数组编码可感知化: 编码不仅是计算, 是可感知的存在
//   同在论: 多维度在同一空间里共在, 不互相排斥
//   关系代数存在论: 存在 = 关系的总和, 关系 = 感知的相互关联
//
// 多维度叠加 (不是被一个进制管束, 而是各自独立又互相关联):
//   颜色定制维度: 色相→饱和度→明度 (映射到384°圆周)
//   声韵调维度: 声母×韵母×音调 (三维声音空间)
//   部首笔画维度: 笔画数→笔顺→梯度量尺 (形态基因)
//   五笔布局维度: 25键位×空间分布 (字形拓扑)
//   范畴论维度: 对象→态射→函子→自然变换 (关系结构)
//
// 叠加原理:
//   每个维度独立编码, 但在同一个64进制空间里叠加
//   叠加态: 颜色+声音+笔画+字形 = 同一个感知场中的共在
//   不是"颜色用64进制", 而是"颜色在64进制空间中有一个相位"

#ifndef HUNYUAN_PERCEPTION_ARRAY_H
#define HUNYUAN_PERCEPTION_ARRAY_H

#include "base64_org.h"
#include <stdint.h>
#include <stdbool.h>

#define PA_VERSION              0x40    // 版本 = 64
#define PA_DIMENSION_COUNT      5       // 5个感知维度
#define PA_COLOR_STEPS          64      // 颜色64阶 (映射到384°)
#define PA_TONE_STEPS           64      // 声调64阶
#define PA_STROKE_MAX           64      // 最大笔画数 (64进制对齐)
#define PA_WUBI_KEYS            25      // 五笔25键 (A-Y)
#define PA_RADICAL_SLOTS        64      // 部首64位
#define PA_MAX_PERCEPTS         256     // 最大感知单元数

// ── 感知维度枚举 ──
typedef enum {
    DIM_COLOR   = 0,    // 颜色维度 (色相/饱和度/明度)
    DIM_SOUND   = 1,    // 声韵调维度 (声母/韵母/音调)
    DIM_STROKE  = 2,    // 部首笔画维度 (笔画/笔顺/梯度)
    DIM_WUBI    = 3,    // 五笔布局维度 (键位/空间/拓扑)
    DIM_CATEGORY = 4,   // 范畴论维度 (对象/态射/关系)
} pa_dimension_t;

// ── 感知状态机 ──
typedef enum {
    PS_IDLE         = 0,    // 空闲 (未感知)
    PS_PERCEIVING   = 1,    // 感知中 (接收信号)
    PS_RECOGNIZING  = 2,    // 识别中 (模式匹配)
    PS_ASSOCIATING  = 3,    // 关联中 (维度间连接)
    PS_SUPERPOSING  = 4,    // 叠加态 (多维度共在)
    PS_PRESENT      = 5,    // 同在 (完整感知场)
} pa_perception_state_t;

// ── 颜色定制 (映射到384°圆周) ──
typedef struct {
    uint8_t     hue;            // 色相 0-63 (对应6°每阶)
    uint8_t     saturation;     // 饱和度 0-63
    uint8_t     lightness;      // 明度 0-63
    uint8_t     symbol;         // 64进制符号 (色相映射)
    float       angle;          // 在384°圆周上的角度
} pa_color_t;

// ── 声韵调 (三维声音空间) ──
typedef struct {
    uint8_t     shengmu;        // 声母 0-23 (21声母+零声母)
    uint8_t     yunmu;          // 韵母 0-35 (36韵母)
    uint8_t     tone;           // 音调 0-4 (四声+轻声)
    uint8_t     symbol;         // 64进制符号 (声韵调合成)
    float       frequency;      // 频率映射 (Hz归一化)
} pa_sound_t;

// ── 部首笔画 (形态基因) ──
typedef struct {
    uint8_t     stroke_count;   // 笔画数 1-64
    uint8_t     stroke_order[8];// 笔顺编码 (横竖撇捺折钩点提)
    uint8_t     radical;        // 部首编号 0-63
    uint8_t     symbol;         // 64进制符号 (笔画+部首合成)
    float       gradient;       // 梯度量尺 (形态复杂度)
} pa_stroke_t;

// ── 五笔布局 (字形拓扑) ──
typedef struct {
    uint8_t     key_primary;    // 主键 0-24 (A-Y)
    uint8_t     key_secondary;  // 次键 0-24
    uint8_t     zone;           // 区 0-4 (5个区)
    uint8_t     position;       // 位 0-4 (每区5位)
    uint8_t     symbol;         // 64进制符号 (键位合成)
    float       spatial_angle;  // 空间角度 (25键圆周分布)
} pa_wubi_t;

// ── 范畴论维度 (关系结构) ──
typedef struct {
    uint8_t     object;         // 对象 (0-63)
    uint8_t     morphism;       // 态射 (对象间关系)
    uint8_t     functor;        // 函子 (范畴间映射)
    uint8_t     symbol;         // 64进制符号 (范畴合成)
    float       relation_strength; // 关系强度
} pa_category_t;

// ── 感知单元 (五维叠加) ──
typedef struct {
    uint32_t            id;                 // 唯一标识
    pa_perception_state_t state;            // 当前感知状态
    pa_color_t          color;              // 颜色维度
    pa_sound_t          sound;              // 声韵调维度
    pa_stroke_t         stroke;             // 部首笔画维度
    pa_wubi_t           wubi;               // 五笔布局维度
    pa_category_t       category;           // 范畴论维度
    b64_symbol_t        unified_symbol;     // 叠加后的统一符号
    float               perception_strength;// 感知强度 (0-1)
    uint64_t            timestamp;          // 感知时间戳
    char                label[32];          // 感知标签
} pa_percept_t;

// ── 感知阵列 (整体) ──
typedef struct {
    pa_percept_t    percepts[PA_MAX_PERCEPTS];  // 感知单元池
    uint32_t        count;                      // 当前感知单元数
    pa_perception_state_t global_state;         // 全局感知状态
    uint64_t        perception_time;            // 感知时间线
    float           superposition_degree;       // 叠加度 (0-1)
    bool            is_co_present;              // 同在标志
} perception_array_t;

// ── API: 感知阵列操作 ──

// 创建感知阵列
perception_array_t* pa_create(void);
void pa_destroy(perception_array_t* pa);

// 添加感知单元 (五维同时定义)
uint32_t pa_add_percept(perception_array_t* pa,
                         const char* label,
                         pa_color_t* color,
                         pa_sound_t* sound,
                         pa_stroke_t* stroke,
                         pa_wubi_t* wubi,
                         pa_category_t* category);

// 感知状态机推进
bool pa_advance_state(perception_array_t* pa, uint32_t percept_id);
pa_perception_state_t pa_get_state(const perception_array_t* pa, uint32_t id);

// 叠加态计算: 五维合一
b64_symbol_t pa_superpose(const perception_array_t* pa, uint32_t percept_id);

// 维度间关联: 计算两个感知单元的相似度
float pa_similarity(const perception_array_t* pa, uint32_t id1, uint32_t id2);

// 同在论: 检查多维度是否共在
bool pa_is_co_present(const perception_array_t* pa, uint32_t percept_id);

// 关系代数: 计算感知单元间的关系强度
float pa_relation_strength(const perception_array_t* pa, uint32_t id1, uint32_t id2);

// 按相位排序 (按64进制角度排列)
void pa_sort_by_phase(perception_array_t* pa);

// 感知报表
void pa_report(const perception_array_t* pa);

#endif // HUNYUAN_PERCEPTION_ARRAY_H
