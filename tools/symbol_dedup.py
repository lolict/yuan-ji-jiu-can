#!/usr/bin/env python3
"""
零维码符号去重脚本
输入：docs/SYMBOL_MASTER_TABLE.md 中的所有符号
输出：去重后的可用符号池（单码点预组合），带 Unicode + 主人 + 相位
"""

import re
import unicodedata

def extract_symbols_from_file(filepath):
    """从 Markdown 表格中提取所有符号"""
    symbols = []
    with open(filepath, 'r', encoding='utf-8') as f:
        for line in f:
            # 匹配表格行 | 符号 | 码点 | ...
            match = re.match(r'\|\s*([^\s|]+)\s*\|', line)
            if match:
                sym = match.group(1).strip()
                if sym and sym != '符号' and sym != '---':
                    symbols.append(sym)
    return symbols

def get_codepoints(char):
    """获取字符的 Unicode 码点序列"""
    return [f"U+{ord(c):04X}" for c in char]

def is_precomposed(char):
    """判断是否为预组合单码点（真正的0维猴子本体）"""
    # 单码点 = 预组合
    if len(char) == 1:
        return True
    # 多码点 = 组合（猴子+帽子）
    return False

def get_unicode_name(char):
    """获取 Unicode 名称"""
    try:
        return unicodedata.name(char, "UNKNOWN")
    except:
        return "UNKNOWN"

def get_base_family(char):
    """通过 Unicode 名称推断主人家族"""
    name = get_unicode_name(char)
    if name == "UNKNOWN":
        return "未知"
    
    # 提取名称中的基础字母
    # 例如 "LATIN SMALL LETTER A WITH ACUTE" → A家族
    match = re.search(r'LATIN (?:SMALL |CAPITAL )?LETTER ([A-Z])', name)
    if match:
        return match.group(1)
    
    # 连体字
    if 'LIGATURE' in name:
        match = re.search(r'LIGATURE ([A-Z]+)', name)
        if match:
            return f"连体_{match.group(1)}"
    
    return "其他"

def classify_phase(char):
    """通过 Unicode 名称推断相位"""
    name = get_unicode_name(char)
    
    # 基础本体（无装饰）
    # 简单判断：名称中只有 "LATIN LETTER X" 没有附加描述
    if re.match(r'^LATIN (SMALL |CAPITAL )?LETTER [A-Z]$', name):
        return "P0_原态"
    
    # 镜像/颠倒/旋转通过名称关键词
    if 'REVERSED' in name or 'TURNED' in name or 'INVERTED' in name:
        return "P2_上下颠倒" if 'TURNED' in name else "P1_左右镜像" if 'REVERSED' in name else "P3_前后翻转"
    if 'ROTATED' in name:
        return "P6_顺时针90" if '90' in name else "P4_顺时针45"
    if 'SIDEWAYS' in name:
        return "P7_逆时针90"
    
    # 有装饰（附加符号）= P0 + 装饰
    return "P0_带装饰"

def main():
    filepath = '/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/docs/SYMBOL_MASTER_TABLE.md'
    
    # 提取所有符号
    symbols = extract_symbols_from_file(filepath)
    print(f"从文件中提取到 {len(symbols)} 个符号")
    
    # 去重（基于码点）
    seen_codepoints = set()
    unique_symbols = []
    duplicates = []
    
    for sym in symbols:
        cps = tuple(get_codepoints(sym[0]) if len(sym) == 1 else get_codepoints(sym))
        if cps not in seen_codepoints:
            seen_codepoints.add(cps)
            unique_symbols.append(sym)
        else:
            duplicates.append(sym)
    
    print(f"去重后：{len(unique_symbols)} 个唯一符号")
    if duplicates:
        print(f"发现重复：{len(duplicates)} 个")
    
    # 分类：预组合 vs 组合
    precomposed = []  # 单码点，可用
    combined = []     # 多码点，不可用
    
    for sym in unique_symbols:
        first_char = sym[0]
        if is_precomposed(first_char):
            precomposed.append({
                'symbol': first_char,
                'codepoint': get_codepoints(first_char)[0],
                'name': get_unicode_name(first_char),
                'family': get_base_family(first_char),
                'phase': classify_phase(first_char),
                'usable': True
            })
        else:
            combined.append({
                'symbol': sym,
                'codepoints': ' '.join(get_codepoints(sym)),
                'usable': False
            })
    
    print(f"预组合（可用）：{len(precomposed)} 个")
    print(f"组合（不可用）：{len(combined)} 个")
    
    # 按家族聚类
    families = {}
    for item in precomposed:
        fam = item['family']
        if fam not in families:
            families[fam] = []
        families[fam].append(item)
    
    # 输出结果文件
    output_path = '/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/docs/DEDUP_SYMBOL_POOL.md'
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("# 零维码 · 去重符号池（P0-1 产出）\n\n")
        f.write(f"- 提取总数：{len(symbols)}\n")
        f.write(f"去重后：{len(unique_symbols)}\n")
        f.write(f"预组合可用：{len(precomposed)}\n")
        f.write(f"组合不可用：{len(combined)}\n")
        f.write(f"家族数量：{len(families)}\n\n")
        
        f.write("---\n\n")
        f.write("## 可用符号池（按家族聚类）\n\n")
        
        for fam in sorted(families.keys()):
            members = families[fam]
            f.write(f"### 家族 {fam}（{len(members)} 个）\n\n")
            f.write("| 编码序号 | 符号 | 码点 | Unicode名称 | 相位 |\n")
            f.write("|---------|------|------|-------------|------|\n")
            for i, m in enumerate(members, 1):
                f.write(f"| {i} | {m['symbol']} | {m['codepoint']} | {m['name']} | {m['phase']} |\n")
            f.write("\n")
        
        if combined:
            f.write("---\n\n")
            f.write("## 不可用符号（组合/多码点）\n\n")
            f.write("| 符号 | 码点序列 | 原因 |\n")
            f.write("|------|---------|------|\n")
            for c in combined:
                f.write(f"| {c['symbol']} | {c['codepoints']} | 多码点组合 |\n")
            f.write("\n")
    
    print(f"\n结果已写入：{output_path}")
    
    # 输出统计
    print("\n=== 家族统计 ===")
    for fam in sorted(families.keys()):
        print(f"  {fam}: {len(families[fam])} 个")

if __name__ == '__main__':
    main()
