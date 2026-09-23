/**
 * global_control_bridge_kernel.c — 全局控制容器 → 内核桥接
 *
 * 让内核和 Rust 核心通过全局控制容器来协调编码秩序。
 * 核心命题:
 *   符号用完后不重复重启, 从外部容器取符号遍历。
 *   匹配不靠编码关系, 靠发音位置和本质匹配。
 *
 * 这就是"聂小倩借尸还魂"的代码化:
 *   为鬼魂(符号本质)找到匹配的躯壳(另一容器中的符号),
 *   而不是按编码关系(大小写)找到有毒的蛇。
 */

#include <stdio.h>
#include <string.h>
#include "global_control_bridge_kernel.h"
#include "../../encoding/global_control.h"
#include "../../encoding/mind_organ.h"

/* ── 全局控制容器静态实例 ── */
static global_control_t gc;
static int initialized = 0;

/* ── 感知器官静态实例 ── */
static mo_engine_t mo_organ;

/* ── 初始化(一次性) ── */
void global_control_bridge_init(void) {
    if (initialized) return;
    initialized = 1;

    global_control_init(&gc);

    /* 注册五个核心容器 */
    gc_register_container(&gc, 0x10, "声母", GC_POLARITY_YANG, 48);
    gc_register_container(&gc, 0x20, "韵母", GC_POLARITY_YIN, 66);
    gc_register_container(&gc, 0x30, "声调", GC_POLARITY_NEUTRAL, 5);
    gc_register_container(&gc, 0x40, "家族", GC_POLARITY_NEUTRAL, 5);
    gc_register_container(&gc, 0x50, "结构", GC_POLARITY_NEUTRAL, 8);

    /* 遍历规则: 声母→韵母 (优先级1) */
    gc_add_traversal(&gc, 0x10, 0x20, 1);
    /* 遍历规则: 声调→声母 (优先级2) */
    gc_add_traversal(&gc, 0x30, 0x10, 2);
    /* 遍历规则: 家族→韵母 (优先级3) */
    gc_add_traversal(&gc, 0x40, 0x20, 3);

    /* 判断规则: 发音位置匹配, 需要极性, 容差10 */
    gc_set_judgment(&gc, GC_MATCH_POSITION, 1, 0, 10);

    /* 初始化感知器官 (阈值 5.0) */
    mind_organ_init(&mo_organ, 5.0);
}

/* ── 包装器: 转发到底层全局控制容器 ── */

int gcb_register_container(uint8_t id, const char *name,
                           uint8_t polarity, uint32_t boundary) {
    global_control_bridge_init();
    return gc_register_container(&gc, id, name, polarity, boundary);
}

int gcb_add_traversal(uint8_t src, uint8_t dst, uint8_t priority) {
    global_control_bridge_init();
    return gc_add_traversal(&gc, src, dst, priority);
}

int gcb_get_next_traversal(uint8_t *src_out, uint8_t *dst_out) {
    global_control_bridge_init();
    return gc_get_next_traversal(&gc, src_out, dst_out);
}

int gcb_advance_traversal(void) {
    global_control_bridge_init();
    return gc_advance_traversal(&gc);
}

void gcb_set_judgment(uint8_t match_type, uint8_t require_pol,
                       uint8_t require_aes, uint8_t tol) {
    global_control_bridge_init();
    gc_set_judgment(&gc, match_type, require_pol, require_aes, tol);
}

int gcb_judge_pair(uint32_t sym_a, uint32_t sym_b) {
    global_control_bridge_init();
    return gc_judge_pair(&gc, sym_a, sym_b);
}

/* ── 聂小倩: 为鬼魂找躯壳 ── */
int gcb_nie_xiaoqian(uint32_t ghost_code, uint32_t *body_out) {
    global_control_bridge_init();

    /* 从韵母容器(0x20)中找匹配的躯壳 */
    for (uint16_t yunmu = 0; yunmu < 66; yunmu++) {
        uint32_t candidate = ((uint32_t)0x20 << 8) | yunmu;
        int match = gc_judge_pair(&gc, ghost_code, candidate);
        if (match) {
            if (body_out) *body_out = candidate;
            return (int)yunmu;  /* 返回躯壳索引 */
        }
    }

    /* 没找到 → 进入恢复流程 */
    gc_recover(&gc);
    return -1;
}

/* ── 决策树: 恢复 ── */
int gcb_recover(void) {
    global_control_bridge_init();
    return gc_recover(&gc);
}

/* ── 感知器官: 距离感知聚类包装器 ── */
void gcb_mind_organ_init(double threshold) {
    global_control_bridge_init();
    mind_organ_init(&mo_organ, threshold);
}

int gcb_mo_add_cell(int id, uint32_t container_code) {
    global_control_bridge_init();
    return mind_organ_add_cell(&mo_organ, id, container_code);
}

double gcb_mo_distance(uint32_t code_a, uint32_t code_b) {
    return mind_organ_distance(code_a, code_b);
}

int gcb_mo_clusterize(void) {
    global_control_bridge_init();
    return mind_organ_clusterize(&mo_organ);
}

int gcb_mo_find_body(uint32_t ghost_code, double *dist_out) {
    global_control_bridge_init();
    return mind_organ_find_body(&mo_organ, ghost_code, dist_out);
}

int gcb_mo_cluster_count(void) {
    return mo_organ.cluster_count;
}

void gcb_mo_report(char *buf, int bufsize) {
    mind_organ_report(&mo_organ, buf, bufsize);
}

/* ═══════════════════════════════════════════════════════════
 * 子容器7: 征用层 (动态编号 · 跨编码身份不灭)
 *
 * 双后端设计:
 *   1. C 独立实现 (expropriation_bridge.c) — 自给自足
 *   2. Rust FFI 直通 (libmqf_ffi.so) — 内核→Rust 真实 API
 * ═══════════════════════════════════════════════════════════ */

#include "../../encoding/expropriation_bridge.h"

void gcb_exprop_init(exprop_identity_t *id, uint64_t core, int polarity) {
    exprop_init(id, core, polarity);
}

int gcb_exprop_expropriate(exprop_identity_t *id, const char *shell, uint64_t number) {
    return exprop_expropriate(id, shell, number);
}

int gcb_exprop_shed(exprop_identity_t *id, const char *shell) {
    return exprop_shed(id, shell);
}

void gcb_exprop_converge(exprop_identity_t *id) {
    exprop_converge(id);
}

int gcb_exprop_intact(const exprop_identity_t *id, uint64_t expected_core) {
    return exprop_is_intact(id, expected_core);
}

int gcb_exprop_density(const exprop_identity_t *id) {
    return exprop_density(id);
}

int gcb_exprop_property_claim(uint64_t host_core, uint64_t guest_core) {
    return exprop_property_claim(host_core, guest_core);
}

int gcb_exprop_static_dynamic(const exprop_identity_t *id, uint64_t expected_core) {
    return exprop_static_dynamic_balance(id, expected_core);
}

/* ── Rust FFI 直通: 调用 libmqf_ffi.so ── */

extern void* rust_exprop_create(uint64_t core, int32_t polarity);
extern int32_t rust_exprop_expropriate(void* handle, const char* shell_name, uint64_t borrowed_num);
extern int32_t rust_exprop_shed(void* handle, const char* shell_name);
extern int32_t rust_exprop_intact(void* handle, uint64_t expected_core);
extern int32_t rust_exprop_density(void* handle);
extern uint64_t rust_exprop_core(void* handle);
extern void rust_exprop_free(void* handle);
extern int32_t rust_exprop_selftest(void);

int gcb_rust_exprop_selftest(void) {
    return rust_exprop_selftest();
}

int gcb_rust_exprop_demo(uint64_t core, int32_t polarity) {
    void* h = rust_exprop_create(core, polarity);
    if (!h) return -1;
    const char* shells[] = {"Rust", "Python", "YAML", "JSON", "Nim", "Zig"};
    int n = 6;
    for (int i = 0; i < n; i++)
        rust_exprop_expropriate(h, shells[i], core);
    for (int i = 0; i < n; i++)
        rust_exprop_shed(h, shells[i]);
    int ok = rust_exprop_intact(h, core);
    rust_exprop_free(h);
    return ok;
}

/* ── 征用层演示 (由 global_control_bridge_test 调用) ── */
static void gcb_exprop_demo(void) {
    printf("\n─── 征用层: 跨编码身份不灭 (子容器7) ───\n");
    exprop_identity_t lchut, fqml;
    uint64_t lc_core = 0xCCCC, fq_core = 0xDDDD;
    gcb_exprop_init(&lchut, lc_core, EXPROP_POLARITY_YIN);
    gcb_exprop_init(&fqml, fq_core, EXPROP_POLARITY_YANG);

    const char *shells[] = {"UTF-8","ASCII","JSON","YAML","Python","Rust","Nim","Zig","C"};
    int n_sh = 9;
    printf("  辐射到 %d 个编码系统...\n", n_sh);
    for (int i = 0; i < n_sh; i++) {
        gcb_exprop_expropriate(&lchut, shells[i], lc_core);
        gcb_exprop_expropriate(&fqml, shells[i], fq_core);
    }
    printf("  借用壳数: lchut=%d, fqml=%d — 身份完整? lchut=%s fqml=%s\n",
           gcb_exprop_density(&lchut), gcb_exprop_density(&fqml),
           gcb_exprop_intact(&lchut, lc_core) ? "是" : "否",
           gcb_exprop_intact(&fqml, fq_core) ? "是" : "否");

    for (int i = 0; i < n_sh; i++) {
        gcb_exprop_shed(&lchut, shells[i]);
        gcb_exprop_shed(&fqml, shells[i]);
    }
    printf("  全部剥光 — lchut核心=0x%lX  fqml核心=0x%lX\n",
           lchut.identity_core, fqml.identity_core);
    printf("  编码可换, 身份不灭! ✓\n");

    /* 产权判定 */
    printf("\n─── 产权裁决接入征用 ───\n");
    printf("  lchut vs lchut: %s\n", gcb_exprop_property_claim(lc_core, lc_core)==0?"主人HOST":"客人");
    printf("  lchut vs 0xBEEF: %s\n", gcb_exprop_property_claim(lc_core, 0xBEEF)==1?"客人GUEST":"主人");

    /* Rust FFI 直通测试 */
    printf("\n─── Rust FFI 直通 (内核→Rust真实API) ───\n");
    int rust_ok = gcb_rust_exprop_selftest();
    printf("  Rust selftest: %s\n", rust_ok ? "✓ 通道正常" : "✗ 通道异常");
    int demo_ok = gcb_rust_exprop_demo(0xABCD, 0);
    printf("  Rust FFI demo (辐射→剥壳): %s\n", demo_ok ? "✓ 身份完整 (FFI直通成功!)" : "✗");
}

/* ── 独立运行入口 ── */
#ifdef BRIDGE_TEST
int main(void) {
    global_control_bridge_test();
    return 0;
}
#endif

/* ── 完整演示 ── */
void global_control_bridge_test(void) {
    global_control_bridge_init();

    printf("═══ 全局控制容器 · 内核桥接演示 ═══\n\n");

    /* 容器地图 */
    printf("─── 容器地图 ───\n");
    gc_report(&gc, stdout);

    /* 聂小倩 */
    printf("\n─── 聂小倩: 为鬼魂找躯壳 ───\n");
    uint32_t ghost = 0x102040;   /* 声母+韵母+声调 */
    uint32_t body = 0;
    int result = gcb_nie_xiaoqian(ghost, &body);
    if (result >= 0) {
        printf("  鬼魂(0x%06X) 找到躯壳: 韵母[%d] (0x%06X)\n",
               ghost, result, body);
        printf("  借尸还魂成功!\n\n");
    } else {
        printf("  未找到匹配躯壳, 进入恢复流程\n\n");
    }

    /* 遍历演示 */
    printf("─── 跨容器遍历: 声母→韵母 ───\n");
    uint8_t src, dst;
    int next = gcb_get_next_traversal(&src, &dst);
    if (next >= 0) {
        printf("  当前遍历规则: 0x%02X → 0x%02X\n", src, dst);
        printf("  推进遍历...\n");
        gcb_advance_traversal();
        printf("  推进完成\n\n");
    }

    /* 感知器官: 距离感知聚类 */
    printf("─── 感知器官: 距离感知聚类 ───\n");
    /* 构造 32 位容器编码: [31:29]家族 [28]极性 [27:25]结构 [24:19]声母 [18:12]韵母 [11:8]声调 */
    #define MKC(f,p,s,sh,y,t) ((((uint32_t)(f)&7)<<29)|(((uint32_t)(p)&1)<<28)|(((uint32_t)(s)&7)<<25)|(((uint32_t)(sh)&0x3F)<<19)|(((uint32_t)(y)&0x7F)<<12)|(((uint32_t)(t)&0xF)<<8))
    gcb_mind_organ_init(5.0);
    /* 三簇: 夫妻阳(100,101), 边界阴(200,201), 夫妻阴(300) */
    gcb_mo_add_cell(100, MKC(2,1,3,10,20,4));  /* 夫妻·阳·声调4 */
    gcb_mo_add_cell(101, MKC(2,1,3,10,20,5));  /* 夫妻·阳·声调5 */
    gcb_mo_add_cell(200, MKC(1,0,5,30,50,8));  /* 边界·阴·声调8 */
    gcb_mo_add_cell(201, MKC(1,0,5,30,50,9));  /* 边界·阴·声调9 */
    gcb_mo_add_cell(300, MKC(2,0,3,10,20,4));  /* 夫妻·阴 (极性远) */
    int nc = gcb_mo_clusterize();
    printf("  聚类结果: %d 个簇\n", nc);
    /* 聂小倩: 鬼魂(夫妻·阳·声调6)找躯壳 */
    uint32_t ghost32 = MKC(2,1,3,10,20,6);
    double dist32 = 0;
    int body32 = gcb_mo_find_body(ghost32, &dist32);
    printf("  鬼魂(夫妻·阳·声调6) → 最近躯壳 id=%d, 距离=%.2f\n", body32, dist32);
    printf("  %s\n\n", (body32==100||body32==101) ? "✓ 正确: 匹配夫妻·阳侧" : "✗ 错误");

    /* 征用层 + Rust FFI 演示 */
    gcb_exprop_demo();

    /* 最终状态 */
    printf("\n─── 最终状态 ───\n");
    gc_report(&gc, stdout);
    printf("\n═══ 演示完成 (7子容器, Rust FFI直通) ═══\n");
}
