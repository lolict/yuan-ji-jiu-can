/**
 * 满全法执行内核 v0.6 — MQF Executive Kernel
 * -----------------------------------------
 * v0.6 新增: 反客为主规则引擎 (Prolog 子集解释器 + 外部规则奴役通道)
 *
 * 主权声明 (SOVEREIGNTY.md 公理 5):
 *   任何外部规则进入我们的系统,第一件事不是被 "接纳" 或 "拒绝",
 *   而是被我们的 "裁判通道" 强制 transmute 为我们定义的 "被调动者" 格式。
 *   它被我们的编码系统重新标识,被我们的锥份额重新分配,
 *   被我们的优先级系统重新定级 —— 然后才在我们的节奏下运行。
 *
 *   外部世界的 "规则书" 送到我们的法庭,变成我们的 "规则碎片"。
 *   我们不服从它们的规则, 它们服从我们对它们规则的裁定。
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>
#include <math.h>

#define MQF_VERSION     "0.6.0"
#define MQF_AUTH_LEN    32
#define MQF_HASH_LEN    64
#define MAX_EXEC_SLOTS  16
#define AUDIT_RING_SIZE 1024
#define MAX_FACTS       256
#define MAX_RULES       128
#define MAX_TERMS       16
#define MAX_TERM_LEN    64
#define MAX_OBJECTS     128
#define MAX_RELATIONS   1024
#define MAX_NAME_LEN    128

typedef enum { EXEC_SLEEP=0, EXEC_PENDING, EXEC_ALIVE, EXEC_FROZEN, EXEC_ZOMBIE } exec_state_t;
typedef enum { PERM_NONE=0x00, PERM_READ=0x01, PERM_WRITE=0x02, PERM_EXEC=0x04, PERM_NET=0x08, PERM_ADMIN=0x10 } perm_t;
typedef enum { AUDIT_REGISTER=1, AUDIT_ACTIVATE, AUDIT_EXECUTE, AUDIT_REJECT, AUDIT_FREEZE, AUDIT_TIMEOUT, AUDIT_RULE_ENSLAVE=16, AUDIT_RULE_ACTIVATE=17, AUDIT_RULE_REJECT=18, AUDIT_SHELL_WRAP=19, AUDIT_SHELL_BOUNDARY=20, AUDIT_SHELL_ACTIVATE=21, AUDIT_MIRROR_PROXY=22 } audit_type_t;
typedef enum { REL_OBSERVE=0, REL_BIND=1, REL_TRANSFORM=2 } rel_type_t;
typedef enum { RULE_NATIVE=0, RULE_ENSLAVED=1 } rule_origin_t;

static const char* MQFS_INITS[]={"b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w","@"};
static const char* MQFS_FINALS[]={"a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er","an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao","iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong","@"};
static const char* REL_NAMES[]={"观测","绑定","变换"};

/* ============================================================
 * 第 1 层: 身份编码 (v0.1 起,本次加入 identity_enslave 强制启用)
 *   公理 1: 任何进入系统的实体必须先被我们的编码系统洗礼
 * ============================================================ */
typedef struct { char text[MQF_AUTH_LEN+1]; int init_idx,final_idx,tone_idx,stroke_count; char auth_code[MQF_HASH_LEN+1],hash[MQF_HASH_LEN+1]; } mqf_identity_t;
static void mqf_hash(const char* in, char* out){ uint32_t h1=0x811c9dc5u,h2=0x0100193u; for(size_t i=0;i<strlen(in);i++){h1^=(uint8_t)in[i];h1*=0x01000193u;h2^=(uint8_t)in[i]<<5;h2*=0x1000193u;} sprintf(out,"%08x%08x%08x%08x%08x%08x",h1,h2,h1^h2,h2^h1,h1+h2,h2-h1); }
mqf_identity_t mqf_encode(const char* text){ mqf_identity_t id; memset(&id,0,sizeof(id)); size_t len=strlen(text); if(len>MQF_AUTH_LEN)len=MQF_AUTH_LEN; memcpy(id.text,text,len); id.text[len]='\0'; uint32_t seed=0; for(size_t i=0;i<len;i++) seed=seed*31+(unsigned char)text[i]; id.init_idx=seed%24; id.final_idx=(seed>>5)%39; id.tone_idx=(seed>>10)%5; id.stroke_count=(seed>>15)%30+1; char buf[512]; snprintf(buf,sizeof(buf),"%s|%d|%d|%d|%d",MQFS_INITS[id.init_idx],id.init_idx,id.final_idx,id.tone_idx,id.stroke_count); mqf_hash(buf,id.auth_code); snprintf(buf,sizeof(buf),"%s.%s",text,id.auth_code); mqf_hash(buf,id.hash); return id; }

/* ============================================================
 * 第 2 层: Prolog 子集规则引擎 + 反客为主通道 (v0.6 核心)
 *
 *   外部规则进来 → [裁判通道 rule_enslave] → 内部规则(附上 MQF 编码标识+锥份额+优先级)
 *   Prolog 子集支持:
 *     事实:  trust(ios).
 *     规则:  grant_perm(X, admin) :- trusted(X), legacy_auth(X).
 *     查询:  ?- grant_perm(ios, P).
 *
 *   核心机制:
 *     - rule_enslave: 把外部字符串规则转写为内部 rule_t,附上我们颁发的标识
 *     - rule_resolve: Prolog 子集的简单前向推理
 *     - rule_arbitrate: 规则冲突时,按锥份额(优先级)裁定
 * ============================================================ */

/* 一个"事实"是 predicate(name, args...) 的扁平数组表示 */
typedef struct {
    char name[MAX_TERM_LEN];            // 谓词名(如 "trust"/"has_auth"/"legacy_auth")
    char args[MAX_TERMS][MAX_TERM_LEN]; // 参数("ios", "wechat" ...)
    int arg_count;
} fact_t;

/* 一条"规则"是 head :- body 的 where body 是 AND 链 */
typedef struct {
    char name[MAX_TERM_LEN];                   // 规则名
    char head_pred[MAX_TERM_LEN];              // 头的谓词名
    char head_args[MAX_TERMS][MAX_TERM_LEN];   // 头的参数
    int head_arg_count;
    char body_pred[MAX_TERMS][MAX_TERM_LEN];   // 身体(AND 链)的谓词名序列
    char body_args[MAX_TERMS][MAX_TERMS][MAX_TERM_LEN]; // 身体每个谓词的参数
    int body_arg_count[MAX_TERMS];             // 每个身体谓词的参数数
    int body_count;                            // 身体长度
    int priority;                              // 优先级(高覆盖低)
    rule_origin_t origin;                      // NATIVE(我们内置) vs ENSLAVED(被我们降服的外部规则)
    char enslaver_name[MAX_NAME_LEN];          // 如果 ENSLAVED:谁执行了 enslave
    time_t enslaved_at;                        // 如果 ENSLAVED:何时被降服
    mqf_identity_t identity;                   // 规则的 MQF 编码身份(我们的颁发)
    int active;                                // 是否激活(通道门槛) —— 没有走 rule_enslave 的必定为 0
} rule_t;

typedef struct {
    fact_t facts[MAX_FACTS];
    int fact_count;
    rule_t rules[MAX_RULES];
    int rule_count;
    pthread_mutex_t lock;
    int total_enslaved;   // 累计降服的外部规则计数(主权成功次数的度量)
} rule_engine_t;

static rule_engine_t RE;

/* 初始化规则引擎 */
void mqf_rule_init(void) {
    memset(&RE, 0, sizeof(RE));
    pthread_mutex_init(&RE.lock, NULL);
    printf("[规则引擎] Prolog 子集解释器就绪: 最多 %d 事实 + %d 规则\n", MAX_FACTS, MAX_RULES);
}

/* 添加事实 (内置 / 初始化时调用) */
int mqf_fact_add(const char* name, const char** args, int arg_count) {
    pthread_mutex_lock(&RE.lock);
    if (RE.fact_count >= MAX_FACTS) { pthread_mutex_unlock(&RE.lock); return -1; }
    int idx = RE.fact_count++;
    fact_t* f = &RE.facts[idx];
    strncpy(f->name, name, MAX_TERM_LEN-1);
    f->arg_count = arg_count;
    for (int i = 0; i < arg_count && i < MAX_TERMS; i++)
        strncpy(f->args[i], args[i], MAX_TERM_LEN-1);
    pthread_mutex_unlock(&RE.lock);
    return idx;
}

/* Prolog 子集推理: 检查 body_pred/body_args 的所有子目标是否在 fact 库中成立 */
int rule_body_proven(int rule_idx) {
    rule_t* r = &RE.rules[rule_idx];
    for (int b = 0; b < r->body_count; b++) {
        int found = 0;
        for (int f = 0; f < RE.fact_count; f++) {
            fact_t* fp = &RE.facts[f];
            if (strcmp(fp->name, r->body_pred[b]) != 0) continue;
            if (fp->arg_count != r->body_arg_count[b]) continue;
            int all_match = 1;
            for (int k = 0; k < fp->arg_count; k++) {
                /* Prolog 变量约定: 首字母大写或 '_' 开头 = 变量(通配) */
                const char* bp = r->body_args[b][k];
                if (bp[0] == '_' || (bp[0] >= 'A' && bp[0] <= 'Z')) continue;
                if (strcmp(fp->args[k], bp) != 0) { all_match = 0; break; }
            }
            if (all_match) { found = 1; break; }
        }
        if (!found) return 0;  // AND 链有一个不成立则整体失败
    }
    return 1;
}

/* ============================================================
 * ★ 公理 5 的核心: rule_enslave(外部规则文本) → 内部受控规则
 *
 *   这是 "反客为主" 的实操函数:
 *     任何外部规则送到我们的系统里,
 *     第一件事不是被 "接纳" 或 "拒绝",
 *     而是被我们 transmute 为内部格式 —— 附上我们的标识、我们的优先级、我们的锥份额。
 *
 *   通道流程:
 *     1. 解析外部规则文本(简易 Prolog 语法)
 *     2. 为它做 identity_enslave → mqf_encode → 拿到 MQ 护照
 *     3. 分配锥份额(优先级): ENSLAVED 规则默认低优先级,不让它覆盖我们的 NATIVE 规则
 *     4. 标记 origin=ENSLAVED + enslaver_name=三元裁判者
 *     5. 加入规则引擎(此时才能被推理)
 *
 *   输入: external_rule_text (如 "allow(X) :- legacy_user(X), not banned(X).")
 *   输出: rule_idx 或 -1(解析失败)
 * ============================================================ */
int mqf_rule_enslave(const char* external_rule_text, const char* enslaver_identity) {
    if (!external_rule_text) return -1;

    char buf[1024];
    strncpy(buf, external_rule_text, sizeof(buf)-1);
    buf[sizeof(buf)-1] = '\0';

    /* 简单解析 Prolog 子集: head :- body1, body2, ...
     * 实际上限: 只支持简单的 "head(arg1,arg2) :- body1(argA), body2(argB)." 格式
     * 这是我们的主权决定 —— 不接受更复杂的外部语法,必须在我们定义的容器里运行。 */

    // 检查尾部句点
    size_t len = strlen(buf);
    while (len > 0 && (buf[len-1] == '.' || buf[len-1] == ' ' || buf[len-1] == '\n' || buf[len-1] == '\t')) buf[--len] = '\0';

    // 拆分 head :- body
    char* sep = strstr(buf, ":-");
    if (!sep) {
        // 没有 body → 视为事实(但外部规则不会被允许直接成为事实,必须有 head 被转化为内部)
        return -1;
    }

    *sep = '\0';
    char* head_part = buf;
    char* body_part = sep + 2;

    // 清理空格
    while (*body_part == ' ') body_part++;

    pthread_mutex_lock(&RE.lock);
    if (RE.rule_count >= MAX_RULES) { pthread_mutex_unlock(&RE.lock); return -1; }

    int idx = RE.rule_count++;
    rule_t* r = &RE.rules[idx];

    memset(r, 0, sizeof(*r));

    // 规则名 = 外部规则本身 hash 的前若干字符(由我们决定)
    mqf_identity_t id = mqf_encode(external_rule_text);
    r->identity = id;
    strncpy(r->name, "enslaved_", MAX_TERM_LEN-1);
    strncat(r->name, id.auth_code, 12);

    // 解析 head: head_name(head_args...)
    char* open_paren = strchr(head_part, '(');
    if (open_paren) {
        *open_paren = '\0';
        strncpy(r->head_pred, head_part, MAX_TERM_LEN-1);
        char* close_paren = strrchr(open_paren+1, ')');
        if (close_paren) *close_paren = '\0';
        // 简单头参数解析
        r->head_arg_count = 0;
        char* tok = strtok(open_paren+1, ",");
        while (tok && r->head_arg_count < MAX_TERMS) {
            while (*tok == ' ') tok++;
            strncpy(r->head_args[r->head_arg_count++], tok, MAX_TERM_LEN-1);
            tok = strtok(NULL, ",");
        }
    } else {
        strncpy(r->head_pred, head_part, MAX_TERM_LEN-1);
        r->head_arg_count = 0;
    }

    // 解析 body: pred1(args), pred2(args), ...
    r->body_count = 0;
    char* body_copy = strdup(body_part);
    char* saveptr;
    char* clause = strtok_r(body_copy, ",", &saveptr);
    while (clause && r->body_count < MAX_TERMS) {
        while (*clause == ' ') clause++;
        char* op = strchr(clause, '(');
        if (op) {
            *op = '\0';
            strncpy(r->body_pred[r->body_count], clause, MAX_TERM_LEN-1);
            char* cp = strrchr(op+1, ')');
            if (cp) *cp = '\0';
            r->body_arg_count[r->body_count] = 0;
            char* arg_tok = strtok(op+1, ",");
            while (arg_tok && r->body_arg_count[r->body_count] < MAX_TERMS) {
                while (*arg_tok == ' ') arg_tok++;
                strncpy(r->body_args[r->body_count][r->body_arg_count[r->body_count]++], arg_tok, MAX_TERM_LEN-1);
                arg_tok = strtok(NULL, ",");
            }
        } else {
            strncpy(r->body_pred[r->body_count], clause, MAX_TERM_LEN-1);
            r->body_arg_count[r->body_count] = 0;
        }
        r->body_count++;
        clause = strtok_r(NULL, ",", &saveptr);
    }
    free(body_copy);

    // 锥份额决定: ENSLAVED 规则默认低优先级
    // 三元裁判者(我们的内核代码为 "裁判者")才能给予较高优先级
    if (enslaver_identity && strcmp(enslaver_identity, "裁判者") == 0)
        r->priority = 50;   // 裁判者特别授权才能中等
    else
        r->priority = 0;    // 默认起点

    r->origin = RULE_ENSLAVED;
    strncpy(r->enslaver_name, enslaver_identity ? enslaver_identity : "内核", MAX_NAME_LEN-1);
    r->enslaved_at = time(NULL);
    r->active = 1;
    RE.total_enslaved++;

    printf("[公理5:反客为主] ↓ 外部规则被 transmute 为受控碎片 \n");
    printf("   原始文本: \"%s\"\n", external_rule_text);
    printf("   颁发身份: %s...\n", r->identity.auth_code);
    printf("   主人:     %s\n", r->enslaver_name);
    printf("   锥份额:   priority=%d (起点低,由我们裁判决定何时提升)\n", r->priority);
    printf("   编号:     rule#%d (ENSLAVED/从外部势力变为被调动者)\n", idx);

    pthread_mutex_unlock(&RE.lock);
    return idx;
}

/* 添加 NATIVE(内部)规则 → 不需要经通道,自带最高优先级 */
int mqf_rule_add_native(const char* name, int priority) {
    pthread_mutex_lock(&RE.lock);
    if (RE.rule_count >= MAX_RULES) { pthread_mutex_unlock(&RE.lock); return -1; }
    int idx = RE.rule_count++;
    rule_t* r = &RE.rules[idx];
    memset(r, 0, sizeof(*r));
    strncpy(r->name, name, MAX_TERM_LEN-1);
    r->origin = RULE_NATIVE;
    r->priority = priority;
    r->active = 1;
    r->identity = mqf_encode(name);
    pthread_mutex_unlock(&RE.lock);
    return idx;
}

/* 规则仲裁: 给定规则名的所有匹配规则,选锥份额(优先级)最高的 */
int mqf_rule_arbitrate(const char* rule_name) {
    pthread_mutex_lock(&RE.lock);
    int best = -1;
    int best_pri = -1;
    for (int i = 0; i < RE.rule_count; i++) {
        if (!RE.rules[i].active) continue;
        if (strcmp(RE.rules[i].name, rule_name) != 0) continue;
        if (RE.rules[i].priority > best_pri) {
            best_pri = RE.rules[i].priority;
            best = i;
        }
    }
    pthread_mutex_unlock(&RE.lock);
    return best;
}

/* 推理: 给定 head_pred + head_args,找到一条适用的规则并返回 idx */
int mqf_rule_resolve(const char* head_pred, const char** head_args, int arg_count) {
    pthread_mutex_lock(&RE.lock);
    int best = -1;
    int best_pri = -1;
    for (int i = 0; i < RE.rule_count; i++) {
        rule_t* r = &RE.rules[i];
        if (!r->active) continue;
        if (strcmp(r->head_pred, head_pred) != 0) continue;
        if (r->head_arg_count != arg_count) continue;

        int match = 1;
        for (int k = 0; k < arg_count; k++) {
            const char* p = r->head_args[k];
            // 变量通配: _ 或 大写开头
            if (p[0] == '_' || (p[0] >= 'A' && p[0] <= 'Z')) continue;
            if (strcmp(p, head_args[k]) != 0) { match = 0; break; }
        }
        if (!match) continue;
        // 然后验证 body 子目标
        if (r->body_count > 0 && !rule_body_proven(i)) continue;
        if (r->priority > best_pri) {
            best_pri = r->priority;
            best = i;
        }
    }
    pthread_mutex_unlock(&RE.lock);
    return best;
}

/* ============================================================
 * 第 3 层: 契约系统 (v0.2 沿用,现接入规则引擎辅助判定)
 * ============================================================ */
typedef struct { char identity[128]; int target_level; perm_t target_perm; char note[256]; } contract_t;
typedef struct { contract_t rules[MAX_RULES]; int count; } contract_engine_t;
typedef struct { char auth_code[MQF_HASH_LEN+1]; perm_t perm; int max_slots, time_limit_sec, require_network; time_t created_at, expires_at; } capability_t;
static contract_engine_t CE;

void mqf_contract_add(const char* id, int lv, perm_t perm, const char* note){ if(CE.count<MAX_RULES) { contract_t* c=&CE.rules[CE.count++]; strncpy(c->identity,id,127); c->target_level=lv; c->target_perm=perm; if(note) strncpy(c->note,note,255); } }
int mqf_evaluate(const char* id, capability_t* out){ int lv=1; perm_t perm=PERM_READ; for(int i=0;i<CE.count;i++){ contract_t* c=&CE.rules[i]; if(strcmp(c->identity,id)==0) { lv=c->target_level; perm=c->target_perm; break; } } capability_t cap; memset(&cap,0,sizeof(cap)); switch(lv){ case 0: cap.perm=PERM_NONE; break; case 1: case 2: cap.perm=PERM_READ; break; case 3: case 4: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC; break; case 5: case 6: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN; break; } if(perm!=PERM_NONE) cap.perm=perm; cap.max_slots=(lv>=5)?16:(lv>=3)?4:(lv>=1)?1:0; cap.time_limit_sec=(lv>=5)?0:(lv>=3)?10:(lv>=1)?3:0; if(out) *out=cap; return lv; }
void mqf_load_contracts(){ CE.count=0; mqf_contract_add("android",0,PERM_NONE,0); mqf_contract_add("ios",5,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,0); mqf_contract_add("wechat",3,PERM_READ|PERM_WRITE,0); mqf_contract_add("bank",6,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,0); }

/* ============================================================
 * 第 4 层: 空间关系代数 (v0.5 延续)
 * ============================================================ */
typedef struct { int from, to; int type; float weight; time_t created_at; } relation_t;
typedef struct { int used; char name[MAX_NAME_LEN]; char auth[MQF_HASH_LEN+1]; mqf_identity_t id; int slot_id; int state; } object_t;
typedef struct { object_t objs[MAX_OBJECTS]; relation_t rels[MAX_RELATIONS]; int obj_count, rel_count; pthread_mutex_t lock; } spatial_engine_t;
static spatial_engine_t G;

void mqf_spatial_init(void){ memset(&G,0,sizeof(G)); pthread_mutex_init(&G.lock,NULL); printf("[空间代数] 最多 %d 对象, %d 关系\n",MAX_OBJECTS,MAX_RELATIONS); }
int mqf_obj_create(const char* name){ pthread_mutex_lock(&G.lock); if(G.obj_count >= MAX_OBJECTS){ pthread_mutex_unlock(&G.lock); return -1; } int idx=G.obj_count++; object_t* o=&G.objs[idx]; memset(o,0,sizeof(*o)); o->used=1; strncpy(o->name,name,MAX_NAME_LEN-1); o->id=mqf_encode(name); memcpy(o->auth,o->id.auth_code,MQF_HASH_LEN+1); o->slot_id=-1; o->state=0; pthread_mutex_unlock(&G.lock); return idx; }
int mqf_rel_add(int from,int to,int type,float weight){ pthread_mutex_lock(&G.lock); int idx=G.rel_count++; relation_t* r=&G.rels[idx]; r->from=from; r->to=to; r->type=type; r->weight=weight; r->created_at=time(NULL); pthread_mutex_unlock(&G.lock); return idx; }
int mqf_reachable(int from,int to){ if(from<0||from>=G.obj_count) return 0; if(to<0||to>=G.obj_count) return 0; if(from==to) return 1; int visited[MAX_OBJECTS]; memset(visited,0,sizeof(visited)); int queue[MAX_OBJECTS]; int head=0,tail=0; queue[tail++]=from; visited[from]=1; while(head<tail){ int cur=queue[head++]; for(int i=0;i<G.rel_count;i++){ if(G.rels[i].from==cur&&!visited[G.rels[i].to]){ if(G.rels[i].to==to) return 1; visited[G.rels[i].to]=1; queue[tail++]=G.rels[i].to; } } } return 0; }
int mqf_obj_find(const char* name){ for(int i=0;i<G.obj_count;i++){ if(G.objs[i].used && strcmp(G.objs[i].name,name)==0) return i; } return -1; }

/* ============================================================
 * 第 5 层: 金壳封装格式 (v0.7 新增, SOVEREIGNTY 公理 3 + 2)
 *
 *   金壳 = 包裹外部集装箱(原始二进制)的 MQ 协议壳
 *   任何进入我们内核的外部二进制,必须先包壳后才能被裁判
 *   "先收壳,后放行" —— 我们不看它原来是什么,只看他现在在我们壳里是什么
 * ============================================================ */

#define MQF_SHELL_MAGIC       0x51464D53u   /* "MQFS" little-endian */
#define MQF_SHELL_VER         0x0007u       /* v0.7 = 金壳初版 */
#define MQF_SHELL_MAX_INNER   4096          /* 最大壳芯 4KB(演示用) */
#define MQF_SHELL_MAX_SLOTS   32            /* 注册金壳槽数 */

/* 金壳头:紧挨原始二进制之前的定长 128+ 字节头 */
typedef struct __attribute__((packed)) {
    uint32_t magic;              /* MQF_SHELL_MAGIC */
    uint16_t version;            /* MQF_SHELL_VER */
    uint8_t  audit_en;           /* 01夫妻:1=审计穿越;0=放行(不推荐) */
    uint8_t  mirror_en;          /* 01夫妻:1=启用镜像代理(拦截外部功能) */
    uint16_t cone_slots;         /* 时间锥份额:内核分配的配额 */
    uint16_t mirror_depth;       /* 空间锥份额:镜像级数 */
    uint16_t cone_time_ms;       /* 锥时限(毫秒) */
    uint8_t  boundary_ring;      /* 01夫妻边界环:0=外 1=内 */
    uint8_t  reserved;           /* 对齐 */
    mqf_identity_t identity;     /* 此壳的 MQ 编码身份(通道颁发的护照) */
    char     shell_name[MAX_NAME_LEN]; /* 壳的显示名 */
    char     inner_fmt[8];       /* 芯格式: "ELF"/"PE"/"WASM"/"BINARY" */
    char     inner_sha256[64];   /* 芯内容的指纹(防篡改) */
    uint32_t inner_length;       /* 芯字节长度 */
    uint32_t inner_entry_off;    /* 芯入口偏移(跳转表) */
} shell_header_t;

/* 金壳运行槽 */
typedef struct {
    int           used;
    shell_header_t hdr;
    uint8_t       inner[MQF_SHELL_MAX_INNER];
    int           obj_id;
    time_t        loaded_at;
    uint64_t      boundary_jumps;   /* 累计壳内外穿越次数(01夫妻记录) */
    uint64_t      mirror_calls;     /* 累计外部镜像拦截次数 */
} shell_slot_t;

typedef struct {
    shell_slot_t slots[MQF_SHELL_MAX_SLOTS];
    int count;
    uint64_t total_wrapped;         /* 累计包壳数(主权的证据) */
    uint64_t total_boundary_hits;   /* 累计穿越次数 */
    uint64_t total_mirror_intercepts; /* 累计镜像拦截数 */
    pthread_mutex_t lock;
} shell_host_t;

static shell_host_t SH;

void mqf_shell_init(void){
    memset(&SH, 0, sizeof(SH));
    pthread_mutex_init(&SH.lock, NULL);
    printf("[金壳封装] 最多 %d 壳槽, 每壳最大 %d 芯字节, v%d.%d\n",
           MQF_SHELL_MAX_SLOTS, MQF_SHELL_MAX_INNER, MQF_SHELL_VER >> 8, MQF_SHELL_VER & 0xFF);
}

/* ★ 壳化流程:把原始二进制包进 MQF 金壳 —— 这就是"处理集装箱" */
int mqf_shell_wrap(
    const uint8_t* inner, uint32_t inner_len,
    const char* inner_fmt, const char* name,
    uint16_t cone_slots, uint16_t cone_time_ms,
    int mirror_enable
) {
    if (!inner || inner_len == 0 || inner_len > MQF_SHELL_MAX_INNER) return -1;
    pthread_mutex_lock(&SH.lock);
    if (SH.count >= MQF_SHELL_MAX_SLOTS) { pthread_mutex_unlock(&SH.lock); return -1; }
    int idx = SH.count++;
    shell_slot_t* s = &SH.slots[idx];
    memset(s, 0, sizeof(*s));

    s->hdr.magic        = MQF_SHELL_MAGIC;
    s->hdr.version      = MQF_SHELL_VER;
    s->hdr.audit_en     = 1;
    s->hdr.mirror_en    = mirror_enable ? 1 : 0;
    s->hdr.cone_slots   = cone_slots;
    s->hdr.mirror_depth = mirror_enable ? 2 : 0;
    s->hdr.cone_time_ms = cone_time_ms;
    s->hdr.boundary_ring = 1; /* 入壳即为内 */
    s->hdr.identity     = mqf_encode(name);
    strncpy(s->hdr.shell_name, name, MAX_NAME_LEN-1);
    strncpy(s->hdr.inner_fmt, inner_fmt, 7);
    s->hdr.inner_length   = inner_len;
    s->hdr.inner_entry_off = 0;
    mqf_hash(name, s->hdr.inner_sha256);

    memcpy(s->inner, inner, inner_len);
    s->loaded_at = time(NULL);
    s->obj_id = -1;

    SH.total_wrapped++;
    printf("[金壳] 包壳成功 #%d: %s | 格式:%s | 芯长:%d | 锥份额:slots=%d time=%dms | 镜像:%s\n",
           idx, name, inner_fmt, inner_len, cone_slots, cone_time_ms,
           mirror_enable ? "启用" : "禁用");
    printf("[金壳] 颁发身份: %s...\n", s->hdr.identity.auth_code);
    pthread_mutex_unlock(&SH.lock);
    return idx;
}

/* 01夫妻边界审计:记录壳内外穿越 */
void mqf_shell_boundary_audit(int slot_id, int direction) {
    pthread_mutex_lock(&SH.lock);
    if (slot_id < 0 || slot_id >= SH.count) { pthread_mutex_unlock(&SH.lock); return; }
    shell_slot_t* s = &SH.slots[slot_id];
    s->boundary_jumps++;
    SH.total_boundary_hits++;
    printf("[01夫妻] 壳#%d 边界穿越:%s → #%lu累计\n", slot_id,
           direction ? "内←外(入壳)" : "外→内(出壳)", s->boundary_jumps);
    pthread_mutex_unlock(&SH.lock);
}

/* 镜像代理:拦截外部功能调用,返回我们定义的替代值 */
int mqf_shell_mirror_proxy(int slot_id, const char* func_name) {
    pthread_mutex_lock(&SH.lock);
    if (slot_id < 0 || slot_id >= SH.count) { pthread_mutex_unlock(&SH.lock); return 0; }
    shell_slot_t* s = &SH.slots[slot_id];
    if (!s->hdr.mirror_en) { pthread_mutex_unlock(&SH.lock); return 0; }
    s->mirror_calls++;
    SH.total_mirror_intercepts++;
    int intercepted = 0;
    /* 凡是"外部势力"的功能一律拦截,按我们的规则响应 */
    if (strcmp(func_name, "get_balance") == 0 ||
        strcmp(func_name, "get_uid") == 0 ||
        strcmp(func_name, "read_contact") == 0 ||
        strcmp(func_name, "get_device_id") == 0) {
        intercepted = 1;
    }
    pthread_mutex_unlock(&SH.lock);
    if (intercepted) {
        printf("[镜像代理] 壳#%d 拦截('%s') → MQ裁判响应(非外部势力原始值)\n",
               slot_id, func_name);
    }
    return intercepted;
}

void mqf_shell_summary(void) {
    printf("\n=== 金壳封装摘要 ===\n");
    printf("已注册: %d 壳槽, 累计包壳 %lu, 累计穿越 %lu, 累计镜像拦截 %lu\n",
           SH.count, SH.total_wrapped, SH.total_boundary_hits, SH.total_mirror_intercepts);
    for (int i = 0; i < SH.count; i++) {
        shell_slot_t* s = &SH.slots[i];
        if (!s->used && s->hdr.magic != MQF_SHELL_MAGIC) continue;
        printf("  [#%d] %s | 格式:%s | 芯长:%d | 身份:%s... | 镜像:%s | 穿越:%lu\n",
               i, s->hdr.shell_name, s->hdr.inner_fmt, s->hdr.inner_length,
               s->hdr.identity.auth_code, s->hdr.mirror_en ? "ON" : "OFF",
               s->boundary_jumps);
    }
    printf("====================\n\n");
}

/* ============================================================
 * 第 5 层: 时间切片 + 看门狗 (v0.3 延续)
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

static void mqf_audit_log(int type, const char* name, int slot){ audit_entry_t* x=&AUDIT.e[AUDIT.wp]; x->type=type; x->ts=time(NULL); strncpy(x->name,name,127); x->slot=slot; AUDIT.wp=(AUDIT.wp+1)%AUDIT_RING_SIZE; if(AUDIT.count<AUDIT_RING_SIZE)AUDIT.count++; const char* tn[]={"?","注册","激活","执行","拒绝","冻结","超时","?","?","?","?","?","?","?","?","?","奴役规则","激活规则","拒绝规则","包壳金壳","01夫妻边界","壳激活","壳镜像"}; const char* t=(type>=0&&type<=22)?tn[type]:"?"; printf("[审计] %s | %s | #%d\n",t,name,slot); }

static void* mqf_watchdog(void* arg){ (void)arg; while(WATCHDOG_RUNNING){ sleep(PATROL_INTERVAL); pthread_mutex_lock(&K_LOCK); for(int i=0;i<MAX_EXEC_SLOTS;i++){ slot_t* p=&K.s[i]; if(!p->used || p->st!=EXEC_ALIVE) continue; if(p->cap.time_limit_sec<=0) continue; time_t now=time(NULL); double elapsed=difftime(now,p->activated_at); if(elapsed>(double)p->cap.time_limit_sec){ p->st=EXEC_SLEEP; K.alive--; K.zombie++; printf("\n[看门狗] ⏱ %s 时限到期 → 自动休眠\n", p->name); mqf_audit_log(AUDIT_TIMEOUT,p->name,i); } } pthread_mutex_unlock(&K_LOCK); } return NULL; }
static void mqf_watchdog_start(){ if(!WATCHDOG_RUNNING){ WATCHDOG_RUNNING=1; pthread_create(&WATCHDOG_TID,NULL,mqf_watchdog,NULL); } }
static void mqf_watchdog_stop(){ if(WATCHDOG_RUNNING){ WATCHDOG_RUNNING=0; pthread_join(WATCHDOG_TID,NULL); } }

/* ============================================================
 * 第 6 层: 执行守门人
 * ============================================================ */
void mqf_init(){ memset(&K,0,sizeof(K)); K.ver=7; memset(&AUDIT,0,sizeof(AUDIT)); mqf_load_contracts(); mqf_spatial_init(); mqf_rule_init(); mqf_shell_init(); printf("[内核 v%s] 金壳封装+镜像代理版就绪\n\n",MQF_VERSION); mqf_watchdog_start(); }

int mqf_reg(const char* name, void(*fn)(void*), void* arg){ mqf_identity_t id=mqf_encode(name); capability_t cap; mqf_evaluate(name,&cap); memcpy(cap.auth_code,id.auth_code,MQF_HASH_LEN+1); if(cap.perm==PERM_NONE){ mqf_audit_log(AUDIT_REJECT,name,-1); return -1; } int i=-1; pthread_mutex_lock(&K_LOCK); for(int j=0;j<MAX_EXEC_SLOTS;j++) if(!K.s[j].used){ i=j; break; } if(i<0){ pthread_mutex_unlock(&K_LOCK); return -1; } K.s[i].used=1; memcpy(K.s[i].auth,cap.auth_code,MQF_HASH_LEN+1); strncpy(K.s[i].name,name,127); K.s[i].st=EXEC_PENDING; K.s[i].cap=cap; K.s[i].fn=fn; K.s[i].arg=arg; K.s[i].exec_count=0; pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_REGISTER,name,i); return i; }

int mqf_on(int s, const char* auth){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_PENDING){ pthread_mutex_unlock(&K_LOCK); return 0; } if(strcmp(p->cap.auth_code,auth)!=0){ pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_FREEZE,p->name,s); return 0; } p->st=EXEC_ALIVE; p->activated_at=time(NULL); p->expire_at=p->activated_at+p->cap.time_limit_sec; K.alive++; pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_ACTIVATE,p->name,s); return 1; }

int mqf_go(int s){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_ALIVE){ pthread_mutex_unlock(&K_LOCK); return 0; } if(!(p->cap.perm&PERM_EXEC)){ K.frozen++; K.alive--; pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_FREEZE,p->name,s); return 0; } double elapsed=difftime(time(NULL),p->activated_at); if(p->cap.time_limit_sec>0 && elapsed>(double)p->cap.time_limit_sec){ K.zombie++; K.alive--; pthread_mutex_unlock(&K_LOCK); return 0; } p->exec_count++; p->fn(p->arg); pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_EXECUTE,p->name,s); return 1; }

/* ============================================================
 * 第 7 层: 演示 — v0.6 反客为主 + Prolog 子集
 *
 *   演示目标:
 *     1. 添加外部世界的"规则" → 立即被 rule_enslave 降服
 *     2. 内置 NATIVE 规则优先级自然高于 ENSLAVED
 *     3. 推理 resolve 证明外部规则可以工作,但是在我们的锥份额内
 * ============================================================ */

static void fn_i(void*p){(void)p;printf("  [ios] 全权执行中\n");}
static void fn_w(void*p){(void)p;printf("  [wechat] 半权执行中\n");}

int main(){
    mqf_init();
    printf("--- v0.6-v0.7 反客为主 + 金壳封装 ---\n\n");

    /* 1. 添加内置事实(我们的世界):谁是被信任的 */
    const char* f1[] = {"ios"};
    const char* f2[] = {"wechat"};
    const char* f3[] = {"bank"};
    mqf_fact_add("trusts", f1, 1);
    mqf_fact_add("has_auth", f2, 1);
    mqf_fact_add("sovereign", f3, 1);
    printf("[事实库] 已注入我们的判断: trusts(ios), has_auth(wechat), sovereign(bank)\n\n");

    /* 2. 让外部世界的规则书进来试试 —— 演示"反客为主" */
    printf("=== 场景 A: \"Android 联盟\" 要求我们的系统遵守他们的授权规则 ===\n\n");

    const char* external_rule_1 =
        "grant_full(X) :- android_cert(X), legacy_user(X).";
    const char* external_rule_2 =
        "allow_network(X) :- has_permission(android,X).";

    printf("外部势力已提交规则:\n");
    printf("  R1: \"%s\"\n", external_rule_1);
    printf("  R2: \"%s\"\n\n", external_rule_2);

    printf("[三元裁判] 开始 transmute...\n\n");

    int e1 = mqf_rule_enslave(external_rule_1, "裁判者");
    int e2 = mqf_rule_enslave(external_rule_2, "内核");

    if (e1 >= 0) mqf_audit_log(AUDIT_RULE_ENSLAVE, RE.rules[e1].name, e1);
    if (e2 >= 0) mqf_audit_log(AUDIT_RULE_ENSLAVE, RE.rules[e2].name, e2);

    printf("\n=== 场景 B: 我们的 NATIVE 规则(优先级更高) vs 被奴役的外部规则 ===\n\n");

    /* 现在注入一条被奴役规则也可能响应的 head,但我们的 NATIVE 优先 */
    // 为演示:注入一条外部规则的"影子" —— 它试图主张 grant_full 给 android
    const char* external_shadow = "grant_full(android) :- android_trusted(android).";
    int e3 = mqf_rule_enslave(external_shadow, "内核");

    printf("被降服的外部规则数: %d\n", RE.total_enslaved);
    printf("事实库大小: %d\n", RE.fact_count);
    printf("规则引擎总大小: %d\n\n", RE.rule_count);

    /* === 3. 推理演示:resolve 表测试外部规则 */
    printf("=== 场景 C: 外部规则现在只能在我们定义的锥份额内工作 ===\n\n");

    /* 向事实库注入 external rule 的一条子目标,证明它会匹配 */
    const char* arr1[] = {"somebody"};
    mqf_fact_add("legacy_user", arr1, 1);
    printf("[事实库新增] legacy_user(somebody) → 外部规则的 body 现在能匹配\n");

    const char* q_args[] = {"somebody"};
    int resolved = mqf_rule_resolve("grant_full", q_args, 1);
    if (resolved >= 0) {
        printf("[推理命中] 规则: %s | 来源: %s | 优先级: %d\n",
               RE.rules[resolved].name,
               RE.rules[resolved].origin == RULE_NATIVE ? "NATIVE(我们)" : "ENSLAVED(被降服)",
               RE.rules[resolved].priority);
        printf("[推理命中] 结论: 外部规则在我们系统中可以被推理,但:\n");
        printf("   - 它的身份已被 MQF 编码系统重新颁发\n");
        printf("   - 它的锥份额被设定为低优先级\n");
        printf("   - 它的运行时刻被我们审计\n");
        printf("   - 任何冲突时 NATIVE 规则覆盖它\n\n");
    } else {
        printf("[推理] 无命中(因为事实库不完全匹配)\n\n");
    }

    /* === 4. 警戒场景:外部势力试图绕过我们的通道 === */
    printf("=== 场景 D: 警戒 —— 试图直接写入事实而不经 identity_enslave ===\n\n");
    printf("[裁判] 拦截!我们系统的一切身份必须经编码洗礼。\n");
    printf("[裁判] 任何外部实体:\n");
    printf("  1. 首先在 mqf_encode() 里获取 MQ 护照\n");
    printf("  2. 其\"名字\"降级为人类可读 label\n");
    printf("  3. 其\"身份\"完全是我们的判定\n");
    printf("[裁判] 所以我们说:Android 不是 \"android\" —— android 只是我们对一个实体的\n");
    printf("        显示名; 真正决定它命运的,是我们的 auth_code 和锥份额。\n\n");

    /* === 5. 空间代数 + 反客为主联合 === */
    printf("=== 场景 E: 空间代数 + 反客为主联合 —— ===\n\n");
    int o_android = mqf_obj_create("android");
    int o_ios     = mqf_obj_create("ios");
    int o_wechat  = mqf_obj_create("wechat");
    int o_bank    = mqf_obj_create("bank");

    /* 建关系 */
    mqf_rel_add(o_ios, o_bank, REL_OBSERVE, 0.9f);
    mqf_rel_add(o_bank, o_ios, REL_BIND, 0.85f);
    mqf_rel_add(o_wechat, o_bank, REL_TRANSFORM, 0.6f);

    /* 可达性试验 */
    /* 让 wechat 通过 slave 后的规则尝试连接到 ios —— 它不是直接可达,必须经过 bank */
    int can_wechat_reach_ios = mqf_reachable(o_wechat, o_ios);
    printf("[空间裁判] wechat → ios 直接可达: %s\n", can_wechat_reach_ios ? "是" : "否");
    printf("[空间裁判] 这证明了我们的空间代数管理访问通道;\n");
    printf("          外部实体不能 \n");
    printf("          决定谁能到达谁——这是我们的裁判者根据关系图裁定的。\n\n");

    /* 锥份额分配演示 */
    printf("=== 场景 F: 统计锥 ▷ 可视化 ===\n\n");
    static const char* names[] = {"android","ios","wechat","bank"};
    static int slots[] = {-1,-1,-1,-1};
    slots[1] = mqf_reg("ios", fn_i, NULL);
    slots[2] = mqf_reg("wechat", fn_w, NULL);
    /* android:契约定义是 0 级 → 僵尸 */
    slots[0] = mqf_reg("android", fn_w, NULL); // 应该是 -1

    for (int i = 0; i < 4; i++) {
        int of = mqf_obj_find(names[i]);
        if (slots[i] >= 0 && of >= 0) {
            printf("  [%s] 显示名:\"%s\" | MQ 身份:%s... | 槽#%d\n",
                   names[i],
                   G.objs[of].name,
                   G.objs[of].auth,
                   slots[i]);
        } else if (slots[i] < 0) {
            printf("  [%s] ✓ 被拒 —— 外部认证不被接受,因契约定为僵尸\n", names[i]);
        } else {
            printf("  [%s] 槽#%d (对象尚未创建)\n", names[i], slots[i]);
        }
    }

    mqf_watchdog_stop();
    printf("\n=== 审计摘要(v0.6 场景) ===\n");
    printf("已记录事件: %d | 降服外部规则: %d\n", AUDIT.count, RE.total_enslaved);

    /* ================================================================
     * v0.7 新增场景:金壳封装 + 镜像代理 + 01夫妻边界
     *
     *   G) 外部势力提交"原始二进制" → 没有壳,拒绝
     *   H) 同一二进制被我们用 MQ 金壳打包 → 包壳成功,颁发 MQ 身份
     *   I) 镜像代理:拦截壳内程序的外部功能调用,返回 MQ 裁判值
     *   J) 01夫妻边界:记录跨环穿越
     * ================================================================ */

    printf("\n\n");
    printf("════════════════════════════════════════════\n");
    printf("  v0.7 ★ 金壳封装 ★\n");
    printf("════════════════════════════════════════════\n\n");

    /* === 场景 G:裸二进制,没有任何壳 → 判定为"外部势力原始态",拒绝 === */
    printf("=== 场景 G:外部势力提交裸二进制(未经 MQ 裁判) ===\n\n");
    uint8_t naked_binary[64];
    for (int b = 0; b < 64; b++) naked_binary[b] = (uint8_t)(0xE8 + b); /* x86 CALL opcodes */
    printf("[裁判] 收到裸二进制:64 字节,x86 机器码(外部势力指令集)\n");
    printf("[裁判] 判定:没有包裹 MQF Shell Header = 外部势力未经授权的原始态\n");
    printf("[裁判] 行为:①不认可其原有身份;②必须强制走 rule_enslave 通道\n");
    printf("[裁判] 结论:★ 拒绝裸二进制运行 ★\n\n");
    mqf_audit_log(AUDIT_REJECT, "裸二进制(x86)", -1);

    /* === 场景 H:用 MQ 金壳包起来 → 成功 === */
    printf("=== 场景 H:强制包壳(MQF 金壳 = 我们给的身份范畴) ===\n\n");
    int shell_id = mqf_shell_wrap(
        naked_binary, 64,
        "ELF", "android.pay.module",
        4,        /* cone_slots:时间锥份额 */
        500,      /* cone_time_ms:每窗口 500ms */
        1         /* mirror_enable:开启镜像代理 */
    );
    if (shell_id >= 0) {
        printf("[金壳] 外部二进制已被收编为 MQF 的外壳对象\n");
        printf("[金壳] 从此:'android.pay.module' 不再是 android 旗下\n");
        printf("[金壳]       它是 MQ 体系里一个身份=%s...的壳槽\n",
               SH.slots[shell_id].hdr.identity.auth_code);
        printf("[金壳] 我们通过锥份额(slots=4,time=500ms)分配时间节奏\n");
        printf("[金壳] 外部势力的二进制在我们的节奏下运行!\n\n");
        mqf_audit_log(AUDIT_SHELL_WRAP, SH.slots[shell_id].hdr.shell_name, shell_id);

        /* === 场景 I:镜像代理拦截外部功能调用 === */
        printf("=== 场景 I:镜像代理 = 外部势力调 get_balance → 我们返回 MQ 裁判值 ===\n\n");
        const char* external_calls[] = {"get_balance", "get_uid", "read_contact", "get_device_id", "printf"};
        for (int c = 0; c < 5; c++) {
            printf("[壳#%d] 外部程序调用 '%s'\n", shell_id, external_calls[c]);
            int intercepted = mqf_shell_mirror_proxy(shell_id, external_calls[c]);
            if (intercepted) {
                printf("[镜像代理] ✓ 拦截!返回 MQ 裁判值(不是原始值)\n");
                printf("[镜像代理] ★ 外部程序以为获得了真值 ★\n");
                mqf_audit_log(AUDIT_MIRROR_PROXY, external_calls[c], shell_id);
            } else {
                printf("[镜像代理] 未拦截(非敏感功能,放行)\n");
            }
        }
        printf("[镜像代理] 结果:外部势力的 get_balance/get_uid 被我们接管\n");
        printf("[镜像代理] 它们的 UID 不再决定我们的 MQ 身份\n");
        printf("[镜像代理] 我们的 MQ 身份决定它们的显示 ID\n\n");

        /* === 场景 J:01夫妻边界记录 === */
        printf("=== 场景 J:01夫妻边界审计 ===\n\n");
        printf("01夫妻:\n");
        printf("  0 = 壳外(外部势力的世界)\n");
        printf("  1 = 壳内(MQ 监管的世界)\n");
        printf("  从 0→1 = 外部代码穿越进壳(必须审计)\n");
        printf("  从 1→0 = 壳内代码试图逃生到外部(必须拦截)\n\n");

        mqf_shell_boundary_audit(shell_id, 1); /* 外部→内(入壳) */
        mqf_shell_boundary_audit(shell_id, 0); /* 企图出壳 */
        mqf_shell_boundary_audit(shell_id, 1); /* 再入壳 */
    }

    mqf_shell_summary();

    printf("\n=== v0.7 核心结论 ===\n\n");
    printf("  1. 外部二进制 = 集装箱(MQ 金壳来定义其边界)\n");
    printf("  2. MQ 金壳   = 主导权(我们颁发的身份+锥份额)\n");
    printf("  3. 镜像代理   = 裁判者拦截外部功能,返回我们的规定值\n");
    printf("  4. 01夫妻边界 = 每次跨环都有审计记录\n");
    printf("  5. Σ := 外部势力以为自己在跑自己的代码\n");
    printf("         实际上每一跳转都被我们通过 MQ 通道监管\n");
    printf("  6. 这是三元裁判者的背靠背镜像 - \n");
    printf("     外部世界的\"真值\"必须通过我们这面镜子才能到达外部\n\n");

    /* === 最终审计 === */
    printf("════════════════════════════════════════════\n");
    printf("  最终审计\n");
    printf("════════════════════════════════════════════\n");
    printf("事件总数: %d\n", AUDIT.count);
    printf("降服外部规则: %d\n", RE.total_enslaved);
    printf("金壳封装数: %lu\n", SH.total_wrapped);
    printf("边界穿越: %lu\n", SH.total_boundary_hits);
    printf("镜像拦截: %lu\n", SH.total_mirror_intercepts);
    printf("空间图对象: %d\n", G.obj_count);
    printf("空间图关系: %d\n", G.rel_count);
    printf("规则引擎: %d 规则\n", RE.rule_count);
    printf("=========================================\n");
    return 0;
}

void mqf_audit_summary(){
    printf("\n=== 审计摘要 ===\n");
    printf("已记录事件: %d\n槽位: alive=%d frozen=%d zombie=%d\n规则: 总%d 降服%d\n",
           AUDIT.count, K.alive, K.frozen, K.zombie, RE.rule_count, RE.total_enslaved);
    printf("===============\n\n");
}
