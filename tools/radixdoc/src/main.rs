// radixdoc - 面向进制寻址的文档系统生成器 (Rust 零依赖)
//
// 输入: 对话 Markdown (DeepSeek 分享导出格式, 以 "## 用户 / ## 助手" 分节)
// 输出: 工作空间(目录页) + 每份文档一页 + 注册清单 + 清单 + 解析树 + 验证树
//
// 术语: 清单 / 注册清单 / 工作空间 / 时间版本 / 思维入口 / 工作版本
// 容器化: 每份文档 = 一个容器 = 一个加法地址; 关联 = 符号共享 (进制寻址, 无命名空间)
// 零前端语言: HTML 由 Rust 直接生成, 页面之间只用链接 (指针 + 返回目录)
//
// 用法:
//   radixdoc <对话.md> [输出目录]
//   默认输出到 <对话.md 所在目录>/site

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

struct Unit {
    addr: usize,
    user: String,
    assistant: String,
    title: String,
    symbols: Vec<String>,
    keywords: Vec<String>,
    related: Vec<(usize, u64)>,
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn fnv(text: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn push_token(tok: &str, out: &mut Vec<String>) {
    let t = tok.trim();
    let n = t.chars().count();
    if n < 2 || n > 30 {
        return;
    }
    if t.contains('\n') || t.contains('`') {
        return;
    }
    if t.chars().all(|c| c.is_ascii_digit() || c.is_ascii_punctuation() || c.is_whitespace()) {
        return;
    }
    out.push(t.to_string());
}

fn symbols_of(text: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for (i, p) in text.split('`').enumerate() {
        if i % 2 == 1 {
            push_token(p, &mut v);
        }
    }
    let mut rest = text;
    while let Some(a) = rest.find('《') {
        if let Some(b) = rest[a + 3..].find('》') {
            push_token(&rest[a + 3..a + 3 + b], &mut v);
            rest = &rest[a + 3 + b..];
        } else {
            break;
        }
    }
    v.sort();
    v.dedup();
    v
}

fn is_cjk(c: char) -> bool {
    let u = c as u32;
    (0x4e00..=0x9fff).contains(&u)
}

const STOP: &[&str] = &[
    "一个", "我们", "可以", "这个", "那个", "就是", "因为", "所以", "如果", "不是", "什么", "没有",
    "自己", "需要", "进行", "以及", "或者", "但是", "这样", "那么", "时候", "问题", "方法", "使用",
    "通过", "已经", "这些", "那些", "他们", "你们", "它们", "并且", "然后", "现在", "还是", "还有",
    "一些", "很多", "可能", "应该", "必须", "要求", "文档", "系统", "数据", "内容", "功能", "方式",
    "情况", "一个", "这是", "我的", "你的", "他的", "一种", "这种", "它不", "是不", "个容器",
];

fn keywords_of(text: &str) -> Vec<String> {
    let mut v = symbols_of(text);
    let mut word = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            word.push(ch.to_ascii_lowercase());
        } else {
            if word.chars().count() >= 3 && word.chars().any(|c| c.is_ascii_alphabetic()) {
                v.push(word.clone());
            }
            word.clear();
        }
    }
    if word.chars().count() >= 3 {
        v.push(word.clone());
    }
    let chars: Vec<char> = text.chars().filter(|c| is_cjk(*c)).collect();
    for n in [2usize, 3] {
        for w in chars.windows(n) {
            let s: String = w.iter().collect();
            if !STOP.contains(&s.as_str()) {
                v.push(s);
            }
        }
    }
    v.sort();
    v.dedup();
    v
}

fn parse_messages(content: &str) -> Vec<(bool, String)> {
    let mut out: Vec<(bool, String)> = Vec::new();
    let mut cur: Option<(bool, String)> = None;
    for line in content.lines() {
        let t = line.trim_end();
        if t == "## 用户" {
            if let Some(m) = cur.take() {
                out.push(m);
            }
            cur = Some((true, String::new()));
        } else if t == "## 助手" {
            if let Some(m) = cur.take() {
                out.push(m);
            }
            cur = Some((false, String::new()));
        } else if let Some((_, buf)) = cur.as_mut() {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    if let Some(m) = cur.take() {
        out.push(m);
    }
    out
}

fn title_of(user: &str) -> String {
    for line in user.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        let candidate = l.trim_start_matches('#').trim();
        if !candidate.is_empty() {
            let s: String = candidate.chars().take(36).collect();
            return s;
        }
    }
    "(无标题思维入口)".to_string()
}

fn build_units(msgs: &[(bool, String)]) -> Vec<Unit> {
    let mut units: Vec<Unit> = Vec::new();
    let mut i = 0;
    while i < msgs.len() {
        if msgs[i].0 {
            let user = msgs[i].1.clone();
            let mut assistant = String::new();
            if i + 1 < msgs.len() && !msgs[i + 1].0 {
                assistant = msgs[i + 1].1.clone();
                i += 1;
            }
            let mut symbols = symbols_of(&user);
            for s in symbols_of(&assistant) {
                if !symbols.contains(&s) {
                    symbols.push(s);
                }
            }
            let addr = units.len() + 1;
            let keywords = keywords_of(&format!("{}{}", user, assistant));
            units.push(Unit {
                addr,
                title: title_of(&user),
                user,
                assistant,
                symbols,
                keywords,
                related: Vec::new(),
            });
        }
        i += 1;
    }
    units
}

fn build_related(units: &mut [Unit]) {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for u in units.iter() {
        for s in &u.keywords {
            *freq.entry(s.clone()).or_insert(0) += 1;
        }
    }
    let sets: Vec<HashSet<String>> = units
        .iter()
        .map(|u| {
            u.keywords
                .iter()
                .filter(|s| *freq.get(*s).unwrap_or(&1) >= 2)
                .cloned()
                .collect()
        })
        .collect();
    for idx in 0..units.len() {
        let mut score: Vec<(usize, u64)> = Vec::new();
        for j in 0..units.len() {
            if j == idx {
                continue;
            }
            let mut sc = 0u64;
            for s in sets[j].iter() {
                if sets[idx].contains(s) {
                    sc += 1000 / freq[s] as u64;
                }
            }
            if sc > 0 {
                score.push((units[j].addr, sc));
            }
        }
        score.sort_by(|a, b| b.1.cmp(&a.1));
        score.truncate(4);
        units[idx].related = score;
    }
}

fn page(addr: usize, _total: usize) -> String {
    format!("{:04}", addr)
}

fn render_dir(units: &[Unit], work: &str) -> String {
    let mut cards = String::new();
    for u in units {
        let wv = format!("{:06x}", fnv(&format!("{}{}", u.user, u.assistant)) & 0xffffff);
        let mut rels = String::new();
        for (a, sc) in &u.related {
            rels.push_str(&format!(
                "<a class=\"rel\" href=\"pages/{}.html\" title=\"共享权重 {}\">→{}</a> ",
                page(*a, units.len()),
                sc,
                a
            ));
        }
        cards.push_str(&format!(
            "<div class=\"box\" id=\"c{addr}\"><a class=\"addr\" href=\"pages/{p}.html\">地址 {addr}</a>\
<a class=\"title\" href=\"pages/{p}.html\">{title}</a>\
<div class=\"meta\">时间版本 v{addr} · 工作版本 {wv} · 符号 {nsym} · 回答 {alen} 字</div>\
<div class=\"links\">指向: {rels}<a class=\"back\" href=\"#top\">↑ 回目录</a></div></div>\n",
            addr = u.addr,
            p = page(u.addr, units.len()),
            title = esc(&u.title),
            nsym = u.symbols.len(),
            alen = u.assistant.chars().count(),
            rels = if rels.is_empty() { "（独立容器）".to_string() } else { rels }
        ));
    }
    format!(
        "<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\">\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
<title>工作空间 · 进制寻址文档系统</title><style>{css}</style></head>\
<body id=\"top\"><h1>工作空间 · 进制寻址文档系统</h1>\
<p class=\"lead\">容器化材料 · 加法地址索引 · 无命名空间<br>\
清单 {n} 个容器 · 工作空间 <code>{work}</code> · 中心锚点 gtxone 工藤新一</p>\
<div class=\"toolbar\"><a href=\"清单.md\">清单</a><a href=\"注册清单.md\">注册清单</a>\
<a href=\"解析树.html\">解析树</a><a href=\"验证树.html\">验证树</a>\
<a href=\"介绍向导.html\">介绍向导</a><a href=\"注册向导.html\">注册向导</a></div>\
{cards}<p class=\"foot\">公路修到哪里 · 统计地址就索引到哪里</p></body></html>",
        css = CSS,
        n = units.len(),
        work = esc(work),
        cards = cards
    )
}

fn render_page(u: &Unit, total: usize) -> String {
    let wv = format!("{:06x}", fnv(&format!("{}{}", u.user, u.assistant)) & 0xffffff);
    let prev = if u.addr > 1 {
        format!("<a href=\"{}.html\">← 地址 {}</a>", page(u.addr - 1, total), u.addr - 1)
    } else {
        "<span class=\"dim\">← 已是首个</span>".to_string()
    };
    let next = if u.addr < total {
        format!("<a href=\"{}.html\">地址 {} →</a>", page(u.addr + 1, total), u.addr + 1)
    } else {
        "<span class=\"dim\">已是末个 →</span>".to_string()
    };
    let mut rels = String::new();
    for (a, sc) in &u.related {
        rels.push_str(&format!(
            "<li><a href=\"{}.html\">地址 {} </a>· 共享权重 {}</li>\n",
            page(*a, total),
            a,
            sc
        ));
    }
    let mut syms = String::new();
    for s in &u.symbols {
        syms.push_str(&format!("<code>{}</code> ", esc(s)));
    }
    let mut kws = String::new();
    for s in u.keywords.iter().take(24) {
        kws.push_str(&format!("<code>{}</code> ", esc(s)));
    }
    format!(
        "<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\">\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
<title>地址 {addr} · {title}</title><style>{css}</style></head>\
<body><div class=\"topbar\"><a class=\"back\" href=\"../工作空间.html\">← 返回目录</a>\
<span class=\"addr\">地址 {addr} / {total}</span>\
<span class=\"meta\">时间版本 v{addr} · 工作版本 {wv}</span></div>\
<h1>{title}</h1>\
<h2>思维入口 · 提问</h2><pre>{user}</pre>\
<h2>回答 · 工作版本 {wv}</h2><pre>{ans}</pre>\
<h2>符号（容器材料 · 进制寻址）</h2><p>{syms}</p>\
<h2>关联材料（时间串联用）</h2><p>{kws}</p>\
<h2>验证树 · 相互指向</h2><ul>{rels}</ul>\
<h2>时间串联</h2><p>{prev} &nbsp;|&nbsp; {next}</p>\
<p class=\"foot\"><a class=\"back\" href=\"../工作空间.html\">← 返回目录</a> · \
<a href=\"../解析树.html\">解析树</a> · <a href=\"../验证树.html\">验证树</a></p>\
</body></html>",
        addr = u.addr,
        total = total,
        title = esc(&u.title),
        css = CSS,
        wv = wv,
        user = esc(&u.user),
        ans = esc(&u.assistant),
        syms = if syms.is_empty() { "（无符号）".to_string() } else { syms },
        kws = if kws.is_empty() { "（无关联材料）".to_string() } else { kws },
        rels = if rels.is_empty() { "<li>（独立容器，无强关联）</li>".to_string() } else { rels },
        prev = prev,
        next = next
    )
}

fn render_registry(units: &[Unit]) -> String {
    let mut s = String::from("# 注册清单 · 进制寻址文档系统\n\n");
    s.push_str("门牌号即加法地址。术语：清单 / 注册清单 / 工作空间 / 时间版本 / 思维入口 / 工作版本。\n\n");
    s.push_str("| 地址 | 思维入口 | 时间版本 | 工作版本 | 符号数 | 指向 | 页 |\n");
    s.push_str("|---:|---|---|---|---:|---|---|\n");
    for u in units {
        let wv = format!("{:06x}", fnv(&format!("{}{}", u.user, u.assistant)) & 0xffffff);
        let rels: Vec<String> = u.related.iter().map(|(a, _)| a.to_string()).collect();
        s.push_str(&format!(
            "| {} | {} | v{} | {} | {} | {} | [{}](pages/{}.html) |\n",
            u.addr,
            u.title.replace('|', "\\|"),
            u.addr,
            wv,
            u.symbols.len(),
            if rels.is_empty() { "-".to_string() } else { rels.join(",") },
            u.addr,
            page(u.addr, units.len())
        ));
    }
    s
}

fn render_list(units: &[Unit]) -> String {
    let mut s = String::from("# 清单\n\n");
    s.push_str("一个容器一行；地址即门牌号。\n\n");
    for u in units {
        s.push_str(&format!("- [地址 {}] {}\n", u.addr, u.title));
    }
    s
}

// ============ 面向程序的机器格式 (进制寻址数据层, 无 UI) ============
// chain 格式与仓库 rules/cmd.chain 一致: 字段用 | 分隔, 一行一条记录
// 程序按加法地址直接索引, 不需要渲染

// chain 字段转义: 保留可逆的机器格式 (换行/竖线/反斜杠)
fn cesc(s: &str) -> String {
    let mut o = String::new();
    for c in s.chars() {
        match c {
            '\\' => o.push_str("\\\\"),
            '|' => o.push_str("\\u007c"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c => o.push(c),
        }
    }
    o
}

fn wv_of(u: &Unit) -> String {
    format!("{:06x}", fnv(&format!("{}{}", u.user, u.assistant)) & 0xffffff)
}

fn eligible_keywords(units: &[Unit]) -> HashSet<String> {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for u in units.iter() {
        for s in &u.keywords {
            *freq.entry(s.clone()).or_insert(0) += 1;
        }
    }
    freq.into_iter()
        .filter(|(_, f)| *f >= 2)
        .map(|(s, _)| s)
        .collect()
}

fn out_workspace_chain(units: &[Unit]) -> String {
    let mut s = String::from("# 进制寻址工作空间 · 地址|时间版本|工作版本|符号数|指向|标题\n");
    for u in units {
        let rels: Vec<String> = u.related.iter().map(|(a, _)| a.to_string()).collect();
        s.push_str(&format!(
            "{}|v{}|{}|{}|{}|{}\n",
            u.addr,
            u.addr,
            wv_of(u),
            u.symbols.len(),
            rels.join(","),
            u.title.replace('|', "/")
        ));
    }
    s
}

fn out_symbol_index(units: &[Unit]) -> String {
    let eligible = eligible_keywords(units);
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();
    for u in units {
        for s in &u.keywords {
            if eligible.contains(s) {
                idx.entry(s.clone()).or_default().push(u.addr);
            }
        }
    }
    let mut keys: Vec<&String> = idx.keys().collect();
    keys.sort();
    let mut s = String::from("# 符号索引 · 符号|出现地址(逗号分隔)\n");
    for k in keys {
        let addrs: Vec<String> = idx[k].iter().map(|a| a.to_string()).collect();
        s.push_str(&format!("{}|{}\n", k.replace('|', "/"), addrs.join(",")));
    }
    s
}

fn out_parse_chain(units: &[Unit]) -> String {
    let mut s = String::from("# 解析树 · 父地址|子地址 (根 = 0)\n");
    for u in units {
        s.push_str(&format!("0|{}\n", u.addr));
    }
    s
}

fn out_verify_chain(units: &[Unit]) -> String {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut s = String::from("# 验证树 · 地址A|地址B|共享权重 (无向, 只记一次)\n");
    for u in units {
        for (b, w) in &u.related {
            let a = u.addr;
            let key = if a < *b { (a, *b) } else { (*b, a) };
            if seen.insert(key) {
                s.push_str(&format!("{}|{}|{}\n", key.0, key.1, w));
            }
        }
    }
    s
}

fn out_prompt_chain(units: &[Unit]) -> String {
    let mut s = String::from("# 提示词清单 · 地址|时间版本|目标|指向|关键词\n");
    for u in units {
        let rels: Vec<String> = u.related.iter().map(|(a, _)| a.to_string()).collect();
        let kws: Vec<String> = u.keywords.iter().take(8).cloned().collect();
        s.push_str(&format!(
            "{}|v{}|{}|{}|{}\n",
            u.addr,
            u.addr,
            u.title.replace('|', "/"),
            rels.join(","),
            kws.join(",").replace('|', "/")
        ));
    }
    s
}

fn out_container_chain(units: &[Unit]) -> String {
    let mut s = String::from("# 容器全文 · 地址|时间版本|工作版本|标题|符号|关联|问|答\n");
    for u in units {
        let syms: Vec<String> = u.symbols.iter().map(|x| cesc(x)).collect();
        let rels: Vec<String> = u.related.iter().map(|(a, w)| format!("{}:{}", a, w)).collect();
        s.push_str(&format!(
            "{}|v{}|{}|{}|{}|{}|{}|{}\n",
            u.addr,
            u.addr,
            wv_of(u),
            cesc(&u.title),
            syms.join(","),
            rels.join(","),
            cesc(&u.user),
            cesc(&u.assistant)
        ));
    }
    s
}

fn render_intro() -> String {
    let body = r#"<h1>介绍向导 · 这套文档系统是什么</h1>
<p class="lead">工作空间 = 面向进制寻址的文档系统。一个容器 = 一个插件 = 一个三维立体进制节点；用加法串联选址，乘法只用来清点节点数。</p>
<h2>核心术语</h2>
<ul>
<li><b>清单</b>：一个容器一行，记录门牌号与思维入口。</li>
<li><b>注册清单</b>：门牌号（加法地址）+ 清单 + 文件 + 状态 + 版本 + 时间。</li>
<li><b>工作空间</b>：所有容器组成的总目录页（本页的入口）。</li>
<li><b>时间版本</b>：容器在时间轴上的顺序号（v1、v2……），就是加法地址。</li>
<li><b>思维入口</b>：一次提问及其回答，作为一个可索引的容器。</li>
<li><b>工作版本</b>：容器内容的哈希，内容变了版本就变。</li>
</ul>
<h2>逻辑解释</h2>
<ol>
<li>声明容器 → 分配加法地址；公路修到容器 → 容器内符号热可用（非锁死）。</li>
<li>清点各容器符号 → 生成乘法口诀表 → 得到当前最大功率能到达的覆盖范畴。</li>
<li>每个容器有一个相位（前/后/左/右/上/下/中/腰/角），相位进制由此间接产生。</li>
<li>容器之间共享符号 → 相互指向 → 形成解析树与验证树。</li>
</ol>
<h2>阅读顺序</h2>
<p>工作空间 → 任一地址页（提问 → 回答 → 符号 → 验证树 → 时间串联）→ 关联页 → 返回目录。每页底部都有「返回目录」。</p>"#;
    let mut s = String::from("<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\"><title>介绍向导</title><style>");
    s.push_str(CSS);
    s.push_str("</style></head><body><p><a href=\"工作空间.html\">← 返回目录</a></p>");
    s.push_str(body);
    s.push_str("<p class=\"foot\"><a href=\"工作空间.html\">← 返回目录</a> · <a href=\"注册向导.html\">注册向导</a></p></body></html>");
    s
}

fn render_signup() -> String {
    let body = r#"<h1>注册向导 · 如何把文件登记成容器</h1>
<p class="lead">登记即声明：声明它是容器，它才在容器范畴内，才能被加法地址索引。</p>
<h2>字段表</h2>
<pre>门牌号 | 清单 | 文件 | 状态 | 版本 | 时间</pre>
<p>门牌号就是外面的公路地址（一个数字）；文件是括号里的房子；状态/版本/时间是房子的工作标记。</p>
<h2>手动登记（沙箱指令环境 cmdenv）</h2>
<pre>cmdenv reg docs/xxx.md gttx:// 工作中 v1 2026-09-13
cmdenv reglist</pre>
<h2>自动登记（radixdoc）</h2>
<pre>radixdoc &lt;对话.md&gt; [输出目录]
# 自动把每个思维入口登记为容器, 生成注册清单.md 与 注册清单页</pre>
<h2>状态取值</h2>
<ul><li>工作中</li><li>修改中</li><li>已完成</li></ul>
<h2>权限与断点</h2>
<p>本系统用加法地址掌控文档：索引权限、生成权限、读取权限、可视化感知权限，以及断点续接的写入/读取/插入处理。</p>"#;
    let mut s = String::from("<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\"><title>注册向导</title><style>");
    s.push_str(CSS);
    s.push_str("</style></head><body><p><a href=\"工作空间.html\">← 返回目录</a></p>");
    s.push_str(body);
    s.push_str("<p class=\"foot\"><a href=\"工作空间.html\">← 返回目录</a> · <a href=\"介绍向导.html\">介绍向导</a></p></body></html>");
    s
}

fn render_parse_tree(units: &[Unit]) -> String {
    let mut s = String::from("<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\">\
<title>解析树</title><style>");
    s.push_str(CSS);
    s.push_str("</style></head><body><p><a href=\"工作空间.html\">← 返回目录</a></p>\
<h1>解析树 · 思维如何逐层发散</h1><ul class=\"tree\">\n");
    s.push_str("<li>工作空间（根）\n<ul>\n");
    for u in units {
        s.push_str(&format!(
            "<li><a href=\"pages/{}.html\">地址 {} · {}</a>\n<ul><li>符号 {} 个：{}</li></ul></li>\n",
            page(u.addr, units.len()),
            u.addr,
            esc(&u.title),
            u.symbols.len(),
            esc(&u.symbols.iter().take(12).cloned().collect::<Vec<String>>().join(" · "))
        ));
    }
    s.push_str("</ul></li></ul>");
    s.push_str("<p class=\"foot\"><a href=\"工作空间.html\">← 返回目录</a></p></body></html>");
    s
}

fn render_verify_tree(units: &[Unit]) -> String {
    let mut s = String::from("<!DOCTYPE html><html lang=\"zh\"><head><meta charset=\"UTF-8\">\
<title>验证树</title><style>");
    s.push_str(CSS);
    s.push_str("</style></head><body><p><a href=\"工作空间.html\">← 返回目录</a></p>\
<h1>验证树 · 相互验证与前后串联</h1>\n");
    s.push_str("<p class=\"lead\">每个思维入口 = 提问 → 回答 → 与其它容器共享符号（共享权重越高，验证越强）。</p><ul class=\"tree\">\n");
    for u in units {
        let verdict = if u.related.is_empty() {
            "未与他人交叉验证"
        } else {
            "已与其它容器交叉验证"
        };
        s.push_str(&format!(
            "<li><a href=\"pages/{}.html\">地址 {} · {}</a> · <b>{}</b>\n<ul>",
            page(u.addr, units.len()),
            u.addr,
            esc(&u.title),
            verdict
        ));
        for (a, sc) in &u.related {
            s.push_str(&format!(
                "<li>验证 → 地址 {}（共享权重 {}）</li>",
                a, sc
            ));
        }
        s.push_str("</ul></li>\n");
    }
    s.push_str("</ul><p class=\"foot\"><a href=\"工作空间.html\">← 返回目录</a></p></body></html>");
    s
}

const CSS: &str = "body{font-family:system-ui,-apple-system,sans-serif;max-width:900px;margin:0 auto;padding:24px;background:#0e0e12;color:#e8e8f0;line-height:1.6}\
a{color:#a58bd6;text-decoration:none}a:hover{text-decoration:underline}\
h1{color:#f0d090;font-size:1.4rem}h2{color:#a58bd6;font-size:1.05rem;margin-top:1.4em;border-bottom:1px solid #26263a;padding-bottom:4px}\
pre{background:#16161e;border:1px solid #26263a;border-radius:6px;padding:12px;overflow:auto;white-space:pre-wrap;word-break:break-word;font-size:.86rem}\
code{background:#1c1c26;padding:2px 6px;border-radius:4px;font-size:.82rem;margin:2px}\
.box{border-left:3px solid #7b5ea7;background:#181822;border-radius:6px;padding:10px 14px;margin:10px 0}\
.addr{font-family:monospace;color:#e0b060;margin-right:10px;font-size:.9rem}\
.title{font-weight:600;color:#e8e8f0}.meta{color:#8a8aa0;font-size:.8rem;margin-top:4px}\
.links{margin-top:6px;font-size:.85rem}.rel{margin-right:6px}.dim{color:#666}.foot{color:#8a8aa0;font-size:.85rem;margin-top:2em;border-top:1px solid #26263a;padding-top:10px}\
.topbar{display:flex;gap:12px;flex-wrap:wrap;align-items:center;margin-bottom:8px;font-size:.88rem}\
.back{color:#e0b060}.toolbar a{margin-right:12px}\
.lead{color:#b8b8cc}.tree ul{list-style:none;border-left:1px dashed #3a3a52;margin-left:8px;padding-left:16px}";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("用法: radixdoc <对话.md> [输出目录] [--html]");
        println!("  默认(面向程序): 工作空间.chain / 符号索引.chain / 解析树.chain / 验证树.chain / 提示词.chain / 容器全文.chain");
        println!("  --html: 额外生成人类可读页面 (目录 + 每页, 可选, 非主产物)");
        println!("  每份文档 = 一个容器 = 一个加法地址, 程序按地址直接索引");
        return;
    }
    let mut input: Option<String> = None;
    let mut out_dir: Option<String> = None;
    let mut want_html = false;
    for a in &args {
        if a == "--html" {
            want_html = true;
        } else if input.is_none() {
            input = Some(a.clone());
        } else if out_dir.is_none() {
            out_dir = Some(a.clone());
        }
    }
    let input = input.unwrap();
    let content = match fs::read_to_string(&input) {
        Ok(c) => c,
        Err(e) => {
            println!("读取失败: {} ({})", input, e);
            return;
        }
    };
    let out_dir = out_dir.unwrap_or_else(|| {
        let p = Path::new(&input).parent().unwrap_or(Path::new("."));
        p.join("data").to_string_lossy().to_string()
    });
    let _ = fs::create_dir_all(&out_dir);

    let msgs = parse_messages(&content);
    let mut units = build_units(&msgs);
    build_related(&mut units);
    let total = units.len();

    let _ = fs::write(Path::new(&out_dir).join("工作空间.chain"), out_workspace_chain(&units));
    let _ = fs::write(Path::new(&out_dir).join("符号索引.chain"), out_symbol_index(&units));
    let _ = fs::write(Path::new(&out_dir).join("解析树.chain"), out_parse_chain(&units));
    let _ = fs::write(Path::new(&out_dir).join("验证树.chain"), out_verify_chain(&units));
    let _ = fs::write(Path::new(&out_dir).join("提示词.chain"), out_prompt_chain(&units));
    let _ = fs::write(Path::new(&out_dir).join("容器全文.chain"), out_container_chain(&units));

    if want_html {
        let _ = fs::create_dir_all(Path::new(&out_dir).join("pages"));
        let _ = fs::write(Path::new(&out_dir).join("工作空间.html"), render_dir(&units, &out_dir));
        let _ = fs::write(Path::new(&out_dir).join("index.html"), render_dir(&units, &out_dir));
        let _ = fs::write(Path::new(&out_dir).join("解析树.html"), render_parse_tree(&units));
        let _ = fs::write(Path::new(&out_dir).join("验证树.html"), render_verify_tree(&units));
        let _ = fs::write(Path::new(&out_dir).join("介绍向导.html"), render_intro());
        let _ = fs::write(Path::new(&out_dir).join("注册向导.html"), render_signup());
        let _ = fs::write(Path::new(&out_dir).join("注册清单.md"), render_registry(&units));
        let _ = fs::write(Path::new(&out_dir).join("清单.md"), render_list(&units));
        for u in &units {
            let _ = fs::write(
                Path::new(&out_dir).join("pages").join(format!("{}.html", page(u.addr, total))),
                render_page(u, total),
            );
        }
    }

    println!("工作空间(数据层): {}", out_dir);
    println!("  容器 = {} (加法地址)", total);
    println!("  工作空间.chain  = 地址|时间版本|工作版本|符号数|指向|标题");
    println!("  符号索引.chain  = 符号|出现地址  (反向索引, 热符号可直接寻址)");
    println!("  解析树.chain    = 父地址|子地址 (根=0)");
    println!("  验证树.chain    = 地址A|地址B|共享权重");
    println!("  提示词.chain    = 地址|时间版本|目标|指向|关键词  (喂给其它 AI 的参数)");
    println!("  容器全文.chain  = 地址|时间版本|工作版本|标题|符号|关联|问|答");
    if want_html {
        println!("  人类页面: {}/工作空间.html (+pages/)", out_dir);
    }
    let linked = units.iter().filter(|u| !u.related.is_empty()).count();
    println!("  交叉验证容器 = {} / {}", linked, total);
}
