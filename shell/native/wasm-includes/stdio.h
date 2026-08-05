/* 囡囧共生:freestanding stdio.h stub for syntax check */
#ifndef NJFG_STDIO_H
#define NJFG_STDIO_H
#include <stddef.h>
typedef struct { char dummy; } FILE;
extern FILE* stdin;
extern FILE* stdout;
int snprintf(char* buf, size_t n, const char* fmt, ...);
int printf(const char* fmt, ...);
size_t fread(void* ptr, size_t size, size_t nmemb, FILE* stream);
size_t fwrite(const void* ptr, size_t size, size_t nmemb, FILE* stream);
int fflush(FILE* stream);
#endif
