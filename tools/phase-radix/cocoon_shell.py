#!/usr/bin/env python3
"""
cocoon_shell.py — 蚕蛹反向寄生壳

核心哲学:
  使外部系统模仿自身 · 而非自身适应外部规则

架构:
  ┌─────────────────────────────────────────────────────────────┐
  │  Cocoon Package (蚕蛹初始包)                                │
  │  ├── Parser (解析器) — 识别外部系统格式                      │
  │  ├── Shell Catalog (寄生壳清单) — 已知外部系统适配方案       │
  │  ├── Matcher (匹配器) — 自动选择最佳寄生壳                   │
  │  ├── Shedding/Wearing (脱壳穿衣) — 执行适配转换              │
  │  ├── Config Generator (配置生成) — 产出挂载配置              │
  │  └── Kernel Mounter (内核挂载) — 将壳连接到 WASM 内核        │
  └─────────────────────────────────────────────────────────────┘

流程:
  1. 下载/加载蚕蛹初始包
  2. 解析外部系统 (识别其规则/格式)
  3. 选择寄生壳 (手动或自动匹配)
  4. 执行脱壳穿衣 (外部规则 → 内部规则映射)
  5. 生成配置文件
  6. 挂载到 WASM 内核

红线:
  - 绝不修改自身规则去适应外部
  - 所有外部输入必须经过寄生壳转译
  - 寄生壳是可替换的 · 规则是固定的

用法:
  python3 cocoon_shell.py --catalog           — 列出可用寄生壳
  python3 cocoon_shell.py --parse <file>      — 解析外部系统
  python3 cocoon_shell.py --match <type>      — 匹配最佳寄生壳
  python3 cocoon_shell.py --wear <shell>      — 执行穿衣操作
  python3 cocoon_shell.py --mount <config>    — 挂载到内核
"""

import os
import re
import json
import hashlib
import time
from pathlib import Path
from typing import List, Dict, Optional, Any, Tuple
from enum import Enum

# ── 路径 ──────────────────────────────────────────────────

TOOLS_DIR = Path(__file__).parent.resolve()
ENCODING_DIR = TOOLS_DIR.parent.parent / "encoding"
CONFIG_DIR = TOOLS_DIR / "phase-radix" / "cocoon_configs"

# ── 寄生壳类型 ─────────────────────────────────────────────

class ShellType(Enum):
    """寄生壳类型 · 针对不同外部系统"""
    HTTP_API = "http_api"           # REST/GraphQL API
    DATABASE = "database"           # SQL/NoSQL 数据库
    FILE_SYSTEM = "file_system"     # 文件系统接口
    WEBSOCKET = "websocket"         # WebSocket 流
    GRPC = "grpc"                   # gRPC 服务
    CLI = "cli"                     # 命令行工具
    LIBRARY = "library"             # 共享库/DLL
    BROWSER = "browser"             # 浏览器自动化
    EMAIL = "email"                 # 邮件系统
    DNS = "dns"                     # DNS 解析
    CUSTOM = "custom"               # 自定义壳

class ShellCapability(Enum):
    """寄生壳能力"""
    READ = "read"                   # 读取外部
    WRITE = "write"                 # 写入外部
    STREAM = "stream"               # 流式传输
    BATCH = "batch"                 # 批量操作
    ASYNC = "async"                 # 异步操作
    ENCRYPT = "encrypt"             # 加密传输
    CACHE = "cache"                 # 缓存层

# ── 寄生壳定义 ─────────────────────────────────────────────

class ParasiticShell:
    """寄生壳定义"""
    def __init__(self, shell_id: str, name: str, shell_type: ShellType,
                 capabilities: List[ShellCapability],
                 description: str = "", version: str = "1.0"):
        self.id = shell_id
        self.name = name
        self.type = shell_type
        self.capabilities = capabilities
        self.description = description
        self.version = version
        self.rules: Dict[str, str] = {}  # 外部规则 → 内部规则映射
    
    def add_rule_mapping(self, external_rule: str, internal_rule: str):
        """添加规则映射"""
        self.rules[external_rule] = internal_rule
    
    def to_dict(self) -> Dict:
        return {
            "id": self.id,
            "name": self.name,
            "type": self.type.value,
            "capabilities": [c.value for c in self.capabilities],
            "description": self.description,
            "version": self.version,
            "rules": self.rules,
        }

# ── 寄生壳目录 ─────────────────────────────────────────────

class ShellCatalog:
    """寄生壳目录 · 管理所有可用壳"""
    
    def __init__(self):
        self.shells: Dict[str, ParasiticShell] = {}
        self._load_defaults()
    
    def _load_defaults(self):
        """加载默认寄生壳定义"""
        # HTTP API 壳
        http = ParasiticShell(
            "http_rest", "HTTP REST API", ShellType.HTTP_API,
            [ShellCapability.READ, ShellCapability.WRITE, ShellCapability.BATCH],
            "RESTful API 适配壳 · 将 HTTP 动词映射为内部流操作",
        )
        http.add_rule_mapping("GET", "WK_FLOW_PRELOAD")
        http.add_rule_mapping("POST", "FLOW_PROPERTY")
        http.add_rule_mapping("PUT", "FLOW_INTERPRET")
        http.add_rule_mapping("DELETE", "FLOW_DISTINGUISH")
        http.add_rule_mapping("Content-Type", "MCCP encoding")
        http.add_rule_mapping("Authorization", "anchor hash")
        self.shells[http.id] = http
        
        # 数据库壳
        db = ParasiticShell(
            "sql_db", "SQL Database", ShellType.DATABASE,
            [ShellCapability.READ, ShellCapability.WRITE, ShellCapability.BATCH, ShellCapability.CACHE],
            "SQL 数据库适配壳 · 将表/行/列映射为容器/符号/属性",
        )
        db.add_rule_mapping("TABLE", "CONTAINER")
        db.add_rule_mapping("ROW", "SYMBOL")
        db.add_rule_mapping("COLUMN", "PROPERTY")
        db.add_rule_mapping("INDEX", "INTERLOCK")
        db.add_rule_mapping("FOREIGN_KEY", "CHAIN")
        self.shells[db.id] = db
        
        # 文件系统壳
        fs = ParasiticShell(
            "fs_bridge", "File System", ShellType.FILE_SYSTEM,
            [ShellCapability.READ, ShellCapability.WRITE, ShellCapability.STREAM],
            "文件系统适配壳 · 将文件/目录映射为符号/容器",
        )
        fs.add_rule_mapping("FILE", "CELL")
        fs.add_rule_mapping("DIRECTORY", "CONTAINER")
        fs.add_rule_mapping("EXTENSION", "POLARITY")
        fs.add_rule_mapping("METADATA", "WITNESS")
        fs.add_rule_mapping("CONTENT", "MCCP_CODEPOINT")
        self.shells[fs.id] = fs
        
        # WebSocket 壳
        ws = ParasiticShell(
            "ws_stream", "WebSocket Stream", ShellType.WEBSOCKET,
            [ShellCapability.READ, ShellCapability.WRITE, ShellCapability.STREAM, ShellCapability.ASYNC],
            "WebSocket 适配壳 · 将消息流映射为八流统一",
        )
        ws.add_rule_mapping("MESSAGE", "FLOW")
        ws.add_rule_mapping("CHANNEL", "CONTAINER")
        ws.add_rule_mapping("SUBSCRIBE", "PRELOAD")
        ws.add_rule_mapping("PUBLISH", "PROPERTY")
        self.shells[ws.id] = ws
        
        # gRPC 壳
        grpc = ParasiticShell(
            "grpc_bridge", "gRPC Service", ShellType.GRPC,
            [ShellCapability.READ, ShellCapability.WRITE, ShellCapability.STREAM, ShellCapability.BATCH],
            "gRPC 适配壳 · 将 service/method 映射为容器/符号",
        )
        grpc.add_rule_mapping("SERVICE", "CONTAINER")
        grpc.add_rule_mapping("METHOD", "SYMBOL")
        grpc.add_rule_mapping("STREAM", "FLOW")
        self.shells[grpc.id] = grpc
        
        # CLI 壳
        cli = ParasiticShell(
            "cli_bridge", "CLI Tool", ShellType.CLI,
            [ShellCapability.READ, ShellCapability.WRITE],
            "命令行工具适配壳 · 将参数/选项映射为属性/极性",
        )
        cli.add_rule_mapping("ARGUMENT", "PROPERTY")
        cli.add_rule_mapping("FLAG", "POLARITY")
        cli.add_rule_mapping("SUBCOMMAND", "CONTAINER")
        self.shells[cli.id] = cli
        
        # 浏览器壳
        browser = ParasiticShell(
            "browser_bridge", "Browser Automation", ShellType.BROWSER,
            [ShellCapability.READ, ShellCapability.WRITE, ShellCapability.STREAM],
            "浏览器适配壳 · 将 DOM/事件映射为容器/流",
        )
        browser.add_rule_mapping("DOM_ELEMENT", "SYMBOL")
        browser.add_rule_mapping("EVENT", "FLOW")
        browser.add_rule_mapping("COOKIE", "ANCHOR")
        browser.add_rule_mapping("SESSION", "STATE")
        self.shells[browser.id] = browser
    
    def list_shells(self) -> List[ParasiticShell]:
        return list(self.shells.values())
    
    def get(self, shell_id: str) -> Optional[ParasiticShell]:
        return self.shells.get(shell_id)
    
    def find_by_type(self, shell_type: ShellType) -> List[ParasiticShell]:
        return [s for s in self.shells.values() if s.type == shell_type]
    
    def find_by_capability(self, cap: ShellCapability) -> List[ParasiticShell]:
        return [s for s in self.shells.values() if cap in s.capabilities]

# ── 外部系统解析器 ─────────────────────────────────────────

class ExternalSystemParser:
    """解析外部系统 · 识别其规则/格式"""
    
    def __init__(self):
        self.detected_type: Optional[ShellType] = None
        self.detected_rules: Dict[str, str] = {}
        self.confidence: float = 0.0
    
    def parse_file(self, filepath: Path) -> Dict[str, Any]:
        """解析文件识别外部系统类型"""
        text = filepath.read_text(encoding='utf-8', errors='replace')
        suffix = filepath.suffix.lower()
        
        # 基于扩展名和内容特征识别
        if suffix in ('.json',):
            return self._detect_json_api(text)
        elif suffix in ('.sql', '.db', '.sqlite'):
            return self._detect_database(text)
        elif suffix in ('.proto',):
            return self._detect_grpc(text)
        elif suffix in ('.html', '.htm'):
            return self._detect_browser(text)
        elif suffix in ('.graphql',):
            return self._detect_graphql(text)
        elif 'openapi' in text.lower() or 'swagger' in text.lower():
            return self._detect_openapi(text)
        else:
            return self._detect_generic(text)
    
    def _detect_json_api(self, text: str) -> Dict:
        self.detected_type = ShellType.HTTP_API
        self.confidence = 0.7
        # 检查是否是 API 响应格式
        if '"data"' in text or '"errors"' in text:
            self.detected_rules["format"] = "JSON:API"
            self.confidence = 0.85
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": self.detected_rules}
    
    def _detect_database(self, text: str) -> Dict:
        self.detected_type = ShellType.DATABASE
        self.confidence = 0.9
        self.detected_rules["dialect"] = "SQL"
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": self.detected_rules}
    
    def _detect_grpc(self, text: str) -> Dict:
        self.detected_type = ShellType.GRPC
        self.confidence = 0.95
        self.detected_rules["format"] = "Protocol Buffers"
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": self.detected_rules}
    
    def _detect_browser(self, text: str) -> Dict:
        self.detected_type = ShellType.BROWSER
        self.confidence = 0.8
        self.detected_rules["format"] = "HTML/DOM"
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": self.detected_rules}
    
    def _detect_graphql(self, text: str) -> Dict:
        self.detected_type = ShellType.HTTP_API
        self.confidence = 0.9
        self.detected_rules["format"] = "GraphQL"
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": self.detected_rules}
    
    def _detect_openapi(self, text: str) -> Dict:
        self.detected_type = ShellType.HTTP_API
        self.confidence = 0.95
        self.detected_rules["format"] = "OpenAPI/Swagger"
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": self.detected_rules}
    
    def _detect_generic(self, text: str) -> Dict:
        self.detected_type = ShellType.CUSTOM
        self.confidence = 0.3
        return {"type": self.detected_type.value, "confidence": self.confidence, "rules": {}}

# ── 匹配器 ─────────────────────────────────────────────────

class ShellMatcher:
    """自动匹配最佳寄生壳"""
    
    def __init__(self, catalog: ShellCatalog):
        self.catalog = catalog
    
    def match(self, system_type: ShellType, required_caps: List[ShellCapability] = None) -> Optional[ParasiticShell]:
        """根据系统类型和能力需求匹配最佳壳"""
        candidates = self.catalog.find_by_type(system_type)
        
        if not candidates:
            return None
        
        if not required_caps:
            return candidates[0]
        
        # 按能力匹配度排序
        def score(shell: ParasiticShell) -> int:
            return sum(1 for c in required_caps if c in shell.capabilities)
        
        candidates.sort(key=score, reverse=True)
        return candidates[0] if candidates else None

# ── 脱壳穿衣引擎 ───────────────────────────────────────────

class ShedWearEngine:
    """脱壳穿衣引擎 · 执行外部规则到内部规则的转换"""
    
    def __init__(self, shell: ParasiticShell):
        self.shell = shell
        self.transformation_log: List[Dict] = []
    
    def shed(self, external_data: Dict) -> Dict:
        """脱壳: 剥离外部系统的规则包装"""
        stripped = {}
        for key, value in external_data.items():
            # 将外部键名映射为内部键名
            internal_key = self.shell.rules.get(key, key)
            stripped[internal_key] = value
            self.transformation_log.append({
                "action": "shed",
                "external": key,
                "internal": internal_key,
            })
        return stripped
    
    def wear(self, internal_data: Dict) -> Dict:
        """穿衣: 将内部数据包装为外部系统可接受的格式"""
        wrapped = {}
        # 反向映射
        reverse_rules = {v: k for k, v in self.shell.rules.items()}
        for key, value in internal_data.items():
            external_key = reverse_rules.get(key, key)
            wrapped[external_key] = value
            self.transformation_log.append({
                "action": "wear",
                "internal": key,
                "external": external_key,
            })
        return wrapped
    
    def transform(self, data: Dict, direction: str = "in") -> Dict:
        """双向转换"""
        if direction == "in":
            return self.shed(data)
        else:
            return self.wear(data)

# ── 配置生成器 ─────────────────────────────────────────────

class ConfigGenerator:
    """生成寄生壳挂载配置"""
    
    def __init__(self, shell: ParasiticShell):
        self.shell = shell
    
    def generate(self, mount_point: str = "/cocoon", extra: Dict = None) -> Dict:
        """生成完整配置"""
        config = {
            "version": "1.0",
            "shell": self.shell.to_dict(),
            "mount_point": mount_point,
            "created": time.strftime('%Y-%m-%dT%H:%M:%S'),
            "kernel": {
                "type": "wasm",
                "symbol_table": True,
                "interlock": True,
            },
            "rules": self.shell.rules,
            "integrity": {
                "hash_algorithm": "sha256",
                "shell_hash": hashlib.sha256(
                    json.dumps(self.shell.to_dict(), sort_keys=True).encode()
                ).hexdigest()[:16],
            },
        }
        if extra:
            config.update(extra)
        return config
    
    def save(self, config: Dict, path: Path) -> Path:
        """保存配置到文件"""
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(config, indent=2, ensure_ascii=False), encoding='utf-8')
        return path

# ── 内核挂载器 ─────────────────────────────────────────────

class KernelMounter:
    """将寄生壳挂载到 WASM 内核"""
    
    def __init__(self, config: Dict):
        self.config = config
        self.mounted = False
        self.mount_time = None
    
    def mount(self) -> bool:
        """执行挂载"""
        # 验证配置完整性
        if not self._validate():
            return False
        
        self.mounted = True
        self.mount_time = time.time()
        return True
    
    def _validate(self) -> bool:
        """验证配置"""
        required = ["shell", "mount_point", "rules"]
        return all(k in self.config for k in required)
    
    def status(self) -> Dict:
        return {
            "mounted": self.mounted,
            "shell": self.config.get("shell", {}).get("name", "?"),
            "mount_point": self.config.get("mount_point", "?"),
            "mount_time": self.mount_time,
        }

# ── CLI 入口 ───────────────────────────────────────────────

def main():
    import argparse
    parser = argparse.ArgumentParser(
        description='蚕蛹反向寄生壳 — 使外部系统模仿自身规则'
    )
    parser.add_argument('--catalog', action='store_true', help='列出可用寄生壳')
    parser.add_argument('--parse', metavar='FILE', help='解析外部系统')
    parser.add_argument('--match', metavar='TYPE', help='匹配最佳寄生壳')
    parser.add_argument('--wear', metavar='SHELL_ID', help='执行穿衣操作')
    parser.add_argument('--demo', action='store_true', help='演示完整流程')
    
    args = parser.parse_args()
    
    catalog = ShellCatalog()
    
    if args.catalog:
        print("═══ 寄生壳目录 ═══")
        for shell in catalog.list_shells():
            caps = ', '.join(c.value for c in shell.capabilities)
            print(f"  [{shell.id:12s}] {shell.name:20s} 类型={shell.type.value:12s}")
            print(f"              能力: [{caps}]")
            print(f"              规则映射: {len(shell.rules)} 条")
            print()
    
    elif args.parse:
        p = ExternalSystemParser()
        result = p.parse_file(Path(args.parse))
        print("═══ 外部系统解析 ═══")
        print(f"类型: {result['type']}")
        print(f"置信度: {result['confidence']}")
        print(f"规则: {result['rules']}")
    
    elif args.match:
        type_map = {t.value: t for t in ShellType}
        st = type_map.get(args.match)
        if not st:
            print(f"未知类型: {args.match}")
            return
        matcher = ShellMatcher(catalog)
        shell = matcher.match(st)
        if shell:
            print("═══ 匹配结果 ═══")
            print(f"推荐壳: {shell.name} ({shell.id})")
            print(f"能力: {[c.value for c in shell.capabilities]}")
            print(f"规则映射: {shell.rules}")
        else:
            print(f"未找到匹配的壳: {args.match}")
    
    elif args.wear:
        shell = catalog.get(args.wear)
        if not shell:
            print(f"未知壳: {args.wear}")
            return
        engine = ShedWearEngine(shell)
        # 演示数据转换
        demo_external = {"GET": "/api/users", "Content-Type": "application/json"}
        result = engine.shed(demo_external)
        print("═══ 脱壳穿衣演示 ═══")
        print(f"壳: {shell.name}")
        print(f"外部数据: {demo_external}")
        print(f"转换后: {result}")
    
    elif args.demo:
        print("═══ 蚕蛹反向寄生壳 · 完整流程演示 ═══")
        print()
        
        # 1. 解析外部系统
        print("[1] 解析外部系统")
        parser = ExternalSystemParser()
        demo_data = '{"GET": "/api/data", "Content-Type": "application/json", "Authorization": "Bearer xxx"}'
        Path('/tmp/demo_api.json').write_text(demo_data)
        result = parser.parse_file(Path('/tmp/demo_api.json'))
        print(f"  类型: {result['type']}, 置信度: {result['confidence']}")
        
        # 2. 匹配寄生壳
        print("[2] 匹配寄生壳")
        matcher = ShellMatcher(catalog)
        shell = matcher.match(ShellType.HTTP_API, [ShellCapability.READ, ShellCapability.WRITE])
        print(f"  选中: {shell.name}")
        
        # 3. 脱壳穿衣
        print("[3] 脱壳穿衣")
        engine = ShedWearEngine(shell)
        external = {"GET": "/api/users", "Content-Type": "application/json"}
        internal = engine.shed(external)
        print(f"  外部: {external}")
        print(f"  内部: {internal}")
        
        # 4. 生成配置
        print("[4] 生成配置")
        gen = ConfigGenerator(shell)
        config = gen.generate(mount_point="/cocoon/api")
        print(f"  挂载点: {config['mount_point']}")
        print(f"  规则数: {len(config['rules'])}")
        
        # 5. 挂载
        print("[5] 挂载内核")
        mounter = KernelMounter(config)
        ok = mounter.mount()
        print(f"  挂载: {'成功' if ok else '失败'}")
        
        print()
        print("═══ 演示完成 · 外部系统已适配为自身规则 ═══")
        os.remove('/tmp/demo_api.json')
    
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
