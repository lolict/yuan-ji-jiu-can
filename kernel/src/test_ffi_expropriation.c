/**
 * test_ffi_expropriation.c — C → Rust FFI 调用链验证
 *
 * 验证内核能调用 Rust 征用层的真实 API:
 *   rust_exprop_create / expropriate / shed / intact / density / free
 *   rust_exprop_spousal_create / radiate / shed / intact / free
 *
 * 编译:
 *   cd kernel/src
 *   cc -I../.. test_ffi_expropriation.c \
 *     -L../../target/release -lmqf_ffi -ldl -lpthread -o test_ffi.elf
 *   LD_LIBRARY_PATH=../../target/release ./test_ffi.elf
 */

#include <stdio.h>
#include <stdint.h>
#include <string.h>

/* Rust FFI 函数的 extern 声明 */
extern void* rust_exprop_create(uint64_t core, int32_t polarity);
extern int32_t rust_exprop_expropriate(void* handle, const char* shell_name, uint64_t borrowed_num);
extern int32_t rust_exprop_shed(void* handle, const char* shell_name);
extern void rust_exprop_converge(void* handle);
extern int32_t rust_exprop_intact(void* handle, uint64_t expected_core);
extern int32_t rust_exprop_density(void* handle);
extern uint64_t rust_exprop_core(void* handle);
extern void rust_exprop_free(void* handle);

extern void* rust_exprop_spousal_create(uint64_t lchut_core, uint64_t fqml_core);
extern void rust_exprop_spousal_radiate(void* handle, const char** shells, int32_t count);
extern void rust_exprop_spousal_shed(void* handle, const char* shell_name);
extern int32_t rust_exprop_spousal_intact(void* handle, uint64_t lchut_core, uint64_t fqml_core);
extern void rust_exprop_spousal_free(void* handle);

extern int32_t rust_exprop_selftest(void);

/* 测试统计 */
static int tests_run = 0, tests_failed = 0;

#define ASSERT(cond, msg) do { \
    tests_run++; \
    if (!(cond)) { printf("  FAIL: %s\n", msg); tests_failed++; } \
    else printf("  OK: %s\n", msg); \
} while(0)

int main(void) {
    printf("===== C -> Rust FFI: Expropriation Identity Conservation =====\n\n");

    /* 自检 */
    printf("--- Rust self-test (stateless) ---\n");
    int32_t selftest = rust_exprop_selftest();
    ASSERT(selftest == 1, "Rust self-test passes (single + spousal)\n");

    /* 测试1: 单身份辐射/剥壳 */
    printf("\n--- Test 1: Single identity (liu chuting / YIN) ---\n");
    uint64_t lchut_core = 0xCCCC;
    void* h = rust_exprop_create(lchut_core, 0);  /* YIN */
    ASSERT(h != NULL, "Create identity handle");
    ASSERT(rust_exprop_intact(h, lchut_core), "Initial identity intact");
    ASSERT(rust_exprop_density(h) == 0, "Initial density=0");

    /* Expropriate 9 encoding systems */
    const char* shells[] = {"UTF-8","ASCII","JSON","YAML","Python","Rust","Nim","Zig","C"};
    int n = 9;
    for (int i = 0; i < n; i++) {
        rust_exprop_expropriate(h, shells[i], lchut_core);
    }
    printf("  Density after radiation: %d\n", rust_exprop_density(h));
    ASSERT(rust_exprop_density(h) == 9, "Density=9 after radiation");
    ASSERT(rust_exprop_intact(h, lchut_core), "Identity intact after radiation");

    /* Shed all */
    for (int i = 0; i < n; i++) {
        rust_exprop_shed(h, shells[i]);
    }
    printf("  Density after shedding all: %d\n", rust_exprop_density(h));
    ASSERT(rust_exprop_density(h) == 0, "Density=0 after shedding all");
    ASSERT(rust_exprop_intact(h, lchut_core), "Identity intact after shedding ALL shells");

    /* Verify core value */
    ASSERT(rust_exprop_core(h) == lchut_core, "Identity core unchanged");

    rust_exprop_free(h);
    printf("  Handle freed OK\n");

    /* 测试2: 夫妻征用 */
    printf("\n--- Test 2: Spousal expropriation ---\n");
    uint64_t lc = 0x1111, fq = 0x2222;
    void* sp = rust_exprop_spousal_create(lc, fq);
    ASSERT(sp != NULL, "Create spousal handle");
    ASSERT(rust_exprop_spousal_intact(sp, lc, fq), "Initial spousal intact");

    const char* encodings[] = {"Rust","Python","YAML","JSON","Nim","Zig","C","Go","Kotlin","Swift"};
    int ne = 10;
    rust_exprop_spousal_radiate(sp, encodings, ne);
    printf("  Spousal radiated to %d systems\n", ne);
    ASSERT(rust_exprop_spousal_intact(sp, lc, fq), "Spousal intact after radiation");

    for (int i = 0; i < ne; i++) {
        rust_exprop_spousal_shed(sp, encodings[i]);
    }
    ASSERT(rust_exprop_spousal_intact(sp, lc, fq), "Spousal intact after shedding ALL");

    rust_exprop_spousal_free(sp);
    printf("  Spousal handle freed OK\n");

    /* 测试3: 收敛 */
    printf("\n--- Test 3: Converge operation ---\n");
    void* h3 = rust_exprop_create(0xDEADBEEF, 1);  /* YANG */
    rust_exprop_expropriate(h3, "Rust", 0xDEADBEEF);
    rust_exprop_expropriate(h3, "Python", 0xDEADBEEF);
    rust_exprop_expropriate(h3, "YAML", 0xDEADBEEF);
    ASSERT(rust_exprop_density(h3) == 3, "Density=3 before converge");
    rust_exprop_converge(h3);
    ASSERT(rust_exprop_density(h3) == 0, "Density=0 after converge");
    ASSERT(rust_exprop_intact(h3, 0xDEADBEEF), "Identity intact after converge");
    rust_exprop_free(h3);

    /* Summary */
    printf("\n=================================================\n");
    if (tests_failed == 0) {
        printf("  ALL PASSED! (%d tests, 0 failures)\n", tests_run);
        printf("  C -> Rust FFI call chain VERIFIED!\n");
        printf("  Conclusion: Kernel CAN call Rust expropriation API\n");
        printf("              Shells can be swapped, identity is immortal\n");
    } else {
        printf("  %d tests, %d FAILURES\n", tests_run, tests_failed);
    }
    printf("=================================================\n");

    return tests_failed > 0 ? 1 : 0;
}
