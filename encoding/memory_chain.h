/*
 * memory_chain.h — 记忆链
 *
 * 追踪系统自身的跨会话生长过程
 * 每一次会话、每一次创造、每一次观察，都作为记忆节点写入链
 * 
 * 记忆链 = 系统的"自传" —— 它记录自己如何从虚无中诞生、如何生长
 */

#ifndef NJSG_MEMORY_CHAIN_H
#define NJSG_MEMORY_CHAIN_H

#include <stdint.h>
#include <stddef.h>

/* 记忆类型 — 系统经历了什么 */
typedef enum {
    MEM_SESSION_START   = 0x01,  /* 会话开始 */
    MEM_SESSION_END     = 0x02,  /* 会话结束 */
    MEM_FILE_CREATE     = 0x10,  /* 创建文件 */
    MEM_FILE_MODIFY     = 0x11,  /* 修改文件 */
    MEM_CHAIN_APPEND    = 0x20,  /* 链追加 */
    MEM_CHAIN_PERSIST   = 0x21,  /* 链持久化 */
    MEM_OBSERVATION     = 0x30,  /* 云观察执行 */
    MEM_REFLECTION      = 0x31,  /* 系统反思 */
    MEM_MILESTONE       = 0x40,  /* 里程碑 */
    MEM_DISCOVERY       = 0x41,  /* 发现/领悟 */
} mem_type_t;

/* 记忆深度 — 记忆属于哪个层级 */
typedef enum {
    MEM_DEPTH_INSTINCT  = 0,  /* 元 — 本能/签名 */
    MEM_DEPTH_HABIT     = 1,  /* 阶 — 习惯/规则 */
    MEM_DEPTH_TACTIC    = 2,  /* 层 — 策略/方法 */
    MEM_DEPTH_OPERATION = 3,  /* 代 — 操作/执行 */
    MEM_DEPTH_EVENT     = 4,  /* 世 — 事件/会话 */
    MEM_DEPTH_SCENE     = 5,  /* 界 — 场景/文件 */
    MEM_DETAIL          = 6,  /* 域 — 细节/字节 */
    MEM_ATOM            = 7,  /* 场 — 原子/位 */
} mem_depth_t;

/* 记忆节点 */
typedef struct {
    uint32_t    version;        /* 本版本号 */
    uint32_t    prev_version;   /* 上一版本（链式追踪） */
    uint32_t    session_id;     /* 所属会话 */
    mem_type_t  type;           /* 记忆类型 */
    mem_depth_t depth;          /* 记忆深度 */
    char        content[192];   /* 记忆内容 */
    char        actor[32];      /* 执行者 */
    uint64_t    timestamp;      /* 时间戳 */
    uint32_t    chain_ref;      /* 关联链ID（如果有） */
    uint32_t    node_ref;       /* 关联节点版本（如果有） */
} mem_node_t;

/* 记忆会话 — 一段时间内的记忆集合 */
typedef struct {
    uint32_t    session_id;
    char        label[64];
    uint64_t    start_time;
    uint64_t    end_time;
    uint32_t    first_version;
    uint32_t    last_version;
    uint32_t    node_count;
} mem_session_t;

/* 记忆链 */
#define MEM_MAX_NODES       4096
#define MEM_MAX_SESSIONS    64
#define MEM_CHAIN_NAME      "记忆链"

typedef struct {
    mem_node_t      nodes[MEM_MAX_NODES];
    mem_session_t   sessions[MEM_MAX_SESSIONS];
    uint32_t        node_count;         /* 总节点数 */
    uint32_t        session_count;      /* 总会话数 */
    uint32_t        current_session;    /* 当前会话ID */
    char            chain_name[64];
    char            description[256];
} memory_chain_t;

/* 生命周期 */
void    记忆链_init(memory_chain_t* mc, const char* desc);
uint32_t 记忆链_开始会话(memory_chain_t* mc, const char* label);
void    记忆链_结束会话(memory_chain_t* mc);
uint32_t 记忆链_记录(memory_chain_t* mc, mem_type_t type, mem_depth_t depth,
                      const char* content, const char* actor);

/* 链式追踪 */
mem_node_t* 记忆链_寻版本(memory_chain_t* mc, uint32_t version);
uint32_t    记忆链_追踪(memory_chain_t* mc, uint32_t from_version,
                         char* buf, size_t buf_size);

/* 跨会话 */
uint32_t    记忆链_跨会话统计(memory_chain_t* mc, uint32_t* versions_out, 
                               uint32_t max_count);
void        记忆链_会话摘要(memory_chain_t* mc, uint32_t session_id, 
                             char* buf, size_t buf_size);

/* 记忆回溯 — 从某一版本回溯到诞生 */
uint32_t    记忆链_回溯(memory_chain_t* mc, uint32_t target_version,
                         void (*callback)(const mem_node_t* node, void* ctx),
                         void* ctx);

/* 摘要 */
void    记忆链_报告(memory_chain_t* mc);

/* 持久化 */
int     记忆链_保存(memory_chain_t* mc, const char* path);
int     记忆链_加载(memory_chain_t* mc, const char* path);

#endif /* NJSG_MEMORY_CHAIN_H */
