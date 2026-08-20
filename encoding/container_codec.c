/**
 * container_codec.c — 容器渲染框架实现
 *
 * 核心:
 *   1bit封装16个字 · 绝缘体墙 · 截止=开始 · 零维点
 *   三种模式: FULL全量化 / SELECT自定义 / TEMPLATE模板
 */

#include "container_codec.h"
#include <string.h>

/* 内部: 2的幂次查表 (替代位移) */
static const uint16_t pow2_table[16] = {
    1, 2, 4, 8, 16, 32, 64, 128,
    256, 512, 1024, 2048, 4096, 8192, 16384, 32768
};

/* ═══════════════════════════════════════════════════════════════
 * 内部: 生成框架名称
 * 格式: C{容器数}s{每容器字数}t{层级深度}-{模式}
 * ═══════════════════════════════════════════════════════════════ */

static void generate_name(container_codec_t *codec) {
    const char *mode_str = "FULL";
    if (codec->mode == FRAME_SELECT)   mode_str = "SELECT";
    if (codec->mode == FRAME_TEMPLATE) mode_str = "TEMPLATE";

    /* 手动构造字符串 (避免 sprintf) */
    char *p = codec->name;
    *p++ = 'C';

    /* 容器数 */
    if (codec->container_count >= 100) {
        *p++ = '0' + (codec->container_count / 100);
        *p++ = '0' + ((codec->container_count / 10) % 10);
    }
    if (codec->container_count >= 10) {
        *p++ = '0' + ((codec->container_count / 10) % 10) ||
               (codec->container_count >= 100 ? '0' : 0);
    }
    *p++ = '0' + (codec->container_count % 10);
    *p++ = 's';
    *p++ = '0' + (codec->chars_per_container / 10);
    *p++ = '0' + (codec->chars_per_container % 10);
    *p++ = 't';
    *p++ = '0' + codec->tier_depth;
    *p++ = '-';

    const char *m = mode_str;
    while (*m) *p++ = *m++;
    *p = '\0';
}

/* ═══════════════════════════════════════════════════════════════
 * ═══════════════════════════════════════════════════════════════ */

void codec_init(container_codec_t *codec, frame_mode_t mode,
                uint16_t containers, uint8_t chars_per, uint8_t tiers) {
    memset(codec, 0, sizeof(*codec));
    codec->magic           = CONTAINER_CODEC_MAGIC;
    codec->mode            = mode;
    codec->container_count = containers;
    codec->chars_per_container = chars_per > CHARS_PER_FRAME ? CHARS_PER_FRAME : chars_per;
    codec->tier_depth      = tiers;

    /* 基准坐标 */
    codec->base_coord.x = 0;
    codec->base_coord.y = 0;
    codec->base_coord.z = 0;

    generate_name(codec);
}

/* ═══════════════════════════════════════════════════════════════
 * 封装字群到字框
 *
 * 16个字被封装进一个字框 · 占用1bit
 * 字框是测量单位 · 不是里面的字
 * ═══════════════════════════════════════════════════════════════ */

int codec_seal_chars(container_codec_t *codec,
                     const uint32_t *chars, uint8_t count) {
    if (!codec || codec->magic != CONTAINER_CODEC_MAGIC) return -1;

    uint16_t remaining = count;
    uint8_t  offset    = 0;

    while (remaining > 0) {
        /* 找空闲字框或创建新框 */
        uint16_t fi = codec->frame_count;
        if (fi >= codec->container_count) {
            /* 超过容量 · 触发绝缘体 */
            return -2;
        }

        render_frame_t *f = &codec->frames[fi];
        f->frame_id   = fi;
        f->bit_size   = 1;  /* 始终=1 · 1框=1bit */
        f->color      = (uint8_t)(fi % 16);
        f->visible    = FQ_FULL_8;  /* 初始全可见 */
        f->hide_mask  = 0;

        /* 填充字 */
        uint8_t copy = (remaining > codec->chars_per_container) ?
                       codec->chars_per_container : (uint8_t)remaining;

        for (uint8_t i = 0; i < copy; i++) {
            f->chars[i] = chars[offset + i];
        }
        f->char_count = copy;

        /* 相位设置: 左=结束(240 if full) · 右=开始(15 if not full) */
        uint16_t cap = codec->chars_per_container;
        if (copy >= cap) {
            f->phase_left  = FQ_PHASE_FULL;   /* 已装满 · 左边看作结束 */
            f->phase_right = FQ_PHASE_EMPTY;  /* 右边开始 · 绝缘体连接 */
        } else {
            f->phase_left  = (uint8_t)((copy * FQ_PHASE_FULL) / cap);
            f->phase_right = FQ_PHASE_FULL - f->phase_left;
        }

        /* 设置绝缘体墙 */
        if (fi > 0) {
            insulator_t *w = &codec->walls[fi - 1];
            w->is_wall       = 1;  /* 这是一个墙符号 */
            w->left_ended    = 1;  /* 左边结束 */
            w->right_started = 1;  /* 右边开始 */
            w->zero_dim      = 1;  /* 零维 · 只是一个点 */
            codec->wall_count++;
        }

        codec->frame_count++;
        codec->total_chars += copy;
        codec->bits_used += 1;  /* 1框=1bit */

        remaining -= copy;
        offset    += copy;
    }

    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 克隆容器
 *
 * 当字数超过容量时 · 自动克隆
 * 超过254后 · 使用WALL_SYMBOL · 截止=开始
 * ═══════════════════════════════════════════════════════════════ */

int codec_clone_container(container_codec_t *codec, uint16_t frame_idx) {
    if (!codec || codec->magic != CONTAINER_CODEC_MAGIC) return -1;

    /* 创建新字框 (克隆) */
    uint16_t fi = codec->frame_count;
    if (fi >= MAX_FRAMES) return -1;

    render_frame_t *src = &codec->frames[frame_idx];
    render_frame_t *dst = &codec->frames[fi];

    /* 复制定义 (克隆) */
    dst->frame_id   = fi;
    dst->bit_size   = 1;
    dst->color      = src->color;
    dst->visible    = src->visible;
    dst->hide_mask  = src->hide_mask;
    dst->char_count = 0;  /* 新克隆的空框 */

    /* 继承相位 · 左边结束=右边开始 */
    dst->phase_left  = FQ_PHASE_EMPTY;
    dst->phase_right = FQ_PHASE_FULL;

    /* 如果超过254 · 使用WALL_SYMBOL */
    if (fi >= CLONE_MAX) {
        insulator_t *w = &codec->walls[fi - 1];
        w->is_wall       = 1;
        w->left_ended    = 1;
        w->right_started = 1;
        w->zero_dim      = 1;
    }

    codec->frame_count++;
    codec->bits_used += 1;
    return (int)fi;
}

/* ═══════════════════════════════════════════════════════════════
 * 设置可见性
 *
 * 可以编程让某些字临时消失/出现
 * 但不改变1bit封装的定义 · 定态永远有效
 * ═══════════════════════════════════════════════════════════════ */

void codec_set_visibility(container_codec_t *codec, uint16_t frame_idx,
                          uint16_t hide_mask) {
    if (!codec || codec->magic != CONTAINER_CODEC_MAGIC) return;
    if (frame_idx >= codec->frame_count) return;

    /* 设置隐藏掩码 · 但不改变bit_size */
    codec->frames[frame_idx].hide_mask = (uint16_t)(hide_mask & FQ_FULL_16);

    /* 可见性模板更新 */
    uint8_t vis = 0;
    for (uint8_t i = 0; i < codec->frames[frame_idx].char_count; i++) {
        if (!(hide_mask & pow2_table[i])) {
            vis |= pow2_table[i / 2];  /* 简化 · 每2字共享1位 */
        }
    }
    codec->frames[frame_idx].visible = vis;
}

/* ═══════════════════════════════════════════════════════════════
 * 渲染: 字框 → 圆筒坐标 + 颜色 + 相位
 *
 * 输出到三维空间: 位置 · 颜色 · 左右相位
 * ═══════════════════════════════════════════════════════════════ */

int codec_render_frame(container_codec_t *codec, uint16_t frame_idx,
                       cylinder_coord_t *coord,
                       uint8_t *color, uint8_t *phase_left, uint8_t *phase_right) {
    if (!codec || codec->magic != CONTAINER_CODEC_MAGIC) return -1;
    if (frame_idx >= codec->frame_count) return -1;

    render_frame_t *f = &codec->frames[frame_idx];

    /* 圆筒坐标: 按字框id线性排布 */
    coord->x = (uint8_t)(codec->base_coord.x + (frame_idx % 16));
    coord->y = (uint8_t)(codec->base_coord.y + (frame_idx / 16) % 16);
    coord->z = (uint8_t)(codec->base_coord.z + (frame_idx / 256));

    *color      = f->color;
    *phase_left  = f->phase_left;
    *phase_right = f->phase_right;

    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 编译整个框架
 *
 * 处理三种模式:
 *   FULL     = 所有容器都有效 · 编译为最大容器集合
 *   SELECT   = 只编译选择的容器 · 局部串联
 *   TEMPLATE = 编译预设的几个常用模板
 * ═══════════════════════════════════════════════════════════════ */

int codec_compile(container_codec_t *codec) {
    if (!codec || codec->magic != CONTAINER_CODEC_MAGIC) return -1;

    switch (codec->mode) {
    case FRAME_FULL:
        /* 全量化: 编译所有字框 */
        for (uint16_t i = 0; i < codec->frame_count; i++) {
            cylinder_coord_t coord;
            uint8_t cl, pl, pr;
            codec_render_frame(codec, i, &coord, &cl, &pl, &pr);
            /* 编译结果即为圆筒坐标 + 渲染定态 */
        }
        break;

    case FRAME_SELECT:
        /* 选择模式: 只编译可见的字框 */
        for (uint16_t i = 0; i < codec->frame_count; i++) {
            if (codec->frames[i].hide_mask != FQ_FULL_16) {
                cylinder_coord_t coord;
                uint8_t cl, pl, pr;
                codec_render_frame(codec, i, &coord, &cl, &pl, &pr);
            }
        }
        break;

    case FRAME_TEMPLATE:
        /* 模板模式: 使用预置的几个常用字框组合 */
        /* 模板0: 仅前8框 · 模板1: 16框 · 模板2: 32框 */
        {
            uint16_t tmpl_size = (codec->frame_count >= 32) ? 32 :
                                 (codec->frame_count >= 16) ? 16 : 8;
            for (uint16_t i = 0; i < tmpl_size && i < codec->frame_count; i++) {
                cylinder_coord_t coord;
                uint8_t cl, pl, pr;
                codec_render_frame(codec, i, &coord, &cl, &pl, &pr);
            }
        }
        break;
    }

    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 获取名称
 * ═══════════════════════════════════════════════════════════════ */

const char *codec_get_name(container_codec_t *codec) {
    if (!codec || codec->magic != CONTAINER_CODEC_MAGIC) return "INVALID";
    return codec->name;
}
