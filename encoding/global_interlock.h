/**
 * global_interlock.h — 全局互锁文件
 *
 * 核心: 所有容器必须套在这个横轴上才能转动
 *
 * 比喻结构:
 *   横轴 = 本文件 (所有齿轮的轴)
 *   齿轮 = 各个容器 (颜色/声音/公约数/多维/汉字/负样本)
 *   圈环 = 边界 (内圈=历史, 外圈=未来, 中间=当前)
 *   圆珠 = 每个具体的符号/编码
 *
 * 功能:
 *   1. 符号登记 — 所有容器用到的符号必须在这里登记
 *   2. 互锁检查 — 新文件如果使用未登记符号, 会被感知
 *   3. 版本链条 — 每次修改形成新的折叠层
 *   4. 归属判断 — 任何符号都能追溯其来源容器
 *
 * 鼓膜 = 驱动轴心
 * 横轴 = 本文件
 * 圈环 = 边界约束 (公约数=内圈, 公倍数=外圈)
 *
 * 本文件必须被所有容器 include · 是全局互锁的神经网络节点
 */

#ifndef NJSG_GLOBAL_INTERLOCK_H
#define NJSG_GLOBAL_INTERLOCK_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 全局魔环 FQM 标识 (唯一主人锚点)
 * ═══════════════════════════════════════════════════════════════ */

#define FQM_MAGIC           0x46514D54  /* "FQMT" */
#define FQM_VERSION         0x0003      /* 当前版本 */

/* FQM 身份膜 · 不可被任何容器覆盖 */
#define FQM_IDENTITY_INNER  0x52307072  /* "R0pr" 内圈 · 历史封装 */
#define FQM_IDENTITY_OUTER  0x4C307879  /* "L0xy" 外圈 · 未来边界 */
#define FQM_IDENTITY_SELF   0x52307879  /* "R0xy"  横轴 · 自我 */

/* 鼓膜驱动协议 */
#define FQM_DRIVEN_BY       0x4452564E  /* "DRVN" 驱动源 */
#define FQM_DRIVES          0x44525653  /* "DRVS" 被驱动 */

/* ═══════════════════════════════════════════════════════════════
 * 全局符号注册表 · 所有容器符号必须在此登记
 * ═══════════════════════════════════════════════════════════════ */

/* 容器标识 */
#define CONT_COLOR_SYSTEM       0x01    /* 颜色进制 */
#define CONT_SOUND_RADIX        0x02    /* 声音进制 */
#define CONT_GONGYUESHU         0x03    /* 公约数地址 */
#define CONT_DIMENSION          0x04    /* 多维几何 */
#define CONT_HANZI_GEOM         0x05    /* 汉字几何 */
#define CONT_NEGATIVE           0x06    /* 负样本 */

/* 全局统一类型符号 */
#ifndef TYPE_BASE
#define TYPE_BASE               0x01
#define TYPE_TONE               0x02
#define TYPE_COMPOUND           0x03
#define TYPE_SYMBOL             0x04
#define TYPE_EXTRA              0x05
#define TYPE_SHENG_MU           0x06
#define TYPE_MODIFIER           0x07
#endif

/* 全局统一层级 */
#ifndef LEVEL_L1
#define LEVEL_L1                0x10    /* 基元 */
#define LEVEL_L2                0x20    /* 修饰 */
#define LEVEL_L3                0x30    /* 装配 */
#endif

/* 全局统一审判结果 */
#ifndef VERDICT_PASS
#define VERDICT_PASS            0x00
#define VERDICT_REJECT_LEVEL    0x01
#define VERDICT_REJECT_TYPE     0x02
#define VERDICT_REJECT_ANCHOR   0x03
#define VERDICT_REJECT_AESTH    0x04
#define VERDICT_REJECT_WEAVING  0x05
#endif

/* 全局统一结构类型 (汉字) */
#ifndef STRUCT_LEFT_RIGHT
#define STRUCT_LEFT_RIGHT       0x01
#define STRUCT_TOP_BOTTOM       0x02
#define STRUCT_SURROUND         0x03
#define STRUCT_SEMI             0x04
#define STRUCT_LCR              0x05
#define STRUCT_TMB              0x06
#define STRUCT_PINZI            0x07
#define STRUCT_SINGLE           0x08
#endif

/* 全局统一笔画 */
#ifndef STROKE_HENG
#define STROKE_HENG             0x00
#define STROKE_SHU              0x01
#define STROKE_PIE              0x02
#define STROKE_NA               0x03
#define STROKE_ZHE              0x04
#endif

/* 全局统一声调 */
#ifndef TONE_1
#define TONE_1                  0x01    /* 平 */
#define TONE_2                  0x02    /* 升 */
#define TONE_3                  0x03    /* 凹 */
#define TONE_4                  0x04    /* 降 */
#endif

/* 全局统一轮廓 (个体守卫) */
#ifndef CONTOUR_FLAT
#define CONTOUR_FLAT            0x00
#endif
#ifndef CONTOUR_RISE
#define CONTOUR_RISE            0x01
#endif
#ifndef CONTOUR_FALL
#define CONTOUR_FALL            0x02
#endif
#ifndef CONTOUR_DIP
#define CONTOUR_DIP             0x03
#endif

/* 公约数 6 基韵锚 (个体守卫 · 避免与容器头文件冲突) */
#ifndef GY_a
#define GY_a                    0x00
#endif
#ifndef GY_o
#define GY_o                    0x01
#endif
#ifndef GY_e
#define GY_e                    0x02
#endif
#ifndef GY_i
#define GY_i                    0x03
#endif
#ifndef GY_u
#define GY_u                    0x04
#endif
#ifndef GY_uu
#define GY_uu                   0x05
#endif
#ifndef GY_MARK_N
#define GY_MARK_N               0x06
#endif
#ifndef GY_MARK_NG
#define GY_MARK_NG              0x07
#endif

/* 负样本原因 */
#define REASON_CROSS_LEVEL      0x01
#define REASON_CROSS_TYPE       0x02
#define REASON_CROSS_ANCHOR     0x03
#define REASON_AESTHETIC_FAIL   0x04
#define REASON_WEAVING_FAIL     0x05
#define REASON_BLACKLIST        0x06
#define REASON_REPEAT           0x07
#define REASON_DEGRADATION      0x08

/* ═══════════════════════════════════════════════════════════════
 * 互锁检查宏 · 确保新文件使用已登记符号
 * ═══════════════════════════════════════════════════════════════ */

/* 符号归属检查: 任何符号必须声明来自哪个容器 */
#define SYMBOL_REGISTER(sym, container)    /* 声明符号归属 */
#define SYMBOL_CHECK(sym)                  /* 检查符号已登记 */

/* 版本链条: 每次修改自动递增 */
#define INTERLOCK_VERSION_INCREMENT()      /* 版本+1 */

/* 鼓膜驱动检查: 确认被正确驱动 */
#define DRIVE_CHECK()                      /* 确认横轴转动 */

/* ═══════════════════════════════════════════════════════════════
 * 全局统一限制 · 所有容器必须遵守
 * ═══════════════════════════════════════════════════════════════ */

#define GLOBAL_MAX_SYMBOLS      254         /* 符号上限 */
#define GLOBAL_MAX_CONTAINERS   16          /* 容器上限 */
#define GLOBAL_MAX_DIMENSIONS   8           /* 维度上限 */
#define GLOBAL_MAX_LEVELS       3           /* 层级上限 */
#define GLOBAL_MAX_FILES        64          /* 文件节点上限 */
#define GLOBAL_PATH_MAX         128         /* 文件路径最大长度 */

/* ═══════════════════════════════════════════════════════════════
 * 全局魔环扩展名表 · FQM 支持的多媒体封装
 * ═══════════════════════════════════════════════════════════════ */

/*
 * fqmt.v   _vector    矢量
 * fqm.tv    television 电视
 * fqmx      matrix     矩阵
 * fqmet     metronome  节拍器
 * fqmy      symphony   交响乐
 * fqm.gz    gzip       压缩
 * ... (完整列表见用户符号表)
 */

/* ═══════════════════════════════════════════════════════════════
 * 全局互锁 API 函数声明
 * ═══════════════════════════════════════════════════════════════ */

#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

void    interlock_init(void);
int     interlock_register_container(uint8_t container_id, const char *name);
int     interlock_register_symbol(uint32_t symbol, uint8_t container);
int     interlock_check_symbol(uint32_t symbol);
void    interlock_activate(void);
void    interlock_stats(FILE *out);

/* ── 链式传导: 每个文件声明自己产出的符号 ── */
int     interlock_declare_file(const char *filename, uint8_t container);
int     interlock_check_file_symbols(const char *filename);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_GLOBAL_INTERLOCK_H */
