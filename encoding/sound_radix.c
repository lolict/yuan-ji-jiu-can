/**
 * sound_radix.c — 声音进制
 *
 * 核心理念:
 *   不是采样 (WAV) · 而是事件编码
 *   每个声音事件 = 频率 + 音色 + 时长 + 轮廓 + 强度
 *
 * 编码本体: 8 字节/事件
 *   ┌────────┬──────┬──────┬──────┬──────┬──────┐
 *   │ 符号ID │ 频率 │ 音色 │ 时长 │ 轮廓 │ 强度 │
 *   │ 2字节  │ 2字节 │ 1字节 │ 2字节 │ 1字节 │ 1字节 │
 *   └────────┴──────┴──────┴──────┴──────┴──────┘
 *
 * 频率单位: 0.1Hz (u16 = 0-6553.5Hz, 覆盖人声 80-3000Hz)
 * 音色:     0-7 对应我们的 SYMBOL_TYPE
 * 时长单位: ms (u16 = 0-65秒)
 * 轮廓:     平/升/降/凹/凸
 * 强度:     0-255 (u8)
 *
 * 与颜色进制共享: 审判/衍射结果决定频率偏移和强度
 */

#include "sound_radix.h"
#include "global_interlock.h"
#include <string.h>
#include <math.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

/* ═══════════════════════════════════════════════════════════════
 * 频率表 (基于声学语音学, 归一化到我们的系统)
 * ═══════════════════════════════════════════════════════════════ */

/* 基韵锚中心频率 (Hz × 10) — 基于第一/第二共振峰平均 */
static const uint16_t BASE_FREQ[6] = {
    850,   /* a  — 850 Hz  (开央不圆唇) */
    450,   /* o  — 450 Hz  (后半高圆唇) */
    300,   /* e  — 300 Hz  (后半高不圆唇) */
    2800,  /* i  — 2800 Hz (前高不圆唇) */
    350,   /* u  — 350 Hz  (后高圆唇) */
    2200,  /* ü  — 2200 Hz (前高圆唇) */
};

/* 声母起始频率 (Hz × 10) — 辅音特征频率 */
static const uint16_t SHENG_FREQ[26] = {
    1800, /* b  双唇塞 */
    1800, /* p  双唇塞(送气) */
    300,  /* m  双唇鼻 */
    4000, /* f  唇齿擦 */
    2500, /* d  舌尖塞 */
    2500, /* t  舌尖塞(送气) */
    350,  /* n  舌尖鼻 */
    400,  /* l  舌尖边 */
    300,  /* g  舌根塞 */
    300,  /* k  舌根塞(送气) */
    300,  /* h  舌根擦 */
    3000, /* j  舌面塞 */
    3000, /* q  舌面塞(送气) */
    3000, /* x  舌面擦 */
    1200, /* zh 翘舌塞 */
    1200, /* ch 翘舌塞(送气) */
    1200, /* sh 翘舌擦 */
    1500, /* r  翘舌通 */
    3500, /* z  舌尖前塞 */
    3500, /* c  舌尖前塞(送气) */
    3500, /* s  舌尖前擦 */
    2800, /* y  半元音 */
    300,  /* w  半元音 */
    0,    /* (备用) */
    0,    /* (备用) */
};

/* 声调轮廓定义 (起点频率系数, 终点频率系数, 曲线类型) */
/* 系数 ×100, 曲线: 0=平 1=升 2=降 3=凹(降升) 4=凸(升降) */
typedef struct { uint8_t start, end, curve; } tone_contour_t;
static const tone_contour_t TONE_CONTOUR[4] = {
    {100, 100, 0},  /* 一声: 高平 55 */
    { 60, 100, 1},  /* 二声: 升 35 */
    { 80, 40, 3},   /* 三声: 凹 214 */
    {100, 40, 2},   /* 四声: 降 51 */
};

/* ═══════════════════════════════════════════════════════════════
 * 查找表
 * ═══════════════════════════════════════════════════════════════ */

static const char *BASE_NAMES[6]   = {"a","o","e","i","u","ü"};
static const char *SHENG_NAMES[24] = {
    "b","p","m","f","d","t","n","l",
    "g","k","h","j","q","x","zh","ch",
    "sh","r","z","c","s","y","w","y"
};

static uint16_t lookup_base_freq(const char *name) {
    for (int i = 0; i < 6; i++) {
        if (strcmp(name, BASE_NAMES[i]) == 0) return BASE_FREQ[i];
    }
    return 500;  /* 默认 500Hz */
}

static uint16_t lookup_sheng_freq(const char *name) {
    for (int i = 0; i < 24; i++) {
        if (strcmp(name, SHENG_NAMES[i]) == 0) return SHENG_FREQ[i];
    }
    return 1000;  /* 默认 1000Hz */
}

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void sound_radix_init(sound_radix_t *sr) {
    memset(sr, 0, sizeof(*sr));
    sr->magic = SOUND_RADIX_MAGIC;
    sr->sample_rate = 44100;
    sr->event_count = 0;
    sr->bpm = 60;
}

/* ═══════════════════════════════════════════════════════════════
 * 编码单个事件
 * ═══════════════════════════════════════════════════════════════ */

int sound_radix_encode(sound_radix_t *sr, const char *symbol,
                        uint16_t freq, uint8_t timbre,
                        uint16_t duration, uint8_t contour,
                        uint8_t amplitude) {
    if (!sr || sr->magic != SOUND_RADIX_MAGIC) return -1;
    if (sr->event_count >= SOUND_MAX_EVENTS) return -1;
    
    sound_event_t *ev = &sr->events[sr->event_count];
    memset(ev, 0, sizeof(*ev));
    
    /* 符号ID: 用名称的简单哈希 (唯一标识) */
    uint16_t id = 0;
    for (const char *p = symbol; *p; p++) {
        id = id * 31 + (uint8_t)*p;
    }
    ev->symbol_id = id;
    ev->frequency = freq;
    ev->timbre = timbre;
    ev->duration = duration;
    ev->contour = contour;
    ev->amplitude = amplitude;
    strncpy(ev->symbol, symbol, 15);
    ev->symbol[15] = '\0';
    
    sr->event_count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 编码音节 (声母+韵母+声调 → 事件序列)
 * ═══════════════════════════════════════════════════════════════ */

int sound_radix_encode_syllable(sound_radix_t *sr, const char *shengmu,
                                 const char *yunmu, int tone) {
    if (!sr || sr->magic != SOUND_RADIX_MAGIC) return -1;
    if (tone < 1 || tone > 4) tone = 1;
    
    uint8_t contour = (uint8_t)(tone - 1);
    
    /* 编码声母 (如果有) */
    if (shengmu && shengmu[0]) {
        uint16_t freq = lookup_sheng_freq(shengmu);
        uint8_t amp = (strncmp(shengmu, "m", 1) == 0 ||
                       strncmp(shengmu, "n", 1) == 0 ||
                       strncmp(shengmu, "l", 1) == 0 ||
                       strncmp(shengmu, "r", 1) == 0) ? 80 : 60;
        /* 鼻音/边音/通音能量高一些 */
        uint16_t dur = 80;  /* 声母默认 80ms */
        if (sound_radix_encode(sr, shengmu, freq, SHENGMU_TIMBRE, dur, 0, amp) < 0)
            return -1;
    }
    
    /* 编码韵母 (带声调轮廓) */
    uint16_t yun_freq = lookup_base_freq(yunmu);
    uint16_t yun_dur = 200;  /* 韵母默认 200ms */
    uint8_t yun_amp = 200;   /* 韵母强度 */
    
    if (sound_radix_encode(sr, yunmu, yun_freq, YUNMU_TIMBRE,
                            yun_dur, contour, yun_amp) < 0)
        return -1;
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 渲染: 将事件序列编译为 PCM 采样
 * ═══════════════════════════════════════════════════════════════ */

/* 音色滤波器: 不同 SYMBOL_TYPE 对应不同谐波结构 */
static float timbre_filter(float phase, uint8_t timbre) {
    switch (timbre) {
        case SHENGMU_TIMBRE:
            /* 声母: 噪声感, 丰富高频谐波 */
            return sinf(phase) + 0.3f * sinf(2*phase) + 0.1f * sinf(4*phase) + 0.05f * sinf(8*phase);
        case YUNMU_TIMBRE:
            /* 韵母: 纯音感, 基频+少量谐波 */
            return sinf(phase) + 0.2f * sinf(2*phase) + 0.05f * sinf(3*phase);
        case TONE_TIMBRE:
            /* 声调: 纯基频 */
            return sinf(phase);
        default:
            return sinf(phase);
    }
}

/* 包络: ADSR 简化版 */
static float envelope(float t, float dur, uint8_t contour) {
    (void)contour;  /* 包络不依赖轮廓类型 */
    float attack = 0.05f;
    float release = 0.1f;
    
    if (t < attack) return t / attack;
    if (t > dur - release) return (dur - t) / release;
    return 1.0f;
}

/* 频率轮廓 */
static float apply_contour(float t, float dur, uint8_t contour, float base_freq) {
    float ratio = t / dur;
    if (contour >= 4) contour = 0;
    const tone_contour_t *tc = &TONE_CONTOUR[contour];
    
    float start_factor = tc->start / 100.0f;
    float end_factor = tc->end / 100.0f;
    
    switch (tc->curve) {
        case 0:  /* 平 */
            return base_freq * start_factor;
        case 1:  /* 升 */
            return base_freq * (start_factor + (end_factor - start_factor) * ratio);
        case 2:  /* 降 */
            return base_freq * (start_factor + (end_factor - start_factor) * ratio);
        case 3:  /* 凹 (降升) */
            return base_freq * (start_factor + (end_factor - start_factor) * 
                   (ratio < 0.5f ? 2*ratio*ratio : 1 - 2*(1-ratio)*(1-ratio)));
        case 4:  /* 凸 (升降) */
            return base_freq * (start_factor + (end_factor - start_factor) * 
                   (1 - (1-ratio)*(1-ratio)));
        default:
            return base_freq;
    }
}

int sound_radix_render(sound_radix_t *sr, float *samples, int max_samples) {
    if (!sr || sr->magic != SOUND_RADIX_MAGIC) return -1;
    
    int offset = 0;
    for (uint16_t i = 0; i < sr->event_count; i++) {
        sound_event_t *ev = &sr->events[i];
        if (ev->duration == 0) continue;
        if (ev->amplitude == 0) continue;
        
        int n_samples = sr->sample_rate * ev->duration / 1000;
        if (offset + n_samples > max_samples) n_samples = max_samples - offset;
        if (n_samples <= 0) break;
        
        float amp = ev->amplitude / 255.0f;
        float dt = 1.0f / sr->sample_rate;
        
        for (int j = 0; j < n_samples; j++) {
            float t = j * dt;
            float dur_sec = ev->duration / 1000.0f;
            float freq = apply_contour(t, dur_sec, ev->contour, ev->frequency / 10.0f);
            float phase = 2.0f * M_PI * freq * t;
            float sample = timbre_filter(phase, ev->timbre) * amp * envelope(t, dur_sec, ev->contour);
            
            samples[offset + j] += sample;
        }
        offset += n_samples;
    }
    
    return offset;
}

/* ═══════════════════════════════════════════════════════════════
 * 获取总时长 (ms)
 * ═══════════════════════════════════════════════════════════════ */

uint32_t sound_radix_get_duration(const sound_radix_t *sr) {
    if (!sr || sr->magic != SOUND_RADIX_MAGIC) return 0;
    uint32_t total = 0;
    for (uint16_t i = 0; i < sr->event_count; i++) {
        total += sr->events[i].duration;
    }
    return total;
}

/* ═══════════════════════════════════════════════════════════════
 * 获取统计信息
 * ═══════════════════════════════════════════════════════════════ */

void sound_radix_stats(const sound_radix_t *sr, FILE *out) {
    if (!sr || sr->magic != SOUND_RADIX_MAGIC) return;
    
    fprintf(out, "─── 声音进制统计 ───\n");
    fprintf(out, "事件数: %d / %d\n", sr->event_count, SOUND_MAX_EVENTS);
    fprintf(out, "采样率: %d Hz\n", sr->sample_rate);
    fprintf(out, "总时长: %u ms\n", sound_radix_get_duration(sr));
    fprintf(out, "BPM: %d\n", sr->bpm);
    fprintf(out, "编码密度: %d 字节/事件\n", (int)sizeof(sound_event_t));
    
    if (sr->event_count > 0) {
        fprintf(out, "\n─── 事件列表 ───\n");
        for (uint16_t i = 0; i < sr->event_count; i++) {
            const sound_event_t *ev = &sr->events[i];
            const char *contour_names[] = {"平","升","降","凹","凸"};
            fprintf(out, "  [%2d] %-6s  freq=%5.1fHz  timbre=%d  dur=%3dms  contour=%s  amp=%d\n",
                    i, ev->symbol, ev->frequency / 10.0f, ev->timbre,
                    ev->duration, contour_names[ev->contour % 5], ev->amplitude);
        }
    }
}
