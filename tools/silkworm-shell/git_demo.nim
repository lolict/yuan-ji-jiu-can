# ============================================================
# 蚕蛹 Git 壳 · 真实运行演示
#
# 在临时目录创建真实 Git 仓库，演示：
# 1. 满全法提交 → Git commit 编码转换
# 2. Git 历史 → 满全法流解码
# 3. 九元偏移世界 → Git 分支
# ============================================================

import os, osproc, strutils
import git_shell

const TEST_DIR = "/tmp/silkworm-git-test"

proc setup_test_repo() =
  if dirExists(TEST_DIR):
    removeDir(TEST_DIR)
  createDir(TEST_DIR)
  discard execProcess("git", args=["-C", TEST_DIR, "init", "-b", "root-mlll"],
                     options={poUsePath})
  discard execProcess("git", args=["-C", TEST_DIR, "config", "user.name", "mqf-dev"],
                     options={poUsePath})
  discard execProcess("git", args=["-C", TEST_DIR, "config", "user.email", "nil@mqf"],
                     options={poUsePath})

proc test_encoding() =
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  [测试] 编码往返（满全法 → Git → 满全法）"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  let test_cases = @["mlll", "mltt", "gtt", "wgslz", "njsg",
                     "mlnnn", "mlgy", "lctqf"]
  for tc in test_cases:
    let encoded = encode_mqf_to_git(tc)
    let decoded = decode_git_to_mqf(encoded)
    let ok = if decoded == tc: "✓" else: "✗"
    echo "  " & ok & " " & tc & " → " & encoded & " → " & decoded

proc test_all_phase_branches(shell: GitShell) =
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  [测试] 九元偏移世界 → 分支"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  var off = 0.0
  while off < 0.9:
    discard shell.create_phase_branch("phase-" & $(int(off * 10)), off)
    off += 0.1

proc test_real_git() =
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  [测试] 真实 Git 仓库操作"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  setup_test_repo()
  let shell = new_git_shell(TEST_DIR)
  echo "  仓库路径: " & TEST_DIR
  echo ""

  let commits = @[
    ("mlll", "root"),
    ("mltt", "naming"),
    ("gtt", "route"),
    ("wgslz", "engine"),
    ("njsg", "mutual"),
  ]

  echo "  [提交序列]"
  for (sym, author) in commits:
    let h = shell.commit(sym, author)
    if h.len > 0:
      echo "    " & sym & " → " & h

  test_all_phase_branches(shell)
  report_git(shell)

  # 分支列表
  echo ""
  echo "  [Git 分支]"
  let (bout, ec) = execCmdEx("git -C " & TEST_DIR & " branch -a")
  if ec == 0:
    for line in bout.splitLines():
      echo "    " & line.strip()
  # log
  echo ""
  echo "  [Git Log]"
  let (lout, ec2) = execCmdEx("git -C " & TEST_DIR & " log --oneline --graph --all")
  if ec2 == 0:
    for line in lout.splitLines():
      echo "    " & line

proc cleanup() =
  if dirExists(TEST_DIR):
    removeDir(TEST_DIR)
    echo ""
    echo "  [清场] 临时仓库移除"

proc main() =
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║         蚕蛹 Git 壳 · 真实运行演示               ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo ""
  test_encoding()
  test_real_git()
  cleanup()
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║     蚕蛹 Git 壳 · 演示完成                       ║"
  echo "╚══════════════════════════════════════════════════════╝"

when isMainModule:
  main()
