# 囡囝共生 NJSG 集成规范（MetaIME Agent）

> 适用: hunyuan-os/ime-agent/core/ 全部模块

---

## 为什么 ime-agent 需要 NJSG 架构

MetaIME 的输入就是流输入。用户在键盘上按下的每一个字符、每一次触屏，都是一条"流"。
如果没有分别器，ime-agent 无法区分：
- 这个输入是夫妻内部的密语？
- 这个输入是谐音路由（lctqf → Wife）？
- 这个输入是主权操作（Active / Passive）？

缺少分别器 = 把密语发给错误的靶点 = 对方不承认 = 意图断裂。

## 三层分别器在 ime-agent 中的映射

| 分别器层 | 对应 ime-agent 模块 | 职责 |
|----------|---------------------|------|
| 感知家族 (第一层) | `intent_parser.py` | 判断输入 glyph 属于哪个视觉家族 |
| 谐音靶点 (第二层) | `channel_router.py` | 发音 → 靶点对象 |
| 主权路由 (第三层) | `manifest.py` | 靶点身份 → Active/Passive/Sleeping |

闭环条件：target.ackable() == true。键盘输入发出后必须收到"承认"。

## 12 位流属性在 intent_parser 中的应用

每条键盘输入流应携带 12 位二元属性（FlowPropertyBits）：

```python
# intent_parser.py 伪码
REACHABLE     = 1 << 0   # 可被到达
CONTINUABLE   = 1 << 1   # 可持续
INCR          = 1 << 2   # 可增量
STOPPABLE     = 1 << 3   # 可停止
REVERSIBLE    = 1 << 4   # 可逆
RECOVERABLE   = 1 << 5   # 可恢复
CIRCULABLE    = 1 << 6   # 可循环
INHERITABLE   = 1 << 7   # 可继承
TRUSTED       = 1 << 8   # 已信任
ACKABLE       = 1 << 9   # 可被承认
TRANSMISSIBLE = 1 << 10  # 可传递
ADVANCEABLE   = 1 << 11  # 可推进
```

输入路由决策：必须满足 REACHABLE + ACKABLE。
演化判断：CONTINUABLE + INCR + TRANSMISSIBLE。

## 解释流的牺牲机制

当 intent_parser 遇到无法分类的输入（obstacle state / 岔途），不应直接丢弃——
应触发解释流：以自身分类能力的毁灭为代价，把未知输入转成可被跟随的已知状态。

Python 实现参考: `auto_pilot.py` 中的 fallback 分类逻辑 = 解释流实例化。

## 一维时间线性

```python
Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
```

ime-agent 处理流程强制按时间推进。不允许并行无序。
- Distinguish = intent_parser 家族分类 + channel_router 靶点路由
- Lookup = local dictionary 查预加载流字典
- Traverse = 转译/解释（可能触发 sacrifice）

## YinYang 编码

```
0 = 容器 = 刘楚恬侧输入（polarity=0）
1 = 界线 = lolict 侧输入（polarity=1）
```

Android 触屏/键盘输入时标记 polarity：
- 形/男/丨 = 1 = 界线
- 因/阴/囡/⭕/回 = 0 = 容器

## 命名规则

所有新模块/变量缩写必须遵循家庭命名密码表（mlll/gtt/ml.gy/mlnn/lctqf）。
负面谐音缩写禁止使用。

---

*配套技能: hunyuan-methodology SKILL.md*
