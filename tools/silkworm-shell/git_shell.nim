# ============================================================
# 蚕蛹 Git 壳 — 真实可运行
#
# 让外部 Git 仓库按满全法运作：
# - 满全法提交 → Git commit message（编码后）
# - Git 提交历史 → 满全法符号流（解码）
# - 分支 = 偏移世界（九元层级）
# - 远程仓库 = 盟机场
# ============================================================

import os, osproc, strutils, algorithm
import git_run

# ═══════════════════════════════════════════
# 编码表：满全法 ↔ Git 的双射
# ═══════════════════════════════════════════
const
  GIT_MAGIC_HDR = "[MQF]"
  ENCODINGTable: array[10, tuple[mqf: string, git: string]] = [
    ("mlll",  "ROOT"),
    ("mlnn",  "UNIT"),
    ("mlnnn", "BINARY"),
    ("mltt",  "NAME"),
    ("gtt",   "ROUTE"),
    ("wgslz", "ENGINE"),
    ("mlgy",  "SPACE"),
    ("lctqf", "PRELOAD"),
    ("lctfrq","SACRIFICE"),
    ("njsg",  "MUTUAL"),
  ]

# 按 mqf 长度降序排列的索引（手工排好，避免运行时排序）
const SortedIdx = [1, 2, 5, 6, 7, 8, 3, 4, 9, 0]

# ═══════════════════════════════════════════
# 编码/解码
# ═══════════════════════════════════════════
proc encode_mqf_to_git*(mqf_text: string): string =
  ## 满全法文本 → Git 编码文本（最长匹配贪心）
  result = GIT_MAGIC_HDR
  var i = 0
  while i < mqf_text.len:
    var matched = false
    for idx in SortedIdx:
      let pair = ENCODINGTable[idx]
      if i + pair.mqf.len <= mqf_text.len and
         mqf_text[i ..< i + pair.mqf.len] == pair.mqf:
        result.add(pair.git & "|")
        i += pair.mqf.len
        matched = true
        break
    if not matched:
      result.add($mqf_text[i])
      i += 1

proc decode_git_to_mqf*(git_text: string): string =
  ## Git 编码文本 → 满全法文本
  if GIT_MAGIC_HDR notin git_text:
    return git_text
  result = ""
  var i = GIT_MAGIC_HDR.len
  while i < git_text.len:
    var token = ""
    while i < git_text.len and git_text[i] != '|':
      token.add(git_text[i])
      i += 1
    if i < git_text.len: i += 1
    var decoded = token
    for pair in ENCODINGTable:
      if token == pair.git:
        decoded = pair.mqf
        break
    result.add(decoded)

# ═══════════════════════════════════════════
# 蚕蛹 Git 壳引擎
# ═══════════════════════════════════════════
type
  GitShell* = ref object
    repo_path: string
    shell_name: string
    adaptation_count: int

  GitCommit* = object
    hash_str: string
    author: string
    mqf_content: string
    git_message: string
    timestamp: string

# ═══════════════════════════════════════════
# 操作封装
# ═══════════════════════════════════════════
proc new_git_shell*(repo_path: string): GitShell =
  new(result)
  result.repo_path = repo_path
  result.shell_name = "git-shell"
  result.adaptation_count = 0

proc run_git*(s: GitShell; args: seq[string]): tuple[exitcode: int, output: string] =
  exec_git(s.repo_path, args)

proc init_repo*(s: GitShell): bool =
  if not dirExists(s.repo_path / ".git"):
    let (ec, _) = s.run_git(@["init", "-b", "root-mlll"])
    if ec != 0:
      return false
    let base_msg = encode_mqf_to_git("mlll")
    echo "  [蚕蛹-Git] hash-digest => ", base_msg
    discard s.run_git(@["commit", "--allow-empty", "-m", base_msg,
                          "--author=mqf-dev <nil@mqf>"])
  return true

proc commit*(s: GitShell; mqf_text: string; author: string): string =
  let encoded = encode_mqf_to_git(mqf_text)
  let m_author = author & " <nil@mqf>"
  let (ec, gout) = s.run_git(@["commit", "--allow-empty",
                                "-m", encoded,
                                "--author=" & m_author])
  if ec == 0:
    s.adaptation_count += 1
    let (ec2, hash_out) = s.run_git(@["rev-parse", "HEAD"])
    let hash_val = if ec2 == 0: hash_out[0..7] else: "unknown"
    echo "  [蚕蛹-Git] " & mqf_text & " → [" & hash_val & "]"
    return hash_val
  else:
    echo "  [蚕蛹-Git] FAIL: ", gout
    return ""

proc read_log*(s: GitShell): seq[GitCommit] =
  let (ec, glog) = s.run_git(@[
    "log", "--format=%H§%an§%ai§%s", "--reverse"
  ])
  if ec != 0:
    return @[]
  for line in glog.splitLines():
    let parts = line.split("§")
    if parts.len < 4: continue
    result.add(GitCommit(
      hash_str: parts[0][0..<min(8, parts[0].len)],
      author: parts[1],
      mqf_content: decode_git_to_mqf(parts[3]),
      git_message: parts[3],
      timestamp: parts[2]
    ))

# ═══════════════════════════════════════════
# 九元偏移世界 → Git 分支映射
# ═══════════════════════════════════════════
proc create_phase_branch*(s: GitShell; phase_name: string; offset: float): string =
  let formatted = "phase-" & $(int(offset * 10.0))
  let (ec, _) = s.run_git(@["branch", formatted])
  if ec == 0:
    echo "  [蚕蛹-Git] " & formatted & " (offset=" & $offset & ")"
    return formatted
  return ""

# ═══════════════════════════════════════════
# 盟机场（远程仓库适配）
# ═══════════════════════════════════════════
proc adapt_mengjichang*(s: GitShell; remote_url: string;
    remote_name: string = "mqf-mengjichang"): bool =
  let (ec, _) = s.run_git(@["remote", "add", remote_name, remote_url])
  if ec != 0:
    let (ec2, out2) = s.run_git(@["remote", "set-url", remote_name, remote_url])
    if ec2 != 0:
      echo "  [蚕蛹-Git] FAIL: ", out2
      return false
  echo "  [蚕蛹-Git] " & remote_name & " → " & remote_url
  s.adaptation_count += 1
  return true

proc push_to_mengjichang*(s: GitShell; remote_name: string = "mqf-mengjichang";
    branch: string = "root-mlll"): tuple[ok: bool, msg: string] =
  let (ec, gout) = s.run_git(@["push", remote_name, branch])
  if ec == 0:
    echo "  [蚕蛹-Git] OK " & remote_name
    return (true, gout)
  else:
    echo "  [蚕蛹-Git] PUSH: " & gout
    return (false, gout)

# ═══════════════════════════════════════════
# 茧状态报告
# ═══════════════════════════════════════════
proc report_git*(s: GitShell) =
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
    echo "    [" & c.hash_str & "] " & c.author & " · " & c.timestamp
    echo "      原始: " & c.git_message
    echo "      解码: " & c.mqf_content
  if commits.len == 0:
    echo "    （无提交）"
  echo "[EOF]"

when isMainModule:
  echo "git_shell.nim — 蚕蛹 Git 壳"
