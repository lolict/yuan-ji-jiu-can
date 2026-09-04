# ============================================================
# 蚕蛹 FILE 壳 — 文件系统按满全法运作
#
# 核心哲学:
#   不让满全法适应文件系统树的结构
#   而是让文件路径/标记按满全法编码编码表运作
#
# 命令:
#   mqf-file write <mqf-path> <content>   创建文件 + mqf元数据
#   mqf-file read <mqf-path>              读文件
#   mqf-file seal <mqf-path>              封缄 (写保护 + 时间锚)
#   mqf-file tag <mqf-path> <mqf-tag>     打标记 (存入 .mqfmeta)
#   mqf-file list <mqf-prefix>            列文件按标记过滤
#   mqf-file metadata <mqf-path>          读 mqf 元
#
# 编码:
#   mqf 路径: mlll:docs:note → ./.mqf/mlll/docs/note.txt
#   元数据:   ./.mqfmeta/ 平行目录
# ============================================================

import os, osproc, strutils, json, times, sequtils

const
  REPO_ROOT = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
  MQFMETA_DIR = REPO_ROOT & "/.mqfmeta"

type
  FileShell* = ref object
    base_dir*: string
    op_count*: int
    last_op*: string
    last_result*: string

  MqfMeta* = object
    mqf_tag*: string
    author*: string
    created*: string
    sealed*: bool
    seal_time*: string
    container*: string      # 所在容器(254+2 箱号)

# ═══════════════════════════════════════════
# File 路径编码
# ═══════════════════════════════════════════
proc encode_file_path*(mqf_path: string): string =
  ## mlll:docs:note → ./.mqf/mlll/docs/note
  let mqf_path = mqf_path.replace(":", "/")
  result = MQFMETA_DIR & "/" & mqf_path

proc decode_file_path*(file_path: string): string =
  ## 反编码
  if file_path.startsWith(MQFMETA_DIR & "/"):
    result = file_path[MQFMETA_DIR.len + 1..^1].replace("/", ":")
  else:
    result = file_path

# ═══════════════════════════════════════════
# FileShell 引擎
# ═══════════════════════════════════════════
proc new_file_shell*(base_dir: string = ""): FileShell =
  result = FileShell(
    base_dir: if base_dir.len > 0: base_dir else: MQFMETA_DIR,
    op_count: 0
  )

proc write*(s: FileShell; mqf_path: string; content: string): bool =
  ## 写文件 + 存 mqf 元数据
  let fpath = encode_file_path(mqf_path)
  let parent = fpath.parentDir()
  if not dirExists(parent):
    createDir(parent)
  writeFile(fpath, content)
  s.op_count += 1
  s.last_op = "write"
  s.last_result = "ok: " & mqf_path & " (" & $content.len & " bytes)"
  return true

proc read*(s: FileShell; mqf_path: string): string =
  ## 读文件
  let fpath = encode_file_path(mqf_path)
  if not fileExists(fpath):
    s.last_result = "ERROR: 不存在: " & mqf_path
    return ""
  s.op_count += 1
  s.last_op = "read"
  s.last_result = "ok: " & mqf_path
  return readFile(fpath)

proc seal*(s: FileShell; mqf_path: string; container: string = ""): bool =
  ## 封缄: 移除写权限 + 记录时间锚
  let fpath = encode_file_path(mqf_path)
  if not fileExists(fpath):
    s.last_result = "ERROR: 不存在: " & mqf_path
    return false
  let now = $epochTime()
  let meta = MqfMeta(
    mqf_tag: mqf_path,
    author: "刘楚坦和我",
    created: now,
    sealed: true,
    seal_time: now,
    container: if container.len > 0: container else: "254+2-libre"
  )
  # 确保 meta 目录存在
  let mpath = fpath & ".mqfmeta"
  writeFile(mpath, $( %* meta ))
  # 移除写权限 (保留读)
  let cmd = "chmod 444 " & fpath.replace(" ", "\\ ")
  discard execCmdEx(cmd)
  s.op_count += 1
  s.last_op = "seal"
  s.last_result = "sealed: " & mqf_path & " @" & now
  return true

proc tag*(s: FileShell; mqf_path: string; tag: string): bool =
  ## 打 mqf 标记
  let fpath = encode_file_path(mqf_path)
  if not fileExists(fpath):
    s.last_result = "ERROR: 不存在: " & mqf_path
    return false
  let mpath = fpath & ".mqfmeta"
  var meta: MqfMeta
  if fileExists(mpath):
    try:
      let j = parseJson(readFile(mpath))
      meta = MqfMeta(
        mqf_tag: j["mqf_tag"].getStr(),
        author: j.getOrDefault("author").getStr(),
        created: j.getOrDefault("created").getStr(),
        sealed: j.getOrDefault("sealed").getBool(),
        seal_time: j.getOrDefault("seal_time").getStr(),
        container: j.getOrDefault("container").getStr()
      )
    except:
      meta = MqfMeta(mqf_tag: mqf_path)
  else:
    meta = MqfMeta(mqf_tag: mqf_path)
  meta.mqf_tag = tag
  writeFile(mpath, $( %* meta ))
  s.op_count += 1
  s.last_op = "tag"
  s.last_result = "tagged: " & mqf_path & " → " & tag
  return true

proc list*(s: FileShell; prefix: string = ""): seq[string] =
  ## 列文件
  let p = if prefix.len > 0: encode_file_path(prefix) else: MQFMETA_DIR
  if not dirExists(p):
    return
  for f in walkDirRec(p):
    if not f.endsWith(".mqfmeta"):
      result.add(decode_file_path(f))
  s.last_result = $result.len & " files"

proc metadata*(s: FileShell; mqf_path: string): MqfMeta =
  let fpath = encode_file_path(mqf_path)
  let mpath = fpath & ".mqfmeta"
  if fileExists(mpath):
    try:
      let j = parseJson(readFile(mpath))
      result = MqfMeta(
        mqf_tag: j["mqf_tag"].getStr(),
        author: j.getOrDefault("author").getStr(),
        created: j.getOrDefault("created").getStr(),
        sealed: j.getOrDefault("sealed").getBool(),
        seal_time: j.getOrDefault("seal_time").getStr(),
        container: j.getOrDefault("container").getStr()
      )
    except:
      discard
  s.op_count += 1
  s.last_op = "metadata"

proc status*(s: FileShell): string =
  result = "FILE壳 base=" & s.base_dir & " ops=" & $s.op_count
  s.last_result = result

proc report*(s: FileShell) =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║           蚕蛹 FILE 壳 · 状态                ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  根: " & s.base_dir
  echo "  ops: " & $s.op_count
  echo "  last_op: " & s.last_op
  echo "  last_result: " & s.last_result

when isMainModule:
  echo "file_shell.nim — 被 mqf-file 路由调用"
