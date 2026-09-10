/**
 * global_control_bridge_kernel.h — 全局控制容器 → 内核桥接
 *
 * 让内核和编码系统通过全局控制容器来协调:
 *   - 注册容器 (声母/韵母/声调/家族)
 *   - 设置遍历规则 (声母遍历韵母)
 *   - 执行聂小倩匹配 (为符号找躯壳)
 *   - 决策树恢复 (卡住时不撞墙)
 *
 * 这是"用符号构建秩序"的代码化:
 *   不再是AI自行用编码关系(大小写)找符号,
 *   而是按主人定义的秩序(发音位置+审美)匹配符号。
 */

#ifndef NJSG_GLOBAL_CONTROL_BRIDGE_KERNEL_H
#define NJSG_GLOBAL_CONTROL_BRIDGE_KERNEL_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── 初始化 ── */
void global_control_bridge_init(void);

/* ── 注册容器 ── */
int  gcb_register_container(uint8_t id, const char *name,
                             uint8_t polarity, uint32_t boundary);

/* ── 遍历规则 ── */
int  gcb_add_traversal(uint8_t src, uint8_t dst, uint8_t priority);
int  gcb_get_next_traversal(uint8_t *src_out, uint8_t *dst_out);
int  gcb_advance_traversal(void);

/* ── 判断规则 ── */
void gcb_set_judgment(uint8_t match_type, uint8_t require_pol,
                        uint8_t require_aes, uint8_t tol);
int  gcb_judge_pair(uint32_t sym_a, uint32_t sym_b);

/* ── 聂小倩: 为鬼魂找躯壳 ── */
int  gcb_nie_xiaoqian(uint32_t ghost_code, uint32_t *body_out);

/* ── 决策树: 卡住时恢复 ── */
int  gcb_recover(void);

/* ── 感知器官: 距离感知聚类 (32 位容器编码) ── */
void gcb_mind_organ_init(double threshold);
int  gcb_mo_add_cell(int id, uint32_t container_code);
double gcb_mo_distance(uint32_t code_a, uint32_t code_b);
int  gcb_mo_clusterize(void);
int  gcb_mo_find_body(uint32_t ghost_code, double *dist_out);
int  gcb_mo_cluster_count(void);
void gcb_mo_report(char *buf, int bufsize);

/* ── 完整演示 ── */
void global_control_bridge_test(void);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_GLOBAL_CONTROL_BRIDGE_KERNEL_H */
