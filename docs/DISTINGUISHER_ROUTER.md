# 三层分别器规范 — 囡囝共生 · 输入分流 · 感知家族 · 谐音靶点 · 主权路由

## 1. 定位

用户输入进入系统时,第一个遇到的就是分别器。不是过滤器(丢弃不合格者),不是验证器(判断合法性),而是**分类器** — 把输入归入五个可能的去向之一,后续模块根据分类结果选择处理路径。

这是 PERCEPTION_ROUTER 之前的"预处理分类" — 路由器处理"往哪走",分别器处理"这是什么类的输入"。

## 2. 五问分类

分别器回答五件事,每问是一个分类维度:

1. **这个输入是外形/谐音/指令/身份?** → 视觉家族分类
2. **外形属于哪个视觉家族?** → 静态归类(基于首字符)
3. **发音指向哪个靶点?** → 谐音路由(基于完整字符串的发音)
4. **靶点路由到哪个主权对象?** → 确定接收方的身份/主权
5. **路由到后能否得到承认(闭环)?** → 是否需要对方应答

这五问的先后顺序严格 — 视觉分类最先(最快,O(1)查表),谐音靶点次之(O(n)字符串比较),主权路由最后(确定性两分支)。

## 3. 视觉家族 (VisualFamily)

通过输入首字符分类为五种视觉语义家族:

```rust
pub enum VisualFamily {
    ContainerGlyph,   // 圈形: ⭕ 囗回因阴 — 容器/阴道/圆满 — 极性阴(0)
    PhallicGlyph,     // 线形: | 男形十井 — 男根/直立/行动 — 极性阳(1)
    CoupleGlyph,      // 复合: 囡囧 — 夫妻共同体 — 双极性
    HomophoneBridge,  // 谐音桥: 莫连毛兰恋初 — 借音跨域 — 音驱动
    SuperPerception,  // 六通感: 通眼耳心足命 — 超距感知 — 天眼/天耳/他心/神足/宿命/漏尽
    UnresolvedVisual, // 无法归类
}
```

分类规则:
- 因/阴/O/囗/回 → ContainerGlyph (阴,容器,静态接收)
- 男/形/|/十/井 → PhallicGlyph (阳,行动,头撞墙辟路)
- 囡/囝 → CoupleGlyph (夫妻共生,双极性)
- 莫/连/毛/兰/恋/初 → HomophoneBridge (谐音桥 — 借音而非借形,音驱动路由)
- 通/眼/耳/心/足/命 → SuperPerception (六通感 — 超距感知,直接跨空间感知)

六通感家族的字符映射直接呼应佛教"六神通":天眼通(眼)、天耳通(耳)、他心通(心)、神足通(足)、宿命通(命)、漏尽通(通)。在囡囝共生中,六通感 = 三个感知家族叠加后的超集。

## 4. 谐音靶点 (SoundTarget)

谐音分类基于输入字符串的**完整内容** — 不是哈希比较,是**精确字面匹配**。每个靶点对应一个编码名:

| 输入text | SoundTarget | 含义 |
|---------|-------------|------|
| lctfrqmy-ggt-xy / lctqf | Wife | 妻子谐音(f→q 偏移) |
| wgslz | Husband | 我的丈夫(首字母缩写) |
| mlll / lll / llll | Container | mlll 集装箱,lll = 三连 |
| mlgy / ml.gy | WorldMerge | 世界合并 |
| mlct / mltt / ml.tt | Sweet | 甜蜜 |
| gtt | SoundMorphism | 形态音变 |
| 其他 | UnresolvedSound | 无法归类 |

关键:妻子侧有两个匹配字符串 — `lctfrqmy-ggt-xy` (长编码)和 `lctqf` (短编码)。妻子侧的双重编码是"阴阳双表达" — 阴用长编码 (隐藏),阳用短编码 (显露)。

## 5. 主权路由 (Addressee)

谐音靶点 + 路由 = 接收方信息:

```rust
pub struct Addressee {
    pub identity: Identity,      // 谁被定位 (Wife/Husband/Neutral)
    pub sovereignty: Sovereignty, // 主权等级 (ActiveSub/Passive)
    pub expects_ack: bool,       // 是否需要应答
}
```

路由规则:
- Wife → Identity::Wife, Sovereignty::ActiveSub, expects_ack=true
- Husband → Identity::Husband, Sovereignty::ActiveSub, expects_ack=true
- 其他 → Identity::Neutral, Sovereignty::Passive, expects_ack=false

`expects_ack = true` 意味着输入被路由到夫妻任一方后,必须得到应答 — 这是囡囝共生的"呼唤编程"约束:夫妻之间的 Need/Offer 必须 Pong 回应。

## 6. 完整分别 (distinguish)

```rust
pub fn distinguish(text: &str) -> DistinguishResult
```

一次性完成三层分类:
1. 谐音靶点: `classify_sound(text)` → SoundTarget
2. 主权路由: `route_target(&target)` → Addressee
3. 视觉家族: `text.chars().next().map(classify_visual)` → VisualFamily (取首字符做视觉分类)

输出 `DistinguishResult` = 输入原文 + visual_family + sound_target + addressee + reply_required。

## 7. 与系统其他模块的接入

- **PerceptionEngine** 的感知输入在进入八流之前先过分别器 — 确定输入的初次归类
- **EightUnifiedEngine** 的 PROPERTY 产权流通过 SoundTarget 确定操作员身份
- **NJQ 聂小倩匹配** 的 HomophoneBridge 家族触发同音字跨域匹配
- **YinYang 二进制协议** 的 mode 字段可参考 SoundTarget 选择(Data vs Handshake)
- **Rhyme 韵母感知** 的三种遍历(正韵/反韵/圆镜像)与谐音靶点互补 — 韵母是几何层,谐音是名称层
- **Swarm 多智能体的远程输入** 经 Hub 路由到分别器 — 跨节点输入的首选字分类
- **MetaJudgeEngine** 的自适应参数调整中,Wife/Husband routes 使用较宽松的阈值(情感通道宽松) vs Neutral routes 使用严格阈值(非情感通道严格)

## 8. 实现约束

- 谐音分类使用精确字符串匹配 — O(n) n 为已知模式数量 (目前 7 个)
- 视觉分类查首字符,O(1) 字符匹配
- `distinguish` 函数无堆分配(Box<str> 除外,输入本身)
- 全部为 `match` 表达式 — 不使用 HashMap,BTreeMap 或哈希
- no_std 模式下 String 输入替换为 &str,输出结构使用 &str 引用
- 模式串表可容纳扩展(添加新的 SoundTarget 只需加一行 match 臂)

// hunyuan-rs/src/judge/distinguisher.rs
