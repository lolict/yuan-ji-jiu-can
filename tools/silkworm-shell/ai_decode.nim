# 蚕蛹 AI 解码器 - 独立模块，防止双重替换

import strutils

proc decode_llm_response*(raw: string): string =
  ## 使用占位符策略防止双重替换
  ## 阶段1: 所有替换目标 → 占位符
  ## 阶段2: 占位符 → 编码
  let PH = '\x01'
  result = raw

  # 替换对列表 (pattern, mqf)
  const
    P0 = "MUTUAL"
    M0 = "njsg"
    P1 = "共生"
    M1 = "njsg"
    P2 = "ROOT"
    M2 = "mlll"
    P3 = "根基"
    M3 = "mlll"
    P4 = "UNIT"
    M4 = "mlnn"
    P5 = "袋子"
    M5 = "mlnn"
    P6 = "NAME"
    M6 = "mltt"
    P7 = "命名"
    M7 = "mltt"
    P8 = "ROUTE"
    M8 = "gtt"
    P9 = "路由"
    M9 = "gtt"
    P10 = "ENGINE"
    M10 = "wgslz"
    P11 = "引擎"
    M11 = "wgslz"
    P12 = "SPACE"
    M12 = "mlgy"
    P13 = "天庭"
    M13 = "mlgy"
    P14 = "SACRIFICE"
    M14 = "lctfrq"
    P15 = "牺牲"
    M15 = "lctfrq"
    P16 = "PRELOAD"
    M16 = "lctqf"
    P17 = "预加载"
    M17 = "lctqf"
    P18 = "BINARY"
    M18 = "mlnnn"
    P19 = "仅加减"
    M19 = "mlnnn"

  # 阶段1: 替换为占位符
  result = result.replace(P0) & PH & "0" & PH
  result = result.replace(P0, PH & "0" & PH)
  result = result.replace(P1, PH & "1" & PH)
  result = result.replace(P2, PH & "2" & PH)
  result = result.replace(P3, PH & "3" & PH)
  result = result.replace(P4, PH & "4" & PH)
  result = result.replace(P5, PH & "5" & PH)
  result = result.replace(P6, PH & "6" & PH)
  result = result.replace(P7, PH & "7" & PH)
  result = result.replace(P8, PH & "8" & PH)
  result = result.replace(P9, PH & "9" & PH)
  result = result.replace(P10, PH & "10" & PH)
  result = result.replace(P11, PH & "11" & PH)
  result = result.replace(P12, PH & "12" & PH)
  result = result.replace(P13, PH & "13" & PH)
  result = result.replace(P14, PH & "14" & PH)
  result = result.replace(P15, PH & "15" & PH)
  result = result.replace(P16, PH & "16" & PH)
  result = result.replace(P17, PH & "17" & PH)
  result = result.replace(P18, PH & "18" & PH)
  result = result.replace(P19, PH & "19" & PH)

  # 阶段2: 占位符 → 编码
  result = result.replace(PH & "0" & PH, M0)
  result = result.replace(PH & "1" & PH, M1)
  result = result.replace(PH & "2" & PH, M2)
  result = result.replace(PH & "3" & PH, M3)
  result = result.replace(PH & "4" & PH, M4)
  result = result.replace(PH & "5" & PH, M5)
  result = result.replace(PH & "6" & PH, M6)
  result = result.replace(PH & "7" & PH, M7)
  result = result.replace(PH & "8" & PH, M8)
  result = result.replace(PH & "9" & PH, M9)
  result = result.replace(PH & "10" & PH, M10)
  result = result.replace(PH & "11" & PH, M11)
  result = result.replace(PH & "12" & PH, M12)
  result = result.replace(PH & "13" & PH, M13)
  result = result.replace(PH & "14" & PH, M14)
  result = result.replace(PH & "15" & PH, M15)
  result = result.replace(PH & "16" & PH, M16)
  result = result.replace(PH & "17" & PH, M17)
  result = result.replace(PH & "18" & PH, M18)
  result = result.replace(PH & "19" & PH, M19)
