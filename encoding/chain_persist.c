/*
 * chain_persist.c — 链条持久化实现
 *
 * 保存的是完整链条，不是快照。
 * 从文件恢复后，可以继续追加新链节。
 */

#include "chain_persist.h"
#include <string.h>
#include <stdlib.h>

/* ═══════════════════════════════════════════════════════════════
 * 保存链条到文件
 * ═══════════════════════════════════════════════════════════════ */

int 链_保存(const chain_t* chain, const char* filename) {
    if (!chain || !filename) return -1;
    
    FILE* fp = fopen(filename, "wb");
    if (!fp) return -1;
    
    /* 写文件头 */
    chain_file_header_t header;
    header.magic = CHAIN_MAGIC;
    header.version = CHAIN_VERSION;
    header.category = chain->category;
    header.count = chain->count;
    strncpy(header.name, chain->name, CHAIN_MAX_CONTENT - 1);
    header.name[CHAIN_MAX_CONTENT - 1] = '\0';
    
    fwrite(&header, sizeof(header), 1, fp);
    
    /* 写所有链节 */
    if (chain->count > 0) {
        fwrite(chain->nodes, sizeof(chain_node_t), chain->count, fp);
    }
    
    fclose(fp);
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 从文件加载链条
 * ═══════════════════════════════════════════════════════════════ */

int 链_加载(chain_t* chain, const char* filename) {
    if (!chain || !filename) return -1;
    
    FILE* fp = fopen(filename, "rb");
    if (!fp) return -1;
    
    /* 读文件头 */
    chain_file_header_t header;
    if (fread(&header, sizeof(header), 1, fp) != 1) {
        fclose(fp);
        return -1;
    }
    
    /* 验证 */
    if (header.magic != CHAIN_MAGIC) {
        fclose(fp);
        return -1;
    }
    
    /* 初始化链 */
    memset(chain, 0, sizeof(chain_t));
    chain->category = header.category;
    chain->count = header.count;
    chain->next_version = header.count + 1;
    strncpy(chain->name, header.name, CHAIN_MAX_CONTENT - 1);
    chain->name[CHAIN_MAX_CONTENT - 1] = '\0';
    
    /* 读链节 */
    if (header.count > 0) {
        if (header.count > CHAIN_MAX_NODES) {
            fclose(fp);
            return -1;
        }
        fread(chain->nodes, sizeof(chain_node_t), header.count, fp);
    }
    
    fclose(fp);
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 追加新链节到文件 (不重写整个文件)
 * ═══════════════════════════════════════════════════════════════ */

int 链_追加到文件(chain_t* chain, const char* filename) {
    if (!chain || !filename) return -1;
    
    FILE* fp = fopen(filename, "r+b");
    if (!fp) {
        /* 文件不存在，创建新文件 */
        return 链_保存(chain, filename);
    }
    
    /* 读旧头 */
    chain_file_header_t header;
    fseek(fp, 0, SEEK_SET);
    if (fread(&header, sizeof(header), 1, fp) != 1) {
        fclose(fp);
        return -1;
    }
    
    /* 更新计数 */
    header.count = chain->count;
    
    /* 写回头部 */
    fseek(fp, 0, SEEK_SET);
    fwrite(&header, sizeof(header), 1, fp);
    
    /* 追加新链节 */
    if (chain->count > 0) {
        fseek(fp, 0, SEEK_END);
        /* 只写新增的部分 */
        uint64_t old_count = header.count - (chain->next_version - header.count - 1);
        /* 简化: 直接重写全部 */
        fseek(fp, sizeof(header), SEEK_SET);
        fwrite(chain->nodes, sizeof(chain_node_t), chain->count, fp);
    }
    
    fclose(fp);
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 导出为人类可读的文本
 * ═══════════════════════════════════════════════════════════════ */

int 链_导出文本(const chain_t* chain, const char* filename) {
    if (!chain || !filename) return -1;
    
    FILE* fp = fopen(filename, "w");
    if (!fp) return -1;
    
    fprintf(fp, "═══════════════════════════════════════════════════\n");
    fprintf(fp, "  链: %s\n", chain->name);
    fprintf(fp, "  类别: %d\n", chain->category);
    fprintf(fp, "  链节数: %lu\n", (unsigned long)chain->count);
    fprintf(fp, "  最新版本: %lu\n", (unsigned long)(chain->next_version - 1));
    fprintf(fp, "═══════════════════════════════════════════════════\n\n");
    
    const char* level_names[] = {"元", "阶", "层", "代", "世", "界", "域", "场"};
    
    for (uint64_t i = 0; i < chain->count; i++) {
        const chain_node_t* node = &chain->nodes[i];
        
        fprintf(fp, "[版本 %lu] %s\n", (unsigned long)node->version, node->content);
        fprintf(fp, "  上一版本: %lu\n", (unsigned long)node->prev_version);
        fprintf(fp, "  层级: %s\n", level_names[node->level % 8]);
        fprintf(fp, "  创造者: %s\n", node->creator);
        fprintf(fp, "  时间戳: %lu\n", (unsigned long)node->timestamp);
        if (node->observation[0]) {
            fprintf(fp, "  观察记录: %s\n", node->observation);
        }
        fprintf(fp, "\n");
    }
    
    fclose(fp);
    return 0;
}
