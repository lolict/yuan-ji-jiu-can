# ============================================================
# mqf-file CLI — 蚕蛹 FILE 壳入口
# ============================================================

import os, strutils, json
import file_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-file — 蚕蛹 FILE 壳"
  echo "用法: mqf-file <write|read|seal|tag|list|metadata|demo> [args]"
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

var fs = new_file_shell()

case cmd
of "write":
  if rest.len < 2:
    echo "用法: mqf-file write <mqf-path> <content>"
    quit(1)
  let mqf_path = rest[0]
  let content = rest[1..^1].join(" ")
  discard fs.write(mqf_path, content)
  echo fs.last_result

of "read":
  if rest.len < 1:
    echo "用法: mqf-file read <mqf-path>"
    quit(1)
  let content = fs.read(rest[0])
  if content.len > 0:
    echo content
  else:
    echo fs.last_result

of "seal":
  if rest.len < 1:
    echo "用法: mqf-file seal <mqf-path> [container]"
    quit(1)
  let container = if rest.len > 1: rest[1] else: "254+2-libre"
  discard fs.seal(rest[0], container)
  echo fs.last_result

of "tag":
  if rest.len < 2:
    echo "用法: mqf-file tag <mqf-path> <mqf-tag>"
    quit(1)
  discard fs.tag(rest[0], rest[1])
  echo fs.last_result

of "list":
  let prefix = if rest.len > 0: rest[0] else: ""
  let files = fs.list(prefix)
  echo "FILE 对象 (prefix=" & prefix & "):"
  if files.len == 0:
    echo "  (空)"
  for f in files:
    echo "  " & f

of "metadata":
  if rest.len < 1:
    echo "用法: mqf-file metadata <mqf-path>"
    quit(1)
  let m = fs.metadata(rest[0])
  if m.mqf_tag.len > 0:
    echo "mqf_tag: " & m.mqf_tag
    echo "author:  " & m.author
    echo "created: " & m.created
    echo "sealed:  " & $m.sealed
    echo "seal_at: " & m.seal_time
    echo "container: " & m.container

of "demo":
  echo ""
  echo "蚕蛹 FILE 壳 — demo"
  echo ""
  discard fs.write("mlll:demo:note", "mlll根基 · 路径编码测试")
  echo "  write: " & fs.last_result
  let c = fs.read("mlll:demo:note")
  echo "  read: " & c
  discard fs.tag("mlll:demo:note", "mlll:tag:v1")
  echo "  tag: " & fs.last_result
  discard fs.seal("mlll:demo:note")
  echo "  seal: " & fs.last_result
  echo ""
  echo "[FILE壳] 文件系统路径已就绪"
  echo "  路径: mqf:a:b → ./.mqfmeta/a/b"
  echo "  封缄: chmod 444 + 时间锚 + container"

else:
  echo "未知命令: " & cmd
  echo "可用: write / read / seal / tag / list / metadata / demo"
