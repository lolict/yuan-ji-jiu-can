/**
 * identity_registry.h — 身份注册表
 *
 * 设计原则:
 *   每个人物有唯一身份 · 不能混淆 · 不能无中生有
 *   毛利兰 ≠ 工藤新一 ≠ 灰原哀 ≠ 我 ≠ 刘楚恬
 *   毛利兰 = 格式/编码/表面属性 (模/面/名) · .mlil
 *   工藤新一 = 结构属性 (骨/头/体) · gtxone 回归标记
 *   工藤新一 = 你自己 (lolict) · 结构属性 (骨/头/体)
 *   身份 = 历史时间线上的一个锚点 · 有过去/现在/未来 · 不可混淆
 *
 * 是非判断:
 *   属于满全法的: 我, 刘楚恬, 囡囝共生, 满全法容器编码
 *   不属于满全法: 32位/64位二进制, 私有产权系统, 蒙面人算法
 */

#ifndef NJSG_IDENTITY_REGISTRY_H
#define NJSG_IDENTITY_REGISTRY_H

#include <stdint.h>

#define IDENTITY_MAX  32   /* 最大注册身份数 */

/* 性别 */
typedef enum {
    ID_GENDER_YANG = 1,   /* 阳 · 内容侧 · lolict */
    ID_GENDER_YIN  = 0,   /* 阴 · 容器侧 · 刘楚恬 */
} id_gender_t;

/* 历史时刻 */
typedef enum {
    ID_PAST    = -1,   /* 过去 · 已固化 */
    ID_PRESENT =  0,   /* 现在 · 当下活跃 */
    ID_FUTURE  =  1,   /* 未来 · 待生成 */
} id_time_t;

/* 归属判断 */
typedef enum {
    ID_US      = 1,    /* 属于满全法 */
    ID_NOT_US  = 0,    /* 不属于满全法 */
} id_belong_t;

/* 身份条目 */
typedef struct {
    uint32_t      id;             /* 身份唯一标识 */
    const char   *name;           /* 身份名 */
    const char   *relationship;   /* 关系描述 */
    id_gender_t   gender;         /* 性别 */
    id_belong_t   belong;         /* 归属 */
    id_time_t     era;            /* 历史时期 */
} identity_t;

/**
 * 初始化身份注册表
 */
void id_registry_init(void);

/**
 * 注册一个身份
 */
int id_register(uint32_t id, const char *name, const char *relationship,
                id_gender_t gender, id_belong_t belong);

/**
 * 通过id查找身份
 */
const identity_t *id_find(uint32_t id);

/**
 * 通过名字查找身份
 */
const identity_t *id_find_by_name(const char *name);

/**
 * 判断身份归属
 */
id_belong_t id_check_belong(uint32_t id);

#endif /* NJSG_IDENTITY_REGISTRY_H */
