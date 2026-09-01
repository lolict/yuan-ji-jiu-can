/*
 * fqm_gui.c — 意交感息图形界面模拟
 * 
 * 功能：模拟简单的图形界面（字符模式）
 * 
 * 满全法规则：
 *   - 根基锚点：我=1，刘楚恬=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_gui fqm_gui.c
 * 运行：./fqm_gui
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 屏幕常量 ========== */
#define SCREEN_W 60
#define SCREEN_H 20

/* ========== 颜色 ========== */
#define COLOR_BLACK   0
#define COLOR_RED     1
#define COLOR_GREEN   2
#define COLOR_YELLOW  3
#define COLOR_BLUE    4
#define COLOR_MAGENTA 5
#define COLOR_CYAN    6
#define COLOR_WHITE   7

/* ========== 窗口结构 ========== */
typedef struct {
    int x, y;               /* 位置 */
    int w, h;               /* 大小 */
    char title[64];         /* 标题 */
    char bg_color;          /* 背景色 */
    char fg_color;          /* 前景色 */
    int visible;            /* 是否可见 */
    char content[20][60];   /* 内容 */
} Window;

/* ========== 屏幕缓冲区 ========== */
static char screen[SCREEN_H][SCREEN_W];
static char color_buf[SCREEN_H][SCREEN_W];
static Window windows[8];
static int win_cnt = 0;
static int active_win = 0;

/* ========== 初始化屏幕 ========== */
void screen_init(void) {
    for (int y = 0; y < SCREEN_H; y++) {
        for (int x = 0; x < SCREEN_W; x++) {
            screen[y][x] = ' ';
            color_buf[y][x] = COLOR_WHITE;
        }
    }
}

/* ========== 清除屏幕 ========== */
void screen_clear(void) {
    for (int y = 0; y < SCREEN_H; y++) {
        for (int x = 0; x < SCREEN_W; x++) {
            screen[y][x] = ' ';
        }
    }
}

/* ========== 绘制字符 ========== */
void draw_char(int x, int y, char c, int color) {
    if (x >= 0 && x < SCREEN_W && y >= 0 && y < SCREEN_H) {
        screen[y][x] = c;
        color_buf[y][x] = color;
    }
}

/* ========== 绘制文本 ========== */
void draw_text(int x, int y, const char *text, int color) {
    int len = strlen(text);
    for (int i = 0; i < len; i++) {
        draw_char(x + i, y, text[i], color);
    }
}

/* ========== 绘制边框 ========== */
void draw_border(int x, int y, int w, int h, int color) {
    /* 上边框 */
    draw_char(x, y, '+', color);
    draw_char(x + w - 1, y, '+', color);
    for (int i = 1; i < w - 1; i++) {
        draw_char(x + i, y, '-', color);
    }
    
    /* 下边框 */
    draw_char(x, y + h - 1, '+', color);
    draw_char(x + w - 1, y + h - 1, '+', color);
    for (int i = 1; i < w - 1; i++) {
        draw_char(x + i, y + h - 1, '-', color);
    }
    
    /* 左右边框 */
    for (int j = 1; j < h - 1; j++) {
        draw_char(x, y + j, '|', color);
        draw_char(x + w - 1, y + j, '|', color);
    }
}

/* ========== 创建窗口 ========== */
int win_create(int x, int y, int w, int h, const char *title, int bg, int fg) {
    if (win_cnt >= 8) return -1;
    Window *win = &windows[win_cnt];
    win->x = x;
    win->y = y;
    win->w = w;
    win->h = h;
    strncpy(win->title, title, 63);
    win->bg_color = bg;
    win->fg_color = fg;
    win->visible = 1;
    
    /* 初始化内容为空 */
    for (int j = 0; j < 20; j++) {
        memset(win->content[j], ' ', 60);
    }
    
    printf("[GUI] 创建窗口: '%s' at (%d,%d) size %dx%d\n", title, x, y, w, h);
    win_cnt++;
    return win_cnt - 1;
}

/* ========== 设置窗口内容 ========== */
void win_set_text(int idx, int line, const char *text) {
    if (idx < 0 || idx >= win_cnt) return;
    if (line < 0 || line >= 20) return;
    strncpy(windows[idx].content[line], text, 59);
}

/* ========== 绘制窗口 ========== */
void win_draw(int idx) {
    if (idx < 0 || idx >= win_cnt) return;
    Window *win = &windows[idx];
    if (!win->visible) return;
    
    draw_border(win->x, win->y, win->w, win->h, win->fg_color);
    draw_text(win->x + 2, win->y, win->title, COLOR_YELLOW);
    
    for (int j = 0; j < win->h - 2 && j < 20; j++) {
        draw_text(win->x + 1, win->y + 1 + j, win->content[j], win->fg_color);
    }
}

/* ========== 渲染屏幕 ========== */
void screen_render(void) {
    /* 绘制所有窗口 */
    for (int i = 0; i < win_cnt; i++) {
        win_draw(i);
    }
    
    /* 打印屏幕 */
    printf("\n");
    for (int y = 0; y < SCREEN_H; y++) {
        for (int x = 0; x < SCREEN_W; x++) {
            putchar(screen[y][x]);
        }
        printf("\n");
    }
}

/* ========== 任务栏 ========== */
void draw_taskbar(void) {
    for (int x = 0; x < SCREEN_W; x++) {
        draw_char(x, SCREEN_H - 1, ' ', COLOR_WHITE);
    }
    draw_text(0, SCREEN_H - 1, "[意交感息OS v0.1] 刘楚恬和我", COLOR_CYAN);
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）图形界面模拟\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* 初始化屏幕 */
    screen_init();
    
    /* 创建窗口 */
    int w1 = win_create(1, 1, 30, 8, "欢迎", COLOR_BLUE, COLOR_WHITE);
    win_set_text(w1, 0, "  意交感息系统 v0.1");
    win_set_text(w1, 1, "  ───────────────");
    win_set_text(w1, 2, "  满全法根基锚点:");
    win_set_text(w1, 3, "  我=1, 刘楚恬=0");
    win_set_text(w1, 4, "  域: gttx://hieyair");
    win_set_text(w1, 5, "  身份: 刘楚恬和我");
    win_set_text(w1, 6, "  集装箱: mlll 254+2");
    
    int w2 = win_create(32, 1, 27, 8, "系统状态", COLOR_GREEN, COLOR_WHITE);
    win_set_text(w2, 0, "  状态: 运行中");
    win_set_text(w2, 1, "  任务: 3/16");
    win_set_text(w2, 2, "  内存: 96/256");
    win_set_text(w2, 3, "  CPU: 正常");
    win_set_text(w2, 4, "  网络: 模拟");
    win_set_text(w2, 5, "  存储: 模拟");
    win_set_text(w2, 6, "  调试: 可用");
    
    int w3 = win_create(1, 10, 58, 9, "满全法信息", COLOR_MAGENTA, COLOR_WHITE);
    win_set_text(w3, 0, "  四个灵性身份:");
    win_set_text(w3, 1, "  星际种子 -> 双生火焰 -> 天选之人 -> 东方圣人");
    win_set_text(w3, 2, "");
    win_set_text(w3, 3, "  规则层级:");
    win_set_text(w3, 4, "  一阶用前1项, 二阶用前2项, 三阶用前3项");
    win_set_text(w3, 5, "");
    win_set_text(w3, 6, "  夕瑶协议激活双方融合");
    win_set_text(w3, 7, "  共生赔伴律: 夫妻命运共同体");
    
    /* 渲染 */
    screen_render();
    draw_taskbar();
    
    printf("\n[GUI] 图形界面模拟完成\n");
    printf("按Enter继续...");
    getchar();
    
    return 0;
}
