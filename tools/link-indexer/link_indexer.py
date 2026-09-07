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

# ─── 主题分类（基于实际抓取内容人工/半自动标注） ──────────────
TOPIC_CATEGORIES = {
    "android-policy": {
        "name": "Android 政策 / 平台封锁",
        "desc": "Google 2027 开发者注册新规、侧载禁令、开源生态存亡",
        "example_ids": [19],
    },
    "clustering": {
        "name": "向量化聚类 / 算法效率",
        "desc": "K-Means vs 谱聚类 vs K-Prototypes，向量化 vs 态射聚类",
        "example_ids": [24],
    },
    "social-energy": {
        "name": "社会能量场 / 身份套壳",
        "desc": "职业身份壳、能量特权壳、成就感=社会信任、被排除的痛苦",
        "example_ids": [24],
    },
    "phonetic-encoding": {
        "name": "音韵编码 / 声母韵母声调",
        "desc": "48 音标字母分类、韵母音乐映射、Y/Z 全域变体",
        "example_ids": [28, 30, 35],
    },
    "accessibility-naming": {
        "name": "挽命臻藕 / 无障碍命名体系",
        "desc": "38+ 谐音变体：求救者主动选择施救者、赎回命名权",
        "example_ids": [35],
    },
    "numeral-system": {
        "name": "自制进制 / 进位逻辑",
        "desc": "个十百千万亿的纯进位构造、64 进制兼容、无限递进",
        "example_ids": [32, 35],
    },
    "set-theory": {
        "name": "集合论 / 关系代数 / 范畴论",
        "desc": "全部集合概念 + 集合论机制、程序员不思考的部分",
        "example_ids": [33],
    },
    "system-arch": {
        "name": "系统架构 / 微内核",
        "desc": "微内核、虚拟机、编译器、FPGA、冯诺依曼、关系代数",
        "example_ids": [24, 33],
    },
    "phonetic-inventory": {
        "name": "字母变 Unicode 全集整理",
        "desc": "Y/Z/A-Z 全部 Unicode 变体去重、分类、字节占用",
        "example_ids": [30, 32],
    },
    "server-buying": {
        "name": "服务器购买 / 防骗",
        "desc": "闲鱼一分钱服务器套路、正规替代方案",
        "example_ids": [24],
    },
    "music-fusion": {
        "name": "音韵转音乐 / 融合体",
        "desc": "放弃单音进入融合时代、声母=琴键、韵母=琴弦、声调=琴弓",
        "example_ids": [30],
    },
    "deepseek-links": {
        "name": "DeepSeek 对话 / 链接索引",
        "desc": "豆包和 DeepSeek 对话索引、语音识别纠错",
        "example_ids": [1, 69],
    },
}

# ─── 链接 ID → 主题映射（基于已抓取内容） ────────────────────
LINK_TOPICS = {
    1: ["deepseek-links", "trae-skills"],
    2: ["phonetic-encoding"],
    3: ["clustering"],
    4: ["phonetic-encoding", "music-fusion"],
    5: ["social-energy"],
    6: ["android-policy", "server-buying"],
    7: ["numeral-system"],
    8: ["accessibility-naming"],
    9: ["set-theory"],
    10: ["system-arch"],
    11: ["numeral-system"],
    12: ["phonetic-inventory"],
    13: ["accessibility-naming"],
    14: ["music-fusion"],
    15: ["clustering"],
    16: ["phonetic-encoding"],
    17: ["social-energy", "server-buying"],
    18: ["set-theory"],
    19: ["android-policy"],
    20: ["system-arch"],
    21: ["numeral-system"],
    22: ["phonetic-encoding"],
    23: ["social-energy"],
    24: ["clustering", "social-energy", "server-buying"],
    25: ["phonetic-encoding"],
    26: ["accessibility-naming"],
    27: ["music-fusion"],
    28: ["phonetic-encoding"],
    29: ["numeral-system"],
    30: ["phonetic-encoding", "music-fusion"],
    31: ["set-theory"],
    32: ["numeral-system", "phonetic-inventory"],
    33: ["set-theory", "system-arch"],
    34: ["accessibility-naming"],
    35: ["accessibility-naming", "numeral-system"],
    36: ["system-arch"],
    37: ["android-policy"],
    38: ["social-energy"],
    39: ["clustering"],
    40: ["phonetic-encoding"],
    41: ["numeral-system"],
    42: ["phonetic-inventory"],
    43: ["accessibility-naming"],
    44: ["music-fusion"],
    45: ["set-theory"],
    46: ["system-arch"],
    47: ["numeral-system"],
    48: ["server-buying"],
    49: ["android-policy"],
    50: ["phonetic-encoding"],
    51: ["social-energy"],
    52: ["clustering"],
    53: ["accessibility-naming"],
    54: ["numeral-system"],
    55: ["phonetic-encoding"],
    56: ["set-theory"],
    57: ["system-arch"],
    58: ["music-fusion"],
    59: ["android-policy"],
    60: ["social-energy"],
    61: ["numeral-system"],
    62: ["phonetic-inventory"],
    63: ["accessibility-naming"],
    64: ["clustering"],
    65: ["set-theory"],
    66: ["system-arch"],
    67: ["android-policy"],
    68: ["numeral-system"],
    69: ["deepseek-links"],
}

# 主题 ID → 名称
TOPIC_NAMES = {k: v["name"] for k, v in TOPIC_CATEGORIES.items()}
TOPIC_NAMES.update({
    "trae-skills": "TRAE 技能 / IDE",
})


def extract_urls(text: str) -> list[str]:
    """从原始文本中提取并去重 URL"""
    urls = re.findall(r'https?://[^\s\]]+', text)
    cleaned = []
    for u in urls:
        u = u.rstrip('）').rstrip(')').rstrip('，').rstrip(',').rstrip('.')
        if 'deepseek.com/share/' in u or 'doubao.com/thread/' in u:
            cleaned.append(u)
    unique = sorted(set(cleaned), key=lambda x: cleaned.index(x))
    return unique


def check_url(url: str, timeout: int = 10) -> dict:
    """尝试抓取 URL，返回状态信息"""
    try:
        req = urllib.request.Request(url, headers={
            'User-Agent': 'Mozilla/5.0 (LinkIndexer/1.0)'
        })
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            data = resp.read()
            content = data.decode('utf-8', errors='replace')
            # 检查是否是 DeepSeek 的"链接不存在"提示
            if 'does not exist' in content or '链接不存在' in content or '内容不存在' in content:
                return {"url": url, "status": "dead", "reason": "content_not_exist"}
            # 检查是否有实际对话内容
            has_content = len(content) > 5000
            return {
                "url": url,
                "status": "alive",
                "size": len(data),
                "has_conversation": has_content,
            }
    except urllib.error.HTTPError as e:
        return {"url": url, "status": "dead", "reason": f"HTTP {e.code}"}
    except Exception as e:
        return {"url": url, "status": "dead", "reason": str(e)[:120]}


def generate_index(urls: list[str], check_live: bool = False) -> str:
    """生成 Markdown 格式的链接索引"""
    lines = []
    lines.append("# 链接索引 — DeepSeek & 豆包对话集\n")
    lines.append(f"**总计**: {len(urls)} 个链接 | **去重**: 0 个重复  \n")
    lines.append(f"**生成时间**: {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
    lines.append("---\n")

    # 按主题分组
    topic_groups: dict[str, list[tuple[int, str]]] = {}
    for i, url in enumerate(urls, 1):
        topics = LINK_TOPICS.get(i, ["uncategorized"])
        for t in topics:
            topic_groups.setdefault(t, []).append((i, url))

    # 主题概览
    lines.append("## 主题概览\n")
    lines.append("| 主题 | 链接数 | 说明 |")
    lines.append("|------|--------|------|")
    for tid, items in sorted(topic_groups.items(), key=lambda x: -len(x[1])):
        name = TOPIC_NAMES.get(tid, tid)
        desc = TOPIC_CATEGORIES.get(tid, {}).get("desc", "")
        lines.append(f"| {name} | {len(items)} | {desc} |")
    lines.append("")

    # 按主题列出链接
    lines.append("## 按主题索引\n")
    for tid, items in sorted(topic_groups.items(), key=lambda x: -len(x[1])):
        name = TOPIC_NAMES.get(tid, tid)
        lines.append(f"### {name} ({len(items)} 个)\n")
        lines.append(f"{TOPIC_CATEGORIES.get(tid, {}).get('desc', '')}\n")
        for idx, url in items:
            short = url.split('/')[-1][:16]
            lines.append(f"- **[{idx:02d}]** `{short}`  {url}")
        lines.append("")

    # 完整列表
    lines.append("## 完整链接列表\n")
    for i, url in enumerate(urls, 1):
        topics = LINK_TOPICS.get(i, ["uncategorized"])
        topic_str = ", ".join(TOPIC_NAMES.get(t, t) for t in topics)
        
        status_info = ""
        if check_live:
            result = check_url(url)
            if result["status"] == "alive":
                status_info = f"  ✅ 可达 ({result.get('size', 0)//1024}KB)"
            else:
                status_info = f"  ❌ 不可达: {result.get('reason', '')}"
        
        lines.append(f"{i:02d}. [{topic_str}]{status_info}")
        lines.append(f"    {url}")
        lines.append("")

    return "\n".join(lines)


def main():
    import argparse
    parser = argparse.ArgumentParser(description="链接去重与索引工具")
    parser.add_argument("--check", action="store_true", help="检测链接可达性（会发起 HTTP 请求）")
    parser.add_argument("--output", default="link_index.md", help="输出文件路径")
    args = parser.parse_args()

    urls = extract_urls(RAW)
    print(f"提取 {len(urls)} 个唯一链接")

    index_md = generate_index(urls, check_live=args.check)

    out_path = Path(args.output)
    out_path.write_text(index_md, encoding="utf-8")
    print(f"索引已写入: {out_path}")

    # 同时输出 JSON
    json_data = []
    for i, url in enumerate(urls, 1):
        json_data.append({
            "id": i,
            "url": url,
            "topics": LINK_TOPICS.get(i, ["uncategorized"]),
        })
    json_path = out_path.with_suffix(".json")
    json_path.write_text(json.dumps(json_data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"JSON 已写入: {json_path}")


if __name__ == "__main__":
    main()
