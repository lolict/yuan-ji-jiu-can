# 仓库清单 · repo-inventory

> 自动生成: 2026-09-18 | 基于 `git remote -v`

## 唯一仓库 (9个)

| # | 代号 | 仓库名 | 平台分布 | 备注 |
|---|------|--------|----------|------|
| 1 | HY | hunyuan-os | github+gitcode+gitee | 主仓库·三镜像 |
| 2 | YJ | yuan-ji-jiu-can | github+gitee×2 | 鸳鸯谐韵·双gitee冗余 |
| 3 | ACD | arthur-conan-doyle | gitee | 柯南道尔·壳公司 |
| 4 | KND | kenandaoer | gitee | 柯南道尔·大陆版 |
| 5 | HU | hunyuan-universe | github | 宇宙观协议 |
| 6 | MQD | manquanfa-dict | github | 满全法字典 |
| 7 | MQE | manquanfa-encoding | github | 满全法编码 |
| 8 | MQL | manquanfa-license | github | 满全法许可 |
| 9 | SC | sacred-covenant | github | 神圣契约 |

## 远程别名映射

| 仓库 | origin(主) | 镜像1 | 镜像2 |
|------|-----------|-------|-------|
| hunyuan-os | origin(github) | gitcode | gitee-hunyuan |
| yuan-ji-jiu-can | yuan-ji-jiu-can(github) | gitee | gitee-yj |
| hunyuan-universe | github | — | — |
| manquanfa-{dict,encoding,license} | github | — | — |
| sacred-covenant | github | — | — |
| arthur-conan-doyle | gitee-acd | — | — |
| kenandaoer | gitee-ken | — | — |

## 备份策略

- **Tier 1** (三镜像): hunyuan-os — 任一平台故障不影响代码安全
- **Tier 2** (双镜像): yuan-ji-jiu-can
- **Tier 1.5** (单github): MU/SC/ACND/KND/HU — 依赖 github 单点

## push-all.sh 对应表

→ 见 `tools/sync_remotes.sh` (已有)
→ 新增: `tools/push-all.sh` (新建·全仓库推送)

---

砖块: #71 @repo-inventory
