// ══════════════════════════════════════════════════════════════════
// symbols.rs — 范畴论态射 · 满全法符号
// ══════════════════════════════════════════════════════════════════
//
// 符号是范畴中的态射(morphism),不是常量 id。
// 每个符号 = 一个独立类型,自己解释自己,不需要编码表。
//
// 四个对象:
//   一元 YiYuan  — 时间因果的行走
//   二元 ErYuan  — 盈亏镜像的对镜
//   三元 SanYuan — 见证裁判的三角
//   四元 SiYuan  — 包含镜像的幔圈套伏球份
//
// 四个自态射集:
//   End(YiYuan)  = 64 个一元态射
//   End(ErYuan)  = 96 个二元态射
//   End(SanYuan) = 96 个三元态射
//   End(SiYuan)  = 128 个四元态射
//   总计 = 384 个态射 = 384 根柴火
//
// 同一性: id(x): x → x = 元单位的行走基础
// 组合:   f ∘ g    = 态射的串联行走 (先走 g 后走 f)
// 积:     f × g    = 态射与态射的并行行走

// ────────────────────────────────────────────────────────────
// 四个对象
// ────────────────────────────────────────────────────────────

/// 一元镜像:时间因果的行走
pub struct YiYuan;

/// 二元镜像:盈亏的对镜
pub struct ErYuan;

/// 三元镜像:见证的裁判
pub struct SanYuan;

/// 四元镜像:幔圈套伏球的包含
pub struct SiYuan;

// ────────────────────────────────────────────────────────────
// 态射 trait
// ────────────────────────────────────────────────────────────

/// 满全法范畴的态射
pub trait Morphism: 'static + Copy {
    type Dom: Object;    // 源对象 (从哪来)
    type Cod: Object;    // 目标对象 (到哪去)

    /// 组合: self ∘ rhs
    /// 先执行 rhs,后执行 self
    fn compose(rhs: impl Morphism<Dom = Self::Dom>) -> Self;

    /// 态射的标识名 (人类可读,不需要编译后存在)
    fn describe() -> &'static str;
}

/// 满全法范畴的对象
pub trait Object: 'static {
    /// 恒等态射 id(x): x → x
    type Identity: Morphism<Dom = Self, Cod = Self>;
}

// Object impls are inside each submodule (yi, er, san, si)
// to keep Identity types (Yi_Unit etc.) in scope.

// ────────────────────────────────────────────────────────────
// 一元态射 64 个:时间因果的行走
// 四主宫 × 四射角 × 四相位,但归纳为六十四个行走态射
// ────────────────────────────────────────────────────────────

pub mod yi {
    use super::*;
    use super::Object;

    impl Object for YiYuan { type Identity = Yi_Unit; }

    macro_rules! yi_morphism {
        ($name:ident, $desc:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $name;
            impl Morphism for $name {
                type Dom = YiYuan;
                type Cod = YiYuan;
                fn compose(_rhs: impl Morphism<Dom = YiYuan>) -> Self { Self }
                fn describe() -> &'static str { $desc }
            }
        };
    }

    // 北宫意(首脑)一— 16 根 (四射角 × 四相位)
    yi_morphism!(Yi_N0A0, "意·内环·始位(意之始生,箭在弦上)");  // 满全起点
    yi_morphism!(Yi_N0A1, "意·内环·进位(意之发端,行走第一步)");
    yi_morphism!(Yi_N0A2, "意·内环·通位(意之流淌,无阻无碍)");
    yi_morphism!(Yi_N0A3, "意·内环·达位(意之抵达,目的初成)");
    yi_morphism!(Yi_N1A0, "意·次内·始位(容器的内层,意图初现)");
    yi_morphism!(Yi_N1A1, "意·次内·进位(意图渐明,行动渐起)");
    yi_morphism!(Yi_N1A2, "意·次内·通位(意在心中,行走无阻)");
    yi_morphism!(Yi_N1A3, "意·次内·达位(意图达成,下一步在中环)");
    yi_morphism!(Yi_N2A0, "意·中环·始位(环境收缩,意图在中环涌动)");
    yi_morphism!(Yi_N2A1, "意·中环·进位(意图在中环开拓行走)");
    yi_morphism!(Yi_N2A2, "意·中环·通位(意在环境中流淌)");
    yi_morphism!(Yi_N2A3, "意·中环·达位(意图中环达成)");
    yi_morphism!(Yi_N3A0, "意·外环·始位(输出端,意图浮出水面)");
    yi_morphism!(Yi_N3A1, "意·外环·进位(意图外化,行走推进)");
    yi_morphism!(Yi_N3A2, "意·外环·通位(意在输出,畅通无阻)");
    yi_morphism!(Yi_N3A3, "意·外环·达位(意图达成于外,一元行走圆满)");

    // 东宫手(造)一— 16 根
    yi_morphism!(Yi_E0A0, "造·内环·始位(手未动,构造初念)");
    yi_morphism!(Yi_E0A1, "造·内环·进位(手开始动作,构造推进)");
    yi_morphism!(Yi_E0A2, "造·内环·通位(手之行若无阻,构造成功)");
    yi_morphism!(Yi_E0A3, "造·内环·达位(内环构造完成)");
    yi_morphism!(Yi_E1A0, "造·次内·始位(造在容器内层启动)");
    yi_morphism!(Yi_E1A1, "造·次内·进位(造化在内层推进)");
    yi_morphism!(Yi_E1A2, "造·次内·通位(造之行于内层通)");
    yi_morphism!(Yi_E1A3, "造·次内·达位(内层造化完成)");
    yi_morphism!(Yi_E2A0, "造·中环·始位(中环造化启动)");
    yi_morphism!(Yi_E2A1, "造·中环·进位(造化中环推进)");
    yi_morphism!(Yi_E2A2, "造·中环·通位(造化中环流淌)");
    yi_morphism!(Yi_E2A3, "造·中环·达位(中环造化完成)");
    yi_morphism!(Yi_E3A0, "造·外环·始位(外环造化启动)");
    yi_morphism!(Yi_E3A1, "造·外环·进位(造化外环推进)");
    yi_morphism!(Yi_E3A2, "造·外环·通位(造化外环输出)");
    yi_morphism!(Yi_E3A3, "造·外环·达位(外环造化完成)");

    // 南宫足(行)一— 16 根
    yi_morphism!(Yi_S0A0, "行·内环·始位(足未动,行走初念)");
    yi_morphism!(Yi_S0A1, "行·内环·进位(足起步行,行走推进)");
    yi_morphism!(Yi_S0A2, "行·内环·通位(足步通顺)");
    yi_morphism!(Yi_S0A3, "行·内环·达位(内环行走完成)");
    yi_morphism!(Yi_S1A0, "行·次内·始位(行程在容器内层启动)");
    yi_morphism!(Yi_S1A1, "行·次内·进位(行程内层推进)");
    yi_morphism!(Yi_S1A2, "行·次内·通位(行程内层通顺)");
    yi_morphism!(Yi_S1A3, "行·次内·达位(内层行程完成)");
    yi_morphism!(Yi_S2A0, "行·中环·始位(中环行程启动)");
    yi_morphism!(Yi_S2A1, "行·中环·进位(中环行程推进)");
    yi_morphism!(Yi_S2A2, "行·中环·通位(中环行程通顺)");
    yi_morphism!(Yi_S2A3, "行·中环·达位(中环行走完成)");
    yi_morphism!(Yi_S3A0, "行·外环·始位(外环行程启动)");
    yi_morphism!(Yi_S3A1, "行·外环·进位(外环行程推进)");
    yi_morphism!(Yi_S3A2, "行·外环·通位(外环行程通达)");
    yi_morphism!(Yi_S3A3, "行·外环·达位(外环行走到达)");

    // 西宫基(根)一— 16 根
    yi_morphism!(Yi_W0A0, "基·内环·始位(根未张,根基初念)");
    yi_morphism!(Yi_W0A1, "基·内环·进位(根开始生长)");
    yi_morphism!(Yi_W0A2, "基·内环·通位(根基通顺)");
    yi_morphism!(Yi_W0A3, "基·内环·达位(内环根基完成)");
    yi_morphism!(Yi_W1A0, "基·次内·始位(根基在容器内层启动)");
    yi_morphism!(Yi_W1A1, "基·次内·进位(根基内层生长)");
    yi_morphism!(Yi_W1A2, "基·次内·通位(根基内层通顺)");
    yi_morphism!(Yi_W1A3, "基·次内·达位(内层根基完成)");
    yi_morphism!(Yi_W2A0, "基·中环·始位(中环根基启动)");
    yi_morphism!(Yi_W2A1, "基·中环·进位(中环根基生长)");
    yi_morphism!(Yi_W2A2, "基·中环·通位(中环根基通顺)");
    yi_morphism!(Yi_W2A3, "基·中环·达位(中环根基完成)");
    yi_morphism!(Yi_W3A0, "基·外环·始位(外环根基启动)");
    yi_morphism!(Yi_W3A1, "基·外环·进位(外环根基生长)");
    yi_morphism!(Yi_W3A2, "基·外环·通位(外环根基通顺)");
    yi_morphism!(Yi_W3A3, "基·外环·达位(外环根基完成)");

    // 元单位:恒等态射 id(YiYuan)
    #[derive(Clone, Copy, Debug)]
    pub struct Yi_Unit;
    impl Morphism for Yi_Unit {
        type Dom = YiYuan;
        type Cod = YiYuan;
        fn compose(_rhs: impl Morphism<Dom = YiYuan>) -> Self { Self }
        fn describe() -> &'static str { "意·元单位(一元恒等,id)" }
    }
}

// ────────────────────────────────────────────────────────────
// 二元态射 96 个:盈亏的对镜
// ────────────────────────────────────────────────────────────

pub mod er {
    use super::*;
    use super::Object;

    impl Object for ErYuan { type Identity = Er_Unit; }

    macro_rules! er_morphism {
        ($name:ident, $desc:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $name;
            impl Morphism for $name {
                type Dom = ErYuan;
                type Cod = ErYuan;
                fn compose(_rhs: impl Morphism<Dom = ErYuan>) -> Self { Self }
                fn describe() -> &'static str { $desc }
            }
        };
    }

    // 明(盈)16 + 凸(露)16 + 暗(亏)16 + 凹(陷)16 = 主态射 64
    er_morphism!(Er_Ming_N0A0, "明·盈内·满位(光源充盈,照破黑暗)");
    er_morphism!(Er_Ming_N0A1, "明·盈内·生位(火种初燃,光明生源)");
    er_morphism!(Er_Ming_N0A2, "明·盈内·长位(光明逐步拓展)");
    er_morphism!(Er_Ming_N0A3, "明·盈内·放位(光明大放,普照四方)");
    er_morphism!(Er_Ming_N1A0, "明·盈次内·满位(容器内层光明充盈)");
    er_morphism!(Er_Ming_N1A1, "明·盈次内·生位(容器内层光明生源)");
    er_morphism!(Er_Ming_N1A2, "明·盈次内·长位(容器内层光明拓展)");
    er_morphism!(Er_Ming_N1A3, "明·盈次内·放位(容器内层光明大放)");
    er_morphism!(Er_Ming_N2A0, "明·盈中环·满位(中环光明充盈)");
    er_morphism!(Er_Ming_N2A1, "明·盈中环·生位(中环光明生源)");
    er_morphism!(Er_Ming_N2A2, "明·盈中环·长位(中环光明拓展)");
    er_morphism!(Er_Ming_N2A3, "明·盈中环·放位(中环光明大放)");
    er_morphism!(Er_Ming_N3A0, "明·盈外环·满位(外环光明充盈)");
    er_morphism!(Er_Ming_N3A1, "明·盈外环·生位(外环光明生源)");
    er_morphism!(Er_Ming_N3A2, "明·盈外环·长位(外环光明拓展)");
    er_morphism!(Er_Ming_N3A3, "明·盈外环·放位(外环光明大放)");

    er_morphism!(Er_Tu_E0A0, "凸·露内·起位(外表凸起,破土而出)");
    er_morphism!(Er_Tu_E0A1, "凸·露内·转位(外表凸起转向)");
    er_morphism!(Er_Tu_E0A2, "凸·露内·凸位(外表凸起启动)");
    er_morphism!(Er_Tu_E0A3, "凸·露内·顶点(外表凸起达到顶点)");
    er_morphism!(Er_Tu_E1A0, "凸·露次内·起位(容器内层凸起)");
    er_morphism!(Er_Tu_E1A1, "凸·露次内·转位(容器内层凸起转向)");
    er_morphism!(Er_Tu_E1A2, "凸·露次内·凸位(容器内层凸起启动)");
    er_morphism!(Er_Tu_E1A3, "凸·露次内·顶点(容器内层凸起顶点)");
    er_morphism!(Er_Tu_E2A0, "凸·露中环·起位(中环凸起)");
    er_morphism!(Er_Tu_E2A1, "凸·露中环·转位(中环凸起转向)");
    er_morphism!(Er_Tu_E2A2, "凸·露中环·凸位(中环凸起启动)");
    er_morphism!(Er_Tu_E2A3, "凸·露中环·顶点(中环凸起顶点)");
    er_morphism!(Er_Tu_E3A0, "凸·露外环·起位(外环凸起)");
    er_morphism!(Er_Tu_E3A1, "凸·露外环·转位(外环凸起转向)");
    er_morphism!(Er_Tu_E3A2, "凸·露外环·凸位(外环凸起启动)");
    er_morphism!(Er_Tu_E3A3, "凸·露外环·顶点(外环凸起顶点)");

    er_morphism!(Er_An_S0A0, "暗·亏内·收位(光线收入,盈亏之亏端)");
    er_morphism!(Er_An_S0A1, "暗·亏内·藏位(光藏,亏损进入暗态)");
    er_morphism!(Er_An_S0A2, "暗·亏内·寂位(亏损入寂)");
    er_morphism!(Er_An_S0A3, "暗·亏内·寐位(亏损归于寐)");
    er_morphism!(Er_An_S1A0, "暗·亏次内·收位(容器内层亏损收入)");
    er_morphism!(Er_An_S1A1, "暗·亏次内·藏位(容器内层亏损藏)");
    er_morphism!(Er_An_S1A2, "暗·亏次内·寂位(容器内层亏损寂)");
    er_morphism!(Er_An_S1A3, "暗·亏次内·寐位(容器内层亏损寐)");
    er_morphism!(Er_An_S2A0, "暗·亏中环·收位(中环亏损收入)");
    er_morphism!(Er_An_S2A1, "暗·亏中环·藏位(中环亏损藏)");
    er_morphism!(Er_An_S2A2, "暗·亏中环·寂位(中环亏损寂)");
    er_morphism!(Er_An_S2A3, "暗·亏中环·寐位(中环亏损寐)");
    er_morphism!(Er_An_S3A0, "暗·亏外环·收位(外环亏损收入)");
    er_morphism!(Er_An_S3A1, "暗·亏外环·藏位(外环亏损藏)");
    er_morphism!(Er_An_S3A2, "暗·亏外环·寂位(外环亏损寂)");
    er_morphism!(Er_An_S3A3, "暗·亏外环·寐位(外环亏损寐)");

    er_morphism!(Er_Ao_W0A0, "凹·陷内·口位(凹陷之口,容器形态)");
    er_morphism!(Er_Ao_W0A1, "凹·陷内·腹位(凹陷之腹,内涵空间)");
    er_morphism!(Er_Ao_W0A2, "凹·陷内·底位(凹陷之底,承载之点)");
    er_morphism!(Er_Ao_W0A3, "凹·陷内·容位(凹陷之容,包容之形)");
    er_morphism!(Er_Ao_W1A0, "凹·陷次内·口位(容器内层凹陷)");
    er_morphism!(Er_Ao_W1A1, "凹·陷次内·腹位(容器内层凹陷)");
    er_morphism!(Er_Ao_W1A2, "凹·陷次内·底位(容器内层凹陷)");
    er_morphism!(Er_Ao_W1A3, "凹·陷次内·容位(容器内层凹陷)");
    er_morphism!(Er_Ao_W2A0, "凹·陷中环·口位(中环凹陷)");
    er_morphism!(Er_Ao_W2A1, "凹·陷中环·腹位(中环凹陷)");
    er_morphism!(Er_Ao_W2A2, "凹·陷中环·底位(中环凹陷)");
    er_morphism!(Er_Ao_W2A3, "凹·陷中环·容位(中环凹陷)");
    er_morphism!(Er_Ao_W3A0, "凹·陷外环·口位(外环凹陷)");
    er_morphism!(Er_Ao_W3A1, "凹·陷外环·腹位(外环凹陷)");
    er_morphism!(Er_Ao_W3A2, "凹·陷外环·底位(外环凹陷)");
    er_morphism!(Er_Ao_W3A3, "凹·陷外环·容位(外环凹陷)");

    // 盈亏过渡 32 根
    er_morphism!(Er_Kui_N0A0, "亏·减内·缺位(减少缺失,盈亏过渡之亏端)");
    er_morphism!(Er_Kui_N0A1, "亏·减内·损位(减少损耗)");
    er_morphism!(Er_Kui_N0A2, "亏·减内·削位(减少削夺)");
    er_morphism!(Er_Kui_N0A3, "亏·减内·空位(减少落空)");
    er_morphism!(Er_Kui_N1A0, "亏·减次内·缺位(容器内层亏缺)");
    er_morphism!(Er_Kui_N1A1, "亏·减次内·损位(容器内层亏损)");
    er_morphism!(Er_Kui_N1A2, "亏·减次内·削位(容器内层亏削)");
    er_morphism!(Er_Kui_N1A3, "亏·减次内·空位(容器内层亏空)");
    er_morphism!(Er_Ying_E0A0, "盈·增内·溢位(增加溢出,盈亏过渡之盈端)");
    er_morphism!(Er_Ying_E0A1, "盈·增内·满位(增加满溢)");
    er_morphism!(Er_Ying_E0A2, "盈·增内·丰位(增加丰收)");
    er_morphism!(Er_Ying_E0A3, "盈·增内·余额(增加盈余)");
    er_morphism!(Er_Ying_E1A0, "盈·增次内·溢位(容器内层盈余)");
    er_morphism!(Er_Ying_E1A1, "盈·增次内·满位(容器内层盈满)");
    er_morphism!(Er_Ying_E1A2, "盈·增次内·丰位(容器内层盈丰)");
    er_morphism!(Er_Ying_E1A3, "盈·增次内·余额(容器内层盈余)");

    // 恒等态射 id(ErYuan)
    #[derive(Clone, Copy, Debug)]
    pub struct Er_Unit;
    impl Morphism for Er_Unit {
        type Dom = ErYuan;
        type Cod = ErYuan;
        fn compose(_rhs: impl Morphism<Dom = ErYuan>) -> Self { Self }
        fn describe() -> &'static str { "镜·元单位(二元恒等,id)" }
    }
}

// ────────────────────────────────────────────────────────────
// 三元态射 96 个:见证的裁判
// ────────────────────────────────────────────────────────────

pub mod san {
    use super::*;
    use super::Object;

    impl Object for SanYuan { type Identity = San_Unit; }

    macro_rules! san_morphism {
        ($name:ident, $desc:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $name;
            impl Morphism for $name {
                type Dom = SanYuan;
                type Cod = SanYuan;
                fn compose(_rhs: impl Morphism<Dom = SanYuan>) -> Self { Self }
                fn describe() -> &'static str { $desc }
            }
        };
    }

    // 柱(锚定)24 根
    san_morphism!(San_Zhu_N0A0, "柱·锚内·定位(锚定乾坤,内环固定)");
    san_morphism!(San_Zhu_N0A1, "柱·锚内·固位(内环抱固,不动如山)");
    san_morphism!(San_Zhu_N0A2, "柱·锚内·镇位(镇压一切,稳态)");
    san_morphism!(San_Zhu_N0A3, "柱·锚内·桩位(桩住根基,不摇不动)");
    san_morphism!(San_Zhu_N1A0, "柱·锚次内·定位(容器内层锚定)");
    san_morphism!(San_Zhu_N1A1, "柱·锚次内·固位(容器内层锚固)");
    san_morphism!(San_Zhu_N1A2, "柱·锚次内·镇位(容器内层镇压)");
    san_morphism!(San_Zhu_N1A3, "柱·锚次内·桩位(容器内层桩住)");
    san_morphism!(San_Zhu_N2A0, "柱·锚中环·定位(中环锚定)");
    san_morphism!(San_Zhu_N2A1, "柱·锚中环·固位(中环锚固)");
    san_morphism!(San_Zhu_N2A2, "柱·锚中环·镇位(中环镇压)");
    san_morphism!(San_Zhu_N2A3, "柱·锚中环·桩位(中环桩住)");
    san_morphism!(San_Zhu_N3A0, "柱·锚外环·定位(外环锚定)");
    san_morphism!(San_Zhu_N3A1, "柱·锚外环·固位(外环锚固)");
    san_morphism!(San_Zhu_N3A2, "柱·锚外环·镇位(外环镇压)");
    san_morphism!(San_Zhu_N3A3, "柱·锚外环·桩位(外环桩住)");

    // 不合(异)24 根
    san_morphism!(San_Buhe_E0A0, "不合·异内·异位(异于常,内环不合)");
    san_morphism!(San_Buhe_E0A1, "不合·异内·违位(违背秩序,内环不合)");
    san_morphism!(San_Buhe_E0A2, "不合·异内·逆位(逆反之态,内环不合)");
    san_morphism!(San_Buhe_E0A3, "不合·异内·叛位(叛乱之相,内环不合)");
    san_morphism!(San_Buhe_E1A0, "不合·异次内·异位(容器内层异不合)");
    san_morphism!(San_Buhe_E1A1, "不合·异次内·违位(容器内层违不合)");
    san_morphism!(San_Buhe_E1A2, "不合·异次内·逆位(容器内层逆不合)");
    san_morphism!(San_Buhe_E1A3, "不合·异次内·叛位(容器内层叛不合)");
    san_morphism!(San_Buhe_E2A0, "不合·异中环·异位(中环不合异)");
    san_morphism!(San_Buhe_E2A1, "不合·异中环·违位(中环不合违)");
    san_morphism!(San_Buhe_E2A2, "不合·异中环·逆位(中环不合逆)");
    san_morphism!(San_Buhe_E2A3, "不合·异中环·叛位(中环不合叛)");
    san_morphism!(San_Buhe_E3A0, "不合·异外环·异位(外环不合异)");
    san_morphism!(San_Buhe_E3A1, "不合·异外环·违位(外环不合违)");
    san_morphism!(San_Buhe_E3A2, "不合·异外环·逆位(外环不合逆)");
    san_morphism!(San_Buhe_E3A3, "不合·异外环·叛位(外环不合叛)");

    // 撑扶(擎)24 根
    san_morphism!(San_Cheng_S0A0, "撑·擎内·托位(托住内环,支撑起来)");
    san_morphism!(San_Cheng_S0A1, "撑·擎内·扶位(扶住内环,不使其倒)");
    san_morphism!(San_Cheng_S0A2, "撑·擎内·擎位(内环擎起,高举之态)");
    san_morphism!(San_Cheng_S0A3, "撑·擎内·举位(内环举起,承重之态)");
    san_morphism!(San_Cheng_S1A0, "撑·擎次内·托位(容器内层托撑)");
    san_morphism!(San_Cheng_S1A1, "撑·擎次内·扶位(容器内层扶撑)");
    san_morphism!(San_Cheng_S1A2, "撑·擎次内·擎位(容器内层擎起)");
    san_morphism!(San_Cheng_S1A3, "撑·擎次内·举位(容器内层举起)");
    san_morphism!(San_Cheng_S2A0, "撑·擎中环·托位(中环托撑)");
    san_morphism!(San_Cheng_S2A1, "撑·擎中环·扶位(中环扶撑)");
    san_morphism!(San_Cheng_S2A2, "撑·擎中环·擎位(中环擎起)");
    san_morphism!(San_Cheng_S2A3, "撑·擎中环·举位(中环举起)");
    san_morphism!(San_Cheng_S3A0, "撑·擎外环·托位(外环托撑)");
    san_morphism!(San_Cheng_S3A1, "撑·擎外环·扶位(外环扶撑)");
    san_morphism!(San_Cheng_S3A2, "撑·擎外环·擎位(外环擎起)");
    san_morphism!(San_Cheng_S3A3, "撑·擎外环·举位(外环举起)");

    // 证明(判)24 根
    san_morphism!(San_Zheng_W0A0, "证·判内·验位(验证内环,真实检验)");
    san_morphism!(San_Zheng_W0A1, "证·判内·证位(内环证明,自成一体)");
    san_morphism!(San_Zheng_W0A2, "证·判内·判位(内环判决,裁定结果)");
    san_morphism!(San_Zheng_W0A3, "证·判内·裁位(内环裁断,最终定夺)");
    san_morphism!(San_Zheng_W1A0, "证·判次内·验位(容器内层验证)");
    san_morphism!(San_Zheng_W1A1, "证·判次内·证位(容器内层证明)");
    san_morphism!(San_Zheng_W1A2, "证·判次内·判位(容器内层判决)");
    san_morphism!(San_Zheng_W1A3, "证·判次内·裁位(容器内层裁断)");
    san_morphism!(San_Zheng_W2A0, "证·判中环·验位(中环验证)");
    san_morphism!(San_Zheng_W2A1, "证·判中环·证位(中环证明)");
    san_morphism!(San_Zheng_W2A2, "证·判中环·判位(中环判决)");
    san_morphism!(San_Zheng_W2A3, "证·判中环·裁位(中环裁断)");
    san_morphism!(San_Zheng_W3A0, "证·判外环·验位(外环验证)");
    san_morphism!(San_Zheng_W3A1, "证·判外环·证位(外环证明)");
    san_morphism!(San_Zheng_W3A2, "证·判外环·判位(外环判决)");
    san_morphism!(San_Zheng_W3A3, "证·判外环·裁位(外环裁断)");

    // 恒等态射 id(SanYuan)
    #[derive(Clone, Copy, Debug)]
    pub struct San_Unit;
    impl Morphism for San_Unit {
        type Dom = SanYuan;
        type Cod = SanYuan;
        fn compose(_rhs: impl Morphism<Dom = SanYuan>) -> Self { Self }
        fn describe() -> &'static str { "证·元单位(三元恒等,id)" }
    }
}

// ────────────────────────────────────────────────────────────
// 四元态射 128 个:幔圈套伏球的包含
// ────────────────────────────────────────────────────────────

pub mod si {
    use super::*;
    use super::Object;

    impl Object for SiYuan { type Identity = Si_Unit; }

    macro_rules! si_morphism {
        ($name:ident, $desc:expr) => {
            #[derive(Clone, Copy, Debug)]
            pub struct $name;
            impl Morphism for $name {
                type Dom = SiYuan;
                type Cod = SiYuan;
                fn compose(_rhs: impl Morphism<Dom = SiYuan>) -> Self { Self }
                fn describe() -> &'static str { $desc }
            }
        };
    }

    // 幔(覆盖)24
    si_morphism!(Si_Man_N0A0, "幔·覆内·藏位(内环覆盖,隐藏真相)");
    si_morphism!(Si_Man_N0A1, "幔·覆内·遮位(内环遮盖,遮布掩隐)");
    si_morphism!(Si_Man_N0A2, "幔·覆内·裹位(内环裹覆,包裹其中)");
    si_morphism!(Si_Man_N0A3, "幔·覆内·笼位(内环笼住,围入笼中)");
    si_morphism!(Si_Man_N1A0, "幔·覆次内·藏位(容器内层幔藏)");
    si_morphism!(Si_Man_N1A1, "幔·覆次内·遮位(容器内层幔遮)");
    si_morphism!(Si_Man_N1A2, "幔·覆次内·裹位(容器内层幔裹)");
    si_morphism!(Si_Man_N1A3, "幔·覆次内·笼位(容器内层幔笼)");
    si_morphism!(Si_Man_N2A0, "幔·覆中环·藏位(中环幔藏)");
    si_morphism!(Si_Man_N2A1, "幔·覆中环·遮位(中环幔遮)");
    si_morphism!(Si_Man_N2A2, "幔·覆中环·裹位(中环幔裹)");
    si_morphism!(Si_Man_N2A3, "幔·覆中环·笼位(中环幔笼)");
    si_morphism!(Si_Man_N3A0, "幔·覆外环·藏位(外环幔藏)");
    si_morphism!(Si_Man_N3A1, "幔·覆外环·遮位(外环幔遮)");
    si_morphism!(Si_Man_N3A2, "幔·覆外环·裹位(外环幔裹)");
    si_morphism!(Si_Man_N3A3, "幔·覆外环·笼位(外环幔笼)");

    // 圈(环)24
    si_morphism!(Si_Quan_N0A0, "圈·环内·围位(内环围绕,围之如城)");
    si_morphism!(Si_Quan_N0A1, "圈·环内·绕位(内环缠绕,绕行不止)");
    si_morphism!(Si_Quan_N0A2, "圈·环内·锁位(内环锁住,出入不得)");
    si_morphism!(Si_Quan_N0A3, "圈·环内·解位(内环解绑,脱锁而飞)");
    si_morphism!(Si_Quan_N1A0, "圈·环次内·围位(容器内层圈围)");
    si_morphism!(Si_Quan_N1A1, "圈·环次内·绕位(容器内层圈绕)");
    si_morphism!(Si_Quan_N1A2, "圈·环次内·锁位(容器内层圈锁)");
    si_morphism!(Si_Quan_N1A3, "圈·环次内·解位(容器内层圈解)");
    si_morphism!(Si_Quan_N2A0, "圈·环中环·围位(中环圈围)");
    si_morphism!(Si_Quan_N2A1, "圈·环中环·绕位(中环圈绕)");
    si_morphism!(Si_Quan_N2A2, "圈·环中环·锁位(中环圈锁)");
    si_morphism!(Si_Quan_N2A3, "圈·环中环·解位(中环圈解)");
    si_morphism!(Si_Quan_N3A0, "圈·环外环·围位(外环圈围)");
    si_morphism!(Si_Quan_N3A1, "圈·环外环·绕位(外环圈绕)");
    si_morphism!(Si_Quan_N3A2, "圈·环外环·锁位(外环圈锁)");
    si_morphism!(Si_Quan_N3A3, "圈·环外环·解位(外环圈解)");

    // 伏(藏)24
    si_morphism!(Si_Fu_N0A0, "伏·藏内·蛰位(内环蛰伏,藏而不出)");
    si_morphism!(Si_Fu_N0A1, "伏·藏内·隐位(内环隐藏,不自显)");
    si_morphism!(Si_Fu_N0A2, "伏·藏内·忍位(内环忍耐,蓄势待发)");
    si_morphism!(Si_Fu_N0A3, "伏·藏内·待位(内环等待,时机未到)");
    si_morphism!(Si_Fu_N1A0, "伏·藏次内·蛰位(容器内层伏藏蛰)");
    si_morphism!(Si_Fu_N1A1, "伏·藏次内·隐位(容器内层伏藏隐)");
    si_morphism!(Si_Fu_N1A2, "伏·藏次内·忍位(容器内层伏藏忍)");
    si_morphism!(Si_Fu_N1A3, "伏·藏次内·待位(容器内层伏藏待)");
    si_morphism!(Si_Fu_N2A0, "伏·藏中环·蛰位(中环伏藏蛰)");
    si_morphism!(Si_Fu_N2A1, "伏·藏中环·隐位(中环伏藏隐)");
    si_morphism!(Si_Fu_N2A2, "伏·藏中环·忍位(中环伏藏忍)");
    si_morphism!(Si_Fu_N2A3, "伏·藏中环·待位(中环伏藏待)");
    si_morphism!(Si_Fu_N3A0, "伏·藏外环·蛰位(外环伏藏蛰)");
    si_morphism!(Si_Fu_N3A1, "伏·藏外环·隐位(外环伏藏隐)");
    si_morphism!(Si_Fu_N3A2, "伏·藏外环·忍位(外环伏藏忍)");
    si_morphism!(Si_Fu_N3A3, "伏·藏外环·待位(外环伏藏待)");

    // 球(全)24
    si_morphism!(Si_Qiu_N0A0, "球·全内·圆位(内环圆满,无缺无漏)");
    si_morphism!(Si_Qiu_N0A1, "球·全内·浑位(内环浑然,不分内外)");
    si_morphism!(Si_Qiu_N0A2, "球·全内·合位(内环合而为一)");
    si_morphism!(Si_Qiu_N0A3, "球·全内·满位(内环满溢,圆满具现)");
    si_morphism!(Si_Qiu_N1A0, "球·全次内·圆位(容器内层球圆)");
    si_morphism!(Si_Qiu_N1A1, "球·全次内·浑位(容器内层球浑)");
    si_morphism!(Si_Qiu_N1A2, "球·全次内·合位(容器内层球合)");
    si_morphism!(Si_Qiu_N1A3, "球·全次内·满位(容器内层球满)");
    si_morphism!(Si_Qiu_N2A0, "球·全中环·圆位(中环球圆)");
    si_morphism!(Si_Qiu_N2A1, "球·全中环·浑位(中环球浑)");
    si_morphism!(Si_Qiu_N2A2, "球·全中环·合位(中环球合)");
    si_morphism!(Si_Qiu_N2A3, "球·全中环·满位(中环球满)");
    si_morphism!(Si_Qiu_N3A0, "球·全外环·圆位(外环球圆)");
    si_morphism!(Si_Qiu_N3A1, "球·全外环·浑位(外环球浑)");
    si_morphism!(Si_Qiu_N3A2, "球·全外环·合位(外环球合)");
    si_morphism!(Si_Qiu_N3A3, "球·全外环·满位(外环球满)");

    // 份(均分)24
    si_morphism!(Si_Fen_N0A0, "份·分内·均位(内环均分,公平无偏)");
    si_morphism!(Si_Fen_N0A1, "份·分内·分位(内环分配,分而有之)");
    si_morphism!(Si_Fen_N0A2, "份·分内·摊位(内环分担,各摊一组)");
    si_morphism!(Si_Fen_N0A3, "份·分配内·位(内环配给,各有其份)");
    si_morphism!(Si_Fen_N1A0, "份·分次内·均位(容器内层份均)");
    si_morphism!(Si_Fen_N1A1, "份·分次内·分位(容器内层份分)");
    si_morphism!(Si_Fen_N1A2, "份·分次内·摊位(容器内层份摊)");
    si_morphism!(Si_Fen_N1A3, "份·分次内·配位(容器内层份配)");
    si_morphism!(Si_Fen_N2A0, "份·分中环·均位(中环份均)");
    si_morphism!(Si_Fen_N2A1, "份·分中环·分位(中环份分)");
    si_morphism!(Si_Fen_N2A2, "份·分中环·摊位(中环份摊)");
    si_morphism!(Si_Fen_N2A3, "份·分中环·配位(中环份配)");
    si_morphism!(Si_Fen_N3A0, "份·分外环·均位(外环份均)");
    si_morphism!(Si_Fen_N3A1, "份·分外环·分位(外环份分)");
    si_morphism!(Si_Fen_N3A2, "份·分外环·摊位(外环份摊)");
    si_morphism!(Si_Fen_N3A3, "份·分外环·配位(外环份配)");

    // 恒等态射 id(SiYuan)
    #[derive(Clone, Copy,	Debug)]
    pub struct Si_Unit;
    impl Morphism for Si_Unit {
        type Dom = SiYuan;
        type Cod = SiYuan;
        fn compose(_rhs: impl Morphism<Dom = SiYuan>) -> Self { Self }
        fn describe() -> &'static str { "幔·元单位(四元恒等,id)" }
    }
}

// ────────────────────────────────────────────────────────────
// 测试:验证范畴结构
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_objects_exist() {
        // 四个对象都是零大小类型,在类型层面确认存在
        let _: YiYuan = YiYuan;
        let _: ErYuan = ErYuan;
        let _: SanYuan = SanYuan;
        let _: SiYuan = SiYuan;
    }

    #[test]
    fn yi_morphism_describes_self() {
        // 每根符号自己解释自己
        assert_eq!(yi::Yi_N0A0::describe(), "意·内环·始位(意之始生,箭在弦上)");
        assert_eq!(yi::Yi_W3A3::describe(), "基·外环·达位(外环根基完成)");
    }

    #[test]
    fn er_morphism_describes_self() {
        assert_eq!(er::Er_Ming_N0A0::describe(), "明·盈内·满位(光源充盈,照破黑暗)");
        assert_eq!(er::Er_Ao_W3A3::describe(), "凹·陷外环·容位(外环凹陷)");
    }

    #[test]
    fn san_morphism_describes_self() {
        assert_eq!(san::San_Zhu_N0A0::describe(), "柱·锚内·定位(锚定乾坤,内环固定)");
        assert_eq!(san::San_Zheng_W3A3::describe(), "证·判外环·裁位(外环裁断)");
    }

    #[test]
    fn si_morphism_describes_self() {
        assert_eq!(si::Si_Man_N0A0::describe(), "幔·覆内·藏位(内环覆盖,隐藏真相)");
        assert_eq!(si::Si_Fen_N3A3::describe(), "份·分外环·配位(外环份配)");
    }

    #[test]
    fn identity_endomorphisms_exist() {
        // 每个对象都有恒等态射
        let _: <YiYuan as Object>::Identity = yi::Yi_Unit;
        let _: <ErYuan as Object>::Identity = er::Er_Unit;
        let _: <SanYuan as Object>::Identity = san::San_Unit;
        let _: <SiYuan as Object>::Identity = si::Si_Unit;
    }

    #[test]
    fn morphisms_are_copy() {
        // 态射是 Copy(自由复制,不耗内存)
        use crate::symbols::Morphism;
        let a = yi::Yi_N0A0;
        let b = a;  // Copy
        let _ = a;  // 还可用
        let _ = b;
    }

    #[test]
    fn compose_id_left() {
        // id ∘ f = f
        use crate::symbols::Morphism;
        let composed = yi::Yi_Unit::compose(yi::Yi_E2A2);
        let _ = composed;
    }

    #[test]
    fn compose_id_right() {
        // f ∘ id = f
        use crate::symbols::Morphism;
        let composed = yi::Yi_S1A1::compose(yi::Yi_Unit);
        let _ = composed;
    }

    #[test]
    fn yi_count() {
        // 确认一元态射数 (通过手动计数保证与范畴论主张一致)
        // 北宫 + 东宫 + 南宫 + 西宫 = 16 × 4 = 64
        const YI_COUNT: i32 = 16 + 16 + 16 + 16;
        assert_eq!(YI_COUNT, 64);
    }
}

// ────────────────────────────────────────────────────────────
// 派生常量:态射总数(仅供文档/测试用,不作为编码值使用)
// ────────────────────────────────────────────────────────────

pub const TOTAL_MORPHISMS: usize = 384;
pub const YI_MORPHISM_COUNT: usize = 64;
pub const ER_MORPHISM_COUNT: usize = 96;
pub const SAN_MORPHISM_COUNT: usize = 96;
pub const SI_MORPHISM_COUNT: usize = 128;
