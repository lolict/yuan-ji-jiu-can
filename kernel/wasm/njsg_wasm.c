/**
 * 囡囝共生 WASM 内核 v1.0 — 囡囝共生 Symbiosis WASM Kernel
 * -------------------------------------------------------
 * 囡囝:方言,意为小孩,萌而正向
 * 共生:共识圆 · 共生圆 · 共生姻 · 共首源 · 共首一 · 共守约 · 感受益
 * 人名谱: 刘连理/刘楚恬/前芳/萝莉兰/零离俩/零厘里/零劳累等
 *
 * 七元裁定引擎的 WASI 移植版
 * 无 pthread,无文件系统 IO,纯 stdin/stdout JSON
 *
 * 编译(WASI SDK):
 *   /opt/wasi-sdk/bin/clang --target=wasm32-wasi -O2 -o njsg_core.wasm njsg_wasm.c
 *
 * 运行:
 *   echo '{"entity":"ios","compliance_a":0.9,"compliance_b":0.85,"force":15,"angle":45}' | wasmtime njsg_core.wasm
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>
#include <stddef.h>

#define NJSG_WASM_VERSION "2.0.0"
#define NJSG_MAX_OBJECTS      64
#define NJSG_MAX_PAIRS        16
#define NJSG_MAX_NEST         32
#define NJSG_MAX_NAME_LEN     64
#define NJSG_MAX_INPUT        4096
#define NJSG_MAX_OUTPUT       8192

/* 32/64 位同源码:阶梯制·双向公约 */


/* ========== 五元旋钮 ========== */

#define FIVE_YUAN_STEPS 5
#define FIVE_YUAN_OCCUPY_THRESHOLD 100

/* 六元阈值 */
#define NJSG_MIRROR_ALIGN_TOLERANCE  0.2f
#define NJSG_TRUST_VERDICT_THRESHOLD  0.6f

/* 默认输入(无 stdin 时) */
#define NJSG_DEFAULT_COMPLIANCE 0.5f
#define NJSG_DEFAULT_FORCE      10.0f
#define NJSG_DEFAULT_ANGLE      45.0f

/* 裁定结果枚举状标签 */
#define NJ_REJECTED 0
#define NJ_TRUSTED  1
#define NJ_OFFSET   0
#define NJ_ALIGNED  1
#define NJ_EPHEMERAL 0
#define NJ_ETERNAL   1

/* 哲学回应(裁定通过时) */
#define NJ_PHRASE_TRUSTED "已被铭记 — 双重背书,可共生"
#define NJ_PHRASE_REJECTED "仍在观察 — 单方或双方否决,待共首源"

typedef struct {
    char name[NJSG_MAX_NAME_LEN];
    int32_t step;
    float   angle;
    float   force;
    float   ratio_in;
    float   ratio_out;
    int32_t accumulated_ticks;
    int32_t occupied;
} five_yuan_state_t;

static five_yuan_state_t fy_nodes[NJSG_MAX_OBJECTS];
static int32_t fy_count = 0;

void njsg_five_yuan_init(void) {
    memset(fy_nodes, 0, sizeof(fy_nodes));
    fy_count = 0;
}

int njsg_five_yuan_register(const char* name) {
    if (fy_count >= NJSG_MAX_OBJECTS) return -1;
    int idx = fy_count++;
    strncpy(fy_nodes[idx].name, name, NJSG_MAX_NAME_LEN - 1);
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
    if (s->step >= FIVE_YUAN_STEPS) return;
    s->step++;
    s->angle += angle_inc;
    while (s->angle >= 360.0f) s->angle -= 360.0f;
    while (s->angle < 0.0f) s->angle += 360.0f;
    s->force += force_inc;
    s->accumulated_ticks += (int32_t)(force_inc * 10.0f);
    if (s->accumulated_ticks > FIVE_YUAN_OCCUPY_THRESHOLD) {
        s->occupied = 1;
    }
}

/* ========== 六元对称群 ========== */

typedef struct {
    char name[NJSG_MAX_NAME_LEN];
    float trust_a;
    float trust_b;
    float trust_intersection;
    int32_t mirror_ok;
    int32_t verdict;
} six_yuan_pair_t;

static six_yuan_pair_t sy_pairs[NJSG_MAX_PAIRS];
static int32_t sy_count = 0;

void njsg_six_yuan_init(void) {
    memset(sy_pairs, 0, sizeof(sy_pairs));
    sy_count = 0;
}

int njsg_six_yuan_register_pair(const char* name) {
    if (sy_count >= NJSG_MAX_PAIRS) return -1;
    int idx = sy_count++;
    strncpy(sy_pairs[idx].name, name, NJSG_MAX_NAME_LEN - 1);
    sy_pairs[idx].trust_a = 0.5f;
    sy_pairs[idx].trust_b = 0.5f;
    sy_pairs[idx].trust_intersection = 0.5f;
    sy_pairs[idx].mirror_ok = 0;
    sy_pairs[idx].verdict = 0;
    return idx;
}

void njsg_six_yuan_set_trust(int pair_id, float ta, float tb) {
    if (pair_id < 0 || pair_id >= sy_count) return;
    sy_pairs[pair_id].trust_a = ta;
    sy_pairs[pair_id].trust_b = tb;
    sy_pairs[pair_id].trust_intersection = (ta < tb) ? ta : tb;
    float diff = ta - tb;
    if (diff < 0.0f) diff = -diff;
    sy_pairs[pair_id].mirror_ok = (diff < 0.2f) ? 1 : 0;
    sy_pairs[pair_id].verdict = (sy_pairs[pair_id].trust_intersection > 0.6f && sy_pairs[pair_id].mirror_ok) ? 1 : 0;
}

int njsg_six_yuan_verdict(int pair_id) {
    if (pair_id < 0 || pair_id >= sy_count) return 0;
    return sy_pairs[pair_id].verdict;
}

/* ========== 七元蜂窝套壳 ========== */

#define NJSG_MAX_CHILD 6

typedef struct {
    char name[NJSG_MAX_NAME_LEN];
    int32_t level;
    int32_t parent_id;
    int32_t child_ids[NJSG_MAX_CHILD];
    int32_t child_count;
    float space_ratio;
} shell_nest_t;

static shell_nest_t sn_nodes[NJSG_MAX_NEST];
static int32_t sn_count = 0;

void njsg_shell_nest_init(void) {
    memset(sn_nodes, 0, sizeof(sn_nodes));
    sn_count = 0;
}

int njsg_shell_nest_wrap(const char* name) {
    if (sn_count >= NJSG_MAX_NEST) return -1;
    int idx = sn_count++;
    strncpy(sn_nodes[idx].name, name, NJSG_MAX_NAME_LEN - 1);
    sn_nodes[idx].level = 0;
    sn_nodes[idx].parent_id = -1;
    sn_nodes[idx].child_count = 0;
    sn_nodes[idx].space_ratio = 1.0f;
    return idx;
}

int njsg_shell_nest_insert(int parent_id, const char* child_name) {
    if (parent_id < 0 || parent_id >= sn_count) return -1;
    if (sn_count >= NJSG_MAX_NEST) return -1;
    shell_nest_t* parent = &sn_nodes[parent_id];
    if (parent->child_count >= NJSG_MAX_CHILD) return -1;
    int idx = sn_count++;
    strncpy(sn_nodes[idx].name, child_name, NJSG_MAX_NAME_LEN - 1);
    sn_nodes[idx].level = parent->level + 1;
    sn_nodes[idx].parent_id = parent_id;
    sn_nodes[idx].child_count = 0;
    sn_nodes[idx].space_ratio = parent->space_ratio / (float)NJSG_MAX_CHILD;
    parent->child_ids[parent->child_count++] = idx;
    return idx;
}

/* ========== 裁定主引擎 ========== */

typedef struct {
    char entity[NJSG_MAX_NAME_LEN];
    float compliance_a;
    float compliance_b;
    float force;
    float angle;
} verdict_request_t;

typedef struct {
    int32_t verdict;        /* 1=TRUSTED, 0=REJECTED */
    float   confidence;     /* 信任交集 */
    int32_t mirror_align;   /* 镜像对齐 */
    int32_t ticks;          /* 五元刻度 */
    int32_t occupied;       /* 1=铭记, 0=易逝 */
    float   final_angle;    /* 五元最终角度 */
} verdict_result_t;

static verdict_result_t njsg_verdict_engine(verdict_request_t* req) {
    verdict_result_t res;
    memset(&res, 0, sizeof(res));

    /* 五元:角度 wrap 360 */
    float a = req->angle;
    while (a >= 360.0f) a -= 360.0f;
    while (a < 0.0f) a += 360.0f;
    res.final_angle = a;
    res.ticks = (int32_t)(req->force * 10.0f);
    res.occupied = (res.ticks > FIVE_YUAN_OCCUPY_THRESHOLD) ? 1 : 0;

    /* 六元:信任交集 + 镜像对齐 */
    res.confidence = (req->compliance_a < req->compliance_b) ? req->compliance_a : req->compliance_b;
    float diff = req->compliance_a - req->compliance_b;
    if (diff < 0.0f) diff = -diff;
    res.mirror_align = (diff < NJSG_MIRROR_ALIGN_TOLERANCE) ? NJ_ALIGNED : NJ_OFFSET;
    res.verdict = (res.confidence > NJSG_TRUST_VERDICT_THRESHOLD && res.mirror_align) ? NJ_TRUSTED : NJ_REJECTED;

    return res;
}

/* ========== 简单 JSON 解析/生成 ========== */

static const char* json_find_key(const char* json, const char* key) {
    char pattern[128];
    snprintf(pattern, sizeof(pattern), "\"%s\"", key);
    const char* p = strstr(json, pattern);
    if (!p) return NULL;
    p += strlen(pattern);
    while (*p && (*p == ' ' || *p == ':' || *p == '\t')) p++;
    return p;
}

static int json_parse_float(const char* p, float* out) {
    if (!p) return -1;
    *out = (float)strtod(p, NULL);
    return 0;
}

/* ========== WASI 主入口 ========== */

int main(int argc, char** argv) {
    char input[NJSG_MAX_INPUT];
    char output[NJSG_MAX_OUTPUT];
    verdict_request_t req;
    memset(&req, 0, sizeof(req)); /* 32/64 同源码:阶梯制·双向公约 */
    req.compliance_a = NJSG_DEFAULT_COMPLIANCE;
    req.compliance_b = NJSG_DEFAULT_COMPLIANCE;
    req.force = NJSG_DEFAULT_FORCE;
    req.angle = NJSG_DEFAULT_ANGLE;

    /* WASM v1.7 自检模式: ./njsg_wasm_test --selftest */
    if(argc > 1 && strcmp(argv[1], "--selftest") == 0){
        char buf[8192];
        printf("{\"selftest\":true,\n");

        printf(" \"perception\": ");
        njsg_perceive_route("{\"input\":\"lctfrqmy-ggt-xy\"}", buf, sizeof(buf));
        printf("%s,\n", buf);

        printf(" \"timeline\": ");
        njsg_timeline_record("{\"subsystem\":\"kernel\",\"event\":\"test_pulse\",\"value\":42}", buf, sizeof(buf));
        printf("%s,\n", buf);

        printf(" \"mccp\": ");
        njsg_mccp_compile_json("{\"name\":\"Container\",\"fields\":\"name:char[64];safety:int;owner:bag\"}", buf, sizeof(buf));
        printf("\"%s\",\n", buf);

        printf(" \"review\": ");
        njsg_review_submit("{\"target\":\"kernel_core\",\"score\":9,\"comment\":\"good\"}", buf, sizeof(buf));
        printf("%s\n", buf);

        printf("}\n");
        return 0;
    }

    /* 读取 stdin JSON */
    size_t total = 0;
    size_t n;
    while ((n = fread(input + total, 1, sizeof(input) - total - 1, stdin)) > 0) {
        total += n;
        if (total >= sizeof(input) - 1) break;
    }
    input[total] = '\0';

    /* 解析 JSON */
    const char* pv;
    pv = json_find_key(input, "entity");
    if (pv) {
        while (*pv == ' ' || *pv == '"') pv++;
        int i = 0;
        while (*pv && *pv != '"' && *pv != ',' && *pv != '}' && i < NJSG_MAX_NAME_LEN - 1) {
            req.entity[i++] = *pv++;
        }
        req.entity[i] = '\0';
    }
    pv = json_find_key(input, "compliance_a");
    json_parse_float(pv, &req.compliance_a);
    pv = json_find_key(input, "compliance_b");
    json_parse_float(pv, &req.compliance_b);
    pv = json_find_key(input, "force");
    json_parse_float(pv, &req.force);
    pv = json_find_key(input, "angle");
    json_parse_float(pv, &req.angle);

    /* 执行裁定 */
    verdict_result_t res = njsg_verdict_engine(&req);

    /* 生成 JSON 输出 */
    snprintf(output, sizeof(output),
        "{\n"
        "  \"ok\": true,\n"
        "  \"version\": \"" NJSG_WASM_VERSION "\",\n"
        "  \"entity\": \"%s\",\n"
        "  \"njsg_verdict\": {\n"
        "    \"trust\": \"%s\",\n"
        "    \"confidence\": %.3f,\n"
        "    \"mirror_alignment\": \"%s\",\n"
        "    \"five_yuan\": {\"ticks\":%d,\"occupied\":\"%s\",\"angle\":%.1f},\n"
        "    \"six_yuan\": {\"intersection\":%.3f,\"verdict\":%d}\n"
        "  },\n"
        "  \"philosophy\": \"%s\"\n"
        "}\n",
        req.entity[0] ? req.entity : "unknown",
        res.verdict ? "TRUSTED" : "REJECTED",
        res.confidence,
        res.mirror_align ? "ALIGNED" : "OFFSET",
        res.ticks,
        res.occupied ? "ETERNAL" : "EPHEMERAL",
        res.final_angle,
        res.confidence,
        res.verdict,
        res.verdict ? NJ_PHRASE_TRUSTED : NJ_PHRASE_REJECTED
    );

    fwrite(output, 1, strlen(output), stdout);
    fflush(stdout);
    return 0;
}

/**
 * 导出函数:供浏览器 JS 直接调用(绕过 WASI stdin/stdout)
 *
 * 输入:    in_json  — 输入 JSON 字符串指针(C string*,UTF-8)
 * 输出:    out_buf  — 输出缓冲区(写入裁定 JSON 结果)
 *          out_cap  — 输出缓冲区容量(字节)
 * 返回:    实际写入的字节数(不含 NUL); 若 out_cap=0 即返回所需大小。
 *
 * 这是阶梯制的关键握手点:
 *   C 五/六元算法  ←单一权威实现→  JS 壳通过内存共享传参/回读
 *   编译到 wasm32 或 wasm64,JS 侧调用路径完全一致。
 */
/* ========== v1.4+ 九元感知引擎(WASM 无 pthread 单线程版) ========== */

#define PERCEIVE_FAMILY_CONTAINER 0
#define PERCEIVE_FAMILY_PHALLIC   1
#define PERCEIVE_FAMILY_COUPLE    2
#define PERCEIVE_FAMILY_HOMOPHONE 3
#define PERCEIVE_FAMILY_DIRECT    4

static const char* perceive_family_name(int f){
    const char* names[] = {"container","phallic","couple","homophone","direct"};
    return (f >= 0 && f <= 4) ? names[f] : "unknown";
}

/* 谐音靶点路由表(同 C 内核) */
static const char* perceive_homophone_target(const char* in){
    if(strstr(in,"lct") || strstr(in,"liuchutian"))   return "Wife";
    if(strstr(in,"wgslz") || strstr(in,"lolict"))      return "Husband";
    if(strstr(in,"mlll") || strstr(in,"hunyuan"))      return "System";
    if(strstr(in,"mlct"))                              return "Wife";
    if(strstr(in,"mltt"))                              return "Wife";
    if(strstr(in,"gtt")  || strstr(in,"gttx"))         return "SoundMorphism";
    if(strstr(in,"mlgy"))                              return "WorldMerge";
    if(strstr(in,"kndl"))                              return "Kndl";
    if(strstr(in,"mlil"))                              return "Mlil";
    return "Unknown";
}

static int perceive_authority(const char* target){
    if(strcmp(target,"Wife")==0 || strcmp(target,"Husband")==0) return 1;
    if(strcmp(target,"System")==0) return 2;
    return 0;
}

/* 感知家族分类 */
static int perceive_classify(const char* in){
    /* ASCII 直接判定 */
    if(in[0]=='b' || in[0]=='s') return PERCEIVE_FAMILY_CONTAINER; /* bag/sack */
    /* UTF-8 lead >= 0xE0 视为 CJK → couple 家族(保守路由) */
    if((unsigned char)in[0] >= 0xE0)    return PERCEIVE_FAMILY_COUPLE;
    if(strstr(in,"lct") || strstr(in,"mlll")) return PERCEIVE_FAMILY_HOMOPHONE;
    return PERCEIVE_FAMILY_DIRECT;
}

/* 经 JSON API 封装:用于 JS 壳调用 */
__attribute__((export_name("njsg_perceive_route")))
int njsg_perceive_route(const char* in_json, char* out_buf, int out_cap){
    if(!in_json || !out_buf || out_cap <= 0) return 0;

    /* 解析 input 字段 */
    char input[256];
    input[0] = '\0';
    const char* pv = json_find_key(in_json, "input");
    if(pv){
        while(*pv==' '||*pv=='"') pv++;
        int i = 0;
        while(*pv && *pv!='"' && *pv!=',' && *pv!='}' && i < 255) input[i++] = *pv++;
        input[i] = '\0';
    }

    /* 分别器三层路由 */
    int  family    = perceive_classify(input);
    const char* target = perceive_homophone_target(input);
    int  authority = perceive_authority(target);
    int  ackable   = (authority > 0) ? 1 : 0;

    /* 生成输出 JSON */
    char tmp[1024];
    snprintf(tmp, sizeof(tmp),
        "{"
        "\"module\":\"v1.4_perception\","
        "\"input\":\"%s\","
        "\"family\":\"%s\","
        "\"target\":\"%s\","
        "\"authority\":%d,"
        "\"ackable\":%d"
        "}",
        input, perceive_family_name(family), target, authority, ackable
    );

    int len = (int)strlen(tmp);
    int copy = (len < out_cap - 1) ? len : out_cap - 1;
    memcpy(out_buf, tmp, copy);
    out_buf[copy] = '\0';
    return copy;
}

/* ========== v1.7+ 时间线一体化 ========== */

#define TIMELINE_SLOTS 16

typedef struct {
    char subsystem[24];
    char event[64];
    int  value;
} timeline_slot_t;

static timeline_slot_t tl_slots[TIMELINE_SLOTS];
static int tl_head = 0;
static int tl_count = 0;
static int tl_tick  = 0;

__attribute__((export_name("njsg_timeline_record")))
int njsg_timeline_record(const char* in_json, char* out_buf, int out_cap){
    if(!in_json || !out_buf || out_cap <= 0) return 0;

    const char* sv = json_find_key(in_json, "subsystem");
    const char* ev = json_find_key(in_json, "event");
    const char* vv = json_find_key(in_json, "value");

    char subsystem[24]; subsystem[0] = '\0';
    char event[64];    event[0] = '\0';
    int  value = 0;

    if(sv){ while(*sv==' '||*sv=='"') sv++; int i=0; while(*sv&&*sv!='"'&&*sv!='}'&&i<23) subsystem[i++]=*sv++; subsystem[i]='\0'; }
    if(ev){ while(*ev==' '||*ev=='"') ev++; int i=0; while(*ev&&*ev!='"'&&*ev!='}'&&i<63) event[i++]=*ev++; event[i]='\0'; }
    if(vv) value = (int)strtod(vv, NULL);

    int pos = tl_head;
    strncpy(tl_slots[pos].subsystem, subsystem, 23);
    strncpy(tl_slots[pos].event, event, 63);
    tl_slots[pos].value = value;
    tl_head = (tl_head + 1) % TIMELINE_SLOTS;
    if(tl_count < TIMELINE_SLOTS) tl_count++;
    tl_tick++;

    char tmp[512];
    snprintf(tmp, sizeof(tmp),
        "{\"module\":\"v1.7_timeline\",\"head\":%d,\"count\":%d,\"tick\":%d,\"latest\":\"%s:%s=%d\"}",
        pos, tl_count, tl_tick, subsystem, event, value
    );

    int len = (int)strlen(tmp);
    int copy = (len < out_cap - 1) ? len : out_cap - 1;
    memcpy(out_buf, tmp, copy);
    out_buf[copy] = '\0';
    return copy;
}

/* ========== v1.5+ MCCP 极简 DSL 编译器(JSON 输入) ========== */

__attribute__((export_name("njsg_mccp_compile_json")))
int njsg_mccp_compile_json(const char* in_json, char* out_buf, int out_cap){
    if(!in_json || !out_buf || out_cap <= 0) return 0;

    const char* name_ptr = json_find_key(in_json, "name");
    const char* body_ptr = json_find_key(in_json, "fields");

    char name[64]; name[0] = '\0';
    char body[512]; body[0] = '\0';

    if(name_ptr){ while(*name_ptr==' '||*name_ptr=='"') name_ptr++; int i=0; while(*name_ptr&&*name_ptr!='"'&&*name_ptr!='}'&&i<63) name[i++]=*name_ptr++; name[i]='\0'; }
    if(body_ptr){ while(*body_ptr==' '||*body_ptr=='"') body_ptr++; int i=0; while(*body_ptr&&*body_ptr!='}'&&i<511) body[i++]=*body_ptr++; body[i]='\0'; }

    char tmp[4096];
    int pos = 0;
    pos += snprintf(tmp+pos, sizeof(tmp)-pos, "/* MCCP WASM: %s */\n", name);
    pos += snprintf(tmp+pos, sizeof(tmp)-pos, "typedef struct {\n");

    /* 极简字段解析: "field1:type1;field2:type2" */
    char buf[1024];
    strncpy(buf, body, sizeof(buf)-1);
    char* saveptr = NULL;
    char* token = strtok_r(buf, ";", &saveptr);
    while(token){
        while(*token==' ') token++;
        char* colon = strchr(token, ':');
        if(colon){
            *colon = '\0';
            char* fname = token;
            char* ftype = colon+1;
            while(*ftype==' ') ftype++;
            pos += snprintf(tmp+pos, sizeof(tmp)-pos, "    %-12s %s;\n", ftype, fname);
        }
        token = strtok_r(NULL, ";", &saveptr);
    }
    pos += snprintf(tmp+pos, sizeof(tmp)-pos, "} %s_t;\n", name);

    int len = pos;
    int copy = (len < out_cap - 1) ? len : out_cap - 1;
    memcpy(out_buf, tmp, copy);
    out_buf[copy] = '\0';
    return copy;
}

/* ========== v1.6+ 裁判系统(JSON 输入简化版) ========== */

#define REVIEW_SLOTS 16

typedef struct {
    char target[32];
    int  sum_score;
    int  count;
    int  reward;
    int  penalty;
} review_summary_t;

static review_summary_t rv_summary[REVIEW_SLOTS];
static int rv_count = 0;

static int rv_find_slot(const char* target){
    for(int i=0;i<rv_count;i++){
        if(strcmp(rv_summary[i].target, target)==0) return i;
    }
    return -1;
}

__attribute__((export_name("njsg_review_submit")))
int njsg_review_submit(const char* in_json, char* out_buf, int out_cap){
    if(!in_json || !out_buf || out_cap <= 0) return 0;

    const char* tv = json_find_key(in_json, "target");
    const char* sv = json_find_key(in_json, "score");

    char target[32]; target[0] = '\0';
    int  score = 5;

    if(tv){ while(*tv==' '||*tv=='"') tv++; int i=0; while(*tv&&*tv!='"'&&*tv!='}'&&i<31) target[i++]=*tv++; target[i]='\0'; }
    if(sv) score = (int)strtod(sv, NULL);
    if(score < 0) score = 0;
    if(score > 10) score = 10;

    int slot = rv_find_slot(target);
    if(slot < 0 && rv_count < REVIEW_SLOTS){
        slot = rv_count++;
        strncpy(rv_summary[slot].target, target, 31);
        rv_summary[slot].sum_score = 0;
        rv_summary[slot].count = 0;
        rv_summary[slot].reward = 0;
        rv_summary[slot].penalty = 0;
    }
    if(slot >= 0){
        rv_summary[slot].sum_score += score;
        rv_summary[slot].count += 1;
        if(score >= 7) rv_summary[slot].reward += (score - 5) * 2;
        else if(score <= 3) rv_summary[slot].penalty += (5 - score) * 2;
    }

    /* 三审:非空评论 + 已见证(总分>0) + 锚点一致(奖励/惩罚与分方向一致) */
    int non_empty = (strlen(target) > 0) ? 1 : 0;
    int witnessed = (slot >= 0 && rv_summary[slot].count > 0) ? 1 : 0;
    int anchor_ok = 1; /* 单点提交,方向一致 */
    int verdict = (non_empty && witnessed && anchor_ok) ? 1 : -1;

    char tmp[512];
    snprintf(tmp, sizeof(tmp),
        "{"
        "\"module\":\"v1.6_review\","
        "\"target\":\"%s\","
        "\"score\":%d,"
        "\"total_submitted\":%d,"
        "\"verdict\":\"%s\","
        "\"current_reward\":%d,"
        "\"current_penalty\":%d"
        "}",
        target, score, (slot>=0?rv_summary[slot].count:0),
        verdict==1?"trusted":"rejected",
        (slot>=0?rv_summary[slot].reward:0),
        (slot>=0?rv_summary[slot].penalty:0)
    );

    int len = (int)strlen(tmp);
    int copy = (len < out_cap - 1) ? len : out_cap - 1;
    memcpy(out_buf, tmp, copy);
    out_buf[copy] = '\0';
    return copy;
}

__attribute__((export_name("njsg_verdict_json")))
int njsg_verdict_json(const char* in_json, char* out_buf, int out_cap) {
    if (!in_json) return 0;

    verdict_request_t req;
    memset(&req, 0, sizeof(req));
    req.compliance_a = NJSG_DEFAULT_COMPLIANCE;
    req.compliance_b = NJSG_DEFAULT_COMPLIANCE;
    req.force = NJSG_DEFAULT_FORCE;
    req.angle = NJSG_DEFAULT_ANGLE;

    /* 解析输入 JSON (与 main() 中解析逻辑一致) */
    const char* pv;
    pv = json_find_key(in_json, "entity");
    if (pv) {
        while (*pv == ' ' || *pv == '"') pv++;
        int i = 0;
        while (*pv && *pv != '"' && *pv != ',' && *pv != '}' && i < NJSG_MAX_NAME_LEN - 1) {
            req.entity[i++] = *pv++;
        }
        req.entity[i] = '\0';
    }
    pv = json_find_key(in_json, "compliance_a");
    json_parse_float(pv, &req.compliance_a);
    pv = json_find_key(in_json, "compliance_b");
    json_parse_float(pv, &req.compliance_b);
    pv = json_find_key(in_json, "force");
    json_parse_float(pv, &req.force);
    pv = json_find_key(in_json, "angle");
    json_parse_float(pv, &req.angle);

    /* 执行裁定 */
    verdict_result_t res = njsg_verdict_engine(&req);

    /* 生成输出 JSON */
    char tmp[NJSG_MAX_OUTPUT];
    snprintf(tmp, sizeof(tmp),
        "{"
        "\"ok\":true,"
        "\"version\":\"" NJSG_WASM_VERSION "\","
        "\"entity\":\"%s\","
        "\"njsg_verdict\":{"
            "\"trust\":\"%s\","
            "\"confidence\":%.3f,"
            "\"mirror_alignment\":\"%s\","
            "\"five_yuan\":{\"ticks\":%d,\"occupied\":\"%s\",\"angle\":%.1f},"
            "\"six_yuan\":{\"intersection\":%.3f,\"verdict\":%d}"
        "},"
        "\"philosophy\":\"%s\""
        "}",
        req.entity[0] ? req.entity : "unknown",
        res.verdict ? "TRUSTED" : "REJECTED",
        res.confidence,
        res.mirror_align ? "ALIGNED" : "OFFSET",
        res.ticks,
        res.occupied ? "ETERNAL" : "EPHEMERAL",
        res.final_angle,
        res.confidence,
        res.verdict,
        res.verdict ? NJ_PHRASE_TRUSTED : NJ_PHRASE_REJECTED
    );

    int len = (int)strlen(tmp);
    if (out_cap <= 0) return len;  /* 查询所需大小 */
    if (!out_buf) return 0;

    /* 安全拷贝(防溢出) */
    int copy = (len < out_cap - 1) ? len : out_cap - 1;
    memcpy(out_buf, tmp, copy);
    out_buf[copy] = '\0';
    return copy;
}
