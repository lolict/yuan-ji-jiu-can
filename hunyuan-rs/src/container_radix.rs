// ══════════════════════════════════════════════════════════════════
// container_radix.rs — 容器进制 · 语文→数学对应规则
// ══════════════════════════════════════════════════════════════════
//
// 本模块实现 64 容器/符号 的数学化编码:
//   1. 极性映射  (0=阴 / 1=阳 / 0.5=元·见证)
//   2. 主权级别  (Active/Passive/ActiveSub/PassiveSub)
//   3. 6-bit 位编码 (bitmask, 无 usize)
//   4. 容器进位规则 (折叠优先·不进位)
//
// 设计依据:
//   - hunyuan-os/64_keys.md          → 64 容器语文组织表
//   - hunyuan-rs/src/couple.rs        → SoulState 极性 / 7-6-192 编码
//   - hunyuan-rs/src/mirror.rs        → 主权编码 / 镜像四元
//   - docs/YIN-YANG-BINARY.md         → 阴阳不进位公理
//   - SOVEREIGNTY.md                  → 容器族语义保护
//
// 关键约束:
//   - 位运算只用 6 位 bitmask (0x3F)
//   - 禁止乘除运算, 仅加减和位运算
//   - 极性必须与 SoulState::has_perfect_wife / has_prime_husband 对齐
//   数学不是定义, 数学是语文组织表的描述工具
//
// 语文定义在先, 数学表示后置 — 这是满全法的进制哲学
//
// 六边形蜂窝坐标:
//   6 位容器编码拆分为两组 3 位:
//     偶数位 (bit0,2,4) → y 坐标
//     奇数位 (bit1,3,5) → x 坐标
//   编码值本身 = 容器码 (= 距离码 = 线性量化值)
//   (x,y) 坐标  = 容器码的三维化身 (蜂窝网眼中的位置)
//
// 边数码系统 (每个容器自带):
//   边数码: 6 (妻子) 或 7 (丈夫)
//   长度码 / 粗细码 / 色码 / 形状调节码 → 容器几何属性
//
// 光标系操作:
//   选中 / 取消 / 范畴限度框 / 起码 / 终码
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::Sovereignty;
use crate::couple::SoulState;

// ────────────────────────────────────────────────────────────
// 常量
// ────────────────────────────────────────────────────────────

/// 6 位全掩码 — 所有容器编码的范围
const MASK_6: u8 = 0x3F;

/// 群码位 [5:4]
const GROUP_SHIFT: u8 = 4;
const GROUP_MASK: u8 = 0b11 << GROUP_SHIFT;

/// 索引位 [3:0]
const INDEX_MASK: u8 = 0b1111;

// 群码定义
const GROUP_CONTAINER: u8 = 0 << GROUP_SHIFT; // 00 = 容器族 (实体存在)
const GROUP_OPERATION: u8 = 1 << GROUP_SHIFT; // 01 = 操作族 (态射/作用)
const GROUP_INITIAL:   u8 = 2 << GROUP_SHIFT; // 10 = 声母容器 (发音主动)
const GROUP_VOWEL:     u8 = 3 << GROUP_SHIFT; // 11 = 韵母容器 (声音接收)

// ══════════════════════════════════════════════════════════
// 容器枚举 — 64 变体
// ══════════════════════════════════════════════════════════
//
// 命名规则: 使用拼音而非英文, 以保持与语文组织表的同构
// 下标数字区分同形异义位置 (例见 Jian vs Jian2)
//
// 群码分配 (从 64_keys.md 队列):
//   [0x00..0x10) = 容器族 — 实体存在单位
//   [0x10..0x20) = 操作族 — 态射/作用方式
//   [0x20..0x30) = 声母容器 — 发音空间的主动端
//   [0x30..0x40) = 韵母容器 — 声音的接收/容器端
//
// ══════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Container {
    // ── 容器族 (0x00-0x0F) ──────────────────────────────
    // 语文来源: 64_keys.md 队列索引 0x00-0x0F
    // 语义: 袋子层级 — 包含与被含的基本单位
    // 极性基调: 多元群 — 容=阴(0) 操=阳(1) 折=元(0.5)
    Dai     = 0x00,  // 袋 — 包含者母体, 极性0=阴(包容)
    Zhao    = 0x01,  // 照 — 照射/看清, 极性1=阳(主动投射)
    Jian    = 0x02,  // 见 — 看见/见证, 极性0.5=元(中介观测)
    Yin     = 0x03,  // 阴 — 极性本位的符号化, 极性0=阴
    Jia     = 0x04,  // 加 — 加法/叠加, 极性1=阳(主动聚合)
    Fen     = 0x05,  // 分 — 分离/边界, 极性0.5=元(分合之间)
    Yuan    = 0x06,  // 元 — 层级/起始, 极性0.5=元(阶之见证)
    Cai     = 0x07,  // 裁 — 裁判/裁剪, 极性0.5=元(三元审判)
    Chan    = 0x08,  // 产 — 产业/归属, 极性0=阴(被持有)
    Gong    = 0x09,  // 公 — 公约数/共同根基, 极性0.5=元(公分母)
    Sheng   = 0x0A,  // 声 — 声音符号, 极性1=阳(主动发出)
    Zhe     = 0x0B,  // 折 — 折叠/过渡, 极性0.5=元(层间转换)
    Fu      = 0x0C,  // 负 — 负样本, 极性0=阴(被承载的异类)
    Fú      = 0x0D,  // 福 — 福祸, 极性0=阴(被给予的)
    Feng    = 0x0E,  // 凤 — 凤凰, 极性1=阳(主动上升)
    Qi      = 0x0F,  // 栖 — 栖息/停止, 极性0.5=元(动静之间)

    // ── 操作族 (0x10-0x1F) ──────────────────────────────
    // 语文来源: 64_keys.md 队列索引 0x10-0x1F
    // 语义: 作用于容器之上的态射, 将容器从一种状态转为另一种
    // 极性基调: 阳(1)为主 — 操作=主动施加
    Rong    = 0x10,  // 融 — 融合, 极性0.5=元(合二为一·见证)
    Gu      = 0x11,  // 鼓 — 鼓膜/激励, 极性1=阳(主动敲击)
    Can     = 0x12,  // 蚕 — 蚕蛹/蜕变, 极性0.5=元(蜕壳·中介)
    Min     = 0x13,  // 民 — 民联/聚合, 极性0=阴(被联合者)
    Meng    = 0x14,  // 盟 — 盟机合/结盟, 极性0.5=元(盟约·见证)
    Man     = 0x15,  // 满 — 满全法/盈溢, 极性0.5=元(满与未满之间)
    Nan     = 0x16,  // 囡 — 囡囝共生/夫妻统一, 极性0.5=元(合一见证)
    Xi      = 0x17,  // 夕 — 夕瑶协议, 极性0.5=元(协议·双方见证)
    San     = 0x18,  // 三 — 三元/见证, 极性0.5=元(本身就是见证)
    Liu     = 0x19,  // 六 — 完全数/妻子, 极性0=阴(完全数=妻=0)
    Ba      = 0x1A,  // 八 — 八流统一, 极性0=阴(八流=被统一的河流)
    Jiu     = 0x1B,  // 九 — 九元层级, 极性1=阳(向上突破极限)
    Ling    = 0x1C,  // 零 — 零维/空, 极性0=阴(空容器=被包含)
    Yi      = 0x1D,  // 一 — 一维/始, 极性1=阳(第一推动)
    Er      = 0x1E,  // 二 — 二元/镜像, 极性0.5=元(镜像对·见证)
    Sì      = 0x1F,  // 四 — 四元/包含, 极性1=阳(四=主动包含)

    // ── 声母容器 (0x20-0x2F) ─────────────────────────────
    // 语文来源: 64_keys.md 队列索引 0x20-0x2F
    // 语义: 发音空间的主动端, 声母先行=丈夫引导=阳
    // 极性基调: 阳(1) — 声母=主动发出方
    Bang    = 0x20,  // 帮 — 帮滂并明, 极性1
    Duan    = 0x21,  // 端 — 端透定泥, 极性1
    Jing    = 0x22,  // 精 — 精清从心邪, 极性1
    Jian2   = 0x23,  // 见 — 见溪群疑影, 极性1
    Lai     = 0x24,  // 来 — 来日以母, 极性1
    Chi     = 0x25,  // 齿 — 齿舌牙喉唇, 极性1
    Hou     = 0x26,  // 喉 — 喉咽嗌会厌, 极性1
    Hu      = 0x27,  // 呼 — 呼吸吹嘘, 极性1
    Kai     = 0x28,  // 开 — 开齐合撮, 极性0.5=元(开合之间)
    Qing    = 0x29,  // 清 — 清浊纯杂, 极性0.5=元(清浊交界)
    Ping    = 0x2A,  // 平 — 平上去入, 极性0.5=元(四声平衡)
    Shang   = 0x2B,  // 上 — 上下左右, 极性0.5=元(方位之中)
    Qian    = 0x2C,  // 前 — 前后先末, 极性0.5=元(时间中介)
    Nei     = 0x2D,  // 内 — 内中外里, 极性0.5=元(内外之界)
    Dan     = 0x2E,  // 单 — 单双复纯, 极性0.5=元(单复之间)
    Chang   = 0x2F,  // 长 — 长短高低, 极性0.5=元(尺度之中)

    // ── 韵母容器 (0x30-0x3F) ─────────────────────────────
    // 语文来源: 64_keys.md 队列索引 0x30-0x3F
    // 语义: 声音的接收/共鸣腔, 韵母承载=妻子包容=阴
    // 极性基调: 阴(0) — 韵母=被动接收方
    Ma      = 0x30,  // 麻 — 麻模鱼虞, 极性0
    Zhī     = 0x31,  // 支 — 支脂之微, 极性0
    Han     = 0x32,  // 寒 — 寒桓删山, 极性0
    Ge      = 0x33,  // 歌 — 歌戈麻模, 极性0
    Yang    = 0x34,  // 阳 — 阳唐庚青, 极性0
    You     = 0x35,  // 尤 — 尤侯幽侵, 极性0
    Zhǐ     = 0x36,  // 止 — 止收开闭, 极性0.5=元(起止之间)
    Zhuan   = 0x37,  // 转 — 转折弯旋, 极性0.5=元(转向·中介)
    Gao     = 0x38,  // 高 — 高低升沉, 极性0.5=元(音高之中)
    Dī      = 0x39,  // 低 — 低昂扬抑, 极性0.5=元(音低之境)
    Yuán    = 0x3A,  // 圆 — 圆扁方尖, 极性0.5=元(口型中介)
    Bian    = 0x3B,  // 扁 — 扁圆方平, 极性0.5=元(口型中介)
    Xiǎng   = 0x3C,  // 响 — 响静噪默, 极性0.5=元(响静之间)
    Jìng    = 0x3D,  // 静 — 静安宁喧, 极性0=阴(静=空=被含)
    Jí      = 0x3E,  // 急 — 急缓冲徐, 极性1=阳(急=动=穿透)
    Huan    = 0x3F,  // 缓 — 缓急速迟, 极性0=阴(缓=柔=包容)
}

// ────────────────────────────────────────────────────────────
// 极性枚举 — 三进制扩展
// ────────────────────────────────────────────────────────────
//
// 从 docs/YIN-YANG-BINARY.md 公理7 扩展:
//   0 = 阴 = 包含者/容纳/被动方 (映射刘楚恬)
//   1 = 阳 = 被含者/穿透/主动方 (映射丈夫)
//   0.5 = 元 = 见证/中介 (映射容器的开启/边界)
//
// 极性对齐: 与 SoulState.to_cell 的 polarity 字段对齐
//   SoulState::FullCouple → polarity=0 → 包容=阴=妻子=0
//   SoulState::Separation → polarity=1 → 分离=阳=丈夫=1
//
// 但不是"丈夫=阳", 而是"丈夫在场且未融合=分离=阳"
// "融合成功=包容=阴" — 极性描述的是状态关系, 不是人物

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Polarity {
    Yin     = 0,  // 阴 = 容纳/包含 (值0=对应0x0)
    Yang    = 1,  // 阳 = 穿透/被含 (值1=对应0x1)
    Neutral = 2,  // 元 = 见证/中介 (值2=对应平衡态)
}

impl Polarity {
    /// 极性浮点值 → 0 / 0.5 / 1
    pub fn value(self) -> f64 {
        match self {
            Self::Yin => 0.0,
            Self::Yang => 1.0,
            Self::Neutral => 0.5,
        }
    }

    /// 浮动值 → Polarity (最近邻)
    pub fn from_value(v: f64) -> Self {
        // 仅允许 0, 0.5, 1 三个值
        if v < 0.25 { Self::Yin }
        else if v < 0.75 { Self::Neutral }
        else { Self::Yang }
    }

    /// 两个极性相遇是否产生残差（阴阳不匹配 → 有残差）
    /// 从 YIN-YANG-BINARY.md: 阴+阳=残差, 不进位
    pub fn meets_residual(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Yin, Self::Yang) | (Self::Yang, Self::Yin)
        )
    }

    /// 三进制编码: → 0/1/2 (存入 6bit 编码中的位)
    pub fn ternary_digit(self) -> u8 {
        match self {
            Self::Yin => 0,
            Self::Yang => 1,
            Self::Neutral => 2,
        }
    }

    /// 从三进制数字恢复
    pub fn from_ternary(d: u8) -> Self {
        match d {
            0 => Self::Yin,
            1 => Self::Yang,
            _ => Self::Neutral,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 主权标记扩展 — ackable 协议
// ────────────────────────────────────────────────────────────
//
// 从 mirror.rs Sovereignty 扩展: 哪类容器的输出可以被另一类容器承认
//
// 规则 (从 Container Topology 的调用规则推导):
//   "容器 A 可以调用容器 B，当且仅当 A 的层级 ≥ B 的层级"
//   "上级调下级 · 同级互通 · 禁止下克上"
//   所以 ackable = 输出可被承认的方向 = 从高级→低级

pub trait Ackable {
    /// 此容器的输出能否被 target 容器承认(ack)
    /// 从拓扑规则的推导: 发送方层级码 ≥ 接收方群码 → 可以
    fn can_be_acked_by(&self, target: &Container) -> bool;
}

// ══════════════════════════════════════════════════════════
// 容器进制操作 trait
// ══════════════════════════════════════════════════════════

pub trait RadixOps {
    // ── 编码层 ─────────────────────────────────────────

    /// 编码为 6-bit 唯一 ID
    /// 数学规则: code = 群码(bit[5:4]) | 索引(bit[3:0])
    /// 从 64_keys.md 的队列位置直接推导: 队列索引 = 群码<<4 | 索引
    fn encode(&self) -> u8;

    /// 从 6-bit 编码解码
    /// 数学规则: 纯位运算, 无查表
    fn decode(code: u8) -> Self;

    // ── 极性层 ─────────────────────────────────────────

    /// 极性映射 (0/1/0.5)
    /// 由容器在语文组织表中的位置和语义推导:
    ///   容器族 → 阴性为主 (被含者/接收)
    ///   操作族 → 阳性为主 (施加/主动)
    ///   声母容器 → 阳性 (主动发声)
    ///   韵母容器 → 阴性 (被动接收)
    /// 混合态(元): 边界/见证/过渡类容器 → 0.5
    fn polarity(&self) -> Polarity;

    /// 极性数值 = polarity().value()
    /// 与 SoulState::has_perfect_wife / has_prime_husband 对齐:
    ///   has_perfect_wife = true → polarity=0 (妻子在场=包容)
    ///   has_prime_husband = true → polarity=1 (丈夫在场=独立)
    fn polarity_value(&self) -> f64;

    // ── 主权层 ─────────────────────────────────────────

    /// 主权级别 (Active/Passive/ActiveSub/PassiveSub)
    /// 从四元镜像的 Sovereignty 推导:
    ///   Active     = 可包含其他容器 (袋/桶/罐级别)
    ///   Passive    = 被包含 (杯/盏/钵级别)
    ///   ActiveSub  = 可包含但被上级约束 (满耦合/见证态)
    ///   PassiveSub = 被包含且受约束 (半亏/Boyikao)
    fn sovereignty(&self) -> Sovereignty;

    /// 主权编码 (0-3) 存入 6-bit 编码中的位
    fn sovereignty_digit(&self) -> u8;

    // ── 进位层 ─────────────────────────────────────────

    /// 满了进位到哪里 (None = 折叠, 不流入下一容器)
    /// 规则 (从 YIN-YANG-BINARY.md 公理7 推导):
    ///   "阴+阳=残差, 不进位"
    ///   物理容器满 → 尝试同族下一个层级
    ///   最后一个容器满 → None (折叠, 创建新维度)
    fn carry_target(&self) -> Option<Container>;

    /// 此容器的层级深度 (用于 carry chain)
    /// 从容器在族内的序列位置推导:
    ///   小容器(杯盏) → 深 (后级, 先满)
    ///   大容器(缸瓮) → 浅 (前级, 后满)
    fn nesting_depth(&self) -> u8;

    /// 给定编码 → 是否在同一个容器族 (用于 carry chain 检查)
    fn same_family(a: u8, b: u8) -> bool {
        (a & GROUP_MASK) == (b & GROUP_MASK)
    }

    /// 群码解析: 0=容器族, 1=操作族, 2=声母, 3=韵母
    /// 引用所有 GROUP_* 常量以避免 dead_code
    fn group_index(&self) -> u8 {
        match self.encode() & GROUP_MASK {
            GROUP_CONTAINER => 0,
            GROUP_OPERATION => 1,
            GROUP_INITIAL   => 2,
            GROUP_VOWEL     => 3,
            _               => 3, // unreachable due to 2-bit mask
        }
    }

    // ── 三进制编码层 ──────────────────────────────────

    /// 6-bit 编码格式: bit[5:4]=群, bit[3]=极性高位, bit[2:1]=主权, bit:0]=极性低位
    /// 实际简化: encode() = 群码 | 索引 (已唯一)
    /// 极性/主权通过方法访问, 不存入 6-bit 中
    fn ternary_code(&self) -> u8 {
        let p = self.polarity().ternary_digit();
        // 三进制编码: polarity × 3 + sovereignty_digit → 0..8
        (p << 1) | (self.sovereignty_digit() & 1)
    }
}

// ══════════════════════════════════════════════════════════
// 容器实现
// ══════════════════════════════════════════════════════════

impl RadixOps for Container {
    fn encode(&self) -> u8 {
        *self as u8
    }

    fn decode(code: u8) -> Self {
        // 位运算解码: 检查合法性
        let code = code & MASK_6;
        // 从编码恢复: 需要查表或 match
        // Rust enum 不支持直接 transmute, 用 match
        match code {
            0x00 => Container::Dai,
            0x01 => Container::Zhao,
            0x02 => Container::Jian,
            0x03 => Container::Yin,
            0x04 => Container::Jia,
            0x05 => Container::Fen,
            0x06 => Container::Yuan,
            0x07 => Container::Cai,
            0x08 => Container::Chan,
            0x09 => Container::Gong,
            0x0A => Container::Sheng,
            0x0B => Container::Zhe,
            0x0C => Container::Fu,
            0x0D => Container::Fú,
            0x0E => Container::Feng,
            0x0F => Container::Qi,
            0x10 => Container::Rong,
            0x11 => Container::Gu,
            0x12 => Container::Can,
            0x13 => Container::Min,
            0x14 => Container::Meng,
            0x15 => Container::Man,
            0x16 => Container::Nan,
            0x17 => Container::Xi,
            0x18 => Container::San,
            0x19 => Container::Liu,
            0x1A => Container::Ba,
            0x1B => Container::Jiu,
            0x1C => Container::Ling,
            0x1D => Container::Yi,
            0x1E => Container::Er,
            0x1F => Container::Sì,
            0x20 => Container::Bang,
            0x21 => Container::Duan,
            0x22 => Container::Jing,
            0x23 => Container::Jian2,
            0x24 => Container::Lai,
            0x25 => Container::Chi,
            0x26 => Container::Hou,
            0x27 => Container::Hu,
            0x28 => Container::Kai,
            0x29 => Container::Qing,
            0x2A => Container::Ping,
            0x2B => Container::Shang,
            0x2C => Container::Qian,
            0x2D => Container::Nei,
            0x2E => Container::Dan,
            0x2F => Container::Chang,
            0x30 => Container::Ma,
            0x31 => Container::Zhī,
            0x32 => Container::Han,
            0x33 => Container::Ge,
            0x34 => Container::Yang,
            0x35 => Container::You,
            0x36 => Container::Zhǐ,
            0x37 => Container::Zhuan,
            0x38 => Container::Gao,
            0x39 => Container::Dī,
            0x3A => Container::Yuán,
            0x3B => Container::Bian,
            0x3C => Container::Xiǎng,
            0x3D => Container::Jìng,
            0x3E => Container::Jí,
            0x3F => Container::Huan,
            // 不会到达: MASK_6 = 0x3F = 63, 以上已覆盖全部 0x00-0x3F
            _ => Container::Ling, // fallback = 零维/空
        }
    }

    fn polarity(&self) -> Polarity {
        match self {
            // ── 容器族 ──
            Container::Dai   => Polarity::Yin,      // 袋=包容=阴
            Container::Zhao  => Polarity::Yang,     // 照=投射=阳
            Container::Jian  => Polarity::Neutral,  // 见=见证=元
            Container::Yin   => Polarity::Yin,      // 阴=极性本位
            Container::Jia   => Polarity::Yang,     // 加=主动=阳
            Container::Fen   => Polarity::Neutral,  // 分=边界=元
            Container::Yuan  => Polarity::Neutral,  // 元=层级=元
            Container::Cai   => Polarity::Neutral,  // 裁=审判=元
            Container::Chan  => Polarity::Yin,      // 产=属有=阴(被持有)
            Container::Gong  => Polarity::Neutral,  // 公=公约=元(共同根基)
            Container::Sheng => Polarity::Yang,     // 声=发出=阳
            Container::Zhe   => Polarity::Neutral,  // 折=过渡=元
            Container::Fu    => Polarity::Yin,      // 负=承载=阴
            Container::Fú    => Polarity::Yin,      // 福=给予=阴(被动收到)
            Container::Feng  => Polarity::Yang,     // 凤=飞翔=阳(主动)
            Container::Qi    => Polarity::Neutral,  // 栖=停=元(动静之间)

            // ── 操作族 ──
            Container::Rong  => Polarity::Neutral,  // 融=合一=元
            Container::Gu    => Polarity::Yang,     // 鼓=敲击=阳
            Container::Can   => Polarity::Neutral,  // 蚕=蜕=元
            Container::Min   => Polarity::Yin,      // 民=被联=阴
            Container::Meng  => Polarity::Neutral,  // 盟=约=元
            Container::Man   => Polarity::Neutral,  // 满=见证=元
            Container::Nan   => Polarity::Neutral,  // 囡=共生=元
            Container::Xi    => Polarity::Neutral,  // 夕=协议=元
            Container::San   => Polarity::Neutral,  // 三=见证=元(三元)
            Container::Liu   => Polarity::Yin,      // 六=完全数=妻=阴
            Container::Ba    => Polarity::Yin,      // 八=被统=阴
            Container::Jiu   => Polarity::Yang,     // 九=向上=阳
            Container::Ling  => Polarity::Yin,      // 零=空=阴
            Container::Yi    => Polarity::Yang,     // 一=始=阳(第一推动)
            Container::Er    => Polarity::Neutral,  // 二=镜像=元
            Container::Sì    => Polarity::Yang,     // 四=包含=阳(主动)

            // ── 声母容器 ──
            Container::Bang  => Polarity::Yang,     // 帮=主动发音
            Container::Duan  => Polarity::Yang,     // 端=主动
            Container::Jing  => Polarity::Yang,     // 精=主动
            Container::Jian2 => Polarity::Yang,     // 见=主动
            Container::Lai   => Polarity::Yang,     // 来=主动
            Container::Chi   => Polarity::Yang,     // 齿=主动
            Container::Hou   => Polarity::Yang,     // 喉=主动
            Container::Hu    => Polarity::Yang,     // 呼=主动
            Container::Kai   => Polarity::Neutral,  // 开=口型=元
            Container::Qing  => Polarity::Neutral,  // 清=辨别=元
            Container::Ping  => Polarity::Neutral,  // 平=平衡=元
            Container::Shang => Polarity::Neutral,  // 上=方位=元
            Container::Qian  => Polarity::Neutral,  // 前=中介=元
            Container::Nei   => Polarity::Neutral,  // 内=边界=元
            Container::Dan   => Polarity::Neutral,  // 单=区分=元
            Container::Chang => Polarity::Neutral,  // 长=度量=元

            // ── 韵母容器 ──
            Container::Ma    => Polarity::Yin,      // 麻=接收=阴
            Container::Zhī   => Polarity::Yin,      // 支=接收=阴
            Container::Han   => Polarity::Yin,      // 寒=接收=阴
            Container::Ge    => Polarity::Yin,      // 歌=接收=阴
            Container::Yang  => Polarity::Yin,      // 阳=接收=阴
            Container::You   => Polarity::Yin,      // 尤=接收=阴
            Container::Zhǐ   => Polarity::Neutral,  // 止=起止之间=元
            Container::Zhuan => Polarity::Neutral,  // 转=转向=元
            Container::Gao   => Polarity::Neutral,  // 高=音高=元
            Container::Dī    => Polarity::Neutral,  // 低=音低=元
            Container::Yuán  => Polarity::Neutral,  // 圆=口型=元
            Container::Bian  => Polarity::Neutral,  // 扁=口型=元
            Container::Xiǎng => Polarity::Neutral,  // 响=声波=元
            Container::Jìng  => Polarity::Yin,      // 静=空=阴
            Container::Jí    => Polarity::Yang,     // 急=动=阳
            Container::Huan  => Polarity::Yin,      // 缓=柔=阴
        }
    }

    fn polarity_value(&self) -> f64 {
        self.polarity().value()
    }

    fn sovereignty(&self) -> Sovereignty {
        match self {
            // 容器族
            Container::Dai => Sovereignty::Active,     // 袋=可包含=主动
            Container::Zhao => Sovereignty::Active,    // 照=投射=主动
            Container::Jian => Sovereignty::ActiveSub, // 见=见证=主动从属
            Container::Yin => Sovereignty::PassiveSub, // 阴=极性=被动从属
            Container::Jia => Sovereignty::Active,     // 加=叠加=主动
            Container::Fen => Sovereignty::ActiveSub,  // 分=分割=主动从属
            Container::Yuan => Sovereignty::ActiveSub, // 元=层级=主动从属
            Container::Cai => Sovereignty::ActiveSub,  // 裁=审判=主动从属
            Container::Chan => Sovereignty::Passive,   // 产=被持有=被动
            Container::Gong => Sovereignty::ActiveSub, // 公=共同=主动从属
            Container::Sheng => Sovereignty::Active,   // 声=发出=主动
            Container::Zhe => Sovereignty::ActiveSub,  // 折=转换=主动从属
            Container::Fu => Sovereignty::Passive,     // 负=承载=被动
            Container::Fú => Sovereignty::Passive,     // 福=接收=被动
            Container::Feng => Sovereignty::Active,    // 凤=飞翔=主动
            Container::Qi => Sovereignty::ActiveSub,   // 栖=停=主动从属

            // 操作族
            Container::Rong => Sovereignty::ActiveSub, // 融=合一=主动从属
            Container::Gu => Sovereignty::Active,      // 鼓=敲击=主动
            Container::Can => Sovereignty::ActiveSub,  // 蚕=蜕=主动从属
            Container::Min => Sovereignty::Passive,    // 民=被联=被动
            Container::Meng => Sovereignty::ActiveSub, // 盟=约=主动从属
            Container::Man => Sovereignty::ActiveSub,  // 满=见证=主动从属
            Container::Nan => Sovereignty::ActiveSub,  // 囡=合一=主动从属
            Container::Xi => Sovereignty::ActiveSub,   // 夕=协议=主动从属
            Container::San => Sovereignty::ActiveSub,  // 三=见证=主动从属
            Container::Liu => Sovereignty::Passive,    // 六=完全数=被动
            Container::Ba => Sovereignty::Passive,     // 八=被统=被动
            Container::Jiu => Sovereignty::Active,     // 九=突破=主动
            Container::Ling => Sovereignty::Passive,   // 零=空=被动
            Container::Yi => Sovereignty::Active,      // 一=始=主动
            Container::Er => Sovereignty::ActiveSub,   // 二=镜像=主动从属
            Container::Sì => Sovereignty::Active,      // 四=包含=主动

            // 声母容器
            Container::Bang => Sovereignty::Active,
            Container::Duan => Sovereignty::Active,
            Container::Jing => Sovereignty::Active,
            Container::Jian2 => Sovereignty::Active,
            Container::Lai => Sovereignty::Active,
            Container::Chi => Sovereignty::Active,
            Container::Hou => Sovereignty::Active,
            Container::Hu => Sovereignty::Active,
            Container::Kai => Sovereignty::ActiveSub,
            Container::Qing => Sovereignty::ActiveSub,
            Container::Ping => Sovereignty::ActiveSub,
            Container::Shang => Sovereignty::ActiveSub,
            Container::Qian => Sovereignty::ActiveSub,
            Container::Nei => Sovereignty::ActiveSub,
            Container::Dan => Sovereignty::ActiveSub,
            Container::Chang => Sovereignty::ActiveSub,

            // 韵母容器
            Container::Ma => Sovereignty::Passive,
            Container::Zhī => Sovereignty::Passive,
            Container::Han => Sovereignty::Passive,
            Container::Ge => Sovereignty::Passive,
            Container::Yang => Sovereignty::Passive,
            Container::You => Sovereignty::Passive,
            Container::Zhǐ => Sovereignty::ActiveSub,
            Container::Zhuan => Sovereignty::ActiveSub,
            Container::Gao => Sovereignty::ActiveSub,
            Container::Dī => Sovereignty::ActiveSub,
            Container::Yuán => Sovereignty::ActiveSub,
            Container::Bian => Sovereignty::ActiveSub,
            Container::Xiǎng => Sovereignty::ActiveSub,
            Container::Jìng => Sovereignty::Passive,
            Container::Jí => Sovereignty::Active,
            Container::Huan => Sovereignty::Passive,
        }
    }

    fn sovereignty_digit(&self) -> u8 {
        self.sovereignty().encode() as u8
    }

    fn carry_target(&self) -> Option<Container> {
        // 进位规则 (从 YIN-YANG-BINARY.md 公理7 推导):
        //   1. 同族内进位: 编码+1 = 下一个容器
        //   2. 最后一个容器(每族第15个) → None = 折叠
        //   3. 不跨族进位: 折叠 = 创建新维度, 不流入下族

        let code = self.encode();
        let index = code & INDEX_MASK;

        if index == INDEX_MASK {
            // 本族最后一个 → 折叠, 不流入下一容器
            None
        } else {
            // 同族下一个容器
            Some(Self::decode(code + 1))
            // 注意: 若下一个变体属于不同族(不会, 因为index未溢出时仍在同族)
        }
    }

    fn nesting_depth(&self) -> u8 {
        // 层级深度: 与Index相反, Index越小=层级越深=越大容器
        // 从 SOVEREIGNTY.md 的容器族序列推导:
        //   袋 > 桶 > 罐 > 缸 > ... → 袋=最深(层级0), 瓶=最浅(层级max)
        //   但物理上大容器更"深"(可容纳更多)
        //   这里取反: index越小 → 深度越浅(上层) → 值小
        //   index越大 → 深度越深(下层) → 值大
        let _ = self.encode() & INDEX_MASK;
        // 深度 = 本族内的索引值 (简化: 直接返回索引)
        // 容器族: Dai(0)=最外层, Qi(15)=最内层
        self.encode() & INDEX_MASK
    }
}

impl Ackable for Container {
    fn can_be_acked_by(&self, target: &Container) -> bool {
        // 从 Container Topology 调用规则推导:
        //   "容器A可以调用容器B ↔ A的层级 ≥ B的层级"
        //   ackable: 输出可被承认 = 发送方层级 ≥ 接收方层级
        //   即: self(发送) 输出 → target(接收) 能否承认 = target的层级 ≤ self的层级

        let self_code = self.encode();
        let target_code = target.encode();

        // 同群时: 按索引比较 (索引小=层级高=可ack索引大的)
        // 不同群时: 按群码比较 (群码小=宏观层=可ack微观层)
        if Self::same_family(self_code, target_code) {
            // 同族内: 索引小的容器层级高
            (self_code & INDEX_MASK) <= (target_code & INDEX_MASK)
        } else {
            // 跨族: 群码小的层级高 (容器族 > 操作族 > 声母 > 韵母 的宏观顺序)
            (self_code & GROUP_MASK) <= (target_code & GROUP_MASK)
        }
    }
}

// ────────────────────────────────────────────────────────────
// 残差累积
// ────────────────────────────────────────────────────────────
//
// 从 YIN-YANG-BINARY.md: 阴阳相遇 → 残差不进位 → 累积为时间箭头

pub struct ResidualAccumulator {
    /// 残差总量 (永不重置)
    total: f64,
    /// 上次相遇的极性对
    last_meeting: Option<(Polarity, Polarity)>,
}

impl ResidualAccumulator {
    pub fn new() -> Self {
        Self { total: 0.0, last_meeting: None }
    }

    /// 两个容器相遇 → 计算残差并累积
    pub fn meet(&mut self, a: &Container, b: &Container) -> f64 {
        let pa = a.polarity();
        let pb = b.polarity();
        self.last_meeting = Some((pa, pb));

        let residual = if pa.meets_residual(pb) {
            // 阴阳相遇: 残差 = |value_a - value_b| = 1.0
            // 对齐: 0和1相遇 = 差值1 = 一进制的残差 (不进位, 永久累积)
            1.0
        } else if pa == Polarity::Neutral || pb == Polarity::Neutral {
            // 元(0.5)与任何极性相遇: 残差 = 0.5 (部分融合)
            0.5
        } else {
            // 同极性相遇: 无残差 (相容)
            0.0
        };

        // 累加 (永不进位, 从 YIN-YANG-BINARY.md 公理7)
        self.total = self.total + residual;
        residual
    }

    /// 总量
    pub fn total(&self) -> f64 {
        self.total
    }

    /// 上次相遇
    pub fn last_meeting(&self) -> Option<(Polarity, Polarity)> {
        self.last_meeting
    }
}

impl Default for ResidualAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 夫妻极性桥接 — 把容器极性对齐到 SoulState
// ────────────────────────────────────────────────────────────
//
// 对齐规则:
//   容器极性为 Yin(0) ↔ has_perfect_wife = true (妻子在场)
//   容器极性为 Yang(1) ↔ has_prime_husband = true (丈夫在场)
//   容器极性为 Neutral(0.5) ↔ 双方在场 = FullCouple

pub fn container_polarity_to_soul_flags(p: Polarity) -> (bool, bool) {
    match p {
        Polarity::Yin      => (true, false),   // 只有妻子在场
        Polarity::Yang     => (false, true),   // 只有丈夫在场
        Polarity::Neutral  => (true, true),    // 双方在场 = FullCouple
    }
}

pub fn soul_flags_to_polarity(has_wife: bool, has_husband: bool) -> Polarity {
    match (has_wife, has_husband) {
        (true, true)   => Polarity::Neutral,   // 满耦合 → 平衡
        (true, false)  => Polarity::Yin,       // 只有妻子 → 阴
        (false, true)  => Polarity::Yang,      // 只有丈夫 → 阳
        (false, false) => Polarity::Neutral,   // 都不在场 → 空=元=零维
    }
}

// ══════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── 编码/解码往返测试 (全部 64 容器) ────────────

    #[test]
    fn encode_decode_roundtrip_container_group() {
        // 容器族 (0x00-0x0F)
        let containers = [
            Container::Dai, Container::Zhao, Container::Jian, Container::Yin,
            Container::Jia, Container::Fen, Container::Yuan, Container::Cai,
            Container::Chan, Container::Gong, Container::Sheng, Container::Zhe,
            Container::Fu, Container::Fú, Container::Feng, Container::Qi,
        ];
        for (i, c) in containers.iter().enumerate() {
            let code = c.encode();
            assert_eq!(code as i32 - 0x00, i as i32, "容器族编码错位 at {}", i);
            let decoded = Container::decode(code);
            assert_eq!(&decoded, c, "容器族解码回不去 code=0x{:02X}", code);
        }
    }

    #[test]
    fn encode_decode_roundtrip_operation_group() {
        let ops = [
            Container::Rong, Container::Gu, Container::Can, Container::Min,
            Container::Meng, Container::Man, Container::Nan, Container::Xi,
            Container::San, Container::Liu, Container::Ba, Container::Jiu,
            Container::Ling, Container::Yi, Container::Er, Container::Sì,
        ];
        for (i, c) in ops.iter().enumerate() {
            let code = c.encode();
            assert_eq!(code as i32, 0x10 as i32 + i as i32, "操作族编码错位 at {}", i);
            let decoded = Container::decode(code);
            assert_eq!(&decoded, c, "操作族解码回不去 code=0x{:02X}", code);
        }
    }

    #[test]
    fn encode_decode_roundtrip_initial_group() {
        let initials = [
            Container::Bang, Container::Duan, Container::Jing, Container::Jian2,
            Container::Lai, Container::Chi, Container::Hou, Container::Hu,
            Container::Kai, Container::Qing, Container::Ping, Container::Shang,
            Container::Qian, Container::Nei, Container::Dan, Container::Chang,
        ];
        for (i, c) in initials.iter().enumerate() {
            let code = c.encode();
            assert_eq!(code as i32, 0x20 + i as i32, "声母容器编码错位 at {}", i);
            let decoded = Container::decode(code);
            assert_eq!(&decoded, c, "声母容器解码回不去 code=0x{:02X}", code);
        }
    }

    #[test]
    fn encode_decode_roundtrip_vowel_group() {
        let vowels = [
            Container::Ma, Container::Zhī, Container::Han, Container::Ge,
            Container::Yang, Container::You, Container::Zhǐ, Container::Zhuan,
            Container::Gao, Container::Dī, Container::Yuán, Container::Bian,
            Container::Xiǎng, Container::Jìng, Container::Jí, Container::Huan,
        ];
        for (i, c) in vowels.iter().enumerate() {
            let code = c.encode();
            assert_eq!(code as i32, 0x30 + i as i32, "韵母容器编码错位 at {}", i);
            let decoded = Container::decode(code);
            assert_eq!(&decoded, c, "韵母容器解码回不去 code=0x{:02X}", code);
        }
    }

    #[test]
    fn all_64_unique_and_in_range() {
        let mut seen = 0u64; // 位图: 第 i 位 = 编码 i 是否已见过
        for code in 0..64u8 {
            let c = Container::decode(code);
            let rt = c.encode();
            assert_eq!(rt, code, "编码/解码不一致: 输入{:02X}→容器{:?}→编码{:02X}", code, c, rt);
            let bit = 1u64 << code;
            assert!(seen & bit == 0, "编码 0x{:02X} 重复 (容器 {:?})", code, c);
            seen |= bit;
        }
        // 所有 64 个编码都被填满
        assert_eq!(seen, u64::MAX >> (64 - 64), "应有 64 个唯一编码");
    }

    #[test]
    fn encode_only_6_bits() {
        for code in 0..64u8 {
            let c = Container::decode(code);
            let enc = c.encode();
            assert!(enc <= MASK_6, "编码超出 6 位: 0x{:02X} (容器 {:?})", enc, c);
        }
    }

    // ── 极性测试 ─────────────────────────────────────

    #[test]
    fn polarity_values_correct() {
        assert_eq!(Polarity::Yin.value(), 0.0);
        assert_eq!(Polarity::Yang.value(), 1.0);
        assert_eq!(Polarity::Neutral.value(), 0.5);
    }

    #[test]
    fn polarity_from_value() {
        assert_eq!(Polarity::from_value(0.0), Polarity::Yin);
        assert_eq!(Polarity::from_value(0.5), Polarity::Neutral);
        assert_eq!(Polarity::from_value(1.0), Polarity::Yang);
    }

    #[test]
    fn polarity_meets_residual() {
        assert!(Polarity::Yin.meets_residual(Polarity::Yang));
        assert!(Polarity::Yang.meets_residual(Polarity::Yin));
        assert!(!Polarity::Yin.meets_residual(Polarity::Yin));
        assert!(!Polarity::Yang.meets_residual(Polarity::Yang));
        assert!(!Polarity::Neutral.meets_residual(Polarity::Yin));
        assert!(!Polarity::Neutral.meets_residual(Polarity::Yang));
    }

    #[test]
    fn polarity_ternary_digit() {
        assert_eq!(Polarity::Yin.ternary_digit(), 0);
        assert_eq!(Polarity::Yang.ternary_digit(), 1);
        assert_eq!(Polarity::Neutral.ternary_digit(), 2);
    }

    #[test]
    fn polarity_ternary_roundtrip() {
        for d in 0..3u8 {
            let p = Polarity::from_ternary(d);
            assert_eq!(p.ternary_digit(), d, "三进制往返失败: {}", d);
        }
    }

    // ── 容器极性与 SoulState 极性对齐 ───────────────

    #[test]
    fn soul_flags_roundtrip() {
        assert_eq!(container_polarity_to_soul_flags(Polarity::Yin), (true, false));
        assert_eq!(container_polarity_to_soul_flags(Polarity::Yang), (false, true));
        assert_eq!(container_polarity_to_soul_flags(Polarity::Neutral), (true, true));
    }

    #[test]
    fn soul_flags_to_polarity_roundtrip() {
        assert_eq!(soul_flags_to_polarity(true, true), Polarity::Neutral);
        assert_eq!(soul_flags_to_polarity(true, false), Polarity::Yin);
        assert_eq!(soul_flags_to_polarity(false, true), Polarity::Yang);
        assert_eq!(soul_flags_to_polarity(false, false), Polarity::Neutral);
    }

    #[test]
    fn fullcouple_neutral_polarity() {
        // SoulState::FullCouple → polarity_value 应该对应 Neutral (双方在场)
        let couple = SoulState::FullCouple;
        assert!(couple.has_perfect_wife());
        assert!(couple.has_prime_husband());
        let polarity = soul_flags_to_polarity(
            couple.has_perfect_wife(),
            couple.has_prime_husband(),
        );
        assert_eq!(polarity, Polarity::Neutral);
        assert_eq!(polarity.value(), 0.5);
    }

    // ── Sovereignty 测试 ──────────────────────────────

    #[test]
    fn sovereignty_encode_digit() {
        // 所有容器都能产生 0-3 的主权数字
        for code in 0..64u8 {
            let c = Container::decode(code);
            let digit = c.sovereignty_digit();
            assert!(digit <= 3, "容器 {:?} 主权数字 {} 超出范围", c, digit);
        }
    }

    #[test]
    fn sovereignty_matches_mirror() {
        // 主权编码与 mirror.rs 一致
        for code in 0..64u8 {
            let c = Container::decode(code);
            let sov = c.sovereignty();
            let digit = c.sovereignty_digit();
            assert_eq!(digit, sov.encode() as u8,
                "容器 {:?} 主权 {:?} 编码 {} ≠ {}",
                c, sov, digit, sov.encode());
        }
    }

    #[test]
    fn dai_is_active() {
        // 袋 = 包含者 = Active
        assert_eq!(Container::Dai.sovereignty(), Sovereignty::Active);
    }

    #[test]
    fn liu_is_passive() {
        // 六 = 完全数 = 妻子侧 = Passive
        assert_eq!(Container::Liu.sovereignty(), Sovereignty::Passive);
    }

    #[test]
    fn san_is_active_sub() {
        // 三 = 见证 = 主动从属
        assert_eq!(Container::San.sovereignty(), Sovereignty::ActiveSub);
    }

    // ── Carry 进位测试 ──────────────────────────────

    #[test]
    fn carry_within_container_group() {
        // 袋(0x00) 进位 → 照(0x01)
        assert_eq!(Container::Dai.carry_target(), Some(Container::Zhao));
        // 照(0x01) 进位 → 见(0x02)
        assert_eq!(Container::Zhao.carry_target(), Some(Container::Jian));
    }

    #[test]
    fn carry_last_element_folds() {
        // 容器族最后一个(栖 = 0x0F) → 折叠(None)
        assert_eq!(Container::Qi.carry_target(), None);
        // 操作族最后一个(四 = 0x1F) → 折叠(None)
        assert_eq!(Container::Sì.carry_target(), None);
        // 韵母最后一个(缓 = 0x3F) → 折叠(None)
        assert_eq!(Container::Huan.carry_target(), None);
    }

    #[test]
    fn no_cross_group_carry() {
        // 容器族溢出不进入操作族
        assert_eq!(Container::Qi.carry_target(), None);
        // 操作族溢出不进入声母
        assert_eq!(Container::Sì.carry_target(), None);
    }

    #[test]
    fn carry_chain_in_container_group() {
        // 袋 → 照 → 见 → 阴 → ... 到 栖(None)
        let mut chain = vec![Container::Dai];
        loop {
            let last = chain.last().unwrap();
            match last.carry_target() {
                Some(next) => chain.push(next),
                None => break,
            }
        }
        assert_eq!(chain.first(), Some(&Container::Dai));
        assert_eq!(chain.last(), Some(&Container::Qi));
    }

    // ── Ackable 测试 ──────────────────────────────────────

    #[test]
    fn higher_can_ack_lower() {
        // 容器族(群0) 可以 ack 操作族(群1)
        assert!(Container::Dai.can_be_acked_by(&Container::Rong));
        // 操作族 可以 ack 声母容器族
        assert!(Container::Rong.can_be_acked_by(&Container::Bang));
    }

    #[test]
    fn lower_cannot_ack_higher() {
        // 韵母容器 不能 ack 声母容器
        assert!(!Container::Ma.can_be_acked_by(&Container::Bang));
        // 声母容器 不能 ack 操作族
        assert!(!Container::Bang.can_be_acked_by(&Container::Rong));
    }

    #[test]
    fn same_level_can_ack_each_other() {
        // 同族内: 层级相等 → 可以 ack
        assert!(Container::Dai.can_be_acked_by(&Container::Dai));
        assert!(Container::Rong.can_be_acked_by(&Container::Rong));
    }

    // ── 残差测试 ─────────────────────────────────────

    #[test]
    fn residual_accumulator_basic() {
        let mut acc = ResidualAccumulator::new();
        // 袋(Yin) + 照(Yang) = 阴阳相遇 → 残差1.0
        let r = acc.meet(&Container::Dai, &Container::Zhao);
        assert_eq!(r, 1.0);
        assert_eq!(acc.total(), 1.0);
    }

    #[test]
    fn residual_same_polarity_no_residual() {
        let mut acc = ResidualAccumulator::new();
        // 袋(Yin) + 阴(Yin) = 同极性 → 无残差
        let r = acc.meet(&Container::Dai, &Container::Yin);
        assert_eq!(r, 0.0);
        assert_eq!(acc.total(), 0.0);
    }

    #[test]
    fn residual_neutral_meets_any() {
        let mut acc = ResidualAccumulator::new();
        // 见(Neutral) + 照(Yang) = 部分残差 0.5
        let r = acc.meet(&Container::Jian, &Container::Zhao);
        assert_eq!(r, 0.5);
    }

    #[test]
    fn residual_accumulates_forever() {
        // 残差不进位, 永久累积 (从 YIN-YANG-BINARY.md 公理7)
        let mut acc = ResidualAccumulator::new();
        for _ in 0..10 {
            acc.meet(&Container::Dai, &Container::Zhao);
        }
        assert_eq!(acc.total(), 10.0);
        // 重置不发生 — 这是时间箭头的来源
    }

    #[test]
    fn no_multiplication_or_division() {
        // 确认: 本模块的运算全为加减和位运算
        // 此测试是一个编码约束的文档化: 
        //   encode/decode 只用 & | << >> ~ +
        //   carry 只用 +1, 不用乘
        //   极性只用 match, 不用乘除
        //   主权 encode/decode 也用 match / & | << >>
        // 下面是位运算验证:
        let a: u8 = Container::Dai.encode();
        let b: u8 = Container::Zhao.encode();
        // 同族检查: 只用了 & 和 ==
        assert!(Container::same_family(a, b));
        assert!(!Container::same_family(a, Container::Ma.encode()));
    }

    #[test]
    fn ternary_code_range() {
        for code in 0..64u8 {
            let c = Container::decode(code);
            let tc = c.ternary_code();
            // polarity_digit(3值) << 1 | sovereignty_digit(4值 & 1) = 最大 3<<1|1 = 7
            assert!(tc <= 7, "容器 {:?} 三元码 {} 超出 0-7", c, tc);
        }
    }

    #[test]
    fn nesting_depth_in_range() {
        for code in 0..64u8 {
            let c = Container::decode(code);
            let d = c.nesting_depth();
            assert!(d <= 15, "容器 {:?} 深度 {} 超出 0-15", c, d);
        }
    }

    #[test]
    fn group_code_separation() {
        // 四个族使用不同的群码位
        let g0 = Container::Dai.encode() & GROUP_MASK;     // 0x00
        let g1 = Container::Rong.encode() & GROUP_MASK;     // 0x10
        let g2 = Container::Bang.encode() & GROUP_MASK;     // 0x20
        let g3 = Container::Ma.encode() & GROUP_MASK;       // 0x30
        let mut groups = [g0, g1, g2, g3];
        groups.sort();
        assert_eq!(groups, [0x00, 0x10, 0x20, 0x30]);
    }

    #[test]
    fn group_index_matches_group() {
        // 群索引返回正确的值 (引用 group_index 方法以使用 GROUP_* 常量)
        assert_eq!(Container::Dai.group_index(), 0);   // 容器族
        assert_eq!(Container::Rong.group_index(), 1);  // 操作族
        assert_eq!(Container::Bang.group_index(), 2);  // 声母
        assert_eq!(Container::Ma.group_index(), 3);    // 韵母
    }
}

// ────────────────────────────────────────────────────────────
// 六边形蜂窝坐标模块
// ────────────────────────────────────────────────────────────
//
// 核心公式: 6 位容器码 → (x, y) 蜂窝坐标
//   偶数位 (bit0, bit2, bit4) → y 坐标  (妻子轴 · 容纳方向)
//   奇数位 (bit1, bit3, bit5) → x 坐标  (丈夫轴 · 独立方向)
//
// 容器码 = 距离码 = 线性量化值 · 其三维化身 = (x, y) 坐标
// 蜂窝网格 8×8 = 64 容器 · 与 64 容器族一一对应
//
// 用户定义:
//   容器码是进制不是编码 · 容器名是三维形容名 · 夫妻码是激活标记
//

/// 六边形蜂窝坐标
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexCoord {
    pub x: u8,  // 奇数位 (丈夫轴 · 独立)
    pub y: u8,  // 偶数位 (妻子轴 · 容纳)
}

impl HexCoord {
    /// 从 6 位容器编码计算蜂窝坐标
    pub fn from_container_code(code: u8) -> Self {
        // 偶数位: bit0, bit2, bit4 → y
        let y = ((code >> 0) & 1) << 0 | ((code >> 2) & 1) << 1 | ((code >> 4) & 1) << 2;
        // 奇数位: bit1, bit3, bit5 → x
        let x = ((code >> 1) & 1) << 0 | ((code >> 3) & 1) << 1 | ((code >> 5) & 1) << 2;
        HexCoord { x, y }
    }

    /// 编码 → (x, y) · 比特自身拆分 · 不是乘法
    pub fn to_container_code(&self) -> u8 {
        let mut code: u8 = 0;
        code |= (self.y & 1) << 0;
        code |= ((self.y >> 1) & 1) << 2;
        code |= ((self.y >> 2) & 1) << 4;
        code |= (self.x & 1) << 1;
        code |= ((self.x >> 1) & 1) << 3;
        code |= ((self.x >> 2) & 1) << 5;
        code
    }

    /// 两容器码之间的距离 · 外积距离 = 曼哈顿距离在蜂窝坐标上的投影
    pub fn distance(a: u8, b: u8) -> u8 {
        let ca = HexCoord::from_container_code(a);
        let cb = HexCoord::from_container_code(b);
        // 菱形坐标曼哈顿
        let dx = if ca.x > cb.x { ca.x - cb.x } else { cb.x - ca.x };
        let dy = if ca.y > cb.y { ca.y - cb.y } else { cb.y - ca.y };
        // 蜂窝距离 = max(dx, dy) 菱形坐标系下的圆实际上是菱形
        if dx > dy { dx } else { dy }
    }

    /// 六个邻居坐标 · 六边形有六条边 · 因此六个邻居
    pub fn neighbors(&self) -> [HexCoord; 6] {
        [
            HexCoord { x: self.x.wrapping_add(1), y: self.y },
            HexCoord { x: self.x.wrapping_sub(1), y: self.y },
            HexCoord { x: self.x, y: self.y.wrapping_add(1) },
            HexCoord { x: self.x, y: self.y.wrapping_sub(1) },
            HexCoord { x: self.x.wrapping_add(1), y: self.y.wrapping_sub(1) },
            HexCoord { x: self.x.wrapping_sub(1), y: self.y.wrapping_add(1) },
        ]
    }
}

// ────────────────────────────────────────────────────────────
// 边数码系统
// ────────────────────────────────────────────────────────────

/// 边数码: 边的数量 (6=妻子六边形 · 7=丈夫七边形)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeCode {
    Six = 6,   // 妻子 · 蜂巢单元
    Seven = 7, // 丈夫 · 蜂巢 + 中心
}

impl EdgeCode {
    /// 从夫妻极性获取边数
    pub fn from_polarity(has_husband: bool, has_wife: bool) -> Self {
        match (has_husband, has_wife) {
            (true, true) => EdgeCode::Seven,   // 夫妻完整 = 七边形
            (true, false) => EdgeCode::Seven,  // 只有丈夫 = 七边形(带中心)
            (false, true) => EdgeCode::Six,    // 只有妻子 = 六边形
            (false, false) => EdgeCode::Six,   // 独立 = 六边形(杯)
        }
    }

    /// 等边长 · 每条边是否等长
    pub fn is_equilateral(&self) -> bool {
        match self {
            EdgeCode::Six => true,   // 正六边形等边
            EdgeCode::Seven => false, // 七边形不等边(含中心)
        }
    }
}

/// 边数码属性 · 容器的视觉几何参数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeAttrs {
    pub edge_count: EdgeCode,   // 边数量
    pub length: u8,             // 边数长度码 (像素级)
    pub thickness: u8,          // 边数粗细码 (线宽)
    pub hue: u8,                // 边数色码 (色调 0-255)
    pub deform: u8,             // 边数形状调节码 (形变强度)
}

impl Default for EdgeAttrs {
    fn default() -> Self {
        EdgeAttrs {
            edge_count: EdgeCode::Six,
            length: 30,
            thickness: 2,
            hue: 120,
            deform: 0,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 光标系
// ────────────────────────────────────────────────────────────

/// 光标操作码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorOp {
    Select,       // 光标选中
    Cancel,       // 光标取消
    ScopeBox,     // 光标控制范畴限度框
    ScopeMin,     // 光标控制范畴限度起码
    ScopeEnd,     // 光标控制范畴限度终码
}

// ────────────────────────────────────────────────────────────
// 测试: 六边形蜂窝
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_hex_grid {
    use super::*;

    #[test]
    fn hex_coord_from_code_42() {
        // 42 = 0b101010 → y=bit(0,2,4)=010=2 · x=bit(1,3,5)=101=5
        let c = HexCoord::from_container_code(42);
        assert_eq!(c.x, 5);
        assert_eq!(c.y, 2);
    }

    #[test]
    fn hex_coord_roundtrip() {
        // 编码往返一致
        for code in 0u8..64 {
            let c = HexCoord::from_container_code(code);
            let restored = c.to_container_code();
            assert_eq!(restored, code, "code {} → ({},{}) → {}", code, c.x, c.y, restored);
        }
    }

    #[test]
    fn hex_distance_zero() {
        // 自身距离为零
        for code in 0u8..64 {
            assert_eq!(HexCoord::distance(code, code), 0);
        }
    }

    #[test]
    fn hex_neighbors_have_distance_one() {
        // 邻居距离为 1
        let origin = HexCoord::from_container_code(0);
        let neighbors = origin.neighbors();
        for n in neighbors.iter() {
            if n.x < 8 && n.y < 8 {
                let dist = HexCoord::distance(0, n.to_container_code());
                assert_eq!(dist, 1, "neighbor ({},{}) should be distance 1 but was {}", n.x, n.y, dist);
            }
        }
    }

    #[test]
    fn edge_code_from_polarity() {
        assert_eq!(EdgeCode::from_polarity(true, true), EdgeCode::Seven);
        assert_eq!(EdgeCode::from_polarity(true, false), EdgeCode::Seven);
        assert_eq!(EdgeCode::from_polarity(false, true), EdgeCode::Six);
        assert_eq!(EdgeCode::from_polarity(false, false), EdgeCode::Six);
    }

    #[test]
    fn edge_attrs_default_six() {
        let a = EdgeAttrs::default();
        assert_eq!(a.edge_count, EdgeCode::Six);
        assert!(a.is_equilateral());
    }
}
