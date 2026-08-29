/**
 * MetaIME FFI 测试程序 · 验证三语言互操作
 *
 * 通过 dlopen 分别加载 Nim 和 Zig 的 .so，避免符号冲突
 * 编译: gcc test_ffi.c -ldl -o test_ffi
 * 运行: LD_LIBRARY_PATH=../zig:. ./test_ffi
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
typedef mi_err_t (*fn_radix3d_coordinate)(const char*, const char*, const char*, mi_point3d_t*);
typedef uint8_t (*fn_radix3d_distance)(const char*, const char*);
typedef uint8_t (*fn_gcd)(uint8_t, uint8_t);
typedef uint16_t (*fn_lcm)(uint8_t, uint8_t);

typedef struct {
    void* handle;
    fn_loader_create loader_create;
    fn_loader_destroy loader_destroy;
    fn_loader_load_default loader_load_default;
    fn_loader_container_count loader_container_count;
    fn_translator_create translator_create;
    fn_translator_destroy translator_destroy;
    fn_translator_translate translator_translate;
    fn_radix3d_coordinate radix3d_coordinate;
    fn_radix3d_distance radix3d_distance;
    fn_gcd gcd;
    fn_lcm lcm;
} lib_ops_t;

int load_lib(const char* path, lib_ops_t* ops) {
    ops->handle = dlopen(path, RTLD_NOW);
    if (!ops->handle) {
        fprintf(stderr, "dlopen(%s) failed: %s\n", path, dlerror());
        return -1;
    }
    ops->loader_create = (fn_loader_create)dlsym(ops->handle, "mi_loader_create");
    ops->loader_destroy = (fn_loader_destroy)dlsym(ops->handle, "mi_loader_destroy");
    ops->loader_load_default = (fn_loader_load_default)dlsym(ops->handle, "mi_loader_load_default");
    ops->loader_container_count = (fn_loader_container_count)dlsym(ops->handle, "mi_loader_container_count");
    ops->translator_create = (fn_translator_create)dlsym(ops->handle, "mi_translator_create");
    ops->translator_destroy = (fn_translator_destroy)dlsym(ops->handle, "mi_translator_destroy");
    ops->translator_translate = (fn_translator_translate)dlsym(ops->handle, "mi_translator_translate");
    ops->radix3d_coordinate = (fn_radix3d_coordinate)dlsym(ops->handle, "mi_radix3d_coordinate");
    ops->radix3d_distance = (fn_radix3d_distance)dlsym(ops->handle, "mi_radix3d_distance");
    ops->gcd = (fn_gcd)dlsym(ops->handle, "mi_gcd");
    ops->lcm = (fn_lcm)dlsym(ops->handle, "mi_lcm");
    return 0;
}

void test_zig_ops(const lib_ops_t* zig) {
    printf("\n=== Zig 实现测试 ===\n");

    /* 翻译器 */
    char buf[64];
    mi_err_t err;

    if (zig->translator_translate) {
        err = zig->translator_translate(NULL, "def", buf, sizeof(buf));
        TEST("Zig: translate('def')");
        if (err == MI_OK) {
            printf("result='%s' ", buf);
            if (strcmp(buf, "袋.定义.操作") == 0) PASS();
            else FAIL("wrong translation");
        } else {
            FAIL("error code");
        }

        err = zig->translator_translate(NULL, "class", buf, sizeof(buf));
        TEST("Zig: translate('class')");
        if (err == MI_OK) {
            printf("result='%s' ", buf);
            if (strcmp(buf, "袋.定义.容器") == 0) PASS();
            else FAIL("wrong translation");
        } else {
            FAIL("error code");
        }
    }

    /* 三维进制 - 使用有效的身份名（首、次、三...） */
    if (zig->radix3d_coordinate) {
        mi_point3d_t point;
        err = zig->radix3d_coordinate("首", "次", "三", &point);
        TEST("Zig: coordinate(首,次,三)");
        if (err == MI_OK) {
            printf("sm='%s' ym='%s' t='%s' ", point.shengmu, point.yunmu, point.tone);
            PASS();
        } else {
            FAIL("error code");
        }
    }

    if (zig->radix3d_distance) {
        uint8_t dist = zig->radix3d_distance("首", "五");
        TEST("Zig: distance(首,五)");
        printf("dist=%u ", dist);
        if (dist > 0 && dist <= 12) PASS();
        else FAIL("invalid distance");
    }

    /* GCD/LCM */
    if (zig->gcd) {
        uint8_t gcd = zig->gcd(12, 8);
        TEST("Zig: gcd(12,8)");
        printf("gcd=%u ", gcd);
        if (gcd == 4) PASS(); else FAIL("wrong gcd");
    }

    if (zig->lcm) {
        uint16_t lcm = zig->lcm(12, 8);
        TEST("Zig: lcm(12,8)");
        printf("lcm=%u ", lcm);
        if (lcm == 24) PASS(); else FAIL("wrong lcm");
    }
}

void test_nim_ops(const lib_ops_t* nim) {
    printf("\n=== Nim 实现测试 ===\n");

    /* 加载器 */
    if (nim->loader_create) {
        mi_loader_t* l = nim->loader_create();
        TEST("Nim: loader_create");
        if (l) PASS(); else FAIL("null pointer");

        if (l) {
            mi_err_t err = nim->loader_load_default(l);
            TEST("Nim: loader_load_default");
            if (err == MI_OK) PASS(); else FAIL("error code");

            int count = nim->loader_container_count(l);
            TEST("Nim: loader_container_count");
            printf("count=%d ", count);
            if (count > 0) PASS(); else FAIL("zero containers");

            nim->loader_destroy(l);
            TEST("Nim: loader_destroy");
            PASS();
        }
    }

    /* 翻译器 */
    if (nim->translator_create) {
        mi_translator_t* t = nim->translator_create();
        TEST("Nim: translator_create");
        if (t) PASS(); else FAIL("null pointer");

        if (t) {
            char buf[64];
            mi_err_t err = nim->translator_translate(t, "def", buf, sizeof(buf));
            TEST("Nim: translate('def')");
            if (err == MI_OK) {
                printf("result='%s' ", buf);
                PASS();
            } else {
                FAIL("error code");
            }

            nim->translator_destroy(t);
            TEST("Nim: translator_destroy");
            PASS();
        }
    }
}

void test_rust_ops(const lib_ops_t* rust) {
    printf("\n=== Rust 实现测试 ===\n");

    /* 加载器 */
    if (rust->loader_create) {
        mi_loader_t* l = rust->loader_create();
        TEST("Rust: loader_create");
        if (l) PASS(); else FAIL("null pointer");

        if (l) {
            mi_err_t err = rust->loader_load_default(l);
            TEST("Rust: loader_load_default");
            if (err == MI_OK) PASS(); else FAIL("error code");

            int count = rust->loader_container_count(l);
            TEST("Rust: loader_container_count");
            printf("count=%d ", count);
            if (count >= 0) PASS(); else FAIL("invalid count");

            rust->loader_destroy(l);
            TEST("Rust: loader_destroy");
            PASS();
        }
    }

    /* 翻译器 */
    if (rust->translator_create) {
        mi_translator_t* t = rust->translator_create();
        TEST("Rust: translator_create");
        if (t) PASS(); else FAIL("null pointer");

        if (t) {
            char buf[64];
            mi_err_t err = rust->translator_translate(t, "def", buf, sizeof(buf));
            TEST("Rust: translate('def')");
            if (err == MI_OK) {
                printf("result='%s' ", buf);
                PASS();
            } else {
                FAIL("error code");
            }

            rust->translator_destroy(t);
            TEST("Rust: translator_destroy");
            PASS();
        }
    }

    /* GCD/LCM */
    if (rust->gcd) {
        uint8_t gcd = rust->gcd(12, 8);
        TEST("Rust: gcd(12,8)");
        printf("gcd=%u ", gcd);
        if (gcd == 4) PASS(); else FAIL("wrong gcd");
    }

    if (rust->lcm) {
        uint16_t lcm = rust->lcm(12, 8);
        TEST("Rust: lcm(12,8)");
        printf("lcm=%u ", lcm);
        if (lcm == 24) PASS(); else FAIL("wrong lcm");
    }
}

int main(void) {
    printf("========================================\n");
    printf(" MetaIME FFI 三语言互操作测试\n");
    printf("========================================\n");

    lib_ops_t zig = {0};
    lib_ops_t nim = {0};
    lib_ops_t rust = {0};

    /* 加载 Zig 库 */
    if (load_lib("../zig/libffi_bridge.so", &zig) != 0) {
        fprintf(stderr, "无法加载 Zig 库\n");
    } else {
        test_zig_ops(&zig);
    }

    /* 加载 Nim 库 */
    if (load_lib("./libmetaime_nim_bridge.so", &nim) != 0) {
        fprintf(stderr, "无法加载 Nim 库\n");
    } else {
        test_nim_ops(&nim);
    }

    /* 加载 Rust 库 */
    if (load_lib("./rust_bridge/target/release/libmetaime_rust_bridge.so", &rust) != 0) {
        fprintf(stderr, "无法加载 Rust 库\n");
    } else {
        test_rust_ops(&rust);
    }

    /* 清理 */
    if (zig.handle) dlclose(zig.handle);
    if (nim.handle) dlclose(nim.handle);
    if (rust.handle) dlclose(rust.handle);

    printf("\n========================================\n");
    printf(" 结果: %d 通过, %d 失败\n", pass_count, fail_count);
    printf("========================================\n");

    return fail_count > 0 ? 1 : 0;
}
