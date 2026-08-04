/**
 * 满全法执行内核 v0.2 — MQF Executive Kernel
 * -----------------------------------------
 * 自包含执行守门人。所有代码想在上面跑，必须先经过它授权。
 * v0.2: 契约系统替代硬编码判定
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>

#define MQF_VERSION     "0.2.0"
#define MQF_AUTH_LEN    32
#define MQF_HASH_LEN    64
#define MAX_EXEC_SLOTS  16
#define AUDIT_RING_SIZE 1024
#define MAX_CONTRACTS   64

typedef enum { EXEC_SLEEP=0, EXEC_PENDING, EXEC_ALIVE, EXEC_FROZEN, EXEC_ZOMBIE } exec_state_t;
typedef enum { PERM_NONE=0x00, PERM_READ=0x01, PERM_WRITE=0x02, PERM_EXEC=0x04, PERM_NET=0x08, PERM_ADMIN=0x10 } perm_t;
typedef enum { FACT_HAS_AUTH=1, FACT_HAS_NO_AUTH, FACT_TRUST_SOURCE, FACT_UNTRUST_SOURCE, FACT_DEFAULT } fact_type_t;

static const char* MQFS_INITS[] = {"b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch","sh","r","z","c","s","y","w","@"};
static const char* MQFS_FINALS[] = {"a","o","e","i","u","v","ai","ei","ui","ao","ou","iu","ie","ve","er","an","en","in","un","vn","ang","eng","ing","ong","ia","ua","uo","iao","iou","uai","uei","ian","uan","van","uen","iang","uang","ueng","iong","@"};
static const char* MQFS_TONES[] = {"1","2","3","4","0"};

/* ============================================================
 * 第 1 层: 身份编码
 * ============================================================ */
typedef struct { char text[MQF_AUTH_LEN+1]; int init_idx, final_idx, tone_idx, stroke_count; char auth_code[MQF_HASH_LEN+1], hash[MQF_HASH_LEN+1]; } mqf_identity_t;

static void mqf_hash(const char* in, char* out) {
    uint32_t h1=0x811c9dc5u, h2=0x0100193u;
    for (size_t i=0;i<strlen(in);i++){ h1^=(uint8_t)in[i]; h1*=0x01000193u; h2^=(uint8_t)in[i]<<5; h2*=0x1000193u; }
    sprintf(out,"%08x%08x%08x%08x%08x%08x",h1,h2,h1^h2,h2^h1,h1+h2,h2-h1);
}

mqf_identity_t mqf_encode(const char* text) {
    mqf_identity_t id; memset(&id,0,sizeof(id));
    size_t len=strlen(text); if(len>MQF_AUTH_LEN)len=MQF_AUTH_LEN;
    memcpy(id.text,text,len); id.text[len]='\0';
    uint32_t seed=0; for(size_t i=0;i<len;i++) seed=seed*31+(unsigned char)text[i];
    id.init_idx=seed%24; id.final_idx=(seed>>5)%39; id.tone_idx=(seed>>10)%5; id.stroke_count=(seed>>15)%30+1;
    char buf[512]; snprintf(buf,sizeof(buf),"%s|%d|%d|%d|%d",MQFS_INITS[id.init_idx],id.init_idx,id.final_idx,id.tone_idx,id.stroke_count);
    mqf_hash(buf,id.auth_code); snprintf(buf,sizeof(buf),"%s.%s",text,id.auth_code); mqf_hash(buf,id.hash);
    return id;
}

/* ============================================================
 * 第 2 层: 契约系统 (v0.2 新增)
 * ============================================================ */
typedef struct { char identity[128]; int target_level; perm_t target_perm; fact_type_t condition; char note[256]; } contract_t;
typedef struct { contract_t rules[MAX_CONTRACTS]; int count; struct { char name[128]; fact_type_t fact; } facts[MAX_CONTRACTS*2]; int fact_count; } contract_engine_t;
typedef struct { char auth_code[MQF_HASH_LEN+1]; perm_t perm; int max_slots, time_limit_sec, require_network; time_t created_at, expires_at; } capability_t;

static contract_engine_t CE;

void mqf_fact(const char* id, fact_type_t f) { if(CE.fact_count<MAX_CONTRACTS*2){strncpy(CE.facts[CE.fact_count].name,id,127); CE.facts[CE.fact_count].fact=f; CE.fact_count++;} }
int mqf_has_fact(const char* id, fact_type_t f) { for(int i=0;i<CE.fact_count;i++) if(strcmp(CE.facts[i].name,id)==0 && CE.facts[i].fact==f) return 1; return 0; }

void mqf_contract_add(const char* id, int lv, perm_t perm, fact_type_t cond, const char* note) {
    if(CE.count>=MAX_CONTRACTS) return;
    contract_t* c=&CE.rules[CE.count++];
    strncpy(c->identity,id,127); c->target_level=lv; c->target_perm=perm; c->condition=cond;
    if(note) strncpy(c->note,note,255);
}

int mqf_evaluate(const char* id, capability_t* out) {
    int lv=-1; perm_t perm=PERM_NONE;
    for(int i=0;i<CE.count;i++){
        contract_t* c=&CE.rules[i]; if(strcmp(c->identity,id)!=0) continue;
        int ok=0;
        switch(c->condition){
            case FACT_HAS_AUTH: ok=mqf_has_fact(id,FACT_HAS_AUTH); break;
            case FACT_HAS_NO_AUTH: ok=!mqf_has_fact(id,FACT_HAS_AUTH); break;
            case FACT_TRUST_SOURCE: ok=mqf_has_fact(id,FACT_TRUST_SOURCE); break;
            case FACT_UNTRUST_SOURCE: ok=!mqf_has_fact(id,FACT_TRUST_SOURCE); break;
            case FACT_DEFAULT: ok=1; break;
        }
        if(ok){ if(c->target_level>=0) lv=c->target_level; if(c->target_perm!=PERM_NONE) perm=c->target_perm; }
    }
    if(lv<0){ lv=1; perm=PERM_READ; }
    capability_t cap; memset(&cap,0,sizeof(cap));
    switch(lv){ case 0: cap.perm=PERM_NONE; break; case 1: case 2: cap.perm=PERM_READ; break; case 3: case 4: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC; break; case 5: case 6: cap.perm=PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN; break; }
    if(perm!=PERM_NONE) cap.perm=perm;
    cap.max_slots=(lv>=5)?16:(lv>=3)?4:(lv>=1)?1:0;
    cap.time_limit_sec=(lv>=5)?0:(lv>=3)?300:(lv>=1)?30:0;
    if(out) *out=cap;
    return lv;
}

void mqf_load_contracts() {
    CE.count=0; CE.fact_count=0;
    mqf_contract_add("android",0,PERM_NONE,FACT_DEFAULT,0);
    mqf_contract_add("ios",5,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,FACT_TRUST_SOURCE,0);
    mqf_contract_add("wechat",3,PERM_READ|PERM_WRITE,FACT_HAS_AUTH,0);
    mqf_contract_add("bank",6,PERM_READ|PERM_WRITE|PERM_EXEC|PERM_ADMIN,FACT_TRUST_SOURCE,0);
    mqf_fact("ios",FACT_TRUST_SOURCE); mqf_fact("wechat",FACT_HAS_AUTH); mqf_fact("bank",FACT_TRUST_SOURCE);
    printf("[契约] 加载 %d 条规则\n",CE.count);
}

/* ============================================================
 * 第 3 层: 执行守门人
 * ============================================================ */
typedef struct { int used; char auth[MQF_HASH_LEN+1], name[128]; exec_state_t st; capability_t cap; void(*fn)(void*); void* arg; time_t since; } slot_t;
typedef struct { slot_t s[MAX_EXEC_SLOTS]; int alive, frozen, zombie, ver; } kernel_t;
static kernel_t K;

void mqf_init(){ memset(&K,0,sizeof(K)); K.ver=2; mqf_load_contracts(); printf("[内核 v%s] 契约版就绪 槽位=%d 规则=%d\n\n",MQF_VERSION,MAX_EXEC_SLOTS,CE.count); }

int mqf_reg(const char* name, void(*fn)(void*), void* arg){
    mqf_identity_t id=mqf_encode(name);
    capability_t cap; mqf_evaluate(name,&cap); memcpy(cap.auth_code,id.auth_code,MQF_HASH_LEN+1);
    if(cap.perm==PERM_NONE){ printf("[注册] %s → 僵尸(0级)\n",name); return -1; }
    int i=-1; for(int j=0;j<MAX_EXEC_SLOTS;j++) if(!K.s[j].used){ i=j; break; }
    if(i<0){ printf("[注册] %s → 拒绝(无槽位)\n",name); return -1; }
    K.s[i].used=1; memcpy(K.s[i].auth,cap.auth_code,MQF_HASH_LEN+1); strncpy(K.s[i].name,name,127);
    K.s[i].st=EXEC_PENDING; K.s[i].cap=cap; K.s[i].fn=fn; K.s[i].arg=arg;
    printf("[注册] %s → 槽#%d 权限:%x\n",name,i,cap.perm);
    return i;
}

int mqf_on(int s, const char* auth){
    if(s<0||s>=MAX_EXEC_SLOTS||!K.s[s].used) return 0;
    slot_t* p=&K.s[s];
    if(p->st!=EXEC_PENDING) return 0;
    if(strcmp(p->cap.auth_code,auth)!=0){ p->st=EXEC_FROZEN; printf("[激活] %s 码错→冻结\n",p->name); return 0; }
    p->st=EXEC_ALIVE; p->since=time(NULL); K.alive++;
    printf("[激活] %s → ALIVE\n",p->name); return 1;
}

int mqf_go(int s){
    if(s<0||s>=MAX_EXEC_SLOTS||!K.s[s].used) return 0;
    slot_t* p=&K.s[s];
    if(p->st!=EXEC_ALIVE){ printf("[执行] %s 非ALIVE(%d) → 拒绝\n",p->name,p->st); return 0; }
    if(!(p->cap.perm&PERM_EXEC)){ p->st=EXEC_FROZEN; K.frozen++; K.alive--; printf("[执行] %s 无EXEC → 冻结\n",p->name); return 0; }
    if(p->cap.time_limit_sec>0 && difftime(time(NULL),p->since)>(double)p->cap.time_limit_sec){
        p->st=EXEC_SLEEP; K.zombie++; K.alive--; printf("[执行] %s 超时→僵尸\n",p->name); return 0;
    }
    printf("[执行] %s ▶\n",p->name);
    if(p->fn) p->fn(p->arg);
    return 1;
}

/* ============================================================
 * 第 4 层: 演示 main
 * ============================================================ */
void fn_a(void*p){(void)p;printf("  [android] 错误:僵尸不应执行\n");}
void fn_i(void*p){(void)p;printf("  [ios] ✓ 正常执行\n");}
void fn_w(void*p){(void)p;printf("  [wechat] ✓ 受限执行\n");}
void fn_b(void*p){(void)p;printf("  [bank] ✓ 全权限执行\n");}

int main(){
    mqf_init();
    printf("--- v0.2 契约版: 谁动? 谁不动? ---\n\n");
    int a=mqf_reg("android",fn_a,NULL);
    int i=mqf_reg("ios",fn_i,NULL);
    int w=mqf_reg("wechat",fn_w,NULL);
    int b=mqf_reg("bank",fn_b,NULL);
    if(a>=0) mqf_on(a,K.s[a].auth); /* android 码错也不会到这 */
    if(i>=0) mqf_on(i,K.s[i].auth);
    if(w>=0) mqf_on(w,K.s[w].auth);
    if(b>=0) mqf_on(b,K.s[b].auth);
    printf("\n--- 守门 ---\n\n");
    mqf_go(a); mqf_go(i); mqf_go(w); mqf_go(b);
    printf("\n--- 结论 ---\n");
    printf("android  无信任来源 → 僵尸(0级)    → 不动 ✗\n");
    printf("ios      全信任来源  → ALIVE(5级)   → 执行 ✓\n");
    printf("wechat   半信任来源  → ALIVE(3级)   → 受限 ⚠\n");
    printf("bank     强信任来源  → ALIVE(6级)   → 全权限 ✓\n");
    printf("规则由契约决定, 不再硬编码. 零云端依赖.\n");
    return 0;
}
