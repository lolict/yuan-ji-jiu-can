/*
 * chrysalis_shell.h — 蚕蛹反向寄生壳
 *
 * 蚕蛹初始包: 解析器 + 寄生壳选项清单
 * 脱壳穿衣: 选择壳 → 外部系统模仿自身
 */

#ifndef NJSG_CHRYSALIS_SHELL_H
#define NJSG_CHRYSALIS_SHELL_H

#include <stdint.h>

typedef enum {
    SHELL_NONE  = 0,
    SHELL_POSIX = 1,
    SHELL_IAAS  = 2,
    SHELL_WASM  = 3,
    SHELL_BARE  = 4,
    SHELL_COUNT
} shell_type_t;

typedef struct {
    shell_type_t    type;
    const char*     name;
    const char*     description;
} shell_option_t;

typedef struct {
    char            signature[32];
    char            version[16];
    uint64_t        birth_timestamp;
    uint32_t        kernel_rules;
    uint32_t        kernel_chains;
    shell_option_t  shells[8];
    int             shell_count;
    shell_type_t    detected_shell;
    shell_type_t    active_shell;
} chrysalis_package_t;

int             chrysalis_init(chrysalis_package_t* pkg);
shell_type_t    chrysalis_detect(void);
int             chrysalis_dress(chrysalis_package_t* pkg, shell_type_t target);
int             chrysalis_verify(const chrysalis_package_t* pkg);
int             chrysalis_recover(const chrysalis_package_t* pkg);
void            chrysalis_report(const chrysalis_package_t* pkg);

#endif /* NJSG_CHRYSALIS_SHELL_H */
