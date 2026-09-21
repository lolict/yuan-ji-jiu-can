/* ── H6 成虫 · 裸机零依赖二进制 ── */
/* 由 step_h6_l0_adult_fly.c 自动生成       */
/* 编译: cc -ffreestanding -nostdlib -nostdinc -o bag_bare bag_bare.c  */

typedef enum { YIN=0, YANG=1 } polarity_t;
typedef int (*bag_fn_t)(int a, int b, polarity_t *pol);
typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;

static void *memcpy(void *dst, const void *src, long n) {
    char *d = dst; const char *s = src;
    while(n--) *d++ = *s++;
    return dst;
}
static void *memset(void *dst, int c, long n) {
    char *d = dst;
    while(n--) *d++ = (char)c;
    return dst;
}

static const char bag_name_0[] = "加";
static const char bag_name_1[] = "减";
static const char bag_name_2[] = "乘";
static const char bag_name_3[] = "简";

static int bag_add(int a, int b, polarity_t *pol) { *pol=YANG;  return a+b; }
static int bag_sub(int a, int b, polarity_t *pol) { *pol=YIN;   return a-b; }
static int bag_mul(int a, int b, polarity_t *pol) { *pol=(a*b>=0)?YANG:YIN; return a*b; }
static int bag_avg(int a, int b, polarity_t *pol) { *pol=(a-b>=0)?YANG:YIN; return (a+b)/2; }

static bag_entry_t bag_table[] = {
    { bag_name_0, bag_add },
    { bag_name_1, bag_sub },
    { bag_name_2, bag_mul },
    { bag_name_3, bag_avg },
};
#define BAG_TABLE_SIZE 4

#define RECORD_CAP 64
typedef struct { const char *name; int a,b,res; polarity_t pol; } rec_t;
static rec_t result_buf[RECORD_CAP];
static int result_count = 0;
static void record(const char *n,int a,int b,int r,polarity_t p){
    if(result_count>=RECORD_CAP)return;
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

/* 裸机入口 — 无 libc _start, 直接调 bag_start */
__attribute__((section(".text.start")))
void _start(void) {
    bag_run_all();
    /* 裸机: 无限循环 (无 OS 可退出) */
    __asm__ volatile ("1: jmp 1b");
}

#ifdef HOST_ADAPTER
/* 宿主适配: 用于验证 */
int main(void){
    bag_run_all();
    for(int i=0;i<result_count;i++){
        printf("%s(%d,%d)=%d [%s]\n",
               result_buf[i].name, result_buf[i].a,
               result_buf[i].b, result_buf[i].res,
               result_buf[i].pol==YANG?"阳":"阴");
    }
    return 0;
}
#endif
