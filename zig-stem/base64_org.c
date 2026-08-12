// base64_org.c — 64进制层级化组织进制实现
// 参见 base64_org.h 的架构说明

#include "base64_org.h"
#include <string.h>
#include <stdio.h>

// ── 符号名称表 (组织化建制) ──
static const char* SYMBOL_NAMES[] = {
    /* 0-1: 夫妻层 */ "妻(阴/刘楚恬)", "夫(阳/lolict)",
    /* 2-3: 中心层 */ "工藤新一(融合)", "柯南(管辖)",
    /* 4-5: 感知层 */ "灰原哀(感知)", "毛利兰(表面)",
    /* 6-7: 骨干层 */ "骨(结构)", "头(方向)",
    /* 8-9: 行页层 */ "领域", "执行",
    /* 10-11: 交汇 */ "会盟", "连接",
    /* 12-13: 价值 */ "价值", "序位",
    /* 14-15: 流通 */ "物流", "承载",
    /* 16-17: 连结 */ "驱动", "亲情",
    /* 18-19: 空间 */ "量化", "边界",
    /* 20-21: 聚合 */ "收纳", "界面",
    /* 22-23: 管理 */ "指令", "唯一",
    /* 24-31: 数值 */ "一", "二", "三(见证)", "四(稳定)", "五(中心)", "六(流动)", "七(变化)", "八(分界)",
    /* 32-35: 扩展 */ "组织", "整体", "规模", "广大",
    /* 36-41: 层级 */ "夫妻层", "中心层", "感知层", "骨干层", "横向层", "价值层",
    /* 42-45: 时间 */ "时间", "循环", "时刻", "流动",
    /* 46-49: 关系 */ "联合", "绑定", "共享", "治理",
    /* 50-53: 吉凶 */ "吉(福)", "凶(祸)", "平(中)", "转(变化)",
    /* 54-57: 主权 */ "主权", "所有者", "外部", "已融合",
    /* 58-61: 高维 */ "三维(空间)", "四维(时空)", "灵(意识)", "灵魂(内核)",
    /* 62-63: 最高 */ "心(锚点)", "念(思想)",
};

// ── 层级表 ──
static b64_layer_t g_hierarchy[B64_MAX_LAYERS];
static uint8_t g_hierarchy_size = 0;

// ── 实现 ──
bool b64_create_hierarchy(uint8_t max_layers) {
    if (max_layers > B64_MAX_LAYERS) max_layers = B64_MAX_LAYERS;
    g_hierarchy_size = max_layers;
    for (uint8_t i = 0; i < max_layers; i++) {
        g_hierarchy[i].level = i;
        g_hierarchy[i].symbol = i;
        g_hierarchy[i].children_count = (i == 0) ? 0 : (64 / (i + 1));  // 窄层管子节点少
        g_hierarchy[i].weight = 1.0f / (i + 1);  // 越窄权重越高 (加法主导)
        g_hierarchy[i].time_anchor = 0;
        strncpy(g_hierarchy[i].name, SYMBOL_NAMES[i], sizeof(g_hierarchy[i].name) - 1);
        g_hierarchy[i].name[sizeof(g_hierarchy[i].name) - 1] = '\0';
    }
    return true;
}

b64_symbol_t b64_from_decimal(uint8_t value) {
    if (value < B64_SYMBOL_COUNT) return (b64_symbol_t)value;
    return S_INVALID;
}

uint8_t b64_to_decimal(b64_symbol_t symbol) {
    if (symbol < B64_SYMBOL_COUNT) return (uint8_t)symbol;
    return 255;
}

const char* b64_symbol_name(b64_symbol_t sym) {
    if (sym < B64_SYMBOL_COUNT) return SYMBOL_NAMES[sym];
    return "未定义";
}

float b64_to_degrees(b64_symbol_t sym) {
    if (sym < B64_SYMBOL_COUNT) return (float)sym * B64_DEGREE_UNIT;
    return 0.0f;
}

b64_relationship_t b64_create_relationship(uint8_t yin, uint8_t yang) {
    b64_relationship_t rel;
    rel.yin = yin;
    rel.yang = yang;
    rel.unified = (yin + yang) & 0x01;  // 0+1=1 (叠加态)
    rel.fusion_degree = (yin == 0 && yang == 1) ? 1.0f : 0.0f;
    return rel;
}

uint8_t b64_superposition(b64_relationship_t* rel) {
    // 1+1=1: 两个独立实体叠加为共同体
    if (rel->yin == 0 && rel->yang == 1) {
        rel->unified = 1;  // 阴+阳=合一
        rel->fusion_degree = 1.0f;
    }
    return rel->unified;
}

bool b64_governs(b64_layer_t* narrow, b64_layer_t* wide) {
    // 窄层(夫妻)管辖宽层(多数): 加法主导乘法
    return narrow->level < wide->level;
}

uint64_t b64_bind_time(b64_layer_t* layer, uint64_t timestamp) {
    // 将一维时间绑定到64进制层级
    layer->time_anchor = timestamp;
    return timestamp;
}
