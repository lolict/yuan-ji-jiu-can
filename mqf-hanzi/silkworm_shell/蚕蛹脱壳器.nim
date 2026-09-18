## 蚕蛹反向寄生壳 · 脱壳穿衣器
##
##  流程：环境探测 → 解析清单 → 匹配寄生壳 → 脱壳穿衣 → 生成配置 → 挂载内核
##
##  核心哲学：让外部系统模仿满全法，而非满全法适应外部
##
##  运行：nim c --cc:clang -r 蚕蛹脱壳器.nim
##        nim c --cc:clang -r 蚕蛹脱壳器.nim -- --shell=cli
##        nim c --cc:clang -r 蚕蛹脱壳器.nim -- --shell=browser

import std/parsecfg
import std/tables
import std/strutils
import std/os
import std/sequtils
import std/times

# =================================================================
#  类型
# =================================================================
type
  寄生壳* = object
    名称*: string
    匹配平台*: seq[string]
    环境指示器*: seq[string]
    内核源码*: string
    编译目标*: string
    穿衣操作*: seq[string]
    反向寄生机制*: string
    手动选择*: string
    穿衣产物*: seq[string]

  环境信息* = object
    操作系统*: string
    编译器*: string
    运行时*: string
    包管理器*: string

  穿衣结果* = object
    壳名称*: string
    状态*: string    # matched / manual / fallback
    配置文件*: string
    挂载入口*: string

# =================================================================
#  环境探测
# =================================================================
proc 探测环境(): 环境信息 =
  result.操作系统 = "linux"
  result.编译器 = "clang"
  result.运行时 = "native"
  result.包管理器 = "none"

  # 探测 nim
  if findExe("nim") != "":
    result.运行时 = "nim"

  # 探测 wasm-ld
  when defined(wasm):
    result.运行时 = "wasm"
    result.操作系统 = "browser"

  # 探测包管理器
  for pm in ["npm", "yarn", "pnpm", "cargo", "pip"]:
    if findExe(pm) != "":
      result.包管理器 = pm
      break

# =================================================================
#  解析清单文本（简单版，支持 [@段名] + key=value）
# =================================================================
proc 解析清单*(路径: string): seq[寄生壳] =
  if not 路径.fileExists: return @[]

  var 当前壳:寄生壳
  var in_section = false

  for rawLine in 路径.readFile.splitLines:
    let 行 = rawLine.strip
    if 行.len == 0 or 行[0] == '#':
      # 空行或注释 → 段结束
      if in_section and 当前壳.名称.len > 0:
        result.add(当前壳)
        当前壳 =寄生壳()
        in_section = false
      continue

    if 行.len > 2 and 行[0] == '[' and 行[1] == '@' and 行[^1] == ']':
      in_section = true
      当前壳.名称 = 行[2 .. ^2]
      continue

    if in_section and '=' in 行:
      let 分隔 = 行.find('=')
      let 键 = 行[0 ..< 分隔].strip
      discard 行[分隔+1 .. ^1].strip
      case 键
      of "匹配目标":
        discard  # placeholder · 解析逻辑由内置壳列表替代

# =================================================================
#  简化版·手动清单（先用静态定义，因为清单格式解析复杂）
# =================================================================
proc 内置壳列表(): seq[寄生壳] =
  result = @[
    寄生壳(
      名称: "browser-wasm",
      匹配平台: @["browser", "web"],
      环境指示器: @["window", "wasm"],
      内核源码: "../wasm_kernel/mqf_container.c",
      编译目标: "wasm32",
      穿衣操作: @[
        "编译 mqf_container.c → wasm32",
        "生成 js_glue.js",
        "注入域规则数据",
        "暴露 mqf_bridge_* 导出"
      ],
      反向寄生机制: "JS 调用 mqf.wasm 导出函数，浏览器事件循环遵循 SMRL 引擎极性判决",
      手动选择: "browser"
    ),
    寄生壳(
      名称: "cli-nim",
      匹配平台: @["linux", "macos"],
      环境指示器: @["nim", "clang"],
      内核源码: "../wasm_kernel/mqf_container.c",
      编译目标: "kernel_64",
      穿衣操作: @[
        "调用阶梯构建器 → kernel_64",
        "Nim 读取域索引 → 容器路由",
        "CLI 参数按容器层级分组",
        "帮助文本由容器字段反射生成"
      ],
      反向寄生机制: "CLI 参数按相位分组，路由由极性判决决定",
      手动选择: "cli"
    ),
    寄生壳(
      名称: "mobile-android",
      匹配平台: @["android"],
      环境指示器: @["kotlin", "jni"],
      内核源码: "../wasm_kernel/mqf_container.c",
      编译目标: "arm64",
      穿衣操作: @[
        "交叉编译 mqf_container.c → libmqf_core.a",
        "生成 Kotlin JNI 绑定(MqfBridge.kt)",
        "注入 MqfService 到 AndroidManifest",
        "Activity 生命周期 → SMRL 五步循环"
      ],
      反向寄生机制: "Android Activity 事件被映射到 测距/吃/合一/沉淀/驱动",
      手动选择: "android"
    ),
    寄生壳(
      名称: "cloud-docker",
      匹配平台: @["docker", "kubernetes"],
      环境指示器: @["containerd", "podman"],
      内核源码: "../wasm_kernel/mqf_container.c",
      编译目标: "kernel_64",
      穿衣操作: @[
        "编译为静态二进制 + 域索引",
        "打入 distroless 镜像",
        "暴露 gRPC: ContainerJudge/Relate/Merge",
        "业务微服务调用 mqf-sidecar 做裁决"
      ],
      反向寄生机制: "服务网格 sidecar，业务代码不变，旁路裁决器由满全法驱动",
      手动选择: "docker"
    )
  ]

# =================================================================
#  匹配壳
# =================================================================
proc 匹配壳*(壳列表: seq[寄生壳], 环境: 环境信息, 用户指定: string): 寄生壳 =
  # 用户手动指定
  if 用户指定.len > 0:
    for 壳 in 壳列表:
      if 壳.手动选择 == 用户指定 or 壳.名称 == 用户指定:
        return 壳
    echo "  [警告] 未找到指定壳='" & 用户指定 & "'，回退自动匹配"

  # 自动匹配
  for 壳 in 壳列表:
    for 平台 in 壳.匹配平台:
      if 环境.操作系统 == 平台 or 环境.包管理器 == 平台:
        return 壳

  # 默认回退
  result = 壳列表[0]  # browser 作为默认
  result.名称 = result.名称 & " [fallback]"

# =================================================================
#  生成穿衣配置
# =================================================================
proc 生成穿衣配置*(壳: 寄生壳, 环境: 环境信息): 穿衣结果 =
  result.壳名称 = 壳.名称
  result.状态 = "worn"
  result.配置文件 = "mqf_worn_" & 壳.手动选择 & ".toml"

  # 生成配置文件
  var 配置: seq[string]
  配置.add "# 蚕蛹穿衣结果 · 自动生成"
  配置.add "# 时间: " & $now()
  配置.add "# 脱壳器版本·v0.1"
  配置.add ""
  配置.add "[壳]"
  配置.add "名称=" & 壳.名称
  配置.add "编译目标=" & 壳.编译目标
  配置.add "内核=" & 壳.内核源码
  配置.add "反向寄生=" & 壳.反向寄生机制
  配置.add ""
  配置.add "[环境]"
  配置.add "操作系统=" & 环境.操作系统
  配置.add "编译器=" & 环境.编译器
  配置.add "运行时=" & 环境.运行时
  配置.add "包管理器=" & 环境.包管理器
  配置.add ""
  配置.add "[穿衣操作]"
  for i, 操作 in 壳.穿衣操作:
    配置.add "  步骤" & $(i+1) & "=" & 操作
  配置.add ""
  配置.add "[挂载]"
  配置.add "域规则入口=../自举编译器/域索引.txt"
  配置.add "内核初始化=mqf_bridge_init"
  配置.add "第一个容器=阳容器·相位0·第1层·坐标0x00010000"

  result.配置文件.writeFile(配置.join("\n") & "\n")
  result.挂载入口 = 壳.手动选择 & "_main"

# =================================================================
#  演示
# =================================================================
when isMainModule:
  echo "========================================="
  echo "  蚕蛹反向寄生壳 · 脱壳穿衣器"
  echo "========================================="

  # 解析用户 --shell 参数
  var 用户指定壳 = ""
  var i = 1
  while i < paramCount():
    let 参 = paramStr(i)
    if 参.startsWith("--shell="):
      用户指定壳 = 参[8..^1].strip
    inc i

  # 环境探测
  echo ""
  echo "▸ 运行:"
  let 环境 = 探测环境()
  echo "  操作系统: " & 环境.操作系统
  echo "  编译器:   " & 环境.编译器
  echo "  运行时:   " & 环境.运行时
  echo "  包管理器: " & 环境.包管理器

  # 获取壳列表
  let 壳们 = 内置壳列表()
  echo ""
  echo "▸ 已注册寄生壳(" & $壳们.len & "):"
  for 壳 in 壳们:
    echo "    " & 壳.名称 & " (" & 壳.手动选择 & ") → 匹配: " & 壳.匹配平台.join(", ")

  # 匹配
  echo ""
  if 用户指定壳.len > 0:
    echo "▸ 用户手动选择: " & 用户指定壳
  else:
    echo "▸ 自动匹配模式"

  let 目标壳 = 匹配壳(壳们, 环境, 用户指定壳)
  echo "▸ 匹配结果: " & 目标壳.名称

  # 穿衣
  echo ""
  echo "▸ 脱壳穿衣:"
  for idx, 步骤 in 目标壳.穿衣操作:
    echo "    [" & $(idx+1) & "] " & 步骤

  # 反向寄生
  echo ""
  echo "▸ 反向寄生机制:"
  echo "    " & 目标壳.反向寄生机制

  # 生成配置
  let 结果 = 生成穿衣配置(目标壳, 环境)
  echo ""
  echo "▸ 穿衣结果:"
  echo "    状态: " & 结果.状态
  echo "    配置文件: " & 结果.配置文件
  echo "    挂载入口: " & 结果.挂载入口

  echo ""
  echo "========================================="
  echo "  蚕蛹穿衣完成"
  echo "  内核着装完毕，外部系统将模仿满全法"
  echo "========================================="

  # 打印生成的配置文件
  if 结果.配置文件.fileExists:
    echo ""
    echo "── 生成的配置文件 ──"
    echo 结果.配置文件.readFile
