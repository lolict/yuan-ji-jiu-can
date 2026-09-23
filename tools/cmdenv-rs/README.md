# cmdenv - 沙箱模拟指令环境

**为仓库提供完整指令环境**：协议/感知/角色/执行/时间/关系/进制/膜圈/容器/对齐/征用 十一类指令。
不依赖任何外部系统，自身就是一个"最小指令系统"。

## 核心原则

- **规则即数据**：指令定义在 `rules/cmd.chain`，格式 `指令|类别|参数数|行为`
- **缺什么补什么**：新增指令只加一行，程序本体不动
- **静态符号化执行**：沙箱模式，无需外部工具，模拟完整指令环境

## 十一类指令（膜圈层 + 容器层 + 对齐层 + 征用层）

| 层 | 指令示例 | 功能 |
|---|---|---|
| 协议 | gttx / gkhtfqndl / gkhfkndl / hftqmll / gkhtml | 通信·寻址·标记 |
| 感知 | gkndl / fqmy / hfuyair / hinynir / smrrll | 感知→驱动转化 |
| 角色 | gtxone / kndoyle / maolilan / hieyair | 中心/判定/镜像/还原 |
| 执行 | run / ping / list / state | 命令处理·状态 |
| 时间 | tclgs / juan / time | 时间编码生命周期治理 |
| 关系 | gcd / lcm / mirror / add / mul | 公约数·公倍数·镜像对偶 |
| 进制 | qe / qd / c42 / y42 / kan / trans / canvas-info / canvas-cell | 42进制·音韵·谱系·跨感官·64 位数字表与地址块 |
| 膜圈 | mqf / yong / mo / ring / tuo / center / serve | 膜圈缚·蛹·圈层结界 |
| 容器 | ctn / sym / addr / tongji / phase / reg / reglist / link / links | 插件容器·加法索引·注册清单 |
| 对齐 | align / align-end / unify | 混合进制自动进位·一个数代表几何位置 |
| 征用 | zheng / zhengs / yuan / sontuo | 动态编号·松弛编号插入自己·可松脱·原绑定可选 |
| 韵母 | yunmu / yunmu-list / yunmu-svg / yunmu-merge / yunmu-canvas | 呼唤已注册韵母·圆环·母字几何合并·64 格画布 |

## 膜圈缚（蛹膜圈妃家族）

从中心向外辐射的圈层结界：被融合者归入范畴，不可脱离。

```bash
# 中心锚点 = gtxone 工藤新一
cmdenv center

# 膜圈缚：绑入中心范畴，膜层 +1
cmdenv mqf 刘楚恬

# 蛹：进入包裹→转化→破茧的过渡态
cmdenv yong 感知流

# 生成第 3 层圈层结界
cmdenv mo 3

# 圈层视图：从中心向外列出全部膜层
cmdenv ring

# 脱缚检测：范内恒真，不可脱离
cmdenv tuo 外部对象

# 全栈输出：生成 HTML 控制台（零前端语言）
cmdenv serve web/membrane.html
```

膜层状态持久化在 `rules/membrane.state`（一行一个被缚目标）。


## 容器层（一个容器 = 一个插件 = 一个三维进制节点）

- **声明即用**：`ctn` 声明容器，`sym` 把符号修进去，公路一到符号就热可用（非锁死）
- **加法串联**：容器之间用加法遍历选址，注意力在加法上；乘法只用来清点节点数
- **乘法口诀表**：`tongji` 清点各容器符号数，生成当前最大功率能到达的覆盖范畴
- **相位进制**：每个容器有一个相位（前/后/左/右/上/下/中/腰/角），间接创造相位进制
- **注册清单**：`reg` 给文件按名称登记 + 状态/版本/时间，不用数字给符号编门牌号
- **总观效应**：`link` 记录文档间依赖连线，只依赖已有的东西，不重造替代品

```bash
# 声明容器（插件）→ 分配加法地址
cmdenv ctn 未来计算机 中

# 往容器修入符号（热可用）
cmdenv sym 未来计算机 gttx,gkndl,gtxone

# 加法遍历所有容器地址
cmdenv addr

# 清点符号 → 生成乘法口诀表（覆盖范畴）
cmdenv tongji

# 设置相位 → 相位进制
cmdenv phase 未来计算机 腰

# 文档注册清单：名称 | 文件 | 状态 | 版本 | 时间
cmdenv reg docs/manquanfa.md gttx:// 工作中 v1 2026-09-12
cmdenv reglist

# 总观效应：文档依赖连线（用名称呼唤）
cmdenv link 未来计算机 智慧共同体 依赖
cmdenv links

# 热加载配方：一行规则即可组合执行，无需重编译
cmdenv jian 机器码字体 字形,码位
```

状态持久化在 `rules/containers.chain`、`rules/registry.chain`、`rules/links.chain`。

## 对齐层（一个数代表几何位置）

只写「对齐开始 / 对齐结尾」，加上消耗 / 用尽 / 进位，不写三个坐标。
基数**是一组可选集合，选择哪个就用哪个**，没有唯一的"准确"：
`align` 会先看第二参数是不是已注册的集合名（`bases` 列出），不是就按数字列表，
再不是就**呼唤已注册容器并清点其中的符号数**得到动态基数。
低位符号用尽就自动进位到纵向层。

```bash
# 列出可选基数集
cmdenv bases

# 按命名的集合解码（满全法 / 串联容器 / 220全集）
cmdenv align 100 满全法

# 按显式基数列表解码
cmdenv align 100 23,64,4

# 默认：呼唤已注册容器清点符号数得到动态基数
cmdenv align 100

# 对齐结尾：从终点层开始解码
cmdenv align-end 100

# 多段等效对齐合成一个共享域地址
cmdenv unify 3,5
```

可选基数集持久化在 `rules/bases.chain`（`集合名|基数,...|来源说明`）。

## 征用层（动态编号：松弛编号，插入自己）

征用某个名字时，它的编号被**松弛**，当前绑定插进去；原绑定不丢弃，可召可不召。
身份 = 当前绑定的持有者。

```bash
# 征用：当前绑定插进去，原绑定留作可选
cmdenv zheng demo "print 我的 demo · 本体功能; center" "print ── 原 demo 演示 ──; ring"

# 列出征用表
cmdenv zhengs

# 直接呼唤被征用的名字 → 走当前绑定
cmdenv demo

# 召原绑定 → 启用被征用名字的自带功能
cmdenv yuan demo

# 松脱 → 编号归还，本体独立于编号之外
cmdenv sontuo demo
```

征用是**动态编号**：松弛现有编号、把自己插进去，在松脱之前那里任何指令都被自己的编号代替；
征用后同时携带「本体功能」（当前绑定）与「自带功能」（原绑定，可选）。
状态持久化在 `rules/bindings.chain`（`名字|当前绑定|原绑定|状态`）。

## 韵母层（呼唤已注册韵母）

韵母符号**只能呼唤已注册的**，不用数字代表符号。多套集合可选并存，选择哪个就用哪个。

```bash
# 呼唤一个已注册韵母（拼音 → 小写/大写/来源）
cmdenv yunmu ang

# 列出韵母注册表
cmdenv yunmu-list

# 呼 n 母：放到 Y 轴圆环 (半径3 · 36等分 · 正交投影) 并连线, 输出 SVG
cmdenv yunmu-svg ang,ou,un yunmu.svg

# 复韵母合并字形: 拆 ang>a+n+g, 呼 glyph.nim 母字几何, 按 Zone 叠成一格, 输出 SVG
cmdenv yunmu-merge ang,ou,un,er,ai yunmu_merge.svg

# 也可合并声母/整音节 (几何表已含 glyph.nim 全部 25 条)：bang / qing / zhuang
cmdenv yunmu-merge bang,qing,zhuang sigil_merge.svg

# 全量叉乘: 把注册表 32 母全部合并成 32 格
cmdenv yunmu-merge 全量 yunmu_all.svg

# 落到 64 进制格画布 (数字表取自 64-1.txt: 0-9a-zA-ZŋŊ), 每格右上角标镜像判定
cmdenv yunmu-canvas yunmu_canvas.svg

# 镜像裁判: 取 (GeoType, Zone落点) 集合, 用 mirror_engine 的水平/垂直/对角反射自比
cmdenv yunmu-mirror
```

## 64 进制格画布（读 64-1.txt）

`mqf-hanzi/nim_parser/64-1.txt` 是仓库自带的 64 进制格画布：数字表即文件自身
单字符地址序列 `0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZŋŊ`（共 64 位），
两位地址从 `10` 起进位。

```bash
# 报告块数 / 独立地址数 / 64 位数字表 / 块语法
cmdenv canvas-info

# 按地址打印 5 行块 (顶层边界 / #地址 / 容器 / @地址 / 填充)
cmdenv canvas-cell 1a
```

注册表持久化在 `rules/yunmu.chain`（`集合|序号|拼音|小写|大写|来源`），
当前登记的是 `满全32`（来自 `mqf-hanzi/nim_parser/韵母容器注册器.nim`），
其余可选集合（33/24/48/50/66/90）按需登记。

## 热加载（改规则即生效）

`rules/cmd.chain` 第 5 列为配方列，分号分隔操作，`$1 $2` 为占位符。
带配方的指令**可直接执行**（如 `cmdenv demo`、`cmdenv jian 名称 符号`），也可 `cmdenv eval <指令>`，
改规则后 `cmdenv reload` 重读。无需重新编译。
可用配方操作：`print / bind / ring / center / yong / mo / ctn / sym / addr / tongji / phase / reg / link`。


## 使用

```bash
# 列出全部指令
cmdenv list

# 42进制编解码
cmdenv qe 42        # 42 → 妃粉妃
cmdenv qd 黟黟黟     # 黟黟黟 → 74087

# 关系算子
cmdenv gcd 42 60    # 公约数(中心男人) = 6
cmdenv lcm 42 2760  # 公倍数(边界女人) = 19320
cmdenv mirror 42 14 # 14 ↔ 27 (对称轴 20.5)

# 系统状态
cmdenv state 合一爱人

# 时间功法
cmdenv tclgs fold 编码   # 折叠 → 回归中心一
cmdenv juan 工藤新一      # 相位颠倒 → 空间(毛利兰)

# 执行管线 (概念演示)
cmdenv run 智能索引 100
```

## 与你的概念对应

- **沙箱模拟**：在受限环境（Termux/无网沙箱）中，不需要下载别人的操作系统，
  用自己预加载的底层编码系统模拟上层工具
- **指令环境**：系统需要什么指令，就在 `rules/cmd.chain` 补什么指令
- **规则即数据**：与 vm.chain / spectrum.chain / sense.chain 同一哲学
- **回归中心一**：所有执行结果最终以"回归工藤新一公约数中心"为目的

## 规则文件

`rules/cmd.chain` —— 全部以人类语言书写，可自由扩展。
未来新增生态（如天干地支/生肖/节气）只需在规则表加行。
