# ============================================================
# 蚕蛹反向寄生壳 · 外壳注册表
# 列出所有可寄生的外壳选项
# ============================================================

import std/tables
import ../kernel

type
  ShellType* = enum
    stWeb,        # 网页外壳 (HTML/JS)
    stApi,        # API 外壳 (REST/JSON)
    stCli,        # 命令行外壳 (终端)
    stDatabase,   # 数据库外壳 (SQL)
    stFileSystem, # 文件系统外壳 (目录/文件)
    stMessage     # 消息外壳 (IM/邮件)

  ShellInfo* = object
    name*: string
    shell_type*: ShellType
    description*: string
    interface_sig*: string   # 外壳的接口签名
    compatible*: bool

# 外壳注册表
var SHELL_REGISTRY*: seq[ShellInfo] = @[]

# 初始化注册表
proc init_registry*() {.sideEffect.} =
  if SHELL_REGISTRY.len > 0: return
  SHELL_REGISTRY = @[
    ShellInfo(
      name: "web-shell",
      shell_type: stWeb,
      description: "寄生到网页服务器，让网页显示内核内容",
      interface_sig: "HTTP/HTML",
      compatible: true
    ),
    ShellInfo(
      name: "api-shell",
      shell_type: stApi,
      description: "寄生到 API 网关，让外部调用内核接口",
      interface_sig: "REST/JSON",
      compatible: true
    ),
    ShellInfo(
      name: "cli-shell",
      shell_type: stCli,
      description: "寄生到命令行工具，让终端输出内核状态",
      interface_sig: "STDIN/STDOUT",
      compatible: true
    ),
    ShellInfo(
      name: "db-shell",
      shell_type: stDatabase,
      description: "寄生到数据库，让表结构反映内核状态",
      interface_sig: "SQL",
      compatible: true
    ),
    ShellInfo(
      name: "fs-shell",
      shell_type: stFileSystem,
      description: "寄生到文件系统，让目录结构映射内核",
      interface_sig: "PATH/INODE",
      compatible: true
    ),
    ShellInfo(
      name: "msg-shell",
      shell_type: stMessage,
      description: "寄生到消息系统，让消息携带内核数据",
      interface_sig: "TOPIC/PAYLOAD",
      compatible: true
    )
  ]

# 列出所有外壳
proc list_shells*(): string {.sideEffect.} =
  init_registry()
  var s = "蚕蛹外壳清单\n"
  s &= "────────────────────────────\n"
  for i, sh in SHELL_REGISTRY:
    s &= $i & ". " & sh.name & " [" & sh.interface_sig & "]\n"
    s &= "   " & sh.description & "\n"
    s &= "   兼容: " & (if sh.compatible: "是" else: "否") & "\n\n"
  result = s

# 选择外壳
proc select_shell*(idx: int): ShellInfo {.sideEffect.} =
  init_registry()
  if idx >= 0 and idx < SHELL_REGISTRY.len:
    result = SHELL_REGISTRY[idx]
  else:
    result = ShellInfo(name: "invalid", compatible: false)

# 获取外壳数量
proc shell_count*(): int {.sideEffect.} =
  init_registry()
  result = SHELL_REGISTRY.len
