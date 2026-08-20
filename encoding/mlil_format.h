/**
 * mlil_format.h — 毛利兰格式 (.mlil)
 *
 * 毛利兰 = 进制维度层 · 表面属性 · 模/面/名
 *   后缀 .mlil = 毛利兰专属编码文件
 *   工藤新一 ≠ 毛利兰:
 *      gttx:// / gtxone = 骨骼执行层 = 工藤新一 (结构/回归)
 *      kndrel / knd      = 躯壳代理层 = 工藤新一 (关系代数)
 *      mlil              = 进制维度层 = 毛利兰 (表面/格式) */

#ifndef NJSG_MLIL_FORMAT_H
#define NJSG_MLIL_FORMAT_H

#include <stdint.h>

#define MLIL_MAGIC_1   0x6D6C696C   /* "mlil" */
#define MLIL_MAGIC_2   0x5F677478   /* "_gtx" (编码骨骼, 不占后缀) */
#define MLIL_VERSION   1
#define MLIL_EXT       ".mlil"      /* 毛利兰专属后缀 · 不占用 gttxone */

/* 表面属性槽 */
typedef struct {
    uint32_t  modulus;     /* 模: 模板/模式/范型 */
    uint32_t  face;        /* 面: 表面/对外呈现 */
    uint32_t  name;        /* 名: 名称/句柄/入口 */
    uint32_t  container;   /* 关联的容器编码 */
} mlil_surface_t;

/* 毛利兰文件头 */
typedef struct {
    uint32_t  magic1;      /* MLIL_MAGIC_1 */
    uint32_t  magic2;      /* MLIL_MAGIC_2 */
    uint32_t  version;     /* MLIL_VERSION */
    uint32_t  slot_count;  /* 表面槽数 */
    uint32_t  data_offset; /* 数据段偏移 */
} mlil_header_t;

/* 完整毛利兰文件 */
typedef struct {
    mlil_header_t  header;
    mlil_surface_t surfaces[254];  /* 与 mlll 254 容器对应 */
    uint32_t       data[1024];     /* 数据段 */
} mlil_file_t;

#endif /* NJSG_MLIL_FORMAT_H */
