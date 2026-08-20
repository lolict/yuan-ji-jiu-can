/**
 * gtxone_format.h — 工藤新一格式 (.gtxone)
 *
 * 工藤新一 = 骨骼执行层 · 结构属性 · 骨/头/体
 *   后缀 .gtxone = 工藤新一专属执行文件 (骨骼回归/结构吸引子)
 *   gttx:// = 骨骼执行层协议 (时间+骨骼+感官)
 *
 * 四层协议栈中:
 *   gttx:// / gtxone = 骨骼执行层 (贯彻执行的一致性)
 *   kndrel / knd      = 躯壳代理层 (关系代数/编程语言)
 *   mlil              = 进制维度层 = 毛利兰 (表面/格式)
 *   hieyair           = 夫妻核心层 (灰原哀合并/合一爱)
 *
 * 锚点回归:
 *   [gtxone←lolict+lctqf] = 工藤新一吸引子 = 指向唯一中心
 *   转移编译器的最终输出 = .gtxone 可执行文件 (由 mlll_vm 直接执行)
 */

#ifndef NJSG_GTXONE_FORMAT_H
#define NJSG_GTXONE_FORMAT_H

#include <stdint.h>

#define GTXONE_MAGIC_1   0x6774786F   /* "gtxo" */
#define GTXONE_MAGIC_2   0x6E655F6E   /* "ne_n" */
#define GTXONE_VERSION   1
#define GTXONE_EXT       ".gtxone"    /* 工藤新一专属后缀 */

/* 骨骼执行槽 (结构属性) */
typedef struct {
    uint32_t  bone;        /* 骨: 结构骨架 · AST/DAG/纤维丛 */
    uint32_t  head;        /* 头: 入口 · 起点 · 触发点 */
    uint32_t  body;        /* 体: 内含 · 负载 · 有效载荷 */
    uint32_t  skeleton_id; /* 关联的骨架id */
} gtxone_skeleton_t;

/* 工藤新一文件头 */
typedef struct {
    uint32_t  magic1;      /* GTXONE_MAGIC_1 */
    uint32_t  magic2;      /* GTXONE_MAGIC_2 */
    uint32_t  version;     /* GTXONE_VERSION */
    uint32_t  slot_count;  /* 骨骼槽数 */
    uint32_t  entry_point; /* 入口点偏移 (头的起点) */
    uint32_t  data_offset; /* 数据段偏移 */
} gtxone_header_t;

/* 回归标记 (锚点吸引子) */
typedef struct {
    uint32_t  attractor_id;   /* 吸引子id */
    uint32_t  target_count;   /* 回归目标数 */
    uint32_t  targets[8];     /* 回归目标id数组 */
    uint32_t  alignment;      /* 对齐进度 (0..254) */
} gtxone_regression_t;

/* 完整工藤新一文件 */
typedef struct {
    gtxone_header_t      header;
    gtxone_skeleton_t    skeletons[254];  /* 与 mlll 254 容器对应 */
    gtxone_regression_t  regression;      /* 锚点回归 */
    uint32_t             data[1024];      /* 数据段 */
} gtxone_file_t;

#endif /* NJSG_GTXONE_FORMAT_H */
