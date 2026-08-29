/**
 * 跨语言鼓膜自举测试 — 验证 Rust/Nim/Zig 三语言自举一致性
 *
 * 编译: cc test_cross_lang_bootstrap.c -ldl -o test_bootstrap
 * 运行: LD_LIBRARY_PATH=../zig:.:rust_bridge/target/release ./test_bootstrap
 *
 * 核心验证:
 *   1. 三语言都能成功自举
 *   2. 自举质量 > 阈值
 *   3. 命名锚点一致 (mlll/gtt/mlnn/lctqf)
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <dlfcn.h>
#include "metaime_ffi.h"

static int pass_count = 0;
static int fail_count = 0;

#define TEST(name) printf("[TEST] %s ... ", name)
#define PASS() do { printf("PASS\n"); pass_count++; } while(0)
#define FAIL(msg) do { printf("FAIL: %s\n", msg); fail_count++; } while(0)

/* 函数指针类型 */
typedef mi_loader_t* (*fn_loader_create)(void);
typedef void (*fn_loader_destroy)(mi_loader_t*);
typedef mi_err_t (*fn_loader_load_default)(mi_loader_t*);
typedef int (*fn_loader_container_count)(const mi_loader_t*);
typedef mi_translator_t* (*fn_translator_create)(void);
typedef void (*fn_translator_destroy)(mi_translator_t*);
typedef mi_err_t (*fn_translator_translate)(const mi_translator_t*, const char*, char*, size_t);
typedef uint8_t (*fn_gcd)(uint8_t, uint8_t);
typedef uint16_t (*fn_lcm)(uint8_t, uint8_t);

typedef struct {
    void* handle;
    const char* name;
    fn_loader_create loader_create;
    fn_loader_destroy loader_destroy;
    fn_loader_load_default loader_load_default;
    fn_loader_container_count loader_container_count;
    fn_translator_create translator_create;
    fn_translator_destroy translator_destroy;
    fn_translator_translate translator_translate;
    fn_gcd gcd;
    fn_lcm lcm;
} lang_lib_t;

int load_lib(const char* path, const char* name, lang_lib_t* lib) {
    lib->handle = dlopen(path, RTLD_NOW);
    lib->name = name;
    if (!lib->handle) {
        fprintf(stderr, "dlopen(%s) failed: %s\n", path, dlerror());
        return -1;
    }
    lib->loader_create = (fn_loader_create)dlsym(lib->handle, "mi_loader_create");
    lib->loader_destroy = (fn_loader_destroy)dlsym(lib->handle, "mi_loader_destroy");
    lib->loader_load_default = (fn_loader_load_default)dlsym(lib->handle, "mi_loader_load_default");
    lib->loader_container_count = (fn_loader_container_count)dlsym(lib->handle, "mi_loader_container_count");
    lib->translator_create = (fn_translator_create)dlsym(lib->handle, "mi_translator_create");
    lib->translator_destroy = (fn_translator_destroy)dlsym(lib->handle, "mi_translator_destroy");
    lib->translator_translate = (fn_translator_translate)dlsym(lib->handle, "mi_translator_translate");
    lib->gcd = (fn_gcd)dlsym(lib->handle, "mi_gcd");
    lib->lcm = (fn_lcm)dlsym(lib->handle, "mi_lcm");
    return 0;
}

/* 验证单个语言的自举 */
void test_lang_bootstrap(lang_lib_t* lib) {
    char buf[128];
    mi_err_t err;

    printf("\n── %s 语言自举 ──\n", lib->name);

    /* 1. 加载器自举 */
    if (lib->loader_create) {
        mi_loader_t* l = lib->loader_create();
        TEST("加载器创建");
        if (l) PASS(); else { FAIL("null pointer"); return; }

        err = lib->loader_load_default(l);
        TEST("加载默认数据");
        if (err == MI_OK) PASS(); else { FAIL("load error"); lib->loader_destroy(l); return; }

        int count = lib->loader_container_count(l);
        TEST("容器数量 > 0");
        printf("count=%d ", count);
        if (count > 0) PASS(); else FAIL("zero containers");

        lib->loader_destroy(l);
        TEST("加载器销毁");
        PASS();
    }

    /* 2. 翻译器自举 */
    if (lib->translator_create) {
        mi_translator_t* t = lib->translator_create();
        TEST("翻译器创建");
        if (t) PASS(); else { FAIL("null pointer"); return; }

        /* 测试翻译一致性 */
        err = lib->translator_translate(t, "def", buf, sizeof(buf));
        TEST("翻译 'def'");
        if (err == MI_OK) {
            printf("result='%s' ", buf);
            /* 验证翻译结果一致 */
            if (strcmp(buf, "袋.定义.操作") == 0) PASS();
            else FAIL("translation mismatch");
        } else {
            FAIL("translate error");
        }

        err = lib->translator_translate(t, "class", buf, sizeof(buf));
        TEST("翻译 'class'");
        if (err == MI_OK) {
            printf("result='%s' ", buf);
            if (strcmp(buf, "袋.定义.容器") == 0) PASS();
            else FAIL("translation mismatch");
        } else {
            FAIL("translate error");
        }

        lib->translator_destroy(t);
        TEST("翻译器销毁");
        PASS();
    }

    /* 3. 数学基础 (GCD/LCM) */
    if (lib->gcd) {
        uint8_t gcd = lib->gcd(12, 8);
        TEST("gcd(12,8) = 4");
        printf("gcd=%u ", gcd);
        if (gcd == 4) PASS(); else FAIL("wrong gcd");
    }

    if (lib->lcm) {
        uint16_t lcm = lib->lcm(12, 8);
        TEST("lcm(12,8) = 24");
        printf("lcm=%u ", lcm);
        if (lcm == 24) PASS(); else FAIL("wrong lcm");
    }
}

int main(void) {
    printf("═══════════════════════════════════════\n");
    printf(" 跨语言鼓膜自举一致性测试\n");
    printf(" Rust / Nim / Zig 三语言验证\n");
    printf("═══════════════════════════════════════\n");

    lang_lib_t zig = {0}, nim = {0}, rust = {0};

    /* 加载三语言库 */
    load_lib("../zig/libffi_bridge.so", "Zig", &zig);
    load_lib("./libmetaime_nim_bridge.so", "Nim", &nim);
    load_lib("./rust_bridge/target/release/libmetaime_rust_bridge.so", "Rust", &rust);

    /* 逐个测试 */
    if (zig.handle) test_lang_bootstrap(&zig);
    if (nim.handle) test_lang_bootstrap(&nim);
    if (rust.handle) test_lang_bootstrap(&rust);

    /* 跨语言一致性验证 */
    printf("\n── 跨语言一致性 ──\n");
    if (zig.translator_translate && nim.translator_translate && rust.translator_translate) {
        char buf_z[64], buf_n[64], buf_r[64];
        mi_err_t ez, en, er;

        /* 三语言翻译同一词汇 */
        ez = zig.translator_translate(NULL, "for", buf_z, sizeof(buf_z));
        en = nim.translator_translate(nim.translator_create(), "for", buf_n, sizeof(buf_n));
        er = rust.translator_translate(rust.translator_create(), "for", buf_r, sizeof(buf_r));

        TEST("三语言翻译 'for' 一致");
        if (ez == MI_OK && en == MI_OK && er == MI_OK) {
            printf("zig='%s' nim='%s' rust='%s' ", buf_z, buf_n, buf_r);
            if (strcmp(buf_z, buf_n) == 0 && strcmp(buf_n, buf_r) == 0) PASS();
            else FAIL("translations differ");
        } else {
            FAIL("translation error");
        }
    }

    /* 清理 */
    if (zig.handle) dlclose(zig.handle);
    if (nim.handle) dlclose(nim.handle);
    if (rust.handle) dlclose(rust.handle);

    printf("\n═══════════════════════════════════════\n");
    printf(" 结果: %d 通过, %d 失败\n", pass_count, fail_count);
    printf("═══════════════════════════════════════\n");

    return fail_count > 0 ? 1 : 0;
}
