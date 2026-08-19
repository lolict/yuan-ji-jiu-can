/**
 * hanzi_table.h — 汉字感知场 · 容器化预加载表
 *
 * 设计原则 (满全法 · 无哈希):
 *   · 哈希定位是蒙面人, 不知道它内部怎么把符号搅浑
 *   · 我们的路由是透明的: 感知家族 → 镜像对比 → 三元裁判 → 容器化
 *   · 1257字被分为5个感知家族, 每个家族是一个"容器入口"
 *   · 查找时逐一镜像对比, 三元裁判通过后容器化编码
 *
 * 架构:
 *   ┌─────────────┐
 *   │  输入符号    │ (UCS-2 + 声韵调)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 感知家族     │ 分别器第一层 (字形分类)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 镜像观察     │ 族内逐一镜像对比
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 三元裁判     │ 感知确认 + 分类归属 + 动力充足
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 容器化       │ 32位编码 (可降维至一元时间线)
 *   └──────┬──────┘
 *          ▼
 *   ┌─────────────┐
 *   │ 筷子夹持     │ 二元公倍数夹持 → 投入流水线
 *   └─────────────┘
 */

#ifndef NJSG_HANZI_TABLE_H
#define NJSG_HANZI_TABLE_H

#include "perception_field.h"
#include <stdint.h>

/* ── 配置 ────────────────────────────────────────────── */
#define HANZI_MAX_CHARS     1257  /* 目标容量 */
#define HANZI_ACTUAL_COUNT  1257  /* 全量装载 (当前实际数) */

/* ═══════════════════════════════════════════════════════════════
 * API · 感知场路由
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化感知场 (裁判引擎 + 流水线)
 *   hanzi_init() 必须在任何其他 API 之前调用一次
 */
void hanzi_init(void);

/**
 * 感知场路由主入口: UCS-2 + 声韵调 → 容器编码
 *   返回 0 = 未通过裁判; 非0 = 32位容器编码
 */
uint32_t hanzi_route(uint16_t ucs2, uint8_t sm_idx, uint8_t ym_idx, uint8_t tone);

/**
 * 反查: 容器编码 → UCS-2
 *   若不存在返回 0xFFFF
 */
uint16_t hanzi_to_ucs2(uint32_t container_code);

/**
 * UCS-2 → 拼音查找 (回退用, 当调用者不知拼音时)
 *   返回 0=找到, -1=未找到
 */
int hanzi_ucs2_to_pinyin(uint16_t ucs2, uint8_t *sm_idx, uint8_t *ym_idx, uint8_t *tone);

/**
 * 按拼音查找首个匹配字
 *   返回 UCS-2 编码, 找不到返回 -1
 */
int hanzi_find_by_pinyin(int initial, int final, int tone);

/**
 * 获取表内汉字总数
 */
int hanzi_count(void);

/**
 * 获取裁判统计 (准入/拒绝次数)
 */
uint32_t hanzi_get_judge_stats(uint32_t *admit, uint32_t *reject);

#endif /* NJSG_HANZI_TABLE_H */
