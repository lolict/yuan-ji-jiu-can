/**
 * sound_radix.h — 声音进制
 *
 * 不是采样 (WAV) · 而是事件编码
 * 每个事件 = 频率 + 音色 + 时长 + 轮廓 + 强度
 *
 * 编码本体: 8 字节/事件
 * 与颜色进制共享审判/衍射体系
 */

#ifndef NJSG_SOUND_RADIX_H
#define NJSG_SOUND_RADIX_H

#include <stdint.h>
#include <stdio.h>

#define SOUND_RADIX_MAGIC   0x534E4452  /* "SNDR" */
#define SOUND_MAX_EVENTS    254         /* 与集装箱字框上限一致 */

/* 音色类型 (与颜色系统 SYMBOL_TYPE 对应) */
#define SHENGMU_TIMBRE  0x06    /* 声母音色: 噪声感 · 丰富谐波 */
#define YUNMU_TIMBRE    0x01    /* 韵母音色: 纯音感 · 基频为主 */
#define TONE_TIMBRE     0x02    /* 声调音色: 纯基频 */

/* 轮廓类型 */
#define CONTOUR_FLAT    0   /* 一声: 高平 */
#define CONTOUR_RISE    1   /* 二声: 升 */
#define CONTOUR_FALL    2   /* 四声: 降 */
#define CONTOUR_DIP     3   /* 三声: 凹 (降升) */
#define CONTOUR_PEAK    4   /* 凸 (升降) */

typedef struct {
    uint16_t    symbol_id;      /* 符号标识 */
    uint16_t    frequency;      /* 频率 (× 0.1Hz, 范围 0-6553.5Hz) */
    uint8_t     timbre;         /* 音色类型 (SYMBOL_TYPE) */
    uint8_t     contour;        /* 音高轮廓 (平/升/降/凹/凸) */
    uint16_t    duration;       /* 时长 (毫秒) */
    uint8_t     amplitude;      /* 强度 (0-255) */
    char        symbol[16];     /* 符号名 (调试用) */
} sound_event_t;

typedef struct {
    uint32_t    magic;
    uint16_t    sample_rate;    /* 采样率 (默认 44100) */
    uint16_t    bpm;            /* 节拍速度 */
    uint16_t    event_count;    /* 当前事件数 */
    uint16_t    reserved;
    sound_event_t events[SOUND_MAX_EVENTS];
} sound_radix_t;

/* ── API ───────────────────────────────────────────────────── */

void    sound_radix_init(sound_radix_t *sr);
int     sound_radix_encode(sound_radix_t *sr, const char *symbol,
                            uint16_t freq, uint8_t timbre,
                            uint16_t duration, uint8_t contour,
                            uint8_t amplitude);
int     sound_radix_encode_syllable(sound_radix_t *sr, const char *shengmu,
                                     const char *yunmu, int tone);
int     sound_radix_render(sound_radix_t *sr, float *samples, int max_samples);
uint32_t sound_radix_get_duration(const sound_radix_t *sr);
void    sound_radix_stats(const sound_radix_t *sr, FILE *out);

#endif /* NJSG_SOUND_RADIX_H */
