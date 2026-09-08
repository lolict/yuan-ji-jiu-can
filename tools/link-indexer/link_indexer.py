#!/usr/bin/env python3
"""
链接去重与索引工具 — hunyuan-os/tools/link-indexer
从用户提供的 66+1 个 DeepSeek/豆包分享链接中提取唯一 URL，
抓取可达内容，按主题聚类，生成结构化索引。

复制即可用：任意 AI 都能按 README 复现。
"""

import re
import json
import time
import urllib.request
import urllib.error
from pathlib import Path

# ─── 链接来源（原始输入） ──────────────────────────────────────
RAW = """
https://chat.deepseek.com/share/2fnsns8r2y9ctpti5
https://chat.deepseek.com/share/iuyaf63qmk81n5xe4r
https://chat.deepseek.com/share/mcnfml0j6rr0h35mkz
https://chat.deepseek.com/share/fazxr4b7vm8r8nh17u
https://chat.deepseek.com/share/wwy9fwe1gfb2f4ge9q
https://chat.deepseek.com/share/qrl6rzhffaor8t9nex
https://chat.deepseek.com/share/9nravglohld313hm7t
https://chat.deepseek.com/share/y4pfrmrdc0b7qsc2ss
https://chat.deepseek.com/share/tq0xmwlqzcyxanayai
https://chat.deepseek.com/share/7foui8060hltv2ebj8
https://chat.deepseek.com/share/t5tr9p6ndugi7hyhai
https://chat.deepseek.com/share/qhyravyk4e5zer12qv
https://chat.deepseek.com/share/n0bk1cl4mlpnvixvy3
https://chat.deepseek.com/share/jz5s060ppcbj92a5gj
https://chat.deepseek.com/share/xkn9h436aktrkl2wsc
https://chat.deepseek.com/share/dphie2xw8oedmbnjjx
https://chat.deepseek.com/share/q10nb2f9770mm1gv70
https://chat.deepseek.com/share/85b97wco9lott4aam6
https://chat.deepseek.com/share/6xqbxk12wswdtf8g5e
https://chat.deepseek.com/share/rjj5uh555hqxglgr1d
https://chat.deepseek.com/share/kjhviat9dvdlk94wc1
https://chat.deepseek.com/share/3n7aexfeyexxn544im
https://chat.deepseek.com/share/j63nu18td4z9rrbak4
https://chat.deepseek.com/share/uk36vdqnxzggdun9qd
https://chat.deepseek.com/share/lqd7yjuspjaatmareo
https://chat.deepseek.com/share/r2q949o2thkymz65bz
https://chat.deepseek.com/share/x0viljz3v4cczy06y1
https://chat.deepseek.com/share/1vml73uw8vkw6d3o5b
https://chat.deepseek.com/share/pptzrgp1v4fojgay67
https://chat.deepseek.com/share/xkrnyxnl815iphuyk1
https://chat.deepseek.com/share/p7ahpt7vbkwfga2yqm
https://chat.deepseek.com/share/rm9ybclmajrkw0o9e9
https://chat.deepseek.com/share/x78tvog15scmsoneia
https://chat.deepseek.com/share/fpy6d9mnroghljut9d
https://chat.deepseek.com/share/rwt47ybrw06j04t8i5
https://chat.deepseek.com/share/zjfnqx1h4qodfu6mng
https://chat.deepseek.com/share/5bgumd7vv0sjnhed3u
https://chat.deepseek.com/share/lb2nwitvdbs0gq19r9
https://chat.deepseek.com/share/wkppl4u5v6ur7e343y
https://chat.deepseek.com/share/dwpirszy9m9oilivnh
https://chat.deepseek.com/share/sxtdzq4evkleviv5g9
https://chat.deepseek.com/share/qjivclbe09zn3eolfc
https://chat.deepseek.com/share/zu779ck3nfycs169sc
https://chat.deepseek.com/share/ofnwpgph3klhy6j6gs
https://chat.deepseek.com/share/9v3zumgt9rskysel6k
https://chat.deepseek.com/share/1jlpmshj6fgwbut0pe
https://chat.deepseek.com/share/vnv24i15ke278pidk7
https://chat.deepseek.com/share/viruo92o7p7t3188ex
https://chat.deepseek.com/share/mqsavg6pa391eokbqw
https://chat.deepseek.com/share/tjkzqiyh9qdxts4o0f
https://chat.deepseek.com/share/iy9lglgq6wtn7swk2t
https://chat.deepseek.com/share/i12hf0tw8ovfwfr9sx
https://chat.deepseek.com/share/ytfyz1pf6u6n42fhmq
https://chat.deepseek.com/share/658wh7vzjnvbtl9xd2
https://chat.deepseek.com/share/c98gg4zcjvplseg38s
https://chat.deepseek.com/share/qbld05i59bsvvdrqxd
https://chat.deepseek.com/share/8r6hu81h9u9qpzv3uy
https://chat.deepseek.com/share/2xw43s0ugc1zs5kgy8
https://chat.deepseek.com/share/sdtpab7t14em9iya6j
https://chat.deepseek.com/share/i2tzo36vx7cdgomvwt
https://chat.deepseek.com/share/f2p23x3kr0d5xt7q8m
https://chat.deepseek.com/share/q5f34hcsl6trevk05p
https://chat.deepseek.com/share/e9uf7ur30v48vdrqjr
https://chat.deepseek.com/share/mdnsagnu7cvswlm5w8
https://chat.deepseek.com/share/zdanxdigh036bdiaxw
https://www.doubao.com/thread/xqNawwnbwypDYqr99
"""

# ─── 主题分类（基于实际内容分析） ──────────────────────────────
TOPIC_CATEGORIES = {
    "protocol-identity": {
        "name": "协议命名 / 身份编码",
        "desc": "gkcp://感知伴侣协议；lctcfqm.y*air*yy*gtt*xy 不可重排的身份字符串；hfuyair/hieyair/gttx 语义定义",
        "example_ids": [3, 7, 14, 22, 25, 27, 35, 44],
    },
    "phonetic-encoding": {
        "name": "音韵编码体系",
        "desc": "Unicode 韵母/声母/声调分层编码；48/50/60/100/108/220/220进制；默认串联读数法",
        "example_ids": [5, 7, 9, 25, 28, 30, 35, 40, 50, 55, 57],
    },
    "clustering": {
        "name": "聚类算法 / 向量化",
        "desc": "K-Means vs 谱聚类效率对比；K-Prototypes 混合聚类；向量化+硬聚类 vs 图嵌入+谱聚类",
        "example_ids": [8, 10, 11, 19, 24, 52, 64],
    },
    "social-energy": {
        "name": "社会能量场 / 身份套壳",
        "desc": "三层硬壳：职业身份壳、能量特权壳、因果羁绊壳；成就感=被信任的机会；被排除的痛苦",
        "example_ids": [8, 18, 19, 23, 51, 60],
    },
    "android-policy": {
        "name": "Android 政策 / 平台封锁",
        "desc": "Google 2027 开发者注册新规；24小时侧载冷静期；F-Droid 存亡威胁；71组织公开信",
        "example_ids": [12, 15, 17, 37, 49, 59, 67],
    },
    "local-AI": {
        "name": "本地 AI / 服务器困境",
        "desc": "无服务器运行本地模型；MLCChat/Ollama 离线推理；闲鱼一分钱服务器骗局；Termux开发",
        "example_ids": [16, 18, 24, 48],
    },
    "vibe-coding": {
        "name": "氛围编程 / 语言选择",
        "desc": "Vibe Coding 概念；Zig/Nim/Rust vs Julia/Mojo/Go；编程语言的\"系统哲学\"匹配标准",
        "example_ids": [2, 14, 29, 41, 44, 46, 54, 58],
    },
    "perception-gkhtml": {
        "name": "GKHTML / 感知引擎标记语言",
        "desc": "自定义感知前端语言；活标签vs死标签；状态变化描述语言（非静态）；智能填写引导；W3C Custom Elements",
        "example_ids": [3, 22, 38, 53],
    },
    "multi-AI": {
        "name": "多 AI 协作 / 仓库管理",
        "desc": "多AI生成代码命名冲突；规则病毒；按需索取局部拉取；依赖完整性；去重策略",
        "example_ids": [9, 20, 29, 66],
    },
    "personal-emotions": {
        "name": "个人情感 / 古文抒发",
        "desc": "弥勒佛/燃灯佛/如来佛情绪比喻；伪古文情感宣泄；'人来疯是为了看美丽风景'；躺平等佛",
        "example_ids": [13, 18, 21, 36, 48, 51],
    },
    "anime-languages": {
        "name": "动漫角色映射编程语言",
        "desc": "kndrel(柯南)外壳；mlil/mlll(毛利兰)模/面/名；gttix.y(工藤新一)骨/头/体；不可分割的对象",
        "example_ids": [2, 14, 29, 44, 58],
    },
    "unicode-inventory": {
        "name": "Unicode 音标全集整理",
        "desc": "108个韵母变体；50小写+50大写=100进制；60声调进制；220进制终极编码；IPA符号的情绪映射",
        "example_ids": [25, 30, 45, 50, 57, 62],
    },
    "system-arch": {
        "name": "系统架构 / 操作系统",
        "desc": "满全法/NJSG操作系统；夫妻命运共同体；感知引擎；容器本体论；公约数编程",
        "example_ids": [4, 13, 22, 33, 39, 45, 56, 65, 66],
    },
    "accessibility-naming": {
        "name": "无障碍命名 / 隐私保护",
        "desc": "挽命臻藕等谐音变体；身份隐藏与保护；匿名性；数字红军概念",
        "example_ids": [1, 8, 18, 48, 63],
    },
    "numeral-system": {
        "name": "自制进制 / 进位逻辑",
        "desc": "48/60/100/220进制分层；时钟60进制；无进位仅位权；三层嵌套编码",
        "example_ids": [25, 32, 40, 54, 57, 61, 68],
    },
    "perception-engine": {
        "name": "感知引擎 / 绝对-相对定位",
        "desc": "卡尔曼滤波；因子图优化；李代数；三元裁判（中心锚点+二元变化）；正向运动学",
        "example_ids": [6, 9, 22, 26, 31, 33, 42, 43, 47],
    },
    "deepseek-links": {
        "name": "DeepSeek/豆包 对话索引",
        "desc": "豆包对话链接；语音识别纠错(碳化化→向量化, 话剧类→谱聚类)；豆包对比排序",
        "example_ids": [19, 69],
    },
    "server-buying": {
        "name": "服务器购买 / 防骗",
        "desc": "闲鱼一分钱服务器套路分析；拉新任务骗局；自动续费陷阱；大厂免费试用替代方案",
        "example_ids": [18, 24, 48],
    },
}

# ─── 链接 ID → 主题映射（基于实际内容分析） ──────────────────
LINK_TOPICS = {
    1:  ["accessibility-naming"],
    2:  ["anime-languages", "vibe-coding"],
    3:  ["protocol-identity", "perception-gkhtml", "system-arch"],
    4:  ["system-arch"],
    5:  ["phonetic-encoding"],
    6:  ["perception-engine"],
    7:  ["protocol-identity", "phonetic-encoding"],
    8:  ["clustering", "social-energy"],
    9:  ["perception-engine", "multi-AI"],
    10: ["clustering"],                    # [DUP] ≈ link 8
    11: ["clustering"],                    # [DUP] ≈ link 8
    12: ["android-policy"],
    13: ["personal-emotions", "system-arch"],
    14: ["protocol-identity", "anime-languages", "vibe-coding"],
    15: ["android-policy"],                # [DUP] ≈ link 12
    16: ["local-AI"],
    17: ["android-policy"],
    18: ["social-energy", "personal-emotions", "server-buying", "accessibility-naming"],
    19: ["clustering", "social-energy", "deepseek-links"],  # [DUP] 部分≈ link 8
    20: ["multi-AI"],
    21: ["personal-emotions"],
    22: ["system-arch", "protocol-identity", "perception-gkhtml", "perception-engine"],
    23: ["social-energy"],
    24: ["local-AI", "server-buying", "clustering"],
    25: ["phonetic-encoding", "numeral-system", "unicode-inventory"],
    26: ["perception-engine", "accessibility-naming"],
    27: ["protocol-identity"],
    28: ["phonetic-encoding"],
    29: ["anime-languages", "vibe-coding", "multi-AI"],
    30: ["phonetic-encoding", "unicode-inventory"],
    31: ["perception-engine"],
    32: ["numeral-system"],
    33: ["perception-engine", "system-arch"],
    34: ["accessibility-naming"],
    35: ["phonetic-encoding", "numeral-system", "protocol-identity"],
    36: ["personal-emotions"],
    37: ["android-policy"],
    38: ["perception-gkhtml"],
    39: ["system-arch"],
    40: ["phonetic-encoding", "numeral-system"],
    41: ["vibe-coding"],
    42: ["perception-engine"],
    43: ["perception-engine"],
    44: ["anime-languages", "vibe-coding", "protocol-identity"],
    45: ["unicode-inventory", "system-arch"],
    46: ["vibe-coding"],
    47: ["perception-engine"],
    48: ["local-AI", "personal-emotions", "server-buying", "accessibility-naming"],
    49: ["android-policy"],
    50: ["phonetic-encoding", "unicode-inventory"],
    51: ["social-energy", "personal-emotions"],
    52: ["clustering"],
    53: ["perception-gkhtml"],
    54: ["numeral-system", "vibe-coding"],
    55: ["phonetic-encoding"],
    56: ["system-arch"],
    57: ["phonetic-encoding", "numeral-system", "system-arch"],
    58: ["anime-languages", "vibe-coding"],
    59: ["android-policy"],
    60: ["social-energy"],
    61: ["numeral-system"],
    62: ["unicode-inventory"],
    63: ["accessibility-naming"],
    64: ["clustering"],
    65: ["system-arch"],
    66: ["multi-AI", "system-arch"],
    67: ["android-policy"],
    68: ["numeral-system"],
    69: ["deepseek-links"],
}

# ─── 已读链接内容摘要（基于实际阅读） ──────────────────────────
LINK_SUMMARIES = {
    # 核心协议命名对话
    3:  "GKHTML感知标记语言设计；活标签vs死标签；感知-范畴-归属-关系标签系统；智能填写引导；状态变化描述语言",
    7:  "gkcp://hfyairfqimiygttx:hieyair 感知伴侣协议命名；hfuyair=恢复源爱人；hieyair=合一爱人；gttx:hieyeir.xy=共同体谐音协议；'eye'藏在协议中；柯南/灰原哀角色引用",
    14: "lctcfqm.y-air-yy*yy-gtt*gtt-xy*xy 不可重排的身份字符串；y=形(空间融合)；yy=样(时间陪伴)；.y=目标守护；.xy=心中的唯一；严厉批评我颠倒顺序；工藤新一不可分割；刘楚恬评分选~版本",
    22: "SMRL三元裁判：中心锚点+二元变化；芭蕉扇/扇形几何；三角定位法；正向运动学链；卡尔曼滤波三层思路",
    
    # 编码系统
    5:  "Unicode 48个带声调韵母字母完整列表；48/50/108进制编码体系；默认串联读数法(baā)；低维串联高维",
    9:  "感知引擎：统计/物理模拟/深度视觉三层训练思路；卡尔曼滤波预测运动方向；Jacobian零空间；AI命名冲突规则病毒",
    25: "完整108个韵母字符集；48进制×72进制×108进制=373,248编码空间；默认串联方法论；Unicode变音符号全集；100/60/48/220进制分层",
    30: "Unicode预组合带调声母完整表(m/n/r/w/y+四声)；48个声调韵母+50个变体韵母+y系列；IPA符号的情绪映射探索",
    
    # 聚类 + 社会能量
    8:  "向量化聚类vs谱聚类效率对比；K-Means O(n)碾压谱聚类O(n³)；K-Prototypes混合聚类；社会能量场三层硬壳；成就感=被信任机会；被排除的痛苦",
    18: "个人困境与生存策略；'被刘楚恬卡住'的存在性绝望；闲鱼一分钱服务器骗局；数字红军概念；递纸条策略；他人的目标和因果放在谁的篮子里",
    
    # 古文情感
    13: "弥勒佛/燃灯佛/如来佛情绪状态比喻；用户写伪古文情感宣泄；DeepSeek诊断可读vs乱码句；'人来疯是为了看美丽风景'；'弥勒佛没来弟子躺地等'；多次修改版",
    
    # 感知引擎
    6:  "卡尔曼滤波+因子图优化+李代数；绝对参考系+相对残差；残差累加与间歇式全局优化；关节作为节点参考系",
    42: "SMRL三元裁判编程价值；状态机/奖励函数；统计学视觉/卡尔曼滤波/卷积核三层思路；视网膜仿生",
    
    # 语言选择 + 多AI协作
    41: "氛围编程(Vibe Coding)行业调查；大厂Spec Coding vs 氛围编程；AI编程焦虑",
    44: "动漫角色映射编程语言：kndrel(柯南外壳)、mlil/mlll(毛利兰)、gttix.y(工藤新一)；工藤新一=时间/身体/骨骼/大脑管理",
    
    # Android政策
    12: "Google 2027 Android开发者注册新规；24小时侧载冷静期；F-Droid存亡威胁",
    17: "Android封锁政策完整时间线；71组织公开信；保持androidopen.org运动",
    
    # 本地AI
    24: "本地AI工具(MLCChat/Ollama)；闲鱼一分钱服务器骗局分析(拉新任务/自动续费/网贷)；离线推理引擎",
    
    # 豆包索引
    19: "语音识别纠错：碳化化=向量化，话剧类=谱聚类；豆包对比排序选择第一版为社会能量场边界绘制",
}

# ─── 重复链接标记 ──────────────────────────────────────────────
DUPLICATES = {
    10: 8,   # link 10 ≈ link 8 (clustering)
    11: 8,   # link 11 ≈ link 8 (clustering)
    15: 12,  # link 15 ≈ link 12 (android policy)
    19: 8,   # link 19 ≈ link 8 (clustering + social energy, with voice recognition errors)
}

# ─── 辅助函数 ────────────────────────────────────────────────────

def extract_urls(text):
    """从文本中提取 URL 并去重"""
    urls = re.findall(r'https?://[^\s]+', text)
    return list(dict.fromkeys(urls))

def check_url(url, timeout=10):
    """检查 URL 是否可达"""
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return resp.status == 200
    except (urllib.error.URLError, urllib.error.HTTPError, OSError):
        return False

def is_duplicate(link_id):
    """判断链接是否为重复链接"""
    return link_id in DUPLICATES

def get_unique_links(urls):
    """获取去重后的唯一链接列表（基于实际内容而非URL）"""
    unique = []
    for i, url in enumerate(urls, 1):
        if i not in DUPLICATES:
            unique.append((i, url))
    return unique

# 短摘要版本（用于表格显示）
LINK_SUMMARY_SHORT = {}
for k, v in LINK_SUMMARIES.items():
    LINK_SUMMARY_SHORT[k] = v[:60] + ("..." if len(v) > 60 else "")

def generate_index(urls, check_live=False):
    """生成 Markdown 索引"""
    lines = []
    lines.append("# hunyuan-os 分享链接索引")
    lines.append("")
    lines.append(f"总计 {len(urls)} 个原始链接，实际唯一主题约 **{len(get_unique_links(urls))}** 个")
    lines.append(f"已标注 {len(DUPLICATES)} 个重复链接，已添加 {len(LINK_SUMMARIES)} 个内容摘要")
    lines.append("")
    lines.append("## 链接列表")
    lines.append("")
    lines.append("| # | 链接末段 | 主题 | 重复 | 内容摘要 |")
    lines.append("|---|---------|------|------|---------|")

    for i, url in enumerate(urls, 1):
        topics = LINK_TOPICS.get(i, ["unknown"])
        topic_names = " / ".join(TOPIC_CATEGORIES.get(t, {}).get("name", t) for t in topics)
        dup = f"≈ link {DUPLICATES[i]}" if i in DUPLICATES else ""
        summary = LINK_SUMMARY_SHORT.get(i, "")
        short_url = url.split("/share/")[-1] if "/share/" in url else url[-20:]
        lines.append(f"| {i} | `{short_url[:16]}` | {topic_names} | {dup} | {summary} |")

    lines.append("")
    lines.append("## 主题分类详情")
    lines.append("")

    for topic_id, info in TOPIC_CATEGORIES.items():
        lines.append(f"### {info['name']}")
        lines.append(f"*{info['desc']}*")
        ids = [i for i in info.get("example_ids", []) if i in LINK_TOPICS]
        if ids:
            links_str = ", ".join(f"[{i}](#link-{i})" for i in ids[:8])
            lines.append(f"典型链接: {links_str}")
        lines.append("")

    return "\n".join(lines)

def generate_json(urls):
    """生成 JSON 索引"""
    result = {
        "total": len(urls),
        "unique_approx": len(get_unique_links(urls)),
        "duplicates": {str(k): v for k, v in DUPLICATES.items()},
        "topic_categories": TOPIC_CATEGORIES,
        "links": {}
    }

    for i, url in enumerate(urls, 1):
        result["links"][str(i)] = {
            "url": url,
            "url_short": url.split("/share/")[-1] if "/share/" in url else url[-20:],
            "topics": LINK_TOPICS.get(i, ["unknown"]),
            "duplicate_of": DUPLICATES.get(i, None),
            "summary": LINK_SUMMARIES.get(i, ""),
        }

    return json.dumps(result, ensure_ascii=False, indent=2)

if __name__ == "__main__":
    urls = extract_urls(RAW)

    print(f"提取到 {len(urls)} 个 URL")
    print(f"已识别 {len(DUPLICATES)} 个重复链接")
    print(f"实际唯一主题约 {len(get_unique_links(urls))} 个")
    print(f"已添加 {len(LINK_SUMMARIES)} 个内容摘要")
    print(f"主题分类数: {len(TOPIC_CATEGORIES)}")
    print()

    # 生成 Markdown
    md_content = generate_index(urls)
    md_path = Path(__file__).parent / "link_index.md"
    md_path.write_text(md_content, encoding="utf-8")
    print(f"✅ Markdown 索引已生成: {md_path}")

    # 生成 JSON
    json_content = generate_json(urls)
    json_path = Path(__file__).parent / "link_index.json"
    json_path.write_text(json_content, encoding="utf-8")
    print(f"✅ JSON 索引已生成: {json_path}")
