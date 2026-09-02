/**
 * mind_organ.c — 距离感知聚类实现
 *
 * DBSCAN 变体 + 多维度距离度量
 */

#include "mind_organ.h"
#include <string.h>
#include <math.h>

/* ── 距离计算 ────────────────────────────────────────────── */

NJSG_EXPORT NJSG_float mo_distance(NJSG_uint64 a, NJSG_uint64 b, mo_dist_type_t type) {
    if (type == MO_DIST_SOVEREIGN) {
        /* 主权距离: |sovereignty_a - sovereignty_b| 归一化 */
        NJSG_uint32 sa = WK_CELL_SOVEREIGN(a);
        NJSG_uint32 sb = WK_CELL_SOVEREIGN(b);
        NJSG_int32 diff = (NJSG_int32)(sa > sb ? sa - sb : sb - sa);
        return (NJSG_float)diff / (NJSG_float)0xFFFFFF;
    }
    else if (type == MO_DIST_POLARITY) {
        /* 极性距离: 不同极性 = 1 · 相同 = 0 */
        return (WK_CELL_POLARITY(a) == WK_CELL_POLARITY(b)) ? 0.0f : 1.0f;
    }
    else if (type == MO_DIST_WITNESS) {
        /* 见证距离 */
        NJSG_uint8 wa = WK_CELL_WITNESS(a);
        NJSG_uint8 wb = WK_CELL_WITNESS(b);
        NJSG_int16 diff = (NJSG_int16)(wa > wb ? wa - wb : wb - wa);
        return (NJSG_float)diff / 15.0f;
    }
    else if (type == MO_DIST_MCCP) {
        /* 码点距离 */
        NJSG_uint32 ma = WK_CELL_MCP(a);
        NJSG_uint32 mb = WK_CELL_MCP(b);
        NJSG_int32 diff = (NJSG_int32)(ma > mb ? ma - mb : mb - ma);
        return (NJSG_float)diff / (NJSG_float)0xFFFFFFFF;
    }
    else {
        /* 综合几何距离: 各维度加权合成 */
        NJSG_float d_sov = mo_distance(a, b, MO_DIST_SOVEREIGN);
        NJSG_float d_pol = mo_distance(a, b, MO_DIST_POLARITY);
        NJSG_float d_wit = mo_distance(a, b, MO_DIST_WITNESS);
        NJSG_float d_mcp = mo_distance(a, b, MO_DIST_MCCP);
        
        /* 权重: 主权 50% · 极性 20% · 见证 15% · 码点 15% */
        NJSG_float dist = 0.50f * d_sov + 0.20f * d_pol + 0.15f * d_wit + 0.15f * d_mcp;
        return dist;
    }
}

/* ── 距离矩阵 ────────────────────────────────────────────── */

NJSG_EXPORT void mo_build_matrix(const NJSG_uint64 *cells, NJSG_uint32 count,
                                  mo_distmatrix_t *matrix, mo_dist_type_t type) {
    if (!cells || !matrix || count == 0 || count > MO_MAX_CELLS) return;
    
    matrix->count = count;
    for (NJSG_uint32 i = 0; i < count; i++) {
        matrix->ids[i] = (NJSG_uint16)i;
        matrix->matrix[i * count + i] = 0.0f;
        for (NJSG_uint32 j = i + 1; j < count; j++) {
            NJSG_float d = mo_distance(cells[i], cells[j], type);
            matrix->matrix[i * count + j] = d;
            matrix->matrix[j * count + i] = d;
        }
    }
}

/* ── DBSCAN 聚类 ─────────────────────────────────────────── */

NJSG_EXPORT void mo_cluster(const NJSG_uint64 *cells, NJSG_uint32 count,
                             NJSG_float eps, NJSG_uint16 min_pts,
                             mo_dist_type_t type, mo_result_t *result) {
    if (!cells || !result || count == 0 || count > MO_MAX_CELLS) return;
    
    memset(result, 0, sizeof(*result));
    result->dist_type = type;
    
    /* 构建距离矩阵 */
    mo_distmatrix_t matrix;
    mo_build_matrix(cells, count, &matrix, type);
    
    /* 访问标记: 0=未访问, -1=噪声, >0=簇ID */
    int labels[MO_MAX_CELLS];
    memset(labels, 0, sizeof(labels));
    
    NJSG_uint16 cluster_id = 0;
    
    for (NJSG_uint32 i = 0; i < count; i++) {
        if (labels[i] != 0) continue;  /* 已处理 */
        
        /* 找 i 的 ε-邻域 */
        NJSG_uint16 neighbors[MO_MAX_CELLS];
        NJSG_uint16 n_count = 0;
        
        for (NJSG_uint32 j = 0; j < count; j++) {
            if (matrix.matrix[i * count + j] <= eps) {
                neighbors[n_count++] = (NJSG_uint16)j;
            }
        }
        
        if (n_count < min_pts) {
            labels[i] = -1;  /* 标记为噪声 */
            continue;
        }
        
        /* 创建新簇 */
        cluster_id++;
        if (cluster_id > MO_MAX_CLUSTERS) break;
        
        NJSG_uint16 cidx = cluster_id - 1;
        result->clusters[cidx].member_count = 0;
        result->clusters[cidx].centroid_id = (NJSG_uint16)i;
        
        /* 添加核心点 */
        labels[i] = cluster_id;
        result->clusters[cidx].member_ids[result->clusters[cidx].member_count++] = (NJSG_uint16)i;
        
        /* 扩展簇 */
        for (NJSG_uint16 n = 0; n < n_count; n++) {
            NJSG_uint16 ni = neighbors[n];
            
            if (labels[ni] == -1) {
                /* 边界点加入当前簇 */
                labels[ni] = cluster_id;
                result->clusters[cidx].member_ids[result->clusters[cidx].member_count++] = ni;
            }
            
            if (labels[ni] != 0) continue;
            
            labels[ni] = cluster_id;
            result->clusters[cidx].member_ids[result->clusters[cidx].member_count++] = ni;
            
            /* 找 ni 的邻域 */
            NJSG_uint16 sub_neighbors[MO_MAX_CELLS];
            NJSG_uint16 sn_count = 0;
            for (NJSG_uint32 k = 0; k < count; k++) {
                if (matrix.matrix[ni * count + k] <= eps) {
                    sub_neighbors[sn_count++] = (NJSG_uint16)k;
                }
            }
            
            if (sn_count >= min_pts) {
                /* 合并 */
                for (NJSG_uint16 s = 0; s < sn_count; s++) {
                    NJSG_uint16 si = sub_neighbors[s];
                    /* 标记已归属 */
                    if (labels[si] == 0 || labels[si] == -1) {
                        labels[si] = cluster_id;
                    }
                    NJSG_bool exists = NJSG_FALSE;
                    for (NJSG_uint16 m = 0; m < result->clusters[cidx].member_count; m++) {
                        if (result->clusters[cidx].member_ids[m] == si) {
                            exists = NJSG_TRUE;
                            break;
                        }
                    }
                    if (!exists) {
                        result->clusters[cidx].member_ids[result->clusters[cidx].member_count++] = si;
                    }
                }
            }
        }
    }
    
    result->cluster_count = cluster_id;
    
    /* 统计噪声点 */
    result->noise_count = 0;
    for (NJSG_uint32 i = 0; i < count; i++) {
        if (labels[i] == -1) result->noise_count++;
    }
    
    /* 计算内聚度 */
    NJSG_float total_cohesion = 0.0f;
    for (NJSG_uint16 c = 0; c < result->cluster_count; c++) {
        mo_cluster_t *cl = &result->clusters[c];
        if (cl->member_count < 2) {
            cl->cohesion = 1.0f;
        } else {
            /* 平均距离越小 = 内聚度越高 */
            NJSG_float avg_dist = 0.0f;
            NJSG_uint16 pairs = 0;
            for (NJSG_uint16 m1 = 0; m1 < cl->member_count; m1++) {
                for (NJSG_uint16 m2 = m1 + 1; m2 < cl->member_count; m2++) {
                    NJSG_uint32 i1 = cl->member_ids[m1];
                    NJSG_uint32 i2 = cl->member_ids[m2];
                    avg_dist += matrix.matrix[i1 * count + i2];
                    pairs++;
                }
            }
            avg_dist = pairs > 0 ? avg_dist / pairs : 0.0f;
            cl->cohesion = 1.0f / (1.0f + avg_dist * 10.0f);
        }
        total_cohesion += cl->cohesion;
    }
    result->avg_cohesion = result->cluster_count > 0 ? total_cohesion / result->cluster_count : 0.0f;
}

/* ── 最近邻 ──────────────────────────────────────────────── */

NJSG_EXPORT int mo_nearest(const NJSG_uint64 *cells, NJSG_uint32 count,
                            NJSG_uint32 idx, mo_dist_type_t type) {
    if (!cells || idx >= count || count < 2) return -1;
    
    NJSG_float min_dist = 1e10f;
    int nearest = -1;
    
    for (NJSG_uint32 i = 0; i < count; i++) {
        if (i == idx) continue;
        NJSG_float d = mo_distance(cells[idx], cells[i], type);
        if (d < min_dist) {
            min_dist = d;
            nearest = (int)i;
        }
    }
    
    return nearest;
}

/* ── 统计字符串 ──────────────────────────────────────────── */

NJSG_EXPORT void mo_stats(const mo_result_t *result, char *buf, NJSG_size buf_size) {
    if (!result || !buf || buf_size == 0) return;
    
    /* 简化 sprintf (WASM 友好) */
    NJSG_size pos = 0;
    
    #define MO_APPEND(s) do { \
        const char *_p = (s); \
        while (*_p && pos < buf_size - 1) buf[pos++] = *_p++; \
    } while(0)
    
    const char *dist_names[] = {"几何", "主权", "极性", "见证", "码点"};
    const char *dname = (result->dist_type <= MO_DIST_MCCP) ? dist_names[result->dist_type] : "?";
    
    MO_APPEND("MO clusters=");
    /* 数字转字符 */
    char num[12];
    NJSG_uint32 vals[3] = {result->cluster_count, result->noise_count, (NJSG_uint32)(result->avg_cohesion * 100)};
    const char *labels[] = {"", " noise=", " cohesion="};
    for (int i = 0; i < 3; i++) {
        MO_APPEND(labels[i]);
        int n = 11;
        NJSG_uint32 v = vals[i];
        if (v == 0) {
            num[--n] = '0';
        } else {
            while (v > 0 && n > 0) {
                num[--n] = '0' + (v % 10);
                v /= 10;
            }
        }
        for (int j = n; j < 11; j++) buf[pos++] = num[j];
        if (pos < buf_size - 1) buf[pos++] = ' ';
    }
    if (pos > 0 && buf[pos-1] == ' ') pos--;
    if (pos < buf_size - 1) buf[pos++] = '\n';
    
    /* 各簇摘要 */
    for (NJSG_uint16 c = 0; c < result->cluster_count && pos < buf_size - 40; c++) {
        mo_cluster_t *cl = &result->clusters[c];
        MO_APPEND("  [");
        int n = 11;
        NJSG_uint32 v = c;
        if (v == 0) { num[--n] = '0'; }
        else { while (v > 0 && n > 0) { num[--n] = '0' + (v % 10); v /= 10; } }
        for (int j = n; j < 11; j++) buf[pos++] = num[j];
        MO_APPEND("] size=");
        n = 11;
        v = cl->member_count;
        if (v == 0) { num[--n] = '0'; }
        else { while (v > 0 && n > 0) { num[--n] = '0' + (v % 10); v /= 10; } }
        for (int j = n; j < 11; j++) buf[pos++] = num[j];
        MO_APPEND(" cohesion=");
        n = 11;
        v = (NJSG_uint32)(cl->cohesion * 100);
        if (v == 0) { num[--n] = '0'; }
        else { while (v > 0 && n > 0) { num[--n] = '0' + (v % 10); v /= 10; } }
        for (int j = n; j < 11; j++) buf[pos++] = num[j];
        if (pos < buf_size - 1) buf[pos++] = '\n';
    }
    
    buf[pos] = '\0';
    
    #undef MO_APPEND
}
