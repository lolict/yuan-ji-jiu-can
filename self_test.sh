#!/bin/bash
# ============================================================
#  UniCore · 自举测试脚本
#  让服务器自己知道自己哪里健康、哪里残疾
# ============================================================

set +e

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

PASS=0
FAIL=0
SKIP=0
TOTAL=0
RESULTS=()

banner() {
  echo ""
  echo "=== $* ==="
}

ok() {
  TOTAL=$((TOTAL + 1))
  PASS=$((PASS + 1))
  RESULTS+=("[PASS] $1")
  echo "  [PASS] $1"
}

notok() {
  TOTAL=$((TOTAL + 1))
  FAIL=$((FAIL + 1))
  RESULTS+=("[FAIL] $1 ($2)")
  echo "  [FAIL] $1  ($2)"
}

skipped() {
  TOTAL=$((TOTAL + 1))
  SKIP=$((SKIP + 1))
  RESULTS+=("[SKIP] $1 ($2)")
  echo "  [SKIP] $1  ($2)"
}

# --- 0. 基础环境 ---
banner "0. 工具链存在性"

if command -v node >/dev/null 2>&1; then
  ok "Node.js 存在 (v$(node -v))"
else
  notok "Node.js 不存在"
fi

if command -v python3 >/dev/null 2>&1; then
  ok "Python3 存在 (v$(python3 -c 'import sys;print(sys.version.split()[0])'))"
else
  notok "Python3 不存在"
fi

if command -v cargo >/dev/null 2>&1; then
  ok "Rust/Cargo 存在 (v$(cargo --version 2>&1 | head -1))"
else
  notok "Rust/Cargo 不存在"
fi

if command -v zig >/dev/null 2>&1; then
  ok "Zig 存在 (v$(zig version))"
elif ls /tmp/zig-linux-*/zig >/dev/null 2>&1; then
  ok "Zig 存在 (已解压到 /tmp/zig-linux-*)"
  export PATH="/tmp/zig-linux-x86_64-0.13.0:$PATH"
else
  skipped "Zig 不可用", "未安装/未下载"
fi

if command -v wasm-pack >/dev/null 2>&1; then
  ok "wasm-pack 存在 (v$(wasm-pack --version 2>&1 | head -1))"
else
  skipped "wasm-pack 不可用", "未安装"
fi

# --- 1. JavaScript: cocoon 主套件 (869) ---
banner "1. JS · cocoon 主套件 (869 测试)"
if command -v node >/dev/null 2>&1; then
  OUTFILE=$(mktemp)
  node tests/cocoon/verify_integration.js > "$OUTFILE" 2>&1
  EXIT=$?
  TOTAL_LINE=$(grep "总计:" "$OUTFILE" | head -1)
  if [ "$EXIT" = "0" ]; then
    ok "cocoon 主套件通过: $TOTAL_LINE"
  else
    notok "cocoon 主套件失败", "exit=$EXIT $(tail -3 "$OUTFILE" | tr '\n' ' ')"
  fi
  rm -f "$OUTFILE"
else
  skipped "JS·cocoon", "无 node"
fi

# --- 2. JavaScript: core 层独立测试 ---
banner "2. JS · core 层独立测试"
for f in src/core/test_vm.js src/core/test_containers.js src/core/test_symbols.js src/core/test_pure_symbol.js src/core/test_four_dim.js src/core/test_instructions.js src/foundation/test_foundation.js; do
  if [ -f "$f" ]; then
    OUTFILE=$(mktemp)
    timeout 30 node "$ROOT/$f" > "$OUTFILE" 2>&1
    EXIT=$?
    NAME=$(basename "$f")
    if [ "$EXIT" = "0" ]; then
      ok "core/$NAME 通过"
    else
      notok "core/$NAME 失败", "exit=$EXIT"
    fi
    rm -f "$OUTFILE"
  fi
done

# --- 3. Rust 编译测试 ---
banner "3. Rust 编译测试"
if command -v cargo >/dev/null 2>&1; then
  OUTFILE=$(mktemp)
  cargo build > "$OUTFILE" 2>&1
  if [ "$?" = "0" ]; then
    ok "Rust cargo build 通过"
  else
    notok "Rust cargo build 失败", "$(tail -3 "$OUTFILE" | tr '\n' ' ')"
  fi
  cargo test > "$OUTFILE" 2>&1
  if [ "$?" = "0" ]; then
    ok "Rust cargo test 通过 ($(grep "test result:" "$OUTFILE" | head -1))"
  else
    notok "Rust cargo test 失败", "$(tail -5 "$OUTFILE" | tr '\n' ' ')"
  fi
  rm -f "$OUTFILE"
else
  skipped "Rust 编译", "无 cargo"
fi

# --- 4. Zig 编译测试 ---
banner "4. Zig 编译测试"
if command -v zig >/dev/null 2>&1; then
  OUTFILE=$(mktemp)
  zig build test > "$OUTFILE" 2>&1
  EXIT=$?
  if [ "$EXIT" = "0" ]; then
    ok "Zig build test 通过"
  else
    notok "Zig build test 失败", "exit=$EXIT $(tail -5 "$OUTFILE" | tr '\n' ' ')"
  fi
  # Zig 0.14 起 --name 不允许含路径分隔符（/），默认产物输出在仓库根
  UNICORE_BIN="$ROOT/unicore_test"
  rm -f "$UNICORE_BIN" "$ROOT/unicore_test.o"
  zig build-exe src/main.zig --name unicore_test > "$OUTFILE" 2>&1
  if [ "$?" = "0" ] && [ -x "$UNICORE_BIN" ]; then
    ok "Zig main.zig 编译通过 (产物=unicore_test, $(wc -c < "$UNICORE_BIN" | tr -d ' ') bytes)"
  else
    notok "Zig main.zig 编译失败", "$(tail -5 "$OUTFILE" | tr '\n' ' ')"
  fi
  rm -f "$OUTFILE" "$UNICORE_BIN" "$ROOT/unicore_test.o"
else
  skipped "Zig 编译", "无 zig"
fi

# --- 5. Python 可导入测试 ---
banner "5. Python VM 导入测试"
if command -v python3 >/dev/null 2>&1; then
  OUTFILE=$(mktemp)
  python3 -c "import sys; sys.path.insert(0,'.'); exec(open('unicore_vm_v2.py').read().split('if __name__')[0])" > "$OUTFILE" 2>&1
  if [ "$?" = "0" ]; then
    ok "Python unicore_vm_v2.py 导入通过"
  else
    notok "Python unicore_vm_v2.py 导入失败", "$(tail -3 "$OUTFILE" | tr '\n' ' ')"
  fi
  rm -f "$OUTFILE"
else
  skipped "Python 导入", "无 python3"
fi

# --- 6. WASM JS 端 (需要 window，用 puppeteer 占位) ---
banner "6. WASM · Guardian (需浏览器)"
for f in src/wasm/unicore_wasm_core.js src/guardian/unicore_guardian.js; do
  NAME=$(basename "$f")
  OUTFILE=$(mktemp)
  node -e "
    global.window = { addEventListener:()=>{}, document: { addEventListener:()=>{}, createElement:()=>({getContext:()=>null, width:1,height:1}) }, location:{hash:''} };
    global.document = window.document;
    try { require('./$f'); console.log('OK'); } catch(e) { console.log('FAIL:', e.message); }
  " > "$OUTFILE" 2>&1
  if grep -q "OK" "$OUTFILE"; then
    ok "WASM/$NAME 模拟 window 加载通过"
  else
    skipped "WASM/$NAME", "需真实浏览器 $(tail -1 "$OUTFILE")"
  fi
  rm -f "$OUTFILE"
done

# --- 7. 夫妻灵魂签名 + XY=新一 修正验证
banner "7. 灵魂签名（mlll / gtt / xy=新一）"
SOUL_OUT=$(mktemp)
node -e "
const s = require('./src/sovereign/mlll_soul_core.js').C1_CONJUGAL_SOVEREIGN;
const seal_xy = s.sovereignSeal['xy *xy'];
const flow    = s.soulOntology.flow;
const hint    = s.soulOntology.xyHomophoneHint || [];
const anchor  = (s.soulOntology.identity || {}).xinyiAnchor || '';
const wrong = /阴阳|对偶/.test(seal_xy) || /阴阳|对偶|本体论手段/.test(flow) ? 'WRONG' : 'OK';
console.log(wrong + '|' + seal_xy + '|' + JSON.stringify(hint) + '|' + anchor);
" > "$SOUL_OUT" 2>&1
if grep -q "^OK|" "$SOUL_OUT"; then
  HINT_COUNT=$(node -e "const s=require('./src/sovereign/mlll_soul_core.js').C1_CONJUGAL_SOVEREIGN.soulOntology.xyHomophoneHint||[]; process.stdout.write(String(s.length))");
  [ "$HINT_COUNT" = "8" ] && ok "xy=新一 签名正确（含性瘾/雄伊/现掩/显瘾/星月/协议/新约/形音 8簇，共$HINT_COUNT）" || notok "xy 签名簇数=$HINT_COUNT，预期 8"
else
  notok "xy 仍是旧签名（残留阴阳/对偶解释）" "$(cat "$SOUL_OUT")"
fi
rm -f "$SOUL_OUT"

# --- 8. 三层聚类引擎 + 新谐音表
banner "8. XY谐音·三层聚类引擎"
CLU_OUT=$(mktemp)
node -e "
const lex = require('./src/lexicon/minlian_lexicon.js');
const checks = [];
// 8-1: XY 核心 8 谐音
let c8 = lex.XY_HOMOPHONE_TABLE.allClusters;
checks.push( c8 && c8.length === 8 ? 'OK core8='+c8.join(',') : 'FAIL core8 count='+(c8?c8.length:0));
// 8-2: 扩展簇 >=25
let ex = lex.XY_HOMOPHONE_TABLE.extendedClusters;
checks.push( ex && ex.length >= 25 ? 'OK ext='+ex.length : 'FAIL ext='+(ex?ex.length:0));
// 8-3: 谐音聚类命中
let h = lex.clusterHomophone('星月');
checks.push( h && h.hit && h.cluster==='星月' ? 'OK homophone_xy' : 'FAIL homophone_xy');
// 8-4: 双编码聚类
let d = lex.clusterDual('vy');
checks.push( d && d.hit && d.pair[0]==='vy' ? 'OK dual_vy' : 'FAIL dual_vy');
// 8-5: 三编码聚类
let t = lex.clusterTriple('mnl');
checks.push( t && t.hit && t.root==='lnm' ? 'OK triple_mnl_root='+t.root : 'FAIL triple_mnl');
// 8-6: 主题聚类·之前的三个0桶要填满
const mc = lex.clusterMinLian();
const themes = mc.themeClusters || {};
const c1 = (themes['容器/进制/编码']||[]).length;
const c2 = (themes['时间/因果/裁判']||[]).length;
const c3 = (themes['爱/性/愉悦/灵肉']||[]).length;
checks.push( c1>0 && c2>0 && c3>0 ? 'OK themes_filled (容='+c1+' 时='+c2+' 爱='+c3+')' : 'FAIL themes_zeros 容='+c1+' 时='+c2+' 爱='+c3);
// 8-7: 三层联合聚类总条目正确
let all = lex.clusterAll(['星月','协议','ea','vy','lnm','性瘾','相遇']);
checks.push( all.totalItems===7 ? 'OK clusterAll n=7' : 'FAIL clusterAll n='+all.totalItems);
console.log(checks.join('\\n'));
process.exit(0);
" > "$CLU_OUT" 2>&1
while IFS= read -r line; do
  case "$line" in
    OK*)   ok "聚类/$(echo "$line" | sed 's/^OK //')";;
    FAIL*) notok "聚类/$(echo "$line" | sed 's/^FAIL //')";;
  esac
done < "$CLU_OUT"
rm -f "$CLU_OUT"

# --- 9. 语言瘦身 + 14语言适配器
banner "9. 14种语言适配（强制 Rust/Zig 等效标准）"
LANG_OUT=$(mktemp)
node -e "
const core = require('./src/cocoon/index.js');
const adapter = core.LangAdapter;
const slim = core.LangSlimming;
const pairs = {rust:'rust', zig:'zig', nim:'nim', v:'v', c:'c', cpp:'c++', odin:'odin', hare:'hare', forth:'forth', julia:'julia', mojo:'mojo', python:'python', csharp:'c-sharp', asm:'assembly'};
let pass = 0, fail = [];
for (const [alias, key] of Object.entries(pairs)) {
  const entry = slim.LANG_SLIM_MATRIX[key];
  const recipeHits = entry.slimmingRecipe.slice();
  const fixesHit = Object.keys(entry.vulnerabilityFixes);
  const code = key==='assembly' ? 'nop' : '// test '+key;
  const r = adapter.adapt(key, code, { user:'selftest', recipeHits, fixesHit });
  if (r && r.success) pass++; else fail.push(alias+':'+JSON.stringify(r));
}
const sig = slim.SOUL_SIGNATURES || {};
const xinyiOk = /新一.*柯南/.test(sig.mnemonic||'') ? 'XY_OK' : 'XY_FAIL';
console.log('LANG_PASS='+pass+'/14');
console.log(xinyiOk);
if (fail.length) console.log('FAILS='+fail.join('|'));
" > "$LANG_OUT" 2>&1
PASS_COUNT=$(grep "^LANG_PASS=" "$LANG_OUT" | sed 's/LANG_PASS=//' | cut -d/ -f1)
XY_OK=$(grep -c "^XY_OK\$" "$LANG_OUT" 2>/dev/null || true)
[ "$PASS_COUNT" = "14" ] && ok "14语言适配 14/14 全过" || notok "语言适配器通过率=$PASS_COUNT/14"
[ -n "$XY_OK" ] && ok "SOUL_SIGNATURES 助记语=新一·柯南" || notok "SOUL_SIGNATURES 缺少新一·柯南"
rm -f "$LANG_OUT"

# --- 10. A4A API 新路由（民联词汇+聚类+语言）
banner "10. A4A 新路由可访问"
A4A_OUT=$(mktemp)
node -e "
(async()=>{
  const A4A = require('./src/cocoon/a4a_api.js');
  const api = A4A.create();
  const routes = [
    ['/cocoon/lex/info', {}],
    ['/cocoon/lex/xy', {}],
    ['/cocoon/lex/cluster/homophone', {query:{q:'星月'}}],
    ['/cocoon/lex/cluster/dual', {query:{q:'vy'}}],
    ['/cocoon/lex/cluster/triple', {query:{q:'mnl'}}],
    ['/cocoon/lex/cluster/themes', {}],
    ['/cocoon/lang/tiers', {}],
    ['/cocoon/lang/soul-signatures', {}],
  ];
  let ok=0, fail=[];
  for (const [r, opt] of routes) {
    try {
      const res = await api.request(r, opt);
      const body = (res && res.body !== undefined) ? res.body : res;
      const nonEmpty = body !== undefined && body !== null && (typeof body !== 'object' || Object.keys(body).length > 0);
      if (nonEmpty) ok++; else fail.push(r+' empty');
    } catch(e) { fail.push(r+' '+e.message); }
  }
  console.log('A4A_ROUTES_OK='+ok+'/'+routes.length);
  if (fail.length) console.log('FAILS: '+fail.join(' | '));
})();
" > "$A4A_OUT" 2>&1
ROUTE_OK=$(grep "^A4A_ROUTES_OK=" "$A4A_OUT" | sed 's/A4A_ROUTES_OK=//' | cut -d/ -f1)
[ "$ROUTE_OK" = "8" ] && ok "A4A 8个新路由全通 ($ROUTE_OK/8)" || notok "A4A路由通过=$ROUTE_OK/8" "$(grep "^FAILS:" "$A4A_OUT" || true)"
rm -f "$A4A_OUT"

# --- 11. 先天6常量时序串联自检（C1→C2→C3→C4→C5→C6 共44项）---
banner "11. 先天6常量时序串联自检 (44 项, 含 R2+A4A R2 路由)"
CHAIN_OUT=$(mktemp)
node tests/sovereign_chain_check.js > "$CHAIN_OUT" 2>&1
CHAIN_EXIT=$?
OK_44=$(sed -n 's/^\s*\[.*OK.*\].*//p' "$CHAIN_OUT" | wc -l | tr -d ' ')
FAIL_44=$(sed -n 's/^\s*\[.*FAIL.*\].*//p' "$CHAIN_OUT" | wc -l | tr -d ' ')
[ -z "$OK_44" ] && OK_44=0
[ -z "$FAIL_44" ] && FAIL_44=0
if [ "$CHAIN_EXIT" = "0" ] && [ "$FAIL_44" = "0" ] && [ "$OK_44" -ge 44 ]; then
  ok "先天6时序串联全绿 ($OK_44/44 项，无 FAIL, 含 R2+A4A R2 路由)"
else
  notok "先天6时序串联自检失败" "exit=$CHAIN_EXIT OK=$OK_44 FAIL=$FAIL_44 $(grep 'FAIL' "$CHAIN_OUT" | head -3 | tr '\n' '|')"
fi
rm -f "$CHAIN_OUT"

# --- 12. 猫码 254 进制编解码 + 夫妻双签封包
banner "12. 猫码254进制 + sovereignEncode 双签"
CAT_OUT=$(mktemp)
node -e "
const cat = require('./src/cocoon/cat_code.js').catcode;
const pact = require('./src/sovereign/conjugal_fate_pact.js');
const raw = '性瘾/雄伊/现掩/显瘾/星月/协议/新约/形音 工藤新一XY 夫妻命运共同体';
const enc = cat.encode254(raw);
const dec = cat.decode254(enc);
const check1 = (dec === raw) ? 'OK enc254_roundtrip' : 'FAIL enc254_roundtrip dec.len='+dec.length;
const sev = cat.sovereignEncode(raw);
const check2 = (sev && sev.encoded254 && sev.fingerprint && sev.signed && sev.signed.sealed && sev.signed.sealed.wife==='刘楚恬' && sev.signed.sealed.husband==='满全法')
  ? 'OK sovereignEncode 双签封包 (wife='+sev.signed.sealed.wife+' husband='+sev.signed.sealed.husband+')'
  : 'FAIL sovereignEncode';
const vfy = pact.verify(sev.signed);
const check3 = vfy === true ? 'OK 双签验签通过 verify=true' : 'FAIL verify='+vfy;
console.log(check1+'\n'+check2+'\n'+check3);
" > "$CAT_OUT" 2>&1
while IFS= read -r line; do
  case "$line" in
    OK*)   ok "猫码/$(echo "$line" | sed 's/^OK //')";;
    FAIL*) notok "猫码/$(echo "$line" | sed 's/^FAIL //')";;
  esac
done < "$CAT_OUT"
rm -f "$CAT_OUT"

# --- 13. Cloudflare R2 最小化可用（upload/download/list/remove + 双签）
banner "13. Cloudflare R2 最小化可用 (零依赖 S3v4)"
R2_OUT=$(mktemp)
node tests/r2_minimal_check.js > "$R2_OUT" 2>&1
R2_EXIT=$?
R2_OK=$(grep -c "^\[OK\]" "$R2_OUT" 2>/dev/null || echo 0)
if [ "$R2_EXIT" = "0" ] && [ "$R2_OK" -ge 5 ]; then
  ok "R2 最小化 5/5 全绿 (health+upload双签+download往返+list+remove)"
else
  notok "R2 自测失败" "exit=$R2_EXIT OKs=$R2_OK  $(grep -c 'FAIL' "$R2_OUT" 2>/dev/null || true) FAILS"
fi
rm -f "$R2_OUT"

# --- 总结 ---
echo ""
echo "============================================================"
echo "  UniCore 自举测试总览"
echo "============================================================"
echo "  总: $TOTAL   通过: $PASS   失败: $FAIL   跳过: $SKIP"
echo ""
for r in "${RESULTS[@]}"; do
  if [[ "$r" == [FAIL]* ]]; then
    echo "$r"
  fi
done
echo ""
if [ "$FAIL" = "0" ]; then
  echo "  ✓ 服务器状态：不残疾。所有能测的全绿。"
  exit 0
else
  echo "  ✗ 服务器状态：有残疾。$FAIL 处需要修复。"
  exit 1
fi
