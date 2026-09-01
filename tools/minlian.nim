# ============================================================
# 民联浏览器 · 包入口 (API aggregator)
# 独立 nim check 用 when false 跳过，避免顶层路径找不到 import
# ============================================================

when false:
  import browser/types
  import browser/distance
  import browser/cluster
  import browser/wraptree

  export types
  export distance
  export cluster
  export wraptree
