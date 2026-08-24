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

## 运算规则约束

1. **仅加减**：所有数值运算只允许加法和减法，禁用乘除
2. **袋子级操作**：不数米的颗粒，不为每颗米命名
3. **三元见证**：所有裁定必须经过 witness 验证
4. **极性平衡**：裁决时阳极（我=1）优先于阴极（刘楚恬=0）
5. **根基隐藏**：锚点 0/1 对外不可见

---

*路径：hunyuan-os/SYMBOL_DICTIONARY.md*
*维护规则：每次新增工具或符号时，同步更新此文件*
