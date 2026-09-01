/**
 * container_ontology_bridge.h — 容器本体论统一桥接
 *
 * 把本轮建立的全部能力打包成一个统一出口:
 *   ┌──────────────────────────────────────────────────┐
 *   │               容器本体论统一桥接                    │
 *   ├──────────────────────────────────────────────────┤
 *   │ 子容器1: 编码系统 (1257字·五大家族·YinYang)       │
 *   │ 子容器2: 全局控制 (地图·遍历·判断·决策树·聂小倩)  │
 *   │ 子容器3: 产权流   (vA+vB==2·v_core · 9法条)      │
 *   │ 子容器4: 部首星图 (水·火·言三部 · 容器拥有树)     │
 *   │ 子容器5: 免疫记忆 (负样本·回写协议·自动冻结)      │
 *   └──────────────────────────────────────────────────┘
 *
 * 使用方式:
 *   #include "container_ontology_bridge.h"
 *   container_ontology_init();
 *   uint32_t code = container_encode_utf8("囡");
 *   int infr = container_property_judge(100, 120, 80);
 *
 * 如需完整产权类型定义, 额外 include "property_judge.h"
 *
 * 核心防撞墙法则:
 *   法则1: 跨容器遍历 — 当前容器用完去外部取, 不重复重启
 *   法则2: 本质匹配 — 看发音位置+视觉外观, 不看编码关系
 *   法则3: 双重平衡 — 总量平衡≠个体平衡, vA==vB==v_core
 *   法则4: 免疫记忆 — 错误是疫苗, 必须保存
 *   法则5: 维度生长 — 0维→1维=秩序生长, 折叠=新层
 */

#ifndef NJSG_CONTAINER_ONTOLOGY_BRIDGE_H
#define NJSG_CONTAINER_ONTOLOGY_BRIDGE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── 一次性初始化全部子容器 ── */
void container_ontology_init(void);

/* ═══════════════════════════════════════════════════════════
 * 子容器1: 编码系统
 * ═══════════════════════════════════════════════════════════ */

uint32_t container_encode_utf8(const char *utf8);
int      container_get_family(uint32_t code);
int      container_get_polarity(uint32_t code);
int      container_hanzi_count(void);

/* ═══════════════════════════════════════════════════════════
 * 子容器2: 全局控制
 * ═══════════════════════════════════════════════════════════ */

int  container_traversal_advance(void);
int  container_nie_xiaoqian(uint32_t ghost_code, uint32_t *body_out);
int  container_recover(void);

/* ═══════════════════════════════════════════════════════════
 * 子容器3: 产权流
 * 返回: 0=平衡, 1=虚报, 2=侵吞, 3=混淆, 4=篡改, 5=双方虚报
 * ═══════════════════════════════════════════════════════════ */

int  container_property_judge(uint32_t v_core, uint32_t v_a, uint32_t v_b);

/* ═══════════════════════════════════════════════════════════
 * 子容器4: 部首星图
 * ═══════════════════════════════════════════════════════════ */

int  container_star_count(void);
int  container_star_group_size(uint8_t radical_idx);

/* ═══════════════════════════════════════════════════════════
 * 子容器5: 免疫记忆 (简化接口, 完整实现见 negative_sample.h)
 * ═══════════════════════════════════════════════════════════ */

int  container_record_failure(uint32_t pattern, const char *reason);
int  container_is_known_failure(uint32_t pattern);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_CONTAINER_ONTOLOGY_BRIDGE_H */
