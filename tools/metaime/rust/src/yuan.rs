//! Yuan · 一元二元三元四元层级解释器
//! 一元=袋 · 二元=乾坤 · 三元=见证裁判 · 四元=包含嵌套

use crate::radix3d::Point3D;

/// 元层级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YuanLevel {
    Yi,    // 一元 (袋)
    Er,    // 二元 (乾坤)
    San,   // 三元 (见证裁判)
    Si,    // 四元 (包含嵌套)
}

/// 元层级解释器
pub struct YuanInterpreter;

impl YuanInterpreter {
    /// 创建解释器
    pub const fn new() -> Self {
        YuanInterpreter
    }

    /// 一元解释: 袋 · 容器本身
    pub const fn yi(index: usize) -> &'static str {
        // 一元就是容器本身
        if index == 0 { "袋" } else { "" }
    }

    /// 二元解释: 乾坤 · 位置(0=乾/阳, 1=坤/阴)
    pub const fn er(index: usize) -> &'static str {
        if index == 0 {
            "乾"  // 阳 · 奇数位
        } else if index == 1 {
            "坤"  // 阴 · 偶数位
        } else {
            ""
        }
    }

    /// 判断位置阴阳
    pub const fn is_yang(position: usize) -> bool {
        position % 2 == 0
    }

    /// 三元解释: 见证裁判 · 位置(见=1, 证=2, 裁=3)
    pub const fn san(position: usize) -> &'static str {
        if position == 1 {
            "见"
        } else if position == 2 {
            "证"
        } else if position == 3 {
            "裁"
        } else {
            ""
        }
    }

    /// 四元解释: 包含嵌套 · 位置(包=1, 含=2, 嵌=3, 套=4)
    pub const fn si(position: usize) -> &'static str {
        if position == 1 {
            "包"
        } else if position == 2 {
            "含"
        } else if position == 3 {
            "嵌"
        } else if position == 4 {
            "套"
        } else {
            ""
        }
    }

    /// 判断符号属于哪一元
    pub const fn classify(symbol: &str) -> &str {
        // 一元
        if const_str_eq_yi(symbol) {
            "一元";
            return "一元";
        }
        // 二元
        if const_str_eq_er(symbol) {
            return "二元";
        }
        // 三元
        if const_str_eq_san(symbol) {
            return "三元";
        }
        // 四元
        if const_str_eq_si(symbol) {
            return "四元";
        }
        "未知"
    }

    /// 层级序列: 从一元到四元的层级序列
    pub const fn hierarchy(position: usize) -> &'static str {
        if position == 0 {
            "零"
        } else if position == 1 {
            "一"
        } else if position == 2 {
            "二"
        } else if position == 3 {
            "三"
        } else if position == 4 {
            "四"
        } else if position == 5 {
            "五"
        } else if position == 6 {
            "六"
        } else if position == 7 {
            "七"
        } else if position == 8 {
            "八"
        } else if position == 9 {
            "九"
        } else {
            ""
        }
    }

    /// 裁决: 根据三元位置判断裁决结果
    pub const fn judge(position: usize) -> &'static str {
        // 三元见证裁判: 位置决定裁决
        if position == 1 {
            "见"  // 见到 = 可见 = 通过
        } else if position == 2 {
            "证"  // 证明 = 可证 = 有效
        } else if position == 3 {
            "裁"  // 裁定 = 裁决 = 终审
        } else {
            ""   // 无裁决
        }
    }

    /// 升级操作: 从低级元到高级元的升级路径
    pub const fn upgrade(position: usize) -> &'static str {
        if position == 1 {
            "一→二"  // 袋→乾坤
        } else if position == 2 {
            "二→三"  // 乾坤→见证
        } else if position == 3 {
            "三→四"  // 见证→包含
        } else {
            ""
        }
    }

    /// 降级操作: 从高级元到低级元的降级路径
    pub const fn downgrade(position: usize) -> &'static str {
        if position == 4 {
            "四→三"  // 包含→见证
        } else if position == 3 {
            "三→二"  // 见证→乾坤
        } else if position == 2 {
            "二→一"  // 乾坤→袋
        } else {
            ""
        }
    }

    /// 根据坐标点推算元层级
    pub const fn classify_point(point: Point3D) -> YuanLevel {
        if point.x == 0 && point.y == 0 && point.z == 0 {
            YuanLevel::Yi
        } else if point.z == 0 {
            YuanLevel::Er
        } else if point.z == 1 || point.z == 2 {
            YuanLevel::San
        } else {
            YuanLevel::Si
        }
    }
}

/// 编译时字符串比较辅助函数
const fn const_str_eq_yi(s: &str) -> bool {
    let bytes = s.as_bytes();
    // "袋"
    bytes.len() == 3 && bytes[0] == 231 && bytes[1] == 210 && bytes[2] == 232
}

const fn const_str_eq_er(s: &str) -> bool {
    let bytes = s.as_bytes();
    // "乾" or "坤"
    if bytes.len() != 3 { return false; }
    (bytes[0] == 229 && bytes[1] == 189 && bytes[2] == 191) ||  // 乾
    (bytes[0] == 229 && bytes[1] == 150 && bytes[2] == 168)     // 坤
}

const fn const_str_eq_san(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 3 { return false; }
    // "见" or "证" or "裁"
    (bytes[0] == 232 && bytes[1] == 167 && bytes[2] == 129) ||  // 见
    (bytes[0] == 232 && bytes[1] == 175 && bytes[2] == 129) ||  // 证
    (bytes[0] == 232 && bytes[1] == 163 && bytes[2] == 129)     // 裁
}

const fn const_str_eq_si(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 3 { return false; }
    // "包" or "含" or "嵌" or "套"
    (bytes[0] == 229 && bytes[1] == 140 && bytes[2] == 133) ||  // 包
    (bytes[0] == 229 && bytes[1] == 144 && bytes[2] == 171) ||  // 含
    (bytes[0] == 229 && bytes[1] == 181 && bytes[2] == 133) ||  // 嵌
    (bytes[0] == 229 && bytes[1] == 165 && bytes[2] == 155)     // 套
}

/// 静态解释器实例
pub static DEFAULT_YUAN: YuanInterpreter = YuanInterpreter::new();
