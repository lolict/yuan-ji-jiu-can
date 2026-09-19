// ────────────────────────────────────────────────────────────
// 一维串联 — 所有集装箱拆散, 排在一条时间线
// ────────────────────────────────────────────────────────────

/// 一维时间线 — 全局唯一位置 → (家族, 身份)
///
/// 串联公式: 全局序号 = 家族码 × 家族跨距 + 时间线位置
/// 反推: 家族码 = 全局序号 / 家族跨距, 时间线位置 = 全局序号 % 家族跨距
pub struct 一维时间线;

impl 一维时间线 {
    /// 计算某身份的全局唯一序号 (编译期可计算)
    pub const fn 全局序号(家族码: u8, 时间线位置: u16) -> u32 {
        家族码 as u32 * 家族跨距 + 时间线位置 as u32
    }

    /// 从全局序号反推家族码
    pub const fn 家族码(全局序号: u32) -> u8 {
        (全局序号 / 家族跨距) as u8
    }

    /// 从全局序号反推时间线位置
    pub const fn 时间线位置(全局序号: u32) -> u16 {
        (全局序号 % 家族跨距) as u16
    }

    /// 所有身份的全局唯一序号 (遍历所有家族)
    ///
    /// 返回: Vec<(全局序号, 家族名, 身份名)>
    /// 保证每个全局序号唯一, 即便身份名相同也能通过序号区分
    pub fn 全部序号() -> Vec<(u32, &'static str, &'static str)> {
        let mut result = Vec::with_capacity(167);

        // 声母 24
        let sm = 预编译容器::声母();
        for i in 0..sm.总数() {
            let e = &sm.全列()[i];
            result.push((Self::全局序号(sm.家族码, e.时间线), "声母", e.名字));
        }

        // 韵母 39
        let ym = 预编译容器::韵母();
        for i in 0..ym.总数() {
            let e = &ym.全列()[i];
            result.push((Self::全局序号(ym.家族码, e.时间线), "韵母", e.名字));
        }

        // 声调 8
        let sd = 预编译容器::声调();
        for i in 0..sd.总数() {
            let e = &sd.全列()[i];
            result.push((Self::全局序号(sd.家族码, e.时间线), "声调", e.名字));
        }

        // 部首 25
        let bs = 预编译容器::部首();
        for i in 0..bs.总数() {
            let e = &bs.全列()[i];
            result.push((Self::全局序号(bs.家族码, e.时间线), "部首", e.名字));
        }

        // 结构 8
        let jg = 预编译容器::结构();
        for i in 0..jg.总数() {
            let e = &jg.全列()[i];
            result.push((Self::全局序号(jg.家族码, e.时间线), "结构", e.名字));
        }

        // 几何 4
        let jh = 预编译容器::几何();
        for i in 0..jh.总数() {
            let e = &jh.全列()[i];
            result.push((Self::全局序号(jh.家族码, e.时间线), "几何", e.名字));
        }

        // 颜色 7
        let ys = 预编译容器::颜色();
        for i in 0..ys.总数() {
            let e = &ys.全列()[i];
            result.push((Self::全局序号(ys.家族码, e.时间线), "颜色", e.名字));
        }

        // 相位 5
        let xw = 预编译容器::相位();
        for i in 0..xw.总数() {
            let e = &xw.全列()[i];
            result.push((Self::全局序号(xw.家族码, e.时间线), "相位", e.名字));
        }

        // 笔画 8
        let bh = 预编译容器::笔画();
        for i in 0..bh.总数() {
            let e = &bh.全列()[i];
            result.push((Self::全局序号(bh.家族码, e.时间线), "笔画", e.名字));
        }

        // 五笔 25
        let wb = 预编译容器::五笔();
        for i in 0..wb.总数() {
            let e = &wb.全列()[i];
            result.push((Self::全局序号(wb.家族码, e.时间线), "五笔", e.名字));
        }

        // 音乐 12
        let yy = 预编译容器::音乐();
        for i in 0..yy.总数() {
            let e = &yy.全列()[i];
            result.push((Self::全局序号(yy.家族码, e.时间线), "音乐", e.名字));
        }

        // 身份锚点 2 (低位持有者=0, 高位持有者=1)
        let anchor = 预编译容器::身份锚点();
        for i in 0..anchor.总数() {
            let e = &anchor.全列()[i];
            result.push((Self::全局序号(anchor.家族码, e.时间线), "身份锚点", e.名字));
        }

        result
    }

    /// 嵌合 — 两个不同家族的身份串联后, 确认全局唯一
    ///
    /// 即便名字相同, 全局序号不同 → 不同存在
    pub fn 嵌合唯一(家族A码: u8, 身份A线: u16, 家族B码: u8, 身份B线: u16) -> (u32, u32) {
        (Self::全局序号(家族A码, 身份A线), Self::全局序号(家族B码, 身份B线))
    }

    /// 通过全局序号获取命名空间全名
    pub fn 全名(全局序号: u32) -> (&'static str, u16) {
        let fam = Self::家族码(全局序号);
        let pos = Self::时间线位置(全局序号);
        let fam_name = match fam {
            FAMILY_INITIAL => "声母",
            FAMILY_VOWEL => "韵母",
            FAMILY_TONE => "声调",
            FAMILY_RADICAL => "部首",
            FAMILY_STRUCTURE => "结构",
            FAMILY_GEOMETRY => "几何",
            FAMILY_COLOR => "颜色",
            FAMILY_PHASE => "相位",
            FAMILY_STROKE => "笔画",
            FAMILY_WUBI => "五笔",
            FAMILY_MUSIC => "音乐",
            _ => "未知",
        };
        (fam_name, pos)
    }
}
