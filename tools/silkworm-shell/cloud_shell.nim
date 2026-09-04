# ============================================================
# 蚕蛹 CLOUD 壳 — 云资源按满全法运作
#
# 核心哲学:
#   不让满全法适应云厂商 API 语义
#   而是让云存储/云函数按满全法编码表运作
#
# 命令:
#   mqf-cloud put <local> <mqf-remote>   上传并编码路径
#   mqf-cloud get <mqf-remote> <local>   下载并解码路径
#   mqf-cloud list <mqf-prefix>          列出云资源(前缀编码)
#   mqf-cloud sync <dir>                 双向同步
#   mqf-cloud status                     云状态报告
#
# 编码:
#   本地路径: /home/user/file.txt
#   云路径:   cloud://mlll/ROOT/file.txt → /v1/mlll/ROOT/file.txt
# ============================================================

import os, osproc, strutils, json, times

const
  REPO_ROOT = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
  CLOUD_DIR = REPO_ROOT & "/.njsg-swarm/cloud"

type
  CloudShell* = ref object
    endpoint*: string       # 盟机场端点
    bucket*: string         # 默认容器
    last_op*: string
    last_result*: string
    op_count*: int

  CloudObject* = object
    name*: string
    size*: int
    mtime*: string
    mqf_tag*: string

# ═══════════════════════════════════════════
# Cloud 路径编码: 满全法 → cloud:// URI → HTTP 路径
# ═══════════════════════════════════════════
proc encode_cloud_path*(mqf_path: string): string =
  if mqf_path.startsWith("cloud://"):
    result = "/" & mqf_path[8..^1]
  elif mqf_path.startsWith("/v1/"):
    result = mqf_path
  elif mqf_path.startsWith("/"):
    result = "/v1" & mqf_path
  else:
    result = "/v1/" & mqf_path

proc decode_cloud_path*(cloud_path: string): string =
  if cloud_path.startsWith("/v1/"):
    result = cloud_path[4..^1]
  else:
    result = cloud_path

# ═══════════════════════════════════════════
# CloudShell 引擎 (本地云: 用磁盘模拟)
# ═══════════════════════════════════════════
proc new_cloud_shell*(endpoint: string = ""): CloudShell =
  new(result)
  result.endpoint = if endpoint.len > 0: endpoint else: "file://" & CLOUD_DIR
  result.bucket = "mlll"
  result.op_count = 0

proc put*(s: CloudShell; local_path: string; remote_mqf: string): bool =
  ## 上传: 本地文件 → 云路径 (编码存储)
  let dst_path = CLOUD_DIR & encode_cloud_path(remote_mqf)
  let dst_dir = dst_path.parentDir()
  if not dirExists(dst_dir):
    createDir(dst_dir)
  if not fileExists(local_path):
    s.last_result = "ERROR: 本地文件不存在: " & local_path
    return false
  copyFile(local_path, dst_path)
  let meta = CLOUD_DIR & "/.meta/" & remote_mqf.encode_cloud_path() & ".json"
  if not dirExists(meta.parentDir()):
    createDir(meta.parentDir())
  let j = %* { "src": local_path, "dst": remote_mqf,
              "ts": $epochTime(), "size": getFileSize(local_path),
              "mqf": "mlll:cloud:" & remote_mqf }
  writeFile(meta, $j)
  s.op_count += 1
  s.last_op = "put"
  s.last_result = "ok: " & local_path & " → " & remote_mqf
  return true

proc get*(s: CloudShell; remote_mqf: string; local_path: string): bool =
  ## 下载: 云路径 → 本地文件
  let src_path = CLOUD_DIR & encode_cloud_path(remote_mqf)
  if not fileExists(src_path):
    s.last_result = "ERROR: 云文件不存在: " & remote_mqf
    return false
  let dst_dir = local_path.parentDir()
  if not dirExists(dst_dir):
    createDir(dst_dir)
  copyFile(src_path, local_path)
  s.op_count += 1
  s.last_op = "get"
  s.last_result = "ok: " & remote_mqf & " → " & local_path
  return true

proc list*(s: CloudShell; prefix_mqf: string = ""): seq[CloudObject] =
  ## 列出云对象 (前缀过滤)
  let prefix_path = CLOUD_DIR & encode_cloud_path(prefix_mqf)
  if not dirExists(prefix_path):
    s.last_result = "ERROR: 前缀不存在: " & prefix_mqf
    return
  for kind, f in walkDir(prefix_path):
    if kind == pcFile and not f.endsWith(".json"):
      result.add(CloudObject(
        name: f.extractFilename(),
        size: getFileSize(f),
        mtime: $getLastModificationTime(f).toUnix(),
        mqf_tag: "mqf:" & f.replace(CLOUD_DIR & "/", "")
      ))
  s.last_result = "ok: " & $result.len & " 对象"

proc sync*(s: CloudShell; local_dir: string): tuple[uploaded: int; downloaded: int] =
  ## 双向同步
  if not dirExists(local_dir):
    s.last_result = "ERROR: 本地目录不存在: " & local_dir
    return (0, 0)
  if not dirExists(CLOUD_DIR):
    createDir(CLOUD_DIR)
  # 上传缺失文件
  for kind, lf in walkDir(local_dir):
    if kind == pcFile:
      let rel = lf.replace(local_dir & "/", "")
      let cloud_f = CLOUD_DIR & "/" & rel
      if not fileExists(cloud_f):
        discard s.put(lf, rel)
        result.uploaded += 1
      elif getFileSize(lf) != getFileSize(cloud_f):
        discard s.put(lf, rel)
        result.uploaded += 1
  # 下载缺失文件
  for kind, cf in walkDir(CLOUD_DIR):
    if kind == pcFile and not cf.endsWith(".json"):
      let rel = cf.replace(CLOUD_DIR & "/", "")
      let local_f = local_dir & "/" & rel
      if not fileExists(local_f):
        discard s.get(rel, local_f)
        result.downloaded += 1
  s.last_result = "sync: ↑" & $result.uploaded & " ↓" & $result.downloaded

proc status*(s: CloudShell): string =
  result = "CLOUD壳 " & s.endpoint & " 容器:" & s.bucket
  result &= " ops:" & $s.op_count
  if dirExists(CLOUD_DIR):
    var file_count = 0
    for f in walkFiles(CLOUD_DIR & "/*"):
      if not f.endsWith(".json"):
        file_count += 1
    result &= " files:" & $file_count
  else:
    result &= " (未初始化)"

proc report*(s: CloudShell) =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║           蚕蛹 CLOUD 壳 · 状态               ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  " & s.status()
  echo "  最近操作: " & s.last_op
  echo "  最近结果: " & s.last_result

when isMainModule:
  echo "cloud_shell.nim — 被 mqf-cloud 路由调用"
