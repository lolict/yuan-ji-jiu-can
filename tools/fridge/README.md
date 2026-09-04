# tools/fridge · 冰箱状态机

> **编译焖煮·运行时只取用**

## 概念

- **配方书 (Recipe)** — 声明式规则：从哪个状态，遇哪个事件，做什么动作，到哪个状态
- **焖煮 (stew)** — 编译时把配方搅拌成查表板：`O(n)` 一次成型
- **查表板 (Plate)** — 运行时只读：`(状态, 事件) → (动作, 下一状态)` 映射表
- **上菜 (serve)** — 运行时纯查表：输入当前状态 + 事件，直接返回转移

## 设计原则

1. **预计算优于运行时** — 所有转移在 `stew()` 时算好
2. **查表优于判断** — 运行时无分支、无条件、无计算
3. **极性轨迹** — 每条转移携带极性 (阴/阳/平衡)，构成夫妻状态流
4. **加法空间封印** — "加"状态 `ssAdd × evTouch → acSeal`，运算本身不可分割

## 使用

```bash
export PATH="/mnt/data/catpaw/home/.choosenim/toolchains/nim-2.2.10/bin:$PATH"
nim c --cc:clang --run fridge_demo.nim
nim c --cc:clang --run fridge_baked_actions.nim
```

## 文件

| 文件 | 作用 |
|------|------|
| fridge_types.nim | 17 状态 + 11 事件 + 12 动作 + 极性编码 |
| fridge.nim | stew() / serve() / serveBatch() / validateRecipe / printPlate |
| fridge_demo.nim | NJSG 七态转移演示（主入口） |
| fridge_baked_actions.nim | bakePath() / executeBaked() 预烘焙动作序列 |

## 极性编码规则（满全法）

| 极性 | 数值 | 角色 |
|------|------|------|
| polYin (阴) | 0.0 | 容器 = 刘楚恬 |
| polYang (阳) | 1.0 | 界线 = 我 |
| polBoth (平衡) | 0.5 | 九元·囡囝共生 |
