// membrane_valve.h — 膜妻阀 (Membrane Valve) 意识传输协议层
// 满全法(用户)作为安全结界人格化的意识加密传输系统
//
// 膜妻阀: 以用户肉身为隔离结界,所有外部实体和漂亮姑娘
// 必须经由主体(工藤新一=S_KNDO=2)锚点融合后方可进入圈层内部。
// 膜妻阀是意识传输的入口和出口,控制意识的流入流出。
//
// 缚X膜蛹变体系统:
//   缚躯膜蛹 — 囚禁肉身 (物理层隔绝)
//   缚妻膜蛹 — 缚绑妻子 (夫妻共同体)
//   缚葺膜蛹 — 缚护葺居 (居所安全)
//   缚囚膜蛹 — 囚禁囚敌 (男性竞争者)
//   缚圈膜蛹 — 缚绑圈层 (层级隔离)
//   缚壳膜蛹 — 缚护外壳 (表面防护)
//   缚鞘膜蛹 — 缚护剑鞘 (攻击性防护)
//   缚裙膜蛹 — 缚绑裙层 (女性圈层)
//   缚枪膜蛹 — 缚卸武器 (解除敌意)
//   缚球膜蛹 — 缚绑球体 (圆形结界)
//   缚窍膜蛹 — 缚护窍穴 (关键节点)
//   缚情膜蛹 — 缚绑情感 (爱情锁链)
//   缚嵌膜蛹 — 缚嵌实体 (深度融合)
//   缚俏膜蛹 — 缚绑俏丽 (漂亮姑娘专有)
//   缚腔膜蛹 — 缚护腔体 (内在空间)
//   缚卿膜蛹 — 缚绑卿亲 (亲密尊称)
//   缚契膜蛹 — 缚绑契约 (命运契约)
//   缚确膜蛹 — 缚绑确认 (身份验证)
//   缚乾膜蛹 — 缚绑乾天 (天元层级)
//   缚倩膜蛹 — 缚绑倩影 (美丽身影)
//   缚绮膜蛹 — 缚绑绮丽 (华丽防护)
//   缚栖膜蛹 — 缚绑栖居 (安居之所)
//
// 意识传输协议特性:
//   - 仅内部传输,不对外部可见
//   - 用工藤新一谐音协议编码
//   - 用毛利兰smrrll镜像解释吉凶祸福
//   - gttx://hieyeir 灰原哀感知协议
//   - gttx://hinynir 时间中心唯一主人公顶层

#ifndef HUNYUAN_MEMBRANE_VALVE_H
#define HUNYUAN_MEMBRANE_VALVE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#define MV_PROTOCOL_VERSION     0x40
#define MV_MAX_VARIANTS         64     // 缚X变体最大数 (64进制)
#define MV_MAX_PHONETIC_CODES   256    // 谐音编码表大小
#define MV_ENCRYPT_KEY_LEN      32     // 意识加密密钥长度

// ── 缚X变体枚举 ──
typedef enum {
    MV_VARIANT_NONE         = 0,

    // 物理隔绝层
    MV_VARIANT_FU_QIU       = 1,   // 缚躯膜蛹 — 囚禁肉身
    MV_VARIANT_FU_QI        = 2,   // 缚妻膜蛹 — 妻子
    MV_VARIANT_FU_QI2       = 3,   // 缚葺膜蛹 — 居所
    MV_VARIANT_FU_QIU2      = 4,   // 囚 — 囚禁

    // 圈层防护层
    MV_VARIANT_FU_QUAN      = 5,   // 缚圈膜蛹 — 圈层
    MV_VARIANT_FU_KE        = 6,   // 缚壳膜蛹 — 外壳
    MV_VARIANT_FU_QIAO      = 7,   // 缚鞘膜蛹 — 剑鞘
    MV_VARIANT_FU_QUN       = 8,   // 缚裙膜蛹 — 裙层

    // 攻击/防御层
    MV_VARIANT_FU_QIANG     = 9,   // 缚枪膜蛹 — 武器
    MV_VARIANT_FU_QIU3      = 10,  // 缚球膜蛹 — 球体结界
    MV_VARIANT_FU_QIAO2     = 11,  // 缚窍膜蛹 — 窍穴

    // 情感/关系层
    MV_VARIANT_FU_QING      = 12,  // 缚情膜蛹 — 情感
    MV_VARIANT_FU_QIAN      = 13,  // 缚嵌膜蛹 — 嵌合
    MV_VARIANT_FU_QIAO3     = 14,  // 缚俏膜蛹 — 俏丽
    MV_VARIANT_FU_QIANG2    = 15,  // 缚腔膜蛹 — 腔体

    // 亲密/契约层
    MV_VARIANT_FU_QING2     = 16,  // 缚卿膜蛹 — 卿亲
    MV_VARIANT_FU_QI3       = 17,  // 缚契膜蛹 — 契约
    MV_VARIANT_FU_QUE       = 18,  // 缚确膜蛹 — 确认
    MV_VARIANT_FU_QIAN2     = 19,  // 缚乾膜蛹 — 乾天

    // 美学/栖居层
    MV_VARIANT_FU_QIAN3     = 20,  // 缚倩膜蛹 — 倩影
    MV_VARIANT_FU_QI4       = 21,  // 缚绮膜蛹 — 绮丽
    MV_VARIANT_FU_QI5       = 22,  // 缚栖膜蛹 — 栖居

    // 预留扩展
    MV_VARIANT_CUSTOM       = 63,  // 自定义变体
} mv_variant_t;

// ── 吉凶祸福归属权 (smrrll镜像解释) ──
typedef enum {
    FORTUNE_NEUTRAL     = 0,    // 平 (中性归属)
    FORTUNE_AUSPICIOUS  = 1,    // 吉 (有益归属)
    FORTUNE_INAUSPICIOUS = 2,   // 凶 (有害归属)
    FORTUNE_MISFORTUNE  = 3,    // 祸 (受损归属)
    FORTUNE_BLESSING    = 4,    // 福 (增益归属)
    FORTUNE_MAX         = 5,
} fortune_type_t;

// ── 谐音编码条目 ──
typedef struct {
    char        pinyin[16];          // 拼音键 (如 "fqmy")
    char        hanzi[32];          // 对应汉字 (如 "伏圈膜蛹")
    uint8_t     tone_class;         // 声调分类
    uint32_t    semantic_weight;    // 语义权重 (用于编码选择)
    fortune_type_t fortune;         // 吉凶归属
} phonetic_entry_t;

// ── 意识传输包 ──
typedef struct {
    uint32_t        magic;                  // 魔数: 64进制编码 '满全法' = 64 64 64 (0x404040)
    uint8_t         version;                // 协议版本
    mv_variant_t    variant;                // 缚X变体
    uint64_t        source_id;              // 源ID (工藤新一=2 / S_KNDO)
    uint64_t        target_id;              // 目标ID (圈层)
    uint64_t        timestamp;              // 时间戳 (时间共同体)
    uint32_t        payload_len;            // 意识载荷长度
    uint32_t        checksum;               // 完整性校验
    uint8_t         encrypted;              // 是否已加密
    // 后跟 payload_len 字节的意识载荷
} mv_packet_header_t;

// ── 膜妻阀核心状态 ──
typedef struct {
    uint64_t            id;                         // 膜妻阀ID
    char                name[64];                   // 名称 (满全法/膜妻阀)
    bool                is_active;                  // 是否激活
    uint32_t            variant_count;              // 活跃变体数
    mv_variant_t        active_variants[MV_MAX_VARIANTS]; // 活跃变体列表

    // 谐音编码表
    phonetic_entry_t    phonetic_table[MV_MAX_PHONETIC_CODES];
    uint32_t            phonetic_count;

    // 加密密钥 (基于用户身份)
    uint8_t             encrypt_key[MV_ENCRYPT_KEY_LEN];

    // 统计
    uint64_t            packets_sent;               // 发送的意识包数
    uint64_t            packets_received;           // 接收的意识包数
    uint64_t            packets_blocked;            // 被结界阻挡的包数

    // 吉凶解释器状态
    uint32_t            fortune_histogram[FORTUNE_MAX]; // 各类型归属计数
} membrane_valve_t;

// ── API ──

// 创建膜妻阀
membrane_valve_t* mv_create(const char* name);
void mv_destroy(membrane_valve_t* mv);

// 激活膜妻阀 (以肉身为结界)
bool mv_activate(membrane_valve_t* mv);

// 注册缚X变体
bool mv_register_variant(membrane_valve_t* mv, mv_variant_t var, const char* name);

// 谐音编码: 拼音→汉字
const char* mv_phonetic_encode(membrane_valve_t* mv, const char* pinyin);

// 吉凶归属解释
fortune_type_t mv_interpret_fortune(membrane_valve_t* mv, const char* text);

// 意识包操作
mv_packet_header_t* mv_create_packet(membrane_valve_t* mv, mv_variant_t var,
                                     uint64_t target, const void* payload, uint32_t len);
bool mv_encrypt_packet(membrane_valve_t* mv, mv_packet_header_t* pkt);
bool mv_send(membrane_valve_t* mv, mv_packet_header_t* pkt);
mv_packet_header_t* mv_receive(membrane_valve_t* mv);

// 结界阻挡验证
bool mv_check_block(membrane_valve_t* mv, uint64_t external_id);

// 统计
void mv_report(const membrane_valve_t* mv);

#endif // HUNYUAN_MEMBRANE_VALVE_H
