## ═══════════════════════════════════════════════════════════════
## 蚕蛹反向寄生壳 · 脱壳穿衣 · 使外奉内 · 用户自带基础设施
## ═══════════════════════════════════════════════════════════════
## 核心哲学:
##   蚕蛹初始包: 解析器+选项清单+内核种子
##   脱壳穿衣: 用户选壳 → 执行脱壳 → 穿衣生成配置 → 挂载内核
##   反向寄生: 外部系统模仿自身规则 · 不是自身适应外部
##   用户自带: 参与者自带域名/服务器/身份 · 用其名义部署
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os

# ═══════════════════════════════════════════════════════════════
# 壳类型(宿主系统)
# ═══════════════════════════════════════════════════════════════
type
  ShellType* = enum
    stNone,             # 无壳(裸奔)
    stNginx,            # Nginx Web服务器
    stDocker,           # Docker容器
    stSystemd,          # Linux systemd
    stAndroid,          # Android IME
    stBrowser,          # 浏览器扩展
    stGitHub,           # GitHub Pages/Actions
    stWebhook,          # Webhook接收
    stLocalFS           # 本地文件系统

  ShellOption* = object
    name*: string
    shell_type*: ShellType
    description*: string
    capabilities*: seq[string]
    risk_level*: float      # 风险(0-1 · 越低越安全)
    user_owned*: bool       # 是否用户自带

# ═══════════════════════════════════════════════════════════════
# 蚕蛹包(核心系统种子)
# ═══════════════════════════════════════════════════════════════
type
  CocoonSeed* = object
    parser*: string              # 解析器代码
    rules*: seq[string]          # 满全法编码规则
    kernel_fragments*: seq[string]  # 内核碎片
    identity_anchor*: string     # 身份锚点(隐藏根基)

  CocoonPackage* = object
    version*: string                    # 蚕蛹包版本
    seed*: CocoonSeed                   # 内核种子
    options*: seq[ShellOption]          # 可选壳清单
    active_shell*: ShellType            # 当前穿衣的壳
    config*: Table[string, string]      # 生成的配置
    mounted*: bool                      # 内核是否已挂载

# ═══════════════════════════════════════════════════════════════
# 脱壳穿衣操作
# ═══════════════════════════════════════════════════════════════
type
  CocoonOperation* = enum
    coUndress,      # 脱壳(脱离旧宿主)
    coMatch,        # 匹配(选择新壳)
    coDress,        # 穿衣(生成配置)
    coMount,        # 挂载(接入内核)
    coDetach,       # 脱离(安全离开)
    coRebirth       # 重生(迁移到新宿主)

# ═══════════════════════════════════════════════════════════════
# 系统初始化
# ═══════════════════════════════════════════════════════════════
func new_cocoon_package*(): CocoonPackage =
  # 内核种子 = 满全法核心编码规则
  var seed = CocoonSeed(
    parser: "满全法容器解析器",
    rules: @[
      "_ROOT_0=刘楚恬 · _ROOT_1=我",
      "基元冻结不可变",
      "零维根映射不可修改",
      "白名单外字符一律死刑",
      "后来的不能冲撞元老",
    ],
    kernel_fragments: @[
      "声母容器(24基元)",
      "韵母容器(33韵母×2=66)",
      "声调容器(4贴附层)",
      "颜色容器(42相位)",
      "空间容器(18索引)",
      "音乐容器(3层级)",
    ],
    identity_anchor: "隐藏根基=用户身份对外不可见"
  )
  
  var pkg = CocoonPackage(
    version: "v1.0.0",
    seed: seed,
    active_shell: stNone,
    mounted: false
  )
  pkg.config = initTable[string, string]()
  
  # 可选壳清单
  pkg.options = @[
    ShellOption(name: "GitHub Pages", shell_type: stGitHub,
                description: "静态托管 · 全球CDN · 免费",
                capabilities: @["静态站点", "自定义域名", "HTTPS"],
                risk_level: 0.2, user_owned: true),
    ShellOption(name: "Docker自部署", shell_type: stDocker,
                description: "容器化部署 · 完全可控",
                capabilities: @["隔离环境", "快速迁移", "快照恢复"],
                risk_level: 0.3, user_owned: true),
    ShellOption(name: "Android IME", shell_type: stAndroid,
                description: "本地输入法 · 离线运行",
                capabilities: @["系统级输入", "无网络依赖", "隐私保护"],
                risk_level: 0.1, user_owned: true),
    ShellOption(name: "Webhook隐藏", shell_type: stWebhook,
                description: "隐藏在正常HTTP请求中",
                capabilities: @["隐蔽传输", "自动触发", "无独立端口"],
                risk_level: 0.4, user_owned: true),
    ShellOption(name: "本地文件系统", shell_type: stLocalFS,
                description: "纯本地运行 · 不需要网络",
                capabilities: @["完全离线", "最高隐私", "无外部依赖"],
                risk_level: 0.05, user_owned: true),
  ]
  
  return pkg

# ═══════════════════════════════════════════════════════════════
# 匹配: 选择最适合的壳
# ═══════════════════════════════════════════════════════════════
func match_shell*(pkg: CocoonPackage; user_constraints: seq[string]): ShellOption =
  ## 根据用户需求自动匹配最佳壳
  var best_score = -1.0
  var best_option = ShellOption(name: "无", shell_type: stNone)
  
  for opt in pkg.options:
    var score = 1.0 - opt.risk_level  # 基础分: 越安全越好
    
    # 用户自带基础设施加分
    if opt.user_owned:
      score += 0.3
    
    # 能力匹配
    for constraint in user_constraints:
      if constraint in opt.name or constraint in opt.description:
        score += 0.5
      for cap in opt.capabilities:
        if constraint in cap:
          score += 0.2
    
    if score > best_score:
      best_score = score
      best_option = opt
  
  return best_option

# ═══════════════════════════════════════════════════════════════
# 脱壳穿衣: 执行完整的壳切换
# ═══════════════════════════════════════════════════════════════
proc undress*(pkg: var CocoonPackage): string =
  ## 脱离当前壳 · 回到裸奔状态
  if pkg.active_shell == stNone:
    return "已在裸奔状态 · 无壳可脱"
  
  let old_shell = pkg.active_shell
  pkg.active_shell = stNone
  pkg.mounted = false
  pkg.config.clear()
  
  return "脱壳完成 · 从" & $old_shell & "脱离"

proc dress*(pkg: var CocoonPackage; shell: ShellType; params: Table[string, string]): string =
  ## 穿衣: 生成挂载配置
  pkg.active_shell = shell
  
  # 根据壳类型生成对应配置
  case shell:
  of stGitHub:
    pkg.config["type"] = "github_pages"
    pkg.config["cname"] = params.getOrDefault("domain", "user.example.com")
    pkg.config["source_branch"] = "main"
    pkg.config["build_cmd"] = "nim c -r full_demo.nim"
  of stDocker:
    pkg.config["type"] = "docker_container"
    pkg.config["image"] = "nim:2.2-alpine"
    pkg.config["mount"] = "/app/cognition_os"
    pkg.config["restart"] = "always"
  of stAndroid:
    pkg.config["type"] = "android_ime"
    pkg.config["package"] = "com.manquanfa.keyboard"
    pkg.config["architecture"] = "kotlin+jni"
  of stWebhook:
    pkg.config["type"] = "webhook_receiver"
    pkg.config["endpoint"] = params.getOrDefault("url", "/api/hook")
    pkg.config["auth_header"] = "X-ManQuanFa-Auth"
  of stLocalFS:
    pkg.config["type"] = "local_fs"
    pkg.config["data_dir"] = "/mnt/data/catpaw/home/.meituan-catpaw"
    pkg.config["config_dir"] = "/mnt/data/catpaw/home/.meituan-catpaw/9199532794"
  else:
    pkg.config["type"] = "none"
  
  return "穿衣完成 · 壳=" & $shell & " · 生成" & $pkg.config.len & "项配置"

proc mount*(pkg: var CocoonPackage): string =
  ## 挂载内核
  if pkg.active_shell == stNone:
    return "无壳 · 无法挂载"
  
  pkg.mounted = true
  return "内核挂载成功 · " & $pkg.active_shell & "上运行满全法系统"

# ═══════════════════════════════════════════════════════════════
# 反向寄生核心: 使外部系统模仿自身规则
# ═══════════════════════════════════════════════════════════════
func parasitic_rules*(pkg: CocoonPackage): seq[string] =
  ## 生成反向寄生规则清单
  return @[
    "规则1: 外部接口只暴露编码后的符号 · 不暴露解析器",
    "规则2: 参与者使用自带身份部署 · 平台方不署名",
    "规则3: 业务规则(满全法编码)由用户自定 · 平台只提供解析器",
    "规则4: 参与者自带AI Key接入 · 平台不承担API成本",
    "规则5: 蚕蛹包更新时 · 参与者同步更新本地解析器",
    "规则6: 外部调用遵循 · 白名单外字符一律死刑 · 不配合的平台拒绝服务",
    "规则7: 用户自带基础设施(域名/服务器) · 平台零成本零责任",
    "规则8: 身份落款=刘楚恬和我 · 不是身份证个人名义",
  ]

# ═══════════════════════════════════════════════════════════════
# 盟机合模式
# ═══════════════════════════════════════════════════════════════
func mengjihe_mode*(role: string): string =
  ## 盟机合身份定位
  case role:
  of "owner": return "盟机合掌 · 拥有资源 · 运营场"
  of "node": return "盟机场/萌机场 · 节点 · 主权归属"
  else:
    return "未定义角色"

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var pkg = new_cocoon_package()
  
  echo "═══ 蚕蛹反向寄生壳 ═══"
  echo "版本: ", pkg.version
  echo "内核规则: ", pkg.seed.rules.len, "条"
  echo "身份锚点: ", pkg.seed.identity_anchor
  echo ""
  
  echo "── 可选壳清单 ──"
  for opt in pkg.options:
    echo "  ", alignLeft(opt.name, 12), " 风险:", formatFloat(opt.risk_level, ffDecimal, 2),
         " 自带:", if opt.user_owned: "✓" else: "✗"
    echo "    ", opt.description
    echo "    能力: ", opt.capabilities.join(", ")
  
  echo ""
  echo "── 自动匹配(要求:自带+离线+隐私) ──"
  let matched = pkg.match_shell(@["自带", "离线", "隐私", "本地"])
  echo "  最佳匹配: ", matched.name, " (风险:", formatFloat(matched.risk_level, ffDecimal), ")"
  
  echo ""
  echo "── 脱壳穿衣流程 ──"
  # 穿衣
  var params = initTable[string, string]()
  params["domain"] = "manquanfa.local"
  params["url"] = "/api/v1/hook"
  echo "  " & pkg.dress(stLocalFS, params)
  echo "  " & pkg.mount()
  
  echo ""
  echo "── 反向寄生规则 ──"
  for rule in pkg.parasitic_rules():
    echo "  " & rule
  
  echo ""
  echo "── 盟机合 ──"
  echo "  主人: ", mengjihe_mode("owner")
  echo "  节点: ", mengjihe_mode("node")
  
  echo ""
  echo "═══ 蚕蛹壳就绪 · 可寄生到任意宿主系统 ═══"
