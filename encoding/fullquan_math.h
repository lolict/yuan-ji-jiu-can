/**
 * fullquan_math.h — 满全法数学常量
 *
 * 用查表和命名常量替代位移运算符
 * 2^n 直接给值 · 一目了然
 */

#ifndef NJSG_FULLQUAN_MATH_H
#define NJSG_FULLQUAN_MATH_H

#include <stdint.h>

/* 2的幂次 · 不用位移 · 直接定义 */
#define FQ_POW2_0   1
#define FQ_POW2_1   2
#define FQ_POW2_2   4
#define FQ_POW2_3   8
#define FQ_POW2_4   16
#define FQ_POW2_5   32
#define FQ_POW2_6   64
#define FQ_POW2_7   128
#define FQ_POW2_8   256
#define FQ_POW2_9   512
#define FQ_POW2_10  1024
#define FQ_POW2_11  2048
#define FQ_POW2_12  4096
#define FQ_POW2_13  8192
#define FQ_POW2_14  16384
#define FQ_POW2_15  32768
#define FQ_POW2_16  65536
#define FQ_POW2_17  131072
#define FQ_POW2_18  262144
#define FQ_POW2_19  524288
#define FQ_POW2_20  1048576
#define FQ_POW2_21  2097152
#define FQ_POW2_22  4194304
#define FQ_POW2_23  8388608
#define FQ_POW2_24  16777216
#define FQ_POW2_25  33554432
#define FQ_POW2_26  67108864
#define FQ_POW2_27  134217728
#define FQ_POW2_28  268435456
#define FQ_POW2_29  536870912
#define FQ_POW2_30  1073741824
#define FQ_POW2_31  2147483648u

/* 位掩码 · 低n位全1 */
#define FQ_MASK_1   1
#define FQ_MASK_2   3
#define FQ_MASK_3   7
#define FQ_MASK_4   15
#define FQ_MASK_5   31
#define FQ_MASK_6   63
#define FQ_MASK_7   127
#define FQ_MASK_8   255
#define FQ_MASK_9   511
#define FQ_MASK_10  1023
#define FQ_MASK_11  2047
#define FQ_MASK_12  4095
#define FQ_MASK_13  8191
#define FQ_MASK_14  16383
#define FQ_MASK_15  32767
#define FQ_MASK_16  65535

/* 满值常量 */
#define FQ_FULL_8   255
#define FQ_FULL_16  65535
#define FQ_FULL_32  4294967295u

/* 常用相位值 */
#define FQ_PHASE_FULL   240   /* 已装满 */
#define FQ_PHASE_EMPTY  15    /* 刚开始 */

#endif /* NJSG_FULLQUAN_MATH_H */
