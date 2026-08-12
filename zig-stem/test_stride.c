/* test_stride.c — stride base arithmetic tests */

#include <stdio.h>
#include <stdint.h>
#include <assert.h>

typedef uint8_t  mqf_u8;
typedef uint16_t mqf_u16;
typedef uint32_t mqf_u32;
typedef uint64_t mqf_u64;
typedef int32_t  mqf_i32;

typedef struct {
    mqf_u32 sheet;
    mqf_u8  symbol;
} mqf_stride;

/* external functions from main.c */
extern mqf_stride mqf_stride_from_u64(mqf_u64 n);
extern mqf_u64    mqf_stride_to_u64(mqf_stride s);
extern mqf_stride mqf_stride_add(mqf_stride a, mqf_stride b);
extern mqf_stride mqf_stride_mul(mqf_stride a, mqf_stride b);
extern mqf_i32    mqf_stride_compare(mqf_stride a, mqf_stride b);
extern mqf_u64    mqf_stride_absolute(mqf_stride s);
extern mqf_i32    mqf_is_adjacent(mqf_u8 x, mqf_u8 y, mqf_u8 z,
                                  mqf_u8 ax, mqf_u8 ay, mqf_u8 az);

void test_stride_from_to_u64() {
    mqf_stride s0 = mqf_stride_from_u64(0);
    assert(s0.sheet == 0 && s0.symbol == 0);

    mqf_stride s63 = mqf_stride_from_u64(63);
    assert(s63.sheet == 0 && s63.symbol == 63);

    mqf_stride s64 = mqf_stride_from_u64(64);
    assert(s64.sheet == 1 && s64.symbol == 0);

    mqf_u64 i;
    for (i = 0; i < 1000; i++) {
        mqf_stride s = mqf_stride_from_u64(i);
        mqf_u64 back = mqf_stride_to_u64(s);
        assert(back == i);
    }
}

void test_stride_add_identity() {
    mqf_stride zero = mqf_stride_from_u64(0);
    mqf_u64 i;
    for (i = 0; i < 200; i++) {
        mqf_stride a = mqf_stride_from_u64(i);
        mqf_stride sum = mqf_stride_add(a, zero);
        assert(mqf_stride_compare(sum, a) == 0);
    }
}

void test_spiral_non_overlap() {
    mqf_stride s0 = mqf_stride_from_u64(0);
    mqf_stride s1 = mqf_stride_from_u64(64);
    mqf_stride s2 = mqf_stride_from_u64(128);

    mqf_u64 abs0 = mqf_stride_absolute(s0);
    mqf_u64 abs1 = mqf_stride_absolute(s1);
    mqf_u64 abs2 = mqf_stride_absolute(s2);

    assert(abs0 == 0);
    assert(abs1 == 24);
    assert(abs2 == 48);
    assert(abs0 != abs1);
    assert(abs1 != abs2);
}

void test_period_15_16() {
    mqf_u64 abs_15 = 15 * 24;
    assert(abs_15 % 360 == 0);
    assert(abs_15 % 384 == 360);

    mqf_u64 abs_16 = 16 * 24;
    assert(abs_16 % 384 == 0);
    assert(abs_16 % 360 == 24);
}

void test_adjacent() {
    assert(mqf_is_adjacent(0, 0, 0, 1, 0, 0) == 1);
    assert(mqf_is_adjacent(0, 0, 0, 0, 1, 0) == 1);
    assert(mqf_is_adjacent(0, 0, 0, 0, 0, 1) == 1);
    assert(mqf_is_adjacent(0, 0, 0, 2, 0, 0) == 0);
    assert(mqf_is_adjacent(0, 0, 0, 1, 1, 0) == 0);
}

int main() {
    test_stride_from_to_u64();
    test_stride_add_identity();
    test_spiral_non_overlap();
    test_period_15_16();
    test_adjacent();
    printf("all stride tests passed!\n");
    return 0;
}
