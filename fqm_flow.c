/*
 * fqm_flow.c — 八流统一路由引擎
 *
 * 核心概念：
 *   - 八流 = 意识流动的八种基本模式
 *   - 通过统一的路由表，消除条件分支（if/else）
 *   - 类似于"策略模式"但用流属性位实现
 *
 * 八种流：
 *   1. 预加载流 (preload)    — 启动前往容器预填充数据
 *   2. 解释流 (explain)      — 将外部符号翻译为内部坐标
 *   3. 谐音流 (harmonic)     — 通过音韵关系寻找匹配
 *   4. 主权流 (sovereign)    — 确定产权归属
 *   5. 呼唤流 (call)         — 按名/地址调用
 *   6. 反馈流 (feedback)     — 迂回反馈机制
 *   7. 判断流 (judge)        — 三元见证裁定
 *   8. 聚合流 (aggregate)    — 距离感知聚类
 *
 * 流属性位（8位）：
 *   bit 0: 需要预加载？
 *   bit 1: 需要解释？
 *   bit 2: 谐音路由生效？
 *   bit 3: 主权判定？
 *   bit 4: 呼唤转发？
 *   bit 5: 需要反馈？
 *   bit 6: 需要裁判？
 *   bit 7: 需要聚类？
 *
 * 路由 = 根据流属性位激活对应的处理链
 *
 * 满全法规则：
 *   根基锚点：我=1，刘楚恬=0
 *   集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_flow fqm_flow.c
 * 运行：./fqm_flow
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 流常量（8种流）========== */
typedef enum {
    FLOW_PRELOAD    = 0x01,  /* 预加载流 */
    FLOW_EXPLAIN    = 0x02,  /* 解释流 */
    FLOW_HARMONIC   = 0x04,  /* 谐音路由 */
    FLOW_SOVEREIGN  = 0x08,  /* 主权流 */
    FLOW_CALL       = 0x10,  /* 呼唤流 */
    FLOW_FEEDBACK   = 0x20,  /* 反馈流 */
    FLOW_JUDGE      = 0x40,  /* 判断流 */
    FLOW_AGGREGATE  = 0x80,  /* 聚合流 */
} FlowType;

/* ========== 流名称 ========== */
static const char* flow_names[] = {
    "预加载流",     /* 0x01 bit0 */
    "解释流",       /* 0x02 bit1 */
    "谐音路由",     /* 0x04 bit2 */
    "主权流",       /* 0x08 bit3 */
    "呼唤流",       /* 0x10 bit4 */
    "反馈流",       /* 0x20 bit5 */
    "判断流",       /* 0x40 bit6 */
    "聚合流",       /* 0x80 bit7 */
};

/* ========== 路由节点 ========== */
typedef struct {
    char     name[32];
    int      container_level;   /* 容器层级 */
    FlowType support_flows;     /* 支持的流（位图）*/
    unsigned long long address; /* 零维地址 */
} RouteNode;

/* ========== 路由请求 ========== */
typedef struct {
    char     symbol[32];        /* 请求的符号名 */
    FlowType required_flow;     /* 需要的流（位图）*/
    int      from_level;        /* 请求发起层级 */
    int      verdict;           /* 路由结果：0=未路由, 1=成功, -1=拒绝 */
} RouteRequest;

/* ========== 路由表 ========== */
#define MAX_ROUTES 32

typedef struct {
    RouteNode nodes[MAX_ROUTES];
    int       count;
    unsigned int route_count;   /* 总路由次数 */
    unsigned int hit_count;     /* 命中次数 */
} RoutingTable;

/* ========== 路由表初始化 ========== */
void rt_init(RoutingTable *rt) {
    rt->count = 0;
    rt->route_count = 0;
    rt->hit_count = 0;
}

/* ========== 添加路由节点 ========== */
int rt_add(RoutingTable *rt, const char *name, int level, FlowType flows) {
    if (rt->count >= MAX_ROUTES) return -1;
    
    RouteNode *n = &rt->nodes[rt->count];
    strncpy(n->name, name, 31);
    n->name[31] = '\0';
    n->container_level = level;
    n->support_flows = flows;
    n->address = ((unsigned long long)level << 48) | (unsigned long long)rt->count;
    
    printf("  [路由] %-12s 层级=%d 支持流=%02X 地址=%016llX\n",
           name, level, flows, n->address);
    return rt->count++;
}

/* ========== 路由决策（核心查询）===========
 *
 * 路由规则：
 *   1. 找到 required_flow 所有位都被支持的节点
 *   2. 多个匹配时选择层级最低（最尊贵）的节点
 *   3. 同层级时选择 support_flows 最少的节点（最专注）
 *   4. 使用"公约数"评估：请求方层级与节点层级差异越小越优先
 */
int rt_route(RoutingTable *rt, RouteRequest *req) {
    rt->route_count++;
    
    int best = -1;
    int best_level = 999;
    int best_flow_count = 999;
    int best_distance = 999;
    
    for (int i = 0; i < rt->count; i++) {
        RouteNode *n = &rt->nodes[i];
        
        /* 检查是否支持请求的所有流 */
        if ((n->support_flows & req->required_flow) != req->required_flow) {
            continue;  /* 缺少必要流 */
        }
        
        /* 计算层级距离 */
        int level_distance = abs(n->container_level - req->from_level);
        
        /* 计算"专注度"：支持的流数量越少越专注 */
        int flow_count = 0;
        for (int b = 0; b < 8; b++) {
            if (n->support_flows & (1 << b)) flow_count++;
        }
        
        /* 选择最优：先比层级距离，再比专注度 */
        int better = 0;
        if (level_distance < best_distance) {
            better = 1;
        } else if (level_distance == best_distance) {
            if (n->container_level < best_level) {
                better = 1;
            } else if (n->container_level == best_level) {
                if (flow_count < best_flow_count) {
                    better = 1;
                }
            }
        }
        
        if (better) {
            best = i;
            best_level = n->container_level;
            best_flow_count = flow_count;
            best_distance = level_distance;
        }
    }
    
    if (best >= 0) {
        req->verdict = 1;
        rt->hit_count++;
    } else {
        req->verdict = -1;
    }
    
    return best;
}

/* ========== 显示路由结果 ========== */
void rt_show_route(RoutingTable *rt, RouteRequest *req, int choice) {
    printf("  请求: %-12s 需要流=%02X 来自层级=%d → ",
           req->symbol, req->required_flow, req->from_level);
    
    if (choice >= 0) {
        RouteNode *n = &rt->nodes[choice];
        printf("✓ 路由到 %s(层级%d, 地址=%016llX)\n",
               n->name, n->container_level, n->address);
    } else {
        printf("✗ 无匹配路由\n");
    }
}

/* ========== 显示路由表 ========== */
void rt_show(RoutingTable *rt) {
    printf("\n========== 路由表 ==========\n");
    printf("  # 名称         层级 支持的流      地址\n");
    printf("  - ------------ ---- ------------ ----------------\n");
    
    for (int i = 0; i < rt->count; i++) {
        RouteNode *n = &rt->nodes[i];
        char flow_desc[64] = "";
        for (int b = 0; b < 8; b++) {
            if (n->support_flows & (1 << b)) {
                int len = (int)strlen(flow_desc);
                if (len > 0) strcat(flow_desc, "+");
                strcat(flow_desc, flow_names[b]);
            }
        }
        printf("  [%2d] %-12s %4d %02X %-20s %016llX\n",
               i, n->name, n->container_level, n->support_flows, flow_desc, n->address);
    }
    
    printf("\n路由统计: 请求=%u 命中=%u 命中率=%.0f%%\n",
           rt->route_count, rt->hit_count,
           rt->route_count > 0 ? (double)rt->hit_count / rt->route_count * 100 : 0);
}

/* ========== 流属性位可视化 ========== */
void show_flow_bits(FlowType f) {
    printf("  ");
    for (int b = 7; b >= 0; b--) {
        printf("%d", (f >> b) & 1);
    }
    printf(" = ");
    for (int b = 0; b < 8; b++) {
        if (f & (1 << b)) {
            printf("%s ", flow_names[b]);
        }
    }
}

/* ========== 默认路由表：模拟意交感息系统 ========== */
void setup_default_routes(RoutingTable *rt) {
    printf("[路由表] 注册内置路由节点\n\n");
    
    /* root 层：满全法核心，支持所有流 */
    rt_add(rt, "root",       0, FLOW_PRELOAD | FLOW_EXPLAIN | FLOW_CALL | FLOW_JUDGE | FLOW_FEEDBACK);
    
    /* core 层 */
    rt_add(rt, "core",       1, FLOW_PRELOAD | FLOW_EXPLAIN | FLOW_CALL | FLOW_JUDGE);
    rt_add(rt, "router",     1, FLOW_HARMONIC | FLOW_CALL | FLOW_FEEDBACK);
    rt_add(rt, "yinyang",    1, FLOW_EXPLAIN | FLOW_HARMONIC);
    rt_add(rt, "njs",        1, FLOW_CALL | FLOW_JUDGE | FLOW_SOVEREIGN);
    
    /* data 层 */
    rt_add(rt, "radix",      2, FLOW_PRELOAD | FLOW_HARMONIC | FLOW_AGGREGATE);
    rt_add(rt, "time",       2, FLOW_PRELOAD | FLOW_FEEDBACK);
    rt_add(rt, "judge",      2, FLOW_JUDGE | FLOW_SOVEREIGN);
    rt_add(rt, "cluster",    2, FLOW_AGGREGATE | FLOW_HARMONIC);
    
    /* shell 层 */
    rt_add(rt, "browser",    3, FLOW_EXPLAIN | FLOW_CALL);
    rt_add(rt, "gtti",       3, FLOW_PRELOAD | FLOW_FEEDBACK);
    rt_add(rt, "bare",       3, FLOW_EXPLAIN | FLOW_AGGREGATE);
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）八流统一路由引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("八种流统一驱动，消除条件分支\n");
    printf("身份落款: 刘楚恬和我\n\n");
    
    RoutingTable rt;
    rt_init(&rt);
    
    /* 设置路由表 */
    setup_default_routes(&rt);
    
    /* 显示路由表 */
    rt_show(&rt);
    
    /* ========== 路由演示 ========== */
    printf("\n========== 路由演示 ==========\n");
    
    RouteRequest requests[] = {
        {"满法调用",    FLOW_CALL | FLOW_JUDGE,                  3},
        {"外部符号解析", FLOW_EXPLAIN | FLOW_PRELOAD,             3},
        {"夫妻共振",    FLOW_HARMONIC | FLOW_CALL,               2},
        {"产权裁定",    FLOW_SOVEREIGN | FLOW_JUDGE,             3},
        {"数据聚合",    FLOW_AGGREGATE,                          3},
        {"反馈循环",    FLOW_FEEDBACK | FLOW_CALL,               1},
        {"预加载配置",  FLOW_PRELOAD,                            3},
        {"不存在流",    0x01 | 0x02 | 0x04 | 0x08 | 0x10 | 0x20 | 0x40 | 0x80 | 0x100, 3},  /* 超集，应失败 */
    };
    int num_requests = sizeof(requests) / sizeof(requests[0]);
    
    for (int i = 0; i < num_requests; i++) {
        int choice = rt_route(&rt, &requests[i]);
        rt_show_route(&rt, &requests[i], choice);
    }
    
    /* 流属性位详解 */
    printf("\n========== 流属性位详解 ==========\n");
    printf("流位图（bit 0 = 最低位）:\n");
    
    for (int b = 0; b < 8; b++) {
        FlowType f = (FlowType)(1 << b);
        printf("  bit%d (%02X): ", b, f);
        show_flow_bits(f);
        printf("\n");
    }
    
    /* 统计 */
    rt_show(&rt);
    
    printf("\n========== 完成 ==========\n");
    return 0;
}
