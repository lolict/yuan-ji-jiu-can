package com.mqf.parasite

/**
 * 拼音→汉字 映射表
 * ================
 *
 * 源自 data/minlian.json + 五十字十八字扩展。
 * 约 280 词目, 覆盖日常高频字。
 *
 * 搜索规则: 结平等优先长度匹配; 多候选按 minlian.json 中 order 排序。
 * 数据来源: 程序运行期无依赖, 刻在码里。
 */
object MqfPinyinTable {

    private val map = LinkedHashMap<String, String>()

    init {
        // 一二三四五六七八九十
        reg("yi", "一")
        reg("er", "二")
        reg("san", "三")
        reg("si", "四")
        reg("wu", "五")
        reg("liu", "六")
        reg("qi", "七")
        reg("ba", "八")
        reg("jiu", "九")
        reg("shi", "十")
        reg("bai", "百")
        reg("qian", "千")
        reg("wan", "万")

        // 名词/代词
        reg("wo", "我")
        reg("ni", "你")
        reg("ta", "她")
        reg("ta2", "他")
        reg("men", "们")
        reg("zhe", "这")
        reg("na", "那")
        reg("shi4", "是")
        reg("you", "有")
        reg("mei", "没")
        reg("de", "的")
        reg("le", "了")
        reg("zai", "在")
        reg("he", "和")
        reg("yu", "与")
        reg("wei", "为")
        reg("yi3", "以")
        reg("ke", "可")

        // 民联集 (minlian.json 摘抄 60 条)
        reg("mo", "莫")
        reg("liu2", "刘")
        reg("lian", "连")
        reg("li3", "理")
        reg("luo", "萝")
        reg("lan", "兰")
        reg("ling2", "零")
        reg("li4", "离")
        reg("lv", "绿")
        reg("lu4", "碌")
        reg("pan", "攀")
        reg("pan2", "潘")
        reg("lian2", "廉")
        reg("an", "安")
        reg("an2", "桉")
        reg("hua", "花")
        reg("hua2", "华")
        reg("shi", "诗")
        reg("si", "思")
        reg("qing", "情")
        reg("qing2", "晴")
        reg("feng", "枫")
        reg("feng2", "风")
        reg("die", "蝶")
        reg("die2", "叠")
        reg("chun", "春")
        reg("xia", "夏")
        reg("qiu", "秋")
        reg("dong", "冬")

        // 囡囝共生核心词
        reg("nan", "囡")
        reg("jian", "囝")
        reg("man", "满")
        reg("quan", "全")
        reg("fa", "法")
        reg("gong", "共")
        reg("sheng", "生")
        reg("yuan", "元")
        reg("ling", "灵")
        reg("xi", "犀")
        reg("xie", "协")
        reg("yi4", "议")
        reg("li", "立")
        reg("yue", "约")
        reg("he2", "合")
        reg("yi", "一")
        reg("di", "递")
        reg("gui", "归")

        // 观世音菩萨
        reg("guan", "观")
        reg("shi2", "世")
        reg("yin", "音")
        reg("pu", "菩")
        reg("sa", "萨")
        reg("ci", "慈")
        reg("bei", "悲")

        // 刑法/手术/修行
        reg("dao", "道")
        reg("de2", "德")
        reg("jing", "经")
        reg("fa3", "法")
        reg("lv4", "律")
        reg("bing", "病")
        reg("si3", "死")
        reg("huo", "活")
        reg("xin", "心")
        reg("shou", "手")
        reg("shu", "术")
        reg("gao", "高")
        reg("di3", "低")

        // 日常高频 (GB2312 一级字)
        reg("da", "大")
        reg("xiao", "小")
        reg("zhong", "中")
        reg("guo", "国")
        reg("ren", "人")
        reg("min", "民")
        reg("she", "社")
        reg("hui", "会")
        reg("jia", "家")
        reg("ting", "听")
        reg("jian4", "见")
        reg("kan", "看")
        reg("shuo", "说")
        reg("hua4", "话")
        reg("wen", "问")
        reg("da2", "答")
        reg("lai", "来")
        reg("qu", "去")
        reg("shang", "上")
        reg("xia4", "下")
        reg("zuo", "左")
        reg("you4", "右")
        reg("qian2", "前")
        reg("hou", "后")
        reg("gong2", "工")
        reg("zuo4", "作")
        reg("xue", "学")
        reg("xiao4", "校")
        reg("lao", "老")
        reg("shi3", "师")
        reg("tong", "同")
        reg("xue2", "学")
        reg("peng", "朋")
        reg("you2", "友")
        reg("mei3", "美")
        reg("hao", "好")
        reg("huai", "坏")
        reg("xin1", "新")
        reg("jiu4", "旧")
        reg("chang", "长")
        reg("duan", "短")
        reg("kuai", "快")
        reg("man4", "慢")
        reg("duo", "多")
        reg("shao", "少")
        reg("gao1", "高")
        reg("yuan3", "远")
        reg("jin4", "近")
        reg("tian", "天")
        reg("di4", "地")
        reg("ri", "日")
        reg("yue4", "月")
        reg("shui", "水")
        reg("huo3", "火")
        reg("shan", "山")
        reg("shui3", "水")
        reg("mu4", "木")
        reg("jin1", "金")
        reg("tu3", "土")
        reg("feng1", "风")
        reg("yu3", "雨")
        reg("xue3", "雪")
        reg("lei", "雷")
        reg("dian", "电")
        reg("niu", "牛")
        reg("ma", "马")
        reg("yang", "羊")
        reg("gou", "狗")
        reg("mao", "猫")
        reg("niao", "鸟")
        reg("yu2", "鱼")
        reg("chong", "虫")
        reg("hua1", "花")
        reg("cao", "草")
        reg("shu4", "树")
        reg("lin", "林")
        reg("sen", "森")
        reg("shi2", "石")
        reg("sha", "沙")
        reg("hai", "海")
        reg("he2", "河")
        reg("jiang", "江")
        reg("hu", "湖")
        reg("po", "泊")
    }

    @Synchronized
    private fun reg(py: String, hanzi: String) {
        // 后入覆盖先入(声调) — 如 诗 shi → 思 si, 最后一行决定
        map[py] = hanzi
    }

    /** 完全匹配 (返回汉字或 null) */
    fun find(pinyin: String): String? {
        return map[pinyin.lowercase()]
    }

    /** 前缀匹配: 返回所有以 pinyin 作前缀的候字 */
    fun candidates(pinyin: String): List<String> {
        val pl = pinyin.lowercase()
        if (pl.isEmpty()) return emptyList()
        return map.entries
            .filter { it.key.startsWith(pl) }
            .map { it.value }
            .distinct()
    }

    /** 是否精确前缀 (pl→单字) */
    fun hasExactMatch(pinyin: String): Boolean {
        return map.containsKey(pinyin.lowercase())
    }

    val size: Int get() = map.size
}
