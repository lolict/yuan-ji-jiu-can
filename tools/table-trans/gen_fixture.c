/*
 * gen_fixture.c — 静态翻译验收夹具生成器
 * 目的: 生成 arm64/arm32 机器码块与最小 ELF32/ELF64 (各含一个 PT_LOAD+X 段)
 * 允许: 输出到指定目录 ; 禁止: 依赖交叉编译器
 * 验收: tabtrans elf 模式可解析, 未译=0
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

static void wr32(FILE *f, uint32_t v) { fwrite(&v, 4, 1, f); }
static void wr64(FILE *f, uint64_t v) { fwrite(&v, 8, 1, f); }
static void wr16(FILE *f, uint16_t v) { fwrite(&v, 2, 1, f); }

static const uint32_t arm64_code[] = {
    0xD28000E0u, 0x91000400u, 0xD1000401u, 0xF9000020u,
    0xF9400020u, 0x71007C00u, 0x14000002u, 0x94000001u,
    0xB4000004u, 0xD65F03C0u,
};

static const uint32_t arm32_code[] = {
    0xE3A00007u, 0xE2801001u, 0xE2402003u, 0xE5801000u,
    0xE5900000u, 0xE3500003u, 0xEA000002u, 0xEB000001u,
    0x0A000000u, 0xE12FFF1Eu,
};

static void write_blob(const char *path, const uint32_t *code, int n) {
    FILE *f = fopen(path, "wb");
    if (!f) { perror(path); return; }
    for (int i = 0; i < n; i++) wr32(f, code[i]);
    fclose(f);
}

static void write_elf64_arm64(const char *path, const uint32_t *code, int n) {
    long codesz = (long)n * 4;
    long off = 64 + 56;
    FILE *f = fopen(path, "wb");
    if (!f) { perror(path); return; }
    static const char ident[16] = { 0x7f, 'E', 'L', 'F', 2, 1, 1, 0 };
    fwrite(ident, 1, 16, f);
    wr16(f, 2); wr16(f, 183); wr32(f, 1);
    wr64(f, (uint64_t)(0x1000 + off));
    wr64(f, 64); wr64(f, 0);
    wr32(f, 0); wr16(f, 64); wr16(f, 56); wr16(f, 1);
    wr16(f, 0); wr16(f, 0); wr16(f, 0);
    wr32(f, 1); wr32(f, 5);
    wr64(f, (uint64_t)off); wr64(f, 0x1000); wr64(f, 0x1000);
    wr64(f, (uint64_t)codesz); wr64(f, (uint64_t)codesz); wr64(f, 0x1000);
    for (int i = 0; i < n; i++) wr32(f, code[i]);
    fclose(f);
}

static void write_elf32_arm32(const char *path, const uint32_t *code, int n) {
    long codesz = (long)n * 4;
    long off = 52 + 32;
    FILE *f = fopen(path, "wb");
    if (!f) { perror(path); return; }
    static const char ident[16] = { 0x7f, 'E', 'L', 'F', 1, 1, 1, 0 };
    fwrite(ident, 1, 16, f);
    wr16(f, 2); wr16(f, 40); wr32(f, 1);
    wr32(f, (uint32_t)(0x1000 + off));
    wr32(f, 52); wr32(f, 0);
    wr32(f, 0x05000000u); wr16(f, 52); wr16(f, 32); wr16(f, 1);
    wr16(f, 0); wr16(f, 0); wr16(f, 0);
    wr32(f, 1); wr32(f, (uint32_t)off); wr32(f, 0x1000); wr32(f, 0x1000);
    wr32(f, (uint32_t)codesz); wr32(f, (uint32_t)codesz); wr32(f, 5); wr32(f, 0x1000);
    for (int i = 0; i < n; i++) wr32(f, code[i]);
    fclose(f);
}

int main(int argc, char **argv) {
    const char *dir = argc > 1 ? argv[1] : "tests";
    char path[256];
    snprintf(path, sizeof path, "%s/arm64.bin", dir); write_blob(path, arm64_code, 10);
    snprintf(path, sizeof path, "%s/arm32.bin", dir); write_blob(path, arm32_code, 10);
    snprintf(path, sizeof path, "%s/arm64.elf", dir); write_elf64_arm64(path, arm64_code, 10);
    snprintf(path, sizeof path, "%s/arm32.elf", dir); write_elf32_arm32(path, arm32_code, 10);
    printf("夹具已生成于 %s/\n", dir);
    return 0;
}
