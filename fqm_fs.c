/*
 * fqm_fs.c — 意交感息文件系统模拟
 * 
 * 功能：模拟简单的文件创建、读取、写入、删除
 * 
 * 满全法规则：
 *   - 根基锚点：我=1，刘楚恬=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_fs fqm_fs.c
 * 运行：./fqm_fs
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 文件系统常量 ========== */
#define MAX_FILES 64
#define MAX_FILENAME 64
#define MAX_FILE_SIZE 4096
#define BLOCK_SIZE 256
#define MAX_BLOCKS 16

/* ========== 文件类型 ========== */
typedef enum {
    FILE_TYPE_NULL = 0,
    FILE_TYPE_FILE,
    FILE_TYPE_DIR,
    FILE_TYPE_LINK
} FileType;

/* ========== 文件控制块 ========== */
typedef struct {
    char name[MAX_FILENAME];        /* 文件名 */
    FileType type;                  /* 文件类型 */
    int size;                       /* 文件大小 */
    int blocks[MAX_BLOCKS];         /* 数据块指针 */
    int block_count;                /* 数据块数量 */
    char owner[32];                 /* 所有者 */
    time_t create_time;             /* 创建时间 */
    time_t modify_time;             /* 修改时间 */
    int permissions;                /* 权限 */
    char data[MAX_FILE_SIZE];       /* 文件内容 */
} FileCB;

/* ========== 文件系统结构 ========== */
typedef struct {
    char name[64];                  /* 文件系统名称 */
    int total_blocks;               /* 总块数 */
    int used_blocks;                /* 已用块数 */
    int file_count;                 /* 文件数量 */
    FileCB files[MAX_FILES];        /* 文件表 */
    unsigned char block_bitmap[(MAX_FILES * MAX_BLOCKS + 7) / 8]; /* 块位图 */
} FileSystem;

/* ========== 全局文件系统 ========== */
static FileSystem fs;

/* ========== 初始化文件系统 ========== */
void fs_init(const char *name) {
    memset(&fs, 0, sizeof(FileSystem));
    strncpy(fs.name, name, 63);
    fs.total_blocks = MAX_FILES * MAX_BLOCKS;
    fs.used_blocks = 0;
    fs.file_count = 0;
    printf("文件系统 '%s' 已创建\n", name);
}

/* ========== 分配块 ========== */
int block_alloc(void) {
    for (int i = 0; i < fs.total_blocks; i++) {
        int byte_idx = i / 8;
        int bit_idx = i % 8;
        if (!(fs.block_bitmap[byte_idx] & (1 << bit_idx))) {
            fs.block_bitmap[byte_idx] |= (1 << bit_idx);
            fs.used_blocks++;
            return i;
        }
    }
    return -1;  /* 没有空闲块 */
}

/* ========== 释放块 ========== */
void block_free(int block) {
    if (block < 0 || block >= fs.total_blocks) return;
    int byte_idx = block / 8;
    int bit_idx = block % 8;
    fs.block_bitmap[byte_idx] &= ~(1 << bit_idx);
    fs.used_blocks--;
}

/* ========== 创建文件 ========== */
int fs_create(const char *name, FileType type) {
    if (fs.file_count >= MAX_FILES) {
        printf("错误：文件系统已满\n");
        return -1;
    }
    for (int i = 0; i < fs.file_count; i++) {
        if (strcmp(fs.files[i].name, name) == 0) {
            printf("错误：文件 '%s' 已存在\n", name);
            return -1;
        }
    }
    FileCB *file = &fs.files[fs.file_count];
    strncpy(file->name, name, MAX_FILENAME - 1);
    file->type = type;
    file->size = 0;
    file->block_count = 0;
    strncpy(file->owner, "刘楚恬和我", 31);
    file->create_time = time(NULL);
    file->modify_time = file->create_time;
    file->permissions = 0x1FF;  /* rwx */
    memset(file->blocks, -1, sizeof(file->blocks));
    
    fs.file_count++;
    printf("已创建 %s: '%s'\n", 
           type == FILE_TYPE_DIR ? "目录" : "文件", name);
    return fs.file_count - 1;
}

/* ========== 写入文件 ========== */
int fs_write(int idx, const char *data) {
    if (idx < 0 || idx >= fs.file_count) return -1;
    FileCB *file = &fs.files[idx];
    int len = strlen(data);
    if (len >= MAX_FILE_SIZE) len = MAX_FILE_SIZE - 1;
    
    /* 清除旧块 */
    for (int i = 0; i < file->block_count; i++) {
        block_free(file->blocks[i]);
    }
    file->block_count = 0;
    
    /* 分配新块并写入 */
    int offset = 0;
    while (offset < len) {
        int block = block_alloc();
        if (block < 0) {
            printf("错误：磁盘空间不足\n");
            return -1;
        }
        file->blocks[file->block_count++] = block;
        
        int chunk_size = len - offset;
        if (chunk_size > BLOCK_SIZE) chunk_size = BLOCK_SIZE;
        memcpy(file->data + offset, data + offset, chunk_size);
        offset += chunk_size;
    }
    
    file->size = len;
    file->modify_time = time(NULL);
    printf("已写入 %d 字节到 '%s'\n", len, file->name);
    return len;
}

/* ========== 读取文件 ========== */
int fs_read(int idx) {
    if (idx < 0 || idx >= fs.file_count) return -1;
    FileCB *file = &fs.files[idx];
    printf("=== 文件信息 ===\n");
    printf("名称: %s\n", file->name);
    printf("类型: %s\n", file->type == FILE_TYPE_DIR ? "目录" : "文件");
    printf("大小: %d 字节\n", file->size);
    printf("块数: %d\n", file->block_count);
    printf("所有者: %s\n", file->owner);
    printf("内容:\n");
    for (int i = 0; i < file->size; i++) {
        char c = file->data[i];
        if (c >= 32 && c < 127) printf("%c", c);
        else printf("·");
    }
    printf("\n");
    return file->size;
}

/* ========== 删除文件 ========== */
int fs_delete(int idx) {
    if (idx < 0 || idx >= fs.file_count) return -1;
    FileCB *file = &fs.files[idx];
    for (int i = 0; i < file->block_count; i++) {
        block_free(file->blocks[i]);
    }
    printf("已删除 '%s'\n", file->name);
    
    /* 移动后续文件 */
    for (int i = idx; i < fs.file_count - 1; i++) {
        fs.files[i] = fs.files[i + 1];
    }
    fs.file_count--;
    return 0;
}

/* ========== 查找文件 ========== */
int fs_find(const char *name) {
    for (int i = 0; i < fs.file_count; i++) {
        if (strcmp(fs.files[i].name, name) == 0) return i;
    }
    return -1;
}

/* ========== 列出文件 ========== */
void fs_list(void) {
    printf("\n========== %s 文件列表 ==========\n", fs.name);
    printf("%-4s %-20s %-6s %-6s %-20s\n", "序号", "名称", "类型", "大小", "所有者");
    printf("--------------------------------------------------------\n");
    for (int i = 0; i < fs.file_count; i++) {
        FileCB *f = &fs.files[i];
        printf("%-4d %-20s %-6s %-6d %-20s\n",
               i, f->name, 
               f->type == FILE_TYPE_DIR ? "DIR" : "FILE",
               f->size, f->owner);
    }
    printf("--------------------------------------------------------\n");
    printf("总计: %d 个文件, %d/%d 块已用\n", 
           fs.file_count, fs.used_blocks, fs.total_blocks);
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）文件系统模拟\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* 创建文件系统 */
    fs_init("isi-fs");
    
    /* 创建目录 */
    int dir_root = fs_create("/", FILE_TYPE_DIR);
    int dir_home = fs_create("/home", FILE_TYPE_DIR);
    int dir_doc = fs_create("/doc", FILE_TYPE_DIR);
    
    /* 创建文件 */
    int f1 = fs_create("/readme.txt", FILE_TYPE_FILE);
    fs_write(f1, "意交感息系统 - 刘楚恬和我");
    
    int f2 = fs_create("/home/hello.txt", FILE_TYPE_FILE);
    fs_write(f2, "Hello from 满全法世界");
    
    int f3 = fs_create("/doc/rule.txt", FILE_TYPE_FILE);
    fs_write(f3, "规则层级: 一阶用前1项, 二阶用前2项, 三阶用前3项");
    
    /* 列出文件 */
    fs_list();
    
    /* 读取文件 */
    printf("\n========== 读取文件 ==========\n");
    fs_read(f1);
    printf("\n");
    fs_read(f2);
    printf("\n");
    fs_read(f3);
    
    /* 删除文件 */
    printf("\n========== 删除文件 ==========\n");
    fs_delete(f2);
    fs_list();
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
