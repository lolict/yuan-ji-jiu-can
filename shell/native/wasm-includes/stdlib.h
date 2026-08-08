/* 囡囝共生:freestanding stdlib.h stub for syntax check */
#ifndef NJFG_STDLIB_H
#define NJFG_STDLIB_H
#include <stddef.h>
void* malloc(size_t n);
void free(void* p);
long strtod(const char* nptr, char** endptr);
#endif
