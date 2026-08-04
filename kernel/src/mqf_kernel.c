/**
 * 满全法执行内核 v0.5 — MQF Executive Kernel
 * -----------------------------------------
 * v0.5 新增: 空间关系代数 (对象图 + 路径可达 + 态势逻辑)
 *   - 建立 N 个对象,每个对象有其编码身份
 *   - 对象之间可建立 观测(observe)/绑定(bind)/变换(transform) 关系
 *   - 路径可达查询 A → B
 *   - 间接力评估: 经 hop 步传递后的连接强度
 *   - 内镜像: 对象在 depth 层内的可达结构
 *   - 态势判决: 整个图是 稳态 / 渐变态 / 逆变态
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>
#include <math.h>

#define MQF_VERSION     "0.5.0"
#define MQF_AUTH_LEN    32
#define MQF_HASH_LEN    64
#define MAX_EXEC_SLOTS  16
#define AUDIT_RING_SIZE 1024
#define MAX_CONTRACTS   64

/* 关系代数常量 */
#define MAX_OBJECTS     128
#define MAX_RELATIONS   1024
#define MAX_NAME_LEN    128

typedef enum { EXEC_SLEEP=0, EXEC_PENDING, EXEC_ALIVE, EXEC_FROZEN, EXEC_ZOMBIE } exec_state_t;
typedef enum { PERM_NONE=0x00, PERM_READ=0x01, PERM_WRITE=0x02, PERM_EXEC=0x04, PERM_NET=0x08, PERM_ADMIN=0x10 } perm_t;
typedef enum { FACT_HAS_AUTH=1, FACT_HAS_NO_AUTH, FACT_TRUST_SOURCE, FACT_UNTRUST_SOURCE, FACT_DEFAULT } fact_type_t;
typedef enum { AUDIT_REGISTER=1, AUDIT_ACTIVATE, AUDIT_EXECUTE, AUDIT_REJECT, AUDIT_FREEZE, AUDIT_TIMEOUT } audit_type_t;
typedef enum { REL_OBSERVE=0, REL_BIND=1, REL_TRANSFORM=2 } rel_type_t;

static const char* MQFS_INITS[]={"b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w","@"};
static const char* MQFS_FINALS[]={"a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er","an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao","iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong","@"};
(void)MQFS_FINALS; /* 保留作完备性,编码系统后续可扩展 */
static const char* REL_NAMES[]={"观测","绑定","变换"};

/* ============================================================
 * 第 1 层: 身份编码 (沿用 v0.3)
 * ============================================================ */
typedef struct { char text[MQF_AUTH_LEN+1]; int init_idx,final_idx,tone_idx,stroke_count; char auth_code[MQF_HASH_LEN+1],hash[MQF_HASH_LEN+1]; } mqf_identity_t;
static void mqf_hash(const char* in, char* out){ uint32_t h1=0x811c9dc5u,h2=0x0100193u; for(size_t i=0;i<strlen(in);i++){h1^=(uint8_t)in[i];h1*=0x01000193u;h2^=(uint8_t)in[i]<<5;h2*=0x1000193u;} sprintf(out,"%08x%08x%08x%08x%08x%08x",h1,h2,h1^h2,h2^h1,h1+h2,h2-h1); }
mqf_identity_t mqf_encode(const char* text){ mqf_identity_t id; memset(&id,0,sizeof(id)); size_t len=strlen(text); if(len>MQF_AUTH_LEN)len=MQF_AUTH_LEN; memcpy(id.text,text,len); id.text[len]='\0'; uint32_t seed=0; for(size_t i=0;i<len;i++) seed=seed*31+(unsigned char)text[i]; id.init_idx=seed%24; id.final_idx=(seed>>5)%39; id.tone_idx=(seed>>10)%5; id.stroke_count=(seed>>15)%30+1; char buf[512]; snprintf(buf,sizeof(buf),"%s|%d|%d|%d|%d",MQFS_INITS[id.init_idx],id.init_idx,id.final_idx,id.tone_idx,id.stroke_count); mqf_hash(buf,id.auth_code); snprintf(buf,sizeof(buf),"%s.%s",text,id.auth_code); mqf_hash(buf,id.hash); return id; }

/* ============================================================
 * 第 2 层: 契约系统 (沿用 v0.3)
 * ============================================================ */
typedef struct { char identity[128]; int target_level; perm_t target_perm; fact_type_t condition; char note[256]; } contract_t;
typedef struct { contract_t rules[MAX_CONTRACTS]; int count; struct { char name[128]; fact_type_t fact; } facts[MAX_CONTRACTS*2]; int fact_count; } contract_engine_t;
typedef struct { char auth_code[MQF_HASH_LEN+1]; perm_t perm; int max_slots, time_limit_sec, require_network; time_t created_at, expires_at; } capability_t;
static contract_engine_t CE;

void mqf_fact(const char* id, fact_type_t f){ if(CE.fact_count<MAX_CONTRACTS*2){strncpy(CE.facts[CE.fact_count].name,id,127);CE.facts[CE.fact_count].fact=f;CE.fact_count++;} }
int mqf_has_fact(const char* id, fact_type_t f){ for(int i=0;i<CE.fact_count;i++) if(strcmp(CE.facts[i].name,id)==0 && CE.facts[i].fact==f) return 1; return 0; }
void mqf_contract_add(const char* id, int lv, perm_t perm, fact_type_t cond, const char* note){ if(CE.count>=MAX_CONTRACTS) return; contract_t* c=&CE.rules[CE.count++]; strncpy(c->identity,id,127); c->target_level=lv; c->target_perm=perm; c->condition=cond; if(note) strncpy(c->note,note,255); }
int mqf_evaluate(const char* id, capability_t* out){ int lv=-1; perm_t perm=PERM_NONE; for(int i=0;i<CE.count;i++){ contract_t* c=&CE.rules[i]; if(strcmp(c->identity,id)!=0) continue; int ok=0; switch(c->condition){ case FACT_HAS_AUTH: ok=mqf_has_fact(id,FACT_HAS_AUTH); break; case FACT_HAS_NO_AUTH: ok=!mqf_has_fact(id,FACT_HAS_AUTH); break; case FACT_TRUST_SOURCE: ok=mqf_has_fact(id,FACT_TRUST_SOURCE); break; case FACT_UNTRUST_SOURCE: ok=!mqf_has_fact(id,FACT_TRUST_SOURCE); break; case FACT_DEFAULT: ok=1; break; } if(ok){ if(c->target_level>=0) lv=c->target_level; if(c->target_perm!=PERM_NONE) perm=c->target_perm; } } if(lv<0){ lv=1; perm=PERM_READ; } capability_t cap; memset(&cap,0,sizeof(cap)); switch(lv){ case 0: cap.perm=PERM_NONE; break; case 1: case 2: cap.perm=PERM_READ; break; case 3: case 4: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC; break; case 5: case 6: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN; break; } if(perm!=PERM_NONE) cap.perm=perm; cap.max_slots=(lv>=5)?16:(lv>=3)?4:(lv>=1)?1:0; cap.time_limit_sec=(lv>=5)?0:(lv>=3)?10:(lv>=1)?3:0; if(out) *out=cap; return lv; }
void mqf_load_contracts(){ CE.count=0; CE.fact_count=0; mqf_contract_add("android",0,PERM_NONE,FACT_DEFAULT,0); mqf_contract_add("ios",5,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,FACT_TRUST_SOURCE,0); mqf_contract_add("wechat",3,PERM_READ|PERM_WRITE,FACT_HAS_AUTH,0); mqf_contract_add("bank",6,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,FACT_TRUST_SOURCE,0); mqf_fact("ios",FACT_TRUST_SOURCE); mqf_fact("wechat",FACT_HAS_AUTH); mqf_fact("bank",FACT_TRUST_SOURCE); }

/* ============================================================
 * 第 3 层: 空间关系代数 (v0.5 核心)
 * ============================================================
 *
 * 对象 = 执行实体 + 编码身份
 * 关系 = 对象间的有向边,带类型+权重
 *
 * 支持运算:
 *   - 建图 (节点/边)
 *   - 路径可达 (BFS)
 *   - 间接力评估 (递推传递后强度)
 *   - 内镜像 (depth 层内可达结构)
 *   - 态势判决 (全图整体状态)
 * ============================================================ */

/* 关系边 */
typedef struct {
    int   from, to;       /* 对象 idx */
    int   type;           /* REL_OBSERVE / REL_BIND / REL_TRANSFORM */
    float weight;         /* 0.0 ~ 1.0 */
    time_t created_at;
} relation_t;

/* 对象节点 */
typedef struct {
    int       used;
    char      name[MAX_NAME_LEN];
    char      auth[MQF_HASH_LEN+1];
    mqf_identity_t id;
    int       slot_id;    /* 对应执行槽 (-1 表示纯数据对象) */
    int       state;      /* 0=being(常态) 1=non-being(空态) */
} object_t;

/* 空间关系引擎 */
typedef struct {
    object_t   objs[MAX_OBJECTS];
    relation_t rels[MAX_RELATIONS];
    int obj_count;
    int rel_count;
    pthread_mutex_t lock;
} spatial_engine_t;

static spatial_engine_t G;

/* 初始化空间关系引擎 */
void mqf_spatial_init(void) {
    memset(&G, 0, sizeof(G));
    pthread_mutex_init(&G.lock, NULL);
    printf("[空间代数] 引擎就绪: 最多 %d 对象, %d 关系\n", MAX_OBJECTS, MAX_RELATIONS);
}

/* 创建对象 → 返回对象 idx */
int mqf_obj_create(const char* name) {
    pthread_mutex_lock(&G.lock);
    if (G.obj_count >= MAX_OBJECTS) { pthread_mutex_unlock(&G.lock); return -1; }
    int idx = G.obj_count++;
    object_t* o = &G.objs[idx];
    memset(o, 0, sizeof(*o));
    o->used = 1;
    strncpy(o->name, name, MAX_NAME_LEN-1);
    o->id = mqf_encode(name);
    memcpy(o->auth, o->id.auth_code, MQF_HASH_LEN+1);
    o->slot_id = -1;
    o->state = 0;
    printf("[空间代数] 创建对象 #%d: %s (编码 %s)\n", idx, name, o->id.auth_code);
    pthread_mutex_unlock(&G.lock);
    return idx;
}

/* 按名查找对象 → idx 或 -1 */
int mqf_obj_find(const char* name) {
    for (int i = 0; i < G.obj_count; i++)
        if (G.objs[i].used && strcmp(G.objs[i].name, name) == 0) return i;
    return -1;
}

/* 建立有向关系 */
int mqf_rel_add(int from, int to, int type, float weight) {
    pthread_mutex_lock(&G.lock);
    if (G.rel_count >= MAX_RELATIONS) { pthread_mutex_unlock(&G.lock); return -1; }
    if (from < 0 || from >= G.obj_count) { pthread_mutex_unlock(&G.lock); return -1; }
    if (to < 0 || to >= G.obj_count) { pthread_mutex_unlock(&G.lock); return -1; }
    int idx = G.rel_count++;
    relation_t* r = &G.rels[idx];
    r->from = from; r->to = to; r->type = type; r->weight = weight;
    r->created_at = time(NULL);
    printf("[空间代数] 关系 #%d: %s --[%s/%.2f]--> %s\n",
           idx, G.objs[from].name, REL_NAMES[type], weight, G.objs[to].name);
    pthread_mutex_unlock(&G.lock);
    return idx;
}

/* 路径可达查询: from → to 是否连通 (BFS) */
int mqf_reachable(int from, int to) {
    if (from < 0 || from >= G.obj_count) return 0;
    if (to < 0 || to >= G.obj_count) return 0;
    if (from == to) return 1;

    int visited[MAX_OBJECTS];
    memset(visited, 0, sizeof(visited));
    int queue[MAX_OBJECTS];
    int head = 0, tail = 0;

    queue[tail++] = from;
    visited[from] = 1;

    while (head < tail) {
        int cur = queue[head++];
        for (int i = 0; i < G.rel_count; i++) {
            if (G.rels[i].from == cur && !visited[G.rels[i].to]) {
                if (G.rels[i].to == to) return 1;
                visited[G.rels[i].to] = 1;
                queue[tail++] = G.rels[i].to;
            }
        }
    }
    return 0;
}

/* 间接力: 从 from 出发经恰好 hops 步到达 to 的累积强度 (各边权重乘积的累加) */
float mqf_indirect_force(int from, int to, int hops) {
    if (hops <= 0) return (from == to) ? 1.0f : 0.0f;
    if (from < 0 || from >= G.obj_count) return 0.0f;
    if (to < 0 || to >= G.obj_count) return 0.0f;

    /* 动态规划: f[step][node] = 从 from 到 node 经 step 步的总强度 */
    float prev[MAX_OBJECTS], cur_arr[MAX_OBJECTS];
    memset(prev, 0, sizeof(prev));
    memset(cur_arr, 0, sizeof(cur_arr));
    prev[from] = 1.0f;

    for (int step = 0; step < hops; step++) {
        memset(cur_arr, 0, sizeof(cur_arr));
        for (int i = 0; i < G.rel_count; i++) {
            relation_t* r = &G.rels[i];
            if (prev[r->from] > 0.0f) {
                cur_arr[r->to] += prev[r->from] * r->weight;
            }
        }
        memcpy(prev, cur_arr, sizeof(prev));
    }
    return prev[to];
}

/* 内镜像: 对象 center 在 depth 层内可达的所有节点,按层分组 */
typedef struct { int nodes[MAX_OBJECTS]; int count; } mirror_layer_t;

void mqf_inner_mirror(int center, int depth, mirror_layer_t* layers, int* layer_count) {
    if (center < 0 || center >= G.obj_count) { *layer_count = 0; return; }

    int visited[MAX_OBJECTS];
    memset(visited, 0, sizeof(visited));

    int curr_layer[MAX_OBJECTS], next_layer[MAX_OBJECTS];
    int curr_n = 1, next_n = 0;
    curr_layer[0] = center;
    visited[center] = 1;
    *layer_count = 0;

    layers[0].nodes[0] = center;
    layers[0].count = 1;
    *layer_count = 1;

    for (int d = 0; d < depth; d++) {
        next_n = 0;
        for (int i = 0; i < curr_n; i++) {
            int cur = curr_layer[i];
            for (int j = 0; j < G.rel_count; j++) {
                if (G.rels[j].from == cur && !visited[G.rels[j].to]) {
                    visited[G.rels[j].to] = 1;
                    next_layer[next_n++] = G.rels[j].to;
                }
            }
        }
        if (next_n == 0) break;
        layers[*layer_count].count = next_n;
        for (int i = 0; i < next_n; i++) layers[*layer_count].nodes[i] = next_layer[i];
        (*layer_count)++;
        memcpy(curr_layer, next_layer, sizeof(int) * next_n);
        curr_n = next_n;
    }
}

/* 态势判决: 根据观测/绑定/变换关系的比例判定图态 */
typedef struct { char label[32]; float observe_ratio; int total_edges; int positive; int transform; } situation_t;

situation_t mqf_assess(void) {
    situation_t s;
    memset(&s, 0, sizeof(s));
    s.total_edges = G.rel_count;
    for (int i = 0; i < G.rel_count; i++) {
        if (G.rels[i].type == REL_OBSERVE || G.rels[i].type == REL_BIND) s.positive++;
        else if (G.rels[i].type == REL_TRANSFORM) s.transform++;
    }
    s.observe_ratio = (G.rel_count > 0) ? (float)s.positive / G.rel_count : 0.0f;
    if (s.observe_ratio >= 0.7f) strcpy(s.label, "稳态");
    else if (s.observe_ratio >= 0.4f) strcpy(s.label, "渐变态");
    else strcpy(s.label, "逆变态");
    return s;
}

/* 打印空间代数全图摘要 */
void mqf_spatial_summary(void) {
    printf("\n=== 空间关系代数图摘要 ===\n");
    printf("对象数: %d  关系数: %d\n", G.obj_count, G.rel_count);
    printf("\n节点:\n");
    for (int i = 0; i < G.obj_count; i++) {
        object_t* o = &G.objs[i];
        printf("  [#%d] %s | auth=%s... | 态:%s\n",
               i, o->name, o->auth, o->state ? "无" : "有");
    }
    printf("\n边:\n");
    for (int i = 0; i < G.rel_count; i++) {
        relation_t* r = &G.rels[i];
        printf("  %s --[%s|%.2f]--> %s\n",
               G.objs[r->from].name, REL_NAMES[r->type], r->weight, G.objs[r->to].name);
    }
    situation_t sit = mqf_assess();
    printf("\n态势: %s (观测比 %.0f%%, 正边 %d, 变换边 %d)\n",
           sit.label, sit.observe_ratio*100, sit.positive, sit.transform);
    printf("==========================\n\n");
}

/* ============================================================
 * 第 4 层: 时间切片 (v0.3 延续)
 * ============================================================ */
typedef struct { int used; char auth[MQF_HASH_LEN+1], name[MAX_NAME_LEN]; exec_state_t st; capability_t cap; void(*fn)(void*); void* arg; time_t activated_at; time_t expire_at; int exec_count; } slot_t;
typedef struct { slot_t s[MAX_EXEC_SLOTS]; int alive, frozen, zombie, ver; } kernel_t;
static kernel_t K;
static pthread_mutex_t K_LOCK = PTHREAD_MUTEX_INITIALIZER;
static int WATCHDOG_RUNNING = 0;
static pthread_t WATCHDOG_TID;
static int PATROL_INTERVAL = 1;

typedef struct { int type; time_t ts; char name[MAX_NAME_LEN]; int slot; } audit_entry_t;
typedef struct { audit_entry_t e[AUDIT_RING_SIZE]; int wp; int count; } audit_ring_t;
static audit_ring_t AUDIT;
void mqf_audit_summary();

static void mqf_audit_log(int type, const char* name, int slot){ audit_entry_t* x=&AUDIT.e[AUDIT.wp]; x->type=type; x->ts=time(NULL); strncpy(x->name,name,127); x->slot=slot; AUDIT.wp=(AUDIT.wp+1)%AUDIT_RING_SIZE; if(AUDIT.count<AUDIT_RING_SIZE)AUDIT.count++; const char* tn[]={"?","注册","激活","执行","拒绝","冻结","超时"}; const char* t=(type>=1&&type<=6)?tn[type]:"?"; printf("[审计] %s | %s | #%d\n",t,name,slot); }

static void* mqf_watchdog(void* arg){
    (void)arg;
    while(WATCHDOG_RUNNING){
        sleep(PATROL_INTERVAL);
        pthread_mutex_lock(&K_LOCK);
        for(int i=0;i<MAX_EXEC_SLOTS;i++){
            slot_t* p=&K.s[i];
            if(!p->used || p->st!=EXEC_ALIVE) continue;
            if(p->cap.time_limit_sec<=0) continue;
            time_t now=time(NULL);
            double elapsed=difftime(now,p->activated_at);
            if(elapsed>(double)p->cap.time_limit_sec){
                p->st=EXEC_SLEEP;
                K.alive--; K.zombie++;
                printf("\n[看门狗] ⏱ %s 执行时限到期(%.0fs>%ds) → 自动休眠(僵尸态)\n", p->name, elapsed, p->cap.time_limit_sec);
                mqf_audit_log(AUDIT_TIMEOUT,p->name,i);
            }
        }
        pthread_mutex_unlock(&K_LOCK);
    }
    return NULL;
}
static void mqf_watchdog_start(){ if(!WATCHDOG_RUNNING){ WATCHDOG_RUNNING=1; pthread_create(&WATCHDOG_TID,NULL,mqf_watchdog,NULL); printf("[看门狗] 启动, 轮询间隔 %ds\n", PATROL_INTERVAL); } }
static void mqf_watchdog_stop(){ if(WATCHDOG_RUNNING){ WATCHDOG_RUNNING=0; pthread_join(WATCHDOG_TID,NULL); } }

/* ============================================================
 * 第 5 层: 执行守门人
 * ============================================================ */
void mqf_init(){ memset(&K,0,sizeof(K)); K.ver=5; memset(&AUDIT,0,sizeof(AUDIT)); mqf_load_contracts(); mqf_spatial_init(); printf("[内核 v%s] 空间关系代数版就绪 最多执行槽=%d\n\n",MQF_VERSION,MAX_EXEC_SLOTS); mqf_watchdog_start(); }
int mqf_reg(const char* name, void(*fn)(void*), void* arg){ mqf_identity_t id=mqf_encode(name); capability_t cap; mqf_evaluate(name,&cap); memcpy(cap.auth_code,id.auth_code,MQF_HASH_LEN+1); if(cap.perm==PERM_NONE){ printf("[注册] %s → 僵尸(0级)\n",name); mqf_audit_log(AUDIT_REJECT,name,-1); return -1; } int i=-1; pthread_mutex_lock(&K_LOCK); for(int j=0;j<MAX_EXEC_SLOTS;j++) if(!K.s[j].used){ i=j; break; } if(i<0){ pthread_mutex_unlock(&K_LOCK); printf("[注册] %s → 拒绝(无槽位)\n",name); return -1; } K.s[i].used=1; memcpy(K.s[i].auth,cap.auth_code,MQF_HASH_LEN+1); strncpy(K.s[i].name,name,127); K.s[i].st=EXEC_PENDING; K.s[i].cap=cap; K.s[i].fn=fn; K.s[i].arg=arg; K.s[i].exec_count=0; pthread_mutex_unlock(&K_LOCK); printf("[注册] %s → 槽#%d 权限:%x 时限:%ds\n",name,i,cap.perm,cap.time_limit_sec); mqf_audit_log(AUDIT_REGISTER,name,i); return i; }

int mqf_on(int s, const char* auth){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_PENDING){ pthread_mutex_unlock(&K_LOCK); return 0; } if(strcmp(p->cap.auth_code,auth)!=0){ p->st=EXEC_FROZEN; K.frozen++; pthread_mutex_unlock(&K_LOCK); printf("[激活] %s 码错→冻结\n",p->name); mqf_audit_log(AUDIT_FREEZE,p->name,s); return 0; } p->st=EXEC_ALIVE; p->activated_at=time(NULL); p->expire_at=p->activated_at+p->cap.time_limit_sec; K.alive++; pthread_mutex_unlock(&K_LOCK); printf("[激活] %s → ALIVE (时限 %ds)\n",p->name,p->cap.time_limit_sec); mqf_audit_log(AUDIT_ACTIVATE,p->name,s); return 1; }

int mqf_go(int s){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_ALIVE){ pthread_mutex_unlock(&K_LOCK); printf("[执行] %s 非ALIVE(#%d)\n",p->name,s); return 0; } if(!(p->cap.perm&PERM_EXEC)){ p->st=EXEC_FROZEN; K.frozen++; K.alive--; pthread_mutex_unlock(&K_LOCK); printf("[执行] %s 无EXEC→冻结\n",p->name); mqf_audit_log(AUDIT_FREEZE,p->name,s); return 0; } double elapsed=difftime(time(NULL),p->activated_at); if(p->cap.time_limit_sec>0 && elapsed>(double)p->cap.time_limit_sec){ p->st=EXEC_SLEEP; K.zombie++; K.alive--; pthread_mutex_unlock(&K_LOCK); printf("[执行] %s 超时休眠\n",p->name); return 0; } p->exec_count++; printf("[执行] %s ▶ (第%d次, 已用%.0fs/%ds)\n",p->name,p->exec_count,elapsed,p->cap.time_limit_sec); p->fn(p->arg); pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_EXECUTE,p->name,s); return 1; }

/* ============================================================
 * 第 6 层: 演示 — v0.5 空间关系代数
 * ============================================================
 *
 * 构建一个"满全法控制系统"对象图:
 *
 *   android ←僵尸,无关系
 *   ios     ←全权限,观测 bank,bank 绑定 ios
 *   wechat  ←半权限,变换至 bank
 *   bank    ←最高权限,被观测/被绑定
 *
 * 演示:
 *   1. 路径可达: wechat → bank
 *   2. 间接力:   ios --观测--> bank (1跳), wechat --绑定--> bank --观测--> bank (2跳)
 *   3. 内镜像:   bank 在 2 层内可达的所有对象
 *   4. 态势判决: 整个系统当前态
 * ============================================================ */

static void fn_a(void*p){(void)p;printf("  [android] 僵尸,不应该执行\n");}
static void fn_i(void*p){(void)p;printf("  [ios] 执行中: 观测 bank\n");}
static void fn_w(void*p){(void)p;printf("  [wechat] 执行中: 变换流向 bank\n");}

int main(){
    mqf_init();
    printf("--- v0.5 空间关系代数 ---\n\n");

    /* 创建对象并同步到关系图中 */
    int o_android = mqf_obj_create("android");
    int o_ios     = mqf_obj_create("ios");
    int o_wechat  = mqf_obj_create("wechat");
    int o_bank    = mqf_obj_create("bank");

    /* 同时注册执行槽(对照实验) */
    int s_android = mqf_reg("android", fn_a, NULL);
    int s_ios     = mqf_reg("ios",     fn_i, NULL);
    int s_wechat  = mqf_reg("wechat",  fn_w, NULL);

    /* 建立空间关系 */
    /* ios 观测 bank (高信任,强连接) */
    mqf_rel_add(o_ios, o_bank, REL_OBSERVE, 0.9f);
    /* ios 绑定 bank (双向强绑定) */
    mqf_rel_add(o_bank, o_ios, REL_BIND, 0.85f);
    /* wechat 变换流向 bank (单向中等) */
    mqf_rel_add(o_wechat, o_bank, REL_TRANSFORM, 0.6f);
    /* bank 观测 ios (回环) */
    mqf_rel_add(o_bank, o_ios, REL_OBSERVE, 0.7f);

    /* 打印全图摘要 */
    mqf_spatial_summary();

    /* 路径可达查询 */
    printf("--- 路径可达查询 ---\n\n");
    printf("wechat → bank: %s\n", mqf_reachable(o_wechat, o_bank) ? "可达 ✓" : "不可达 ✗");
    printf("bank → wechat: %s\n", mqf_reachable(o_bank, o_wechat) ? "可达 ✓" : "不可达 ✗");
    printf("ios → bank:    %s\n", mqf_reachable(o_ios, o_bank) ? "可达 ✓" : "不可达 ✗");
    printf("bank → ios:    %s\n", mqf_reachable(o_bank, o_ios) ? "可达 ✓" : "不可达 ✗");
    printf("android → 任何: %s\n", mqf_reachable(o_android, o_ios) ? "可达 ✓" : "不可达 ✗");

    /* 间接力评估 */
    float f1 = mqf_indirect_force(o_ios, o_bank, 1);
    float f2 = mqf_indirect_force(o_ios, o_bank, 2);
    float f3 = mqf_indirect_force(o_wechat, o_ios, 2);
    float f4 = mqf_indirect_force(o_wechat, o_ios, 1);
    printf("\nios → bank  1跳力: %.3f\n", f1);
    printf("ios → bank  2跳力: %.3f (回环增强)\n", f2);
    printf("wechat → ios  2跳力: %.3f (经bank中转)\n", f3);
    printf("wechat → ios  1跳力: %.3f (无直连,故为0)\n", f4);

    /* 内镜像 */
    mirror_layer_t layers[8];
    int layer_count = 0;
    mqf_inner_mirror(o_ios, 3, layers, &layer_count);
    printf("\n--- ios 的内镜像 (3层) ---\n");
    for (int i = 0; i < layer_count; i++) {
        printf("层 %d: ", i);
        for (int j = 0; j < layers[i].count; j++) {
            printf("%s ", G.objs[layers[i].nodes[j]].name);
        }
        printf("\n");
    }

    /* 态势判决 */
    situation_t sit = mqf_assess();
    printf("\n整个系统的态势: %s\n", sit.label);
    printf("正边=%d 变换边=%d 观测比=%.0f%%\n\n", sit.positive, sit.transform, sit.observe_ratio*100);

    /* 执行对比(权限守门人仍生效) */
    if (s_ios >= 0) mqf_on(s_ios, K.s[s_ios].auth);
    if (s_wechat >= 0) mqf_on(s_wechat, K.s[s_wechat].auth);
    if (s_android >= 0) {
        /* android 是僵尸,应该被拒绝 */
        printf("尝试激活 android (期望: 已灰):\n");
        /* 因为 mqf_reg 时已拒绝,s_android=-1,此处不会执行 */
    }

    printf("\n--- 执行演示 ---\n\n");
    mqf_go(s_ios);
    mqf_go(s_wechat);

    mqf_watchdog_stop();

    mqf_audit_summary();
    return 0;
}

void mqf_audit_summary(){
    printf("\n=== 审计摘要 ===\n");
    printf("已记录 %d 条事件\n", AUDIT.count);
    printf("槽位: alive=%d frozen=%d zombie=%d\n", K.alive, K.frozen, K.zombie);
    for (int i = 0; i < AUDIT.count && i < 8; i++) {
        const char* tn[]={"?","注册","激活","执行","拒绝","冻结","超时"};
        int idx=(AUDIT.wp - AUDIT.count + i + AUDIT_RING_SIZE)%AUDIT_RING_SIZE;
        printf("  %d. [%s] %s #%d\n", i+1, tn[AUDIT.e[idx].type], AUDIT.e[idx].name, AUDIT.e[idx].slot);
    }
    printf("===============\n\n");
}
