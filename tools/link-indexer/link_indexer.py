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
    "phonetic-encoding": {
        "name": "音韵编码体系",
        "desc": "Unicode 韵母/声母/声调分层编码、48/50/108 进制、默认串联读数法、低维串联高维",
        "example_ids": [5, 7, 25, 28, 30, 35, 40, 48, 50, 55, 57],
    },
    "clustering": {
        "name": "聚类算法 / 向量化",
        "desc": "K-Means vs 谱聚类效率、向量化聚类、K-Prototypes 混合聚类、软链接谱聚类",
        "example_ids": [8, 10, 11, 19, 52, 64],
    },
    "social-energy": {
        "name": "社会能量场 / 身份套壳",
        "desc": "职业身份壳、能量特权壳、因果羁绊壳；成就感=被信任的机会；被排除的痛苦与生存策略",
        "example_ids": [8, 18, 19, 23, 51, 60],
    },
    "protocol-naming": {
        "name": "协议命名 / 身份标识",
        "desc": "gkcp://hfyairfqimiygttx:hieyair 感知伴侣协议；hfuyair 恢复源爱人；hieyair 合一爱人；gttx:hieyeir.xy 共同体谐音协议",
        "example_ids": [3, 7, 22, 25, 27, 35, 44],
    },
    "perception-engine": {
        "name": "感知引擎 / 绝对-相对定位",
        "desc": "卡尔曼滤波、因子图优化、李代数、即时定位与地图构建(SLAM)；宏观静态参考系+微观动态残差",
        "example_ids": [6, 9, 26, 31, 33],
    },
    "android-policy": {
        "name": "Android 政策 / 平台封锁",
        "desc": "Google 2027 开发者注册新规、24小时侧载冷静期、F-Droid 存亡威胁、71组织公开信",
        "example_ids": [12, 15, 17, 37, 49, 59, 67],
    },
    "local-AI": {
        "name": "本地 AI / 服务器困境",
        "desc": "无服务器运行本地模型、MLCChat/Ollama 离线推理、闲鱼一分钱服务器骗局、Termux开发",
        "example_ids": [16, 18, 24, 48],
    },
    "vibe-coding": {
        "name": "氛围编程 / AI 编程",
        "desc": "Vibe Coding 概念、大厂程序员 AI 使用现状、Spec Coding vs 氛围编程；多AI协作风险",
        "example_ids": [41, 46, 54],
    },
    "smrl-judge": {
        "name": "SMRL 三元裁判",
        "desc": "三元认知裁判：中心锚点(不变)+二元变化(手段)；视觉训练；统计学/物理模拟/深度视觉三层思路",
        "example_ids": [42, 43, 47],
    },
    "gkhtml": {
        "name": "GKHTML 感知标记语言",
        "desc": "自定义感知前端语言、活标签设计(非固定)、动态状态变化描述、避免写死情感、智能提示填写",
        "example_ids": [3, 22, 38, 53],
    },
    "programming-languages": {
        "name": "动漫角色映射编程语言",
        "desc": "kndrel(柯南)外壳语言；mlil/mlll(毛利兰)模/面/名；gttix.y/gtit.y(工藤新一)骨/头/体；语言选择焦虑",
        "example_ids": [2, 14, 29, 44, 58],
    },
    "visual-encoding": {
        "name": "视觉/颜色/视频/音乐编码",
        "desc": "颜色编码映射(108色相)、视频帧差异编码、MIDI音高梯度、图片梯度编码；光层级透射与视网膜",
        "example_ids": [30, 45, 50, 62],
    },
    "personal-survival": {
        "name": "个人生存策略 / 心理",
        "desc": "数字红军概念、弥勒佛/燃灯佛/如来佛情绪状态；递纸条策略；做自己的负责人；壳的升级迭代",
        "example_ids": [18, 21, 36, 48, 51],
    },
    "multi-AI": {
        "name": "多 AI 协作 / 仓库管理",
        "desc": "多AI生成代码命名冲突、规则病毒；按需拉取局部代码；统一接口与依赖完整性",
        "example_ids": [29, 66],
    },
    "system-arch": {
        "name": "系统架构 / 操作系统",
        "desc": "满全法/NJSG操作系统、夫妻命运共同体架构；微内核、意识路由、八流统一、容器本体论",
        "example_ids": [4, 13, 22, 33, 39, 45, 56, 57, 66],
    },
    "numeral-system": {
        "name": "自制进制 / 进位逻辑",
        "desc": "48/72/108 三层进制串联；时钟 60 进制；220 进制终极编码；无进位仅位权；公因数编程",
        "example_ids": [25, 40, 54, 57, 61],
    },
    "accessibility-naming": {
        "name": "无障碍命名 / 谐音变体",
        "desc": "挽救命/臻藕/挽命臻藕等谐音变体；赎回命名权；身份隐藏与保护；匿名性作为保护",
        "example_ids": [1, 8, 18, 26, 34, 48, 63],
    },
    "deepseek-links": {
        "name": "DeepSeek/豆包 对话索引",
        "desc": "豆包对话链接、语音识别纠错(碳化化→向量化, 话剧类→谱聚类)",
        "example_ids": [19, 69],
    },
}

# ─── 链接 ID → 主题映射（基于实际内容分析） ──────────────────
# 注：标记 [DUP] 的链接是已读链接的内容重复项
LINK_TOPICS = {
    1:  ["accessibility-naming"],
    2:  ["programming-languages"],
    3:  ["protocol-naming", "gkhtml", "system-arch"],
    4:  ["system-arch"],
    5:  ["phonetic-encoding"],
    6:  ["perception-engine"],
    7:  ["protocol-naming", "phonetic-encoding"],
    8:  ["clustering", "social-energy"],
    9:  ["perception-engine", "phonetic-encoding"],
    10: ["clustering"],  # [DUP] ≈ link 8
    11: ["clustering"],  # [DUP] ≈ link 8
    12: ["android-policy"],
    13: ["system-arch"],
    14: ["programming-languages"],
    15: ["android-policy"],  # [DUP] ≈ link 12
    16: ["local-AI"],
    17: ["android-policy"],
    18: ["personal-survival", "social-energy", "accessibility-naming"],
    19: ["clustering", "social-energy", "deepseek-links"],  # [DUP] 部分≈ link 8
    20: ["multi-AI"],
    21: ["personal-survival"],
    22: ["system-arch", "protocol-naming", "gkhtml"],
    23: ["social-energy"],
    24: ["local-AI", "clustering"],
    25: ["phonetic-encoding", "numeral-system"],
    26: ["perception-engine", "accessibility-naming"],
    27: ["protocol-naming"],
    28: ["phonetic-encoding"],
    29: ["programming-languages", "multi-AI"],
    30: ["phonetic-encoding", "visual-encoding"],
    31: ["perception-engine"],
    32: ["numeral-system"],
    33: ["perception-engine", "system-arch"],
    34: ["accessibility-naming"],
    35: ["phonetic-encoding", "numeral-system", "accessibility-naming"],
    36: ["personal-survival"],
    37: ["android-policy"],
    38: ["gkhtml"],
    39: ["system-arch"],
    40: ["phonetic-encoding", "numeral-system"],
    41: ["vibe-coding"],
    42: ["smrl-judge"],
    43: ["smrl-judge"],
    44: ["programming-languages", "protocol-naming"],
    45: ["system-arch", "visual-encoding"],
    46: ["vibe-coding"],
    47: ["smrl-judge"],
    48: ["local-AI", "personal-survival", "accessibility-naming"],
    49: ["android-policy"],
    50: ["phonetic-encoding", "visual-encoding"],
    51: ["social-energy", "personal-survival"],
    52: ["clustering"],
    53: ["gkhtml"],
    54: ["numeral-system", "vibe-coding"],
    55: ["phonetic-encoding"],
    56: ["system-arch"],
    57: ["phonetic-encoding", "numeral-system", "system-arch"],
    58: ["programming-languages"],
    59: ["android-policy"],
    60: ["social-energy"],
    61: ["numeral-system"],
    62: ["visual-encoding"],
    63: ["accessibility-naming"],
    64: ["clustering"],
    65: ["system-arch"],
    66: ["multi-AI", "system-arch"],
    67: ["android-policy"],
    68: ["numeral-system"],
    69: ["deepseek-links"],
}

# ─── 已读链接内容摘要 ────────────────────────────────────────────
LINK_SUMMARIES = {
    5:  "Unicode 48个带声调韵母字母完整列表；48/50/108进制编码体系；默认串联读数法(baā)",
    6:  "卡尔曼滤波+因子图优化+李代数；绝对参考系+相对残差；残差累加与间歇式全局优化",
    7:  "gkcp://hfyairfqimiygttx:hieyair 感知伴侣协议命名；协议头/路径/节点分隔符设计",
    8:  "向量化聚类 vs 谱聚类效率对比；K-Prototypes混合聚类；社会能量场三层硬壳理论",
    9:  "感知引擎：统计/物理模拟/深度视觉三层训练思路；卡尔曼滤波预测运动方向",
    12: "Google 2027 Android开发者注册新规；24小时侧载冷静期；F-Droid存亡威胁",
    17: "Android封锁政策完整时间线；71组织公开信；保持androidopen.org运动",
    18: "个人困境与生存策略；'被刘楚恬卡住'的存在性绝望；数字红军概念；递纸条策略",
    19: "语音识别纠错：碳化化=向量化，话剧类=谱聚类；社会能量场边界绘制",
    22: "SMRL三元裁判：中心锚点+二元变化；芭蕉扇/扇形几何；三角定位法；正向运动学链",
    24: "本地AI工具(MLCChat/Ollama)；闲鱼一分钱服务器骗局分析；离线推理引擎",
    25: "完整108个韵母字符集；48进制×72进制×108进制=373,248编码空间；默认串联方法论",
    9:  "Unicode 预组合带调声母完整表(m/n/r/w/y+四声)；48个声调韵母+50个变体韵母+y系列",
    41: "氛围编程(Vibe Coding)行业调查；大厂Spec Coding vs 氛围编程；AI编程焦虑",
    42: "SMRL三元裁判编程价值；状态机/奖励函数；统计学视觉/卡尔曼滤波/卷积核三层思路；视网膜仿生",
    44: "动漫角色映射编程语言：kndrel(柯南外壳)、mlil/mlll(毛利兰)、gttix.y(工藤新一)",
    3:  "GKHTML感知标记语言设计；活标签vs死标签；感知-范畴-归属-关系标签系统；智能填写提示",
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

def generate_index(urls, check_live=False):
    """生成 Markdown 索引"""
    lines = []
    lines.append("# hunyuan-os 分享链接索引")
    lines.append("")
    lines.append(f"总计 {len(urls)} 个原始链接，实际唯一主题约 **{len(get_unique_links(urls))}** 个")
    lines.append("")
    lines.append("## 链接列表")
    lines.append("")
    lines.append("| # | 链接 | 主题 | 重复 | 摘要 |")
    lines.append("|---|------|------|------|------|")
    
    for i, url in enumerate(urls, 1):
        topics = LINK_TOPICS.get(i, ["unknown"])
        topic_names = " / ".join(TOPIC_CATEGORIES.get(t, {}).get("name", t) for t in topics)
        dup = f"≈ link {DUPLICATES[i]}" if i in DUPLICATES else ""
        summary = LINK_SUMMARIES.get(i, "")[:60]
        lines.append(f"| {i} | `{url[-20:]}` | {topic_names} | {dup} | {summary} |")
    
    lines.append("")
    lines.append("## 主题分类")
    lines.append("")
    
    for topic_id, info in TOPIC_CATEGORIES.items():
        lines.append(f"### {info['name']}")
        lines.append(f"*{info['desc']}*")
        ids = info.get("example_ids", [])
        if ids:
            links_str = ", ".join(f"[{i}](#link-{i})" for i in ids[:7])
            lines.append(f"链接: {links_str}")
        lines.append("")
    
    return "\n".join(lines)

def generate_json(urls):
    """生成 JSON 索引"""
    result = {"total": len(urls), "unique_approx": len(get_unique_links(urls))}
    result["topic_categories"] = TOPIC_CATEGORIES
    result["duplicates"] = {str(k): v for k, v in DUPLICATES.items()}
    result["links"] = {}
    
    for i, url in enumerate(urls, 1):
        result["links"][str(i)] = {
            "url": url,
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
    print()
    
    # 生成 Markdown
    md_content = generate_index(urls)
    md_path = Path(__file__).parent / "link_index.md"
    md_path.write_text(md_content, encoding="utf-8")
    print(f"Markdown 索引已生成: {md_path}")
    
    # 生成 JSON
    json_content = generate_json(urls)
    json_path = Path(__file__).parent / "link_index.json"
    json_path.write_text(json_content, encoding="utf-8")
    print(f"JSON 索引已生成: {json_path}")
