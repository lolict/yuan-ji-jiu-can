/**
 * heyair.h — 灰原哀 · 横向层 · 感知器官 · 念动力管辖
 */

#ifndef MQF_HEYAIR_H
#define MQF_HEYAIR_H

#include <stdint.h>
#include <stdbool.h>

#define MAX_PERCEPTION_CHANNELS 64
#define MAX_WEAVE_ROWS 256
#define MAX_WEAVE_COLS 64
#define NEN_DECAY 0.9f
#define MAX_CLUSTERS 16
#define MAX_CLUSTER_SIZE 64

typedef enum {
    WEAVE_EMPTY    = 0x00,
    WEAVE_ACTIVE   = 0x01,
    WEAVE_SETTLING = 0x02,
    WEAVE_STABLE   = 0x03,
} weave_state;

typedef enum {
    PERCEIVE_BINARY   = 0x00,
    PERCEIVE_MERGE    = 0x01,
    PERCEIVE_DISTANCE = 0x02,
    PERCEIVE_PATTERN  = 0x03,
} perceive_type;

typedef struct {
    uint32_t       id;
    perceive_type  type;
    float          sensitivity;
    uint32_t       target_node;
    float          last_reading;
    bool           is_active;
} perception_channel_t;

typedef struct {
    uint32_t id;
    weave_state state;
    uint32_t channel_count;
    perception_channel_t channels[MAX_PERCEPTION_CHANNELS];
    float weave_matrix[MAX_WEAVE_ROWS][MAX_WEAVE_COLS];
    uint32_t row_count;
    uint32_t col_count;
    float nen_power;
    float nen_focus;
} heyair_t;

typedef struct {
    uint32_t member_ids[MAX_CLUSTER_SIZE];
    uint32_t member_count;
    float    centroid;
} cluster_t;

typedef struct {
    uint32_t count;
    cluster_t clusters[MAX_CLUSTERS];
} cluster_result;

/* API */
heyair_t heyair_create(void);
perception_channel_t* heyair_add_channel(heyair_t* h, perceive_type type,
                                          float sensitivity, uint32_t target);
float heyair_perceive(heyair_t* h, uint32_t channel_id, float input_value);
float heyair_merge_binary(float a, float b, float merge_strength);
bool heyair_is_merged(float a, float b, float tolerance);
bool heyair_weave_add_row(heyair_t* h, const float* values, uint32_t count);
void heyair_weave_align_columns(heyair_t* h);
void heyair_nen_inject(heyair_t* h, float amount);
void heyair_nen_focus(heyair_t* h, float focus);
float heyair_nen_at_distance(const heyair_t* h, float distance);
cluster_result heyair_cluster(const float* values, uint32_t count, float threshold);

#endif /* MQF_HEYAIR_H */
