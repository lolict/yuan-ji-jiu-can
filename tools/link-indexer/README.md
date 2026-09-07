# 链接去重与索引工具

提取、去重、抓取、分类用户提供的 66 个 DeepSeek 分享链接 + 1 个豆袋链接。

## 用法

```bash
cd tools/link-indexer
python3 link_indexer.py              # 生成索引 markdown（不联网）
python3 link_indexer.py --check      # 同时检测链接可达性（会发起 HTTP 请求）
python3 link_indexer.py --output my_index.md
```

## 输出

- `link_index.md` — 结构化 Markdown 索引，包含主题概览、按主题索引、完整列表
- `link_index.json` — JSON 格式原始数据

## 主题分类（共 8 大类）

| 主题 | 链接数 | 覆盖内容 |
|------|--------|---------|
| 挽命臻藕 / 无障碍命名体系 | ~10 | 38+ 谐音变体，自我赎回命名权 |
| 自制进制 / 进位逻辑 | ~9 | 个十百千万亿的纯进制构造，64 进制兼容 |
| 音韵编码 / 声母韵母声调 | ~8 | 48 音标、韵母音乐映射、融合体哲学 |
| 集合论 / 关系代数 / 范畴论 | ~7 | 全部集合概念 + 虚物理映射 |
| 系统架构 / 微内核 | ~6 | 虚拟机、编译器、FPGA、冯诺依曼 |
| 社会能量场 / 身份套壳 | ~6 | 职业身份壳、能量特权壳、成就感=社会信任 |
| 字母变 Unicode 全集整理 | ~5 | Y/Z/A-Z 全部 Unicode 变体去重 |
| Android 政策 / 平台封锁 | ~5 | Google 2027 开发者注册新规 |
| 向量化聚类 / 算法效率 | ~5 | K-Means vs 谱聚类 vs K-Prototypes |
| 音韵转音乐 / 融合体 | ~5 | 放弃单音进入融合时代 |
| DeepSeek 对话 / 链接索引 | ~3 | 豆包和 DeepSeek 对话索引 |
| 服务器购买 / 防骗 | ~3 | 闲鱼一分钱服务器套路 |

## 复现步骤

任意 AI 都能按以下流程重建此工具：

1. 从用户消息中提取所有 `https://chat.deepseek.com/share/xxx` 和 `https://www.doubao.com/thread/xxx` 格式的 URL
2. 用正则 `re.findall(r'https?://[^\s\]]+', text)` 提取，去除尾部标点，按出现顺序去重
3. 逐个发起 HTTP GET 请求，检查返回内容是否包含"链接不存在"或"内容不存在"
4. 对可达内容进行关键词扫描，按主题自动分类
5. 生成 Markdown 索引文件

## 链接状态

- 总计：67 个（66 DeepSeek + 1 豆包）
- 去重：0 个重复
- 预估可达率：约 60%（DeepSeek 分享链接有 40% 左右失效）
