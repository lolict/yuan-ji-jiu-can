"""
相位进制 CLI

命令行入口，支持：
  - 创建画布
  - 安装部件（加法）
  - 拆解部件（减法）
  - 编译输出（RGB/ASCII/JSON/WAV）
  - 治理校验
  - 网络构建与遍历
"""

import argparse
import json
import os
import sys
from typing import Dict, List, Optional

from .canvas import PhaseCanvas, PhaseCell, PhaseRadix, hsv_to_rgb
from .arithmetic import PhaseArithmetic, AddOperation, SubtractOperation
from .blockchain import BlockchainIndex, SignalNode
from .compiler import PhaseCompiler, BatchCompiler
from .governance import GovernancePipeline


def main():
    parser = argparse.ArgumentParser(
        prog="phase-radix",
        description="满全法 · 相位进制画布 — 离散相位单元 + 加减运算",
    )
    
    subparsers = parser.add_subparsers(dest="command", help="子命令")
    
    # ---- canvas: 创建画布 ----
    canvas_parser = subparsers.add_parser("canvas", help="创建相位画布")
    canvas_parser.add_argument("name", help="画布名称")
    canvas_parser.add_argument("--width", "-W", type=int, default=16)
    canvas_parser.add_argument("--height", "-H", type=int, default=16)
    canvas_parser.add_argument("--fill-phase", "-p", type=float, default=0.0)
    canvas_parser.add_argument("--fill-weight", "-w", type=float, default=3.0)
    
    # ---- install: 加法 ----
    install_parser = subparsers.add_parser("install", help="加法安装部件")
    install_parser.add_argument("target", help="目标画布名")
    install_parser.add_argument("source", help="源部件名")
    install_parser.add_argument("--offset-x", "-x", type=int, default=0)
    install_parser.add_argument("--offset-y", "-y", type=int, default=0)
    install_parser.add_argument("--blend", "-b", default="average",
                                choices=["average", "max", "weighted"])
    
    # ---- subtract: 减法 ----
    sub_parser = subparsers.add_parser("subtract", help="减法拆解")
    sub_parser.add_argument("target", help="目标画布名")
    sub_parser.add_argument("x", type=int)
    sub_parser.add_argument("y", type=int)
    sub_parser.add_argument("w", type=int)
    sub_parser.add_argument("h", type=int)
    sub_parser.add_argument("--mode", "-m", default="deactivate",
                            choices=["deactivate", "erase", "extract"])
    
    # ---- compile: 编译输出 ----
    compile_parser = subparsers.add_parser("compile", help="编译输出")
    compile_parser.add_argument("name", help="画布名")
    compile_parser.add_argument("--format", "-f", default="ascii",
                                choices=["ascii", "png", "json", "wave"])
    compile_parser.add_argument("--output", "-o", help="输出文件路径")
    compile_parser.add_argument("--scale", "-s", type=int, default=10)
    
    # ---- show: 显示画布 ----
    show_parser = subparsers.add_parser("show", help="显示画布")
    show_parser.add_argument("name", help="画布名")
    show_parser.add_argument("--format", "-f", default="ascii",
                             choices=["ascii", "stats", "rgb"])
    
    # ---- demo: 运行演示 ----
    demo_parser = subparsers.add_parser("demo", help="运行完整演示")
    
    # ---- governance: 治理校验 ----
    gov_parser = subparsers.add_parser("governance", help="治理校验")
    gov_parser.add_argument("--node-a", help="节点A的JSON描述")
    gov_parser.add_argument("--node-b", help="节点B的JSON描述")
    gov_parser.add_argument("--operation", default="pairing",
                            choices=["pairing", "diffraction"])
    
    # ---- blockchain: 区块链 ----
    chain_parser = subparsers.add_parser("chain", help="区块链操作")
    chain_parser.add_argument("--build-demo", action="store_true",
                              help="构建演示网络")
    chain_parser.add_argument("--traverse-from", help="从哪个节点遍历")
    chain_parser.add_argument("--depth", type=int, default=3)
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    # 简单的内存存储（实际使用可用持久化）
    radix = PhaseRadix()
    compiler = PhaseCompiler()
    pipeline = GovernancePipeline()
    
    if args.command == "canvas":
        canvas = radix.canvas(args.name, args.width, args.height)
        # 填充初始值
        for row in canvas.cells:
            for cell in row:
                cell.phase_value = args.fill_phase
                cell.weight = args.fill_weight
        print(f"创建画布: {args.name} ({args.width}x{args.height})")
        print(compiler.to_ascii(canvas))
    
    elif args.command == "install":
        target = radix.get_component(args.target) or _load_canvas(args.target)
        source = radix.get_component(args.source) or _load_canvas(args.source)
        if target and source:
            AddOperation.execute(target, source, args.offset_x, args.offset_y, args.blend)
            print(f"已安装 {args.source} → {args.target}")
        else:
            print("画布未找到")
    
    elif args.command == "subtract":
        target = _load_canvas(args.target)
        if target:
            SubtractOperation.execute(target, args.x, args.y, args.w, args.h, args.mode)
            print(f"已在 {args.target} 上拆解区域 ({args.x},{args.y},{args.w},{args.h})")
            print(compiler.to_ascii(target))
    
    elif args.command == "compile":
        canvas = _load_canvas(args.name)
        if not canvas:
            print(f"画布 {args.name} 未找到")
            return
        
        output_path = args.output
        if args.format == "ascii":
            output = compiler.to_ascii(canvas)
            print(output)
            if output_path:
                with open(output_path, "w") as f:
                    f.write(output)
        elif args.format == "png":
            if not output_path:
                output_path = f"{args.name}.png"
            compiler.save_image(canvas, output_path, args.scale)
            print(f"已保存图像: {output_path}")
        elif args.format == "json":
            output = compiler.to_json(canvas)
            print(output)
            if output_path:
                with open(output_path, "w") as f:
                    f.write(output)
        elif args.format == "wave":
            if not output_path:
                output_path = f"{args.name}.wav"
            compiler.save_wave(canvas, output_path)
            print(f"已保存音频: {output_path}")
    
    elif args.command == "show":
        canvas = _load_canvas(args.name)
        if not canvas:
            print(f"画布 {args.name} 未找到")
            return
        
        if args.format == "ascii":
            print(compiler.to_ascii(canvas))
        elif args.format == "stats":
            stats = canvas.statistics()
            print(json.dumps(stats, ensure_ascii=False, indent=2))
        elif args.format == "rgb":
            rgb = compiler.to_rgb_matrix(canvas)
            print(f"RGB 矩阵: {len(rgb[0])}x{len(rgb)}")
    
    elif args.command == "demo":
        _run_demo()
    
    elif args.command == "governance":
        if args.node_a and args.node_b:
            node_a = json.loads(args.node_a)
            node_b = json.loads(args.node_b)
        else:
            # 演示数据
            node_a = {
                "name": "a", "level": "L1", "type": "韵母",
                "owner": "SELF", "weight": 2.0, "family": "latin", "roundness": 0.7,
            }
            node_b = {
                "name": "á", "level": "L3", "type": "声调",
                "owner": "SELF", "weight": 3.0, "family": "latin", "roundness": 0.6,
            }
        
        report = pipeline.validate_pair(node_a, node_b, args.operation)
        print(report.summary())
    
    elif args.command == "chain":
        if args.build_demo:
            index = _build_demo_chain()
            print("区块链演示网络已构建")
            print(f"节点数: {len(index.nodes)}")
            print(json.dumps(index.statistics(), ensure_ascii=False, indent=2))
            
            if args.traverse_from:
                paths = index.traverse_from(args.traverse_from, args.depth)
                print(f"\n从 {args.traverse_from} 遍历:")
                for path in paths:
                    print(f"  {' → '.join(path)}")


def _load_canvas(name: str) -> Optional[PhaseCanvas]:
    """从 JSON 文件加载画布"""
    filepath = f"{name}.json"
    if not os.path.exists(filepath):
        return None
    
    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)
    
    canvas = PhaseCanvas(name=data["name"], width=data["width"], height=data["height"])
    for cell_data in data.get("cells", []):
        x, y = cell_data["x"], cell_data["y"]
        cell = canvas.get(x, y)
        if cell:
            cell.phase_value = cell_data.get("phase_value", 0)
            cell.weight = cell_data.get("weight", 1)
            cell.density = cell_data.get("density", 1)
            cell.level = cell_data.get("level", "L1")
            cell.owner = cell_data.get("owner", "SELF")
            cell.family = cell_data.get("family", "latin")
            cell.active = cell_data.get("active", True)
    
    return canvas


def _build_demo_chain() -> BlockchainIndex:
    """构建演示用的区块链网络"""
    index = BlockchainIndex()
    
    # 声母节点
    for name in ["b", "p", "m", "f"]:
        node = SignalNode(
            signal_id=name,
            signal_type="audio",
            annotate={
                "level": "L1", "type": "声母", "owner": "SELF",
                "visual_weight": 3.0, "family": "latin",
                "glyph_label": name, "unicode_char": name,
            }
        )
        index.add_node(node)
    
    # 韵母节点
    for name in ["a", "o", "e", "ai", "ao"]:
        node = SignalNode(
            signal_id=name,
            signal_type="audio",
            annotate={
                "level": "L1", "type": "韵母", "owner": "SELF",
                "visual_weight": 2.0, "family": "latin",
                "glyph_label": name, "unicode_char": name,
            }
        )
        index.add_node(node)
    
    # 声调节点（L3）
    for name in ["ā", "á", "ǎ", "à"]:
        node = SignalNode(
            signal_id=name,
            signal_type="audio",
            annotate={
                "level": "L3", "type": "声调", "owner": "SELF",
                "visual_weight": 2.5, "family": "latin",
                "glyph_label": name, "unicode_char": name,
            }
        )
        index.add_node(node)
    
    # 建立连接（声母 → 韵母 是合法路径）
    for consonant in ["b", "p", "m", "f"]:
        for vowel in ["a", "o", "e", "ai", "ao"]:
            index.link(consonant, vowel)
    
    # 韵母 → 声调（韵母 + 声调 = 带调音节）
    for vowel in ["a", "o", "e"]:
        for tone in ["ā", "á", "ǎ", "à"]:
            # 只有对应的才连（a→ā, a→á, a→ǎ, a→à）
            if vowel in tone or tone.startswith(vowel[0]):
                index.link(vowel, tone)
    
    return index


def _run_demo():
    """运行完整演示"""
    print("\n" + "█" * 60)
    print("  满全法 · 相位进制画布 — 演示")
    print("  离散相位单元 + 加减运算")
    print("█" * 60)
    
    # 1. 创建画布
    radix = PhaseRadix()
    canvas_a = radix.canvas("a_glyph", 16, 16)
    
    # 填充一个"韵母a"的特征：中等字重、圆润
    for row in canvas_a.cells:
        for cell in row:
            cell.phase_value = 120.0  # 绿色相位
            cell.weight = 2.0
            cell.density = 1.5
            cell.roundness = 0.8
            cell.family = "latin"
            cell.level = "L1"
    
    compiler = PhaseCompiler()
    print("\n【步骤1】创建韵母 a 的相位画布:")
    print(compiler.to_ascii(canvas_a))
    
    # 2. 创建声调部件
    tone_mark = radix.canvas("tone_acute", 4, 4)
    for row in tone_mark.cells:
        for cell in row:
            cell.phase_value = 60.0
            cell.weight = 1.0
            cell.density = 0.5
            cell.level = "L2"
    
    radix.register_component("acute_mark", tone_mark)
    
    # 3. 加法装配
    print("\n【步骤2】加法装配：把声调部件安装到韵母上")
    AddOperation.execute(canvas_a, tone_mark, 6, 2, "average")
    print(compiler.to_ascii(canvas_a))
    
    # 4. 溯源
    arith = PhaseArithmetic()
    reverse = arith.reverse_assembly(canvas_a)
    print(f"\n【步骤3】装配溯源: {reverse['inference']}")
    print(f"  源部件: {reverse['source_components']}")
    
    # 5. 治理校验
    print("\n【步骤4】治理校验")
    pipeline = GovernancePipeline()
    
    node_a = {
        "name": "a", "level": "L1", "type": "韵母",
        "owner": "SELF", "weight": 2.0, "family": "latin", "roundness": 0.7,
    }
    node_b_wrong = {
        "name": "á", "level": "L3", "type": "声调",
        "owner": "SELF", "weight": 3.0, "family": "latin", "roundness": 0.6,
    }
    node_b_ok = {
        "name": "o", "level": "L1", "type": "韵母",
        "owner": "SELF", "weight": 2.5, "family": "latin", "roundness": 0.9,
    }
    
    print("\n  校验 a(L1) ↔ á(L3) [错误配对]:")
    report1 = pipeline.validate_pair(node_a, node_b_wrong, "diffraction")
    print(report1.summary())
    
    print("\n  校验 a(L1) ↔ o(L1) [正确配对]:")
    report2 = pipeline.validate_pair(node_a, node_b_ok, "diffraction")
    print(report2.summary())
    
    # 6. 区块链网络
    print("\n【步骤5】区块链式纠缠网络")
    index = _build_demo_chain()
    print(f"  网络统计: {json.dumps(index.statistics(), ensure_ascii=False)}")
    
    print("\n  从 'b' 出发的链式传导:")
    paths = index.traverse_from("b", max_depth=3)
    for path in paths[:5]:
        print(f"    {' → '.join(path)}")
    
    presence = index.global_presence("b")
    print(f"\n  全局在场（从 b 辐射）:")
    for nid, intensity in sorted(presence.items(), key=lambda x: -x[1])[:6]:
        bar = "█" * int(intensity * 20)
        print(f"    {nid:<8} {intensity:.3f} {bar}")
    
    print("\n█" * 60)
    print("  演示完成")


if __name__ == "__main__":
    main()
