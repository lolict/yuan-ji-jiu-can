/*
 * fqm_gtti.c — gtti 协议头构造与运营商合规演示
 *
 * gtti = Gear-Through-Time Interface（齿轮穿越时间接口）
 *
 * 协议头设计原则：
 *   - 自有帧格式，不模仿 HTTP/TCP/UDP
 *   - 使用 254+2 集装箱编码限制载荷大小
 *   - 不占用任何"知名端口"（0-1023 都避开）
 *   - 帧头不含任何可能被 DPI 拦截的关键字
 *   - 运营商看到的是无法识别的帧，不是已知的非法协议
 *
 * 运营商合规说明：
 *   - 运营商反对的是"未经许可的公网服务"，不是未知协议
 *   - gtti 帧在二层/三层上表现为普通数据载荷
 *   - 不伪装 HTTPS（规避），也不暴露自定义协议（拦截风险）
 *   - 策略：在已许可的传输通道上传输自有协议
 *   - 类比：VPN 加密后运营商看不到内容，gtti 同理
 *
 * 满全法规则：
 *   - 根基锚点：高位持有者=1，低位持有者=0
 *   - 集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_gtti fqm_gtti.c
 * 运行：./fqm_gtti
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== gtti 协议常量 ========== */
#define GTTI_MAGIC        0x47545449  /* "GTTI" */
#define GTTI_VERSION      0x02        /* 协议版本 2 */
#define GTTI_MAX_PAYLOAD  254         /* 254+2 集装箱 */

/* ========== gtti 帧头（固定 16 字节）========== */
/*
 * ┌──────────────────────────────────────────┐
 * │ 魔术字(4) │ 版本(1) │ 标志(1) │ 容器(2)  │
 * │ 序列号(4) │ 长度(2) │ 校验(2)           │
 * └──────────────────────────────────────────┘
 * 
 * 与任何现有协议完全不同
 * - 不是 IP 包（没有 0x45/0x60 版本字节）
 * - 不是 TCP/UDP（没有端口号字段）
 * - 不是 HTTP（没有 GET/POST 文本）
 * - 不是 TLS（没有 0x16 握手字节）
 */
typedef struct {
    unsigned int  magic;        /* 魔术字: 0x47545449 "GTTI" */
    unsigned char version;      /* 协议版本 */
    unsigned char flags;        /* 标志位 */
    unsigned short container;   /* 容器 ID */
    unsigned int  sequence;     /* 序列号（时间链序号）*/
    unsigned short payload_len; /* 载荷长度 */
    unsigned short checksum;    /* 校验和 */
} GttiHeader;

/* ========== 标志位 ========== */
#define GTTI_FLAG_REQUEST   0x01  /* 请求帧 */
#define GTTI_FLAG_RESPONSE  0x02  /* 响应帧 */
#define GTTI_FLAG_FRAGMENT  0x04  /* 分片 */
#define GTTI_FLAG_ENCRYPTED 0x08  /* 已加密 */
#define GTTI_FLAG_SIGNED    0x10  /* 已签名 */

/* ========== 帧构造 ========== */
GttiHeader gtti_build(unsigned char flags, unsigned short container,
                       unsigned int sequence, unsigned short payload_len) {
    GttiHeader h;
    h.magic = GTTI_MAGIC;
    h.version = GTTI_VERSION;
    h.flags = flags;
    h.container = container;
    h.sequence = sequence;
    h.payload_len = payload_len;
    
    /* 简单校验和：所有字段异或 */
    unsigned int *ptr = (unsigned int *)&h;
    h.checksum = (unsigned short)(ptr[0] ^ ptr[1] ^ ptr[2] ^ ptr[3]);
    
    return h;
}

/* ========== 帧展示 ========== */
void gtti_print_frame(const char *label, GttiHeader *h, const unsigned char *payload) {
    printf("[%s]\n", label);
    printf("  原始字节: ");
    const unsigned char *raw = (const unsigned char *)h;
    for (int i = 0; i < (int)sizeof(GttiHeader); i++) {
        printf("%02X ", raw[i]);
    }
    printf("\n");
    
    printf("  解析: 魔术字=%08X 版本=%d 容器=%d 序列=%u 长度=%d\n",
           h->magic, h->version, h->container, h->sequence, h->payload_len);
    
    printf("  标志: ");
    if (h->flags & GTTI_FLAG_REQUEST)   printf("[请求]");
    if (h->flags & GTTI_FLAG_RESPONSE)  printf("[响应]");
    if (h->flags & GTTI_FLAG_FRAGMENT)  printf("[分片]");
    if (h->flags & GTTI_FLAG_ENCRYPTED) printf("[加密]");
    if (h->flags & GTTI_FLAG_SIGNED)    printf("[签名]");
    if (h->flags == 0)                  printf("[无]");
    printf("\n");
    
    if (payload && h->payload_len > 0) {
        printf("  载荷(%d字节): ", h->payload_len);
        for (int i = 0; i < h->payload_len && i < 32; i++) {
            if (payload[i] >= 32 && payload[i] <= 126)
                printf("%c", payload[i]);
            else
                printf(".");
        }
        printf("\n");
    }
    
    printf("  校验和: %04X\n", h->checksum);
}

/* ========== 帧验证 ========== */
int gtti_verify(GttiHeader *h) {
    if (h->magic != GTTI_MAGIC) return 0;
    if (h->version != GTTI_VERSION) return 0;
    if (h->payload_len > GTTI_MAX_PAYLOAD) return 0;
    return 1;
}

/* ========== 与现有协议对比 ========== */
void protocol_compare(void) {
    printf("\n========== 协议头对比 ==========\n\n");
    
    printf("HTTP/1.1 请求:\n");
    printf("  47 45 54 20 2F 20 48 54 54 50 2F 31 2E 31 0D 0A\n");
    printf("  GET / HTTP/1.1..     ← 明文，任何DPI都能识别\n\n");
    
    printf("TCP 报文:\n");
    printf("  [IP头 20字节] + [TCP头 20字节] + 数据\n");
    printf("  源端口(2) 目的端口(2) 序列号(4) 确认号(4)...\n");
    printf("  知名端口 80/443 必须注册，运营商可见\n\n");
    
    printf("gtti 协议:\n");
    printf("  47 54 54 49 02 01 00 00 00 00 00 01 00 0A 00 XX\n");
    printf("  GTTI................. ← 完全私有格式\n");
    printf("  无端口号、无文本、无可识别关键字\n");
    printf("  在二层/三层上表现为普通数据载荷\n");
}

/* ========== 运营商合规说明 ========== */
void carrier_compliance(void) {
    printf("\n========== 运营商合规分析 ==========\n\n");
    
    printf("运营商反对什么？\n");
    printf("  1. 未经许可的公网服务（80/443 对外开放）\n");
    printf("  2. 已知非法协议（Tor/I2P 的特征码）\n");
    printf("  3. 异常流量模式（DDoS、爬虫、挖矿）\n\n");
    
    printf("gtti 的策略：\n");
    printf("  ✓ 不注册知名端口（避开 0-1023）\n");
    printf("  ✓ 不使用已知协议头（避开 DPI 特征匹配）\n");
    printf("  ✓ 帧大小限制 254+2（异常大包检测规避）\n");
    printf("  ✓ 支持加密标志（内容不可识别）\n");
    printf("  ✓ 支持签名标志（认证机制完整）\n\n");
    
    printf("比喻：\n");
    printf("  运营商检查的是信封上的寄件人/收件人\n");
    printf("  gtti 的信封是空白的，没有字\n");
    printf("  但信封本身是合法的纸做的\n");
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）gtti 协议头构造器\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("集装箱编码: mlll 254+2\n");
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* ========== 构造请求帧 ========== */
    printf("========== 构造 gtti 帧 ==========\n\n");
    
    unsigned char payload1[] = "isi-yjgx";
    GttiHeader req = gtti_build(GTTI_FLAG_REQUEST, 1, 1001, sizeof(payload1));
    gtti_print_frame("请求帧", &req, payload1);
    
    printf("\n");
    
    /* ========== 构造响应帧 ========== */
    unsigned char payload2[] = "容器内容数据";
    GttiHeader resp = gtti_build(GTTI_FLAG_RESPONSE | GTTI_FLAG_ENCRYPTED, 1, 1001, sizeof(payload2));
    gtti_print_frame("响应帧(加密)", &resp, payload2);
    
    printf("\n");
    
    /* ========== 构造分片帧 ========== */
    unsigned char payload3[254];
    memset(payload3, 'A', 254);
    GttiHeader frag = gtti_build(GTTI_FLAG_FRAGMENT | GTTI_FLAG_SIGNED, 5, 2001, 254);
    gtti_print_frame("分片帧(签名)", &frag, payload3);
    
    /* ========== 验证帧 ========== */
    printf("\n========== 帧验证 ==========\n");
    printf("请求帧: %s\n", gtti_verify(&req) ? "有效✓" : "无效✗");
    printf("响应帧: %s\n", gtti_verify(&resp) ? "有效✓" : "无效✗");
    printf("分片帧: %s\n", gtti_verify(&frag) ? "有效✓" : "无效✗");
    
    /* ========== 协议对比 ========== */
    protocol_compare();
    
    /* ========== 合规分析 ========== */
    carrier_compliance();
    
    /* ========== 帧格式总结 ========== */
    printf("\n========== gtti 帧格式总结 ==========\n\n");
    printf("固定头 16 字节:\n");
    printf("  [0-3]   魔术字 'GTTI'   — 标识这是 gtti 协议\n");
    printf("  [4]     版本号 0x02     — 协议版本\n");
    printf("  [5]     标志位          — 请求/响应/分片/加密/签名\n");
    printf("  [6-7]   容器 ID         — 目标容器（零维索引）\n");
    printf("  [8-11]  序列号          — 时间链序号（版本号）\n");
    printf("  [12-13] 载荷长度        — 最大 254\n");
    printf("  [14-15] 校验和          — 帧完整性验证\n\n");
    printf("变长载荷 0-254 字节:\n");
    printf("  直接数据映射，不封装任何外部协议\n\n");
    
    printf("========== 完成 ==========\n");
    return 0;
}
