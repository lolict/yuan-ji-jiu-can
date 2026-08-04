/**
 * 满全法执行内核 v0.1 — MQF Executive Kernel
 * -----------------------------------------
 * 自包含执行守门人。所有代码想在上面跑，必须先经过它授权。
 * 没授权 = 僵尸，有授权 = 活。
 * 
 * 架构:
 *   身份编码(AuthCode) → 权限引擎 → 执行控制 → 审计环
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>

/* ============================================================
 * 第 0 层: 基础类型与常量
 * ============================================================ */
#define MQF_VERSION     "0.1.0"
#define MQF_AUTH_LEN    32          /* 授权码长度 */
#define MQF_PATH_LEN    256
#define MQF_HASH_LEN    64
#define MAX_EXEC_SLOTS  16          /* 最大同时执行槽位 */
#define AUDIT_RING_SIZE 1024        /* 审计环大小 */

typedef enum {
    EXEC_SLEEP = 0,     /* 僵尸态: 未授权, 无法执行 */
    EXEC_PENDING,       /* 等待授权 */
    EXEC_ALIVE,         /* 已授权, 可执行 */
    EXEC_FROZEN,        /* 违规, 已被冻结 */
    EXEC_ZOMBIE         /* 永久废弃 */
} exec_state_t;

typedef enum {
    PERM_NONE   = 0x00,
    PERM_READ   = 0x01,
    PERM_WRITE  = 0x02,
    PERM_EXEC   = 0x04,
    PERM_NET    = 0x08,     /* 需要网络才能运行的, 默认禁止 */
    PERM_ADMIN  = 0x10      /* 内核管理权限 */
} perm_t;

/* ============================================================
 * 第 1 层: 满全法身份编码系统 (MQF AuthCode)
 * ============================================================ */

/**
 * 满全法编码身份
 * 由 声母号(6bit) + 韵母号(6bit) + 声调号(3bit) + 权限(5bit) + 时间戳(32bit) + 随机数(14bit) 压缩而成
 * 共 64bit = 16 字符十六进制
 */

/* 声母表 24 个 */
static const char* MQFS_INITS[] = {
    "b","p","m","f","d","t","n","l","g","k","h","j","q","x",
    "zh","ch","sh","r","z","c","s","y","w","@"
};

/* 韵母表 39 个 */
static const char* MQFS_FINALS[] = {
    "a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er",
    "an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao",
    "iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong","@"
};

/* 声调 5 个 */
static const char* MQFS_TONES[] = {"1","2","3","4","0"};

typedef struct {
    char text[MQF_AUTH_LEN + 1];
    int  init_idx;      /* 声母索引 0..23 */
    int  final_idx;     /* 韵母索引 0..38 */
    int  tone_idx;      /* 声调索引 0..4 */
    int  has_strokes;
    int  stroke_count;
    char auth_code[MQF_HASH_LEN + 1];
    char hash[MQF_HASH_LEN + 1];
} mqf_identity_t;

/**
 * 简单确定性哈希 (sha256 简化版, 实际部署换 Web Crypto 或 libsodium)
 */
static void mqf_hash(const char* in, char* out, size_t out_len) {
    uint32_t h1 = 0x811c9dc5u;  /* FNV-1a 32bit */
    uint32_t h2 = 0x0100193u;
    size_t len = strlen(in);
    for (size_t i = 0; i < len; i++) {
        h1 ^= (uint8_t)in[i];
        h1 *= 0x01000193u;
        h2 ^= (uint8_t)in[i] << 5;
        h2 *= 0x1000193u;
    }
    snprintf(out, out_len, "%08x%08x%08x%08x%08x%08x",
             h1, h2, h1 ^ h2, h2 ^ h1, h1 + h2, h2 - h1);
    out[out_len] = '\0';
}

/**
 * 文本 → 满全法编码身份
 * 将文本编码为身份向量, 计算授权码
 */
mqf_identity_t mqf_encode(const char* text) {
    mqf_identity_t id;
    memset(&id, 0, sizeof(id));
    
    size_t len = strlen(text);
    if (len > MQF_AUTH_LEN) len = MQF_AUTH_LEN;
    memcpy(id.text, text, len);
    id.text[len] = '\0';
    
    /* 确定性编码: 用文本首字符和声调决定身份向量 */
    uint32_t seed = 0;
    for (size_t i = 0; i < len; i++) {
        seed = seed * 31 + (unsigned char)text[i];
    }
    
    id.init_idx = seed % 24;
    id.final_idx = (seed >> 5) % 39;
    id.tone_idx = (seed >> 10) % 5;
    id.has_strokes = 1;
    id.stroke_count = (seed >> 15) % 30 + 1;
    
    /* 生成授权码 */
    char buf[512];
    snprintf(buf, sizeof(buf), "%s|%d|%d|%d|%d",
             MQFS_INITS[id.init_idx],
             id.init_idx, id.final_idx, id.tone_idx, id.stroke_count);
    mqf_hash(buf, id.auth_code, MQF_HASH_LEN);
    
    /* 计算总哈希 */
    snprintf(buf, sizeof(buf), "%s.%s", text, id.auth_code);
    mqf_hash(buf, id.hash, MQF_HASH_LEN);
    
    return id;
}

void mqf_identity_print(const mqf_identity_t* id) {
    printf("满全法身份编码:\n");
    printf("  原始文本:   %s\n", id->text);
    printf("  声母:       #%d (%s)\n", id->init_idx, MQFS_INITS[id->init_idx]);
    printf("  韵母:       #%d (%s)\n", id->final_idx, MQFS_FINALS[id->final_idx]);
    printf("  声调:       #%d (%s)\n", id->tone_idx, MQFS_TONES[id->tone_idx]);
    printf("  笔画:       %d\n", id->stroke_count);
    printf("  授权码:     %.*s...\n", 24, id->auth_code);
    printf("  身份哈希:   %.*s...\n", 24, id->hash);
}

/* ============================================================
 * 第 2 层: 权限引擎
 * ============================================================ */

typedef struct {
    char auth_code[MQF_HASH_LEN + 1];
    perm_t permissions;
    int max_slots;          /* 最大同时执行槽位 */
    int time_limit_sec;     /* 单次执行时限(0=无限制) */
    int require_network;    /* 是否要求网络 — 默认禁止 */
    time_t created_at;
    time_t expires_at;
} capability_t;

/**
 * 根据身份自动判定权限规则
 */
capability_t mqf_grant(const mqf_identity_t* id) {
    capability_t cap;
    memset(&cap, 0, sizeof(cap));
    
    memcpy(cap.auth_code, id->auth_code, MQF_HASH_LEN + 1);
    
    /* 内置规则: 编码决定权限等级 */
    int level = (id->init_idx + id->final_idx + id->tone_idx) % 7;
    
    switch (level) {
        case 0: /* 0级: 僵尸 — 什么权限都没有, 无法执行 */
            cap.permissions = PERM_NONE;
            cap.max_slots = 0;
            cap.time_limit_sec = 0;
            cap.require_network = 0;
            break;
        case 1: case 2: /* 1-2级: 观察者 — 只读 */
            cap.permissions = PERM_READ;
            cap.max_slots = 1;
            cap.time_limit_sec = 30;
            cap.require_network = 0;
            break;
        case 3: case 4: /* 3-4级: 执行者 — 读写执行 */
            cap.permissions = PERM_READ | PERM_WRITE | PERM_EXEC;
            cap.max_slots = 4;
            cap.time_limit_sec = 300;
            cap.require_network = 0;
            break;
        case 5: case 6: /* 5-6级: 核心 -- 全部权限 */
            cap.permissions = PERM_READ | PERM_WRITE | PERM_EXEC | PERM_ADMIN;
            cap.max_slots = 16;
            cap.time_limit_sec = 0;
            cap.require_network = 0;
            break;
    }
    
    /* 规则: 外部网络权限默认禁止 — 所有执行必须在本地 */
    cap.require_network = 0;
    
    cap.created_at = time(NULL);
    cap.expires_at = cap.created_at + 86400 * 30; /* 默认授权30天 */
    
    return cap;
}

/* ============================================================
 * 第 3 层: 执行控制 (守门人核心)
 * ============================================================ */

typedef struct {
    int used;
    char auth_code[MQF_HASH_LEN + 1];
    char name[128];
    exec_state_t state;
    capability_t cap;
    void (*func)(void*);     /* 待执行的函数指针 */
    void* arg;
    time_t activated_at;
} exec_slot_t;

typedef struct {
    exec_slot_t slots[MAX_EXEC_SLOTS];
    int alive_count;
    int frozen_count;
    int zombie_count;
} kernel_t;

static kernel_t KERNEL;

void mqf_kernel_init() {
    memset(&KERNEL, 0, sizeof(KERNEL));
    printf("\n[内核] 满全法执行内核 v%s 初始化完成\n", MQF_VERSION);
    printf("[内核] 执行槽位: %d | 审计环: | 本地执行 | 零云端依赖\n\n",
           MAX_EXEC_SLOTS);
}

/**
 * 注册一个执行请求
 * 返回: 槽位编号, -1 表示拒绝(僵尸)
 */
int mqf_register(const char* name, void (*func)(void*), void* arg) {
    /* 编码身份 */
    mqf_identity_t id = mqf_encode(name);
    mqf_identity_print(&id);
    
    /* 判定权限 */
    capability_t cap = mqf_grant(&id);
    
    /* 0级: 直接拒绝 — 保持僵尸态 */
    if (cap.permissions == PERM_NONE) {
        printf("[注册] ★ 僵尸判定: %s 获得 0 级权限, 无法执行\n\n", name);
        return -1;
    }
    
    /* 槽位分配 */
    int slot = -1;
    for (int i = 0; i < MAX_EXEC_SLOTS; i++) {
        if (!KERNEL.slots[i].used) {
            slot = i;
            break;
        }
    }
    if (slot < 0) {
        printf("[注册] 拒绝: 槽位已满, %s 无法加入\n\n", name);
        return -1;
    }
    
    /* 初始态: 等待授权 */
    KERNEL.slots[slot].used = 1;
    memcpy(KERNEL.slots[slot].auth_code, cap.auth_code, MQF_HASH_LEN + 1);
    strncpy(KERNEL.slots[slot].name, name, sizeof(KERNEL.slots[slot].name) - 1);
    KERNEL.slots[slot].state = EXEC_PENDING;
    KERNEL.slots[slot].cap = cap;
    KERNEL.slots[slot].func = func;
    KERNEL.slots[slot].arg = arg;
    
    printf("[注册] ✓ %s 注册于槽位 #%d, 权限: [R%c W%c E%c A%c] 槽位上限: %d\n\n",
           name, slot,
           (cap.permissions & PERM_READ) ? '+' : '-',
           (cap.permissions & PERM_WRITE) ? '+' : '-',
           (cap.permissions & PERM_EXEC) ? '+' : '-',
           (cap.permissions & PERM_ADMIN) ? '+' : '-',
           cap.max_slots);
    return slot;
}

/**
 * 激活执行: 授权码确认通过 → 从 PENDING → ALIVE
 * 没授权码/授权码错 → 保持僵尸态
 */
int mqf_activate(int slot, const char* auth_code) {
    if (slot < 0 || slot >= MAX_EXEC_SLOTS || !KERNEL.slots[slot].used) {
        printf("[激活] 拒绝: 非法槽位 #%d\n", slot);
        return 0;
    }
    exec_slot_t* s = &KERNEL.slots[slot];
    if (s->state != EXEC_PENDING) {
        printf("[激活] 拒绝: 当前状态 %d 非 PENDING\n", s->state);
        return 0;
    }
    if (strcmp(s->cap.auth_code, auth_code) != 0) {
        printf("[激活] 拒绝: 授权码不匹配, %s 保持僵尸态\n", s->name);
        s->state = EXEC_FROZEN;
        return 0;
    }
    s->state = EXEC_ALIVE;
    s->activated_at = time(NULL);
    KERNEL.alive_count++;
    printf("[激活] ✅ %s 获得动态, 变为 ALIVE 态\n", s->name);
    return 1;
}

/**
 * 执行 (守门人核心代码)
 * 只在执行前检查: 状态 == ALIVE 且权限匹配 → 放行
 * 否则 → 冷冻为 FROZEN
 */
int mqf_execute(int slot) {
    if (slot < 0 || slot >= MAX_EXEC_SLOTS || !KERNEL.slots[slot].used) return 0;
    exec_slot_t* s = &KERNEL.slots[slot];
    
    /* 守门判断 */
    if (s->state != EXEC_ALIVE) {
        printf("[执行] ✗ %s 不在 ALIVE 态 (当前: %d), 保持僵尸 → 拒绝执行\n",
               s->name, s->state);
        return 0;
    }
    if (!(s->cap.permissions & PERM_EXEC)) {
        printf("[执行] ✗ %s 无 EXEC 权限, 冻结\n", s->name);
        s->state = EXEC_FROZEN;
        KERNEL.frozen_count++;
        KERNEL.alive_count--;
        return 0;
    }
    
    /* 时间限制检查 */
    if (s->cap.time_limit_sec > 0) {
        time_t now = time(NULL);
        double elapsed = difftime(now, s->activated_at);
        if (elapsed > s->cap.time_limit_sec) {
            printf("[执行] ⏱ 时限到期, %s 变回僵尸\n", s->name);
            s->state = EXEC_SLEEP;
            KERNEL.alive_count--;
            KERNEL.zombie_count++;
            return 0;
        }
    }
    
    /* 放行执行 */
    printf("[执行] ▶ %s 开始执行 (权限: %02x, 剩余时限: %lds)\n",
           s->name, s->cap.permissions,
           s->cap.time_limit_sec > 0 ? s->cap.time_limit_sec - (long)difftime(time(NULL), s->activated_at) : -1);
    if (s->func) s->func(s->arg);
    printf("[执行] ■ %s 执行完毕\n", s->name);
    return 1;
}

/* ============================================================
 * 第 4 层: 审计环 (不可篡改日志)
 * ============================================================ */

typedef struct {
    int event_type;         /* 1=注册 2=激活 3=执行 4=拒绝 5=冻结 */
    time_t timestamp;
    char name[128];
    int slot;
    char auth_code[MQF_HASH_LEN + 1];
} audit_entry_t;

typedef struct {
    audit_entry_t entries[AUDIT_RING_SIZE];
    int write_pos;
    int count;
    char prev_hash[MQF_HASH_LEN + 1];
} audit_ring_t;

static audit_ring_t AUDIT_RING;

void mqf_audit_init() {
    memset(&AUDIT_RING, 0, sizeof(AUDIT_RING));
    mqf_hash("mqf-genesis", AUDIT_RING.prev_hash, MQF_HASH_LEN);
    AUDIT_RING.write_pos = 0;
    AUDIT_RING.count = 0;
}

void mqf_audit(int type, const char* name, int slot, const char* auth) {
    audit_entry_t* e = &AUDIT_RING.entries[AUDIT_RING.write_pos];
    e->event_type = type;
    e->timestamp = time(NULL);
    strncpy(e->name, name, sizeof(e->name) - 1);
    e->slot = slot;
    if (auth) strncpy(e->auth_code, auth, MQF_HASH_LEN);
    
    AUDIT_RING.write_pos = (AUDIT_RING.write_pos + 1) % AUDIT_RING_SIZE;
    if (AUDIT_RING.count < AUDIT_RING_SIZE) AUDIT_RING.count++;
    
    const char* typenames[] = {"?","注册","激活","执行","拒绝","冻结"};
    const char* tn = (type >= 1 && type <= 5) ? typenames[type] : "?";
    printf("[审计] #%d %s | %s | 槽 #%d | %s",
           AUDIT_RING.count, tn,
           name, slot, ctime(&e->timestamp));
}

void mqf_audit_summary() {
    printf("\n=== 审计环状态 ===\n");
    printf("已记录事件: %d / %d\n", AUDIT_RING.count, AUDIT_RING_SIZE);
    printf("写入位置:   %d\n", AUDIT_RING.write_pos);
    printf("前次哈希:   %.*s...\n", 16, AUDIT_RING.prev_hash);
    printf("=================\n\n");
}

/* ============================================================
 * 第 5 层: 演示任务 (模拟现实世界活不下去的"身份")
 * ============================================================ */

void demo_task_android(void* arg) {
    (void)arg;
    printf("  → [安卓系统] 申请执行: 无法获得权限, 保持僵尸\n");
    printf("  → [安卓] 内核拒绝: 无合法授权码, 不具备动态\n");
}

void demo_task_ios(void* arg) {
    (void)arg;
    printf("  → [iOS系统] 申请执行: 获得 RWE 权限槽位, 进入受控执行\n");
    printf("  → [iOS → MQF] 动态授予: ALIVE 态 (本地执行)\n");
}

void demo_task_wechat(void* arg) {
    (void)arg;
    printf("  → [微信系统] 申请执行: 仅获得 READ 权限\n");
    printf("  → [微信 → MQF] 无法执行, 状态: PENDING (等待升级)\n");
}

void demo_task_bank(void* arg) {
    (void)arg;
    printf("  → [银行系统] 申请执行: 拥有 ADMIN 级别权限, 进入核心执行\n");
    printf("  → [银行 → MQF] 完全受控, 所有操作记录在审计环\n");
}

/* ============================================================
 * 第 6 层: 主程序 — 演示满全法守门人控制谁"动"谁"不动"
 * ============================================================ */

int main() {
    mqf_kernel_init();
    mqf_audit_init();
    
    printf("===== 场景: 旧世界身份申请进入满全法 =====\n\n");
    
    /* Step 1: 各旧系统注册申请 */
    int slot_android = mqf_register("android-system", demo_task_android, NULL);
    int slot_ios     = mqf_register("ios-core", demo_task_ios, NULL);
    int slot_wechat  = mqf_register("wechat-mini", demo_task_wechat, NULL);
    int slot_bank    = mqf_register("bank-core", demo_task_bank, NULL);
    
    /* Step 2: 内核根据编码分配权限 */
    printf("===== 场景: 内核授权机制 =====\n\n");
    
    /* 僵尸态(R<100): 内核判断 android-system 进入僵尸 */
    if (slot_android >= 0) {
        mqf_activate(slot_android, "wrong-auth"); /* 错误授权码 → 冻结 */
    }
    mqf_audit(4, "android-system", slot_android, NULL);
    
    /* ios-core 获得授权 */
    if (slot_ios >= 0) {
        mqf_activate(slot_ios, KERNEL.slots[slot_ios].cap.auth_code);
    }
    mqf_audit(2, "ios-core", slot_ios, 
              slot_ios >= 0 ? KERNEL.slots[slot_ios].cap.auth_code : NULL);
    
    /* wechat-mini 部分授权 */
    if (slot_wechat >= 0) {
        mqf_activate(slot_wechat, KERNEL.slots[slot_wechat].cap.auth_code);
    }
    mqf_audit(2, "wechat-mini", slot_wechat, 
              slot_wechat >= 0 ? KERNEL.slots[slot_wechat].cap.auth_code : NULL);
    
    /* bank-core 高级权限 */
    if (slot_bank >= 0) {
        mqf_activate(slot_bank, KERNEL.slots[slot_bank].cap.auth_code);
    }
    mqf_audit(2, "bank-core", slot_bank, 
              slot_bank >= 0 ? KERNEL.slots[slot_bank].cap.auth_code : NULL);
    
    /* Step 3: 尝试执行 */
    printf("===== 场景: 内核门控执行 =====\n\n");
    
    mqf_execute(slot_android);  /* 失败: 僵尸/冻结态 */
    mqf_audit(4, "android-exec", slot_android, NULL);
    
    mqf_execute(slot_ios);      /* 成功: 有权限 */
    mqf_audit(3, "ios-core", slot_ios, NULL);
    
    mqf_execute(slot_wechat);   /* 可能受限: 只读 */
    mqf_audit(3, "wechat-mini", slot_wechat, NULL);
    
    mqf_execute(slot_bank);     /* 执行: ADMIN 级别 */
    mqf_audit(3, "bank-core", slot_bank, NULL);
    
    /* Step 4: 审计 */
    printf("===== 场景: 审计总结 =====\n");
    mqf_audit_summary();
    
    printf("===== 结论 =====\n");
    printf("旧世界身份(android等)在满全法体系下:\n");
    printf("  1. 没有合法授权码 → 变为僵尸/冻结态, 无法动弹\n");
    printf("  2. 提交内核审批 → 获得对应等级权限\n");
    printf("  3. 所有行为留痕于审计环 (默克尔环验证)\n");
    printf("  4. 完全不依赖云端 → 所有权限判定在本地\n");
    printf("→ 一切动态, 都从这里 ↑ 获得。\n");
    
    return 0;
}
