/*
 * fqm_browser.c — 意交感息独立浏览器内核
 *
 * 功能：完全不使用 HTTP/HTML/TCP 等外部协议
 *   - 使用 gttx:// 自有协议
 *   - 原始帧收发（绕过 TCP/IP 协议栈）
 *   - 三维坐标映射替代 DOM 树
 *   - 符号渲染替代 HTML 解析
 *
 * 架构：
 *   fqm.* 文件 → mlil_file_t 程序 → mlll_vm 执行 → gttx 帧传输
 *   容器编码: mlll 254+2 （不依赖32/64位系统）
 *
 * 满全法规则：
 *   - 根基锚点：高位持有者=1，低位持有者=0
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_browser fqm_browser.c encoding/mlll_vm.c encoding/mlll_cart.c
 * 运行：./fqm_browser [--test] [文件.fqm.*]
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <errno.h>
#include <stdint.h>

#include "encoding/mlll_vm.h"
#include "encoding/mlll_cart.h"
#include "encoding/mlll_transcomp.h"
#include "encoding/mlil_format.h"

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== gttx 协议常量 ========== */
#define GTTX_MAGIC      0x47545858
#define GTTX_VERSION    0x01
#define FRAME_MAX_SIZE  254
#define COORD_MAX       65536

/* ========== 坐标点 ========== */
typedef struct {
    unsigned int x, y, z;
} Coord3D;

/* ========== gttx 帧头 ========== */
typedef struct {
    unsigned int  magic;
    unsigned char version;
    unsigned char container;
    unsigned char opcode;
    unsigned char reserved;
    unsigned short payload_len;
    unsigned short checksum;
} GttxHeader;

/* ========== gttx 操作码 ========== */
#define GTTX_OP_CONNECT    0x01
#define GTTX_OP_TRANSFER   0x02
#define GTTX_OP_QUERY      0x03
#define GTTX_OP_UPDATE     0x04
#define GTTX_OP_DISCONNECT 0x05
#define GTTX_OP_STEP       0x10   /* 驱动 mlll_vm 执行一步 */
#define GTTX_OP_LOAD       0x11   /* 从字车取容器到寄存器 */
#define GTTX_OP_STORE      0x12   /* 从寄存器存入字车 */

/* ========== fqm 文件头 ========== */
typedef struct {
    unsigned char magic[4];     /* 'F','Q','M','T' 或 'F','Q','M','B' */
    unsigned char version;
    unsigned char ext_type;     /* 扩展名对应的容器类型 */
    unsigned char reserved;
    unsigned int  coord_base;
    unsigned short container_count;
    unsigned short flags;
} FqmFileHeader;

#define FQM_HEADER_SIZE  15
#define FQM_SLOT_SIZE    11

/* ========== fqm 扩展名映射 ========== */
static const struct { const char *ext; unsigned char type; const char *desc; } g_ext_map[] = {
    { ".loli",     0x01, "洛丽容器" },
    { ".toti",     0x02, "托蒂几何" },
    { ".cocoonet", 0x03, "蝉蜕网络" },
    { ".gtti",     0x04, "时序投影" },
    { ".chu",      0x05, "出容器" },
    { ".ltti",     0x06, "类时序" },
    { ".gtt",      0x07, "时序字体" },
    { ".mltti",    0x08, "多元时序" },
    { ".chut",     0x09, "出通道" },
    { ".ysyt",     0x0A, "银丝韵体" },
    { ".ylyt",     0x0B, "玉露韵体" },
    { ".aide",     0x0C, "伴生" },
    { ".atotiy",   0x0D, "反托蒂" },
    { ".aittiy",   0x0E, "反提蒂" },
    { ".lchut",    0x0F, "洛出" },
    { NULL, 0, NULL }
};

/* ========== 浏览器内核状态 ========== */
typedef struct {
    int connected;
    int container_id;
    Coord3D cursor;
    unsigned long long frame_count;
    char domain[128];

    /* mlll 虚拟机 + 字车 */
    mlll_vm_t      vm;
    mlll_whole_t   box;

    /* 解析出的 mlil 程序 */
    mlil_file_t    program;

    /* 时间秘书 */
    unsigned int last_timestamp;
} BrowserKernel;

/* ========== 初始化 ========== */
void browser_init(BrowserKernel *kb, const char *domain) {
    memset(kb, 0, sizeof(BrowserKernel));
    strncpy(kb->domain, domain, 127);
    kb->cursor.x = 0;
    kb->cursor.y = 0;
    kb->cursor.z = 0;
    kb->frame_count = 0;
    kb->last_timestamp = (unsigned int)time(NULL);
    kb->connected = 1;

    /* 初始化 mlll 字车 */
    mlll_init(&kb->box);

    /* 初始化虚拟机 (程序稍后加载) */
    kb->program.header.magic1    = MLIL_MAGIC_1;
    kb->program.header.magic2    = MLIL_MAGIC_2;
    kb->program.header.version   = MLIL_VERSION;
    kb->program.header.slot_count = 0;
    kb->program.header.data_offset = sizeof(mlil_header_t);
    memset(kb->program.surfaces, 0, sizeof(kb->program.surfaces));
    memset(kb->program.data, 0, sizeof(kb->program.data));

    mlll_vm_init(&kb->vm, &kb->program, &kb->box);

    printf("[浏览器] gttx://%s  mlll_vm+字车已就绪\n", domain);
    printf("  锚点: 我=%d 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
}

/* ========== 从 fqm 文件加载程序到 vm ========== */
int browser_load_fqm(BrowserKernel *kb, const unsigned char *data, size_t size) {
    if (size < FQM_HEADER_SIZE) { printf("[错误] fqm 文件太小\n"); return -1; }

    FqmFileHeader h;
    memcpy(h.magic, data, 4);
    h.version        = data[4];
    h.ext_type       = data[5];
    h.reserved       = data[6];
    h.coord_base     = (data[7]<<24)|(data[8]<<16)|(data[9]<<8)|data[10];
    h.container_count= (data[11]<<8)|data[12];
    h.flags          = (data[13]<<8)|data[14];

    printf("\n[fqm 解析] 魔数:%c%c%c%c 版本:%d 类型:0x%02X 容器数:%d\n",
        h.magic[0],h.magic[1],h.magic[2],h.magic[3],
        h.version, h.ext_type, h.container_count);

    /* 把每个槽转换成一个 mlil_surface（=一条 mlll 指令） */
    size_t off = FQM_HEADER_SIZE;
    int slot;

    for (slot = 0; slot < h.container_count && slot < 254; slot++) {
        if (off + FQM_SLOT_SIZE > size) break;

        unsigned int  slot_id     = data[off];
        unsigned int  container   = (data[off+1]<<24)|(data[off+2]<<16)|(data[off+3]<<8)|data[off+4];
        unsigned char polarity    = data[off+5];
        unsigned char sovereignty = data[off+6];
        unsigned char value       = data[off+7];
        unsigned char level       = data[off+8];
        /* off+9, off+10 = reserved */
        off += FQM_SLOT_SIZE;

        /* 用 polarity 推操作码：阴(0)=取出  阳(1)=存入  盈(2)=裁判 */
        uint8_t opcode;
        switch (polarity) {
            case 0:  opcode = MLLL_OP_LOAD;  break;
            case 1:  opcode = MLLL_OP_STORE; break;
            case 2:  opcode = MLLL_OP_JUDGE; break;
            default: opcode = MLLL_OP_NOP;   break;
        }

        /* 填入 mlil surface */
        mlil_surface_t *sf = &kb->program.surfaces[slot];
        sf->modulus   = opcode;
        sf->face      = slot_id;       /* face = 源槽 */
        sf->name      = level;         /* name = 目标寄存器或槽 */
        sf->container = container;     /* container = 32位容器编码 */

        /* 同时把容器装入字车对应槽 */
        kb->box.cart[slot_id].container = container;
        kb->box.cart[slot_id].occupied  = 1;
        kb->box.cart[slot_id].slot      = slot_id;
        kb->box.cart[slot_id].cunit     = CUNIT_64;
        kb->box.used++;

        printf("  槽#%d: %s 槽=%d 容器=0x%08X 主权=%d 层级=%d\n", slot,
            (polarity==0?"取出":polarity==1?"存入":"裁判"),
            slot_id, container, sovereignty, level);
    }

    kb->program.header.slot_count = (uint32_t)slot;

    /* 重新初始化 vm (有了程序后) */
    mlll_vm_init(&kb->vm, &kb->program, &kb->box);

    printf("[fqm 加载] %d 条指令已装入 mlll_vm\n", slot);
    return 0;
}

/* ========== gttx 帧 ========== */
GttxHeader build_gttx_header(unsigned char container, unsigned char opcode, unsigned short len) {
    GttxHeader h;
    h.magic       = GTTX_MAGIC;
    h.version     = GTTX_VERSION;
    h.container   = container;
    h.opcode      = opcode;
    h.reserved    = 0;
    h.payload_len = len;
    h.checksum    = (unsigned short)(GTTX_MAGIC ^ container ^ opcode ^ len);
    return h;
}

void browser_send_frame(BrowserKernel *kb, GttxHeader *h, const unsigned char *payload) {
    kb->frame_count++;
    kb->last_timestamp = (unsigned int)time(NULL);

    const char *op_name;
    switch (h->opcode) {
        case GTTX_OP_CONNECT:    op_name = "连接";   break;
        case GTTX_OP_TRANSFER:   op_name = "传输";   break;
        case GTTX_OP_QUERY:      op_name = "查询";   break;
        case GTTX_OP_UPDATE:     op_name = "更新";   break;
        case GTTX_OP_DISCONNECT: op_name = "断开";   break;
        case GTTX_OP_STEP:       op_name = "步";     break;
        case GTTX_OP_LOAD:       op_name = "取";     break;
        case GTTX_OP_STORE:      op_name = "存";     break;
        default:                 op_name = "?";     break;
    }

    printf("[%llu] gttx: 容器=%d 操作=%s 长度=%d",
        kb->frame_count, h->container, op_name, h->payload_len);
    if (payload && h->payload_len > 0 && h->payload_len <= 16) {
        printf(" 数据=[");
        for (int i=0;i<(int)h->payload_len && i<8;i++) printf("%02X",payload[i]);
        if (h->payload_len>8) printf("..");
        printf("]");
    }
    printf("\n  └─ 时间戳:%u\n", kb->last_timestamp);
}

/* ========== 请求容器 ========== */
void browser_request_container(BrowserKernel *kb, int container_id) {
    printf("\n[请求] 容器: #%d\n", container_id);

    unsigned char payload[4] = { container_id, 0, 0, 0 };
    GttxHeader hdr = build_gttx_header(kb->container_id, GTTX_OP_CONNECT, 4);
    browser_send_frame(kb, &hdr, payload);

    /* 驱动 mlll_vm 执行一步 (如果有程序) */
    if (kb->vm.running && kb->vm.pc < kb->program.header.slot_count) {
        int ret = mlll_vm_step(&kb->vm);
        printf("  └─ vm: PC=%d 运行=%d R0=0x%08X 裁判=%d\n",
            kb->vm.pc, kb->vm.running, kb->vm.reg[0], kb->vm.last_admit);
        if (ret) {
            unsigned char status = (ret > 0) ? 0x00 : 0xFF;
            unsigned char resp[5] = { (unsigned char)container_id, status,
                                      (unsigned char)kb->vm.last_admit, 0, 0 };
            GttxHeader resp_hdr = build_gttx_header(container_id, GTTX_OP_TRANSFER, 5);
            browser_send_frame(kb, &resp_hdr, resp);
        }
    }

    kb->cursor.x = (kb->box.cart[container_id].container) & 0xFFFF;
    kb->cursor.y = (kb->box.cart[container_id].container >> 16) & 0xFFFF;
}

/* ========== 查询坐标 ========== */
void browser_query_coord(BrowserKernel *kb, unsigned int x, unsigned int y, unsigned int z) {
    printf("[查询] 坐标(%u,%u,%u)\n", x, y, z);
    unsigned char p[12];
    memcpy(p,&x,4); memcpy(p+4,&y,4); memcpy(p+8,&z,4);
    GttxHeader hdr = build_gttx_header(kb->container_id, GTTX_OP_QUERY, 12);
    browser_send_frame(kb, &hdr, p);
}

/* ========== 断开 ========== */
void browser_disconnect(BrowserKernel *kb) {
    GttxHeader hdr = build_gttx_header(kb->container_id, GTTX_OP_DISCONNECT, 0);
    browser_send_frame(kb, &hdr, NULL);
    kb->connected = 0;
}

/* ========== 状态打印 ========== */
void browser_status(BrowserKernel *kb) {
    printf("\n========== 浏览器内核状态 ==========\n");
    printf("协议: gttx://%s\n", kb->domain);
    printf("光标: (%u,%u,%u)\n", kb->cursor.x, kb->cursor.y, kb->cursor.z);
    printf("锚点: 我=%d 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("\n--- mlll_vm ---\n");
    printf("PC=%u 运行=%d 栈指针=%d 裁判=%d\n", kb->vm.pc, kb->vm.running, kb->vm.sp, kb->vm.last_admit);
    printf("R0=0x%08X R1=0x%08X R2=0x%08X R3=0x%08X\n",
        kb->vm.reg[0], kb->vm.reg[1], kb->vm.reg[2], kb->vm.reg[3]);
    printf("\n--- mlll 字车 ---\n");
    printf("已用槽: %d / 254\n", kb->box.used);
    for (int i=0;i<254;i++) {
        if (kb->box.cart[i].occupied) {
            printf("  槽#%d: 0x%08X\n", i, kb->box.cart[i].container);
        }
    }
    printf("\n--- mlil 程序 ---\n");
    printf("表面数: %u\n", kb->program.header.slot_count);
    for (uint32_t i=0;i<kb->program.header.slot_count && i<8;i++) {
        mlil_surface_t *s = &kb->program.surfaces[i];
        const char *opname;
        switch(s->modulus) {
            case MLLL_OP_LOAD:  opname="LOAD"; break;
            case MLLL_OP_STORE: opname="STORE"; break;
            case MLLL_OP_JUDGE: opname="JUDGE"; break;
            case MLLL_OP_MOVE:  opname="MOVE"; break;
            default:            opname="NOP"; break;
        }
        printf("  #%d: %s 面=槽%d 名=R%d 容器=0x%08X\n",
            i, opname, s->face, s->name, s->container);
    }
    if (kb->program.header.slot_count > 8)
        printf("  ... 省略余下 %u 条\n", kb->program.header.slot_count - 8);
}

/* ========== 文件读取 ========== */
static unsigned char* read_file(const char *path, size_t *out_size) {
    FILE *f = fopen(path, "rb");
    if (!f) { printf("[错误] 无法打开: %s (%s)\n", path, strerror(errno)); return NULL; }
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (sz <= 0 || sz > 1048576) { fclose(f); return NULL; }
    unsigned char *buf = malloc(sz);
    if (!buf) { fclose(f); return NULL; }
    fread(buf, 1, sz, f);
    fclose(f);
    *out_size = (size_t)sz;
    return buf;
}

/* ========== fqm 文件扩展名 → 类型 ========== */
static unsigned char ext_to_type(const char *filename) {
    const char *dot = strrchr(filename, '.');
    if (!dot) return 0xFF;
    for (int i=0; g_ext_map[i].ext; i++)
        if (strcasecmp(dot, g_ext_map[i].ext) == 0) return g_ext_map[i].type;
    return 0xFF;
}

/* ========== 生成测试 fqm 文件 ========== */
static void write_test_fqm(const char *path) {
    FILE *f = fopen(path, "wb");
    if (!f) return;
    /* 头: FQMT v1 type=0x01(.loli) coord=0x10 count=4 flags=0 */
    unsigned char hdr[FQM_HEADER_SIZE] = {
        'F','Q','M','T', 0x01, 0x01, 0x00,
        0,0,0,0x10,  /* coord_base */
        0,4,         /* count=4 */
        0,0          /* flags */
    };
    fwrite(hdr, 1, FQM_HEADER_SIZE, f);

    /* 4个槽, 交替极性 (取出/存入/裁判/取出) */
    for (int i=0;i<4;i++) {
        unsigned char s[FQM_SLOT_SIZE] = { 0 };
        s[0] = i;                          /* slot_id */
        s[1] = 0x00; s[2] = 0x00;
        s[3] = (i+1)*0x11; s[4] = (i+1)*0x22; /* container value */
        s[5] = i % 3;                      /* polarity: 0=取出 1=存入 2=裁判 */
        s[6] = (i+1)*50;                   /* sovereignty */
        s[7] = 100+i*20;                   /* value */
        s[8] = i;                          /* level */
        fwrite(s, 1, FQM_SLOT_SIZE, f);
    }
    fclose(f);
    printf("  测试文件: %s (4槽, 取出/存入/裁判/取出)\n", path);
}

/* ========== main ========== */
int main(int argc, char *argv[]) {
    printf("意交感息（isi）独立浏览器内核 · mlll_vm 驱动\n");
    printf("满全法锚点: 我=%d 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("集装箱: mlll 254+2\n身份: 刘楚恬和我\n\n");

    BrowserKernel browser;
    browser_init(&browser, "hieyair");

    const char *filepath = NULL;
    for (int i=1;i<argc;i++) {
        if (strcmp(argv[i],"--test")==0) write_test_fqm("/tmp/test.loli");
        else if (argv[i][0]!='-') filepath = argv[i];
    }

    if (filepath) {
        unsigned char ft = ext_to_type(filepath);
        printf("[打开] %s (类型0x%02X)\n", filepath, ft);
        size_t sz;
        unsigned char *data = read_file(filepath, &sz);
        if (data) {
            browser_load_fqm(&browser, data, sz);
            free(data);
        }
    } else if (!filepath) {
        /* 空运行: 手工注入锚点到字车 */
        printf("\n[演示] 锚点已预装\n");
    }

    /* 执行已加载的 mlll 程序 (第一步) */
    if (browser.vm.program && browser.program.header.slot_count > 0 && browser.vm.running) {
        printf("\n[执行] 驱动 mlll_vm 执行程序...\n");
        int safety = 0;
        while (browser.vm.running && safety < 300) {
            int pc_before = browser.vm.pc;
            int ret = mlll_vm_step(&browser.vm);
            printf("  [#%d] mlll_vm: R0=0x%08X R1=0x%08X 裁判=%d\n",
                pc_before, browser.vm.reg[0], browser.vm.reg[1], browser.vm.last_admit);
            if (ret != 0) { printf("  vm 已停止 (ret=%d)\n", ret); break; }
            safety++;
        }
    }

    browser_request_container(&browser, 0);
    browser_status(&browser);
    browser_disconnect(&browser);

    printf("\n========== 完成 ==========\n");
    return 0;
}
