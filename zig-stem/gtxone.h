/**
 * gtxone.h — 融合中心 · 夫妻二人视为一个人 · 中心视角的唯一性
 */

#ifndef MQF_GTXONE_H
#define MQF_GTXONE_H

#include <stdint.h>
#include <stdbool.h>

#define MAX_INTERNAL_DIFFERENCES 64
#define MAX_EXTERNAL_LINKS 256
#define FUSION_THRESHOLD 0.9f
#define GTXONE_VALUE 0x01

typedef enum {
    FUSION_NONE      = 0x00,
    FUSION_PARTIAL   = 0x01,
    FUSION_COMPLETE  = 0x02,
    FUSION_UNIQUE    = 0x03,
} fusion_state;

typedef enum {
    VIEW_OUTWARD    = 0x00,
    VIEW_INWARD     = 0x01,
    VIEW_HORIZONTAL = 0x02,
} view_direction;

typedef struct {
    uint32_t id;
    fusion_state state;
    float    fusion_strength;
    uint32_t partner_a;
    uint32_t partner_b;
    uint32_t internal_diff_count;
    float    internal_diffs[MAX_INTERNAL_DIFFERENCES];
    uint32_t governed_count;
    uint32_t governed_ids[MAX_EXTERNAL_LINKS];
    bool     is_unique;
    uint32_t generation;
} gtxone_t;

/* API */
gtxone_t gtxone_create(uint32_t a, uint32_t b, uint32_t gen);
bool gtxone_fuse(gtxone_t* g, float strength_delta);
bool gtxone_internalize_difference(gtxone_t* g, float diff_value);
bool gtxone_govern(gtxone_t* g, uint32_t external_id);
uint32_t gtxone_power(const gtxone_t* g);
const uint32_t* gtxone_view_outward(const gtxone_t* g, uint32_t* count);
const float* gtxone_view_inward(const gtxone_t* g, uint32_t* count);
float gtxone_density(const gtxone_t* g);
bool is_unique_center(uint8_t value);

#endif /* MQF_GTXONE_H */
