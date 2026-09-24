/*
 * step_h7_l0_adult_return.c
 *
 * MCCP Phase 3 · Step H7 — 成虫飞回宿主 · syscall 复通
 *
 * 核心理念:
 *   H5 蚕蛹脱壳 (袋反向产 C, 脱离解释器)
 *   H6 成虫飞走 (裸机零依赖, 脱离宿主 OS)
 *   H7 成虫归来 (裸机通过 raw syscall 回宿主输出, 穿衣复通)
 *
 * = 五步循环闭环: 自观(H4) → 脱壳(H5) → 飞走(H6) → 归来穿衣(H7)
 *
 * H7 新增机制:
 *   - 裸机 syscall 桥: write(1, buf, len) 通过 x86_64 `syscall` 指令
 *   - 裸机 printf: fmt_int + fmt_str + sys_write 直接拼输出缓冲区
 *   - _start → bag_run_all() → sys_write 逐条输出 → sys_exit(0)
 *
 * raw syscall (x86_64 Linux):
 *   write  = syscall 1  (rdi=fd, rsi=buf, rdx=len)
 *   exit   = syscall 60 (rdi=exit_code)
 *
 * 满全法:
 *   层号: H6 裸机层 → + H7 syscall 复通层
 *   声母: _start → write syscall → 宿主 OS
 *   韵母: 宿主服务 (stdio ←→ 裸机)
 *   折叠: 袋子行为 → 机器码 syscall 参数
 *   下一步 (H8): 蚕蛹完全体 — 宿主 syscall 反向调用袋子 (流式双向)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <ctype.h>

/* ═══════════════════════════════════════════════════
 * 袋子行为
 * ═══════════════════════════════════════════════════ */
typedef enum { YIN=0, YANG=1 } polarity_t;
typedef int (*bag_fn_t)(int a, int b, polarity_t *pol);
typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;

static int bag_add(int a, int b, polarity_t *p){ *p=YANG; return a+b; }
static int bag_sub(int a, int b, polarity_t *p){ *p=YIN;  return a-b; }
static int bag_mul(int a, int b, polarity_t *p){ *p=(a*b>=0)?YANG:YIN; return a*b; }
static int bag_avg(int a, int b, polarity_t *p){ *p=(a-b>=0)?YANG:YIN; return (a+b)/2; }

static bag_entry_t bag_table[] = {
    { "加", bag_add },
    { "减", bag_sub },
    { "乘", bag_mul },
    { "简", bag_avg },
};
#define BAG_TABLE_SIZE ((int)(sizeof(bag_table)/sizeof(bag_table[0])))

/* ═══════════════════════════════════════════════════
 * H5/H6 风格: 结果记录器
 * ═══════════════════════════════════════════════════ */
#define REC_CAP 64
static struct { const char *name; int a,b,res; polarity_t pol; } result_buf[REC_CAP];
static int result_count = 0;

static void record(const char *n, int a, int b, int r, polarity_t p){
    if(result_count>=REC_CAP) return;
    result_buf[result_count].name=n;
    result_buf[result_count].a=a;
    result_buf[result_count].b=b;
    result_buf[result_count].res=r;
    result_buf[result_count].pol=p;
    result_count++;
}

static void bag_run_all(void){
    polarity_t pol;
    int vals[][2]={{6,4},{10,3},{8,8},{7,2}};
    for(int i=0;i<BAG_TABLE_SIZE;i++)
        for(int j=0;j<4;j++){
            pol=YIN;
            int r=bag_table[i].fn(vals[j][0],vals[j][1],&pol);
            record(bag_table[i].name,vals[j][0],vals[j][1],r,pol);
        }
}

/* ═══════════════════════════════════════════════════
 * H7: 裸机源码生成器 — 产出含 raw syscall 的 .c
 * ═══════════════════════════════════════════════════ */

static void emit_h7_bare(const char *fname){
    FILE *f=fopen(fname,"w");
    if(!f){ printf("  ⚠ %s 写入失败\n",fname); return; }

    fprintf(f,"/* ── H7 成虫归来 · 裸机 syscall 复通宿主 ── */\n");
    fprintf(f,"/* 由 step_h7_l0_adult_return.c 自动生成                    */\n");
    fprintf(f,"/* 编译: cc -ffreestanding -nostdlib -nostdinc -o bag bag_h7.c */\n\n");

    /* 类型 */
    fprintf(f,"typedef enum { YIN=0, YANG=1 } polarity_t;\n");
    fprintf(f,"typedef int (*bag_fn_t)(int a, int b, polarity_t *pol);\n");
    fprintf(f,"typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;\n\n");

    /* 裸机 write syscall (x86_64 Linux) */
    fprintf(f,"/* 裸机 write(1, buf, len) syscall */\n");
    fprintf(f,"static long sys_write(int fd, const void *buf, long len){\n");
    fprintf(f,"    long ret;\n");
    fprintf(f,"    __asm__ volatile (\n");
    fprintf(f,"        \"syscall\"\n");
    fprintf(f,"        : \"=a\"(ret)\n");
    fprintf(f,"        : \"a\"(1L), \"D\"((long)fd), \"S\"(buf), \"d\"(len)\n");
    fprintf(f,"        : \"rcx\",\"r11\",\"memory\");\n");
    fprintf(f,"    return ret;\n");
    fprintf(f,"}\n\n");

    /* 裸机 exit syscall */
    fprintf(f,"static void sys_exit(int code){\n");
    fprintf(f,"    __asm__ volatile (\n");
    fprintf(f,"        \"syscall\"\n");
    fprintf(f,"        :\n");
    fprintf(f,"        : \"a\"(60L), \"D\"((long)code)\n");
    fprintf(f,"        : \"rcx\",\"r11\",\"memory\");\n");
    fprintf(f,"}\n\n");

    /* 裸机 memset / strlen / itoa */
    fprintf(f,"static void *memcpy(void *dst,const void *src,long n){\n");
    fprintf(f,"    char *d=dst; const char *s=src; while(n--) *d++=*s++; return dst;\n");
    fprintf(f,"}\n");
    fprintf(f,"static void *memset(void *d, int c, long n){\n");
    fprintf(f,"    char *p=d; while(n--) *p++=(char)c; return d;\n");
    fprintf(f,"}\n");
    fprintf(f,"static long strlen_(const char *s){ long n=0; while(s[n]) n++; return n; }\n\n");

    /* 裸机整数→字符串 (小缓冲) */
    fprintf(f,"static int itoa(int v, char *buf, int bsz){\n");
    fprintf(f,"    char tmp[12]; int i=0, neg=0;\n");
    fprintf(f,"    if(v<0){ neg=1; v=-v; }\n");
    fprintf(f,"    if(v==0){ buf[0]='0'; buf[1]=0; return 1; }\n");
    fprintf(f,"    while(v>0 && i<10){ tmp[i++]='0'+(v%%10); v/=10; }\n");
    fprintf(f,"    int p=0; if(neg) buf[p++]='-';\n");
    fprintf(f,"    for(int j=i-1;j>=0;j--) buf[p++]=tmp[j];\n");
    fprintf(f,"    buf[p]=0; return p;\n");
    fprintf(f,"}\n\n");

    /* 袋子函数名 (baked-in UTF-8) */
    fprintf(f,"static const char bag_name_0[] = \"加\";\n");
    fprintf(f,"static const char bag_name_1[] = \"减\";\n");
    fprintf(f,"static const char bag_name_2[] = \"乘\";\n");
    fprintf(f,"static const char bag_name_3[] = \"简\";\n\n");

    /* 袋子行为 */
    fprintf(f,"static int bag_add(int a,int b,polarity_t *p){*p=YANG;return a+b;}\n");
    fprintf(f,"static int bag_sub(int a,int b,polarity_t *p){*p=YIN;return a-b;}\n");
    fprintf(f,"static int bag_mul(int a,int b,polarity_t *p){*p=(a*b>=0)?YANG:YIN;return a*b;}\n");
    fprintf(f,"static int bag_avg(int a,int b,polarity_t *p){*p=(a-b>=0)?YANG:YIN;return (a+b)/2;}\n\n");

    /* 表 */
    fprintf(f,"static bag_entry_t bag_table[]={\n");
    fprintf(f,"    {bag_name_0,bag_add},{bag_name_1,bag_sub},{bag_name_2,bag_mul},{bag_name_3,bag_avg}\n");
    fprintf(f,"};\n#define BAG_TABLE_SIZE %d\n\n",BAG_TABLE_SIZE);

    /* 记录 */
    fprintf(f,"typedef struct { const char *name; int a,b,res; polarity_t pol; } rec_t;\n");
    fprintf(f,"static rec_t result_buf[%d];\n",REC_CAP);
    fprintf(f,"static int result_count=0;\n");
    fprintf(f,"static void record(const char *n,int a,int b,int r,polarity_t p){\n");
    fprintf(f,"    if(result_count>=%d)return;\n",REC_CAP);
    fprintf(f,"    result_buf[result_count].name=n;\n");
    fprintf(f,"    result_buf[result_count].a=a;\n");
    fprintf(f,"    result_buf[result_count].b=b;\n");
    fprintf(f,"    result_buf[result_count].res=r;\n");
    fprintf(f,"    result_buf[result_count].pol=p;\n");
    fprintf(f,"    result_count++;\n");
    fprintf(f,"}\n");
    fprintf(f,"static void bag_run_all(void){\n");
    fprintf(f,"    polarity_t pol;\n");
    fprintf(f,"    int vals[][2]={{6,4},{10,3},{8,8},{7,2}};\n");
    fprintf(f,"    for(int i=0;i<BAG_TABLE_SIZE;i++)\n");
    fprintf(f,"        for(int j=0;j<4;j++){\n");
    fprintf(f,"            pol=YIN;\n");
    fprintf(f,"            int r=bag_table[i].fn(vals[j][0],vals[j][1],&pol);\n");
    fprintf(f,"            record(bag_table[i].name,vals[j][0],vals[j][1],r,pol);\n");
    fprintf(f,"        }\n");
    fprintf(f,"}\n\n");

    /* 裸机 printf 逐行输出 */
    fprintf(f,"/* 裸机 printf — 核心循环: 把 result_buf[i] 格式化到 out_buf,然后用 sys_write 输出 */\n");
    fprintf(f,"static void print_results(void){\n");
    fprintf(f,"    char out_buf[256];\n");
    fprintf(f,"    char num_buf[16];\n");
    fprintf(f,"    for(int i=0;i<result_count;i++){\n");
    fprintf(f,"        int p=0;\n");
    fprintf(f,"        /* 袋子名 (UTF-8) */\n");
    fprintf(f,"        for(const char *s=result_buf[i].name; *s; s++) out_buf[p++]=*s;\n");
    fprintf(f,"        out_buf[p++]= '(';\n");
    fprintf(f,"        { int l=itoa(result_buf[i].a,num_buf,16); for(int k=0;k<l;k++) out_buf[p++]=num_buf[k]; }\n");
    fprintf(f,"        out_buf[p++]= ',';\n");
    fprintf(f,"        { int l=itoa(result_buf[i].b,num_buf,16); for(int k=0;k<l;k++) out_buf[p++]=num_buf[k]; }\n");
    fprintf(f,"        out_buf[p++]=')'; out_buf[p++]='=';\n");
    fprintf(f,"        { int l=itoa(result_buf[i].res,num_buf,16); for(int k=0;k<l;k++) out_buf[p++]=num_buf[k]; }\n");
    fprintf(f,"        /* [阳] 或 [阴] 通过字节循环输出 */\n");
    fprintf(f,"        {\n");
    fprintf(f,"            static const char y[]=\"[阳]\\n\";\n");
    fprintf(f,"            static const char n[]=\"[阴]\\n\";\n");
    fprintf(f,"            const char *pp = (result_buf[i].pol==YANG)?y:n;\n");
    fprintf(f,"            for(int z=0; pp[z]; z++) out_buf[p++]=pp[z];\n");
    fprintf(f,"        }\n");
    fprintf(f,"        sys_write(1, out_buf, p);\n");
    fprintf(f,"    }\n");
    fprintf(f,"}\n\n");

    /* 裸机入口: _start → bag_run_all → print_results → sys_exit */
    fprintf(f,"void _start(void) __attribute__((section(\".text.start\")));\n");
    fprintf(f,"void _start(void){\n");
    fprintf(f,"    /* 先输出一道分隔 (通过 sys_write) */\n");
    fprintf(f,"    static const char hdr[] = \"── 裸机 syscall · H7 成虫归来 ──\\n\";\n");
    fprintf(f,"    sys_write(1, hdr, sizeof(hdr)-1);\n");
    fprintf(f,"    /* 袋子行为 */\n");
    fprintf(f,"    bag_run_all();\n");
    fprintf(f,"    /* 裸机 printf → 宿主 OS stdout */\n");
    fprintf(f,"    print_results();\n");
    fprintf(f,"    /* 退出 */\n");
    fprintf(f,"    sys_exit(0);\n");
    fprintf(f,"}\n");

    fclose(f);
    printf("  📄 H7 裸机 syscall 源码已写出: %s\n",fname);
}

/* ═══════════════════════════════════════════════════
 * main — 演示 H7 调用流程
 * ═══════════════════════════════════════════════════ */
int main(void){
    setvbuf(stdout,NULL,_IONBF,0);

    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  成 虫  飞 回 宿 主  (H7)                       ║\n");
    printf("║  裸机 syscall write → 宿主 OS stdout            ║\n");
    printf("║  五步循环: H4观 → H5壳 → H6走 → H7回            ║\n");
    printf("╚══════════════════════════════════════════════════╝\n\n");

    /* 步骤 A: 用宿主模式跑出结果对比 */
    printf("── 宿主模式 (参照) ──\n");
    bag_run_all();
    for(int i=0;i<result_count;i++)
        printf("  %s(%d,%d)=%d [%s]\n",
               result_buf[i].name,result_buf[i].a,result_buf[i].b,
               result_buf[i].res, result_buf[i].pol==YANG?"阳":"阴");

    /* 步骤 B: 产出 H7 裸机源码 */
    printf("\n── H7: 裸机 syscall ──\n");
    emit_h7_bare("bag_h7.c");
    printf("  🔧 编译裸机...\n");
    int rc = system("cc -ffreestanding -nostdlib -nostdinc -o bag_h7 bag_h7.c 2>&1");
    if(rc==0){
        printf("  ✅ 裸机编译成功: bag_h7\n");
        printf("  🔍 readelf -h bag_h7:\n");
        system("readelf -h bag_h7 2>&1 | grep -E 'Type|Entry|Machine' | sed 's/^/    /'");
        printf("  🔍 nm -u bag_h7 (外部符号):\n");
        int has_ext = system("nm -u bag_h7 2>/dev/null | grep -v '^$'");
        if(has_ext==0) printf("    (零外部!)\n");
        else printf("    ⚠ 存在外部符号\n");

        printf("\n  ▶ 运行裸机 (raw syscall write → stdout):\n");
        system("./bag_h7 2>&1 | sed 's/^/    /'");
        int run_rc = system("./bag_h7 > /dev/null 2>&1");
        if(run_rc==0) printf("  ✅ 裸机运行成功 (exit=0)\n");
        else printf("  ⚠ 裸机运行码=%d\n", run_rc);
    } else {
        printf("  ⚠ 裸机编译失败\n");
    }

    /* 总结 */
    printf("\n╔══════════════════════════════════════════════════╗\n");
    printf("║  H7 成虫飞回宿主 — 完成                         ║\n");
    printf("║  1. 裸机通过 syscall(1=write) 输出到宿主 stdout  ║\n");
    printf("║  2. 无需 libc printf — 自写 itoa + write 循环   ║\n");
    printf("║  3. sys_exit(0) 正常归还控制权                  ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║  五步循环闭环: H4观 → H5壳 → H6飞 → H7归       ║\n");
    printf("║  层: H6裸机层 → + H7 syscall复通层             ║\n");
    printf("║  声母: _start→write syscall→宿主 OS            ║\n");
    printf("║  韵母: 宿主服务 (stdio ←→ 裸机)                 ║\n");
    printf("║  下一步 (H8): 蚕蛹完全体 — 宿主→袋子反向调用    ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");
    return 0;
}
