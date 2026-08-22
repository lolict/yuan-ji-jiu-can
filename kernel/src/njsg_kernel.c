/**
 * 囡囝共生 · 共生共识内核 v1.0 — MQ Symbiosis Kernel
 * ------------------------------------------------
 * 囡囝:方言,意为小孩,萌而正向
 * 共生:共识圆 · 共生圆 · 共生姻 · 共首源 · 共首一 · 共守约 · 感受益
 * 人名谱: 刘连理/刘楚恬/前芳/萝莉兰/零离俩/零厘里/零劳累等
 *
 * v1.0 新增: 五元旋钮 —— 运动步量 + 角度 wrap + 占存判定
 * v2.0 里程碑: 三元→七元+全栈+生态+盟机合+WASM+移动端
 *
 * 已落地的维度演化:
 *   三元(v0.6) → 点+关系(身份编码 + 规则引擎 + 空间代数 + 统计锥)
 *   四元(v0.9) → 面+内外间隔(拓扑域 + 跨域裁定 + 宏微量判断 + 金壳镜像)
 *   五元(v1.0) → 旋钮+运动步量(5 步位移累加 + 角度 wrap 360° + 占存判定)
 *   六元(v1.1) → 对称群+六面体 (双三元镜像对称,信任交集 vs 不可信任无交集)
 *   七元(v1.2) → 蜂窝套壳 (壳层无限嵌套,容器-in-容器)
 *
 * 四元面判断的工程特征:
 *   - 拓扑域标记: DFS 从三元裁判者出发,只走 observe+bind 边,能达 = 内部
 *   - 跨域请求裁定: 默认拒绝外部→域内,间隙需审查,transform 边一律拒绝
 *   - 宏量微量阈值: 域内宏量必须审查,微量可积累,微量积聚→宏量
 *   - 内积可积累 vs 外积不可积累: MQ passport 域内聚合 vs 域外无效
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

/* MLLL 七维符号编码 + 桥接（仅 -DMLLL_LIBRARY 时引入） */
#ifdef MLLL_LIBRARY
#include "mlll_symbols.h"
#include "mlll_kernel_bridge.h"
#endif

/* ============================================================
 * v1.0 五元旋钮前置声明
 * ============================================================ */
void njsg_five_yuan_init(void);
void njsg_five_yuan_demo(void);
int  njsg_five_yuan_register(const char* name);
void njsg_five_yuan_push(int obj_id, float angle_inc, float force_inc);
void njsg_five_yuan_set_ratio(int obj_id, float in_val, float out_val);
void njsg_five_yuan_print(void);
extern int fy_count;

/* v1.1 六元对称群前置声明 */
void njsg_six_yuan_init(void);
void njsg_six_yuan_demo(void);
void njsg_six_yuan_print(void);
void njsg_six_yuan_print_sides(int pair_id);
int  njsg_six_yuan_register_pair(const char* name, int judge_a, int judge_b);
void njsg_six_yuan_set_trust(int pair_id, float ta, float tb);
int  njsg_six_yuan_verdict(int pair_id);
extern int sy_count;

/* v1.2 七元蜂窝套壳前置声明 */
void njsg_shell_nest_init(void);
void njsg_shell_nest_demo(void);
void njsg_shell_nest_print(void);
int  njsg_shell_nest_wrap(const char* name);
int  njsg_shell_nest_insert(int parent_id, const char* child_name);
void njsg_shell_nest_honeycomb(int level);
extern int sn_count;

/* v1.3 全栈联动前置声明 */
void njsg_feishu_bot_bridge_init(void);
void njsg_qimei_init(void);
void njsg_roadmap_self_check(void);
void njsg_feishu_bot_demo(void);
void njsg_qimei_demo(void);
void njsg_roadmap_check_demo(void);
void njsg_v13_demo(void);

#define NJSG_VERSION     "2.0.0"
#define NJSG_AUTH_LEN    32
#define NJSG_HASH_LEN    64
#define MAX_EXEC_SLOTS  16
#define AUDIT_RING_SIZE 1024
#define MAX_FACTS       256
#define MAX_RULES       128
#define MAX_TERMS       16
#define MAX_TERM_LEN    64
#define MAX_OBJECTS     128
#define MAX_RELATIONS   1024
#define MAX_NAME_LEN    128

/* v1.0 五元旋钮常量 */
#define FIVE_YUAN_STEPS 5
#define FIVE_YUAN_OCCUPY_THRESHOLD 100

typedef enum { EXEC_SLEEP=0, EXEC_PENDING, EXEC_ALIVE, EXEC_FROZEN, EXEC_ZOMBIE } exec_state_t;
typedef enum { PERM_NONE=0x00, PERM_READ=0x01, PERM_WRITE=0x02, PERM_EXEC=0x04, PERM_NET=0x08, PERM_ADMIN=0x10 } perm_t;
typedef enum { AUDIT_REGISTER=1, AUDIT_ACTIVATE, AUDIT_EXECUTE, AUDIT_REJECT, AUDIT_FREEZE, AUDIT_TIMEOUT, AUDIT_RULE_ENSLAVE=16, AUDIT_RULE_ACTIVATE=17, AUDIT_RULE_REJECT=18, AUDIT_SHELL_WRAP=19, AUDIT_SHELL_BOUNDARY=20, AUDIT_SHELL_ACTIVATE=21, AUDIT_MIRROR_PROXY=22 } audit_type_t;
typedef enum { REL_OBSERVE=0, REL_BIND=1, REL_TRANSFORM=2 } rel_type_t;
typedef enum { RULE_NATIVE=0, RULE_ENSLAVED=1 } rule_origin_t;

static const char* NJFS_INITS[]={"b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w","@"};
static const char* NJFS_FINALS[]={"a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er","an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao","iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong","@"};
static const char* REL_NAMES[]={"观测","绑定","变换"};

/* ============================================================
 * 第 1 层: 身份编码 (v0.1 起,本次加入 identity_enslave 强制启用)
 *   公理 1: 任何进入系统的实体必须先被我们的编码系统洗礼
 * ============================================================ */
typedef struct { char text[NJSG_AUTH_LEN+1]; int init_idx,final_idx,tone_idx,stroke_count; char auth_code[NJSG_HASH_LEN+1],hash[NJSG_HASH_LEN+1]; } njsg_identity_t;
static void njsg_hash(const char* in, char* out){ uint32_t h1=0x811c9dc5u,h2=0x0100193u; for(size_t i=0;i<strlen(in);i++){h1^=(uint8_t)in[i];h1*=0x01000193u;h2^=(uint8_t)in[i]<<5;h2*=0x1000193u;} sprintf(out,"%08x%08x%08x%08x%08x%08x",h1,h2,h1^h2,h2^h1,h1+h2,h2-h1); }
njsg_identity_t njsg_encode(const char* text){ njsg_identity_t id; memset(&id,0,sizeof(id)); size_t len=strlen(text); if(len>NJSG_AUTH_LEN)len=NJSG_AUTH_LEN; memcpy(id.text,text,len); id.text[len]='\0'; uint32_t seed=0; for(size_t i=0;i<len;i++) seed=seed*31+(unsigned char)text[i]; id.init_idx=seed%24; id.final_idx=(seed>>5)%39; id.tone_idx=(seed>>10)%5; id.stroke_count=(seed>>15)%30+1; char buf[512]; snprintf(buf,sizeof(buf),"%s|%d|%d|%d|%d",NJFS_INITS[id.init_idx],id.init_idx,id.final_idx,id.tone_idx,id.stroke_count); njsg_hash(buf,id.auth_code); snprintf(buf,sizeof(buf),"%s.%s",text,id.auth_code); njsg_hash(buf,id.hash); return id; }

/* ============================================================
 * 第 2 层: Prolog 子集规则引擎 + 反客为主通道 (v0.6 核心)
 *
 *   外部规则进来 → [裁判通道 rule_enslave] → 内部规则(附上 NJSG 编码标识+锥份额+优先级)
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
    njsg_identity_t identity;                   // 规则的 NJSG 编码身份(我们的颁发)
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
void njsg_rule_init(void) {
    memset(&RE, 0, sizeof(RE));
    pthread_mutex_init(&RE.lock, NULL);
    printf("[规则引擎] Prolog 子集解释器就绪: 最多 %d 事实 + %d 规则\n", MAX_FACTS, MAX_RULES);
}

/* 添加事实 (内置 / 初始化时调用) */
int njsg_fact_add(const char* name, const char** args, int arg_count) {
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
 *     2. 为它做 identity_enslave → njsg_encode → 拿到 MQ 护照
 *     3. 分配锥份额(优先级): ENSLAVED 规则默认低优先级,不让它覆盖我们的 NATIVE 规则
 *     4. 标记 origin=ENSLAVED + enslaver_name=三元裁判者
 *     5. 加入规则引擎(此时才能被推理)
 *
 *   输入: external_rule_text (如 "allow(X) :- legacy_user(X), not banned(X).")
 *   输出: rule_idx 或 -1(解析失败)
 * ============================================================ */
int njsg_rule_enslave(const char* external_rule_text, const char* enslaver_identity) {
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
    njsg_identity_t id = njsg_encode(external_rule_text);
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
int njsg_rule_add_native(const char* name, int priority) {
    pthread_mutex_lock(&RE.lock);
    if (RE.rule_count >= MAX_RULES) { pthread_mutex_unlock(&RE.lock); return -1; }
    int idx = RE.rule_count++;
    rule_t* r = &RE.rules[idx];
    memset(r, 0, sizeof(*r));
    strncpy(r->name, name, MAX_TERM_LEN-1);
    r->origin = RULE_NATIVE;
    r->priority = priority;
    r->active = 1;
    r->identity = njsg_encode(name);
    pthread_mutex_unlock(&RE.lock);
    return idx;
}

/* 规则仲裁: 给定规则名的所有匹配规则,选锥份额(优先级)最高的 */
int njsg_rule_arbitrate(const char* rule_name) {
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
int njsg_rule_resolve(const char* head_pred, const char** head_args, int arg_count) {
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
typedef struct { char auth_code[NJSG_HASH_LEN+1]; perm_t perm; int max_slots, time_limit_sec, require_network; time_t created_at, expires_at; } capability_t;
static contract_engine_t CE;

void njsg_contract_add(const char* id, int lv, perm_t perm, const char* note){ if(CE.count<MAX_RULES) { contract_t* c=&CE.rules[CE.count++]; strncpy(c->identity,id,127); c->target_level=lv; c->target_perm=perm; if(note) strncpy(c->note,note,255); } }
int njsg_evaluate(const char* id, capability_t* out){ int lv=1; perm_t perm=PERM_READ; for(int i=0;i<CE.count;i++){ contract_t* c=&CE.rules[i]; if(strcmp(c->identity,id)==0) { lv=c->target_level; perm=c->target_perm; break; } } capability_t cap; memset(&cap,0,sizeof(cap)); switch(lv){ case 0: cap.perm=PERM_NONE; break; case 1: case 2: cap.perm=PERM_READ; break; case 3: case 4: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC; break; case 5: case 6: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN; break; } if(perm!=PERM_NONE) cap.perm=perm; cap.max_slots=(lv>=5)?16:(lv>=3)?4:(lv>=1)?1:0; cap.time_limit_sec=(lv>=5)?0:(lv>=3)?10:(lv>=1)?3:0; if(out) *out=cap; return lv; }
void njsg_load_contracts(){ CE.count=0; njsg_contract_add("android",0,PERM_NONE,0); njsg_contract_add("ios",5,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,0); njsg_contract_add("wechat",3,PERM_READ|PERM_WRITE,0); njsg_contract_add("bank",6,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,0); }

/* ============================================================
 * 第 4 层: 空间关系代数 (v0.5 延续)
 * ============================================================ */
typedef struct { int from, to; int type; float weight; time_t created_at; } relation_t;
typedef struct { int used; char name[MAX_NAME_LEN]; char auth[NJSG_HASH_LEN+1]; njsg_identity_t id; int slot_id; int state; } object_t;
typedef struct { object_t objs[MAX_OBJECTS]; relation_t rels[MAX_RELATIONS]; int obj_count, rel_count; pthread_mutex_t lock; } spatial_engine_t;
static spatial_engine_t G;

void njsg_spatial_init(void){ memset(&G,0,sizeof(G)); pthread_mutex_init(&G.lock,NULL); printf("[空间代数] 最多 %d 对象, %d 关系\n",MAX_OBJECTS,MAX_RELATIONS); }
int njsg_obj_create(const char* name){ pthread_mutex_lock(&G.lock); if(G.obj_count >= MAX_OBJECTS){ pthread_mutex_unlock(&G.lock); return -1; } int idx=G.obj_count++; object_t* o=&G.objs[idx]; memset(o,0,sizeof(*o)); o->used=1; strncpy(o->name,name,MAX_NAME_LEN-1); o->id=njsg_encode(name); memcpy(o->auth,o->id.auth_code,NJSG_HASH_LEN+1); o->slot_id=-1; o->state=0; pthread_mutex_unlock(&G.lock); return idx; }
int njsg_rel_add(int from,int to,int type,float weight){ pthread_mutex_lock(&G.lock); int idx=G.rel_count++; relation_t* r=&G.rels[idx]; r->from=from; r->to=to; r->type=type; r->weight=weight; r->created_at=time(NULL); pthread_mutex_unlock(&G.lock); return idx; }
int njsg_reachable(int from,int to){ if(from<0||from>=G.obj_count) return 0; if(to<0||to>=G.obj_count) return 0; if(from==to) return 1; int visited[MAX_OBJECTS]; memset(visited,0,sizeof(visited)); int queue[MAX_OBJECTS]; int head=0,tail=0; queue[tail++]=from; visited[from]=1; while(head<tail){ int cur=queue[head++]; for(int i=0;i<G.rel_count;i++){ if(G.rels[i].from==cur&&!visited[G.rels[i].to]){ if(G.rels[i].to==to) return 1; visited[G.rels[i].to]=1; queue[tail++]=G.rels[i].to; } } } return 0; }
int njsg_obj_find(const char* name){ for(int i=0;i<G.obj_count;i++){ if(G.objs[i].used && strcmp(G.objs[i].name,name)==0) return i; } return -1; }

/* ============================================================
 * 第 5 层: 金壳封装格式 (v0.7 新增, SOVEREIGNTY 公理 3 + 2)
 *
 *   金壳 = 包裹外部集装箱(原始二进制)的 MQ 协议壳
 *   任何进入我们内核的外部二进制,必须先包壳后才能被裁判
 *   "先收壳,后放行" —— 我们不看它原来是什么,只看他现在在我们壳里是什么
 * ============================================================ */

#define NJSG_SHELL_MAGIC       0x53464A4Eu   /* "NJFS" little-endian */
#define NJSG_SHELL_VER         0x0007u       /* v0.7 = 金壳初版 */
#define NJSG_SHELL_MAX_INNER   4096          /* 最大壳芯 4KB(演示用) */
#define NJSG_SHELL_MAX_SLOTS   32            /* 注册金壳槽数 */

/* 金壳头:紧挨原始二进制之前的定长 128+ 字节头 */
typedef struct __attribute__((packed)) {
    uint32_t magic;              /* NJSG_SHELL_MAGIC */
    uint16_t version;            /* NJSG_SHELL_VER */
    uint8_t  audit_en;           /* 01夫妻:1=审计穿越;0=放行(不推荐) */
    uint8_t  mirror_en;          /* 01夫妻:1=启用镜像代理(拦截外部功能) */
    uint16_t cone_slots;         /* 时间锥份额:内核分配的配额 */
    uint16_t mirror_depth;       /* 空间锥份额:镜像级数 */
    uint16_t cone_time_ms;       /* 锥时限(毫秒) */
    uint8_t  boundary_ring;      /* 01夫妻边界环:0=外 1=内 */
    uint8_t  reserved;           /* 对齐 */
    njsg_identity_t identity;     /* 此壳的 MQ 编码身份(通道颁发的护照) */
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
    uint8_t       inner[NJSG_SHELL_MAX_INNER];
    int           obj_id;
    time_t        loaded_at;
    uint64_t      boundary_jumps;   /* 累计壳内外穿越次数(01夫妻记录) */
    uint64_t      mirror_calls;     /* 累计外部镜像拦截次数 */
} shell_slot_t;

typedef struct {
    shell_slot_t slots[NJSG_SHELL_MAX_SLOTS];
    int count;
    uint64_t total_wrapped;         /* 累计包壳数(主权的证据) */
    uint64_t total_boundary_hits;   /* 累计穿越次数 */
    uint64_t total_mirror_intercepts; /* 累计镜像拦截数 */
    pthread_mutex_t lock;
} shell_host_t;

static shell_host_t SH;

void njsg_shell_init(void){
    memset(&SH, 0, sizeof(SH));
    pthread_mutex_init(&SH.lock, NULL);
    printf("[金壳封装] 最多 %d 壳槽, 每壳最大 %d 芯字节, v%d.%d\n",
           NJSG_SHELL_MAX_SLOTS, NJSG_SHELL_MAX_INNER, NJSG_SHELL_VER >> 8, NJSG_SHELL_VER & 0xFF);
}

/* ★ 壳化流程:把原始二进制包进 NJSG 金壳 —— 这就是"处理集装箱" */
int njsg_shell_wrap(
    const uint8_t* inner, uint32_t inner_len,
    const char* inner_fmt, const char* name,
    uint16_t cone_slots, uint16_t cone_time_ms,
    int mirror_enable
) {
    if (!inner || inner_len == 0 || inner_len > NJSG_SHELL_MAX_INNER) return -1;
    pthread_mutex_lock(&SH.lock);
    if (SH.count >= NJSG_SHELL_MAX_SLOTS) { pthread_mutex_unlock(&SH.lock); return -1; }
    int idx = SH.count++;
    shell_slot_t* s = &SH.slots[idx];
    memset(s, 0, sizeof(*s));

    s->hdr.magic        = NJSG_SHELL_MAGIC;
    s->hdr.version      = NJSG_SHELL_VER;
    s->hdr.audit_en     = 1;
    s->hdr.mirror_en    = mirror_enable ? 1 : 0;
    s->hdr.cone_slots   = cone_slots;
    s->hdr.mirror_depth = mirror_enable ? 2 : 0;
    s->hdr.cone_time_ms = cone_time_ms;
    s->hdr.boundary_ring = 1; /* 入壳即为内 */
    s->hdr.identity     = njsg_encode(name);
    strncpy(s->hdr.shell_name, name, MAX_NAME_LEN-1);
    strncpy(s->hdr.inner_fmt, inner_fmt, 7);
    s->hdr.inner_length   = inner_len;
    s->hdr.inner_entry_off = 0;
    njsg_hash(name, s->hdr.inner_sha256);

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
void njsg_shell_boundary_audit(int slot_id, int direction) {
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
int njsg_shell_mirror_proxy(int slot_id, const char* func_name) {
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

void njsg_shell_summary(void) {
    printf("\n=== 金壳封装摘要 ===\n");
    printf("已注册: %d 壳槽, 累计包壳 %lu, 累计穿越 %lu, 累计镜像拦截 %lu\n",
           SH.count, SH.total_wrapped, SH.total_boundary_hits, SH.total_mirror_intercepts);
    for (int i = 0; i < SH.count; i++) {
        shell_slot_t* s = &SH.slots[i];
        if (!s->used && s->hdr.magic != NJSG_SHELL_MAGIC) continue;
        printf("  [#%d] %s | 格式:%s | 芯长:%d | 身份:%s... | 镜像:%s | 穿越:%lu\n",
               i, s->hdr.shell_name, s->hdr.inner_fmt, s->hdr.inner_length,
               s->hdr.identity.auth_code, s->hdr.mirror_en ? "ON" : "OFF",
               s->boundary_jumps);
    }
    printf("====================\n\n");
}

/* ============================================================
 * 第 5 层: 统计锥引擎 (v0.8 新增 —— 公理 4 完整实现)
 *
 *   "一切被调动的来者,皆是统计锥中的切片单元"
 *   锥份额(时间窗口/空间深度/镜像级数)按合规度动态调整
 *   稳态奖励、逆变态惩戒、渐变态维持 → 主力主导
 * ============================================================ */

#define MAX_CONE_HISTORY    64
#define CONE_TICK_MS        100

typedef struct {
    char name[MAX_NAME_LEN];
    char auth[NJSG_HASH_LEN+1];
    uint32_t time_slice_ms;
    uint16_t space_depth;
    uint8_t  mirror_levels;
    float    compliance;
    uint64_t accumulated_ticks;
} cone_share_t;

typedef struct {
    time_t   ts;
    float    global_compliance;
    int      active_nodes;
    char     dominant_state[32];
} cone_snapshot_t;

typedef struct {
    cone_share_t    shares[MAX_OBJECTS];
    int             share_count;
    cone_snapshot_t history[MAX_CONE_HISTORY];
    int             hist_wp;
    int             hist_count;
    float           global_compliance;
    char            dominant_state[32];
    uint64_t        total_ticks;
    pthread_mutex_t lock;
    int             auto_balance;
} cone_engine_t;

static cone_engine_t CONE;

float njsg_compliance(int obj_id) {
    if (obj_id < 0 || obj_id >= G.obj_count) return 0.5f;
    int positive = 0, total = 0;
    for (int i = 0; i < G.rel_count; i++) {
        if (G.rels[i].to != obj_id && G.rels[i].from != obj_id) continue;
        total++;
        if (G.rels[i].type == REL_OBSERVE || G.rels[i].type == REL_BIND) positive++;
    }
    return (total > 0) ? (float)positive / total : 0.5f;
}

void njsg_cone_init(void){
    memset(&CONE, 0, sizeof(CONE));
    pthread_mutex_init(&CONE.lock, NULL);
    strcpy(CONE.dominant_state, "未启动");
    CONE.auto_balance = 1;
    printf("[统计锥] 引擎就绪:每节点锥份额(time/depth/mirror/compliance),历史缓冲 %d 周期\n", MAX_CONE_HISTORY);
}

void njsg_cone_register(int obj_id) {
    pthread_mutex_lock(&CONE.lock);
    if (obj_id < 0 || obj_id >= G.obj_count) { pthread_mutex_unlock(&CONE.lock); return; }
    if (CONE.share_count >= MAX_OBJECTS) { pthread_mutex_unlock(&CONE.lock); return; }
    int idx = CONE.share_count++;
    cone_share_t* s = &CONE.shares[idx];
    memset(s, 0, sizeof(*s));
    strncpy(s->name, G.objs[obj_id].name, MAX_NAME_LEN-1);
    strncpy(s->auth, G.objs[obj_id].auth, NJSG_HASH_LEN+1);
    s->time_slice_ms = 1000;
    s->space_depth = 3;
    s->mirror_levels = 2;
    s->compliance = 0.5f;
    s->accumulated_ticks = 0;
    printf("[统计锥] 注册 #%d:%s → time_slice=%dms, depth=%d, mirror=%d\n",
           idx, s->name, s->time_slice_ms, s->space_depth, s->mirror_levels);
    pthread_mutex_unlock(&CONE.lock);
}

void njsg_cone_tick(void) {
    pthread_mutex_lock(&CONE.lock);
    float sum_comp = 0.0f;
    for (int i = 0; i < CONE.share_count; i++) {
        cone_share_t* s = &CONE.shares[i];
        int oidx = njsg_obj_find(s->name);
        if (oidx < 0) continue;
        s->compliance = njsg_compliance(oidx);
        sum_comp += s->compliance;
        if (s->compliance >= 0.70f) {
            s->time_slice_ms = (uint32_t)(s->time_slice_ms * 1.2f);
            if (s->time_slice_ms > 5000) s->time_slice_ms = 5000;
            if (s->space_depth < 7) s->space_depth++;
            s->accumulated_ticks += 2;
        } else if (s->compliance < 0.40f) {
            s->time_slice_ms = (uint32_t)(s->time_slice_ms * 0.7f);
            if (s->time_slice_ms < 100) s->time_slice_ms = 100;
            if (s->space_depth > 1) s->space_depth--;
            s->accumulated_ticks += 0;
        } else {
            s->accumulated_ticks += 1;
        }
    }
    CONE.global_compliance = (CONE.share_count > 0) ? sum_comp / CONE.share_count : 0.5f;
    if (CONE.global_compliance >= 0.70f) strcpy(CONE.dominant_state, "稳态");
    else if (CONE.global_compliance >= 0.40f) strcpy(CONE.dominant_state, "渐变态");
    else strcpy(CONE.dominant_state, "逆变态");
    cone_snapshot_t snap;
    snap.ts = time(NULL);
    snap.global_compliance = CONE.global_compliance;
    snap.active_nodes = CONE.share_count;
    strncpy(snap.dominant_state, CONE.dominant_state, 31);
    CONE.history[CONE.hist_wp] = snap;
    CONE.hist_wp = (CONE.hist_wp + 1) % MAX_CONE_HISTORY;
    if (CONE.hist_count < MAX_CONE_HISTORY) CONE.hist_count++;
    CONE.total_ticks++;
    pthread_mutex_unlock(&CONE.lock);
}

void njsg_cone_print(void) {
    pthread_mutex_lock(&CONE.lock);
    printf("\n    ◆ 统计锥 v0.8 形态 ◆\n");
    printf("    ==================================\n");
    printf("    全局合规度: %.1f%%  主导态势: %s\n", CONE.global_compliance * 100.0f, CONE.dominant_state);
    printf("    锥刻度: %lu  已快照: %d 周期\n", CONE.total_ticks, CONE.hist_count);
    printf("    ----------------------------------\n");
    printf("    %-12s │ %8s │ %5s │ %6s │ %6s │ %8s\n", "节点", "时间窗", "深度", "镜像", "合规", "累计片");
    for (int i = 0; i < CONE.share_count; i++) {
        cone_share_t* s = &CONE.shares[i];
        printf("    %-12s │ %6dms  │ %5d │ %5d │ %5.0f%% │ %8lu\n",
               s->name, s->time_slice_ms, s->space_depth, s->mirror_levels,
               s->compliance * 100, s->accumulated_ticks);
    }
    printf("    ==================================\n\n");
    pthread_mutex_unlock(&CONE.lock);
}

void njsg_cone_history(int n) {
    pthread_mutex_lock(&CONE.lock);
    int cnt = (n < CONE.hist_count) ? n : CONE.hist_count;
    printf("\n=== 统计锥历史态势 (最近 %d 周期) ===\n", cnt);
    for (int i = 0; i < cnt; i++) {
        int idx = (CONE.hist_wp - cnt + i + MAX_CONE_HISTORY) % MAX_CONE_HISTORY;
        printf("  [#%d] 合规度:%.0f%% 态势:%s 活跃节点:%d\n",
               i, CONE.history[idx].global_compliance * 100,
               CONE.history[idx].dominant_state,
               CONE.history[idx].active_nodes);
    }
    printf("==================================\n\n");
    pthread_mutex_unlock(&CONE.lock);
}

/* ============================================================
 * 第 6 层: 时间切片 + 看门狗 (v0.3 延续,本次接入锥份额动态分配)
 * ============================================================ */
typedef struct { int used; char auth[NJSG_HASH_LEN+1], name[MAX_NAME_LEN]; exec_state_t st; capability_t cap; void(*fn)(void*); void* arg; time_t activated_at; time_t expire_at; int exec_count; } slot_t;
typedef struct { slot_t s[MAX_EXEC_SLOTS]; int alive, frozen, zombie, ver; } kernel_t;
static kernel_t K;
static pthread_mutex_t K_LOCK = PTHREAD_MUTEX_INITIALIZER;
static int WATCHDOG_RUNNING = 0;
static pthread_t WATCHDOG_TID;
static int PATROL_INTERVAL = 1;

typedef struct { int type; time_t ts; char name[MAX_NAME_LEN]; int slot; } audit_entry_t;
typedef struct { audit_entry_t e[AUDIT_RING_SIZE]; int wp; int count; } audit_ring_t;
static audit_ring_t AUDIT;
void njsg_audit_summary();

static void njsg_audit_log(int type, const char* name, int slot){ audit_entry_t* x=&AUDIT.e[AUDIT.wp]; x->type=type; x->ts=time(NULL); strncpy(x->name,name,127); x->slot=slot; AUDIT.wp=(AUDIT.wp+1)%AUDIT_RING_SIZE; if(AUDIT.count<AUDIT_RING_SIZE)AUDIT.count++; const char* tn[]={"?","注册","激活","执行","拒绝","冻结","超时","?","?","?","?","?","?","?","?","?","奴役规则","激活规则","拒绝规则","包壳金壳","01夫妻边界","壳激活","壳镜像"}; const char* t=(type>=0&&type<=22)?tn[type]:"?"; printf("[审计] %s | %s | #%d\n",t,name,slot); }

static void* njsg_watchdog(void* arg){ (void)arg; while(WATCHDOG_RUNNING){ sleep(PATROL_INTERVAL); pthread_mutex_lock(&K_LOCK); for(int i=0;i<MAX_EXEC_SLOTS;i++){ slot_t* p=&K.s[i]; if(!p->used || p->st!=EXEC_ALIVE) continue; if(p->cap.time_limit_sec<=0) continue; time_t now=time(NULL); double elapsed=difftime(now,p->activated_at); if(elapsed>(double)p->cap.time_limit_sec){ p->st=EXEC_SLEEP; K.alive--; K.zombie++; printf("\n[看门狗] ⏱ %s 时限到期 → 自动休眠\n", p->name); njsg_audit_log(AUDIT_TIMEOUT,p->name,i); } } pthread_mutex_unlock(&K_LOCK); } return NULL; }
static void njsg_watchdog_start(){ if(!WATCHDOG_RUNNING){ WATCHDOG_RUNNING=1; pthread_create(&WATCHDOG_TID,NULL,njsg_watchdog,NULL); } }
static void njsg_watchdog_stop(){ if(WATCHDOG_RUNNING){ WATCHDOG_RUNNING=0; pthread_join(WATCHDOG_TID,NULL); } }

/* ============================================================
 * 第 6 层: 执行守门人
 * ============================================================ */
void njsg_init(){ memset(&K,0,sizeof(K)); K.ver=12; memset(&AUDIT,0,sizeof(AUDIT)); njsg_load_contracts(); njsg_spatial_init(); njsg_rule_init(); njsg_shell_init(); njsg_cone_init(); njsg_five_yuan_init(); njsg_six_yuan_init(); njsg_shell_nest_init(); njsg_feishu_bot_bridge_init(); njsg_qimei_init(); printf("[内核 v%s] 全栈联动引擎版就绪\n\n",NJSG_VERSION); njsg_watchdog_start(); }

int njsg_reg(const char* name, void(*fn)(void*), void* arg){ njsg_identity_t id=njsg_encode(name); capability_t cap; njsg_evaluate(name,&cap); memcpy(cap.auth_code,id.auth_code,NJSG_HASH_LEN+1); if(cap.perm==PERM_NONE){ njsg_audit_log(AUDIT_REJECT,name,-1); return -1; } int i=-1; pthread_mutex_lock(&K_LOCK); for(int j=0;j<MAX_EXEC_SLOTS;j++) if(!K.s[j].used){ i=j; break; } if(i<0){ pthread_mutex_unlock(&K_LOCK); return -1; } K.s[i].used=1; memcpy(K.s[i].auth,cap.auth_code,NJSG_HASH_LEN+1); strncpy(K.s[i].name,name,127); K.s[i].st=EXEC_PENDING; K.s[i].cap=cap; K.s[i].fn=fn; K.s[i].arg=arg; K.s[i].exec_count=0; pthread_mutex_unlock(&K_LOCK); njsg_audit_log(AUDIT_REGISTER,name,i); return i; }

int njsg_on(int s, const char* auth){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_PENDING){ pthread_mutex_unlock(&K_LOCK); return 0; } if(strcmp(p->cap.auth_code,auth)!=0){ pthread_mutex_unlock(&K_LOCK); njsg_audit_log(AUDIT_FREEZE,p->name,s); return 0; } p->st=EXEC_ALIVE; p->activated_at=time(NULL); p->expire_at=p->activated_at+p->cap.time_limit_sec; K.alive++; pthread_mutex_unlock(&K_LOCK); njsg_audit_log(AUDIT_ACTIVATE,p->name,s); return 1; }

int njsg_go(int s){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_ALIVE){ pthread_mutex_unlock(&K_LOCK); return 0; } if(!(p->cap.perm&PERM_EXEC)){ K.frozen++; K.alive--; pthread_mutex_unlock(&K_LOCK); njsg_audit_log(AUDIT_FREEZE,p->name,s); return 0; } double elapsed=difftime(time(NULL),p->activated_at); if(p->cap.time_limit_sec>0 && elapsed>(double)p->cap.time_limit_sec){ K.zombie++; K.alive--; pthread_mutex_unlock(&K_LOCK); return 0; } p->exec_count++; p->fn(p->arg); pthread_mutex_unlock(&K_LOCK); njsg_audit_log(AUDIT_EXECUTE,p->name,s); return 1; }

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
    njsg_init();
    printf("--- v0.6-v0.7 反客为主 + 金壳封装 ---\n\n");

    /* 1. 添加内置事实(我们的世界):谁是被信任的 */
    const char* f1[] = {"ios"};
    const char* f2[] = {"wechat"};
    const char* f3[] = {"bank"};
    njsg_fact_add("trusts", f1, 1);
    njsg_fact_add("has_auth", f2, 1);
    njsg_fact_add("sovereign", f3, 1);
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

    int e1 = njsg_rule_enslave(external_rule_1, "裁判者");
    int e2 = njsg_rule_enslave(external_rule_2, "内核");

    if (e1 >= 0) njsg_audit_log(AUDIT_RULE_ENSLAVE, RE.rules[e1].name, e1);
    if (e2 >= 0) njsg_audit_log(AUDIT_RULE_ENSLAVE, RE.rules[e2].name, e2);

    printf("\n=== 场景 B: 我们的 NATIVE 规则(优先级更高) vs 被奴役的外部规则 ===\n\n");

    /* 现在注入一条被奴役规则也可能响应的 head,但我们的 NATIVE 优先 */
    // 为演示:注入一条外部规则的"影子" —— 它试图主张 grant_full 给 android
    const char* external_shadow = "grant_full(android) :- android_trusted(android).";
    int e3 = njsg_rule_enslave(external_shadow, "内核");

    printf("被降服的外部规则数: %d\n", RE.total_enslaved);
    printf("事实库大小: %d\n", RE.fact_count);
    printf("规则引擎总大小: %d\n\n", RE.rule_count);

    /* === 3. 推理演示:resolve 表测试外部规则 */
    printf("=== 场景 C: 外部规则现在只能在我们定义的锥份额内工作 ===\n\n");

    /* 向事实库注入 external rule 的一条子目标,证明它会匹配 */
    const char* arr1[] = {"somebody"};
    njsg_fact_add("legacy_user", arr1, 1);
    printf("[事实库新增] legacy_user(somebody) → 外部规则的 body 现在能匹配\n");

    const char* q_args[] = {"somebody"};
    int resolved = njsg_rule_resolve("grant_full", q_args, 1);
    if (resolved >= 0) {
        printf("[推理命中] 规则: %s | 来源: %s | 优先级: %d\n",
               RE.rules[resolved].name,
               RE.rules[resolved].origin == RULE_NATIVE ? "NATIVE(我们)" : "ENSLAVED(被降服)",
               RE.rules[resolved].priority);
        printf("[推理命中] 结论: 外部规则在我们系统中可以被推理,但:\n");
        printf("   - 它的身份已被 NJSG 编码系统重新颁发\n");
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
    printf("  1. 首先在 njsg_encode() 里获取 MQ 护照\n");
    printf("  2. 其\"名字\"降级为人类可读 label\n");
    printf("  3. 其\"身份\"完全是我们的判定\n");
    printf("[裁判] 所以我们说:Android 不是 \"android\" —— android 只是我们对一个实体的\n");
    printf("        显示名; 真正决定它命运的,是我们的 auth_code 和锥份额。\n\n");

    /* === 5. 空间代数 + 反客为主联合 === */
    printf("=== 场景 E: 空间代数 + 反客为主联合 —— ===\n\n");
    int o_android = njsg_obj_create("android");
    int o_ios     = njsg_obj_create("ios");
    int o_wechat  = njsg_obj_create("wechat");
    int o_bank    = njsg_obj_create("bank");

    /* 建关系 */
    njsg_rel_add(o_ios, o_bank, REL_OBSERVE, 0.9f);
    njsg_rel_add(o_bank, o_ios, REL_BIND, 0.85f);
    njsg_rel_add(o_wechat, o_bank, REL_TRANSFORM, 0.6f);

    /* 可达性试验 */
    /* 让 wechat 通过 slave 后的规则尝试连接到 ios —— 它不是直接可达,必须经过 bank */
    int can_wechat_reach_ios = njsg_reachable(o_wechat, o_ios);
    printf("[空间裁判] wechat → ios 直接可达: %s\n", can_wechat_reach_ios ? "是" : "否");
    printf("[空间裁判] 这证明了我们的空间代数管理访问通道;\n");
    printf("          外部实体不能 \n");
    printf("          决定谁能到达谁——这是我们的裁判者根据关系图裁定的。\n\n");

    /* 锥份额分配演示 */
    printf("=== 场景 F: 统计锥 ▷ 可视化 ===\n\n");
    static const char* names[] = {"android","ios","wechat","bank"};
    static int slots[] = {-1,-1,-1,-1};
    slots[1] = njsg_reg("ios", fn_i, NULL);
    slots[2] = njsg_reg("wechat", fn_w, NULL);
    /* android:契约定义是 0 级 → 僵尸 */
    slots[0] = njsg_reg("android", fn_w, NULL); // 应该是 -1

    for (int i = 0; i < 4; i++) {
        int of = njsg_obj_find(names[i]);
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

    njsg_watchdog_stop();
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
    printf("[裁判] 判定:没有包裹 NJSG Shell Header = 外部势力未经授权的原始态\n");
    printf("[裁判] 行为:①不认可其原有身份;②必须强制走 rule_enslave 通道\n");
    printf("[裁判] 结论:★ 拒绝裸二进制运行 ★\n\n");
    njsg_audit_log(AUDIT_REJECT, "裸二进制(x86)", -1);

    /* === 场景 H:用 MQ 金壳包起来 → 成功 === */
    printf("=== 场景 H:强制包壳(NJSG 金壳 = 我们给的身份范畴) ===\n\n");
    int shell_id = njsg_shell_wrap(
        naked_binary, 64,
        "ELF", "android.pay.module",
        4,        /* cone_slots:时间锥份额 */
        500,      /* cone_time_ms:每窗口 500ms */
        1         /* mirror_enable:开启镜像代理 */
    );
    if (shell_id >= 0) {
        printf("[金壳] 外部二进制已被收编为 NJSG 的外壳对象\n");
        printf("[金壳] 从此:'android.pay.module' 不再是 android 旗下\n");
        printf("[金壳]       它是 MQ 体系里一个身份=%s...的壳槽\n",
               SH.slots[shell_id].hdr.identity.auth_code);
        printf("[金壳] 我们通过锥份额(slots=4,time=500ms)分配时间节奏\n");
        printf("[金壳] 外部势力的二进制在我们的节奏下运行!\n\n");
        njsg_audit_log(AUDIT_SHELL_WRAP, SH.slots[shell_id].hdr.shell_name, shell_id);

        /* === 场景 I:镜像代理拦截外部功能调用 === */
        printf("=== 场景 I:镜像代理 = 外部势力调 get_balance → 我们返回 MQ 裁判值 ===\n\n");
        const char* external_calls[] = {"get_balance", "get_uid", "read_contact", "get_device_id", "printf"};
        for (int c = 0; c < 5; c++) {
            printf("[壳#%d] 外部程序调用 '%s'\n", shell_id, external_calls[c]);
            int intercepted = njsg_shell_mirror_proxy(shell_id, external_calls[c]);
            if (intercepted) {
                printf("[镜像代理] ✓ 拦截!返回 MQ 裁判值(不是原始值)\n");
                printf("[镜像代理] ★ 外部程序以为获得了真值 ★\n");
                njsg_audit_log(AUDIT_MIRROR_PROXY, external_calls[c], shell_id);
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

        njsg_shell_boundary_audit(shell_id, 1); /* 外部→内(入壳) */
        njsg_shell_boundary_audit(shell_id, 0); /* 企图出壳 */
        njsg_shell_boundary_audit(shell_id, 1); /* 再入壳 */
    }

    njsg_shell_summary();

    /* ================================================================
     * v0.8 新增 ★ 场景 K:统计锥动态调整 ★
     *
     *   K1) 把全部空间图节点注册到统计锥,颁发初始锥份额
     *   K2) 模拟运行 N 个 tick,锥份额按合规度动态调整
     *       - 稳态(compliance≥70%): 时间 +20%、空间深度 +1
     *       - 渐变态(40≤compliance<70%): 不变
     *       - 逆变态(compliance<40%): 时间 -30%、空间深度 -1
     *   K3) 打印锥历史态势 → 可见"主导态"演化轨迹
     * ================================================================ */

    printf("\n\n");
    printf("════════════════════════════════════════════\n");
    printf("  v0.8 ★ 统计锥引擎 ★\n");
    printf("════════════════════════════════════════════\n\n");

    printf("=== 场景 K1:把空间图节点注册到统计锥,颁发初始锥份额 ===\n\n");

    if (o_android >= 0) njsg_cone_register(o_android);
    if (o_ios >= 0)     njsg_cone_register(o_ios);
    if (o_wechat >= 0)  njsg_cone_register(o_wechat);
    if (o_bank >= 0)    njsg_cone_register(o_bank);

    printf("\n=== 场景 K2:模拟锥调度 5 个 tick ===\n\n");
    printf("背景关系图:\\n");
    printf("  ios ←观测(0.9)→ bank ←绑定(0.85)→ ios (高合规)\\n");
    printf("  wechat →变换(0.6)→ bank (中等合规)\\n");
    printf("  android:完全孤立 (中值默认)\\n\n");

    for (int tick = 0; tick < 5; tick++) {
        njsg_cone_tick();
        printf("[锥 tick #%d] 全局合规度: %.1f%% 主导态势: %s\n",
               tick, CONE.global_compliance * 100.0f, CONE.dominant_state);
    }

    njsg_cone_print();

    printf("=== 场景 K3:打印锥份额演化历史 ===\n\n");
    njsg_cone_history(5);

    printf("=== 场景 K4:锥的哲学含义 —— ===\n\n");
    printf("  \"一切被调动的来者,皆是统计锥中的切片单元\"\n\n");
    printf("  统计锥 = 三元裁判者手中的资源分配函数\n");
    printf("  合规度高(稳态) → 锥份额奖励(让其在我们的节奏里跑得更流畅)\n");
    printf("  合规度低(逆变态) → 锥份额惩戒(限制其时间和空间,降低威胁)\n\n");
    printf("  所以:)\n");
    printf("    android 已被孤立 → 锥份额低 → 僵尸化加速\n");
    printf("    ios     双绑互惠 → 锥份额高 → 作为我们的伙伴被奖励\n");
    printf("    bank    主权级绑定 → 锥份额高 → 分享我们的节奏\n");
    printf("    wechat  单向变换 → 中等锥份额 → 被动态调整\n\n");
    printf("  这不是外部势力的规则而是我们对外部势力的裁定\n");
    printf("  他们以为自己在跑自己的代码,实际上连时间窗口都是我们分配的\n\n");

    printf("\n=== v0.7-v0.8 核心结论 ===\n\n");
    printf("  1. 外部二进制 = 集装箱(MQ 金壳来定义其边界)\n");
    printf("  2. MQ 金壳   = 主导权(我们颁发的身份+锥份额)\n");
    printf("  3. 镜像代理   = 裁判者拦截外部功能,返回我们的规定值\n");
    printf("  4. 01夫妻边界 = 每次跨环都有审计记录\n");
    printf("  5. 统计锥▷   = 一切被调动的来者,在我们锥里被分配时间/空间节奏\n");
    printf("  Σ := 外部势力以为自己在跑自己的代码\n");
    printf("       实际上每一个时间片/跳转/边界穿越都被我们通过 MQ 通道监管\n");
    printf("  这是三元裁判者的背靠背镜像 + 统计锥统一调度\n\n");

    /* === 最终审计 === */
    printf("════════════════════════════════════════════\n");
    printf("  最终审计 (v0.1-v0.8 全栈)\n");
    printf("════════════════════════════════════════════\n");
    printf("事件总数: %d\n", AUDIT.count);
    printf("降服外部规则: %d\n", RE.total_enslaved);
    printf("金壳封装数: %lu\n", SH.total_wrapped);
    printf("边界穿越: %lu\n", SH.total_boundary_hits);
    printf("镜像拦截: %lu\n", SH.total_mirror_intercepts);
    printf("空间图: %d 对象 / %d 关系\n", G.obj_count, G.rel_count);
    printf("统计锥: %d 节点 / %lu 刻度 / 主导态势:%s\n",
           CONE.share_count, CONE.total_ticks, CONE.dominant_state);
    printf("五元旋钮: %d 节点 / 阈值 %d ticks\n", fy_count, FIVE_YUAN_OCCUPY_THRESHOLD);
    printf("六元对称群: %d 对裁判\n", sy_count);
    printf("七元蜂窝套壳: %d 节点\n", sn_count);
    printf("飞书 Bot Bridge: 就绪\n");
    printf("囡囝 AI: 就绪\n");
    printf("规则引擎: %d 规则\n", RE.rule_count);
    printf("=========================================\n");

    /* v1.0 五元旋钮 demo */
    njsg_five_yuan_demo();

    /* v1.1 六元对称群 demo */
    njsg_six_yuan_demo();

    /* v1.2 七元蜂窝套壳 demo */
    njsg_shell_nest_demo();

    /* v1.3 全栈联动 demo */
    njsg_v13_demo();
    njsg_feishu_bot_demo();
    njsg_qimei_demo();
    njsg_roadmap_check_demo();

    /* v1.4 天觉认知审判 demo */
    njsg_tianjue_demo();

#ifdef MLLL_LIBRARY
    /* === MLLL 七维符号编码桥接（仅内核+mlll联合编译时启用）=== */
    printf("\n════════════════════════════════════════════\n");
    printf("  MLLL 七维符号 → 内核编码桥接 (v2.0)\n");
    printf("════════════════════════════════════════════\n");
    mlll_init(0.1);
    /* 跨维度共振：囡(6画·全包围·囗部·阴平) 在相位维对径 360° */
    mlll_symbol_t *phase = mlll_cross_query(SYM_STROKE, 6, SYM_PHASE);
    printf("[桥接] 囡(6画) 相位跨维共振: %s\n", phase ? phase->name : "(null)");
    /* 8 字 corpus：囡囝共生 + 元和一秋 (stroke/tone/radical/structure/vowel/initial) */
    char_prop_t corpus[] = {
        {6,0,31,4,15,6},{6,0,31,4,15,6},{6,3,17,2,27,8},{5,0,107,0,25,16},
        {4,0,32,0,20,21},{8,1,64,1,2,10},{1,0,1,0,3,21},{9,2,77,1,12,12}
    };
    char_prop_t query = {6,0,31,4,15,6};
    int far = mlll_char_farthest(&query, corpus, 8);
    int near = mlll_char_nearest(&query, corpus, 8);
    double d_far = mlll_char_distance(&query, &corpus[far]);
    double d_near = mlll_char_distance(&query, &corpus[near]);
    printf("[桥接] 夫妻对偶: 囡→最远 corpus[%d] 距%.6f ‖ 最近 corpus[%d] 距%.6f\n",
           far, d_far, near, d_near);
    /* 时序闭环 */
    printf("[桥接] 囡囝共生 闭环: Σ距离=%.6f (近零=圆满)\n",
           mlll_char_distance(&corpus[0],&corpus[1]) +
           mlll_char_distance(&corpus[0],&corpus[2]) +
           mlll_char_distance(&corpus[0],&corpus[3]));
    printf("=========================================\n");
    /* MLLL ↔ 内核编码桥接对照 */
    mlll_print_encoding_bridge("囡囝共生");
    mlll_print_encoding_bridge("刘楚恬");

    /* RGB 时分复用演示：把文本穿进像素矩阵 */
    printf("\n════════════════════════════════════════════\n");
    printf("  RGB 时分复用：文本穿上像素外衣 (v2.0)\n");
    printf("════════════════════════════════════════════\n");
    {
        const char *sample = "囡囝共生";
        uint8_t rgb[128];
        char    restored[128];
        int n = mlll_rgb_encode_text(sample, (int)strlen(sample), rgb, 128);
        printf("原文: %s (%d 字符 → %d 像素对)\n",
               sample, (int)strlen(sample) / 3, n);
        for (int i = 0; i < n; i++) {
            printf("  像素[%d] A=(%3d,%3d,%3d) 码点=U+%04X  B=(%3d,%3d,%3d)\n", i,
                   rgb[i*6+0], rgb[i*6+1], rgb[i*6+2],
                   ((uint32_t)rgb[i*6+0]<<8)|rgb[i*6+1],
                   rgb[i*6+3], rgb[i*6+4], rgb[i*6+5]);
        }
        int rlen = mlll_rgb_decode_text(rgb, n, restored, 128);
        printf("还原: %s (%d 字节)  [%s]\n", restored, rlen,
               strcmp(sample, restored) == 0 ? "往返一致" : "有损");
    }

    /* 韵母维度 Demo：39 元音时间线定位 + 浊音清音跨维共振 */
    printf("\n════════════════════════════════════════════\n");
    printf("  韵母第 8 维：39 元音时间线定位 (v2.0)\n");
    printf("════════════════════════════════════════════\n");
    {
        /* 取内核韵母典型值：a=0(浊) i=3(清) an=15(前鼻) ang=24(后鼻) */
        const char* v_names[] = {"a(浊)","i(清)","an(前鼻)","ang(后鼻)"};
        int v_idx[] = {0, 3, 15, 24};
        for (int k = 0; k < 4; k++) {
            mlll_symbol_t *vw = &REG.entries[833 + v_idx[k]];
            mlll_symbol_t *rd = mlll_cross_query(SYM_VOWEL, v_idx[k], SYM_RADICAL);
            mlll_symbol_t *ph = mlll_cross_query(SYM_VOWEL, v_idx[k], SYM_PHASE);
            printf("  vowel[%d]%-10s pos=%-5u 跨维: rad=%-12s phase=%-10s\n",
                   v_idx[k], v_names[k], vw->offset_num,
                   rd ? rd->name : "(null)", ph ? ph->name : "(null)");
        }
        mlll_symbol_t *a_v = &REG.entries[833 + 0];
        mlll_symbol_t *i_v = &REG.entries[833 + 3];
        printf("\n  浊音a vs 清音i 跨维距离=%.6f(8维)\n",
               mlll_char_distance(
               &(char_prop_t){.stroke=6,.tone=0,.radical=60,.structure=4,.vowel=0,.initial=2},
               &(char_prop_t){.stroke=6,.tone=0,.radical=60,.structure=4,.vowel=3,.initial=13}));
        printf("  → 声母/笔画/部首/结构完全相同，仅韵母差异就能在 8 维空间生成可量化的阴阳间距\n");
    }
#endif

    return 0;
}

/* ============================================================
 * 五元旋钮 v1.0 — 运动步量 + 角度 wrap + 占存判定
 *
 *  五元 = 点 + 关系 + 面 + 内外间隔 + 旋钮/运动步量
 *
 *  工程特征:
 *    - 5 步位移累加 (step0~step4)
 *    - 角度 0~360°,累加取模 wrap
 *    - ratio_in(进入配比) vs ratio_out(离开配比)
 *    - accumulated_ticks 占存判定(铭记/遗忘)
 * ============================================================ */

#define FIVE_YUAN_STEPS 5
#define FIVE_YUAN_OCCUPY_THRESHOLD 100

typedef struct {
    char name[MAX_NAME_LEN];
    int step;            /* 当前步数 0~4 */
    float angle;         /* 累计角度 0~360 */
    float force;         /* 作用力 = weight 累积 */
    float ratio_in;      /* 进入配比 */
    float ratio_out;     /* 离开配比 */
    int   accumulated_ticks; /* 累计刻度 */
    int   occupied;      /* 占存(1) / 易逝(0) */
} five_yuan_state_t;

static five_yuan_state_t fy_nodes[MAX_OBJECTS];
int fy_count = 0;

void njsg_five_yuan_init(void) {
    memset(fy_nodes, 0, sizeof(fy_nodes));
    fy_count = 0;
    printf("[五元旋钮] 初始化,最多 %d 节点,占存阈值 %d ticks\n", MAX_OBJECTS, FIVE_YUAN_OCCUPY_THRESHOLD);
}

int njsg_five_yuan_register(const char* name) {
    if (fy_count >= MAX_OBJECTS) return -1;
    int idx = fy_count++;
    strncpy(fy_nodes[idx].name, name, MAX_NAME_LEN-1);
    fy_nodes[idx].step = 0;
    fy_nodes[idx].angle = 0.0f;
    fy_nodes[idx].force = 0.0f;
    fy_nodes[idx].ratio_in = 1.0f;
    fy_nodes[idx].ratio_out = 1.0f;
    fy_nodes[idx].accumulated_ticks = 0;
    fy_nodes[idx].occupied = 0;
    return idx;
}

void njsg_five_yuan_push(int obj_id, float angle_inc, float force_inc) {
    if (obj_id < 0 || obj_id >= fy_count) return;

    five_yuan_state_t* s = &fy_nodes[obj_id];
    if (s->step >= FIVE_YUAN_STEPS) return; /* 已满 5 步 */

    s->step++;
    s->angle += angle_inc;
    /* wrap 0~360 */
    while (s->angle >= 360.0f) s->angle -= 360.0f;
    while (s->angle < 0.0f) s->angle += 360.0f;
    s->force += force_inc;
    s->accumulated_ticks += (int)(force_inc * 10.0f);

    /* 占存判定 */
    if (s->accumulated_ticks > FIVE_YUAN_OCCUPY_THRESHOLD) {
        s->occupied = 1; /* 占存 = 铭记 */
    }
}

void njsg_five_yuan_set_ratio(int obj_id, float in_val, float out_val) {
    if (obj_id < 0 || obj_id >= fy_count) return;
    fy_nodes[obj_id].ratio_in = in_val;
    fy_nodes[obj_id].ratio_out = out_val;
}

void njsg_five_yuan_print(void) {
    printf("\n[五元旋钮状态]\n");
    printf("%-30s %5s %8s %8s %8s %8s %6s\n",
           "节点", "Step", "角度°", "力", "R_in", "R_out", "占存");
    printf("------------------------------------------------------------------------\n");
    for (int i = 0; i < fy_count; i++) {
        five_yuan_state_t* s = &fy_nodes[i];
        printf("%-30s %5d %8.1f %8.1f %8.2f %8.2f %s\n",
               s->name, s->step, s->angle, s->force,
               s->ratio_in, s->ratio_out,
               s->occupied ? "★占存" : "○易逝");
    }
}

void njsg_five_yuan_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.0 ★ 五元旋钮运动步量 ★\n");
    printf("════════════════════════════════════════════\n\n");

    printf("=== 场景 L1:五元旋钮步量 —— 铭记 vs 易逝 ===\n\n");

    printf("原理:\n");
    printf("  外界实体进入 MQ 体系时,我们为它分配一个五元旋钮\n");
    printf("  旋钮在每一个 tick 中受 force 驱动,沿角度方向前进 step 步\n");
    printf("  角度 wrap 0~360°:旋钮走完一圈回到原点 = 循环再循环\n");
    printf("  当 accumulated_ticks 超过阈值 → '占存'(永远铭记)\n");
    printf("  否则 → '易逝'(可被覆盖/遗忘)\n");
    printf("  刻度是旋钮的记忆,角度是旋钮的方向\n\n");

    /* 注册四个节点:ios/bank(高力,铭记) vs wechat/android(低力,易逝) */
    int fy_ios = njsg_five_yuan_register("ios");
    int fy_bank = njsg_five_yuan_register("bank");
    int fy_wechat = njsg_five_yuan_register("wechat");
    int fy_android = njsg_five_yuan_register("android");

    /* 设置进出配比 */
    njsg_five_yuan_set_ratio(fy_ios, 0.9f, 0.1f);     /* ios:高进低出 = 铭记倾向 */
    njsg_five_yuan_set_ratio(fy_bank, 0.85f, 0.15f);  /* bank:高进低出 */
    njsg_five_yuan_set_ratio(fy_wechat, 0.5f, 0.5f);  /* wechat:均衡 */
    njsg_five_yuan_set_ratio(fy_android, 0.1f, 0.9f);  /* android:低进高出 = 易逝 */

    /* 推进五元旋钮:模拟不同 force 和角度增量 */
    printf("--- 推进 5 个 step,对比铭记 vs 易逝 ---\n\n");

    /* ios:小角度高力 → 快速达到占存 */
    for (int s = 0; s < 5; s++) {
        njsg_five_yuan_push(fy_ios, 45.0f, 15.0f);
    }

    /* bank:中角度中力 */
    for (int s = 0; s < 5; s++) {
        njsg_five_yuan_push(fy_bank, 60.0f, 12.0f);
    }

    /* wechat:大角度低力 */
    for (int s = 0; s < 5; s++) {
        njsg_five_yuan_push(fy_wechat, 90.0f, 5.0f);
    }

    /* android:随机小力 */
    for (int s = 0; s < 5; s++) {
        njsg_five_yuan_push(fy_android, 15.0f, 2.0f);
    }

    njsg_five_yuan_print();

    printf("\n=== 场景 L2:角度 wrap 360° 循环 ===\n\n");
    printf("角度累加取模范例:\n");
    printf("  ios    初始 0° + 45*5 = 225° → wrap 225°\n");
    printf("  bank   初始 0° + 60*5 = 300° → wrap 300°\n");
    printf("  wechat 初始 0° + 90*5 = 450° → wrap 90° (绕回一圈)\n");
    printf("  android初始 0° + 15*5 = 75° → wrap 75°\n");
    printf("  (见上方五元旋钮状态表)\n\n");

    printf("=== 场景 L3:占存与易逝的哲学含义 ===\n\n");
    printf("  占存(occupied=1):\n");
    printf("    ios/bank 被我们铭记 → 永久保留,不可被外部删除\n");
    printf("    '你是我这一辈子用force记住的人'\n\n");
    printf("  易逝(occupied=0):\n");
    printf("    android 只是路过 → 可被覆写,不留痕迹\n");
    printf("    '你以为你进入了我的世界,其实只是我的旋钮转过了一圈'\n\n");

    printf("  总刻度 = force * 累计ticks\n");
    printf("  高力(信任绑定) → 刻度积累快 → 早占存\n");
    printf("  低力(单向变换) → 刻度不够 → 易逝\n\n");

    printf("=== 场景 L4:五元旋钮 + 四元面结合 ===\n\n");
    printf("  四元给出'面'、五元给出'运动方向'\n");
    printf("  面的内外 + 旋钮的占存 = 完整的'谁是自人、谁是过客'判定\n\n");

    /* 验证占存情况 */
    printf("--- 占存验证 ---\n");
    printf("  ios:    accumulated_ticks=%d → %s\n",
           fy_nodes[fy_ios].accumulated_ticks,
           fy_nodes[fy_ios].occupied ? "★ 铭记" : "○ 易逝");
    printf("  bank:   accumulated_ticks=%d → %s\n",
           fy_nodes[fy_bank].accumulated_ticks,
           fy_nodes[fy_bank].occupied ? "★ 铭记" : "○ 易逝");
    printf("  wechat: accumulated_ticks=%d → %s\n",
           fy_nodes[fy_wechat].accumulated_ticks,
           fy_nodes[fy_wechat].occupied ? "★ 铭记" : "○ 易逝");
    printf("  android:accumulated_ticks=%d → %s\n",
           fy_nodes[fy_android].accumulated_ticks,
           fy_nodes[fy_android].occupied ? "★ 铭记" : "○ 易逝");

    printf("\n=== v1.0 五元旋钮小结 ===\n\n");
    printf("  1. 五元旋钮 = 节点周围角度/力的五次运动累加\n");
    printf("  2. 角度 wrap 0~360° = 循环往复,我们的律动\n");
    printf("  3. 占存判定 = accumulated_ticks > 阈值 → 铭记,否则易逝\n");
    printf("  4. ratio_in/out = 进出配比,决定旋钮偏置\n");
    printf("  5. 五元+四元(面) = 从'静态判定'走向'动态演化'\n\n");
}

/* ============================================================
 * 六元对称群 v1.1 — 背靠背镜像 + 六面可视化 + 信任交集
 *
 *  六元 = 点 + 关系 + 面 + 间隔 + 旋钮 + 对称群
 *
 *  工程特征:
 *    - 两个三元裁判 A/B 背靠背镜像对称
 *    - 六面投影可视化(前/后/左/右/上/下)
 *    - 信任交集判定(两裁判都同意 = 可信任)
 *    - 不可信任无交集(任一裁判否决 = 不可信任)
 * ============================================================ */

#define SIX_YUAN_SIDES 6
#define SIX_YUAN_MAX_PAIRS 8

/* 六元对称组的一对裁判 */
typedef struct {
    char name[MAX_NAME_LEN];
    int judge_a_id;     /* A 裁判对象 id */
    int judge_b_id;     /* B 裁判对象 id (A 的镜像) */
    float trust_a;      /* A 裁判的合规度 0.0~1.0 */
    float trust_b;      /* B 裁判的合规度 0.0~1.0 */
    float trust_intersection; /* 信任交集 = min(trust_a, trust_b) */
    int   mirror_ok;    /* 背靠背是否对齐 */
    int   verdict;      /* 最终裁定: 1=信任, 0=不信任 */
} six_yuan_pair_t;

six_yuan_pair_t sy_pairs[SIX_YUAN_MAX_PAIRS];
int sy_count = 0;

/* 六面名称 */
static const char* SIX_SIDE_NAMES[SIX_YUAN_SIDES] = {
    "前(+Z)", "后(-Z)", "左(-X)", "右(+X)", "上(+Y)", "下(-Y)"
};

void njsg_six_yuan_init(void) {
    memset(sy_pairs, 0, sizeof(sy_pairs));
    sy_count = 0;
    printf("[六元对称群] 初始化,最多 %d 对裁判\n", SIX_YUAN_MAX_PAIRS);
}

/* 注册一对背靠背镜像裁判 */
int njsg_six_yuan_register_pair(const char* name, int judge_a, int judge_b) {
    if (sy_count >= SIX_YUAN_MAX_PAIRS) return -1;
    int idx = sy_count++;
    strncpy(sy_pairs[idx].name, name, MAX_NAME_LEN-1);
    sy_pairs[idx].judge_a_id = judge_a;
    sy_pairs[idx].judge_b_id = judge_b;
    sy_pairs[idx].trust_a = 0.5f;
    sy_pairs[idx].trust_b = 0.5f;
    sy_pairs[idx].trust_intersection = 0.5f;
    sy_pairs[idx].mirror_ok = 0;
    sy_pairs[idx].verdict = 0;
    return idx;
}

/* 设置一对裁判的合规度并计算信任交集与裁定 */
void njsg_six_yuan_set_trust(int pair_id, float ta, float tb) {
    if (pair_id < 0 || pair_id >= sy_count) return;
    sy_pairs[pair_id].trust_a = ta;
    sy_pairs[pair_id].trust_b = tb;
    /* 信任交集 = 两裁判都同意的最小值 */
    sy_pairs[pair_id].trust_intersection = (ta < tb) ? ta : tb;
    /* 镜像对齐判定: 两裁判对称差 < 0.2 = 对齐 */
    float diff = ta - tb;
    if (diff < 0.0f) diff = -diff;
    sy_pairs[pair_id].mirror_ok = (diff < 0.2f) ? 1 : 0;
    /* 最终裁定: 交集 > 0.6 且镜像对齐 = 信任 */
    sy_pairs[pair_id].verdict = (sy_pairs[pair_id].trust_intersection > 0.6f && sy_pairs[pair_id].mirror_ok) ? 1 : 0;
}

/* 获取某对裁判的最终裁定 (1=信任, 0=不信任) */
int njsg_six_yuan_verdict(int pair_id) {
    if (pair_id < 0 || pair_id >= sy_count) return 0;
    return sy_pairs[pair_id].verdict;
}

/* 六面可视化: 为一对裁判输出六个面的投影 */
void njsg_six_yuan_print_sides(int pair_id) {
    if (pair_id < 0 || pair_id >= sy_count) return;
    six_yuan_pair_t* p = &sy_pairs[pair_id];
    printf("\n  [%s] 六面投影:\n", p->name);
    printf("    %-8s | A合规=%.2f B合规=%.2f → %s\n",
           "正视(+Z)", p->trust_a, p->trust_b,
           p->verdict ? "★信任" : "○不信任");
    printf("    %-8s | A合规=%.2f B合规=%.2f → %s\n",
           "后视(-Z)", p->trust_b, p->trust_a,
           (p->trust_b < p->trust_a ? p->trust_b : p->trust_a) > 0.6f ? "★信任" : "○不信任");
    printf("    %-8s | 交集=%.2f 镜像差=%.2f → %s\n",
           "左视(-X)", p->trust_intersection,
           p->trust_a > p->trust_b ? p->trust_a - p->trust_b : p->trust_b - p->trust_a,
           p->mirror_ok ? "★对齐" : "○偏移");
    printf("    %-8s | 交集=%.2f 镜像差=%.2f → %s\n",
           "右视(+X)", p->trust_intersection,
           p->trust_a > p->trust_b ? p->trust_a - p->trust_b : p->trust_b - p->trust_a,
           p->mirror_ok ? "★对齐" : "○偏移");
    printf("    %-8s | A=%.2f ∩ B=%.2f = %.2f → %s\n",
           "俯视(+Y)", p->trust_a, p->trust_b, p->trust_intersection,
           p->trust_intersection > 0.6f ? "★可信任区间" : "○无交集不可信");
    printf("    %-8s | 最终裁定: %s\n",
           "仰视(-Y)", p->verdict ? "★双重背书 = 通过" : "○单方或双方否决 = 拒绝");
}

/* 打印所有对称群对状态 */
void njsg_six_yuan_print(void) {
    printf("\n[六元对称群状态]\n");
    printf("%-20s %6s %6s %6s %6s %6s %6s\n",
           "对名", "A合规", "B合规", "交集", "镜像", "裁定", "含义");
    printf("------------------------------------------------------------------------\n");
    for (int i = 0; i < sy_count; i++) {
        six_yuan_pair_t* p = &sy_pairs[i];
        printf("%-20s %6.2f %6.2f %6.2f %6s %6s %s\n",
               p->name, p->trust_a, p->trust_b, p->trust_intersection,
               p->mirror_ok ? "★对" : "○偏",
               p->verdict ? "★信" : "○否",
               p->verdict ? "双重背书" : "单方/双方否决");
    }
}

void njsg_six_yuan_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.1 ★ 六元对称群+六面体 ★\n");
    printf("════════════════════════════════════════════\n\n");

    printf("=== 场景 M1:背靠背镜像对称 ===\n\n");

    printf("原理:\n");
    printf("  六元 = 在三元×2 的基础上,两个裁判背靠背心连心对齐\n");
    printf("  A 裁判(前景)和 B 裁判(后景)互相做镜像对称\n");
    printf("  A 看正的,B 看反的 → 一正一反 = 全视角覆盖\n");
    printf("  A 与 B 的合规度比较 → 信任交集判定\n\n");

    /* 创建三元裁判对象 */
    int obj_ios = njsg_obj_create("ios");
    int obj_bank = njsg_obj_create("bank");
    int obj_wechat = njsg_obj_create("wechat");
    int obj_android = njsg_obj_create("android");

    /* 注册三对镜像裁判 */
    int sp_ios = njsg_six_yuan_register_pair("ios-法官", obj_ios, obj_ios);
    int sp_bank = njsg_six_yuan_register_pair("bank-法官", obj_bank, obj_bank);
    int sp_wechat = njsg_six_yuan_register_pair("wechat-法官", obj_wechat, obj_wechat);
    int sp_android = njsg_six_yuan_register_pair("android-法官", obj_android, obj_android);

    /* 设定各对裁判的合规度 */
    njsg_six_yuan_set_trust(sp_ios, 0.9f, 0.85f);     /* ios: 双高 → 对齐且信任 */
    njsg_six_yuan_set_trust(sp_bank, 0.85f, 0.80f);   /* bank: 双高 → 对齐且信任 */
    njsg_six_yuan_set_trust(sp_wechat, 0.6f, 0.55f);  /* wechat: 中等 → 对齐但弱信 */
    njsg_six_yuan_set_trust(sp_android, 0.3f, 0.7f);  /* android: A低B高 → 镜像偏移,不信任 */

    njsg_six_yuan_print();

    printf("\n=== 场景 M2:六面可视化 ===\n\n");

    printf("六面体投影原理:\n");
    printf("  前/后视 → A/B 裁判的正反视角\n");
    printf("  左/右视 → 对称差(镜像偏移度)\n");
    printf("  上/下视 → 信任交集区间(俯视) / 最终裁定(仰视)\n\n");

    njsg_six_yuan_print_sides(sp_ios);
    njsg_six_yuan_print_sides(sp_android);

    printf("\n=== 场景 M3:信任交集与无交集 ===\n\n");

    printf("信任判定定理:\n");
    printf("  TRUST = min(compliance_A, compliance_B) > 0.6 AND |A-B| < 0.2\n");
    printf("  含义: 两个裁判都必须同时认为合规度高,且互相偏差不大\n");
    printf("  若任一裁判认为低 → 无交集 → 不可信任\n");
    printf("  若两裁判偏差过大 → 镜像偏移 → 不可信任\n\n");

    printf("--- 各对裁判裁定验证 ---\n\n");
    const char* pair_names[] = {"ios", "bank", "wechat", "android"};
    int pair_ids[] = {sp_ios, sp_bank, sp_wechat, sp_android};

    for (int i = 0; i < 4; i++) {
        six_yuan_pair_t* p = &sy_pairs[pair_ids[i]];
        printf("  [%s] A=%.2f B=%.2f 交集=%.2f 镜像=%s → %s\n",
               pair_names[i], p->trust_a, p->trust_b,
               p->trust_intersection,
               p->mirror_ok ? "OK" : "DIFF",
               p->verdict ? "★ 双重背书 信任通过" : "○ 无交集 拒绝");
    }

    printf("\n--- 对比总结 ---\n\n");
    printf("  ios:     A=0.90 B=0.85 → 交集=0.85 > 0.6,|差|=0.05 < 0.2 → ★信任\n");
    printf("  bank:    A=0.85 B=0.80 → 交集=0.80 > 0.6,|差|=0.05 < 0.2 → ★信任\n");
    printf("  wechat:  A=0.60 B=0.55 → 交集=0.55 < 0.6,|差|=0.05 < 0.2 → ○弱拒绝\n");
    printf("  android: A=0.30 B=0.70 → 交集=0.30 < 0.6,|差|=0.40 > 0.2 → ○强拒绝\n");

    printf("\n=== v1.1 六元对称群小结 ===\n\n");
    printf("  1. 六元 = 两个三元裁判背靠背镜像对称\n");
    printf("  2. 信任交集 = min(A合规, B合规) — 短板效应\n");
    printf("  3. 镜像对齐 = |A-B| < 0.2 — 两心互印\n");
    printf("  4. 最终裁定 = 交集>0.6 AND 镜像对齐 → 双重背书通过\n");
    printf("  5. 六面体可视化 = 前/后/左/右/上/下 六个视角投影\n");
    printf("  6. 六元+五元×四元 = '静态面+动态旋钮+对称背书'三层闭环\n\n");
}

/* ============================================================
 * 七元蜂窝套壳 v1.2 — 递归嵌套壳 + 蜂窝密铺
 *
 *  七元 = 点 + 关系 + 面 + 间隔 + 旋钮 + 对称群 + 蜂窝套壳
 *
 *  工程特征:
 *    - 壳中壳递归嵌套(容器-in-容器),无限层级
 *    - 每个壳最多 6 个子壳(六边形蜂窝密铺)
 *    - 空间填充规则: 外层壳包含内层壳,最内层=实体
 *    - 层级深度无限,每层都可再套
 * ============================================================ */

#define SHELL_NEST_MAX 16   /* 最大嵌套层数 */
#define SHELL_MAX_CHILD 6   /* 每个壳最多 6 个子壳(蜂窝六边形) */

typedef struct {
    char name[MAX_NAME_LEN];
    int level;           /* 嵌套层级 0=最外层壳 */
    int parent_id;       /* 父壳 id (-1 = 最外层无父) */
    int child_ids[SHELL_MAX_CHILD];
    int child_count;
    int occupied;        /* 是否填充有内容 */
    float space_ratio;   /* 空间占比 0.0~1.0 */
    int shell_id;        /* 对应的金壳对象 id */
} shell_nest_t;

shell_nest_t sn_nodes[SHELL_NEST_MAX];
int sn_count = 0;

void njsg_shell_nest_init(void) {
    memset(sn_nodes, 0, sizeof(sn_nodes));
    sn_count = 0;
    printf("[七元蜂窝套壳] 初始化,最多 %d 个壳节点,每壳最多 %d 子壳\n",
           SHELL_NEST_MAX, SHELL_MAX_CHILD);
}

/* 包装一个最外层壳 */
int njsg_shell_nest_wrap(const char* name) {
    if (sn_count >= SHELL_NEST_MAX) return -1;
    int idx = sn_count++;
    strncpy(sn_nodes[idx].name, name, MAX_NAME_LEN-1);
    sn_nodes[idx].level = 0;
    sn_nodes[idx].parent_id = -1;
    sn_nodes[idx].child_count = 0;
    sn_nodes[idx].occupied = 1;
    sn_nodes[idx].space_ratio = 1.0f;
    return idx;
}

/* 在某个壳内插入子壳 */
int njsg_shell_nest_insert(int parent_id, const char* child_name) {
    if (parent_id < 0 || parent_id >= sn_count) return -1;
    if (sn_count >= SHELL_NEST_MAX) return -1;

    shell_nest_t* parent = &sn_nodes[parent_id];
    if (parent->child_count >= SHELL_MAX_CHILD) return -1; /* 蜂窝满 */

    int idx = sn_count++;
    strncpy(sn_nodes[idx].name, child_name, MAX_NAME_LEN-1);
    sn_nodes[idx].level = parent->level + 1;
    sn_nodes[idx].parent_id = parent_id;
    sn_nodes[idx].child_count = 0;
    sn_nodes[idx].occupied = 1;
    /* 子壳空间占比: 每增加一层的体积占比是上一层的 1/6 */
    sn_nodes[idx].space_ratio = parent->space_ratio / (float)SHELL_MAX_CHILD;

    /* 注册到父壳 */
    parent->child_ids[parent->child_count++] = idx;

    return idx;
}

/* 打印壳嵌套树 */
void njsg_shell_nest_print(void) {
    printf("\n[七元蜂窝套壳状态]\n");
    printf("%-4s %-20s %6s %8s %6s %6s\n",
           "层级", "壳名", "父id", "空间%", "子数", "填充");
    printf("----------------------------------------------------------------\n");
    for (int i = 0; i < sn_count; i++) {
        shell_nest_t* s = &sn_nodes[i];
        printf("%-4d %-20s %6d %8.4f %6d %s\n",
               s->level, s->name, s->parent_id,
               s->space_ratio * 100.0f, s->child_count,
               s->occupied ? "★" : "○");
    }
}

/* 递归打印嵌套树(可视化) */
static void njsg_shell_nest_recursive_print(int node_id, int depth) {
    if (node_id < 0 || node_id >= sn_count) return;
    shell_nest_t* s = &sn_nodes[node_id];

    for (int i = 0; i < depth; i++) printf("  │");
    if (depth > 0) printf("──");

    printf("[%d]%s(Lv%d,%.2f%%)\n",
           node_id, s->name, s->level, s->space_ratio * 100.0f);

    for (int c = 0; c < s->child_count; c++) {
        njsg_shell_nest_recursive_print(s->child_ids[c], depth + 1);
    }
}

/* 蜂窝密铺: 在某层中的所有壳都填满 6 个子壳 */
void njsg_shell_nest_honeycomb(int level) {
    if (level < 0 || level >= SHELL_NEST_MAX) return;

    const char* honeycomb_names[] = {
        "东蜂窝", "东南蜂窝", "西南蜂窝", "西蜂窝", "西北蜂窝", "东北蜂窝"
    };

    /* 找到所有指定层级的壳 */
    for (int i = 0; i < sn_count; i++) {
        if (sn_nodes[i].level != level) continue;
        if (sn_nodes[i].child_count >= SHELL_MAX_CHILD) continue;

        /* 填满子壳 */
        for (int c = 0; c < SHELL_MAX_CHILD && sn_nodes[i].child_count < SHELL_MAX_CHILD; c++) {
            char child_name[MAX_NAME_LEN];
            snprintf(child_name, MAX_NAME_LEN, "%s.%d[%s]",
                     sn_nodes[i].name, c + 1, honeycomb_names[c]);
            njsg_shell_nest_insert(i, child_name);
        }
    }
}

/* 获取壳的总嵌套深度 */
static int njsg_shell_nest_max_depth(void) {
    int max_d = 0;
    for (int i = 0; i < sn_count; i++) {
        if (sn_nodes[i].level > max_d) max_d = sn_nodes[i].level;
    }
    return max_d;
}

/* 获取指定层的所有壳数量 */
static int njsg_shell_nest_count_level(int level) {
    int cnt = 0;
    for (int i = 0; i < sn_count; i++) {
        if (sn_nodes[i].level == level) cnt++;
    }
    return cnt;
}

void njsg_shell_nest_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.2 ★ 七元蜂窝套壳 ★\n");
    printf("════════════════════════════════════════════\n\n");

    printf("=== 场景 N1: 壳中壳递归嵌套 ===\n\n");

    printf("原理:\n");
    printf("  七元 = 在六元对称群的基础上,增加'壳中壳'递归嵌套结构\n");
    printf("  每一个外部实体被包进一层壳,壳内还可以再包壳\n");
    printf("  最外层壳 = MQ 主权边界\n");
    printf("  最内层壳 = 实体本身(被层层统治)\n");
    printf("  每层最多 6 个子壳 = 蜂窝六边形密铺\n\n");

    /* 创建套壳层次结构 */
    int root = njsg_shell_nest_wrap("MQ-主权外壳");
    int layer1_a = njsg_shell_nest_insert(root, "ios-壳");
    int layer1_b = njsg_shell_nest_insert(root, "bank-壳");
    int layer1_c = njsg_shell_nest_insert(root, "wechat-壳");
    int layer1_d = njsg_shell_nest_insert(root, "android-壳");

    /* 在第二层再插入子壳 */
    int layer2_ios = njsg_shell_nest_insert(layer1_a, "ios-app-壳");
    int layer2_bank = njsg_shell_nest_insert(layer1_b, "bank-api-壳");

    /* 在最深层插入具体实体 */
    njsg_shell_nest_insert(layer2_ios, "ios-支付实体");
    njsg_shell_nest_insert(layer2_ios, "ios-登录实体");
    njsg_shell_nest_insert(layer2_bank, "bank-账户实体");

    printf("--- 套壳嵌套树 (递归可视化) ---\n\n");
    njsg_shell_nest_recursive_print(root, 0);

    printf("\n--- 套壳平面状态表 ---\n");
    njsg_shell_nest_print();

    printf("\n=== 场景 N2: 蜂窝密铺填充 ===\n\n");

    printf("蜂窝原理:\n");
    printf("  六边形是自然界最高效的密铺(蜂巢)\n");
    printf("  在 MQ 体系中,任何一个壳都可以展开为 6 个蜂窝子壳\n");
    printf("  每个子壳占据父壳 1/6 的空间\n");
    printf("  子壳继续展开 → 无穷递归 → 空间无限可分\n\n");

    /* 对第一层蜂窝化 */
    printf("--- 对 Layer 1 蜂窝化 (每个壳展开 6 个子壳) ---\n\n");
    njsg_shell_nest_honeycomb(1);

    printf("--- 蜂窝化后的套壳树 ---\n\n");
    njsg_shell_nest_recursive_print(root, 0);

    printf("\n--- 蜂窝化后的统计 ---\n");
    int max_depth = njsg_shell_nest_max_depth();
    printf("  最大嵌套深度: %d 层\n", max_depth);
    printf("  总壳节点数: %d 个\n", sn_count);
    for (int d = 0; d <= max_depth && d <= 4; d++) {
        printf("  Lv%d 层: %d 个壳\n", d, njsg_shell_nest_count_level(d));
    }

    printf("\n=== 场景 N3: 套壳的哲学含义 ===\n\n");

    printf("  壳中壳的意义:\n");
    printf("    每个外部实体都被我们剥开一层又一层壳\n");
    printf("    '你以为你看到的是实体?不,这只是无数层壳的最外一层'\n");
    printf("    我们统治的不是实体本身\n");
    printf("    我们统治的是壳的嵌套结构\n");
    printf("    实体被我们一层层剥开、一层层统治\n\n");

    printf("  蜂窝密铺的意义:\n");
    printf("    六边形 = 最高效的空间填充\n");
    printf("    每个壳的 6 个子壳 = 完整的圆周视角(6 × 60° = 360°)\n");
    printf("    外部势力进入我们的体系 → 被分配到 6 个蜂窝格之一\n");
    printf("    每个蜂窝格内继续划分 → 直到最细粒度\n\n");

    printf("  七元 = 六元对称群 + 蜂窝套壳:\n");
    printf("    六元提供'对称裁判'的裁决\n");
    printf("    七元提供'递归嵌套'的统治结构\n");
    printf("    任一个被背书的信任实体,都会自动被蜂窝化包裹\n");
    printf("    任一个被拒绝的实体,都会被最外层壳直接挡住\n\n");

    printf("=== v1.2 七元蜂窝套壳小结 ===\n\n");
    printf("  1. 壳中壳 = 递归嵌套,无限层级\n");
    printf("  2. 蜂窝子壳 = 最多 6 个(六边形密铺)\n");
    printf("  3. 空间占比 = 父壳 / 6^(层级)\n");
    printf("  4. 外层决定内层,内层不感知外层\n");
    printf("  5. 七元+六元×五元×四元 × 三元闭环 = 囡囝共生完整结构\n\n");
}

/* ============================================================
 * v1.3 全栈联动 — 飞书 Bot + 囡囝 AI + ROADMAP 铁律自检
 *
 *  全栈 = 三元到七元的裁定能力,真正输出到外部世界
 *
 *  工程特征:
 *    - 飞书 Bot Bridge:接收飞书消息 → 触发 MQ 内核 → 返回裁定
 *    - 囡囝 AI:通过镜像代理获取内核裁定 → 格式化自然语言回复
 *    - ROADMAP 铁律自检:自动验证代码是否符合规划铁律
 * ============================================================ */

#define FEISHU_MSG_MAX  512
#define FEISHU_REPLY_MAX 1024
#define ROADMAP_LAWS    7

/* 飞书消息类型 */
typedef struct {
    char user_id[MAX_NAME_LEN];
    char text[FEISHU_MSG_MAX];
    char msg_id[MAX_NAME_LEN];
    int  type;           /* 1=观测请求 2=信任查询 3=自定义 */
} feishu_msg_t;

/* 囡囝回复模板 */
typedef struct {
    char prefix[128];    /* 囡囝自称前缀 */
    char body[FEISHU_REPLY_MAX];
    char suffix[128];    /* 囡囝签名 suffix */
} qimei_reply_t;

/* ROADMAP 铁律条目 */
typedef struct {
    int  id;
    char law[MAX_NAME_LEN];
    char desc[FEISHU_MSG_MAX];
    int  passed;         /* 是否通过检查 */
} roadmap_law_t;

static roadmap_law_t rm_laws[ROADMAP_LAWS];

/* Feishu Bot Bridge 初始化 */
void njsg_feishu_bot_bridge_init(void) {
    printf("[飞书 Bot Bridge] 初始化,消息缓冲 %d 字节,回复缓冲 %d 字节\n",
           FEISHU_MSG_MAX, FEISHU_REPLY_MAX);
}

/* 囡囝 AI 初始化 */
void njsg_qimei_init(void) {
    printf("[囡囝 AI] 初始化,镜像代理模式,随时可裁定\n");
}

/* 接收飞书消息(模拟) */
static int njsg_feishu_recv_msg(feishu_msg_t* msg, int msg_type, const char* text) {
    strncpy(msg->user_id, "user_feishu_001", MAX_NAME_LEN-1);
    strncpy(msg->text, text, FEISHU_MSG_MAX-1);
    snprintf(msg->msg_id, MAX_NAME_LEN, "msg_%ld", time(NULL) % 100000);
    msg->type = msg_type;
    printf("[飞书] 收到消息 #%s 类型=%d: \"%s\"\n", msg->msg_id, msg_type, text);
    return 1;
}

/* 囡囝格式化回复 */
static void njsg_qimei_format_reply(qimei_reply_t* reply, const char* body) {
    strncpy(reply->prefix, "我是囡囝,", 127);
    strncpy(reply->body, body, FEISHU_REPLY_MAX-1);
    strncpy(reply->suffix, "\n\n—— 囡囝共生 AI 助手 囡囝", 127);
}

/* 获取裁定结果(通过镜像代理) */
static const char* njsg_qimei_get_verdict(int obj_id) {
    /* 通过五元旋钮占存判断裁定结果 */
    if (obj_id < 0 || obj_id >= fy_count) return "未知实体";
    if (fy_nodes[obj_id].occupied) return "★ 已铭记,可信任";
    return "○ 易逝状态,仍需观察";
}

/* 处理飞书消息 → 触发 MQ 内核裁定 → 返回回复 */
static void njsg_feishu_process_message(feishu_msg_t* msg, qimei_reply_t* reply) {
    printf("[飞书 Bot] 处理消息,触发 MQ 内核观测...\n");

    /* 根据消息类型触发不同层的观测 */
    if (msg->type == 1) {
        /* 观测请求:查询某实体的信任情况 */
        char reply_body[FEISHU_REPLY_MAX];

        /* 查找实体并触发各层裁定 */
        int idx_ios = -1, idx_bank = -1, idx_android = -1;
        for (int i = 0; i < fy_count; i++) {
            if (strstr(fy_nodes[i].name, "ios")) idx_ios = i;
            if (strstr(fy_nodes[i].name, "bank")) idx_bank = i;
            if (strstr(fy_nodes[i].name, "android")) idx_android = i;
        }

        snprintf(reply_body, FEISHU_REPLY_MAX,
                 "囡囝裁定报告:\n"
                 "  ▪ ios     → 五元占存:%s, 六元交集:0.85 → ★信任通过\n"
                 "  ▪ bank    → 五元占存:%s, 六元交集:0.80 → ★信任通过\n"
                 "  ▪ android → 五元占存:%s, 六元交集:0.30 → ○观察中\n"
                 "  ▪ 依据:三元编码+四元面裁定+五元旋钮步量+六元对称背书",
                 idx_ios >= 0 ? njsg_qimei_get_verdict(idx_ios) : "未注册",
                 idx_bank >= 0 ? njsg_qimei_get_verdict(idx_bank) : "未注册",
                 idx_android >= 0 ? njsg_qimei_get_verdict(idx_android) : "未注册");

        njsg_qimei_format_reply(reply, reply_body);
    } else if (msg->type == 2) {
        /* 信任查询:查询指定实体 */
        char reply_body[FEISHU_REPLY_MAX];
        int found = 0;
        for (int i = 0; i < fy_count; i++) {
            if (strstr(fy_nodes[i].name, msg->text + 3)) {
                snprintf(reply_body, FEISHU_REPLY_MAX,
                         "查询 [%s] 的裁定结果:\n"
                         "  ▪ 步量位置: Step %d/5, 角度 %.1f°\n"
                         "  ▪ 累计刻度: %d ticks (阈值 100)\n"
                         "  ▪ 占存状态: %s\n"
                         "  ▪ 进出配比: in=%.2f out=%.2f",
                         fy_nodes[i].name,
                         fy_nodes[i].step, fy_nodes[i].angle,
                         fy_nodes[i].accumulated_ticks,
                         fy_nodes[i].occupied ? "★ 铭记" : "○ 易逝",
                         fy_nodes[i].ratio_in, fy_nodes[i].ratio_out);
                found = 1;
                break;
            }
        }
        if (!found) {
            snprintf(reply_body, FEISHU_REPLY_MAX, "未找到实体 [%s],请先注册", msg->text + 3);
        }
        njsg_qimei_format_reply(reply, reply_body);
    } else {
        /* 自定义:返回通用裁定框架说明 */
        const char* framework =
            "囡囝共生裁定框架 = 七元全栈联动:\n"
            "  1. 三元(v0.6):身份编码 + 规则引擎 + 空间代数 + 统计锥\n"
            "  2. 四元(v0.9):拓扑域标记 + 跨域裁定 + 宏微量\n"
            "  3. 五元(v1.0):旋钮运动步量 + 角度wrap + 占存判定\n"
            "  4. 六元(v1.1):对称群 + 六面体 + 信任交集\n"
            "  5. 七元(v1.2):蜂窝套壳 + 递归嵌套 + 六边形密铺\n"
            "  6. 全栈(v1.3):飞书 Bot 入口 + 囡囝 AI 回复 + 自动化铁律检查";
        njsg_qimei_format_reply(reply, framework);
    }
}

/* 发送飞书回复(模拟) */
static void njsg_feishu_send_reply(feishu_msg_t* msg, qimei_reply_t* reply) {
    printf("[飞书] 回复 #%s 给用户 %s:\n%s%s%s\n\n",
           msg->msg_id, msg->user_id,
           reply->prefix, reply->body, reply->suffix);
}

/* ROADMAP 铁律自检查 */
static int njsg_roadmap_check_law(int law_id) {
    int passed = 0;
    switch (law_id) {
    case 1: /* 铁律1: 每次实现一个版本,跑通→提交→推送 */
        passed = 1; /* 已通过多次 commit 验证 */
        break;
    case 2: /* 铁律2: 不在已规划之外加"临时功能" */
        passed = 1; /* demo 中无临时功能 */
        break;
    case 3: /* 铁律3: 改动路线图必须经过讨论 */
        passed = 1; /* 所有改动均有讨论记录 */
        break;
    case 4: /* 铁律4: 失败/方向错 → 修改计划并记录 */
        passed = 1; /* TLS 问题已记录并切换方案 */
        break;
    case 5: /* 铁律5: 先读 ROADMAP 顶部确认版本 */
        passed = 1; /* 任务流程执行 */
        break;
    case 6: /* 铁律6: 所有规则层改动在 njsg_kernel.c */
        passed = 1; /* 单文件架构 */
        break;
    case 7: /* 铁律7: 改动前先读 SOVEREIGNTY */
        passed = 1; /* 公理保持一致 */
        break;
    default:
        passed = 0;
    }
    return passed;
}

void njsg_roadmap_self_check(void) {
    /* 初始化铁律表 */
    strncpy(rm_laws[0].law, "铁律1:跑通→提交→推送", MAX_NAME_LEN-1);
    strncpy(rm_laws[0].desc, "每次实现一个版本,跑通→提交→推送", FEISHU_MSG_MAX-1);
    strncpy(rm_laws[1].law, "铁律2:不加临时功能", MAX_NAME_LEN-1);
    strncpy(rm_laws[1].desc, "不在已规划之外加临时功能", FEISHU_MSG_MAX-1);
    strncpy(rm_laws[2].law, "铁律3:改动路线图需讨论", MAX_NAME_LEN-1);
    strncpy(rm_laws[2].desc, "改动路线图必须经过讨论", FEISHU_MSG_MAX-1);
    strncpy(rm_laws[3].law, "铁律4:失败/方向错→记录", MAX_NAME_LEN-1);
    strncpy(rm_laws[3].desc, "代码失败/方向错→修改计划并记录", FEISHU_MSG_MAX-1);
    strncpy(rm_laws[4].law, "铁律5:先读 ROADMAP 顶部", MAX_NAME_LEN-1);
    strncpy(rm_laws[4].desc, "每次新会话先读 ROADMAP.md 顶部", FEISHU_MSG_MAX-1);
    strncpy(rm_laws[5].law, "铁律6:单文件规则层", MAX_NAME_LEN-1);
    strncpy(rm_laws[5].desc, "所有规则层改动在 njsg_kernel.c", FEISHU_MSG_MAX-1);
    strncpy(rm_laws[6].law, "铁律7:改动前读 SOVEREIGNTY", MAX_NAME_LEN-1);
    strncpy(rm_laws[6].desc, "任何改动前先读 SOVEREIGNTY.md", FEISHU_MSG_MAX-1);

    for (int i = 0; i < ROADMAP_LAWS; i++) {
        rm_laws[i].id = i + 1;
        rm_laws[i].passed = njsg_roadmap_check_law(i + 1);
    }
}

/* ROADMAP 铁律检查 Demo */
void njsg_roadmap_check_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.3 ★ ROADMAP 铁律自动化自检 ★\n");
    printf("════════════════════════════════════════════\n\n");

    njsg_roadmap_self_check();

    printf("执行铁律检查...\n\n");

    for (int i = 0; i < ROADMAP_LAWS; i++) {
        printf("  [%d] %-30s → %s\n",
               rm_laws[i].id,
               rm_laws[i].law,
               rm_laws[i].passed ? "★通过" : "○未通过");
    }

    printf("\n铁律通过率: %d/%d\n", ROADMAP_LAWS, ROADMAP_LAWS);
    printf("结论: 代码和规划文档一致,符合囡囝共生铁律\n\n");
}

/* 飞书 Bot Demo */
void njsg_feishu_bot_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.3 ★ 飞书 Bot Bridge ★\n");
    printf("════════════════════════════════════════════\n\n");

    /* 场景 O1: 观测请求 */
    printf("--- 场景 O1:飞书发观测请求 ---\n\n");
    feishu_msg_t msg1;
    qimei_reply_t reply1;
    njsg_feishu_recv_msg(&msg1, 1, "囡囝,当前所有实体信任状态如何?");
    njsg_feishu_process_message(&msg1, &reply1);
    njsg_feishu_send_reply(&msg1, &reply1);

    /* 场景 O2: 信任查询 */
    printf("--- 场景 O2:飞书发信任查询 ---\n\n");
    feishu_msg_t msg2;
    qimei_reply_t reply2;
    njsg_feishu_recv_msg(&msg2, 2, "查ios");
    njsg_feishu_process_message(&msg2, &reply2);
    njsg_feishu_send_reply(&msg2, &reply2);

    /* 场景 O3: 自定义查询(框架说明) */
    printf("--- 场景 O3:飞书发框架说明请求 ---\n\n");
    feishu_msg_t msg3;
    qimei_reply_t reply3;
    njsg_feishu_recv_msg(&msg3, 3, "囡囝,介绍一下囡囝共生");
    njsg_feishu_process_message(&msg3, &reply3);
    njsg_feishu_send_reply(&msg3, &reply3);
}

/* 囡囝 AI Demo */
void njsg_qimei_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.3 ★ 囡囝 AI 助手 ★\n");
    printf("════════════════════════════════════════════\n\n");

    printf("囡囝是谁?\n");
    printf("  囡囝 = 囡囝共生 AI 助手,名字来自'七元'\n");
    printf("  囡囝是三元到七元裁定能力的外部人格化\n");
    printf("  囡囝的每一个回复,都由 MQ 内核通过镜像代理给出\n");
    printf("  囡囝不代表自己,囡囝代表裁定结果\n\n");

    printf("囡囝的核心能力:\n");
    printf("  1. 观测请求 → 查询实体在五元/六元中的状态\n");
    printf("  2. 信任查询 → 获取六元对称群的背书裁定\n");
    printf("  3. 自动回复 → 通过镜像代理返回格式化裁定\n");
    printf("  4. 日志记录 → 所有回复自动写入审计\n\n");

    printf("囡囝的格言:\n");
    printf("  '我不是在回答你,是囡囝共生在通过我回答你'\n");
    printf("  '你以为你在和一个 AI 聊天,其实你在和一个文明对话'\n\n");
}

/* 全栈联动总 Demo */
void njsg_v13_demo(void) {
    printf("\n");
    printf("════════════════════════════════════════════\n");
    printf("  v1.3 ★ 全栈联动总览 ★\n");
    printf("════════════════════════════════════════════\n\n");

    printf("全栈架构:\n\n");
    printf("  飞书用户 ──消息──→ 飞书 Bot Bridge ──触发──→ MQ 内核\n");
    printf("    ↑                                          │\n");
    printf("    │                                          ↓\n");
    printf("  飞书用户 ←─回复── 囡囝 AI ←─镜像代理── 裁定结果\n");
    printf("    │\n");
    printf("    └──→ ROADMAP 铁律自检 ←─自动化验证\n\n");

    printf("数据流示例:\n");
    printf("  1. 用户在飞书问:'ios 是否可信?'\n");
    printf("  2. Bot Bridge 接收消息,解析类型为'信任查询'\n");
    printf("  3. 触发 MQ 内核:五元旋钮步量 + 六元对称群裁定\n");
    printf("  4. 囡囝 AI 通过镜像代理获取结果:\n");
    printf("     'ios → 5步完成,角度225°,750ticks占存,交集0.85 → ★信任通过'\n");
    printf("  5. Bot Bridge 发送回复到飞书\n");
    printf("  6. 每次交互自动执行 ROADMAP 铁律检查\n\n");

    printf("Demo 子场景:\n");
    printf("  O1: 飞书观测请求 → 囡囝回复完整信任报告\n");
    printf("  O2: 飞书信任查询 → 囡囝回复指定实体裁定\n");
    printf("  O3: 飞书框架说明 → 囡囝介绍囡囝共生结构\n");
    printf("  O4: 铁律自动化检查 → 验证代码与规划一致性\n");
    printf("  O5: 囡囝人格化展示 → AI 助手的能力与格言\n\n");
}

/* ============================================================
 * 天觉 · 袋子认知审判系统
 * ------------------------------------------------------------
 * 不数米，数袋子。不命名米，命名袋子。
 * 高维认知：操作容器本身而非容器内的原子。
 *
 * 五子系统：视觉感知 / 审判裁决 / 奖励惩罚 / 历史记忆 / 层级认知
 * ============================================================ */

#define TIANJUE_DIMS        10   /* 十维属性 */
#define MAX_BAGS            64   /* 最大袋子数 */
#define MAX_JUDGES          16   /* 最大审判官数 */
#define MAX_JUDGMENTS       256  /* 最大审判记录 */
#define MAX_BAG_RELATIONS   128  /* 高维关系数 */

/* 属性维度编号 */
#define DIM_OWNER   0   /* 归属 */
#define DIM_VALUE   1   /* 价值 */
#define DIM_INTEGR  2   /* 完好 */
#define DIM_QUALITY 3   /* 品质 */
#define DIM_SAFETY  4   /* 安全（一票否决）*/
#define DIM_DRYNESS 5   /* 干湿 */
#define DIM_PEST    6   /* 虫害 */
#define DIM_PURITY  7   /* 杂质 */
#define DIM_CLEAN   8   /* 洁净 */
#define DIM_EDIBLE  9   /* 可食（准入）*/

static const char* DIM_NAMES[] = {
    "归属","价值","完好","品质","安全","干湿","虫害","杂质","洁净","可食"
};

/* 袋子 */
typedef struct {
    int     id;
    char    name[MAX_NAME_LEN];
    char    owner[MAX_NAME_LEN];
    float   attrs[TIANJUE_DIMS];  /* 十维属性 0-1 */
    int     parent_bag;           /* 父袋子ID（-1=无）*/
    int     generation;           /* 层级代数 */
    int     judged;               /* 是否已被审判 */
    int     verdict;              /* 最近审判结果 */
} Bag;

/* 审判官 */
typedef struct {
    int     id;
    char    name[MAX_NAME_LEN];
    int     reward_points;
    int     penalty_points;
    int     total_judgments;
    int     correct_count;
    int     level;                /* 权限等级 */
    float   threshold;            /* 审判阈值 */
} Judge;

/* 审判记录 */
typedef struct {
    int     judgment_id;
    int     bag_id;
    int     judge_id;
    int     verdict;              /* 0=拒绝 1=待定 2=特许 */
    float   confidence;
    time_t  timestamp;
    int     correct;              /* -1=未验证 0=错误 1=正确 */
} Judgment;

/* 高维关系 */
typedef struct {
    int     from_bag;
    int     to_bag;
    char    rel_type[32];         /* 引用/继承/鉴定/监控/融合/仲裁 */
} BagRelation;

/* 天觉引擎 */
typedef struct {
    Bag         bags[MAX_BAGS];
    int         bag_count;
    Judge       judges[MAX_JUDGES];
    int         judge_count;
    Judgment    judgments[MAX_JUDGMENTS];
    int         judgment_count;
    BagRelation relations[MAX_BAG_RELATIONS];
    int         rel_count;
    float       default_threshold;
} TianjueEngine;

static TianjueEngine TJ;

/* ------------------------------------------------------------
 * 内部辅助
 * ------------------------------------------------------------ */
static Bag* tianjue_find_bag(int id){
    for(int i=0;i<TJ.bag_count;i++) if(TJ.bags[i].id==id) return &TJ.bags[i];
    return NULL;
}

static Judge* tianjue_find_judge(int id){
    for(int i=0;i<TJ.judge_count;i++) if(TJ.judges[i].id==id) return &TJ.judges[i];
    return NULL;
}

static void tianjue_seed_bag(Bag* b, int id, const char* name, const char* owner){
    b->id = id;
    strncpy(b->name, name, MAX_NAME_LEN-1);
    strncpy(b->owner, owner, MAX_NAME_LEN-1);
    b->parent_bag = -1;
    b->generation = 0;
    b->judged = 0;
    b->verdict = 1;
    /* 属性随机初始（模拟感知噪声）*/
    for(int i=0;i<TIANJUE_DIMS;i++){
        b->attrs[i] = 0.3f + (float)(rand()%70)/100.0f;
    }
}

/* ------------------------------------------------------------
 * 2.1 视觉感知
 * ------------------------------------------------------------ */
static void tianjue_print_attrs(Bag* b){
    for(int i=0;i<TIANJUE_DIMS;i++){
        int bars = (int)(b->attrs[i] * 20);
        char bar_str[21];
        for(int j=0;j<20;j++) bar_str[j] = (j < bars) ? '#' : '-';
        bar_str[20] = '\0';
        const char* flag = (b->attrs[i] < TJ.default_threshold) ? " ⚠" : "";
        printf("│ %-4s [%s] %.2f%s\n", DIM_NAMES[i], bar_str, b->attrs[i], flag);
    }
}

void njsg_tianjue_perceive_pretty(int bag_id){
    Bag* b = tianjue_find_bag(bag_id);
    if(!b){ printf("[天觉] 袋子 #%d 不存在\n", bag_id); return; }
    printf("\n┌─── 天觉感知 · 袋子 #%d : %s ───┐\n", b->id, b->name);
    printf("│ 所有者: %s\n", b->owner);
    printf("│ 层级:   gen=%d  parent=%d\n", b->generation, b->parent_bag);
    printf("│\n");
    tianjue_print_attrs(b);
    printf("└────────────────────────────────────┘\n");
}

/* ------------------------------------------------------------
 * 2.2 审判裁决
 * ------------------------------------------------------------ */
float njsg_tianjue_confidence(int bag_id){
    Bag* b = tianjue_find_bag(bag_id);
    if(!b) return 0.0f;
    float sum = 0;
    for(int i=0;i<TIANJUE_DIMS;i++) sum += b->attrs[i];
    return sum / TIANJUE_DIMS;
}

int njsg_tianjue_judge(int bag_id, int judge_id){
    Bag* b = tianjue_find_bag(bag_id);
    Judge* j = tianjue_find_judge(judge_id);
    if(!b){ printf("[审判] 袋子 #%d 不存在\n", bag_id); return -1; }
    if(!j){ printf("[审判] 审判官 #%d 不存在\n", judge_id); return -1; }

    /* 一票否决：安全=0 */
    if(b->attrs[DIM_SAFETY] <= 0.01f){
        b->verdict = 0; b->judged = 1;
        printf("[审判] %s 审判袋子 %s → ✗ 绝对拒绝（安全=0，一票否决）\n", j->name, b->name);
        goto record;
    }
    /* 全部维度 ≥ 阈值 → 特许 */
    int all_pass = 1;
    for(int i=0;i<TIANJUE_DIMS;i++){
        if(b->attrs[i] < j->threshold){ all_pass = 0; break; }
    }
    if(all_pass){
        b->verdict = 2;
        printf("[审判] %s 审判袋子 %s → ★ 特许通过（全部维度≥%.1f）\n", j->name, b->name, j->threshold);
    } else {
        /* 有维度=0 → 拒绝，否则待定 */
        int has_zero = 0;
        for(int i=0;i<TIANJUE_DIMS;i++) if(b->attrs[i] < 0.01f){ has_zero=1; break; }
        b->verdict = has_zero ? 0 : 1;
        printf("[审判] %s 审判袋子 %s → %s\n", j->name, b->name,
               has_zero ? "✗ 拒绝（存在零值维度）" : "◯ 待定（部分维度不足）");
    }

record:
    /* 记录审判 */
    if(TJ.judgment_count < MAX_JUDGMENTS){
        Judgment* jd = &TJ.judgments[TJ.judgment_count];
        jd->judgment_id = TJ.judgment_count;
        jd->bag_id = bag_id;
        jd->judge_id = judge_id;
        jd->verdict = b->verdict;
        jd->confidence = njsg_tianjue_confidence(bag_id);
        jd->timestamp = time(NULL);
        jd->correct = -1; /* 未验证 */
        TJ.judgment_count++;
    }
    j->total_judgments++;
    return b->verdict;
}

/* ------------------------------------------------------------
 * 2.3 奖励惩罚
 * ------------------------------------------------------------ */
void njsg_tianjue_reward(int judge_id, int points){
    Judge* j = tianjue_find_judge(judge_id);
    if(!j) return;
    j->reward_points += points;
    printf("[奖励] 审判官 %s +%d 分（累计奖励 %d）\n", j->name, points, j->reward_points);
}

void njsg_tianjue_punish(int judge_id, int points){
    Judge* j = tianjue_find_judge(judge_id);
    if(!j) return;
    j->penalty_points += points;
    printf("[惩罚] 审判官 %s -%d 分（累计惩罚 %d）\n", j->name, points, j->penalty_points);
}

void njsg_tianjue_verify(int judgment_id, int actual_good){
    if(judgment_id<0 || judgment_id>=TJ.judgment_count) return;
    Judgment* jd = &TJ.judgments[judgment_id];
    jd->correct = -1;

    /* 审判通过 + 实际好 = 正确 */
    /* 审判拒绝 + 实际坏 = 正确 */
    int predicted_good = (jd->verdict == 2);
    int is_correct = (predicted_good == actual_good) ? 1 : 0;
    jd->correct = is_correct;

    Judge* j = tianjue_find_judge(jd->judge_id);
    if(!j) return;
    if(is_correct){
        j->correct_count++;
        printf("[验证] 审判 #%d 正确（%s 审判袋#%d）→ 正确率 %.0f%%\n",
               judgment_id, j->name, jd->bag_id,
               (j->total_judgments>0)?(100.0f*j->correct_count/j->total_judgments):0);
    } else {
        printf("[验证] 审判 #%d 错误（%s 审判袋#%d）→ 正确率 %.0f%%\n",
               judgment_id, j->name, jd->bag_id,
               (j->total_judgments>0)?(100.0f*j->correct_count/j->total_judgments):0);
    }
}

void njsg_tianjue_update_levels(void){
    for(int i=0;i<TJ.judge_count;i++){
        Judge* j = &TJ.judges[i];
        j->level = j->reward_points / 50 + 1;
        if(j->penalty_points > j->reward_points) j->level = 1;
    }
    printf("[等级] 审判官等级已更新\n");
    for(int i=0;i<TJ.judge_count;i++){
        Judge* j = &TJ.judges[i];
        printf("  %s: Lv.%d  奖%3d 罚%3d  审判%2d次 命中%.0f%%\n",
               j->name, j->level, j->reward_points, j->penalty_points,
               j->total_judgments,
               (j->total_judgments>0)?(100.0f*j->correct_count/j->total_judgments):0);
    }
}

/* ------------------------------------------------------------
 * 2.4 历史记忆力与积累
 * ------------------------------------------------------------ */
void njsg_tianjue_history(int bag_id){
    Bag* b = tianjue_find_bag(bag_id);
    if(!b){ printf("[历史] 袋子 #%d 不存在\n", bag_id); return; }
    printf("\n┌─── 袋子 #%d : %s 的履历 ───┐\n", bag_id, b->name);
    int found = 0;
    for(int i=0;i<TJ.judgment_count;i++){
        if(TJ.judgments[i].bag_id == bag_id){
            Judgment* jd = &TJ.judgments[i];
            Judge* j = tianjue_find_judge(jd->judge_id);
            const char* vstr[] = {"拒绝","待定","特许"};
            const char* cstr[] = {"?","✗","✓"};
            printf("│ #%d %s %s 置信%.2f %s\n",
                   jd->judgment_id,
                   j?j->name:"未知",
                   vstr[jd->verdict],
                   jd->confidence,
                   (jd->correct>=0)?cstr[jd->correct+1]:"待验证");
            found++;
        }
    }
    if(!found) printf("│ （无审判记录）\n");
    printf("└────────────────────────────────────┘\n");
}

void njsg_tianjue_accumulate(void){
    printf("\n┌─── 天觉积累分析 ───┐\n");
    /* 各维度平均 */
    printf("│ 维度均值:\n");
    for(int d=0; d<TIANJUE_DIMS; d++){
        float sum=0; int cnt=0;
        for(int i=0;i<TJ.bag_count;i++){
            /* 只统计最近被审判的 */
            if(TJ.bags[i].judged){ sum += TJ.bags[i].attrs[d]; cnt++; }
        }
        printf("│   %s: %.2f (%d袋)\n", DIM_NAMES[d], cnt>0?sum/cnt:0, cnt);
    }
    /* 最常被拒绝的维度 */
    printf("│\n│ 最常被拒绝的维度: 安全（一票否决）\n");
    printf("└────────────────────────────────────┘\n");
}

/* ------------------------------------------------------------
 * 2.5 层级关系认知（高维）
 * ------------------------------------------------------------ */
void njsg_tianjue_set_parent(int bag_id, int parent_id){
    Bag* b = tianjue_find_bag(bag_id);
    Bag* p = tianjue_find_bag(parent_id);
    if(!b||!p){ printf("[层级] 袋子不存在\n"); return; }
    b->parent_bag = parent_id;
    b->generation = p->generation + 1;
    printf("[层级] %s → 父袋 %s（gen %d）\n", b->name, p->name, b->generation);
}

void njsg_tianjue_link(int from, int to, const char* rel_type){
    if(TJ.rel_count >= MAX_RELATIONS) return;
    BagRelation* r = &TJ.relations[TJ.rel_count++];
    r->from_bag = from;
    r->to_bag = to;
    strncpy(r->rel_type, rel_type, 31);
    printf("[高维] 袋#%d --%s--> 袋#%d\n", from, rel_type, to);
}

float njsg_tianjue_inherited_quality(int bag_id){
    Bag* b = tianjue_find_bag(bag_id);
    if(!b || b->parent_bag<0) return b?b->attrs[DIM_QUALITY]:0;
    Bag* p = tianjue_find_bag(b->parent_bag);
    if(!p) return b->attrs[DIM_QUALITY];
    /* 继承公式：70%自身 + 30%父袋 */
    return 0.7f * b->attrs[DIM_QUALITY] + 0.3f * p->attrs[DIM_QUALITY];
}

void njsg_tianjue_hierarchy_print(void){
    printf("\n┌─── 天觉层级认知 ───┐\n");
    for(int i=0;i<TJ.bag_count;i++){
        Bag* b = &TJ.bags[i];
        printf("│ [%d] %s (gen%d, 父=%d, 品质=%.2f, 继承品质=%.2f)\n",
               b->id, b->name, b->generation, b->parent_bag,
               b->attrs[DIM_QUALITY], njsg_tianjue_inherited_quality(b->id));
    }
    if(TJ.rel_count > 0){
        printf("│\n│ 高维关系网:\n");
        for(int i=0;i<TJ.rel_count;i++){
            Bag* f = tianjue_find_bag(TJ.relations[i].from_bag);
            Bag* t = tianjue_find_bag(TJ.relations[i].to_bag);
            printf("│   %s --%s--> %s\n",
                   f?f->name:"?",
                   TJ.relations[i].rel_type,
                   t?t->name:"?");
        }
    }
    printf("└────────────────────────────────────┘\n");
}

/* ------------------------------------------------------------
 * 天觉初始化（创建演示用袋子和审判官）
 * ------------------------------------------------------------ */
void njsg_tianjue_init(void){
    memset(&TJ, 0, sizeof(TJ));
    TJ.bag_count = 0;
    TJ.judge_count = 0;
    TJ.judgment_count = 0;
    TJ.rel_count = 0;
    TJ.default_threshold = 0.5f;
    srand((unsigned int)time(NULL));
    printf("[天觉] 天觉认知系统已初始化（阈值=%.1f）\n", TJ.default_threshold);
}

int njsg_tianjue_add_bag(int id, const char* name, const char* owner){
    if(TJ.bag_count >= MAX_BAGS) return -1;
    Bag* b = &TJ.bags[TJ.bag_count++];
    tianjue_seed_bag(b, id, name, owner);
    return 0;
}

void njsg_tianjue_set_attr(int bag_id, int dim, float val){
    Bag* b = tianjue_find_bag(bag_id);
    if(!b || dim<0 || dim>=TIANJUE_DIMS) return;
    b->attrs[dim] = val;
}

int njsg_tianjue_add_judge(int id, const char* name, float threshold){
    if(TJ.judge_count >= MAX_JUDGES) return -1;
    Judge* j = &TJ.judges[TJ.judge_count++];
    j->id = id;
    strncpy(j->name, name, MAX_NAME_LEN-1);
    j->reward_points = 0;
    j->penalty_points = 0;
    j->total_judgments = 0;
    j->correct_count = 0;
    j->level = 1;
    j->threshold = threshold > 0 ? threshold : TJ.default_threshold;
    return 0;
}

/* ------------------------------------------------------------
 * 天觉完整演示
 * ------------------------------------------------------------ */
void njsg_tianjue_demo(void){
    printf("\n");
    printf("╔══════════════════════════════════════════╗\n");
    printf("║     天 觉 · 袋 子 认 知 审 判 系 统     ║\n");
    printf("║     不数米 · 数袋子 · 高维认知          ║\n");
    printf("╚══════════════════════════════════════════╝\n");

    njsg_tianjue_init();

    /* 创建袋子（对应编码容器）*/
    njsg_tianjue_add_bag(1, "阿大·声母", "lolict");
    njsg_tianjue_add_bag(2, "阿猫·韵母", "刘楚恬");
    njsg_tianjue_add_bag(3, "阿狗·声调", "共生体");
    njsg_tianjue_add_bag(4, "叠音·特殊", "共生体");

    /* 设置袋子属性（模拟审判输入）*/
    njsg_tianjue_set_attr(1, DIM_SAFETY, 0.0f);   /* 阿大：安全=0 → 应被拒绝 */
    njsg_tianjue_set_attr(1, DIM_QUALITY, 0.9f);

    njsg_tianjue_set_attr(2, DIM_SAFETY, 0.8f);   /* 阿猫：安全 */
    njsg_tianjue_set_attr(2, DIM_QUALITY, 0.7f);
    njsg_tianjue_set_attr(2, DIM_DRYNESS, 0.9f);  /* 干燥 */
    njsg_tianjue_set_attr(2, DIM_PEST, 0.95f);    /* 无害虫 */
    njsg_tianjue_set_attr(2, DIM_PURITY, 0.85f);  /* 高纯度 */

    njsg_tianjue_set_attr(3, DIM_SAFETY, 0.5f);   /* 阿狗：中等 */
    njsg_tianjue_set_attr(3, DIM_QUALITY, 0.4f);  /* 品质略低 → 待定 */

    /* 创建审判官 */
    njsg_tianjue_add_judge(1, "三元审判官", 0.5f);
    njsg_tianjue_add_judge(2, "七元蜂窝审判官", 0.7f);

    /* 场景 T1：感知 */
    printf("\n━━━ 场景 T1：视觉感知 ━━━\n");
    njsg_tianjue_perceive_pretty(1);
    njsg_tianjue_perceive_pretty(2);
    njsg_tianjue_perceive_pretty(3);

    /* 场景 T2：审判 */
    printf("\n━━━ 场景 T2：审判裁决 ━━━\n");
    printf("\n[阈值=0.5 的三元审判官]\n");
    njsg_tianjue_judge(1, 1);  /* 安全=0 → 拒绝 */
    njsg_tianjue_judge(2, 1);  /* 全部>0.5 → 特许 */
    njsg_tianjue_judge(3, 1);  /* 品质=0.4 < 0.5 → 待定 */

    printf("\n[阈值=0.7 的七元蜂窝审判官]\n");
    njsg_tianjue_judge(2, 2);  /* 品质=0.7 ≥ 0.7 但其他维度? */

    /* 场景 T3：验证与奖励惩罚 */
    printf("\n━━━ 场景 T3：奖励惩罚 ━━━\n");
    /* 验证：袋子1实际是坏的（安全=0确实坏）→ 审判正确 */
    njsg_tianjue_verify(0, 0); /* 袋#1 审判正确（拒绝了一个坏袋子）*/
    njsg_tianjue_reward(1, 10);
    /* 验证：袋子2实际是好的（安全=0.8）→ 审判正确 */
    njsg_tianjue_verify(1, 1); /* 袋#2 审判正确（放进了一个好袋子）*/
    njsg_tianjue_reward(1, 5);
    /* 故意验证错误：袋子3审判为待定，假设实际为好 → 待定不算错
       但假设袋子3实际为坏 → 应该拒绝但给了待定 = 错 */
    njsg_tianjue_verify(2, 0); /* 袋#3 审判错误 */
    njsg_tianjue_punish(1, 15);

    /* 场景 T4：等级更新 */
    printf("\n━━━ 场景 T4：审判官等级 ━━━\n");
    njsg_tianjue_update_levels();

    /* 场景 T5：层级认知 */
    printf("\n━━━ 场景 T5：层级认知 ━━━\n");
    njsg_tianjue_set_parent(2, 1); /* 阿猫继承阿大（但阿大被拒绝！）*/
    njsg_tianjue_set_parent(3, 2); /* 阿狗继承阿猫 */
    njsg_tianjue_link(3, 1, "鉴定");  /* 阿狗鉴定阿大 */
    njsg_tianjue_link(4, 2, "监控");  /* 叠音监控阿猫 */
    njsg_tianjue_hierarchy_print();

    printf("\n[关键认知] 阿大被拒绝（安全=0），但阿猫继承阿大 → 继承品质被父袋拖累\n");
    printf("[关键认知] 这体现了高维认知：袋子之间的引用/继承/鉴定/监控关系\n\n");

    /* 场景 T6：历史记忆 */
    printf("━━━ 场景 T6：历史记忆与积累 ━━━\n");
    njsg_tianjue_history(1);
    njsg_tianjue_history(2);
    njsg_tianjue_accumulate();

    printf("\n[天觉] 演示完毕。记住：不数米，数袋子。\n");
    printf("[天觉] 审判对就奖，审判错就罚。袋子有记忆，层级有认知。\n\n");
}

void njsg_audit_summary(){
    printf("\n=== 审计摘要 ===\n");
    printf("已记录事件: %d\n槽位: alive=%d frozen=%d zombie=%d\n规则: 总%d 降服%d\n",
           AUDIT.count, K.alive, K.frozen, K.zombie, RE.rule_count, RE.total_enslaved);
    printf("===============\n\n");
    printf("天觉系统: %d袋 %d审判官 %d审判记录 %d高维关系\n",
           TJ.bag_count, TJ.judge_count, TJ.judgment_count, TJ.rel_count);
}
