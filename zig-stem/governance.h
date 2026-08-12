/**
 * governance.h — 三层管辖结构 · 夫妻(加法) → 中心(gtxone) → 多数(kndoyle)
 */

#ifndef MQF_GOVERNANCE_H
#define MQF_GOVERNANCE_H

#include "superposition.h"
#include "gtxone.h"
#include "heyair.h"

typedef enum {
    LAYER_COUPLE   = 0x00,
    LAYER_CENTER   = 0x01,
    LAYER_GOVERNED = 0x02,
} governance_layer;

typedef struct {
    superposition_t couple;
    layer_t         couple_layer;
    gtxone_t        center;
    uint32_t        governed_count;
    uint32_t        governed_ids[MAX_EXTERNAL_LINKS];
    heyair_t        heyair;
    bool            is_activated;
    uint32_t        activation_gen;
} governance_system_t;

/* API */
governance_system_t governance_create(uint32_t a, uint32_t b);
bool governance_activate(governance_system_t* sys);
bool governance_add_governed(governance_system_t* sys, uint32_t node_id);
float governance_merge_pair(governance_system_t* sys, float a, float b);
void governance_report(const governance_system_t* sys);

#endif /* MQF_GOVERNANCE_H */
