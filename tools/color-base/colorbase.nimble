# 颜色进制 — 相位进制画布工具包
# ColorBase: Phase-encoding canvas with add/sub only arithmetic

version       = "0.1.0"
author        = "hunyuan-os"
description   = "相位进制画布：离散相位单元、仅加减运算、禁用乘除、L1/L2/L3层级"
license       = "Private"

srcDir = "."

bin = @["cb_cli", "cb_repl"]

requires "nim >= 2.0"
