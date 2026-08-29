#!/usr/bin/env python3
"""
FQM 统一解释器 · 元输入法系统

整合所有解释器模块，提供统一入口：
- 袋.声母.装入(b)     → 容器操作
- 三维.坐标(b, a, 1)   → 三维空间
- 降维(b, a, 1, 声调)  → 维度升降
- 公约(声母, 韵母)     → 关系运算
- 一元(目标)           → 元层级
- 翻译(class)          → 外部符号翻译

同时加载 symbol_registry.json 作为运行时符号源。
"""

import os
import sys
import json
import re

# 确保能导入同目录模块
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))


class FQMUnifiedSystem:
    """FQM 统一系统"""
    
    def __init__(self, fqm_file="containers.fqm", registry_file="symbol_registry.json",
                 driver_manifest="EXTERNAL_DRIVER_MANIFEST.json"):
        self.fqm_file = fqm_file
        self.registry_file = registry_file
        self.driver_manifest_file = driver_manifest
        self.loader = None
        self.linker = None
        self.interpreter = None
        self.radix3d = None
        self.dimension = None
        self.divisor = None
        self.yuan = None
        self.translator = None
        self.registry = None
        self.bootstrap_result = None
        self.driver_manifest = None
        self._load_registry()
        self._load_driver_manifest()
    
    def _load_registry(self):
        """加载符号注册表"""
        reg_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), self.registry_file)
        if os.path.exists(reg_path):
            with open(reg_path, 'r', encoding='utf-8') as f:
                self.registry = json.load(f)
        else:
            self.registry = None
    
    def _load_driver_manifest(self):
        """加载外部因果驱动索引"""
        manifest_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), self.driver_manifest_file)
        if os.path.exists(manifest_path):
            with open(manifest_path, 'r', encoding='utf-8') as f:
                self.driver_manifest = json.load(f)
        else:
            self.driver_manifest = None
    
    def start(self):
        """启动完整系统 — 先经过鼓膜自举验证"""
        # ========== 第一步：鼓膜自举 ==========
        import bootstrap as drum_bootstrap
        self.bootstrap_result = drum_bootstrap.bootstrap()
        
        if not self.bootstrap_result["ok"]:
            print("❌ 鼓膜自举失败，系统拒绝启动")
            print(f"   原因: {self.bootstrap_result.get('reason', 'unknown')}")
            return False
        
        # ========== 第二步：加载外部代码 ==========
        from loader import FQMLoader
        from linker import FQMLinker
        from interpreter import FQMInterpreter
        from radix3d import Radix3DInterpreter
        from dimension import DimensionInterpreter
        from divisor import DivisorInterpreter
        from yuan import YuanInterpreter
        from translator import FQMTranslator
        
        self.loader = FQMLoader()
        fqm_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), self.fqm_file)
        if not self.loader.load_file(fqm_path):
            return False
        
        self.linker = FQMLinker(self.loader)
        self.linker.build_ontology_index()
        self.linker.resolve_links()
        
        self.interpreter = FQMInterpreter(self.linker)
        self.radix3d = Radix3DInterpreter(self.linker)
        self.dimension = DimensionInterpreter(self.linker)
        self.divisor = DivisorInterpreter(self.linker)
        self.yuan = YuanInterpreter(self.linker)
        self.translator = FQMTranslator(self.linker)
        
        return True
    
    def execute(self, command):
        """统一执行入口"""
        s = command.strip()
        if not s or s.startswith('#'):
            return None
        
        # 翻译命令: 翻译(class) 或 >class
        if s.startswith('翻译(') or s.startswith('>'):
            return self._cmd_translate(s)
        
        # 错别字纠正
        if s.startswith('纠正(') or s.startswith('纠正 '):
            return self._cmd_typo_correct(s)
        
        # 注册表查询
        if s.startswith('注册.') or s.startswith('符号.'):
            return self._cmd_registry_query(s)
        
        # 外部因果驱动索引查询
        if s.startswith('驱动') or s.startswith('因果'):
            return self._cmd_driver_query(s)
        
        # 骨膜规则查询
        if s == '骨膜' or s == '骨架' or s == '骨魔':
            return self._cmd_bone_rules()
        
        # 自举状态
        if s == '自举' or s == '鼓膜':
            return self._cmd_bootstrap_status()
        
        # 状态总览
        if s == '/status' or s == '状态':
            return self._cmd_status()
        
        # 训练: 声母遍历韵母
        if s == '/train_all' or s == '训练全部':
            return self._cmd_train_all()
        
        # 元层级命令
        if s.startswith('一元(') or s.startswith('二元(') or \
           s.startswith('三元(') or s.startswith('四元(') or \
           s == '层级' or s.startswith('裁判(') or \
           s.startswith('升元(') or s.startswith('降元('):
            return self.yuan.execute(command)
        
        # 公约数命令
        if s.startswith('公约(') or s.startswith('公倍(') or \
           s.startswith('同公(') or s.startswith('齐公(') or \
           s.startswith('关系(') or s == '公约数':
            return self.divisor.execute(command)
        
        # 维度命令
        if s.startswith('降维(') or s.startswith('升维(') or \
           s.startswith('投影(') or s.startswith('展开(') or \
           s == '维度':
            return self.dimension.execute(command)
        
        # 三维命令
        if s.startswith('三维.'):
            return self.radix3d.execute(command)
        
        # 容器操作（默认）
        return self.interpreter.execute(command)
    
    def _cmd_translate(self, command):
        """翻译外部符号"""
        if command.startswith('翻译('):
            match = re.match(r'翻译\((.+)\)', command)
            if match:
                symbol = match.group(1).strip()
                result = self.translator.translate_symbol(symbol)
                if result:
                    return f"翻译: {symbol} → {result}"
                else:
                    return f"翻译: {symbol} → [拒绝]（不在映射表中）"
        elif command.startswith('>'):
            symbol = command[1:].strip()
            result = self.translator.translate_symbol(symbol)
            if result:
                return f"{symbol} → {result}"
            else:
                return f"{symbol} → [拒绝]"
        return "错误: 翻译格式"
    
    def _cmd_registry_query(self, command):
        """查询注册表"""
        if not self.registry:
            return "注册表未加载"
        
        # 注册.声母 — 列出声母所有符号
        match = re.match(r'(?:注册|符号)\.声母$', command)
        if match:
            containers = self.registry.get("containers", {})
            shengmu = containers.get("shengmu", {})
            symbols = shengmu.get("symbols", [])
            result = f"注册表·声母 ({len(symbols)} 个):\n"
            for entry in symbols:
                result += f"  {entry['symbol']} (位置{entry['position']}) 近义词:{entry.get('synonyms',[])}\n"
            return result.strip()
        
        # 注册.韵母 — 列出韵母所有符号
        match = re.match(r'(?:注册|符号)\.韵母$', command)
        if match:
            containers = self.registry.get("containers", {})
            yunmu = containers.get("yunmu", {})
            symbols = yunmu.get("symbols", [])
            result = f"注册表·韵母 ({len(symbols)} 个):\n"
            for entry in symbols:
                result += f"  {entry['symbol']} (位置{entry['position']}) 近义词:{entry.get('synonyms',[])}\n"
            return result.strip()
        
        # 注册.汉字 — 列出汉字符号
        match = re.match(r'(?:注册|符号)\.汉字$', command)
        if match:
            # 汉字符号在 hanzi_symbols 键下
            hanzi = self.registry.get("hanzi_symbols", {})
            symbols = hanzi.get("symbols", [])
            result = f"注册表·汉字 ({len(symbols)} 个):\n"
            for entry in symbols[:20]:
                result += f"  {entry['symbol']} (位置{entry['position']}) {entry.get('container','')}\n"
            if len(symbols) > 20:
                result += f"  ... (共{len(symbols)}个)"
            return result.strip()
        
        # 注册.偏移 — 查询全局偏移
        if command in ('注册.偏移', '符号.偏移'):
            offset = self.registry.get("global_offset", 0)
            return f"全局偏移量: {offset} (0x{offset:02X})"
        
        # 注册.错别字 — 查询错别字纠正规则
        if command in ('注册.错别字', '符号.错别字'):
            corrections = self.registry.get("typo_corrections", {})
            result = f"错别字纠正规则 ({len(corrections)} 条):\n"
            for wrong, correct in list(corrections.items())[:15]:
                result += f"  {wrong} → {correct}\n"
            if len(corrections) > 15:
                result += f"  ... (共{len(corrections)}条)"
            return result.strip()
        
        return "错误: 注册查询格式（可用: 注册.声母 / 注册.韵母 / 注册.汉字 / 注册.偏移 / 注册.错别字）"
    
    def _cmd_typo_correct(self, command):
        """错别字纠正 — 支持单字和整段文字"""
        if not self.registry:
            return "注册表未加载"
        
        corrections = self.registry.get("typo_corrections", {})
        
        # 提取括号内内容
        match = re.match(r'纠正\s*\((.+)\)', command)
        if match:
            text = match.group(1).strip()
        else:
            # 无括号格式: 纠正 带
            parts = command.split(None, 1)
            if len(parts) == 2 and parts[0] == '纠正':
                text = parts[1].strip()
            else:
                return "错误: 纠正格式（可用: 纠正(错字) 或 纠正(句子)）"
        
        # 单字直接查表
        if len(text) == 1:
            correct = corrections.get(text)
            if correct:
                return f"纠正: {text} → {correct}"
            else:
                return f"纠正: {text} → [无纠正规则]"
        
        # 多字/句子：逐字扫描替换
        corrected = text
        found = []
        for wrong, correct in corrections.items():
            if wrong in corrected:
                corrected = corrected.replace(wrong, correct)
                found.append(f"{wrong}→{correct}")
        
        if found:
            return f"全文纠正:\n  原文: {text}\n  纠后: {corrected}\n  替换: {', '.join(found)}"
        return f"全文纠正: {text}（无需纠正）"
    
    def _cmd_driver_query(self, command):
        """查询外部因果驱动索引"""
        if not self.driver_manifest:
            return "外部驱动清单未加载"
        
        # 驱动.L1 — 列出所有外部驱动
        if command in ('驱动.L1', '因果.L1'):
            drivers = self.driver_manifest.get("L1_外部因果驱动索引", {})
            result = "L1 外部因果驱动索引:\n"
            for key, val in drivers.items():
                if key.startswith("_"):
                    continue
                if isinstance(val, dict) and "名称" in val:
                    result += f"  [{key}] {val['名称']}\n"
                    result += f"    触发: {val.get('触发条件', '')}\n"
                    result += f"    → 钩子: {val.get('映射钩子', '')}\n"
            return result.strip()
        
        # 驱动.L2 — 列出所有钩子
        if command in ('驱动.L2', '因果.L2'):
            hooks = self.driver_manifest.get("L2_情节钩子索引", {})
            result = "L2 情节钩子索引:\n"
            for key, val in hooks.items():
                if key.startswith("_"):
                    continue
                if isinstance(val, dict) and "行为过程" in val:
                    result += f"  [{key}] 过程: {val['行为过程']}\n"
                    result += f"    版本: {val.get('依赖版本', '')}  允许运算: {val.get('允许运算', [])}\n"
            return result.strip()
        
        # 驱动.L3 — 当前行为快照
        if command in ('驱动.L3', '因果.L3'):
            snapshot = self.driver_manifest.get("L3_行为过程快照", {})
            result = "L3 行为过程快照:\n"
            result += f"  激活钩子: {snapshot.get('当前激活的钩子', '')}\n"
            result += f"  自检状态: {snapshot.get('自检状态', '')}\n"
            result += f"  最后错误: {snapshot.get('最后错误记录', '')}\n"
            result += f"  容器状态:\n"
            for cname, cinfo in snapshot.get("当前容器状态", {}).items():
                result += f"    {cname}: {cinfo.get('符号数', 0)}/{cinfo.get('容量', 0)} (空{cinfo.get('空槽', 0)})\n"
            return result.strip()
        
        # 驱动.L4 — 版本历史
        if command in ('驱动.L4', '因果.L4'):
            history = self.driver_manifest.get("L4_版本_日期锚定历史", {})
            result = "L4 版本-日期锚定:\n"
            for date, desc in history.items():
                result += f"  {date}: {desc}\n"
            return result.strip()
        
        # 驱动.禁止 — 列出禁止链
        if command in ('驱动.禁止', '因果.禁止'):
            forbidden = self.driver_manifest.get("禁止链", {})
            result = "禁止链（子代理绝对不可用）:\n"
            result += f"  禁用运算: {forbidden.get('禁用运算', [])}\n"
            result += f"  禁用单位: {forbidden.get('禁用单位', '')}\n"
            result += f"  禁用容器: {forbidden.get('禁用容器', [])}\n"
            result += f"  禁用符号: {forbidden.get('禁用符号', [])}\n"
            result += f"  禁用模式: {forbidden.get('禁用模式', [])}\n"
            result += f"  禁用工具: {forbidden.get('禁用工具', [])}\n"
            return result.strip()
        
        # 默认显示所有四层概览
        result = "外部因果驱动索引概览:\n"
        for layer in ["L1_外部因果驱动索引", "L2_情节钩子索引", "L3_行为过程快照", "L4_版本_日期锚定历史"]:
            data = self.driver_manifest.get(layer, {})
            count = len([k for k in data.keys() if not k.startswith("_")])
            result += f"  {layer}: {count} 条目\n"
        result += "\n查询: 驱动.L1 / 驱动.L2 / 驱动.L3 / 驱动.L4 / 驱动.禁止"
        return result.strip()
    
    def _cmd_bone_rules(self):
        """查询骨膜骨架的不可变规则"""
        import bootstrap as drum
        rules = drum.get_skeleton_rules()
        result = "骨膜骨架 · 不可变元规则:\n"
        result += f"{'─' * 40}\n"
        result += f"  允许运算: {rules['allowed_ops']}\n"
        result += f"  禁用运算: {rules['forbidden_ops']}\n"
        result += f"  操作单位: {rules['operation_unit']}级（禁用{rules['forbidden_unit']}级）\n"
        result += f"  见证要件: {rules['witness_requirements']}\n"
        result += f"  容器: {rules['containers']}\n"
        result += f"  最大符号: {rules['max_symbols']}/容器\n"
        result += f"{'─' * 40}\n"
        result += "  这些规则硬编码在bootstrap.py中，不可被外部文件覆盖"
        return result
    
    def _cmd_bootstrap_status(self):
        """查询鼓膜自举状态"""
        if not self.bootstrap_result:
            return "尚未执行自举"
        
        result = "鼓膜自举状态:\n"
        result += f"{'─' * 40}\n"
        result += f"  状态: {'✅ 通过' if self.bootstrap_result['ok'] else '❌ 失败'}\n"
        result += f"  共振签名: {self.bootstrap_result.get('signature', 'N/A')}\n"
        result += f"  振荡器: {self.bootstrap_result.get('oscillator', {}).get('signature', 'N/A')}\n"
        result += f"  启动时间: {self.bootstrap_result.get('oscillator', {}).get('boot_time', 'N/A')}\n"
        result += f"  回响结果: {self.bootstrap_result.get('resonance', 'N/A')}\n"
        return result
    
    def _cmd_status(self):
        """系统状态总览"""
        result = f"{'='*60}\n"
        result += "FQM 统一系统状态\n"
        result += f"{'─'*60}\n"
        
        # 鼓膜自举状态
        if self.bootstrap_result:
            boot_ok = self.bootstrap_result.get("ok", False)
            result += f"鼓膜自举: {'✅ 通过' if boot_ok else '❌ 失败'}\n"
            result += f"  共振签名: {self.bootstrap_result.get('signature', 'N/A')}\n"
        else:
            result += "鼓膜自举: ⏳ 未执行\n"
        
        # 容器状态
        if self.loader:
            containers = self.loader.get_all_containers()
            result += f"容器: {len(containers)} 个\n"
            for name, container in containers.items():
                result += f"  {name}: {len(container.symbols)} 符号\n"
        
        # 链接状态
        if self.linker:
            result += f"链接: {len(self.linker.resolved)} 已解析, {len(self.linker.unresolved)} 未解析\n"
        
        # 注册表
        if self.registry:
            result += f"注册表: 已加载 (版本 {self.registry.get('metadata',{}).get('version','?')})\n"
            result += f"  全局偏移: {self.registry.get('global_offset', 0)}\n"
        
        # 外部驱动清单
        if self.driver_manifest:
            ver = self.driver_manifest.get("系统锚定", {}).get("主版本号", "?")
            result += f"驱动清单: 已加载 ({ver})\n"
        
        # 翻译器
        if self.translator:
            result += f"翻译器: {len(self.translator.foreign_map)} 条映射\n"
        
        result += f"{'='*60}"
        return result
    
    def _cmd_train_all(self):
        """声母遍历韵母 — 0维→1维训练"""
        shengmu = self.loader.get_container("声母")
        yunmu = self.loader.get_container("韵母")
        
        if not shengmu or not yunmu:
            return "错误: 容器不存在"
        
        sm_count = len(shengmu.symbols)
        ym_count = len(yunmu.symbols)
        total = sm_count + ym_count  # 加法，非乘法
        
        result = f"训练模式: 声母({sm_count}) + 韵母({ym_count}) = {total} 次操作\n"
        result += f"{'─'*40}\n"
        
        # 只显示前10个，避免输出过多
        count = 0
        for sm_entry in shengmu.symbols[:5]:
            for ym_entry in yunmu.symbols[:5]:
                count += 1
                result += f"  {count}. {sm_entry['symbol']} + {ym_entry['symbol']}\n"
        
        if sm_count > 5 or ym_count > 5:
            result += f"  ... (共 {sm_count}×{ym_count} 组合，显示前25个)\n"
        
        result += f"{'─'*40}\n"
        result += f"注: 使用加法统摄，总操作数 = {total}（非乘法 {sm_count*ym_count}）"
        return result
    
    def interactive(self):
        """交互模式"""
        print("=" * 60)
        print("FQM 统一系统 · 交互模式")
        print("输入 /help 查看帮助，/quit 退出")
        print("=" * 60)
        
        while True:
            try:
                command = input("\nFQM> ").strip()
            except (EOFError, KeyboardInterrupt):
                print("\n退出")
                break
            
            if not command:
                continue
            
            if command in ('/quit', 'quit', '退出'):
                print("退出 FQM 系统")
                break
            
            if command in ('/help', 'help', '帮助'):
                self._print_help()
                continue
            
            result = self.execute(command)
            if result:
                print(result)
    
    def _print_help(self):
        """打印帮助信息"""
        print("""
╔══════════════════════════════════════════════════════════╗
║                    FQM 统一系统 命令帮助                  ║
╠══════════════════════════════════════════════════════════╣
║ 【容器操作】                                              ║
║   袋.声母              → 查询声母容器所有符号             ║
║   袋.声母.装入(b)      → 把b装入声母容器                  ║
║   袋.声母.取出(首)     → 取出声母容器首个符号             ║
║   袋.声母.比较(b, p)   → 比较两个符号                     ║
║   袋.声母.引.韵母(首)  → 查询声母首个链接的韵母           ║
╠══════════════════════════════════════════════════════════╣
║ 【三维空间】                                              ║
║   三维.坐标(b, a, 1)   → 查询三维坐标                     ║
║   三维.距离(b,a,1,p,o,2) → 计算两点距离                   ║
║   三维.邻近(b, a, 1)   → 查询邻近坐标                     ║
║   三维.路径(b,a,1,p,o,2) → 查找路径                       ║
║   三维.空间            → 列出所有坐标                     ║
╠══════════════════════════════════════════════════════════╣
║ 【维度升降】                                              ║
║   降维(b, a, 1, 声调)  → 去掉声调维度                     ║
║   升维(b, a, 声调)     → 升到三维                         ║
║   投影(b, a, 1, 声母)  → 只保留声母维度                   ║
║   展开(b, 韵母, 声调)  → 从声母展开到完整三维             ║
║   维度                 → 显示维度信息                     ║
╠══════════════════════════════════════════════════════════╣
║ 【关系运算】                                              ║
║   公约(声母, 韵母)     → 查找共有身份                     ║
║   公倍(声母.首, 韵母.首) → 计算组合                       ║
║   同公(b, p)           → 判断是否同一公约数               ║
║   齐公(声母.首, 韵母.首) → 判断是否对齐                   ║
║   关系(b, a)           → 判断两个符号的关系               ║
║   公约数               → 列出所有公约数                   ║
╠══════════════════════════════════════════════════════════╣
║ 【元层级】                                                ║
║   一元(目标)           → 声明一元目标                     ║
║   二元(中心, 辅助)     → 声明二元镜像关系                 ║
║   三元(观测, A, B)     → 声明三元见证                     ║
║   四元(外壳, 内容)     → 声明四元包含关系                 ║
║   裁判(A, B)           → 裁定A与B的关系                   ║
║   层级                 → 查看完整层级结构                 ║
╠══════════════════════════════════════════════════════════╣
║ 【翻译器】                                                ║
║   翻译(class)          → 翻译外部符号为ontology路径        ║
║   >def                 → 简写翻译                         ║
╠══════════════════════════════════════════════════════════╣
║ 【错别字纠正】                                            ║
║   纠正(带)             → 纠正单个错别字                   ║
║   纠正(文子有错)       → 纠正整段文字中的错别字           ║
╠══════════════════════════════════════════════════════════╣
║ 【注册表查询】                                            ║
║   注册.声母            → 查询注册表声母符号               ║
║   注册.韵母            → 查询注册表韵母符号               ║
║   注册.汉字            → 查询注册表汉字符号               ║
║   注册.偏移            → 查询全局偏移量                   ║
║   注册.错别字          → 查询错别字纠正规则               ║
╠══════════════════════════════════════════════════════════╣
║ 【鼓膜自举】                                              ║
║   自举 / 鼓膜           → 查看鼓膜自举状态                 ║
║   骨膜 / 骨架 / 骨魔    → 查看不可变元规则                ║
║   驱动.L1               → 外部因果驱动索引                 ║
║   驱动.L2               → 情节钩子索引                     ║
║   驱动.L3               → 行为过程快照                     ║
║   驱动.L4               → 版本-日期锚定历史                ║
║   驱动.禁止             → 禁止链（子代理不可用）           ║
╠══════════════════════════════════════════════════════════╣
║ 【系统】                                                  ║
║   状态 / /status       → 系统状态总览                     ║
║   训练全部 / /train_all → 声母+韵母训练                   ║
║   帮助 / /help         → 显示本帮助                       ║
║   退出 / /quit         → 退出系统                         ║
╚══════════════════════════════════════════════════════════╝
""")


# ============================================================
# 入口
# ============================================================

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='FQM 统一系统')
    parser.add_argument('--exec', '-e', help='执行单条命令')
    parser.add_argument('--interactive', '-i', action='store_true', help='交互模式')
    args = parser.parse_args()
    
    system = FQMUnifiedSystem()
    if not system.start():
        print("系统启动失败")
        sys.exit(1)
    
    if args.exec:
        result = system.execute(args.exec)
        if result:
            print(result)
    else:
        # 默认交互模式
        system.interactive()
