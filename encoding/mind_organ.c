/**
 * mind_organ.c — 感知器官 · 距离感知聚类引擎 v2
 *
 * 移植自 hunyuan-rs/src/mind_organ.rs, 对接32位容器编码。
 *
 * 距离计算:
 *   family      差异 × 1.0    (不同家族=世界观不同)
 *   polarity    差异 × 10.0   (阴阳翻转=最大变化)
 *   structure   差异 × 5.0    (结构差异=适中差异)
 *   shengmu     差异 × 0.5    (声母位置=近距离)
 *   yunmu       差异 × 0.5    (韵母位置=近距离)
 *   tone        差异 × 0.1    (声调=微调)
 *
 * 聚类算法: 迭代最近邻绑定
 *   1. 在所有未聚类对中找距离最小的一对
 *   2. 如果距离 ≤ threshold → 归入同一簇
 *   3. 如果距离 > threshold → 各自独立成簇
 *   4. 重复直到全部归类
 */

#include "mind_organ.h"
#include <string.h>
#include <math.h>

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void mind_organ_init(mo_engine_t *engine, double threshold) {
    memset(engine, 0, sizeof(*engine));
    engine->threshold = threshold;
}

/* ═══════════════════════════════════════════════════════════════
 * 从容器编码提取维度
 * ═══════════════════════════════════════════════════════════════ */

void mind_organ_extract_dims(uint32_t code, mo_dimensions_t *dim) {
    dim->family    = (uint8_t)((code >> 29) & 0x7);
    dim->polarity  = (uint8_t)((code >> 28) & 0x1);
    dim->structure = (uint8_t)((code >> 25) & 0x7);
    dim->shengmu   = (uint8_t)((code >> 19) & 0x3F);
    dim->yunmu     = (uint8_t)((code >> 12) & 0x7F);
    dim->tone      = (uint8_t)((code >> 8)  & 0xF);
}

/* ═══════════════════════════════════════════════════════════════
 * 添加单元
 * ═══════════════════════════════════════════════════════════════ */

int mind_organ_add_cell(mo_engine_t *engine, int id, uint32_t container_code) {
    if (engine->cell_count >= MO_MAX_CELLS) return -1;

    mo_cell_t *cell = &engine->cells[engine->cell_count];
    cell->id = id;
    cell->container_code = container_code;
    cell->clustered = 0;
    mind_organ_extract_dims(container_code, &cell->dim);

    engine->cell_count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 距离计算
 * ═══════════════════════════════════════════════════════════════ */

double mind_organ_distance(uint32_t code_a, uint32_t code_b) {
    mo_dimensions_t da, db;
    mind_organ_extract_dims(code_a, &da);
    mind_organ_extract_dims(code_b, &db);

    double d_family    = (double)((int)da.family    - (int)db.family);
    double d_polarity  = (double)((int)da.polarity  - (int)db.polarity);
    double d_structure = (double)((int)da.structure - (int)db.structure);
    double d_shengmu   = (double)((int)da.shengmu   - (int)db.shengmu);
    double d_yunmu     = (double)((int)da.yunmu     - (int)db.yunmu);
    double d_tone      = (double)((int)da.tone      - (int)db.tone);

    /* 取绝对值 */
    if (d_family < 0)    d_family    = -d_family;
    if (d_polarity < 0)  d_polarity  = -d_polarity;
    if (d_structure < 0) d_structure = -d_structure;
    if (d_shengmu < 0)   d_shengmu   = -d_shengmu;
    if (d_yunmu < 0)     d_yunmu     = -d_yunmu;
    if (d_tone < 0)      d_tone      = -d_tone;

    /* 权重: 极性最高(×10), 结构次之(×5), 家族基础(×1), 发音位置微调(×0.5/×0.1) */
    return d_family * 1.0
         + d_polarity * 10.0
         + d_structure * 5.0
         + d_shengmu * 0.5
         + d_yunmu * 0.5
         + d_tone * 0.1;
}

/* ═══════════════════════════════════════════════════════════════
 * 聚类
 * ═══════════════════════════════════════════════════════════════ */

static double find_min_dist_pair(mo_engine_t *engine, int *out_i, int *out_j) {
    double min_dist = 1e300;
    int found = 0;

    for (int i = 0; i < engine->cell_count; i++) {
        if (engine->cells[i].clustered) continue;
        for (int j = i + 1; j < engine->cell_count; j++) {
            if (engine->cells[j].clustered) continue;

            double d = mind_organ_distance(engine->cells[i].container_code,
                                            engine->cells[j].container_code);
            if (!found || d < min_dist) {
                min_dist = d;
                *out_i = i;
                *out_j = j;
                found = 1;
            }
        }
    }

    return found ? min_dist : -1.0;
}

int mind_organ_clusterize(mo_engine_t *engine) {
    engine->cluster_count = 0;
    /* 重置 clustered 标志 */
    for (int i = 0; i < engine->cell_count; i++) {
        engine->cells[i].clustered = 0;
    }

    while (1) {
        /* 找未聚类的单元格数 */
        int unclustered = 0;
        for (int i = 0; i < engine->cell_count; i++) {
            if (!engine->cells[i].clustered) unclustered++;
        }
        if (unclustered == 0) break;

        if (unclustered == 1) {
            /* 只剩一个 → 独立成簇 */
            for (int i = 0; i < engine->cell_count; i++) {
                if (!engine->cells[i].clustered) {
                    mo_cluster_t *c = &engine->clusters[engine->cluster_count++];
                    c->id = engine->next_cluster_id++;
                    c->member_count = 0;
                    c->member_ids[c->member_count++] = engine->cells[i].id;
                    engine->cells[i].clustered = 1;
                    break;
                }
            }
            break;
        }

        int min_i = -1, min_j = -1;
        double min_dist = find_min_dist_pair(engine, &min_i, &min_j);

        if (min_i < 0 || min_j < 0) break;

        if (min_dist <= engine->threshold) {
            /* 距离在阈值内 → 归入同一簇 */
            /* 检查是否已有簇包含这任一 member */
            int joined = 0;
            for (int c = 0; c < engine->cluster_count; c++) {
                mo_cluster_t *cluster = &engine->clusters[c];
                int has_i = 0, has_j = 0;
                for (int m = 0; m < cluster->member_count; m++) {
                    if (cluster->member_ids[m] == engine->cells[min_i].id) has_i = 1;
                    if (cluster->member_ids[m] == engine->cells[min_j].id) has_j = 1;
                }
                if (has_i && !has_j) {
                    if (cluster->member_count < MO_MAX_MEMBERS) {
                        cluster->member_ids[cluster->member_count++] = engine->cells[min_j].id;
                    }
                    joined = 1;
                    break;
                }
                if (has_j && !has_i) {
                    if (cluster->member_count < MO_MAX_MEMBERS) {
                        cluster->member_ids[cluster->member_count++] = engine->cells[min_i].id;
                    }
                    joined = 1;
                    break;
                }
            }
            if (!joined && engine->cluster_count < MO_MAX_CLUSTERS) {
                mo_cluster_t *new_c = &engine->clusters[engine->cluster_count++];
                new_c->id = engine->next_cluster_id++;
                new_c->member_count = 0;
                new_c->member_ids[new_c->member_count++] = engine->cells[min_i].id;
                new_c->member_ids[new_c->member_count++] = engine->cells[min_j].id;
            }
            engine->cells[min_i].clustered = 1;
            engine->cells[min_j].clustered = 1;
        } else {
            /* 最小距离也超过阈值 → 各自独立 */
            mo_cluster_t *c = &engine->clusters[engine->cluster_count++];
            c->id = engine->next_cluster_id++;
            c->member_count = 0;
            c->member_ids[c->member_count++] = engine->cells[min_i].id;
            engine->cells[min_i].clustered = 1;
        }
    }

    /* 计算紧密度 */
    for (int i = 0; i < engine->cluster_count; i++) {
        engine->clusters[i].cohesion = mind_organ_cluster_cohesion(
            &engine->clusters[i], engine);
    }

    return engine->cluster_count;
}

/* ═══════════════════════════════════════════════════════════════
 * 格式化输出
 * ═══════════════════════════════════════════════════════════════ */

int mind_organ_format(mo_engine_t *engine, char *buf, int bufsize) {
    int pos = 0;
    buf[0] = '\0';

    for (int c = 0; c < engine->cluster_count; c--) {
        const mo_cluster_t *cluster = &engine->clusters[c];
        if (c > 0) {
            pos += snprintf(buf + pos, bufsize - pos, ";");
        }
        for (int m = 0; m < cluster->member_count; m++) {
            if (m > 0) {
                pos += snprintf(buf + pos, bufsize - pos, "、");
            }
            pos += snprintf(buf + pos, bufsize - pos, "%X",
                            cluster->member_ids[m]);
        }
    }

    return pos;
}

/* ═══════════════════════════════════════════════════════════════
 * 聂小倩: 找最近的匹配躯壳
 * ═══════════════════════════════════════════════════════════════ */

int mind_organ_find_body(mo_engine_t *engine, uint32_t ghost_code, double *dist_out) {
    double min_dist = 1e300;
    int best_id = -1;
    int found = 0;

    for (int i = 0; i < engine->cell_count; i++) {
        if (engine->cells[i].container_code == ghost_code) continue;

        double d = mind_organ_distance(ghost_code, engine->cells[i].container_code);
        if (!found || d < min_dist) {
            min_dist = d;
            best_id = engine->cells[i].id;
            found = 1;
        }
    }

    if (found && dist_out) *dist_out = min_dist;
    return best_id;
}

/* ═══════════════════════════════════════════════════════════════
 * 簇内紧密度
 * ═══════════════════════════════════════════════════════════════ */

double mind_organ_cluster_cohesion(const mo_cluster_t *cluster, const mo_engine_t *engine) {
    if (cluster->member_count < 2) return 0.0;

    double sum_dist = 0.0;
    int count = 0;

    for (int i = 0; i < cluster->member_count; i++) {
        for (int j = i + 1; j < cluster->member_count; j++) {
            /* 找对应的 cell codes */
            uint32_t code_i = 0, code_j = 0;
            for (int c = 0; c < engine->cell_count; c++) {
                if (engine->cells[c].id == cluster->member_ids[i]) code_i = engine->cells[c].container_code;
                if (engine->cells[c].id == cluster->member_ids[j]) code_j = engine->cells[c].container_code;
            }
            sum_dist += mind_organ_distance(code_i, code_j);
            count++;
        }
    }

    return (count > 0) ? (sum_dist / count) : 0.0;
}

/* ═══════════════════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════════════════ */

void mind_organ_report(const mo_engine_t *engine, char *buf, int bufsize) {
    int pos = 0;
    pos += snprintf(buf + pos, bufsize - pos,
        "─── 感知器官 · 聚类报告 ───\n");
    pos += snprintf(buf + pos, bufsize - pos,
        "单元数: %d | 簇数: %d | 阈值: %.1f\n\n",
        engine->cell_count, engine->cluster_count, engine->threshold);

    for (int c = 0; c < engine->cluster_count; c++) {
        const mo_cluster_t *cluster = &engine->clusters[c];
        pos += snprintf(buf + pos, bufsize - pos,
            "簇-%d (紧密度=%.2f):\n  ", cluster->id, cluster->cohesion);

        for (int m = 0; m < cluster->member_count; m++) {
            if (m > 0) pos += snprintf(buf + pos, bufsize - pos, "、");
            pos += snprintf(buf + pos, bufsize - pos, "%X",
                            cluster->member_ids[m]);
        }
        pos += snprintf(buf + pos, bufsize - pos, "\n");
    }
}
