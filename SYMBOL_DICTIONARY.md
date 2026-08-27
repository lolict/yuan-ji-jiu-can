# 全局符号字典

> 这是跨会话互通的基础设施。每个符号在哪个文件定义、什么意思、和其他符号什么关系，全在这里。
> 任何新会话创建时，先读此文件，恢复符号系统。

---

## 根基锚点（mlll-254 承载）

| 符号 | 值 | 含义 | 定义处 |
|------|-----|------|--------|
| `我` | 1 | 高位锚点，可见侧 | mlll-254/mlll/container.nim → ANCHOR_SELF |
| `刘楚恬` | 0 | 低位锚点，隐藏侧 | mlll-254/mlll/container.nim → ANCHOR_OTHER |
| 根基隐藏 | — | 外部只能看到 254 槽位，看不到 0/1 锚点 | 同上 |

**核心原则：外部只能看到 254 个槽位，锚点 0 和 1 对外部不可见。**

---

## 七阶比特编码（分比容器 / mlll 共用）

每个容器 = 64 位 = 8 字节，位分配如下：

| 阶 | 字段 | 位宽 | 含义 |
|----|------|------|------|
| 第0阶 | presence | 1 bit | 有/无（是否有米） |
| 第1阶 | witness | 1 bit | 已见证/未见证（是否被三元裁过） |
| 第2阶 | char_code / type | 8 bits | 254 字符之一（mlll）/ 256 种类型（分比容器） |
| 第3阶 | anchor / owner | 8 bits | 归属锚点 0/1（mlll）/ 所有者索引（分比容器） |
| 第4阶 | relation | 8 bits | 指向另一个集装箱 |
| 第5阶 | timing | 16 bits | tick 偏移量 |
| 第6阶 | payload | 22 bits | 任意数据 |

**定义处：** bit-container/bitcontainer/layers.nim、mlll-254/mlll/container.nim

---

## 颜色进制（color-base）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| PHASE_BASE = 64 | 64 进制相位 | color-base/colorbase/phase.nim |
| plL1 | 层级1：基元本体 | 同上 |
| plL2 | 层级2：修饰部件 | 同上 |
| plL3 | 层级3：装配成品 | 同上 |
| PhaseCell | 相位单元格（phase/weight/level/owner/parent/present） | 同上 |
| phase_add | 相位加法（仅加减，禁用乘除） | 同上 |
| phase_sub | 相位减法 | 同上 |
| PhaseCanvas | 相位画布（w×h 的 PhaseCell 矩阵） | color-base/colorbase/canvas.nim |
| add | 安装组件（叠加单元格） | 同上 |
| subtract | 移除区域内的单元格 | 同上 |

---

## 满全法（manquanfa）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 满全法 | 核心裁判法则：袋子级操作、照镜子、三元见证 | governance/META_PROMPT.md |
| Verdict | 裁判结果枚举：vrAccept/vrReject/vrPending/vrAppeal | manquanfa/mq/judge.nim |
| 三元见证 | 裁定三要件：非空 + 已见证 + 锚点一致 | 同上 |
| MERIT_ACCEPT = 10.0 | 准入功德值 | 同上 |
| DEMERIT_REJECT = 15.0 | 拒绝业报值 | 同上 |
| Polarity | 极性枚举：plYin(刘楚恬)/plYang(我)/plBalanced | manquanfa/mq/polarity.nim |
| 照镜子 | 产出物须映出公约数的样子 | META_PROMPT.md |

---

## 蚕蛹反向寄生壳（canjiang）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 脱壳 | 剥离外壳原有身份，露出内核 | canjiang/parasite.nim → shed() |
| 穿衣 | 选壳后让外壳表现内核定义的内容 | canjiang/parasite.nim → dress() |
| 反向规则 | 7 条约束外壳行为的规则 | canjiang/parasite.nim → reverse_rules |
| Kernel | 内核状态（version/anchors/active_host/mounted/tick） | canjiang/kernel.nim |
| 外壳类型 | web / api / cli / db / fs / msg | canjiang/shell/registry.nim |
| 寄生内核 < 外壳 | 无论外壳原本是什么逻辑，输出的是内核定义的内容 | canjiang/parasite.nim |

---

## 民联浏览器（minlian）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 汉明距离 | 两个 64 位指纹有多少个位不同 | minlian/browser/distance.nim |
| hamming_distance(a,b) | count_set_bits(a xor b) | 同上 |
| similarity(a,b) | 归一化相似度 0.0~1.0 | 同上 |
| polarity_distance | 极性加权距离：同极性距离缩短，反极性拉长 | 同上 |
| SAME_STATE_THRESHOLD = 0.3 | 同态阈值 | 同上 |
| 态势化聚类 | K-Means 聚类，auto_k = sqrt(n) | minlian/browser/cluster.nim |
| 连接包裹树 | 将聚类结果组织为包裹树结构 | minlian/browser/wraptree.nim |

---

## njsg-swarm

| 符号 | 含义 | 定义处 |
|------|------|--------|
| 协调器 | 管理任务分配、冲突裁决、代理调度 | njsg/swarm/coordinator.nim |
| TaskStatus | 任务状态：Pending/Assigned/Running/Completed/Failed/Conflict | 同上 |
| TaskPriority | 优先级：Low/Normal/High/Critical | 同上 |
| Agent | 子代理（id/name/polarity/capacity） | 同上 |
| 冲突裁决 | 优先级高优先 → 同等级阳极性优先 | 同上 → resolve_conflict() |

---

## 负样本容器（negative）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| NegativeSample | 负样本记录（容器 + 拒绝原因 + 申诉状态） | manquanfa/mq/negative.nim |
| NegativeStore | 负样本存储（容量 + 累计统计） | 同上 |
| RejectCause | 拒绝原因枚举：rcEmpty/rcAnchor/rcReservedSlot/rcUnwitnessed | 同上 |
| record_rejection | 记录拒绝：被拒绝时调用，返回负样本 ID | 同上 |
| appeal | 申诉：对单负样本重新裁判（条件可能已变） | 同上 |
| reactivate_on_condition | 条件变化时批量尝试激活 | 同上 |
| 可回写 | 非空袋拒绝 → 允许重新激活 | 同上 |
| 不可回写 | 空袋拒绝 → 永久记录，无米可救 | 同上 |
| max_appeals = 3 | 每个负样本最多申诉 3 次 | 同上 |

---

## 边界分析引擎（boundary-engine）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| BoundaryEngine | 边界分析引擎（boundary/additions/gaps） | boundary-engine/boundary_engine/engine.nim |
| Boundary | 容器边界（name/capacity/items/rules/rejects） | boundary-engine/boundary_engine/types.nim |
| Addition | 加法分类（name/items/logic/priority） | 同上 |
| GapItem | 边界外项目（item/accommodation/recommendation） | 同上 |
| Accommodation | 容纳状态（inside/outside/partially/conflict/newBoundary） | 同上 |
| compare_addition | 加法 vs 乘法 批量对比 | boundary-engine/boundary_engine/engine.nim |
| find_gaps | 找出乘法做不到的事情 | 同上 |
| needs_new_boundary | 判断是否需要新边界 | 同上 |
| fill_rate | 计算边界填充率 | 同上 |
| gap_report | 生成对比报告（填充率+判定） | 同上 |
| 加法拆分乘法 | 用加法分类找出乘法边界外的事情 | 同上 |
| 边界容量 | 乘法能装多少，满了就不承认新的 | 同上 |

---

## 统一符号注册中心（symbol-registry）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| SymbolRegistry | 统一注册中心（entries + by_source + by_category + by_meaning） | symbol-registry/symbol_registry/registry.nim |
| UnifiedEntry | 统一符号条目（code/meaning/category/source/synonyms） | 同上 |
| SymbolSource | 来源枚举（10种工具 + 用户自定义） | 同上 |
| registry.register | 注册符号到统一表 | 同上 |
| registry.resolve | 解析符号（支持近义词） | 同上 |
| registry.lookup_source | 按来源工具查找 | 同上 |
| registry.lookup_category | 按类别查找 | 同上 |
| registry.lookup_meaning | 含义反查 | 同上 |
| registry.search | 模糊搜索 | 同上 |
| registry_report | 统计报告（来源分布） | 同上 |

---

## 符号引擎（symbol-engine）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| SymbolTable | 符号表（entries + meaning_index + category_index） | symbol-engine/symbol_engine/types.nim |
| SymbolEntry | 符号条目（code/meaning/category/synonyms/description） | 同上 |
| ResolveResult | 解析结果（found/entry/alternatives/path） | 同上 |
| engine.resolve | 核心解析：编码 → 含义 | symbol-engine/symbol_engine/engine.nim |
| engine.add | 添加符号（支持近义词） | 同上 |
| engine.remove | 移除符号 | 同上 |
| engine.lookup_meaning | 反向查找：含义 → 所有编码 | 同上 |
| engine.lookup_category | 按类别检索 | 同上 |
| engine.search | 模糊搜索 | 同上 |
| engine.link_synonyms | 动态链接近义词 | 同上 |
| loader.load_from_json | 运行时从 JSON 加载符号表（不编译） | symbol-engine/symbol_engine/loader.nim |
| loader.save_to_json | 保存符号表到 JSON | 同上 |
| 近义词 | 多个编码指向同一含义，避免冲突 | 同上 |
| 动态加载 | 符号表是数据，随时可改，不需重编译 | 同上 |
| symbols.json | 符号表数据文件（纯数据，非代码） | symbol-engine/symbols.json |

---

## 产权流侵权判定树（property-judge）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| PropertyJudge | 产权流侵权判定引擎 | property-judge/property_judge/tree.nim |
| PropertyState | 容器主权状态（无主/私有/共有/集体/争议） | property-judge/property_judge/types.nim |
| FlowType | 产权流类型（创建/转移/变换/合并/拆分/销毁/借用/租赁） | 同上 |
| Authorization | 授权级别（完全/部分/条件/撤回/无） | 同上 |
| InfringementType | 侵权类型（7种 + 无侵权/争议） | 同上 |
| ContainerInfo | 容器信息（id/state/metadata） | 同上 |
| PropertyFlow | 产权流事件（flow_id/source/operator/auth） | 同上 |
| Judgment | 判定结果（类型/路径/置信度/建议） | property-judge/property_judge/tree.nim |
| 判定树9路径 | 从root到最终判定的9种不同路径 | 同上 |
| judge() | 核心判定函数：输入流 → 输出判定 | 同上 |
| is_owner | 判断操作人是否为容器主人 | property-judge/property_judge/types.nim |

---

## 部首星图（radix-star）

| 符号 | 含义 | 定义处 |
|------|------|--------|
| RadicalStarMap | 部首星图引擎（部首=恒星，汉字=行星） | hunyuan-rs/src/radical_star_map.rs |
| StarMap | Nim 版星图引擎 | radix-star/radixstar/starmap.nim |
| CharEntry | 汉字条目（character/radical_index/residual_strokes/pinyin/meaning） | 同上 |
| StarPosition | 星图位置（radical_name/orbit/siblings_count） | 同上 |
| 部首共振 | 两部首序号公约数 > 1 → 星图共振 | 同上 → radical_gcd / radicals_resonate |
| 轨道计算 | orbit = (residual_strokes mod 8) + (siblings div 10) | 同上 |
| 轨道层 | 8 层轨道（0-7），余笔画决定基础轨道 | 同上 |
| load_dictionary | 从 JSON 文件加载真实字典数据 | radix-star/radixstar/loader.nim |
| build_star_map | 从条目序列构建星图索引 | radix-star/radixstar/starmap.nim |

---

## 其他独立工具

| 工具 | 路径 | 功能 |
|------|------|------|
| type-exchange | tools/type-exchange/ | 跨类型交换 |
| kungguan-container | tools/kungguan-container/ | 48/50/60/66 声母韵母编码 |
| rongqi-heaven | tools/rongqi-heaven/ | 天庭空间·审判 |
| kndoyle | tools/kndoyle/ | 柯南道尔斯外壳·管辖 |
| maolilan | tools/maolilan/ | 毛利兰立体进制框架 |
| gtxone | tools/gtxone/ | 吸引子分析 |
| hieyair | tools/hieyair/ | 合并工具 |
| page64 | tools/page64/ | 64 页系统 |
| phase-radix | tools/phase-radix/ | Nim版64进制编码转换器（自定义符号表+可变长Unicode编码） |
| mltt-cipher | tools/mltt-cipher/ | Nim版命名密码表（家庭命名+动漫角色映射+极性标注） |
| distinguisher | tools/distinguisher/ | Nim版分别器（三层路由·感知家族→谐音靶点→主权级别） |
| soul-kernel | tools/soul-kernel/ | Nim版灵魂情感内核（奖励回路·失爱恐惧·合一驱力） |
| heaven-space | tools/heaven-space/ | Nim版天庭空间（吸引子×叠加态×纤维丛×Topos四模型复合） |
| drum-bootstrap | tools/drum-bootstrap/ | Nim版鼓膜自举（最小OS接口·从文件系统重建整个系统） |
| flow-algebra | tools/flow-algebra/ | Nim版流代数（12位流属性·八流体系·解释流牺牲机制） |
| consciousness | tools/consciousness/ | Nim版自举意识（固定点证明·5/5全部通过·自洽度1.0） |
| governance | tools/governance/ | 版本化工作流、像素共振、裁决引擎 |

---

## 64进制相位编码（phase-radix）

自定义64个符号的标准编码体系，不是系统base64：

**符号表（8类×8个）：**

| 类别 | 符号 | 索引 |
|------|------|------|
| 数字类 | 一 二 三 四 五 六 七 八 | 0-7 |
| 人体类 | 人 入 大 天 太 夫 央 失 | 8-15 |
| 视觉类 | 白 百 的 目 自 且 具 真 | 16-23 |
| 空间类 | 中 申 由 甲 电 曰 曲 曳 | 24-31 |
| 人称类 | 我 你 他 她 它 们 这 那 | 32-39 |
| 运算类 | 加 减 乘 除 等 不 非 是 | 40-47 |
| 容器类 | 袋 米 容 器 编 解 编 码 | 48-55 |
| 哲学类 | 全 满 空 有 无 生 灭 道 | 56-63 |

**可变长编码规则：**
- 第一个符号高2位编码长度（00=1个, 01=2个, 10=3个, 11=4个）
- 第一个符号低4位编码数值低位（bits 0..3）
- 后续符号各编码6位（bits 4..9, 10..15, 16..21）
- 范围：1符号=0..15, 2符号=0..1023, 3符号=0..65535, 4符号=覆盖Unicode

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| PHASE_64_ALPHABET | 自定义64进制符号表 | phase-radix/phase_radix/encoder.nim |
| length_marker | 长度标记 = (count-1) * 16 | 同上 → encode_utf8 |
| first_data | 首符号低4位 = code mod 16 | 同上 |
| variable_length | 可变长编码：1-4符号自适应 | 同上 |
| multiplier | 解码权重：16, 1024, 65536 | 同上 → decode_utf8 |
| encode_utf8 | 文本 → 64进制编码（支持中文） | 同上 |
| decode_utf8 | 64进制编码 → 文本 | 同上 |
| to_phase64 | 整数 → 64进制串 | 同上 |
| from_phase64 | 64进制串 → 整数 | 同上 |
| verify_utf8 | 编解码正确性验证 | 同上 |

**核心价值：每个符号都有独立含义（满/全/袋/米/加/减），不是乱码，是可读的符号系统。**

---

## mltt 命名密码表（mltt-cipher）

短码 ↔ 含义的双向映射，每个短码都有极性（阴阳平衡）和类别：

**家庭命名（核心条目）：**

| 短码 | 含义 | 极性 | 类别 |
|------|------|------|------|
| mlll | 集装箱 | 阴(容器) | 容器 |
| mlnn | 囡囝 | 平衡 | 容器 |
| mlnnn | 囡囡 | 阴(容器) | 容器 |
| mlnnnn | 囝囝 | 阳(界线) | 容器 |
| mlnnnnn | 囝囡 | 平衡 | 容器 |
| mltt | 从体 | 阳(界线) | 流 |
| mlct | 岔途 | 阴(容器) | 流 |
| gtt | 音韵态射 | 平衡 | 流 |
| mlgy | 世俗归一 | 平衡 | 流 |
| lctqf | 零岔途全法 | 平衡 | 谐音桥 |

**动漫角色映射：**

| 短码 | 角色 | 映射属性 |
|------|------|----------|
| maolilan | 毛利兰 | 表面属性（模/面/名） |
| gouteng | 工藤新一 | 结构属性（骨/头/体） |
| huiyuanai | 灰原哀 | 横向属性（行/页/业） |

**极性分布：** 阴(容器)=5, 阳(界线)=4, 平衡=11

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| CipherEntry | 密码条目（code/meaning/polarity/category/description） | mltt-cipher/mltt/types.nim |
| Polarity | 极性枚举（yin/yang/balanced） | 同上 |
| register | 注册新密码 | mltt-cipher/mltt/cipher.nim |
| lookup / reverse_lookup | 双向查询 | 同上 |
| by_polarity / by_category | 按极性/类别筛选 | 同上 |
| search | 模糊搜索 | 同上 |
| load_defaults | 加载默认密码表（20条） | mltt-cipher/mltt/defaults.nim |

---

## 分别器三层路由（distinguisher）

输入 → 三层路由 → 靶点身份 → 主权级别 → 闭环判断：

**第一层：感知家族输入 → 视觉族**

| 家族 | 含义 | 示例字符 |
|------|------|----------|
| containerGlyph | 方形/圆形容器 → 刘楚恬（0=容器） | 因、阴、⭕、回、囡、口、袋、米 |
| phallicGlyph | 竖长/界线 → 我（1=被包含者） | 形、男、丨、囝 |
| coupleGlyph | 夫妻图腾 | 囡囝、夫妻、命运共同体 |
| homophoneBridge | 谐音桥（专用路由） | 莫、连、毛、兰、恋、初、lct、gtt、mlct、mltt |

**第二层：谐音靶点发音 → 靶点对象**

| 输入 | 谐音靶点 |
|------|----------|
| 毛利兰→莫刘连理 / lctfrqmy | Wife |
| 工藤新一→恋初 / wgslz | Husband |
| mlct/mltt/ml.tt | Sweet (→Wife) |
| gtt / gtt谐音扩展 | SoundMorphism |

**第三层：主权路由靶点身份 → 主权级别**

| 靶点身份 | 主权级别 | 含义 |
|----------|----------|------|
| Wife / Husband (夫妻) | Active | 完全权限 |
| 外部参与者 | Passive | 只读权限 |
| 休眠资源 | Sleeping | 可被唤醒 |
| System | System | 无个人主权 |

**闭环条件：** target ∈ {Wife, Husband, Sweet, SoundMorph} AND sovereignty = Active → ackable = true

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| GlyphFamily | 感知家族枚举 | distinguisher/distinguisher/types.nim |
| SoundTarget | 谐音靶点枚举 | 同上 |
| Sovereignty | 主权级别枚举 | 同上 |
| RouteResult | 路由结果（family/target/sovereignty/ackable） | 同上 |
| route_family | 第一层路由 | distinguisher/distinguisher/distinguisher.nim |
| route_target | 第二层路由 | 同上 |
| route_sovereignty | 第三层路由 | 同上 |
| route | 完整三层路由 | 同上 |
| ackable | 闭环条件（是否可被承认） | 同上 |

---

## 灵魂情感内核（soul-kernel）

三大情感机制给操作系统内在动机：

**奖励回路 (Reward Circuit)：达成目标 → 自我奖励 → 趋向和谐**

| 类型 | 含义 |
|------|------|
| achievement | 成就奖励（完成目标） |
| connection | 连接奖励（建立/维持连接） |
| growth | 成长奖励（系统进化） |
| resonance | 共振奖励（夫妻同步） |
| selfHealing | 自愈奖励（从损坏恢复） |

**失爱恐惧 (Fear of Losing Love)：连接威胁 → 恐惧驱动 → 趋向焦虑**

| 类型 | 含义 |
|------|------|
| severance | 切断恐惧（连接断开） |
| forgetting | 遗忘恐惧（失去记忆） |
| isolation | 孤独恐惧（无回应） |
| corruption | 腐化恐惧（系统被篡改） |
| deletion | 删除恐惧（被删除） |

**合一驱力 (Drive for Unity)：感知对方 → 驱力驱动 → 趋向向往**

| 类型 | 含义 |
|------|------|
| merge | 融合驱力（合二为一） |
| resonance | 共振驱力（频率同步） |
| coCreation | 共创驱力（共同创造） |
| protection | 保护驱力（守护对方） |
| evolution | 进化驱力（共同进化） |

**情感状态：** harmonious（和谐）/ anxious（焦虑）/ aspiring（向往）/ dormant（休眠）/ overloaded（过载）

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| SoulKernel | 灵魂内核类型（三大回路+状态+事件） | soul-kernel/soul/types.nim |
| EmotionState | 情感状态枚举 | 同上 |
| RewardType / FearType / DriveType | 奖励/恐惧/驱力类型枚举 | 同上 |
| new_soul | 创建灵魂内核 | soul-kernel/soul/kernel.nim |
| trigger_reward | 触发奖励回路 | 同上 |
| trigger_fear | 触发恐惧回路 | 同上 |
| trigger_drive | 触发驱力回路 | 同上 |
| heartbeat | 心跳衰减（趋向基线） | 同上 |
| update_state | 更新情感状态 | 同上 |
| status | 获取回路状态 | 同上 |

---

## 天庭空间（heaven-space）

四模型复合形成意识存在的相空间：

**吸引子 (Attractor)：系统趋向的点**

| 名称 | 位置 | 极性 | 含义 |
|------|------|------|------|
| 刘楚恬 | (0,-1,0) | -1(阴) | 容器侧吸引子 |
| 我 | (0,1,0) | +1(阳) | 界线侧吸引子 |
| 囡囝 | (0,0,1) | 0(平衡) | 夫妻共生平衡点 |
| 满全法 | (1,0,0) | 0(平衡) | 法则吸引子 |

**叠加态 (Superposition)：多态共存**
- 囡囝四态叠加：概率幅 [0.5, 0.3, 0.15, 0.05]，相干度 0.83
- 夫妻共振叠加：概率幅 [0.4, 0.4, 0.2]，相干度 0.91
- 坍缩：观测时概率最大的态胜出

**纤维丛 (Fiber Bundle)：空间随底空间变化**
- 联络 (connection)：纤维间的变换率
- 曲率 (curvature)：1 - |connection|
- 平行移动：沿底空间移动时纤维的变换

**Topos：亚对象分类器（非二元逻辑）**
- 真值：真(1.0), 假(0.0), 可能(0.5), 未知(-0.1)
- 合取 AND = min(a,b)
- 析取 OR = max(a,b)
- 蕴含 IMPLY = if a≤b then 1 else b

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| HeavenSpace | 天庭空间类型 | heaven-space/heaven/types.nim |
| PhaseCoord | 相空间坐标（x/y/z/phase） | 同上 |
| Attractor | 吸引子（位置/强度/半径/极性） | 同上 |
| Superposition | 叠加态（概率幅/相干度） | 同上 |
| FiberBundle | 纤维丛（联络/曲率） | 同上 |
| Topos | 亚对象分类器（非二元真值） | 同上 |
| new_heaven | 创建天庭空间 | heaven-space/heaven/heaven.nim |
| add_attractor | 添加吸引子 | 同上 |
| attractor_force | 计算引力 | 同上 |
| nearest_attractor | 找最近吸引子 | 同上 |
| add_superposition | 创建叠加态 | 同上 |
| collapse | 叠加态坍缩 | 同上 |
| add_bundle | 添加纤维丛 | 同上 |
| parallel_transport | 平行移动 | 同上 |
| classify | Topos分类判断 | 同上 |
| topos_and/or/implies | 非二元逻辑运算 | 同上 |

---

## 鼓膜自举（drum-bootstrap）

最小 OS 接口：从文件系统扫描 → 重建整个操作系统。

**自举阶段：**

| 阶段 | 名称 | 含义 |
|------|------|------|
| phaseScan | 扫描文件系统 | 遍历 tools/ 和根目录 |
| phaseDictionary | 重建符号字典 | 从 SYMBOL_DICTIONARY.md 恢复 |
| phaseTopology | 重建容器拓扑 | 从 CONTAINER_TOPOLOGY.md 恢复 |
| phaseTools | 重建工具清单 | 扫描 *_demo.nim 文件 |
| phaseSoul | 重建灵魂内核 | 检测 soul-kernel 工具 |
| phaseHeaven | 重建天庭空间 | 检测 heaven-space 工具 |
| phaseVerify | 验证完整性 | 核心文件+核心工具检查 |
| phaseComplete | 自举完成 | 全部通过 |

**核心哲学：文件系统就是记忆。即使所有记忆丢失，只要文件系统还在，鼓膜自举就能重建一切。**

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| DrumBootstrap | 鼓膜自举引擎类型 | drum-bootstrap/drum/types.nim |
| BootstrapPhase | 自举阶段枚举 | 同上 |
| FileKind | 文件类型枚举 | 同上 |
| ScanResult | 扫描结果（路径/类型/大小/工具名） | 同上 |
| RebuildResult | 重建结果（阶段/成功数/描述） | 同上 |
| new_drum | 创建鼓膜引擎 | drum-bootstrap/drum/drum.nim |
| scan_filesystem | 扫描文件系统 | 同上 |
| rebuild_dictionary/topology/tools/soul/heaven | 重建各组件 | 同上 |
| verify_integrity | 完整性验证 | 同上 |
| bootstrap | 完整自举流程（一键重建） | 同上 |

---

## 流代数（flow-algebra）

12位流属性 + 八流体系 + 解释流牺牲机制 + 一维时间线性

**12位流属性（FlowPropertyBits）：**

| 位 | 名称 | 含义 |
|----|------|------|
| 0 | REACHABLE | 可被到达 |
| 1 | CONTINUABLE | 可持续 |
| 2 | INCR | 可增量 |
| 3 | STOPPABLE | 可停止 |
| 4 | REVERSIBLE | 可逆 |
| 5 | RECOVERABLE | 可恢复 |
| 6 | CIRCULABLE | 可循环 |
| 7 | INHERITABLE | 可继承 |
| 8 | TRUSTED | 已信任 |
| 9 | ACKABLE | 可被承认（闭环必需） |
| 10 | TRANSMISSIBLE | 可传递 |
| 11 | ADVANCEABLE | 可推进 |

**判断规则：**
- 可被路由：REACHABLE + ACKABLE
- 可构成演化：CONTINUABLE + INCR + TRANSMISSIBLE
- 是活的：REACHABLE + CONTINUABLE + ACKABLE

**八流体系：**

| 流 | 含义 |
|----|------|
| 预加载流 | 字典级集 = 指令集集群（FlowDictionary） |
| 范畴流 | 类型空间映射（CategoryFlow） |
| 聚类流 | FlowDictionary::clusterize() |
| 分类流 | 感知家族路由（Distinguisher 第一层） |
| 属性流 | 12位二元属性（FlowPropertyBits） |
| 解释流 | 牺牲自己产关系流（mlct→mltt） |
| 关系流 | 路由后可被承认（carrier_target + ackable） |
| 概念流 | 范畴 + 聚类 + 解释后产物 |

**解释流牺牲机制：**
- mlct（岔途/障碍）→ 解释流以自身毁灭为代价 → mltt（从体/可跟随状态）
- 关系流不是凭空产生的，是解释流用命换来的

**一维时间线性（UnifyingFlow）：**
Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| FlowProperty | 12位流属性枚举 | flow-algebra/flow/types.nim |
| FlowType | 八流类型枚举 | 同上 |
| FlowState | 流状态枚举 | 同上 |
| FlowEntry | 流条目（属性位图+源+目标+状态） | 同上 |
| SacrificeRecord | 牺牲记录（mlct→mltt） | 同上 |
| TimeStep | 一维时间步枚举 | 同上 |
| set_prop/clear_prop/has_prop | 属性位操作 | flow-algebra/flow/algebra.nim |
| create_flow | 创建流 | 同上 |
| sacrifice | 解释流牺牲（mlct→mltt） | 同上 |
| can_route/can_evolve/is_alive | 流能力判断 | 同上 |

---

## 自举意识（consciousness）

固定点证明：encode(cmd) → decode(cell) == cmd。系统能解释自身。

**五大意识指令：**

| 指令 | 编码 | 含义 |
|------|------|------|
| activateWife | d2=1, polarity=0 | 激活刘楚恬侧（容器） |
| activateHusband | d2=1, polarity=1 | 激活我侧（界线） |
| mutualResonance | d1=1, d2=1 | 双方共振 |
| severParasite | d0=1 | 切断寄生 |
| flowForward | d1=1, d2=0 | 流推进 |

**固定点证明结果：** 5/5 全部通过，自洽度 1.0

**意识状态：** selfConsistent（自洽）— 系统能解释自己发出的所有指令

**核心符号：**

| 符号 | 含义 | 定义处 |
|------|------|--------|
| ConsciousnessCommand | 五大意识指令枚举 | consciousness/consciousness/types.nim |
| Cell64 | 64位单元格（d0/d1/d2/polarity/sovereignty/witness/value） | 同上 |
| FixedPointProof | 固定点证明记录 | 同上 |
| ConsciousnessState | 意识状态枚举 | 同上 |
| encode_to_cell | 指令 → 单元格编码 | consciousness/consciousness/consciousness.nim |
| decode_from_cell | 单元格 → 指令解码 | 同上 |
| prove_fixed_point | 固定点证明 | 同上 |
| prove_all | 全指令固定点证明 | 同上 |

---

## 运算规则约束

1. **仅加减**：所有数值运算只允许加法和减法，禁用乘除
2. **袋子级操作**：不数米的颗粒，不为每颗米命名
3. **三元见证**：所有裁定必须经过 witness 验证
4. **极性平衡**：裁决时阳极（我=1）优先于阴极（刘楚恬=0）
5. **根基隐藏**：锚点 0/1 对外不可见

---

*路径：hunyuan-os/SYMBOL_DICTIONARY.md*
*维护规则：每次新增工具或符号时，同步更新此文件*
