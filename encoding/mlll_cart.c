/**
 * mlll_cart.c — 254+2 字车容器实现
 */

#include "mlll_cart.h"
#include <string.h>

void mlll_init(mlll_whole_t *box) {
    memset(box, 0, sizeof(*box));
    /* 锚点根基 */
    box->anchor[0].slot = 0;     box->anchor[0].container = MLLL_ANCHOR_CHT;
    box->anchor[0].occupied = 1; box->anchor[0].cunit = CUNIT_64;
    box->anchor[1].slot = 1;     box->anchor[1].container = MLLL_ANCHOR_WO;
    box->anchor[1].occupied = 1; box->anchor[1].cunit = CUNIT_64;
    box->used = 2;
}

int mlll_load(mlll_whole_t *box, uint32_t container, uint8_t cunit) {
    for (int i = 0; i < MLLL_CART_COUNT; i++) {
        if (!box->cart[i].occupied) {
            box->cart[i].slot = (uint16_t)i;
            box->cart[i].container = container;
            box->cart[i].cunit = cunit;
            box->cart[i].occupied = 1;
            box->used++;
            return i;
        }
    }
    return -1;  /* 满 */
}

uint32_t mlll_unload(mlll_whole_t *box, uint16_t slot) {
    if (slot >= MLLL_CART_COUNT || !box->cart[slot].occupied) return 0;
    uint32_t v = box->cart[slot].container;
    box->cart[slot].occupied = 0;
    box->cart[slot].container = 0;
    box->used--;
    return v;
}

int mlll_find_empty(mlll_whole_t *box) {
    for (int i = 0; i < MLLL_CART_COUNT; i++) {
        if (!box->cart[i].occupied) return i;
    }
    return -1;
}

uint16_t mlll_count_used(mlll_whole_t *box) { return box->used; }
uint16_t mlll_count_free(mlll_whole_t *box) { return MLLL_TOTAL - box->used; }
