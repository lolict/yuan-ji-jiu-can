/* 囡囧共生:freestanding string.h stub for syntax check */
#ifndef NJFG_STRING_H
#define NJFG_STRING_H
#include <stddef.h>
void* memset(void* s, int c, size_t n);
void* memcpy(void* dest, const void* src, size_t n);
char* strncpy(char* dest, const char* src, size_t n);
char* strstr(const char* haystack, const char* needle);
size_t strlen(const char* s);
#endif
