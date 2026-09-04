# ============================================================
# mqf-cloud CLI — 蚕蛹 CLOUD 壳入口
# ============================================================

import os, strutils
import cloud_shell

let args = commandLineParams()
if args.len == 0:
  echo "mqf-cloud — 蚕蛹 CLOUD 壳"
  echo "用法: mqf-cloud <put|get|list|sync|status|demo> [args]"
  quit(0)

let cmd = args[0]
let rest = if args.len > 1: args[1..^1] else: @[]

var cloud = new_cloud_shell()

case cmd
of "put":
  if rest.len < 2:
    echo "用法: mqf-cloud put <local-path> <mqf-remote>"
    quit(1)
  discard cloud.put(rest[0], rest[1])
  echo cloud.last_result

of "get":
  if rest.len < 2:
    echo "用法: mqf-cloud get <mqf-remote> <local-path>"
    quit(1)
  discard cloud.get(rest[0], rest[1])
  echo cloud.last_result

of "list":
  let prefix = if rest.len > 0: rest[0] else: ""
  let objs = cloud.list(prefix)
  echo "CLOUD 对象 (prefix=" & prefix & "):"
  if objs.len == 0:
    echo "  (空)"
  for o in objs:
    echo "  " & o.name & " (" & $o.size & "B) tag=" & o.mqf_tag

of "sync":
  if rest.len < 1:
    echo "用法: mqf-cloud sync <local-dir>"
    quit(1)
  let r = cloud.sync(rest[0])
  echo "sync: uploaded=" & $r.uploaded & " downloaded=" & $r.downloaded

of "status":
  echo cloud.status()

of "demo":
  echo ""
  echo "蚕蛹 CLOUD 壳 — demo"
  echo ""
  # 本地云路径演示
  const CLOUD_DIR = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/.njsg-swarm/cloud"
  if not dirExists(CLOUD_DIR):
    createDir(CLOUD_DIR)
  # put
  let test_src = CLOUD_DIR & "/_test_src.txt"
  writeFile(test_src, "mlll:cloud-test:hello from mqf-cloud")
  discard cloud.put(test_src, "mlll:test:hello")
  echo "  put: → " & cloud.last_result
  # list
  discard cloud.list("mlll:test")
  echo "  list: " & cloud.last_result
  # status
  echo "  status: " & cloud.status()
  # cleanup
  discard tryRemoveFile(test_src)

  echo ""
  echo "[CLOUD壳] 本地云存储已就绪"
  echo "  路径: cloud://path → /v1/path (put/get/list/sync)"
  echo "  根基: mlll (ROOT) 为默认容器"

else:
  echo "未知命令: " & cmd
  echo "可用: put / get / list / sync / status / demo"
