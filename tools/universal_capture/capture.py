#!/usr/bin/env python3
"""
通用链接捕获器 — 把任意网页封装进单文件容器
================================================
功能：
  1. 批量抓取 URL，去重、提取正文与元数据
  2. 将文本、图片、样式、脚本全部嵌进单个 .htmlz（ZIP 容器）
  3. 构建多维索引：时间 / 空间 / 相位 / 分类 / 声音 / 特征 / 统计
  4. 支持矢量缩略图（SVG 占位）替代位图以节省内存
  5. 输出清单 Markdown + 索引 JSON，可直接被 AI 消费

容器结构（HTMLZ = ZIP 包裹）：
  container.htmlz/
  ├── manifest.json            ← 外层公约（时间 / 空间 / 相位 / 统计元数据）
  ├── index.md                 ← 内层正文（人类可直接阅读）
  ├── pages/
  │   └── <slug>.html          ← 嵌入了 Base64 资源的完整页面
  └── vectors/
      └── <slug>.svg           ← 矢量缩略图（占位 / 真实 trace）

 Invention: 容器 = 公约数（外壳） + 资源（内嵌） + 映射表（索引）
 这与 hunyuan-os 内核的容器进制同构 — 文件本身就是空间，空间本身就是地址。
"""

import argparse
import base64
import hashlib
import html
import json
import os
import re
import sys
import time
import urllib.parse
import urllib.request
import urllib.error
import zipfile
from collections import Counter, defaultdict
from datetime import datetime, timezone
from html.parser import HTMLParser
from pathlib import Path
from typing import Any, Dict, List, Optional

# ---------- 配置 ----------
# 多 UA 池 — 某些站点对爬虫 UA 直接返回 403/空内容，轮换可降低被封概率
DEFAULT_UA = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
)
UA_POOL = [
    DEFAULT_UA,
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0",
]
TIMEOUT = 30
MAX_BYTES = 5 * 1024 * 1024  # 单 URL 上限 5 MB


# ========== 抓取层 ==========
class SimpleHTMLParser(HTMLParser):
    """极简提取器 — 无需第三方依赖"""

    def __init__(self):
        super().__init__()
        self.title = ""
        self.text_parts: List[str] = []
        self.links: List[str] = []
        self._in_title = False
        self._skip_tags = {"script", "style", "noscript"}
        self._skip = 0

    def handle_starttag(self, tag, attrs):
        if tag in self._skip_tags:
            self._skip += 1
        if tag == "title":
            self._in_title = True
        if tag == "a":
            for k, v in attrs:
                if k == "href":
                    self.links.append(v)

    def handle_endtag(self, tag):
        if tag in self._skip_tags:
            self._skip -= 1
        if tag == "title":
            self._in_title = False

    def handle_data(self, data):
        if self._skip > 0:
            return
        if self._in_title:
            self.title += data
        else:
            t = data.strip()
            if t:
                self.text_parts.append(t)

    @property
    def text(self) -> str:
        return " ".join(self.text_parts)


def fetch_url(url: str, ua: str = DEFAULT_UA, cookie: str = "") -> Optional[Dict[str, Any]]:
    """抓取单个 URL，返回结构化结果

    cookie: 可选，手动传入 Cookie 头以通过登录验证
    """
    headers = {"User-Agent": ua}
    if cookie:
        headers["Cookie"] = cookie
    req = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            content_type = resp.headers.get("Content-Type", "")
            raw = resp.read(MAX_BYTES)
            status = resp.status or resp.getcode()
    except (urllib.error.URLError, TimeoutError, ValueError) as e:
        return {"url": url, "error": str(e), "time": datetime.now(timezone.utc).isoformat()}
    except Exception as e:
        return {"url": url, "error": str(e), "time": datetime.now(timezone.utc).isoformat()}

    # 解码
    charset = "utf-8"
    if "charset=" in content_type:
        m = re.search(r"charset=([\w-]+)", content_type)
        if m:
            charset = m.group(1)
    try:
        body = raw.decode(charset, errors="replace")
    except LookupError:
        body = raw.decode("utf-8", errors="replace")

    # 启发式检测是否为 JS 渲染的 SPA
    is_js_rendered = _detect_js_rendered(body)

    parser = SimpleHTMLParser()
    try:
        parser.feed(body)
    except Exception:
        pass

    text_extracted = parser.text
    # 若 text 过短但 body 很长 → 大概率 JS 渲染无正文
    low_content_warning = len(text_extracted) < 50 and len(body) > 500

    return {
        "url": url,
        "final_url": resp.geturl(),
        "status": status,
        "content_type": content_type,
        "size_bytes": len(raw),
        "charset": charset,
        "title": parser.title.strip(),
        "text": text_extracted,
        "links": list(dict.fromkeys(parser.links)),
        "raw": body,
        "time": datetime.now(timezone.utc).isoformat(),
        "is_js_rendered": is_js_rendered,
        "low_content_warning": low_content_warning,
    }


def _detect_js_rendered(body: str) -> bool:
    """启发式判断是否为 JS 渲染页面"""
    if not body:
        return False
    indicators = [
        'id="__NEXT_DATA__"' in body,          # Next.js
        'id="root"' in body and body.count("<div") < 5,
        'id="app"' in body and body.count("<div") < 5,
        "window.__INITIAL_STATE__" in body,     # 通用 SPA
        "cf-browser-verification" in body,      # Cloudflare
        "browser-auth" in body,                  # DeepSeek 验证
    ]
    return any(indicators)


# ========== 多维索引层 ==========
class MultidimensionalIndex:
    """
    一条龙索引器 — 把每条捕获映射到多个寻址维度
    每个维度都是 容器(公约数) → 具体实体(公倍数) 的投影
    """

    def __init__(self):
        self.docs: List[Dict[str, Any]] = []
        self.dimension: Dict[str, Dict[str, List[str]]] = defaultdict(
            lambda: defaultdict(list)
        )
        self.counter: Counter = Counter()

    def add(self, doc: Dict[str, Any]) -> str:
        doc_id = hashlib.sha1(doc["url"].encode()).hexdigest()[:12]
        self.docs.append({**doc, "doc_id": doc_id})

        dt = doc.get("time", "")
        if dt:
            day = dt[:10]
            self.dimension["time"][day].append(doc_id)
            self.dimension["time_iso"][dt].append(doc_id)

        host = urllib.parse.urlparse(doc["url"]).netloc
        self.dimension["space_host"][host].append(doc_id)
        self.dimension["space_scheme"][
            urllib.parse.urlparse(doc["url"]).scheme
        ].append(doc_id)

        ct = doc.get("content_type", "")
        if ct:
            self.dimension["phase_content_type"].setdefault(ct, []).append(doc_id)

        status = doc.get("status", 0)
        self.dimension["category_status"][str(status)].append(doc_id)
        if doc.get("error"):
            self.dimension["category_error"].setdefault(
                self._bucket_error(doc["error"]), []
            ).append(doc_id)

        if host:
            initial = host[0].lower()
            self.dimension["sound_initial"][initial].append(doc_id)

        title = doc.get("title", "")
        if title:
            self.dimension["feature_has_title"]["yes" if title else "no"].append(doc_id)
            for word in self._tokenize(title):
                self.dimension["feature_title_token"][word].append(doc_id)

        sz = doc.get("size_bytes", 0)
        self.dimension["stat_size_bucket"][self._bucket_size(sz)].append(doc_id)
        self.counter["total"] += 1
        self.counter["ok" if not doc.get("error") else "fail"] += 1
        self.counter["bytes"] += sz
        self.dimension["presence_count"]["1" if not doc.get("error") else "0"].append(doc_id)

        return doc_id

    @staticmethod
    def _bucket_size(n: int) -> str:
        if n < 1024:
            return "<1KB"
        if n < 10 * 1024:
            return "1-10KB"
        if n < 100 * 1024:
            return "10-100KB"
        if n < 1024 * 1024:
            return "100KB-1MB"
        return ">1MB"

    @staticmethod
    def _bucket_error(msg: str) -> str:
        msg = msg.lower()
        if "timed out" in msg or "timeout" in msg:
            return "timeout"
        if "403" in msg or "forbidden" in msg:
            return "forbidden"
        if "404" in msg:
            return "not_found"
        if "429" in msg:
            return "rate_limited"
        if "name or service" in msg or "getaddrinfo" in msg:
            return "dns_error"
        return "other"

    @staticmethod
    def _tokenize(text: str) -> List[str]:
        return re.findall(r"[\w一-鿿]+", text.lower())

    def summary(self) -> Dict[str, Any]:
        return {
            "docs_total": len(self.docs),
            "docs_ok": self.counter["ok"],
            "docs_fail": self.counter["fail"],
            "bytes_total": self.counter["bytes"],
            "dimensions": {
                dim: {k: len(vs) for k, vs in entries.items()}
                for dim, entries in self.dimension.items()
            },
        }

    def as_dict(self) -> Dict[str, Any]:
        return {
            "docs": self.docs,
            "dimension": dict(self.dimension),
            "counter": dict(self.counter),
        }


# ========== 矢量占位层 ==========
def make_vector_placeholder(title: str) -> str:
    """
    生成 SVG 矢量占位符 — 替代位图缩略图以节省内存
    真正 scene→vector 的能力需要 potrace / VTracer 等 C 库，
    这里用文字排布 + 几何色块模拟，体积通常 < 200 字节
    """
    safe_title = html.escape(title[:30] if title else "UNTITLED")
    hash_val = int(hashlib.sha1(title.encode()).hexdigest()[:8], 16)
    colors = [
        "#1f2937", "#374151", "#4b5563", "#6b7280",
        "#111827", "#0f172a", "#1e293b", "#334155",
    ]
    bg = colors[hash_val % len(colors)]
    fg = "#f3f4f6"
    return f"""<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="320" height="180" viewBox="0 0 320 180">
  <rect width="320" height="180" fill="{bg}"/>
  <text x="160" y="90" font-family="monospace" font-size="14" fill="{fg}" text-anchor="middle">{safe_title}</text>
</svg>"""


# ========== 容器封装层 ==========
def build_container(
    registry: List[Dict[str, Any]],
    output_path: str,
) -> str:
    """把所有捕获结果打包进单个 .htmlz 文件"""
    out = Path(output_path)
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        manifest = {
            "hunyuan_os": {
                "version": "0.40",
                "container_type": "universal_capture_htmlz",
            },
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "docs_total": len(registry),
            "docs": [
                {
                    "doc_id": r.get("doc_id", ""),
                    "url": r["url"],
                    "title": r.get("title", ""),
                    "status": r.get("status", 0),
                    "time": r.get("time", ""),
                    "size_bytes": r.get("size_bytes", 0),
                    "phase_day": (r.get("time") or "")[:10],
                    "space_host": urllib.parse.urlparse(r["url"]).netloc,
                }
                for r in registry
            ],
        }
        z.writestr("manifest.json", json.dumps(manifest, ensure_ascii=False, indent=2))

        md = ["# 通用链接捕获 — 索引\n\n"]
        for r in registry:
            status = r.get("status", "ERR")
            md.append(f"## [{r.get('title', 'UNTITLED')}](pages/{r['doc_id']}.html)\n")
            md.append(f"- URL: `{r['url']}`\n")
            md.append(f"- 状态: `{status}` / 大小: {r.get('size_bytes', 0)} bytes\n")
            md.append(f"- 时间: `{r.get('time', '')}`\n")
            md.append(f"- 链接数: {len(r.get('links', []))}\n\n")
            text = r.get("text", "")
            snippet = text[:200].replace("\n", " ")
            md.append(f"> ...{snippet}...\n\n---\n\n")
        z.writestr("index.md", "".join(md))

        for r in registry:
            page_html = _embed_to_single_html(r)
            z.writestr(f"pages/{r['doc_id']}.html", page_html)

        for r in registry:
            svg = make_vector_placeholder(r.get("title", r["url"]))
            z.writestr(f"vectors/{r['doc_id']}.svg", svg)

    return str(out)


def _embed_to_single_html(record: Dict[str, Any]) -> str:
    """把 HTML 中的图片转为 data URI 内嵌"""
    raw = record.get("raw", "")
    if not raw:
        return "<html><body>FAILED_LOAD</body></html>"

    def _inline_image(m: re.Match) -> str:
        url = m.group(1)
        try:
            req = urllib.request.Request(url, headers={"User-Agent": DEFAULT_UA})
            with urllib.request.urlopen(req, timeout=10) as r:
                data = r.read(512 * 1024)
            ct = r.headers.get("Content-Type", "image/png")
            b64 = base64.b64encode(data).decode("ascii")
            return f'src="data:{ct};base64,{b64}"'
        except Exception:
            return m.group(0)

    raw = re.sub(r'<img[^>]+src="(https?://[^"]{3,200})"', _inline_image, raw)
    return raw


# ========== 顶层接口 ==========
def main():
    ap = argparse.ArgumentParser(
        description="通用链接捕获器 — 封装网页到单文件容器"
    )
    ap.add_argument(
        "urls",
        nargs="*",
        help="要捕获的 URL（若不传则从 stdin 读取, 每行一个）",
    )
    ap.add_argument(
        "-o",
        "--output",
        default="capture.htmlz",
        help="输出文件路径（默认 ./capture.htmlz）",
    )
    ap.add_argument(
        "--keep-raw-html-dir",
        default=None,
        help="可选：把原始 HTML 也写入这个目录（便于调试/比对）",
    )
    ap.add_argument(
        "--stats-only",
        action="store_true",
        help="仅打印统计信息，不生成容器文件",
    )
    ap.add_argument(
        "--index-output",
        default=None,
        help="独立输出索引 JSON 文件路径（默认 <output>.index.json）",
    )
    args = ap.parse_args()

    urls = args.urls if args.urls else [line.strip() for line in sys.stdin if line.strip()]
    seen = set()
    unique_urls = []
    for u in urls:
        if u not in seen:
            seen.add(u)
            unique_urls.append(u)
    dup_count = len(urls) - len(unique_urls)

    print(f"[捕获器] 共收到 {len(urls)} 个链接，去重后 {len(unique_urls)} 个（丢弃 {dup_count} 个重复）")

    index = MultidimensionalIndex()
    registry: List[Dict[str, Any]] = []
    for i, url in enumerate(unique_urls, 1):
        print(f"[捕获器] ({i}/{len(unique_urls)}) 获取 {url[:80]} ...")
        rec = fetch_url(url)
        doc_id = index.add(rec)
        rec["doc_id"] = doc_id
        registry.append(rec)
        if args.keep_raw_html_dir:
            out_dir = Path(args.keep_raw_html_dir)
            out_dir.mkdir(parents=True, exist_ok=True)
            (out_dir / f"{doc_id}.html").write_text(
                rec.get("raw", ""), encoding="utf-8", errors="replace"
            )

    stats = index.summary()
    print("\n=== 统计 ===")
    print(json.dumps(stats, ensure_ascii=False, indent=2))

    if args.stats_only:
        return

    container_path = args.output
    build_container(registry, container_path)
    print(f"\n[捕获器] 容器已写入 {container_path}")

    idx_path = args.index_output or f"{args.output}.index.json"
    with open(idx_path, "w", encoding="utf-8") as f:
        json.dump(index.as_dict(), f, ensure_ascii=False, indent=2)
    print(f"[捕获器] 索引已写入 {idx_path}")


if __name__ == "__main__":
    main()
