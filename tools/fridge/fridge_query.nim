## 冰箱查询: 916 卡 → 状态机调度 → 全管线验证
##
## 对齐协议 → 满全法状态机桥接 (生产用, 不依赖 JSON)
##
## 管线: AlignCard生成 → card_to_recipe → Recipe焖煮 → serve查询

import std/[strutils, strformat, tables]
import fridge_types, fridge, recipe_db

# ═══════════════════════════════════════════════════════════════
## 916 参数化生成 (与 fridge_aligner 同源)
## ═══════════════════════════════════════════════════════════════

proc gen_cards(total: int = 916; period: int = 64): seq[
    tuple[pack_id: int; operand: int; witness: int; time_window: int; platform: int]
  ] =
  result = newSeq[type(result[0])](total)
  for i in 0 ..< total:
    let fam = i div 64
    let slot = (i mod 63) + 1  # 1..63
    let cc = fam + 1
    let operand = (fam shl 8) or slot  # 确保 > 0
    let witness = (i + 1) + cc * 256 + 1  # 始终非零
    result[i] = (
      pack_id: i + 1,
      operand: operand,
      witness: witness,
      time_window: slot,
      platform: i mod 4
    )

# ═══════════════════════════════════════════════════════════════
## 主入口
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 冰箱查询 · 对齐协议 → 状态机调度 ═══"
  echo ""

  # 1. 生成 916 卡
  let cards = gen_cards(916, 64)
  echo fmt"  生成卡: {cards.len}"

  # 2. 每卡 → CardRecipe
  var card_recipes: seq[CardRecipe] = @[]
  for c in cards:
    card_recipes.add(card_to_recipe(
      pack_id = c.pack_id,
      operand = c.operand,
      witness = c.witness,
      time_window = c.time_window,
      platform = c.platform
    ))

  # 3. 合并为 RecipeBook
  let book = make_recipe_book("916-容器族-对齐", card_recipes)
  echo fmt"  总规则数: {book.totalRules}"
  echo ""

  # 4. 打印统计
  echo book_stats(book)
  echo ""

  # 5. 焖煮成 StewedPlate
  var all_rules: seq[Transition] = @[]
  for cr in card_recipes:
    all_rules.add(cr.rules)

  let recipe = Recipe(rules: all_rules)
  let plate = stew(recipe)
  echo fmt"  焖煮完成: plate 规则={plate.ruleCount} 封印={plate.isSealed}"
  echo ""

  # 6. 查询 sample: 触 → 应 → 路径
  echo "── 前 10 卡 serve 查询 ──"
  for i in 0 ..< min(10, card_recipes.len):
    let cr = card_recipes[i]
    if cr.rules.len == 0: continue
    let r = cr.rules[0]
    let trans = plate.serve(r.fromState, r.event)
    echo fmt"  #{cr.pack_id:4d} {$(r.fromState):4s} × {$(r.event):4s} → {$(trans.action):4s} → {$(trans.toState):4s} | {trans.polarity}"
  echo ""

  # 7. 全量 serve 验证
  echo "── 全量 serve 验证 ──"
  var ok_count = 0
  var reject_count = 0
  var conflict_count = 0
  var seen: Table[(StateSymbol, Event), Transition]

  for cr in card_recipes:
    for r in cr.rules:
      let key = (r.fromState, r.event)
      let trans = plate.serve(r.fromState, r.event)
      # 检查与配方一致
      if trans.action == r.action and trans.toState == r.toState:
        ok_count += 1
      elif trans.action == acReject:
        reject_count += 1
      else:
        # 可能与同 key 的其他卡冲突
        if seen.hasKey(key) and seen[key].toState != trans.toState:
          conflict_count += 1
        seen[key] = trans

  let total_queries = ok_count + reject_count + conflict_count
  echo fmt"  serve 总数: {total_queries}"
  echo fmt"  匹配: {ok_count}"
  echo fmt"  拒绝: {reject_count}"
  echo fmt"  冲突: {conflict_count}"
  echo ""

  # 8. 极性轨迹验证
  echo "── 极性轨迹 (前 20 卡) ──"
  var pol_seq: seq[string] = @[]
  for i in 0 ..< min(20, card_recipes.len):
    let cr = card_recipes[i]
    if cr.rules.len == 0: continue
    pol_seq.add($(cr.polarity))
  echo "  " & pol_seq.join(" → ")
  echo ""

  # 9. 查询总结
  echo "═══ 完成 ═══"
  echo fmt"  916 → {book.totalRules} 规则 → {plate.ruleCount} plate"
  if conflict_count == 0:
    echo "  ✓ 零冲突, 状态机调度一致性 OK"
  else:
    echo fmt"  ⚠ {conflict_count} 冲突 (同 key 多卡重载)"
