# ============================================================
# 蚕蛹 Git 壳 · 真实运行演示
#
# 在临时目录创建真实 Git 仓库，演示：
# 1. 满全法提交 → Git commit 编码转换
# 2. Git 历史 → 满全法流解码
# 3. 九元偏移世界 → Git 分支
# ============================================================

import os, osproc, strutils, times
import git_shell

# ═══════════════════════════════════════════
# 创建临时测试仓库
# ═══════════════════════════════════════════
const TEST_DIR = "/tmp/silkworm-git-test"

proc setup_test_repo() =
  # 清理旧目录
  if dirExists(TEST_DIR):
    removeDir(TEST_DIR)
  createDir(TEST_DIR)
  # 配置 git
  discard execCmdEx("git -C " & TEST_DIR & " init -b root-mlll")
  discard execCmdEx("git -C " & TEST_DIR & " config user.name 'mqf-dev'")
  discard execCmdEx("git -C " & TEST_DIR & " config user.email 'nil@mqf'")

# ═══════════════════════════════════════════
# 编码表验证
# ═══════════════════════════════════════════
proc test_encoding() =
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  [测试] 编码往返（满全法 → Git → 满全法）"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  let test_cases = @[
    "mlll root",
    "mltt naming",
    "gtt route",
    "mix mlll and gtt together"
  ]

  for tc in test_cases:
    let encoded = encode_mqf_to_git(tc)
    let decoded = decode_git_to_mqf(encoded)
    let ok = if decoded == tc: "✓" else: "✗"
    echo "  " & ok & " 输入: \"" & tc & "\""
    echo "      编码: " & encoded
    echo "      解码: " & decoded

# ═══════════════════════════════════════════
# 真实 Git 操作演示
# ═══════════════════════════════════════════
proc test_real_git() =
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  [测试] 真实 Git 仓库操作"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  setup_test_repo()
  let shell = new_git_shell(TEST_DIR)

  echo ""
  echo "  仓库路径: " & TEST_DIR
  echo "  根基分支: root-mlll"
  echo ""

  # 制造一系列满全法提交
  let commits = @[
    ("mlll init container identity", "mqf-dev"),
    ("mltt assign naming table", "mqf-dev"),
    ("gtt establish xi-yao protocol", "lolict"),
    ("wgslz activate smrl engine", "mqf-dev"),
    ("njsg mutual symbiosis active", "lolict"),
  ]

  echo "  [提交序列]"
  for (msg, author) in commits:
    let hash_val = shell.commit(msg, author)
    if hash_val.len > 0:
      echo "    → [" & hash_val & "] " & msg

  # 创建偏移世界分支
  echo ""
  echo "  [偏移世界分支]"
  let offsets = [0.0, 0.1, 0.2, 0.3]
  for off in offsets:
    discard shell.create_phase_branch("phase-" & $(int(off * 10)), off)

  # 报告
  echo ""
  report_git(shell)

  # 列出实际 Git 分支
  echo "  [Git 分支列表]"
  let (bout, ec) = execCmdEx("git -C " & TEST_DIR & " branch -a")
  if ec == 0:
    for line in bout.splitLines():
      echo "    " & line.strip()

  echo ""
  echo "  [Git Log]"
  let (lout, ec2) = execCmdEx("git -C " & TEST_DIR & " log --oneline --graph --all")
  if ec2 == 0:
    for line in lout.splitLines():
      echo "    " & line

# ═══════════════════════════════════════════
# 清场
# ═══════════════════════════════════════════
proc cleanup() =
  if dirExists(TEST_DIR):
    removeDir(TEST_DIR)
    echo ""
    echo "  [清场] 临时仓库已移除: " & TEST_DIR

# ═══════════════════════════════════════════
# 主入口
# ═══════════════════════════════════════════
proc main() =
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║         蚕蛹 Git 壳 · 真实运行演示               ║"
  echo "║  让真实 Git 仓库按满全法运作                      ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo ""

  test_encoding()
  test_real_git()
  cleanup()

  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║     蚕蛹 Git 壳 · 演示完成                       ║"
  echo "║  外部 Git 已按满全法编码运作                      ║"
  echo "╚══════════════════════════════════════════════════════╝"

when isMainModule:
  main()
