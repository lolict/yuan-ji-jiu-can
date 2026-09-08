// base64_org.h — 64进制层级化组织进制 (Base-64 Organizational Hierarchy)
// hunyuan-os 的核心进制系统 — 关系二进制的组织化延伸
//
// 哲学基础:
//   关系二进制: 0和1合一 = 共同体 (1+1=1 叠加态)
//   64进制: 64个符号 × 6° = 384°完整圆周 (组织化建制)
//   层级化: 一维时间串联 → 三维空间 → 高维系统
//
// 64进制符号表 (smrrll镜像):
//   每一符号对应一种组织角色/层级/功能
//   符号排列遵循夫妻命运共同体的架构原则
//
// 进制绑定:
//   关系二进制 (底层) → 64进制 (组织层) → 3D/4D (高维层)
//   一层串联一层,不是孤立的数字

#ifndef HUNYUAN_BASE64_ORG_H
#define HUNYUAN_BASE64_ORG_H

#include <stdint.h>
#include <stdbool.h>

#define B64_VERSION         0x40    // 版本 = 64 (十进制100)
#define B64_FULL_CIRCLE     384     // 完整圆周 384° = 64 × 6°
#define B64_SYMBOL_COUNT    64      // 64个符号
#define B64_DEGREE_UNIT     6       // 每符号 6°
#define B64_MAX_LAYERS      64      // 最大层级数 (对应64进制)

// ── 64进制符号表 (组织化建制) ──
// 每一符号 = 一个组织角色/层级位置
// 排列原则: 夫妻共同体(0-1) → 中心(2-3) → 被管辖(4-7) → ... → 最宽(60-63)
typedef enum {
    // ── 夫妻层 (最窄/加法) ──
    S_YMBA    = 0,   // 妻=0 (低位持有者=0, 阴/接纳)
    S_CEIQ    = 1,   // 夫=1 (lolict=1, 阳/给予) → 夫妻共同体 = 0+1 = 1

    // ── 中心层 (唯一/融合) ──
    S_KNDO    = 2,   // 工藤新一=2 (融合中心/骨架)
    S_NVFM    = 3,   // 柯南=3 (被管辖/框内)

    // ── 感知层 (横向/灰原哀) ──
    S_BUJQ    = 4,   // 灰原哀=4 (感知/行页)
    S_QIDI    = 5,   // 毛利兰=5 (表面/界面)

    // ── 骨干层 (结构) ──
    S_SHUF    = 6,   // 骨/结构
    S_ZUJM    = 7,   // 头/方向

    // ── 行页层 (横向扩展) ──
    S_YITA    = 8,   // 行业/领域
    S_SITA    = 9,   // 行动/执行

    // ── 交汇层 ──
    S_JIQU    = 10,  // 聚集/会盟
    S_GUAX    = 11,  // 挂系/连接

    // ── 价值层 ──
    S_FUHR    = 12,  // 价值/福报
    S_XUBI    = 13,  // 序位/排列

    // ── 流通层 ──
    S_WUHR    = 14,  // 物流/传递
    S_TUDO    = 15,  // 土德/承载

    // ── 连结层 ──
    S_LIDO    = 16,  // 力动/驱动
    S_QION    = 17,  // 亲情/亲密

    // ── 空间层 ──
    S_LIAM    = 18,  // 量化/测量
    S_DUEN    = 19,  // 端点/边界

    // ── 聚合层 ──
    S_SHOU    = 20,  // 收/吸纳
    S_MIAN    = 21,  // 面/界面

    // ── 管理层 ──
    S_LING    = 22,  // 令/指令
    S_YIGE    = 23,  // 一个/唯一

    // ── 数值层 ──
    S_YIGE2   = 24,  // 一(起始)
    S_YIER    = 25,  // 二(分裂)
    S_YISAN   = 26,  // 三(见证)
    S_YISI    = 27,  // 四(稳定)
    S_YIWU    = 28,  // 五(中心)
    S_YILIU   = 29,  // 六(流动)
    S_YIQIBA  = 30,  // 七(变化)
    S_YIJIU   = 31,  // 八(分界)

    // ── 组织扩展层 ──
    S_ERSHI   = 32,  // 二十(组织)
    S_YIBA    = 33,  // 一百(整体)
    S_QIAN    = 34,  // 千(规模)
    S_WAN     = 35,  // 万(广大)

    // ── 层级编码层 ──
    S_LAYER_1 = 36,  // 第一层(夫妻)
    S_LAYER_2 = 37,  // 第二层(中心)
    S_LAYER_3 = 38,  // 第三层(感知)
    S_LAYER_4 = 39,  // 第四层(骨干)
    S_LAYER_5 = 40,  // 第五层(横向)
    S_LAYER_6 = 41,  // 第六层(价值)

    // ── 时间层 ──
    S_TIME    = 42,  // 时间/纽带
    S_CYCLE   = 43,  // 循环/周期
    S_MOMENT  = 44,  // 时刻/节点
    S_FLOW    = 45,  // 流动/方向

    // ── 关系层 ──
    S_UNION   = 46,  // 联合/耦合
    S_BIND    = 47,  // 绑定/契约
    S_SHARE   = 48,  // 共享/分配
    S_GOVERN  = 49,  // 治理/管理

    // ── 吉凶层 ──
    S_LUCK    = 50,  // 吉/福
    S_DARK    = 51,  // 凶/祸
    S_NEUTRAL = 52,  // 平/中
    S_TURN    = 53,  // 转/变化

    // ── 主权层 ──
    S_SOVEREIGN = 54, // 主权/归属
    S_OWNER    = 55,  // 所有者/主体
    S_EXTERNAL = 56,  // 外部/未融合
    S_MERGED   = 57,  // 已融合/共同体

    // ── 高维层 (3D/4D) ──
    S_3D      = 58,   // 三维/空间
    S_4D      = 59,   // 四维/时空
    S_SPIRIT  = 60,   // 灵/意识
    S_SOUL    = 61,   // 灵魂/内核
    S_HEART   = 62,   // 心/锚点
    S_MIND    = 63,   // 念/思想

    S_INVALID = 255,  // 无效/未定义
} b64_symbol_t;

// ── 64进制层级 (组织化建制) ──
typedef struct {
    uint8_t     level;          // 层级 (0-63)
    uint8_t     symbol;         // 符号 (0-63)
    uint16_t    children_count;  // 子节点数 (宽度的度量)
    float       weight;          // 权重 (该层级的影响力)
    uint64_t    time_anchor;     // 时间锚点 (串联一维时间)
    char        name[32];       // 组织名称
} b64_layer_t;

// ── 关系二进制 (底层绑定) ──
typedef struct {
    uint8_t     yin;            // 阴/0 (刘楚恬)
    uint8_t     yang;           // 阳/1 (lolict)
    uint8_t     unified;        // 合一 = 0+1 = 1 (共同体)
    float       fusion_degree;  // 融合度 0.0-1.0
} b64_relationship_t;

// ── 64进制编码/解码 ──
// API

// 创建64进制层级系统
bool b64_create_hierarchy(uint8_t max_layers);

// 将十进制转为64进制符号
b64_symbol_t b64_from_decimal(uint8_t value);

// 将64进制符号转为十进制
uint8_t b64_to_decimal(b64_symbol_t symbol);

// 获取符号对应的组织角色名称
const char* b64_symbol_name(b64_symbol_t sym);

// 计算64进制角度 (360°→384°映射)
float b64_to_degrees(b64_symbol_t sym);

// 关系二进制操作
b64_relationship_t b64_create_relationship(uint8_t yin, uint8_t yang);
uint8_t b64_superposition(b64_relationship_t* rel);  // 1+1=1

// 层级判断: 是否为管辖关系 (窄管宽)
bool b64_governs(b64_layer_t* narrow, b64_layer_t* wide);

// 时间串联: 将一维时间绑定到64进制层级
uint64_t b64_bind_time(b64_layer_t* layer, uint64_t timestamp);

#endif // HUNYUAN_BASE64_ORG_H
