/*
 * fqm_browser.c — 意交感息独立浏览器内核
 *
 * 功能：完全不使用 HTTP/HTML/TCP 等外部协议
 *   - 使用 gttx:// 自有协议
 *   - 原始帧收发（绕过 TCP/IP 协议栈）
 *   - 三维坐标映射替代 DOM 树
 *   - 符号渲染替代 HTML 解析
 *
 * 世界观：阶和世界观 — 容器内外串联
 *   - 每一次请求 = 从一个容器到另一个容器的跳跃
 *   - 页面 = 一帧坐标集合，不是"文档"
 *
 * 满全法规则：
 *   - 根基锚点：高位持有者=1，低位持有者=0
 *   - 集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_browser fqm_browser.c
 * 运行：./fqm_browser
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 协议常量 ========== */
#define GTTX_MAGIC      0x47545858  /* "GTXX" = gttx 魔数 */
#define GTTX_VERSION    0x01        /* gttx 协议版本 */
#define FRAME_MAX_SIZE  254         /* 254+2 集装箱编码 */
#define COORD_MAX       65536       /* 64K 坐标空间 */

/* ========== 坐标点（零维索引）========== */
typedef struct {
    unsigned int x;     /* 横坐标 */
    unsigned int y;     /* 纵坐标 */
    unsigned int z;     /* 时间/版本坐标 */
} Coord3D;

/* ========== gttx 帧头 ========== */
/*
 * 不使用任何现有协议头
 * 完全自有结构：魔数 + 版本 + 容器ID + 操作 + 长度
 */
typedef struct {
    unsigned int  magic;        /* 魔数: 0x47545858 "GTXX" */
    unsigned char version;      /* 协议版本 */
    unsigned char container;    /* 源容器 ID */
    unsigned char opcode;       /* 操作码 */
    unsigned char reserved;     /* 保留 */
    unsigned short payload_len; /* 载荷长度（254 以内）*/
    unsigned short checksum;    /* 校验和 */
} GttxHeader;

/* ========== gttx 操作码 ========== */
#define GTTX_OP_CONNECT  0x01   /* 建立连接（请求） */
#define GTTX_OP_TRANSFER 0x02   /* 传送数据（响应） */
#define GTTX_OP_QUERY    0x03   /* 查询坐标点 */
#define GTTX_OP_UPDATE   0x04   /* 更新容器内容 */
#define GTTX_OP_DISCONNECT 0x05 /* 断开 */

/* ========== 浏览器内核状态 ========== */
typedef struct {
    int connected;              /* 连接状态 */
    int container_id;           /* 当前容器 */
    Coord3D cursor;             /* 当前坐标光标 */
    unsigned long long frame_count; /* 帧计数 */
    char domain[128];           /* gttx 域名 */
    
    /* 符号页缓存（不使用 DOM） */
    Coord3D page_coords[4096];
    int page_coord_count;
    
    /* 时间链秘书 */
    unsigned int last_timestamp;
} BrowserKernel;

/* ========== 容器定义 ========== */
typedef struct {
    char name[64];
    int level;
    Coord3D origin;     /* 容器起点坐标 */
    int size;           /* 容器大小 */
} PageContainer;

#define MAX_CONTAINERS 256
static PageContainer g_containers[MAX_CONTAINERS];
static int g_container_count = 0;

/* ========== fqm 文件头 ========== */
typedef struct {
    unsigned char magic[4];
    unsigned char version;
    unsigned char ext_type;
    unsigned char reserved;
    unsigned int  coord_base;
    unsigned short container_count;
    unsigned short flags;
} FqmFileHeader;

typedef struct {
    unsigned int  slot_id;
    unsigned int  coord;
    unsigned char polarity;
    unsigned char sovereignty;
    unsigned char value;
    unsigned char level;
} FqmSlot;

static const struct { const char *ext; unsigned char type; const char *desc; } g_ext_map[] = {
    {".loli",0x01,"洛丽"},
    {".toti",0x02,"托蒂"},
    {".cocoonet",0x03,"蝉蜕网"},
    {".gtti",0x04,"时序投影"},
    {".chu",0x05,"出容"},
    {".ltti",0x06,"类时序"},
    {".gtt",0x07,"时序字体"},
    {".mltti",0x08,"多元时序"},
    {".chut",0x09,"出通"},
    {".ysyt",0x0A,"银丝韵"},
    {".ylyt",0x0B,"玉露韵"},
    {".aide",0x0C,"伴生"},
    {".atotiy",0x0D,"反托蒂"},
    {".aittiy",0x0E,"反提蒂"},
    {NULL,0,NULL}
};

/* ========== 初始化浏览器内核 ========== */
void browser_init(BrowserKernel *kb, const char *domain) {
    memset(kb, 0, sizeof(BrowserKernel));
    strncpy(kb->domain, domain, 127);
    kb->container_id = 0;
    kb->cursor.x = 0;
    kb->cursor.y = 0;
    kb->cursor.z = 0;
    kb->frame_count = 0;
    kb->page_coord_count = 0;
    kb->last_timestamp = (unsigned int)time(NULL);
    
    printf("[浏览器内核] 初始化完成\n");
    printf("  协议: gttx://%s\n", domain);
    printf("  锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
}

/* ========== 创建容器 ========== */
int browser_create_container(const char *name, int level, 
                              unsigned int ox, unsigned int oy, unsigned int oz) {
    if (g_container_count >= MAX_CONTAINERS) return -1;
    
    PageContainer *c = &g_containers[g_container_count];
    strncpy(c->name, name, 63);
    c->level = level;
    c->origin.x = ox;
    c->origin.y = oy;
    c->origin.z = oz;
    c->size = 0;
    
    printf("[容器] 创建: %s 层级=%d 起点(%u,%u,%u)\n",
           name, level, ox, oy, oz);
    
    return g_container_count++;
}

/* ========== 构建 gttx 帧头 ========== */
GttxHeader build_gttx_header(unsigned char container, unsigned char opcode, 
                              unsigned short payload_len) {
    GttxHeader h;
    h.magic = GTTX_MAGIC;
    h.version = GTTX_VERSION;
    h.container = container;
    h.opcode = opcode;
    h.reserved = 0;
    h.payload_len = payload_len;
    h.checksum = (unsigned short)(GTTX_MAGIC ^ container ^ opcode ^ payload_len);
    return h;
}

/* ========== 打印帧（模拟原始帧收发）========== */
void browser_send_frame(BrowserKernel *kb, GttxHeader *h, const unsigned char *payload) {
    kb->frame_count++;
    kb->last_timestamp = (unsigned int)time(NULL);
    
    printf("[%llu] gttx: ", kb->frame_count);
    printf("魔数=%08X 版本=%d 容器=%d ", h->magic, h->version, h->container);
    
    const char *op_name = "?";
    switch (h->opcode) {
        case GTTX_OP_CONNECT:    op_name = "连接"; break;
        case GTTX_OP_TRANSFER:   op_name = "传输"; break;
        case GTTX_OP_QUERY:      op_name = "查询"; break;
        case GTTX_OP_UPDATE:     op_name = "更新"; break;
        case GTTX_OP_DISCONNECT: op_name = "断开"; break;
    }
    printf("操作=%s 长度=%d", op_name, h->payload_len);
    
    if (payload && h->payload_len > 0 && h->payload_len <= 32) {
        printf(" 数据=[");
        for (int i = 0; i < h->payload_len && i < 16; i++) {
            printf("%02X", payload[i]);
        }
        if (h->payload_len > 16) printf("...");
        printf("]");
    }
    printf("\n");
    
    /* 时间版本号记录 */
    printf("  └─ 时间戳: %u (秘书记录)\n", kb->last_timestamp);
}

/* ========== 请求容器（建立连接并获取数据）========== */
void browser_request_container(BrowserKernel *kb, int container_id) {
    printf("\n[请求] 获取容器: %s\n", g_containers[container_id].name);
    
    /* 发送连接请求帧 */
    unsigned char payload[4] = { container_id, 0, 0, 0 };
    GttxHeader hdr = build_gttx_header(kb->container_id, GTTX_OP_CONNECT, 4);
    browser_send_frame(kb, &hdr, payload);
    
    /* 接收容器内容（模拟响应） */
    unsigned char resp_payload[254];
    memset(resp_payload, 0, 254);
    
    /* 填充坐标数据 */
    Coord3D origin = g_containers[container_id].origin;
    for (int i = 0; i < 8; i++) {
        Coord3D *c = (Coord3D *)(resp_payload + i * sizeof(Coord3D));
        c->x = origin.x + i * 100;
        c->y = origin.y + i * 50;
        c->z = origin.z;
        
        /* 添加到页缓存 */
        if (kb->page_coord_count < 4096) {
            kb->page_coords[kb->page_coord_count++] = *c;
        }
    }
    
    GttxHeader resp_hdr = build_gttx_header(container_id, GTTX_OP_TRANSFER, 8 * sizeof(Coord3D));
    browser_send_frame(kb, &resp_hdr, resp_payload);
    
    /* 光标移动 */
    kb->cursor.x = origin.x;
    kb->cursor.y = origin.y;
    kb->cursor.z = origin.z;
}

/* ========== 查询坐标点 ========== */
Coord3D* browser_query_coord(BrowserKernel *kb, unsigned int x, unsigned int y, unsigned int z) {
    printf("[查询] 坐标(%u, %u, %u)\n", x, y, z);
    
    /* 查询帧 */
    unsigned char query_payload[12];
    memcpy(query_payload, &x, 4);
    memcpy(query_payload + 4, &y, 4);
    memcpy(query_payload + 8, &z, 4);
    
    GttxHeader hdr = build_gttx_header(kb->container_id, GTTX_OP_QUERY, 12);
    browser_send_frame(kb, &hdr, query_payload);
    
    /* 返回当前光标坐标作为结果 */
    return &kb->cursor;
}

/* ========== 断开连接 ========== */
void browser_disconnect(BrowserKernel *kb) {
    GttxHeader hdr = build_gttx_header(kb->container_id, GTTX_OP_DISCONNECT, 0);
    browser_send_frame(kb, &hdr, NULL);
    kb->connected = 0;
    printf("\n[断开] 容器连接已关闭\n");
}

/* ========== 显示浏览器状态 ========== */
void browser_status(BrowserKernel *kb) {
    printf("\n========== 浏览器内核状态 ==========\n");
    printf("协议: gttx://%s\n", kb->domain);
    printf("光标: (%u, %u, %u)\n", kb->cursor.x, kb->cursor.y, kb->cursor.z);
    printf("容器: %s (ID=%d)\n", g_containers[kb->container_id].name, kb->container_id);
    printf("帧数: %llu\n", kb->frame_count);
    printf("页坐标缓存: %d 个点\n", kb->page_coord_count);
    printf("锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    
    printf("\n坐标缓存:\n");
    for (int i = 0; i < kb->page_coord_count; i++) {
        printf("  [%d] (%u, %u, %u)\n", i,
               kb->page_coords[i].x,
               kb->page_coords[i].y,
               kb->page_coords[i].z);
    }
}

/* ========== 主函数 ========== */
static unsigned char* read_fqm_file(const char *path, size_t *out_size) {
    FILE *f = fopen(path, "rb");
    if (!f) { printf("[错误] 无法打开: %s (%s)\n", path, strerror(errno)); return NULL; }
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (sz <= 0 || sz > 1048576) { printf("[错误] 异常大小 %ld\n", sz); fclose(f); return NULL; }
    unsigned char *buf = malloc(sz);
    if (!buf) { fclose(f); return NULL; }
    fread(buf, 1, sz, f);
    fclose(f);
    *out_size = (size_t)sz;
    return buf;
}

static int parse_fqm_header(const unsigned char *data, size_t size, FqmFileHeader *h) {
    if (size < 15) return -1;
    memcpy(h->magic, data, 4);
    h->version = data[4];
    h->ext_type = data[5];
    h->reserved = data[6];
    h->coord_base = (data[7]<<24)|(data[8]<<16)|(data[9]<<8)|data[10];
    h->container_count = (data[11]<<8)|data[12];
    h->flags = (data[13]<<8)|data[14];
    return 0;
}

static unsigned char ext_to_type(const char *filename) {
    const char *dot = strrchr(filename, '.');
    if (!dot) return 0xFF;
    for (int i = 0; g_ext_map[i].ext; i++)
        if (strcasecmp(dot, g_ext_map[i].ext) == 0) return g_ext_map[i].type;
    return 0xFF;
}

static const char* ext_desc(unsigned char type) {
    for (int i = 0; g_ext_map[i].ext; i++)
        if (g_ext_map[i].type == type) return g_ext_map[i].desc;
    return "未知";
}

static int parse_fqm(const unsigned char *data, size_t size, BrowserKernel *kb) {
    FqmFileHeader h;
    if (parse_fqm_header(data, size, &h) < 0) { printf("[错误] 非有效 fqm\n"); return -1; }
    printf("\n[解析]\n 魔数:%c%c%c%c 版本:%d 类型:0x%02X(%s)\n 坐标原点:0x%08X 容器:%d 标志:0x%04X\n",
        h.magic[0],h.magic[1],h.magic[2],h.magic[3], h.version, h.ext_type, ext_desc(h.ext_type),
        h.coord_base, h.container_count, h.flags);
    size_t off = 15;
    for (int i = 0; i < h.container_count && i < MAX_CONTAINERS; i++) {
        if (off + 11 > size) break;
        FqmSlot *s = (FqmSlot*)(data + off); off += 11;
        char nm[64]; snprintf(nm,63,"槽%d_%s", s->slot_id, ext_desc(h.ext_type));
        browser_create_container(nm, s->level, s->coord, s->polarity, s->sovereignty);
        g_containers[g_container_count-1].origin.z = s->value;
    }
    return 0;
}

static void write_test_fqm(const char *path) {
    FILE *f = fopen(path, "wb");
    if (!f) return;
    unsigned char hdr[15] = {'F','Q','M','T',0x01,0x01,0x00,0,0,0,0x10,0,4,0,0};
    fwrite(hdr,1,15,f);
    for (int i=0;i<4;i++) {
        unsigned char s[11] = {0};
        s[0]=i; s[4]=(i%2)?0:1; s[5]=(i+1)*50; s[6]=100+i*20; s[7]=i;
        fwrite(s,1,11,f);
    }
    fclose(f);
    printf(" 测试文件已生成: %s\n", path);
}

int main(int argc, char *argv[]) {
    printf("意交感息（isi）独立浏览器内核\n锚点: 我=%d 刘楚恬=%d\n集装箱: mlll 254+2\n身份: 刘楚恬和我\n\n",
        ANCHOR_SELF, ANCHOR_LIU);

    BrowserKernel browser;
    browser_init(&browser, "hieyair");

    const char *filepath = NULL;
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i],"--test")==0) write_test_fqm("/tmp/test.loli");
        else if (argv[i][0] != '-') filepath = argv[i];
    }

    if (filepath) {
        unsigned char ft = ext_to_type(filepath);
        printf("[打开] %s (类型 0x%02X: %s)\n", filepath, ft, ext_desc(ft));
        size_t sz;
        unsigned char *data = read_fqm_file(filepath, &sz);
        if (data) { parse_fqm(data, sz, &browser); free(data); }
    } else {
        printf("\n[演示容器]\n");
        browser_create_container("演示甲", 0, 16,16,0);
        browser_create_container("演示乙", 1, 32,32,0);
    }

    if (g_container_count > 0) {
        printf("\n[加载]\n");
        for (int i = 0; i < g_container_count; i++) browser_request_container(&browser, i);
    }

    browser_status(&browser);
    browser_disconnect(&browser);
    printf("\n========== 完成 ==========\n");
    return 0;
}
