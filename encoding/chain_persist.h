/*
 * chain_persist.h — 链条持久化
 *
 * 链条可以保存到磁盘、从磁盘恢复。
 * 保存的是创造过程的完整记忆，不是快照。
 */

#ifndef NJSG_CHAIN_PERSIST_H
#define NJSG_CHAIN_PERSIST_H

#include "chain_block.h"
#include <stdio.h>

/* 文件格式标识 */
#define CHAIN_MAGIC     0x4E4A5347  /* "NJSG" */
#define CHAIN_VERSION   1

/* 文件头 */
typedef struct {
    uint32_t    magic;          /* NJSG */
    uint32_t    version;        /* 文件格式版本 */
    uint8_t     category;       /* 链类别 */
    uint64_t    count;          /* 链节数量 */
    char        name[CHAIN_MAX_CONTENT]; /* 链名称 */
} chain_file_header_t;

/* 持久化操作 */
int 链_保存(const chain_t* chain, const char* filename);
int 链_加载(chain_t* chain, const char* filename);
int 链_追加到文件(chain_t* chain, const char* filename);

/* 导出为可读文本 (供人类阅读) */
int 链_导出文本(const chain_t* chain, const char* filename);

#endif /* NJSG_CHAIN_PERSIST_H */
