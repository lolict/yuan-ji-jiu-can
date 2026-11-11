#!/bin/sh
# 表格编程·全局静态翻译 验收脚本
set -e
cd "$(dirname "$0")/.."
mkdir -p tests/out

./gen_fixture tests/out

PASS=0

# wasm: 用 cangjie-wasm 构建的真实产物
for W in fact nested loop2 cont; do
  WASM=../../cangjie-wasm/build/$W.wasm
  if [ -f "$WASM" ]; then
    ./tabtrans wasm "$WASM" > "tests/out/$W.list" 2> "tests/out/$W.err"
    if grep -q "未译: 0" "tests/out/$W.list"; then
      echo "ok wasm $W 全译出"
    else
      echo "FAIL wasm $W 存在未译指令"; exit 1
    fi
  fi
done

# ELF64 + ARM64
./tabtrans elf tests/out/arm64.elf > tests/out/arm64.list
for S in 迁 合 退 出 入 判 召 差 审 复; do
  grep -q " $S " tests/out/arm64.list || { echo "FAIL arm64 缺 $S"; exit 1; }
done
grep -q "未译: 0" tests/out/arm64.list || { echo "FAIL arm64 未译非零"; exit 1; }
echo "ok arm64.elf 十符号全译出"

# ELF32 + ARM32
./tabtrans elf tests/out/arm32.elf > tests/out/arm32.list
for S in 迁 合 退 出 入 判 召 差 审 复; do
  grep -q " $S " tests/out/arm32.list || { echo "FAIL arm32 缺 $S"; exit 1; }
done
grep -q "未译: 0" tests/out/arm32.list || { echo "FAIL arm32 未译非零"; exit 1; }
echo "ok arm32.elf 十符号全译出"

# 裸机器码块
./tabtrans elf tests/out/arm64.bin > /dev/null 2>&1 && { echo "FAIL 裸块应被拒"; exit 1; } || true
echo "ok 裸块拒绝路径"

# 来源列强制: 无来源行载入即拒
cp table.chain tests/out/bad.chain
printf 'gttx://tab|折|控制流|折叠|wasm|0x99:none|无来源行\n' >> tests/out/bad.chain
if ./tabtrans wasm ../../cangjie-wasm/build/fact.wasm --table tests/out/bad.chain > /dev/null 2>&1; then
  echo "FAIL 无来源行被放行"; exit 1
fi
echo "ok 无来源行载入即拒"

# 汇编文本: 符号直呼 + 地址呼唤
cat > tests/out/demo.asm <<'EOF'
# 符号直呼
迁 r0,7
合 r0,r0,1
出 r0,r1
复
@迁
EOF
./tabtrans asm tests/out/demo.asm > tests/out/demo.list
grep -q "@呼唤 迁 -> #0001" tests/out/demo.list || { echo "FAIL 呼唤未命中"; exit 1; }
echo "ok 汇编文本 呼唤命中"

# 口诀清点格与折叠层
grep -q "乘法口诀清点格" tests/out/fact.list
grep -q "折叠层号" tests/out/fact.list
echo "ok 口诀清点格在列"

# verify: 载入歧义检查 + 精确覆盖清点(无采样)
./tabtrans verify wasm > tests/out/verify_wasm.list
./tabtrans verify arm32 > tests/out/verify_arm32.list
./tabtrans verify arm64 > tests/out/verify_arm64.list
for V in wasm arm32 arm64; do
  grep -q "歧义: 0" "tests/out/verify_$V.list" || { echo "FAIL verify $V 有歧义"; exit 1; }
done
EXPECT=$(grep -c "^gttx://tab|.*|wasm|" table.chain)
grep -q "覆盖操作码: $EXPECT / 256" tests/out/verify_wasm.list || { echo "FAIL wasm 覆盖数($EXPECT)不符"; exit 1; }
grep -q "覆盖字数:" tests/out/verify_arm32.list
grep -q "覆盖字数:" tests/out/verify_arm64.list
echo "ok verify 歧义=0 覆盖可数"

PASS=1
echo "验收全绿 ($PASS)"
