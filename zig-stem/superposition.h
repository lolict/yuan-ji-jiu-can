/**
 * superposition.h — 1+1=1 叠加态 · 254次对齐升级 · 递归包裹
 *
 * 这是 hunyuan-os 关系二进制的核心运行时：
 * - 0 和 1 不是对立的，是被夫妻(加法)包裹成一个共同体的
 * - 共同体完成内部对齐后，自身成为更高层的内容
 * - 管辖者 → 被管辖者 → 更高层管辖者的递归链条
 */

#ifndef MQF_SUPERPOSITION_H
#define MQF_SUPERPOSITION_H

#include <stdint.h>
#include <stdbool.h>

#define ALIGNMENT_THRESHOLD 254
#define MAX_LAYERS 64
#define MAX_CONTENT_PER_ALIGNMENT 256

typedef enum {
    LAYER_OPEN       = 0x00,
    LAYER_ALIGNING   = 0x01,
    LAYER_CLOSED     = 0x02,
    LAYER_PROMOTED   = 0x03,
} layer_state;

typedef enum {
    REL_COUPLE  = 0x00,
    REL_BODY    = 0x01,
    REL_CLONE   = 0x02,
    REL_CONTENT = 0x03,
} rel_type;

typedef enum {
    ALIGN_OK           = 0x00,
    ALIGN_THRESHOLD    = 0x01,
    ALIGN_ERROR_FULL   = 0x02,
    ALIGN_ERROR_CLOSED = 0x03,
} align_result;

typedef struct {
    uint32_t id;
    rel_type type;
    uint32_t inner_a;
    uint32_t inner_b;
    uint32_t generation;
    bool     is_closed;
} superposition_t;

typedef struct {
    uint32_t         layer_id;
    layer_state      state;
    uint32_t         couple_id;
    uint32_t         alignment_count;
    uint32_t         content_count;
    superposition_t* contents[MAX_CONTENT_PER_ALIGNMENT];
    uint32_t         generation;
} layer_t;

typedef struct {
    bool     success;
    uint32_t new_content_id;
    uint32_t new_generation;
} promote_result;

typedef struct {
    uint32_t total_layers;
    uint32_t current_generation;
    bool     is_fully_closed;
} recursive_status;

typedef struct {
    uint8_t high;
    uint8_t low;
} base64_pair;

/* API */
superposition_t superposition_create(uint32_t a, uint32_t b, uint32_t gen);
layer_t layer_create(superposition_t* couple, uint32_t gen);
align_result layer_align(layer_t* layer, superposition_t* content);
promote_result layer_promote(layer_t* layer);
recursive_status compute_recursive_status(layer_t* layers, uint32_t count);
bool is_alignment_complete(uint32_t count);
float alignment_progress(uint32_t count);
base64_pair to_base64(uint8_t value);

#endif /* MQF_SUPERPOSITION_H */
