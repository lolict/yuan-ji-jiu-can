/**
 * identity_registry.c — 身份注册表实现
 */

#include "identity_registry.h"
#include <string.h>

static identity_t g_registry[IDENTITY_MAX];
static uint32_t    g_count = 0;

void id_registry_init(void) {
    memset(g_registry, 0, sizeof(g_registry));
    g_count = 0;
}

int id_register(uint32_t id, const char *name, const char *relationship,
                id_gender_t gender, id_belong_t belong) {
    if (g_count >= IDENTITY_MAX) return -1;
    g_registry[g_count].id = id;
    g_registry[g_count].name = name;
    g_registry[g_count].relationship = relationship;
    g_registry[g_count].gender = gender;
    g_registry[g_count].belong = belong;
    g_registry[g_count].era = ID_PRESENT;
    g_count++;
    return 0;
}

const identity_t *id_find(uint32_t id) {
    for (uint32_t i = 0; i < g_count; i++) {
        if (g_registry[i].id == id) return &g_registry[i];
    }
    return 0;
}

const identity_t *id_find_by_name(const char *name) {
    for (uint32_t i = 0; i < g_count; i++) {
        if (g_registry[i].name && strcmp(g_registry[i].name, name) == 0)
            return &g_registry[i];
    }
    return 0;
}

id_belong_t id_check_belong(uint32_t id) {
    const identity_t *r = id_find(id);
    return r ? r->belong : ID_NOT_US;
}
