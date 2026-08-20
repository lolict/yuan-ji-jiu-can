/**
 * seed_signal.h — 时序信号种子
 *
 * 来源: VHDL / Verilog / C signal.h / Rust tokio::sync
 * 提取日期: 2026-08-19
 *
 * 数学种子: "事件驱动 · 时钟周期 · 上升/下降沿"
 *   · 事件 = 容器填装/排空
 *   · 时钟 = 流水线节拍 (一元的 · 一个接一个)
 *   · 边沿 = 筷子夹持的瞬间
 *
 * 产权壳子 (弃):
 *   · Verilog Icarus / Yosys 综合器
 *   · VHDL IEEE 1076 标准库
 *   · POSIX signal / sigaction
 *   · Rust tokio runtime / epoll / IOCP
 *
 * 我们的新编排:
 *   信号 = 流水线的入队/出队
 *   时钟 = 时序递增的节拍器
 *   协议 = 可替换的 "定制云" (codec_declaration)
 */

#ifndef NJSG_SEED_SIGNAL_H
#define NJSG_SEED_SIGNAL_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 事件信号
 *
 * 每个信号 = 一个待处理的事件
 * 事件在流水线上排队 · 按节拍处理
 * ═══════════════════════════════════════════════════════════════ */

#define SIGNAL_MAX_PENDING  1024
#define SIGNAL_MAGIC        0x516N4L

typedef enum {
    SIG_DATA_AVAILABLE = 0x01,   /* 数据已到达容器 */
    SIG_CONTAINER_FULL  = 0x02,   /* 容器已满 · 待排水 */
    SIG_TIMER_TICK      = 0x10,   /* 时钟节拍 */
    SIG_TRANSLATE       = 0x20,   /* 转译请求 */
} signal_kind_t;

typedef struct {
    uint32_t       magic;        /* SIGNAL_MAGIC */
    signal_kind_t  kind;         /* 事件类型 */
    uint32_t       source_id;    /* 源容器 */
    uint32_t       timestamp;    /* 时序戳 (哪个节拍) */
    uint32_t       payload;      /* 附加数据 */
} signal_event_t;

#endif /* NJSG_SEED_SIGNAL_H */
