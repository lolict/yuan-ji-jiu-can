/*
 * fqm_crypto.c — 意交感息加密引擎
 * 
 * 功能：基于规则层级的字符串加密/解密
 * 规则：用前N个字符对后面的字符进行运算加密
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
 * 编译：clang -o fqm_crypto fqm_crypto.c
 * 运行：./fqm_crypto
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 加密规则 ========== */

/* 规则1：用前1个字符加密后1个（流加密） */
void encrypt_rule1(const char *input, char *output, int len) {
    if (len == 0) return;
    output[0] = input[0] ^ ANCHOR_SELF;
    for (int i = 1; i < len; i++) {
        output[i] = input[i] ^ output[i-1];  /* 用前一个加密后的字符 */
    }
}

/* 规则2：用前2个字符加密后1个 */
void encrypt_rule2(const char *input, char *output, int len) {
    if (len == 0) return;
    output[0] = input[0] ^ ANCHOR_SELF;
    if (len > 1) output[1] = input[1] ^ ANCHOR_LIU;
    for (int i = 2; i < len; i++) {
        output[i] = input[i] ^ output[i-1] ^ output[i-2];
    }
}

/* 规则3：用前3个字符加密后1个 */
void encrypt_rule3(const char *input, char *output, int len) {
    if (len == 0) return;
    output[0] = input[0] ^ ANCHOR_SELF;
    if (len > 1) output[1] = input[1] ^ ANCHOR_LIU;
    if (len > 2) output[2] = input[2] ^ ANCHOR_SELF;
    for (int i = 3; i < len; i++) {
        output[i] = input[i] ^ output[i-1] ^ output[i-2] ^ output[i-3];
    }
}

/* ========== 解密规则（逆向运算） ========== */
void decrypt_rule1(const char *input, char *output, int len) {
    if (len == 0) return;
    output[0] = input[0] ^ ANCHOR_SELF;
    for (int i = 1; i < len; i++) {
        output[i] = input[i] ^ input[i-1];
    }
}

void decrypt_rule2(const char *input, char *output, int len) {
    if (len == 0) return;
    output[0] = input[0] ^ ANCHOR_SELF;
    if (len > 1) output[1] = input[1] ^ ANCHOR_LIU;
    for (int i = 2; i < len; i++) {
        output[i] = input[i] ^ input[i-1] ^ input[i-2];
    }
}

void decrypt_rule3(const char *input, char *output, int len) {
    if (len == 0) return;
    output[0] = input[0] ^ ANCHOR_SELF;
    if (len > 1) output[1] = input[1] ^ ANCHOR_LIU;
    if (len > 2) output[2] = input[2] ^ ANCHOR_SELF;
    for (int i = 3; i < len; i++) {
        output[i] = input[i] ^ input[i-1] ^ input[i-2] ^ input[i-3];
    }
}

/* ========== 打印十六进制 ========== */
void print_hex(const char *label, const char *data, int len) {
    printf("%-15s: ", label);
    for (int i = 0; i < len; i++) {
        printf("%02X ", (unsigned char)data[i]);
    }
    printf("\n");
}

/* ========== 打印可打印字符 ========== */
void print_safe(const char *label, const char *data, int len) {
    printf("%-15s: ", label);
    for (int i = 0; i < len; i++) {
        char c = data[i];
        if (c >= 32 && c < 127) printf("%c", c);
        else printf("·");
    }
    printf("\n");
}

/* ========== 主函数 ========== */
int main(void) {
    char plaintext[256];
    char encrypted[256];
    char decrypted[256];
    
    printf("意交感息（isi）加密引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* 输入明文 */
    strcpy(plaintext, "刘楚恬和我");
    int len = strlen(plaintext);
    
    printf("明文: %s (长度=%d)\n\n", plaintext, len);
    
    print_hex("明文(HEX)", plaintext, len);
    print_safe("明文(可打印)", plaintext, len);
    printf("\n");
    
    /* ========== 规则1加密 ========== */
    printf("========== 一阶加密（用前1个字符） ==========\n");
    encrypt_rule1(plaintext, encrypted, len);
    print_hex("一阶加密(HEX)", encrypted, len);
    print_safe("一阶加密(可打印)", encrypted, len);
    
    decrypt_rule1(encrypted, decrypted, len);
    print_safe("一阶解密(可打印)", decrypted, len);
    printf("\n");
    
    /* ========== 规则2加密 ========== */
    printf("========== 二阶加密（用前2个字符） ==========\n");
    encrypt_rule2(plaintext, encrypted, len);
    print_hex("二阶加密(HEX)", encrypted, len);
    print_safe("二阶加密(可打印)", encrypted, len);
    
    decrypt_rule2(encrypted, decrypted, len);
    print_safe("二阶解密(可打印)", decrypted, len);
    printf("\n");
    
    /* ========== 规则3加密 ========== */
    printf("========== 三阶加密（用前3个字符） ==========\n");
    encrypt_rule3(plaintext, encrypted, len);
    print_hex("三阶加密(HEX)", encrypted, len);
    print_safe("三阶加密(可打印)", encrypted, len);
    
    decrypt_rule3(encrypted, decrypted, len);
    print_safe("三阶解密(可打印)", decrypted, len);
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
