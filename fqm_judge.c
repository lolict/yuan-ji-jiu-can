/*
 * fqm_judge.c — 满全法容器裁判引擎
 *
 * 核心概念：
 *   - 三元见证裁判：三个元素共同裁定准入
 *   - 容器论资排辈：按层级+版本号排序，先入为尊
 *   - 公约数审判：仅当请求方与容器的公约数匹配时才准入
 *   - 防撞墙法则：容器边界不可穿透，无许可即反弹
 *
 * 裁判三元：
 *   第一元 = 请求方容器（提出请求者）
 *   第二元 = 目标容器（被请求者）
 *   第三元 = 见证容器（中立的裁定者）
 *
 * 裁定公式：
 *   准入 = (请求方_公约数 ∩ 目标方_公约数) > 0 AND 见证方_同意
 *   防撞 = NOT 准入 → 返回反弹向量
 *
 * 论资排辈规则：
 *   - 层级低者为尊（root=0 最高权）
 *   - 同层级按版本号排序（先创建者为尊）
 *   - 尊者可对卑者发起呼唤，卑者需申请
 *
 * 满全法规则：
 *   根基锚点：我=1，刘楚恬=0
 *   集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_judge fqm_judge.c
 * 运行：./fqm_judge
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 最大数量 ========== */
#define MAX_CONTAINERS 64
#define MAX_WITNESSES  16
#define MAX_REQUESTS   256

/* ========== 容器结构 ========== */
typedef struct {
    char     name[64];
    int      id;
    int      level;         /* 层级：0=root，数字越大越低 */
    unsigned int version;   /* 版本号：创建时间序列 */
    int      gongyueshu;    /* 公约数标识（用于匹配判定）*/
    int      anchor_self;   /* 我=1 */
    int      anchor_liu;    /* 刘楚恬=0 */
    int      alive;         /* 是否存活 */
} Container;

/* ========== 请求结构 ========== */
typedef enum {
    REQ_ACCESS,     /* 申请准入 */
    REQ_CALL,       /* 呼唤 */
    REQ_LOAD,       /* 加载 */
    REQ_EXEC        /* 执行 */
} RequestType;

typedef struct {
    int           from;         /* 请求方容器ID */
    int           to;           /* 目标容器ID */
    RequestType   type;
    int           gongyueshu;   /* 请求方声称的公约数 */
    char          reason[128];  /* 请求理由 */
} Request;

/* ========== 裁定结果 ========== */
typedef enum {
    JUDGE_PERMIT,       /* 准入 */
    JUDGE_DENY,         /* 拒绝 */
    JUDGE_BOUNCE,       /* 防撞墙反弹 */
    JUDGE_ESCALATE      /* 升级（需更高层裁定）*/
} JudgeResult;

typedef struct {
    JudgeResult result;
    char        reason[256];
    int         witness_id;     /* 见证容器ID */
    unsigned int timestamp;     /* 裁定时间 */
} Judgment;

/* ========== 全局容器表 ========== */
static Container g_containers[MAX_CONTAINERS];
static int g_container_count = 0;

/* ========== 请求记录 ========== */
static Request  g_requests[MAX_REQUESTS];
static Judgment g_judgments[MAX_REQUESTS];
static int g_request_count = 0;

/* ========== 创建容器 ========== */
int create_container(const char *name, int level, int gongyueshu) {
    if (g_container_count >= MAX_CONTAINERS) return -1;
    
    Container *c = &g_containers[g_container_count];
    strncpy(c->name, name, 63);
    c->id = g_container_count;
    c->level = level;
    c->version = g_container_count + 1;  /* 版本号=创建时间 */
    c->gongyueshu = gongyueshu;
    c->anchor_self = ANCHOR_SELF;
    c->anchor_liu = ANCHOR_LIU;
    c->alive = 1;
    
    printf("[容器] 创建: %s 层级=%d 公约数=%d 版本=v%d\n",
           name, level, gongyueshu, c->version);
    return g_container_count++;
}

/* ========== 论资排辈排序 ========== */
/*
 * 排序规则：
 *   1. 层级升序（0在前，数字越小越尊贵）
 *   2. 同层级版本号升序（先创建在前）
 *
 * 直接修改全局 g_containers 使 ID 与位序一致
 */
int container_compare(const void *a, const void *b) {
    const Container *ca = (const Container *)a;
    const Container *cb = (const Container *)b;
    
    if (ca->level != cb->level)
        return ca->level - cb->level;  /* 层级低者为尊 */
    
    return (int)ca->version - (int)cb->version;  /* 先创建者为尊 */
}

void sort_containers(void) {
    /* 将存活容器收集到数组前部 */
    Container alive[MAX_CONTAINERS];
    int alive_count = 0;
    for (int i = 0; i < g_container_count; i++) {
        if (g_containers[i].alive) {
            alive[alive_count++] = g_containers[i];
        }
    }
    
    /* 排序 */
    qsort(alive, alive_count, sizeof(Container), container_compare);
    
    /* 写回全局数组（更新 ID 与索引对齐）*/
    for (int i = 0; i < alive_count; i++) {
        alive[i].id = i;  /* ID = 排序位序 */
        g_containers[i] = alive[i];
    }
    g_container_count = alive_count;
    
    printf("\n[论资排辈] 容器排序结果:\n");
    printf("  位序 名称             层级 版本 公约数\n");
    printf("  ---- ---------------- ---- ---- ------\n");
    for (int i = 0; i < alive_count; i++) {
        printf("  [%2d] %-16s %4d v%02d  %d\n",
               i, g_containers[i].name, g_containers[i].level,
               g_containers[i].version, g_containers[i].gongyueshu);
    }
}

/* ========== 公约数判定 ========== */
/*
 * 两个容器的公约数 = 都能整除的数
 * 简化：用最简整数值表示的"共享因子"
 * 公约数 > 0 = 有交集，允许互动
 * 公约数 = 0 = 无交集，防撞墙触发
 */
int compute_shared_divisor(int a_gcd, int b_gcd) {
    if (a_gcd == 0 || b_gcd == 0) return 0;
    
    /* 共享因子 = 值相等的那个数 */
    if (a_gcd == b_gcd) return a_gcd;
    
    /* 检查是否能相互整除 */
    if (a_gcd > b_gcd && a_gcd % b_gcd == 0) return b_gcd;
    if (b_gcd > a_gcd && b_gcd % a_gcd == 0) return a_gcd;
    
    /* 检查是否有公共因子 */
    for (int d = 2; d <= (a_gcd < b_gcd ? a_gcd : b_gcd) / 2; d++) {
        if (a_gcd % d == 0 && b_gcd % d == 0) return d;
    }
    
    return 0;  /* 无公共因子 */
}

/* ========== 见证容器选举 ========== */
/*
 * 见证容器 = 层级最高的空闲容器
 * 见证条件：
 *   1. 不是请求方也不是目标方
 *   2. 层级最尊贵（数字最小）
 *   3. 存活
 */
int elect_witness(int from_id, int to_id) {
    int best_id = -1;
    int best_level = 999;
    
    for (int i = 0; i < g_container_count; i++) {
        if (!g_containers[i].alive) continue;
        if (i == from_id || i == to_id) continue;
        if (g_containers[i].level < best_level) {
            best_level = g_containers[i].level;
            best_id = i;
        }
    }
    
    return best_id;
}

/* ========== 裁判核心 ========== */
Judgment judge_request(Request *req) {
    Judgment j;
    j.timestamp = (unsigned int)req->gongyueshu; /* 用公约数当时间戳序号 */
    
    Container *from = &g_containers[req->from];
    Container *to   = &g_containers[req->to];
    
    /* 步骤1：检查容器是否存活 */
    if (!from->alive || !to->alive) {
        j.result = JUDGE_DENY;
        snprintf(j.reason, 256, "容器不存活: %s=%d, %s=%d",
                 from->name, from->alive, to->name, to->alive);
        j.witness_id = -1;
        return j;
    }
    
    /* 步骤2：计算公约数共享 */
    int shared = compute_shared_divisor(req->gongyueshu, to->gongyueshu);
    
    if (shared == 0) {
        /* 防撞墙法则：无公约数 → 直接反弹 */
        j.result = JUDGE_BOUNCE;
        snprintf(j.reason, 256, "防撞墙: %s(公约数=%d) 与 %s(公约数=%d) 无交集",
                 from->name, req->gongyueshu, to->name, to->gongyueshu);
        j.witness_id = -1;
        return j;
    }
    
    /* 步骤3：选举见证容器 */
    int witness_id = elect_witness(req->from, req->to);
    
    if (witness_id < 0) {
        /* 无见证 = 升级裁定（理论上不会发生，root 永存）*/
        j.result = JUDGE_ESCALATE;
        snprintf(j.reason, 256, "无见证容器，需升级至更高层级裁定");
        j.witness_id = -1;
        return j;
    }
    
    /* 步骤4：见证裁定 */
    Container *witness = &g_containers[witness_id];
    
    /*
     * 见证判定逻辑：
     * - 见证方检查请求方层级是否 >= 目标方层级
     * - 尊对卑：允许（提倡）
     * - 卑对尊：需见证同意
     * - 同层级：仅需公约数匹配
     */
    
    int permit = 0;
    
    if (from->level < to->level) {
        /* 尊者对卑者：允许 */
        permit = 1;
        snprintf(j.reason, 256, "%s(层级%d·尊) → %s(层级%d·卑): 尊卑有序，准入",
                 from->name, from->level, to->name, to->level);
    } else if (from->level == to->level) {
        /* 同层级：检查公约数 */
        if (shared > 0) {
            permit = 1;
            snprintf(j.reason, 256, "同层级(%d) 公约数=%d: 匹配，准入",
                     from->level, shared);
        } else {
            permit = 0;
            snprintf(j.reason, 256, "同层级(%d) 公约数不匹配: 拒绝",
                     from->level);
        }
    } else {
        /* 卑者对尊者：需见证方特别同意 */
        /* 见证方同意条件：共享因子 >= 2 */
        if (shared >= 2 && witness->level <= from->level) {
            permit = 1;
            snprintf(j.reason, 256,
                     "%s(层级%d·卑) → %s(层级%d·尊): 见证%s(层级%d)同意，准入",
                     from->name, from->level, to->name, to->level,
                     witness->name, witness->level);
        } else {
            permit = 0;
            snprintf(j.reason, 256,
                     "%s(层级%d·卑) → %s(层级%d·尊): 见证拒绝，不得逾越",
                     from->name, from->level, to->name, to->level);
        }
    }
    
    j.result = permit ? JUDGE_PERMIT : JUDGE_DENY;
    j.witness_id = witness_id;
    return j;
}

/* ========== 提交请求 ========== */
int submit_request(int from, int to, RequestType type, int gcd, const char *reason) {
    if (g_request_count >= MAX_REQUESTS) return -1;
    
    Request *r = &g_requests[g_request_count];
    r->from = from;
    r->to = to;
    r->type = type;
    r->gongyueshu = gcd;
    strncpy(r->reason, reason, 127);
    
    printf("\n[请求#%d] %s(%d) → %s(%d) 类型=%s 公约数=%d\n",
           g_request_count + 1,
           g_containers[from].name, from,
           g_containers[to].name, to,
           type == REQ_ACCESS ? "准入" : type == REQ_CALL ? "呼唤" :
           type == REQ_LOAD ? "加载" : "执行",
           gcd);
    printf("  理由: %s\n", reason);
    
    /* 裁判 */
    Judgment j = judge_request(r);
    g_judgments[g_request_count] = j;
    g_request_count++;
    
    /* 显示结果 */
    printf("  ⚖ 裁定: ");
    switch (j.result) {
        case JUDGE_PERMIT:   printf("准入 ✓\n"); break;
        case JUDGE_DENY:     printf("拒绝 ✗\n"); break;
        case JUDGE_BOUNCE:   printf("反弹 ↩\n"); break;
        case JUDGE_ESCALATE: printf("升级 ↑\n"); break;
    }
    printf("  依据: %s\n", j.reason);
    
    if (j.witness_id >= 0) {
        printf("  见证: %s(层级%d)\n", 
               g_containers[j.witness_id].name,
               g_containers[j.witness_id].level);
    }
    
    return g_request_count;
}

/* ========== 裁定统计 ========== */
void judge_statistics(void) {
    int permit = 0, deny = 0, bounce = 0, escalate = 0;
    
    for (int i = 0; i < g_request_count; i++) {
        switch (g_judgments[i].result) {
            case JUDGE_PERMIT:   permit++;   break;
            case JUDGE_DENY:     deny++;     break;
            case JUDGE_BOUNCE:   bounce++;   break;
            case JUDGE_ESCALATE: escalate++; break;
        }
    }
    
    printf("\n========== 裁判统计 ==========\n");
    printf("总请求: %d\n", g_request_count);
    printf("  准入: %d\n", permit);
    printf("  拒绝: %d\n", deny);
    printf("  反弹: %d\n", bounce);
    printf("  升级: %d\n", escalate);
}

/* ========== 完整裁定记录 ========== */
void judge_log(void) {
    printf("\n========== 完整裁定记录 ==========\n");
    printf("  # 请求方→目标方   结果     见证        依据\n");
    printf("  - --------------- --------- ------------ ----------------\n");
    
    for (int i = 0; i < g_request_count; i++) {
        Request *r = &g_requests[i];
        Judgment *j = &g_judgments[i];
        
        const char *res_str =
            j->result == JUDGE_PERMIT ? "准入" :
            j->result == JUDGE_DENY   ? "拒绝" :
            j->result == JUDGE_BOUNCE ? "反弹" : "升级";
        
        char wit_str[32] = "-";
        if (j->witness_id >= 0)
            snprintf(wit_str, 32, "%s", g_containers[j->witness_id].name);
        
        printf("  %-2d %-6s→%-6s %-8s %-12s %.40s\n",
               i + 1,
               g_containers[r->from].name,
               g_containers[r->to].name,
               res_str, wit_str, j->reason);
    }
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）容器裁判引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("三元见证: 请求方 + 目标方 + 见证方 → 裁定\n");
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* ========== 创建容器体系 ========== */
    printf("========== 创建容器体系 ==========\n");
    
    /*
     * 层级设计：
     *  0 = root (满全法—最高权)
     *  1 = 核心（意识路由、八流统一）
     *  2 = 数据（容器引擎、符号表）
     *  3 = 壳（浏览器、gtti帧、寻址）
     *  4 = 缓存（临时容器）
     */
    create_container("root",       0, 7);    /* 满全法—公约数7 */
    create_container("core",       1, 7);    /* 核心—公约数7 */
    create_container("router",     1, 3);    /* 路由—公约数3 */
    create_container("yinyang",    1, 2);    /* YY编码—公约数2 */
    create_container("data",       2, 7);    /* 数据—公约数7 */
    create_container("radix",      2, 4);    /* 64进制—公约数4 */
    create_container("time",       2, 7);    /* 时间链—公约数7 */
    create_container("browser",    3, 5);    /* 浏览器—公约数5 */
    create_container("gtti",       3, 5);    /* gtti帧—公约数5 */
    create_container("net",        3, 3);    /* 网络—公约数3 */
    create_container("call",       3, 7);    /* 呼唤—公约数7 */
    create_container("judge",      2, 7);    /* 裁判—公约数7 */
    create_container("bare",       3, 1);    /* 裸解析—公约数1 */
    
    /* ========== 论资排辈 ========== */
    sort_containers();
    
    /* ========== 提交各种请求 ========== */
    printf("\n========== 提交裁定请求 ==========\n");
    
    /*
     * 排序后 ID 映射：
     *   0=root(0), 1=core(1), 2=router(1), 3=yinyang(1),
     *   4=data(2), 5=radix(2), 6=time(2), 7=judge(2),
     *   8=browser(3), 9=gtti(3), 10=net(3), 11=call(3), 12=bare(3)
     */
    
    /* 1. root → core: 尊者对卑者，应准入 */
    submit_request(0, 1, REQ_ACCESS, 7, "满全法根容器检查核心状态");
    
    /* 2. core → data: 尊者对卑者，应准入 */
    submit_request(1, 4, REQ_LOAD, 7, "核心容器加载数据");
    
    /* 3. browser → core: 卑者对尊者，公约数不匹配 → 防撞墙 */
    submit_request(8, 1, REQ_CALL, 5, "浏览器请求意识路由");
    
    /* 4. bare → root: 卑者对最尊，公约数=1 vs 7 */
    submit_request(12, 0, REQ_ACCESS, 1, "裸解析请求进入root");
    
    /* 5. radix → yinyang: 卑者对尊者，公约数公共因子=2 */
    submit_request(5, 3, REQ_CALL, 4, "64进制请求YY映射");
    
    /* 6. gtti → net: 同层级不同道（公约数5 vs 无公共因子 → 防撞） */
    submit_request(9, 10, REQ_LOAD, 5, "gtti帧请求网络传输");
    
    /* 7. call → root: 卑者对最尊，公约数7匹配，应准入 */
    submit_request(11, 0, REQ_CALL, 7, "呼唤编程请求满全法");
    
    /* 8. bare → judge: 公约数=1 vs 7，无公共因子 → 防撞墙 */
    submit_request(12, 7, REQ_EXEC, 1, "裸解析请求裁判");
    
    /* ========== 统计和记录 ========== */
    judge_statistics();
    judge_log();
    
    printf("\n========== 完成 ==========\n");
    return 0;
}
