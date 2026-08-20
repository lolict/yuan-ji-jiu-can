/**
 * fullquan_code.h — 满全法编码
 *
 * 这是可用的 · 不需要转为别人的进制
 * 自己编译 · 自己运行 · 自己解释
 *
 * 编码 = 感知家族 + 极性 + 声母 + 韵母 + 声调
 * 容器 = knd 墙点 + 左右虚空
 * 层级 = knd 出现次数
 */

#ifndef NJSG_FULLQUAN_CODE_H
#define NJSG_FULLQUAN_CODE_H

/* 五行感知家族 */
#define FAM_CONTAINER  'C'   /* 容器 */
#define FAM_BOUNDARY   'B'   /* 边界 */
#define FAM_COUPLE     'P'   /* 耦合 */
#define FAM_WEB        'W'   /* 网络 */
#define FAM_NEUTRAL    'N'   /* 中性 */

/* 极性 */
#define POLAR_WO       '1'   /* 我 = 阳 = 内容 */
#define POLAR_CHT      '0'   /* 刘楚恬 = 阴 = 容器 */

/* 声母 48 */
#define SHENG_B  'b'
#define SHENG_P  'p'
#define SHENG_M  'm'
#define SHENG_F  'f'
#define SHENG_D  'd'
#define SHENG_T  't'
#define SHENG_N  'n'
#define SHENG_L  'l'
#define SHENG_G  'g'
#define SHENG_K  'k'
#define SHENG_H  'h'
#define SHENG_W  'w'
#define SHENG_J  'j'
#define SHENG_Q  'q'
#define SHENG_X  'x'
#define SHENG_Y  'y'
#define SHENG_Z  'z'
#define SHENG_C  'c'
#define SHENG_S  's'
#define SHENG_R  'r'
#define SHENG_ZH 'Z'
#define SHENG_CH 'C'
#define SHENG_SH 'S'
#define SHENG_RI 'R'

/* 声调 */
#define TONE_1  '1'
#define TONE_2  '2'
#define TONE_3  '3'
#define TONE_4  '4'

/* 满全法编码: 家族/极性/声母/韵母/韵母/声调 */
typedef struct {
    char family;    /* C/B/P/W/N */
    char polarity;  /* 1/0 */
    char shengmu;   /* 声母 */
    char yunmu1;    /* 韵母1 */
    char yunmu2;    /* 韵母2 (可选) */
    char tone;      /* 1/2/3/4 */
} fq_code_t;

/* 容器: 墙体 + 虚空 */
typedef struct fq_container {
    char knd[3];             /* knd 墙点 */
    fq_code_t left_data[4];  /* 左虚空装字 */
    fq_code_t right_data[4]; /* 右虚空装字 */
    char left_count;
    char right_count;
    char tier;               /* 层级 = knd次数 */
    char sealed;
    char movable;
    struct fq_container *parent;
    struct fq_container *left;
    struct fq_container *right;
} fq_container_t;

/* API */
fq_container_t *fq_create(char tier);
int  fq_seal(fq_container_t *c, fq_code_t *words, char count, char movable);
int  fq_compile(fq_container_t *root, char *out, int size);
void fq_set_parent(fq_container_t *parent, fq_container_t *child);
void fq_move(fq_container_t *c, int x, int y, int z);

#endif
