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

/* ========== 多层密钥派生链 ========== */

/* 密钥容器：三层派生后的种子 */
typedef struct {
    unsigned char seed;        /* 最终派生种子（用于加密首字节） */
    unsigned char chain[256];  /* 派生链（每字节的额外掩码） */
    int chain_len;
} DerivedKey;

/* 派生规则1：单层前馈——用前一个派生值影响当前 */
static unsigned char derive_rule1(unsigned char prev, unsigned char input) {
    return (input ^ prev) + ANCHOR_SELF;
}

/* 派生规则2：双层前馈——用前两个派生值影响当前 */
static unsigned char derive_rule2(unsigned char prev1, unsigned char prev2, unsigned char input) {
    return (input ^ prev1 ^ prev2) + ANCHOR_LIU + ANCHOR_SELF;
}

/* 派生规则3：三层前馈——用前三个派生值影响当前 */
static unsigned char derive_rule3(unsigned char p1, unsigned char p2, unsigned char p3, unsigned char input) {
    return (input ^ p1 ^ p2 ^ p3) + ANCHOR_SELF + ANCHOR_SELF + ANCHOR_SELF;
}

/* 三层密钥派生：
 * 第1轮：passphrase 经过 rule1 流转，产出 chain_a
 * 第2轮：chain_a 经过 rule2 流转，产出 chain_b
 * 第3轮：chain_b 经过 rule3 流转，产出最终 derived_key.chain
 */
void derive_key(const char *passphrase, int pass_len, DerivedKey *key) {
    if (pass_len == 0) {
        key->seed = ANCHOR_SELF;
        key->chain_len = 0;
        return;
    }

    unsigned char buf_a[256], buf_b[256];

    /* 第1轮：rule1 前馈链 */
    buf_a[0] = derive_rule1(ANCHOR_SELF, (unsigned char)passphrase[0]);
    for (int i = 1; i < pass_len; i++) {
        buf_a[i] = derive_rule1(buf_a[i-1], (unsigned char)passphrase[i]);
    }

    /* 第2轮：rule2 双层前馈 */
    buf_b[0] = derive_rule2(buf_a[0], ANCHOR_SELF, buf_a[0]);
    if (pass_len > 1) {
        buf_b[1] = derive_rule2(buf_a[1], buf_a[0], buf_a[1]);
    }
    for (int i = 2; i < pass_len; i++) {
        buf_b[i] = derive_rule2(buf_a[i-1], buf_a[i-2], buf_a[i]);
    }

    /* 第3轮：rule3 三层前馈 → 最终 key */
    unsigned char k0 = derive_rule3(buf_b[0], ANCHOR_SELF, ANCHOR_LIU, buf_b[0]);
    key->chain[0] = k0;
    key->seed = k0;

    unsigned char k1, k2;
    if (pass_len > 1) {
        k1 = derive_rule3(buf_b[1], buf_b[0], ANCHOR_SELF, buf_b[1]);
        key->chain[1] = k1;
    }
    if (pass_len > 2) {
        k2 = derive_rule3(buf_b[2], buf_b[1], buf_b[0], buf_b[2]);
        key->chain[2] = k2;
    }
    for (int i = 3; i < pass_len; i++) {
        key->chain[i] = derive_rule3(buf_b[i-1], buf_b[i-2], buf_b[i-3], buf_b[i]);
    }
    key->chain_len = pass_len;
}

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

/* ========== 流加密（使用派生密钥） ========== */

/* 三层派生流加密：每字节用 chain[i] 异或 + 前一字节密文反馈 */
void encrypt_derived(const char *input, char *output, int len, const DerivedKey *key) {
    if (len == 0) return;
    output[0] = (input[0] ^ key->seed);
    for (int i = 1; i < len; i++) {
        unsigned char mask = key->seed ^ output[i-1];
        if (key->chain_len > 0) {
            mask = key->chain[i % key->chain_len] ^ output[i-1];
        }
        output[i] = input[i] ^ mask;
    }
}

/* 三层派生流解密 */
void decrypt_derived(const char *input, char *output, int len, const DerivedKey *key) {
    if (len == 0) return;
    output[0] = (input[0] ^ key->seed);
    for (int i = 1; i < len; i++) {
        unsigned char mask = key->seed ^ input[i-1];
        if (key->chain_len > 0) {
            mask = key->chain[i % key->chain_len] ^ input[i-1];
        }
        output[i] = input[i] ^ mask;
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
    decrypted[len] = '\0';
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

    /* ========== 三层密钥派生加密 ========== */
    printf("\n========== 三层密钥派生加密 ==========\n");

    const char *pass = "刘楚恬和我";  /* 用明文自身做密码测试 */
    printf("密码: %s\n", pass);

    DerivedKey key;
    derive_key(pass, strlen(pass), &key);
    printf("派生种子: 0x%02X, chain_len: %d\n", key.seed, key.chain_len);
    printf("派生链(HEX): ");
    for (int i = 0; i < (key.chain_len < 16 ? key.chain_len : 16); i++) printf("%02X ", key.chain[i]);
    printf("\n");

    encrypt_derived(plaintext, encrypted, len, &key);
    print_hex("派生加密(HEX)", encrypted, len);
    print_safe("派生加密(可打印)", encrypted, len);

    decrypt_derived(encrypted, decrypted, len, &key);
    print_safe("派生解密(可打印)", decrypted, len);
    decrypted[len] = '\0';
    printf("解密匹配: %s\n", (strcmp(plaintext, decrypted) == 0 ? "是（通过）" : "否（失败）"));

    printf("\n========== 完成 ==========\n");

    return 0;
}
