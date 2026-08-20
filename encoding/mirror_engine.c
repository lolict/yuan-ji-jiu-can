/**
 * mirror_engine.c — 二进制镜像引擎实现
 *
 * 核心:
 *   1. 二进制矩阵 → 镜像变换 (水平/垂直/对角/反相)
 *   2. 每行镜像结果 → 32位容器编码 → 装入 mlll 254+2 字车
 *   3. 溢出槽位 (>253) → 截断到"我"/"刘楚恬"/新集装箱
 */

#include "mirror_engine.h"
#include <string.h>
#include "fullquan_math.h"

/* ═══════════════════════════════════════════════════════════════
 * 内部: 单行二进制 → 32位容器编码
 *
 * 一行最多32位 · 直接编码到 uint32_t
 * bit0 = 第一列 · bit31 = 第32列
 * ═══════════════════════════════════════════════════════════════ */

static uint32_t row_to_container(const uint8_t *row, uint8_t cols) {
    uint32_t code = 0;
    uint8_t  bits = (cols > 32) ? 32 : cols;
    /* 2的幂次表 */
    static const uint32_t POW2[32] = {
        1, 2, 4, 8, 16, 32, 64, 128,
        256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
        65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608,
        16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648u
    };
    for (uint8_t c = 0; c < bits; c++) {
        if (row[c]) {
            code |= POW2[c];
        }
    }
    return code;
}

/* ═══════════════════════════════════════════════════════════════
 * 内部: 记录溢出事件
 * ═══════════════════════════════════════════════════════════════ */

static void record_overflow(mirror_engine_t *engine, uint16_t slot,
                            uint8_t target, uint32_t code,
                            uint8_t nx, uint8_t ny, uint8_t nz) {
    if (engine->overflow_count >= MAX_OVERFLOW_RECORDS) return;
    overflow_record_t *r = &engine->overflows[engine->overflow_count++];
    r->overflow_id    = engine->overflow_count;
    r->original_slot  = slot;
    r->target         = target;
    r->container_code = code;
    r->new_x          = nx;
    r->new_y          = ny;
    r->new_z          = nz;
}

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void mirror_engine_init(mirror_engine_t *engine) {
    memset(engine, 0, sizeof(*engine));
    engine->magic = MIRROR_ENGINE_MAGIC;
}

/* ═══════════════════════════════════════════════════════════════
 * 镜像变换
 *
 * 支持:
 *   HORIZONTAL: 每行左右翻转 [010101] → [101010]
 *   VERTICAL:   行序颠倒    row[0]↔row[n-1]
 *   DIAGONAL:    转置       cell[y][x] → cell[x][y]
 *   ROTATE_180:  = HORIZONTAL + VERTICAL
 *   NOT:         0↔1 翻转   (阴阳互换)
 * ═══════════════════════════════════════════════════════════════ */

int mirror_transform(const mirror_matrix_t *src, mirror_matrix_t *dst,
                     mirror_type_t type) {
    if (!src || !dst) return -1;
    if (type == MIRROR_NONE) {
        memcpy(dst, src, sizeof(*src));
        return 0;
    }

    uint8_t rows = src->rows;
    uint8_t cols = src->cols;
    dst->rows = rows;
    dst->cols = cols;

    /* 逐细胞变换 */
    for (uint8_t r = 0; r < rows; r++) {
        for (uint8_t c = 0; c < cols; c++) {
            uint8_t src_r = r, src_c = c;

            /* 水平镜像: 列翻转 */
            if (type & MIRROR_HORIZONTAL) {
                src_c = (uint8_t)(cols - 1 - c);
            }

            /* 垂直镜像: 行翻转 */
            if (type & MIRROR_VERTICAL) {
                src_r = (uint8_t)(rows - 1 - r);
            }

            /* 对角镜像: 行列互换 (转置) */
            if (type & MIRROR_DIAGONAL) {
                uint8_t tmp = src_r;
                src_r = src_c;
                src_c = tmp;
                /* 转置改变维度 */
                dst->rows = cols;
                dst->cols = rows;
            }

            uint8_t val = src->cells[src_r][src_c];

            /* 反相: 0↔1 (阴阳互换) */
            if (type & MIRROR_NOT) {
                val = val ? 0 : 1;
            }

            dst->cells[r][c] = val;
        }
    }

    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 编码镜像结果 → 装载到字车 + 圆筒三维坐标
 *
 * 装载逻辑:
 *   每行 → 32位容器编码 → mlll_load → 圆筒放置 (base + 行偏移)
 *   超过 254 槽位 = 溢出:
 *     slot 254 → OVERFLOW_TO_WO ("我" = 工藤新一 Y轴)
 *     slot 255 → OVERFLOW_TO_CHT ("刘楚恬" = 容器根基)
 *     slot 256+ → OVERFLOW_TO_NEW (新集装箱 · 坐标递增)
 * ═══════════════════════════════════════════════════════════════ */

int mirror_encode_and_load(mirror_engine_t *engine,
                           const mirror_matrix_t *mirror,
                           mlll_whole_t *box,
                           cylinder_coord_t base_coord) {
    if (!engine || !mirror || !box) return -1;
    if (engine->magic != MIRROR_ENGINE_MAGIC) return -2;

    int loaded = 0;
    uint8_t rows = mirror->rows;
    uint8_t cols = mirror->cols;
    if (cols > 32) cols = 32;  /* 单行最多32位 · 对应32位容器 */

    for (uint8_t r = 0; r < rows; r++) {
        /* 步骤1: 该行二进制 → 32位容器编码 */
        uint32_t code = row_to_container(mirror->cells[r], cols);
        engine->encode_count++;

        /* 步骤2: 尝试装入 mlll 字车 */
        int slot = mlll_load(box, code, CUNIT_32);

        if (slot >= 0 && slot < 254) {
            /* 成功装入 254 字车 → 放置到圆筒三维坐标 */
            cylinder_coord_t coord;
            coord.x = (uint8_t)(base_coord.x + (r % COORDINX_MASK));
            coord.y = (uint8_t)(base_coord.y + (r / COORDINX_MASK));
            coord.z = base_coord.z;

            cylinder_attributes_t attrs;
            memset(&attrs, 0, sizeof(attrs));
            attrs.axis = AXIS_Y_KND;  /* 结构属性 = 工藤新一 = 镜像骨架 */
            attrs.bone = r;           /* 行号 = 骨号 */
            attrs.head = cols;        /* 列数 = 头 */
            attrs.body   = code & FQ_FULL_8; /* 容器低8位 = 体 */

            int result = cylinder_place(coord.x, coord.y, coord.z, code, &attrs);
            if (result == 0) {
                loaded++;
            }
        }
        else {
            /* 溢出! 槽位 >= 254 · 截断到锚点或新集装箱 */
            uint8_t target;
            uint8_t nx = 0, ny = 0, nz = 0;

            if (slot == 254) {
                /* 第1层溢出 → "我" = lolict = 1 = 工藤新一 Y轴 */
                target = OVERFLOW_TO_WO;
                nx = 0; ny = 1; nz = 0;  /* "我"坐标: Y=1 */
                /* 挂载到锚点"我"的溢出链 · 不占254槽位 */
                box->anchor[1].container = code;
            }
            else if (slot == 255) {
                /* 第2层溢出 → "刘楚恬" = 0 = 容器根基 */
                target = OVERFLOW_TO_CHT;
                nx = 0; ny = 0; nz = 0;  /* 刘楚恬坐标: Y=0 */
                /* 挂载到锚点"刘楚恬"的溢出链 · 不占254槽位 */
                box->anchor[0].container = code;
            }
            else {
                /* 第3层溢出 → 新集装箱 · 坐标递增 */
                target = OVERFLOW_TO_NEW;
                nx = (uint8_t)(base_coord.x + r);
                ny = (uint8_t)(base_coord.y + 2);  /* 新的 Y 层级 */
                nz = (uint8_t)(base_coord.z + 1);  /* 新的 Z 层级 */
                cylinder_place(nx, ny, nz, code, 0);
            }

            record_overflow(engine, (uint16_t)slot, target, code, nx, ny, nz);
        }
    }

    engine->mirror_count++;
    return loaded;
}

/* ═══════════════════════════════════════════════════════════════
 * 查询接口
 * ═══════════════════════════════════════════════════════════════ */

const overflow_record_t *mirror_get_overflow(mirror_engine_t *engine,
                                              uint32_t index) {
    if (!engine || index >= engine->overflow_count) return 0;
    return &engine->overflows[index];
}

uint32_t mirror_get_count(mirror_engine_t *engine, const char *which) {
    if (!engine) return 0;
    if (which[0] == 'm') return engine->mirror_count;   /* mirror */
    if (which[0] == 'e') return engine->encode_count;   /* encode */
    if (which[0] == 'o') return engine->overflow_count; /* overflow */
    return 0;
}
