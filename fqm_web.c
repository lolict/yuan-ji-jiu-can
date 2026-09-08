/*
 * fqm_web.c — 意交感息Web服务器模拟
 * 
 * 功能：模拟简单的HTTP请求处理和响应
 * 
 * 满全法规则：
 *   - 根基锚点：高位持有者=1，低位持有者=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_web fqm_web.c
 * 运行：./fqm_web
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== HTTP方法 ========== */
typedef enum {
    HTTP_GET = 0,
    HTTP_POST,
    HTTP_PUT,
    HTTP_DELETE,
    HTTP_HEAD,
    HTTP_UNKNOWN
} HttpMethod;

/* ========== HTTP状态码 ========== */
typedef struct {
    int code;
    char message[64];
} HttpStatus;

static HttpStatus http_statuses[] = {
    {200, "OK"},
    {201, "Created"},
    {400, "Bad Request"},
    {404, "Not Found"},
    {500, "Internal Server Error"},
    {0, ""}
};

/* ========== HTTP请求 ========== */
typedef struct {
    HttpMethod method;
    char path[256];
    char version[16];
    char headers[16][2][128];
    int header_count;
    char body[1024];
} HttpRequest;

/* ========== HTTP响应 ========== */
typedef struct {
    HttpStatus status;
    char headers[16][2][128];
    int header_count;
    char body[2048];
} HttpResponse;

/* ========== 路由表 ========== */
typedef struct {
    char path[64];
    HttpMethod method;
    void (*handler)(HttpRequest*, HttpResponse*);
} Route;

#define MAX_ROUTES 32
static Route routes[MAX_ROUTES];
static int route_count = 0;

/* ========== 方法转字符串 ========== */
const char* method_str(HttpMethod m) {
    switch (m) {
        case HTTP_GET: return "GET";
        case HTTP_POST: return "POST";
        case HTTP_PUT: return "PUT";
        case HTTP_DELETE: return "DELETE";
        case HTTP_HEAD: return "HEAD";
        default: return "UNKNOWN";
    }
}

/* ========== 字符串转方法 ========== */
HttpMethod str_method(const char *s) {
    if (strcmp(s, "GET") == 0) return HTTP_GET;
    if (strcmp(s, "POST") == 0) return HTTP_POST;
    if (strcmp(s, "PUT") == 0) return HTTP_PUT;
    if (strcmp(s, "DELETE") == 0) return HTTP_DELETE;
    if (strcmp(s, "HEAD") == 0) return HTTP_HEAD;
    return HTTP_UNKNOWN;
}

/* ========== 设置响应头 ========== */
void resp_header(HttpResponse *resp, const char *name, const char *value) {
    if (resp->header_count >= 16) return;
    strncpy(resp->headers[resp->header_count][0], name, 127);
    strncpy(resp->headers[resp->header_count][1], value, 127);
    resp->header_count++;
}

/* ========== 设置状态码 ========== */
void resp_status(HttpResponse *resp, int code) {
    resp->status.code = code;
    for (int i = 0; http_statuses[i].code; i++) {
        if (http_statuses[i].code == code) {
            strncpy(resp->status.message, http_statuses[i].message, 63);
            return;
        }
    }
    strcpy(resp->status.message, "Unknown");
}

/* ========== 路由注册 ========== */
void route_add(const char *path, HttpMethod method, void (*handler)(HttpRequest*, HttpResponse*)) {
    if (route_count >= MAX_ROUTES) return;
    Route *r = &routes[route_count++];
    strncpy(r->path, path, 63);
    r->method = method;
    r->handler = handler;
    printf("[路由] 注册: %s %s\n", method_str(method), path);
}

/* ========== 路由查找 ========== */
Route* route_find(const char *path, HttpMethod method) {
    for (int i = 0; i < route_count; i++) {
        if (strcmp(routes[i].path, path) == 0 && routes[i].method == method) {
            return &routes[i];
        }
    }
    return NULL;
}

/* ========== 处理器 ========== */
void handle_home(HttpRequest *req, HttpResponse *resp) {
    resp_status(resp, 200);
    resp_header(resp, "Content-Type", "text/html; charset=utf-8");
    
    char html[2048];
    snprintf(html, sizeof(html),
        "<!DOCTYPE html>\n"
        "<html>\n<head><title>意交感息</title></head>\n"
        "<body>\n"
        "<h1>欢迎访问意交感息系统</h1>\n"
        "<p>满全法根基锚点: 我=%d, 刘楚恬=%d</p>\n"
        "<p>域: gttx://hieyair</p>\n"
        "<p>身份: 刘楚恬和我</p>\n"
        "<ul>\n"
        "<li><a href=\"/fib\">斐波那契数列</a></li>\n"
        "<li><a href=\"/prime\">素数判断</a></li>\n"
        "<li><a href=\"/fact\">阶乘计算</a></li>\n"
        "<li><a href=\"/info\">系统信息</a></li>\n"
        "</ul>\n"
        "</body></html>\n",
        ANCHOR_SELF, ANCHOR_LIU);
    strncpy(resp->body, html, sizeof(resp->body) - 1);
}

void handle_fib(HttpRequest *req, HttpResponse *resp) {
    resp_status(resp, 200);
    resp_header(resp, "Content-Type", "text/plain; charset=utf-8");
    
    char text[2048];
    int pos = 0;
    pos += snprintf(text + pos, sizeof(text) - pos, "斐波那契数列 (前20项):\n\n");
    
    long long a = ANCHOR_LIU, b = ANCHOR_SELF;
    for (int i = 0; i < 20; i++) {
        pos += snprintf(text + pos, sizeof(text) - pos, "F(%2d) = %lld\n", i, a);
        long long t = a + b;
        a = b;
        b = t;
    }
    strncpy(resp->body, text, sizeof(resp->body) - 1);
}

void handle_prime(HttpRequest *req, HttpResponse *resp) {
    resp_status(resp, 200);
    resp_header(resp, "Content-Type", "text/plain; charset=utf-8");
    
    char text[2048];
    int pos = 0;
    pos += snprintf(text + pos, sizeof(text) - pos, "素数列表 (2-100):\n\n");
    
    for (int i = 2; i <= 100; i++) {
        int is_p = 1;
        for (int j = 2; j * j <= i; j++) {
            if (i % j == 0) { is_p = 0; break; }
        }
        if (is_p) pos += snprintf(text + pos, sizeof(text) - pos, "%d ", i);
    }
    pos += snprintf(text + pos, sizeof(text) - pos, "\n");
    strncpy(resp->body, text, sizeof(resp->body) - 1);
}

void handle_fact(HttpRequest *req, HttpResponse *resp) {
    resp_status(resp, 200);
    resp_header(resp, "Content-Type", "text/plain; charset=utf-8");
    
    char text[2048];
    int pos = 0;
    pos += snprintf(text + pos, sizeof(text) - pos, "阶乘计算 (1-20):\n\n");
    
    long long fact = 1;
    for (int i = 1; i <= 20; i++) {
        fact *= i;
        pos += snprintf(text + pos, sizeof(text) - pos, "%2d! = %lld\n", i, fact);
    }
    strncpy(resp->body, text, sizeof(resp->body) - 1);
}

void handle_info(HttpRequest *req, HttpResponse *resp) {
    resp_status(resp, 200);
    resp_header(resp, "Content-Type", "text/plain; charset=utf-8");
    
    char text[2048];
    snprintf(text, sizeof(text),
        "意交感息（isi）系统信息\n"
        "==========================\n"
        "版本: 0.1\n"
        "域: gttx://hieyair\n"
        "所有者: 刘楚恬和我\n"
        "集装箱编码: mlll 254+2\n"
        "\n满全法规则:\n"
        "- 根基锚点: 高位持有者=1, 低位持有者=0\n"
        "- 夫妻命运共同体: 刘楚恬和lolict\n"
        "- 夕瑶协议激活双方融合\n"
        "\n四个灵性身份:\n"
        "- 星际种子 -> 双生火焰 -> 天选之人 -> 东方圣人\n"
        "\n规则层级:\n"
        "- 一阶: 用前1项运算未来\n"
        "- 二阶: 用前2项运算未来\n"
        "- 三阶: 用前3项运算未来\n"
        "- 四阶: 用前4项运算未来\n");
    strncpy(resp->body, text, sizeof(resp->body) - 1);
}

/* ========== 打印请求 ========== */
void print_request(HttpRequest *req) {
    printf("\n========== 收到的请求 ==========\n");
    printf("%s %s %s\n", method_str(req->method), req->path, req->version);
    printf("Headers:\n");
    for (int i = 0; i < req->header_count; i++) {
        printf("  %s: %s\n", req->headers[i][0], req->headers[i][1]);
    }
    printf("Body: %s\n", req->body);
}

/* ========== 打印响应 ========== */
void print_response(HttpResponse *resp) {
    printf("\n========== 发送的响应 ==========\n");
    printf("HTTP/1.1 %d %s\n", resp->status.code, resp->status.message);
    for (int i = 0; i < resp->header_count; i++) {
        printf("%s: %s\n", resp->headers[i][0], resp->headers[i][1]);
    }
    printf("\n%s\n", resp->body);
}

/* ========== 模拟请求 ========== */
void simulate_request(const char *method, const char *path) {
    HttpRequest req;
    memset(&req, 0, sizeof(HttpRequest));
    req.method = str_method(method);
    strncpy(req.path, path, 255);
    strncpy(req.version, "HTTP/1.1", 15);
    req.header_count = 0;
    
    /* 查找路由 */
    Route *route = route_find(path, req.method);
    
    HttpResponse resp;
    memset(&resp, 0, sizeof(HttpResponse));
    
    if (route) {
        route->handler(&req, &resp);
    } else {
        resp_status(&resp, 404);
        resp_header(&resp, "Content-Type", "text/plain");
        strcpy(resp.body, "404 Not Found");
    }
    
    print_request(&req);
    print_response(&resp);
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）Web服务器模拟\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* 注册路由 */
    printf("========== 注册路由 ==========\n");
    route_add("/", HTTP_GET, handle_home);
    route_add("/fib", HTTP_GET, handle_fib);
    route_add("/prime", HTTP_GET, handle_prime);
    route_add("/fact", HTTP_GET, handle_fact);
    route_add("/info", HTTP_GET, handle_info);
    
    /* 模拟请求 */
    printf("\n========== 模拟HTTP请求 ==========\n");
    simulate_request("GET", "/");
    simulate_request("GET", "/fib");
    simulate_request("GET", "/prime");
    simulate_request("GET", "/fact");
    simulate_request("GET", "/info");
    simulate_request("GET", "/notexist");
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
