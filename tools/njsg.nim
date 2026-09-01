# ============================================================
# njsg-swarm · 包入口 (API aggregator)
# 独立 nim check 用 when false 跳过，避免顶层路径找不到 import
# ============================================================

when false:
  import swarm/coordinator
  import agents/subagent

  export coordinator
  export subagent
