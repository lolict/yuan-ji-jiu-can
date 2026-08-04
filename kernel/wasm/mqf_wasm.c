/**
 * 满全法 WASM 内核 v1.7 — mqf_wasm.c
 * --------------------------------
 * 七元裁定引擎的 WASI 移植版
 * 无 pthread,无文件系统 IO,纯 stdin/stdout JSON
 *
 * 编译(WASI SDK):
 *   /opt/wasi-sdk/bin/clang --target=wasm32-wasi -O2 -o mqf_core.wasm mqf_wasm.c
 *
 * 运行:
 *   echo '{"entity":"ios","compliance_a":0.9,"compliance_b":0.85,"force":15,"angle":45}' | wasmtime mqf_core.wasm
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MQF_WASM_VERSION "2.0.0"
#define MAX_OBJECTS      64
#define MAX_PAIRS        16
#define MAX_NEST         32
#define MAX_NAME_LEN     64
#define MAX_INPUT        4096
#define MAX_OUTPUT       8192

/* ========== 五元旋钮 ========== */

#define FIVE_YUAN_STEPS 5
#define FIVE_YUAN_OCCUPY_THRESHOLD 100

typedef struct {
    char name[MAX_NAME_LEN];
    int step;
    float angle;
    float force;
    float ratio_in;
    float ratio_out;
    int   accumulated_ticks;
    int   occupied;
} five_yuan_state_t;

static five_yuan_state_t fy_nodes[MAX_OBJECTS];
static int fy_count = 0;

void mqf_five_yuan_init(void) {
    memset(fy_nodes, 0, sizeof(fy_nodes));
    fy_count = 0;
}

int mqf_five_yuan_register(const char* name) {
    if (fy_count >= MAX_OBJECTS) return -1;
    int idx = fy_count++;
    strncpy(fy_nodes[idx].name, name, MAX_NAME_LEN - 1);
    fy_nodes[idx].step = 0;
    fy_nodes[idx].angle = 0.0f;
    fy_nodes[idx].force = 0.0f;
    fy_nodes[idx].ratio_in = 1.0f;
    fy_nodes[idx].ratio_out = 1.0f;
    fy_nodes[idx].accumulated_ticks = 0;
    fy_nodes[idx].occupied = 0;
    return idx;
}

void mqf_five_yuan_push(int obj_id, float angle_inc, float force_inc) {
    if (obj_id < 0 || obj_id >= fy_count) return;
    five_yuan_state_t* s = &fy_nodes[obj_id];
    if (s->step >= FIVE_YUAN_STEPS) return;
    s->step++;
    s->angle += angle_inc;
    while (s->angle >= 360.0f) s->angle -= 360.0f;
    while (s->angle < 0.0f) s->angle += 360.0f;
    s->force += force_inc;
    s->accumulated_ticks += (int)(force_inc * 10.0f);
    if (s->accumulated_ticks > FIVE_YUAN_OCCUPY_THRESHOLD) {
        s->occupied = 1;
    }
}

/* ========== 六元对称群 ========== */

typedef struct {
    char name[MAX_NAME_LEN];
    float trust_a;
    float trust_b;
    float trust_intersection;
    int   mirror_ok;
    int   verdict;
} six_yuan_pair_t;

static six_yuan_pair_t sy_pairs[MAX_PAIRS];
static int sy_count = 0;

void mqf_six_yuan_init(void) {
    memset(sy_pairs, 0, sizeof(sy_pairs));
    sy_count = 0;
}

int mqf_six_yuan_register_pair(const char* name) {
    if (sy_count >= MAX_PAIRS) return -1;
    int idx = sy_count++;
    strncpy(sy_pairs[idx].name, name, MAX_NAME_LEN - 1);
    sy_pairs[idx].trust_a = 0.5f;
    sy_pairs[idx].trust_b = 0.5f;
    sy_pairs[idx].trust_intersection = 0.5f;
    sy_pairs[idx].mirror_ok = 0;
    sy_pairs[idx].verdict = 0;
    return idx;
}

void mqf_six_yuan_set_trust(int pair_id, float ta, float tb) {
    if (pair_id < 0 || pair_id >= sy_count) return;
    sy_pairs[pair_id].trust_a = ta;
    sy_pairs[pair_id].trust_b = tb;
    sy_pairs[pair_id].trust_intersection = (ta < tb) ? ta : tb;
    float diff = ta - tb;
    if (diff < 0.0f) diff = -diff;
    sy_pairs[pair_id].mirror_ok = (diff < 0.2f) ? 1 : 0;
    sy_pairs[pair_id].verdict = (sy_pairs[pair_id].trust_intersection > 0.6f && sy_pairs[pair_id].mirror_ok) ? 1 : 0;
}

int mqf_six_yuan_verdict(int pair_id) {
    if (pair_id < 0 || pair_id >= sy_count) return 0;
    return sy_pairs[pair_id].verdict;
}

/* ========== 七元蜂窝套壳 ========== */

#define MAX_CHILD 6

typedef struct {
    char name[MAX_NAME_LEN];
    int level;
    int parent_id;
    int child_ids[MAX_CHILD];
    int child_count;
    float space_ratio;
} shell_nest_t;

static shell_nest_t sn_nodes[MAX_NEST];
static int sn_count = 0;

void mqf_shell_nest_init(void) {
    memset(sn_nodes, 0, sizeof(sn_nodes));
    sn_count = 0;
}

int mqf_shell_nest_wrap(const char* name) {
    if (sn_count >= MAX_NEST) return -1;
    int idx = sn_count++;
    strncpy(sn_nodes[idx].name, name, MAX_NAME_LEN - 1);
    sn_nodes[idx].level = 0;
    sn_nodes[idx].parent_id = -1;
    sn_nodes[idx].child_count = 0;
    sn_nodes[idx].space_ratio = 1.0f;
    return idx;
}

int mqf_shell_nest_insert(int parent_id, const char* child_name) {
    if (parent_id < 0 || parent_id >= sn_count) return -1;
    if (sn_count >= MAX_NEST) return -1;
    shell_nest_t* parent = &sn_nodes[parent_id];
    if (parent->child_count >= MAX_CHILD) return -1;
    int idx = sn_count++;
    strncpy(sn_nodes[idx].name, child_name, MAX_NAME_LEN - 1);
    sn_nodes[idx].level = parent->level + 1;
    sn_nodes[idx].parent_id = parent_id;
    sn_nodes[idx].child_count = 0;
    sn_nodes[idx].space_ratio = parent->space_ratio / (float)MAX_CHILD;
    parent->child_ids[parent->child_count++] = idx;
    return idx;
}

/* ========== 裁定主引擎 ========== */

typedef struct {
    char entity[MAX_NAME_LEN];
    float compliance_a;
    float compliance_b;
    float force;
    float angle;
} verdict_request_t;

typedef struct {
    int   verdict;          /* 1=TRUSTED, 0=REJECTED */
    float confidence;       /* 信任交集 */
    int   mirror_align;     /* 镜像对齐 */
    int   ticks;            /* 五元刻度 */
    int   occupied;         /* 1=铭记, 0=易逝 */
    float final_angle;      /* 五元最终角度 */
} verdict_result_t;

static verdict_result_t mqf_verdict_engine(verdict_request_t* req) {
    verdict_result_t res;
    memset(&res, 0, sizeof(res));

    /* 五元:角度 wrap 360 */
    float a = req->angle;
    while (a >= 360.0f) a -= 360.0f;
    while (a < 0.0f) a += 360.0f;
    res.final_angle = a;
    res.ticks = (int)(req->force * 10.0f);
    res.occupied = (res.ticks > FIVE_YUAN_OCCUPY_THRESHOLD) ? 1 : 0;

    /* 六元:信任交集 + 镜像对齐 */
    res.confidence = (req->compliance_a < req->compliance_b) ? req->compliance_a : req->compliance_b;
    float diff = req->compliance_a - req->compliance_b;
    if (diff < 0.0f) diff = -diff;
    res.mirror_align = (diff < 0.2f) ? 1 : 0;
    res.verdict = (res.confidence > 0.6f && res.mirror_align) ? 1 : 0;

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

int main(void) {
    char input[MAX_INPUT];
    char output[MAX_OUTPUT];
    verdict_request_t req;
    memset(&req, 0, sizeof(req));
    req.compliance_a = 0.5f;
    req.compliance_b = 0.5f;
    req.force = 10.0f;
    req.angle = 45.0f;

    /* 读取 stdin JSON */
    int total = 0;
    int n;
    while ((n = fread(input + total, 1, sizeof(input) - total - 1, stdin)) > 0) {
        total += n;
        if (total >= (int)sizeof(input) - 1) break;
    }
    input[total] = '\0';

    /* 解析 JSON */
    const char* pv;
    pv = json_find_key(input, "entity");
    if (pv) {
        while (*pv == ' ' || *pv == '"') pv++;
        int i = 0;
        while (*pv && *pv != '"' && *pv != ',' && *pv != '}' && i < MAX_NAME_LEN - 1) {
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
    verdict_result_t res = mqf_verdict_engine(&req);

    /* 生成 JSON 输出 */
    snprintf(output, sizeof(output),
        "{\n"
        "  \"ok\": true,\n"
        "  \"version\": \"" MQF_WASM_VERSION "\",\n"
        "  \"entity\": \"%s\",\n"
        "  \"mqf_verdict\": {\n"
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
        res.verdict ? "已被铭记 — 双重背书,可信任" : "仍在观察 — 单方或双方否决"
    );

    fwrite(output, 1, strlen(output), stdout);
    fflush(stdout);
    return 0;
}
