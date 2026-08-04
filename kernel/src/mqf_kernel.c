/**
 * 满全法执行内核 v0.3 — MQF Executive Kernel
 * -----------------------------------------
 * v0.3 新增: 时间切片 (看门狗自动轮询+超时冻结)
 * 功能: 每个执行槽有严格时间窗口,超时自动休眠,必须重新申请授权
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>

#define MQF_VERSION     "0.3.0"
#define MQF_AUTH_LEN    32
#define MQF_HASH_LEN    64
#define MAX_EXEC_SLOTS  16
#define AUDIT_RING_SIZE 1024
#define MAX_CONTRACTS   64
#define PATROL_INTERVAL 1       /* 看门狗轮询间隔(秒) */

typedef enum { EXEC_SLEEP=0, EXEC_PENDING, EXEC_ALIVE, EXEC_FROZEN, EXEC_ZOMBIE } exec_state_t;
typedef enum { PERM_NONE=0x00, PERM_READ=0x01, PERM_WRITE=0x02, PERM_EXEC=0x04, PERM_NET=0x08, PERM_ADMIN=0x10 } perm_t;
typedef enum { FACT_HAS_AUTH=1, FACT_HAS_NO_AUTH, FACT_TRUST_SOURCE, FACT_UNTRUST_SOURCE, FACT_DEFAULT } fact_type_t;
typedef enum { AUDIT_REGISTER=1, AUDIT_ACTIVATE, AUDIT_EXECUTE, AUDIT_REJECT, AUDIT_FREEZE, AUDIT_TIMEOUT } audit_type_t;

static const char* MQFS_INITS[]={"b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w","@"};
static const char* MQFS_FINALS[]={"a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er","an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao","iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong","@"};

/* ============================================================
 * 第 1 层: 身份编码
 * ============================================================ */
typedef struct { char text[MQF_AUTH_LEN+1]; int init_idx,final_idx,tone_idx,stroke_count; char auth_code[MQF_HASH_LEN+1],hash[MQF_HASH_LEN+1]; } mqf_identity_t;
static void mqf_hash(const char* in, char* out){ uint32_t h1=0x811c9dc5u,h2=0x0100193u; for(size_t i=0;i<strlen(in);i++){h1^=(uint8_t)in[i];h1*=0x01000193u;h2^=(uint8_t)in[i]<<5;h2*=0x1000193u;} sprintf(out,"%08x%08x%08x%08x%08x%08x",h1,h2,h1^h2,h2^h1,h1+h2,h2-h1); }
mqf_identity_t mqf_encode(const char* text){ mqf_identity_t id; memset(&id,0,sizeof(id)); size_t len=strlen(text); if(len>MQF_AUTH_LEN)len=MQF_AUTH_LEN; memcpy(id.text,text,len); id.text[len]='\0'; uint32_t seed=0; for(size_t i=0;i<len;i++) seed=seed*31+(unsigned char)text[i]; id.init_idx=seed%24; id.final_idx=(seed>>5)%39; id.tone_idx=(seed>>10)%5; id.stroke_count=(seed>>15)%30+1; char buf[512]; snprintf(buf,sizeof(buf),"%s|%d|%d|%d|%d",MQFS_INITS[id.init_idx],id.init_idx,id.final_idx,id.tone_idx,id.stroke_count); mqf_hash(buf,id.auth_code); snprintf(buf,sizeof(buf),"%s.%s",text,id.auth_code); mqf_hash(buf,id.hash); return id; }

/* ============================================================
 * 第 2 层: 契约系统
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

/* ============================================================ * 第 3 层: 时间切片状态
 * ============================================================ */
typedef struct { int used; char auth[MQF_HASH_LEN+1], name[128]; exec_state_t st; capability_t cap; void(*fn)(void*); void* arg; time_t activated_at; time_t expire_at; int exec_count; } slot_t;
typedef struct { slot_t s[MAX_EXEC_SLOTS]; int alive, frozen, zombie, ver; } kernel_t;
static kernel_t K;
static pthread_mutex_t K_LOCK = PTHREAD_MUTEX_INITIALIZER;
static int WATCHDOG_RUNNING = 0;
static pthread_t WATCHDOG_TID;

/* 审计环 */
typedef struct { int type; time_t ts; char name[128]; int slot; } audit_entry_t;
typedef struct { audit_entry_t e[AUDIT_RING_SIZE]; int wp; int count; } audit_ring_t;
static audit_ring_t AUDIT;
void mqf_audit_summary(); // 前向声明

static void mqf_audit_log(int type, const char* name, int slot){ audit_entry_t* x=&AUDIT.e[AUDIT.wp]; x->type=type; x->ts=time(NULL); strncpy(x->name,name,127); x->slot=slot; AUDIT.wp=(AUDIT.wp+1)%AUDIT_RING_SIZE; if(AUDIT.count<AUDIT_RING_SIZE)AUDIT.count++; const char* tn[]={"?","注册","激活","执行","拒绝","冻结","超时"}; const char* t=(type>=1&&type<=6)?tn[type]:"?"; printf("[审计] %s | %s | #%d\n",t,name,slot); }

/* ============================================================
 * 第 4 层: 看门狗线程 (核心)
 * ============================================================ */
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
 * 第 5 层: 执行守门人 (集成时间检查)
 * ============================================================ */
void mqf_init(){ memset(&K,0,sizeof(K)); K.ver=3; memset(&AUDIT,0,sizeof(AUDIT)); mqf_load_contracts(); printf("[内核 v%s] 时间切片版就绪 最多=%d 窗口级1/3/0s\n\n",MQF_VERSION,MAX_EXEC_SLOTS); mqf_watchdog_start(); }
int mqf_reg(const char* name, void(*fn)(void*), void* arg){ mqf_identity_t id=mqf_encode(name); capability_t cap; mqf_evaluate(name,&cap); memcpy(cap.auth_code,id.auth_code,MQF_HASH_LEN+1); if(cap.perm==PERM_NONE){ printf("[注册] %s → 僵尸(0级)\n",name); mqf_audit_log(AUDIT_REJECT,name,-1); return -1; } int i=-1; pthread_mutex_lock(&K_LOCK); for(int j=0;j<MAX_EXEC_SLOTS;j++) if(!K.s[j].used){ i=j; break; } if(i<0){ pthread_mutex_unlock(&K_LOCK); printf("[注册] %s → 拒绝(无槽位)\n",name); return -1; } K.s[i].used=1; memcpy(K.s[i].auth,cap.auth_code,MQF_HASH_LEN+1); strncpy(K.s[i].name,name,127); K.s[i].st=EXEC_PENDING; K.s[i].cap=cap; K.s[i].fn=fn; K.s[i].arg=arg; K.s[i].exec_count=0; pthread_mutex_unlock(&K_LOCK); printf("[注册] %s → 槽#%d 权限:%x 时限:%ds\n",name,i,cap.perm,cap.time_limit_sec); mqf_audit_log(AUDIT_REGISTER,name,i); return i; }

int mqf_on(int s, const char* auth){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_PENDING){ pthread_mutex_unlock(&K_LOCK); return 0; } if(strcmp(p->cap.auth_code,auth)!=0){ p->st=EXEC_FROZEN; K.frozen++; pthread_mutex_unlock(&K_LOCK); printf("[激活] %s 码错→冻结\n",p->name); mqf_audit_log(AUDIT_FREEZE,p->name,s); return 0; } p->st=EXEC_ALIVE; p->activated_at=time(NULL); p->expire_at=p->activated_at+p->cap.time_limit_sec; K.alive++; pthread_mutex_unlock(&K_LOCK); printf("[激活] %s → ALIVE (时限 %ds)\n",p->name,p->cap.time_limit_sec); mqf_audit_log(AUDIT_ACTIVATE,p->name,s); return 1; }

int mqf_go(int s){ if(s<0||s>=MAX_EXEC_SLOTS) return 0; pthread_mutex_lock(&K_LOCK); slot_t* p=&K.s[s]; if(!p->used||p->st!=EXEC_ALIVE){ pthread_mutex_unlock(&K_LOCK); printf("[执行] %s 非ALIVE(#%d)\n",p->name,s); return 0; } if(!(p->cap.perm&PERM_EXEC)){ p->st=EXEC_FROZEN; K.frozen++; K.alive--; pthread_mutex_unlock(&K_LOCK); printf("[执行] %s 无EXEC→冻结\n",p->name); mqf_audit_log(AUDIT_FREEZE,p->name,s); return 0; } double elapsed=difftime(time(NULL),p->activated_at); if(p->cap.time_limit_sec>0 && elapsed>(double)p->cap.time_limit_sec){ p->st=EXEC_SLEEP; K.zombie++; K.alive--; pthread_mutex_unlock(&K_LOCK); printf("[执行] %s 超时休眠\n",p->name); return 0; } p->exec_count++; printf("[执行] %s ▶ (第%d次, 已用%.0fs/%ds)\n",p->name,p->exec_count,elapsed,p->cap.time_limit_sec); p->fn(p->arg); pthread_mutex_unlock(&K_LOCK); mqf_audit_log(AUDIT_EXECUTE,p->name,s); return 1; }

/* ============================================================
 * 第 6 层: 演示 — v0.3 时间切片 + 看门狗
 * ============================================================ */
void fn_a(void*p){(void)p;printf("  [android] 不应该到达\n");}
void fn_i(void*p){(void)p;printf("  [ios] 全权限执行中(无时间限制)\n");}
void fn_w(void*p){(void)p;printf("  [wechat] 半权限 3s 窗口\n");}

int main(){
    mqf_init();
    printf("--- v0.3 时间切片 + 看门狗 ---\n\n");
    int a=mqf_reg("android",fn_a,NULL);
    int i=mqf_reg("ios",fn_i,NULL);
    int w=mqf_reg("wechat",fn_w,NULL);
    if(i>=0) mqf_on(i,K.s[i].auth);
    if(w>=0) mqf_on(w,K.s[w].auth);
    
    /* 第一轮: 都在窗口内 */
    printf("\n--- 第1轮: 窗口内执行 ---\n\n");
    mqf_go(i);
    mqf_go(w);
    
    /* 第2轮: wechat 超时被看门狗关掉, ios 无限还能跑 */
    printf("\n--- 第2轮: wechat 3s后冻结 ---\n\n");
    printf("[主线程] 等待 4s, 让看门狗轮询...\n");
    sleep(4);
    printf("[主线程] 再次执行:\n");
    mqf_go(i);
    mqf_go(w); /* 此时已自动冻结 */
    
    mqf_watchdog_stop();
    
    printf("\n--- 总结 ---\n");
    mqf_audit_summary();
    return 0;
}

/* 审计摘要 */
void mqf_audit_summary(){ printf("\n=== 审计摘要 ===\n"); printf("已记录 %d 条事件\n",AUDIT.count); printf("槽位: alive=%d frozen=%d zombie=%d\n",K.alive,K.frozen,K.zombie); for(int i=0;i<AUDIT.count && i<8;i++){ const char* tn[]={"?","注册","激活","执行","拒绝","冻结","超时"}; int idx=(AUDIT.wp - AUDIT.count + i + AUDIT_RING_SIZE)%AUDIT_RING_SIZE; printf("  %d. [%s] %s #%d\n",i+1,tn[AUDIT.e[idx].type],AUDIT.e[idx].name,AUDIT.e[idx].slot); } printf("===============\n\n"); }
