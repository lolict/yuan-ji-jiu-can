/**
 * MetaIME FFI Bridge · 三语言互操作接口
 *
 * 定义统一的 C ABI 接口，供 Nim/Rust/Zig 互相调用
 * 所有实现遵循零运行时内存约束（编译时常量 + 栈分配）
 */

#ifndef METAIME_FFI_H
#define METAIME_FFI_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* === 常量 === */
#define MI_MAX_SYMBOLS 64
#define MI_MAX_CONTAINERS 16
#define MI_MAX_NAME_LEN 24
#define MI_MAX_SYMBOL_LEN 8
#define MI_MAX_PATH_LEN 48

/* === 错误码 === */
typedef enum {
    MI_OK = 0,
    MI_ERR_NOT_FOUND = -1,
    MI_ERR_FULL = -2,
    MI_ERR_INVALID = -3,
    MI_ERR_OVERFLOW = -4,
} mi_err_t;

/* === 本体论路径 === */
typedef struct {
    char domain[MI_MAX_NAME_LEN];
    char identity[MI_MAX_NAME_LEN];
} mi_path_t;

/* === 符号条目 === */
typedef struct {
    char symbol[MI_MAX_SYMBOL_LEN];
    uint8_t position;
} mi_symbol_t;

/* === 容器 === */
typedef struct {
    char name[MI_MAX_NAME_LEN];
    mi_symbol_t symbols[MI_MAX_SYMBOLS];
    uint8_t symbol_count;
    uint8_t capacity;
    uint8_t polarity;
} mi_container_t;

/* === 加载器 API === */
typedef struct mi_loader mi_loader_t;

mi_loader_t* mi_loader_create(void);
void mi_loader_destroy(mi_loader_t* l);
mi_err_t mi_loader_load_default(mi_loader_t* l);
int mi_loader_container_count(const mi_loader_t* l);
mi_err_t mi_loader_get_container(const mi_loader_t* l, const char* name, mi_container_t* out);

/* === 链接器 API === */
typedef struct mi_linker mi_linker_t;

mi_linker_t* mi_linker_create(void);
void mi_linker_destroy(mi_linker_t* l);
mi_err_t mi_linker_build_index(mi_linker_t* l, const mi_loader_t* loader);
int mi_linker_lookup(const mi_linker_t* l, const char* domain, const char* symbol, mi_path_t* out);

/* === 翻译器 API === */
typedef struct mi_translator mi_translator_t;

mi_translator_t* mi_translator_create(void);
void mi_translator_destroy(mi_translator_t* t);
mi_err_t mi_translator_translate(const mi_translator_t* t, const char* foreign, char* out, size_t out_len);

/* === 解释器 API === */
typedef struct mi_interpreter mi_interpreter_t;

mi_interpreter_t* mi_interpreter_create(void);
void mi_interpreter_destroy(mi_interpreter_t* i);
mi_err_t mi_interpreter_execute(mi_interpreter_t* i, const char* command, char* out, size_t out_len);

/* === 三维进制 API === */
typedef struct {
    char shengmu[MI_MAX_NAME_LEN];
    char yunmu[MI_MAX_NAME_LEN];
    char tone[MI_MAX_NAME_LEN];
} mi_point3d_t;

mi_err_t mi_radix3d_coordinate(const char* sm, const char* ym, const char* t, mi_point3d_t* out);
uint8_t mi_radix3d_distance(const char* id1, const char* id2);

/* === 公约数 API === */
uint8_t mi_gcd(uint8_t a, uint8_t b);
uint16_t mi_lcm(uint8_t a, uint8_t b);
mi_err_t mi_divisor_relation(uint8_t a, uint8_t b, char* out, size_t out_len);

/* === 元层级 API === */
typedef enum {
    MI_YUAN_ONE = 1,
    MI_YUAN_TWO = 2,
    MI_YUAN_THREE = 3,
    MI_YUAN_FOUR = 4,
} mi_yuan_level_t;

mi_err_t mi_yuan_judge(const char* a, const char* b, char* out, size_t out_len);

#ifdef __cplusplus
}
#endif

#endif /* METAIME_FFI_H */
