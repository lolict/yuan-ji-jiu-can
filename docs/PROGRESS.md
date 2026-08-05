# 囡囝共生 v0.5.1 · 开发进度

## 已完成

### v0.5 底层舞台骨架 (2026-08-03 上午)
- [x] triune.rs — 三元裁判机 (9 tests)
- [x] funnel.rs — 漏斗控制器 (7 tests)
- [x] fractal.rs — 分形调度器 (8 tests)
- [x] relational.rs — 关系代数外壳 (10 tests)
- [x] pleasure.rs — 愉悦动力引擎 (10 tests)
- [x] couple.rs — 夫妻命运共同体 (13 tests)
- [x] eco/ 生态模拟 (物理隐喻) (8 tests)
- [x] 132 tests passing
- [x] 多平台同步 (12+ repos)
- [x] 综合 demo main.rs

### v0.5.1 (2026-08-03 下午)
- [x] Associative Scan (Mamba 风格并行原语)
- [x] ScanTree 树形坍缩结构
- [x] combine_fractal_nodes 漏斗合并
- [x] 总测试数 145 tests passing

## 未开始 / 下一段

### 优先级 P0
- [ ] MCCP 编译器 (MCCP reads MCCP, writes MCCP = 真正自举)
- [ ] IME 打字系统 (Android Kotlin 壳 + Rust 核心)

### 优先级 P1
- [ ] 关系代数引擎第四维主权层 (已定义在 ARCHITECTURE.md)
- [ ] SDF 字体 atlas (已有 font/ v0.3 版本, 需迁移到 Rust)
- [ ] Android 寄生层 (IME + 后台服务)

### 优先级 P2
- [ ] 统计锥可视化 (实时 funnel 坍缩演示)
- [ ] 22亿节点仿真器 (phone layout 已定义, 需模拟)
- [ ] Gitcode 同步 (待 token)

### 优先级 P3
- [ ] Cloudflare Workers 部署 (离线 Web 面板)
- [ ] 自定义字体 SVG 生成 (已有 font_svg.py)

## 架构不变式 (绝不违反)

1. **姓名即协议** — 刘楚恬 + lolict = root of trust
2. **私有优先** — 不依赖公共云, 不上传核心文档
3. **无根权限** — 任何单一节点不拥有全部控制
4. **反公共网络** — 夫妻共同体不在公网暴露
5. **残差守恒** — 分身只记录偏移, 不复制全量
6. **愉悦价值** — 愉悦是基本价值单位, 不是 reward model
7. **法则优先** — 关系代数规则优先于工程便利

## 已知技术债

- Rust eco/ 模块 (物理生态模拟) 方向存疑 — metaphor 已成立, 但长期价值待确认
- IME 层完全未开发 (android/ 目录存 v0.3 Java 原型)
- WASM/WASI 在 IME 场景不可行 (系统级 InputMethodService 不允许沙箱)
