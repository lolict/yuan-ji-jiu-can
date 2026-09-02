# ============================================================
# 蚕蛹 Git 壳 — 真实可运行
#
# 让外部 Git 仓库按满全法运作：
# - 满全法提交 → Git commit message（编码后）
# - Git 提交历史 → 满全法符号流（解码）
# - 分支 = 偏移世界（九元层级）
# - 远程仓库 = 盟机场
# ============================================================

import os, osproc, strutils, json, times, tables, sequtils

# ═══════════════════════════════════════════
# 编码表：满全法 ↔ Git 的双射
# ═══════════════════════════════════════════
const
  GIT_MAGIC_HDR = "[MQF]"       # 满全法 Git 提交标记
  ENCODINGTable: array[10, tuple[mqf: string, git: string]] = [
    ("mlll",  "ROOT"),     # 根基
    ("mlnn",  "UNIT"),     # 袋子
    ("mlnnn", "BINARY"),   # 仅加减
    ("mltt",  "NAME"),     # 命名
    ("gtt",   "ROUTE"),    # 夕瑶协议
    ("wgslz", "ENGINE"),   # SMRL引擎
    ("mlgy",  "SPACE"),    # 天庭
    ("lctqf", "PRELOAD"),  # 预加载
    ("lctfrq","SACRIFICE"),# 牺牲
    ("njsg",  "MUTUAL"),   # 共生
  ]

proc encode_mqf_to_git(mqf_text: string): string =
  result = GIT_MAGIC_HDR & " "
  var i = 0
  while i < mqf_text.len:
    var matched = false
    for pair in ENCODINGTable:
      if mqf_text.continuesWith(pair.mqf, i):
        result.add(pair.git & " ")
        i += pair.mqf.len
        matched = true
        break
    if not matched:
      result.add(mqf_text[i])
      i += 1

proc decode_git_to_mqf(git_text: string): string =
  if GIT_MAGIC_HDR notin git_text:
    return git_text
  result = ""
  let parts = git_text.split(' ')
  var in_header = false
  for part in parts:
    if part == GIT_MAGIC_HDR:
      in_header = true
      continue
    if in_header:
      var decoded = part
      for pair in ENCODINGTable:
        if part == pair.git:
          decoded = pair.mqf
          break
      result.add(decoded)
    else:
      result.add(part)

# ═══════════════════════════════════════════
# 蚕蛹 Git 壳引擎
# ═══════════════════════════════════════════
type
  GitShell* = ref object
    repo_path: string
    shell_name: string
    adaptation_count: int

  GitCommit* = object
    hash: string
    author: string
    mqf_content: string   # 解码后的满全法
    git_message: string   # 原始 Git message
    timestamp: string
    branch: string

  BranchMapping* = object
    mqf_phase: string     # 九元偏移世界名
    git_branch_name: string
    offset_rate: float    # (n-1)*0.1

# ═══════════════════════════════════════════
# 操作封装
# ═══════════════════════════════════════════
proc new_git_shell*(repo_path: string): GitShell =
  new(result)
  result.repo_path = repo_path
  result.shell_name = "git-shell"
  result.adaptation_count = 0

proc run_git(s: GitShell; args: seq[string]): tuple[exitcode: int, output: string] =
  let cmd = "git -C " & s.repo_path & " " & args.join(" ")
  let (output, exitcode) = execCmdEx(cmd)
  return (exitcode, output.strip())

proc init_repo*(s: GitShell): bool =
  ## 初始化仓库（如果没有）
  if not dirExists(s.repo_path / ".git"):
    let (ec, _) = s.run_git(@["init", "-b", "root-mlll"])
    if ec != 0:
      return false
    # 根基提交
    let base_msg = encode_mqf_to_git("mlll root commit")
    echo "  [蚕蛹-Git] 根基提交: ", base_msg
    let (ec2, _) = s.run_git(@["commit", "--allow-empty", "-m", base_msg])
    return ec2 == 0
  return true

proc commit*(s: GitShell; mqf_text: string; author: string): string =
  ## 满全法提交 → Git 提交
  let encoded = encode_mqf_to_git(mqf_text)
  let (ec, gout) = s.run_git(@["commit", "--allow-empty", "-m", encoded,
                                "--author", author & " <nil@mqf>"])
  if ec == 0:
    s.adaptation_count += 1
    # 获取最新 hash
    let (ec2, hash_out) = s.run_git(@["rev-parse", "HEAD"])
    let hash_val = if ec2 == 0: hash_out[0..7] else: "unknown"
    echo "  [蚕蛹-Git] 满全法→Git: ", mqf_text, " → [", hash_val, "]"
    return hash_val
  else:
    echo "  [蚕蛹-Git] 提交失败: ", gout
    return ""

proc read_log*(s: GitShell): seq[GitCommit] =
  ## 读取提交历史，解码为满全法流
  let (ec, glog) = s.run_git(@[
    "log", "--format=§%H§%an§%ai§%s", "--reverse"
  ])
  if ec != 0:
    return @[]

  let lines = glog.splitLines()
  for line in lines:
    if line.len < 5: continue
    let parts = line.split("§")
    if parts.len < 4: continue
    let hash_val = parts[0]
    let author = parts[1]
    let ts = parts[2]
    let git_msg = parts[3]
    let mqf = decode_git_to_mqf(git_msg)
    result.add(GitCommit(
      hash: hash_val[0..7],
      author: author,
      mqf_content: mqf,
      git_message: git_msg,
      timestamp: ts,
      branch: ""
    ))

# ═══════════════════════════════════════════
# 九元偏移世界 → Git 分支映射
# ═══════════════════════════════════════════
proc create_phase_branch*(s: GitShell; phase_name: string; offset: float): string =
  ## 在特定偏移世界创建分支
  let formatted = "phase-" & $(int(offset * 10.0))
  let (ec, _) = s.run_git(@["branch", formatted])
  if ec == 0:
    echo "  [蚕蛹-Git] 偏移世界创建: " & formatted & " (偏移率=" & $offset & ")"
    return formatted
  return ""

# ═══════════════════════════════════════════
# 盟机场适配（远程仓库）
# ═══════════════════════════════════════════
proc adapt_mengjichang*(s: GitShell; remote_url: string; remote_name: string = "mqf-mengjichang"): bool =
  ## 适配盟机场（远程仓库作为盟机场）：外部系统按满全法运作
  # 检查远端是否已存在
  let (ec, _) = s.run_git(@["remote", "add", remote_name, remote_url])
  if ec != 0:
    # 可能已存在，尝试 set-url
    let (ec2, out2) = s.run_git(@["remote", "set-url", remote_name, remote_url])
    if ec2 != 0:
      echo "  [蚕蛹-Git] 盟机场适配失败: ", out2
      return false

  echo "  [蚕蛹-Git] 盟机场已适配: " & remote_name  & " → " & remote_url
  echo "  [蚕蛹-Git] 远端将按满全法编码规范运作"
  s.adaptation_count += 1
  return true

proc push_to_mengjichang*(s: GitShell; remote_name: string = "mqf-mengjichang"; branch: string = "root-mlll"): tuple[ok: bool, msg: string] =
  ## 推送到盟机场（远程同步）
  let (ec, gout) = s.run_git(@["push", remote_name, branch])
  if ec == 0:
    echo "  [蚕蛹-Git] 盟机场同步成功: " & remote_name
    return (true, gout)
  else:
    echo "  [蚕蛹-Git] 盟机场同步提示: " & gout
    return (false, gout)

# ═══════════════════════════════════════════
# 茧状态报告
# ═══════════════════════════════════════════
proc report*(s: GitShell): string =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║          蚕蛹 Git 壳 · 状态报告              ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  仓库: " & s.repo_path
  echo "  壳名: " & s.shell_name
  echo "  适配次数: " & $s.adaptation_count

  echo ""
  echo "  [编码表]"
  for pair in ENCODINGTable:
    echo "    ", pair.mqf, " ↔ ", pair.git

  echo ""
  echo "  [提交历史]（满全法流）"
  let commits = s.read_log()
  for c in commits:
    echo "    [" & c.hash & "] " & c.author & " · " & c.timestamp
    echo "      Git: " & c.git_message
    echo "      解码: " & c.mqf_content

  if commits.len == 0:
    echo "    （无提交）"

  echo ""
  echo "[EOF]"

when isMainModule:
  echo "git_shell.nim — 蚕蛹 Git 壳 真实实现"
  echo "被 import 使用，非独立运行"
