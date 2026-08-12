# 2026-08-03 · 推送凭据与仓库状态 (Private)

## 仓库状态

| 平台 | 仓库名 | 远程 | 状态 |
|------|--------|------|------|
| GitHub | hunyuan-os | origin (HTTPS) | ✅ v0.5 已推 |
| GitHub | yuan-ji-jiu-can ("援技救残") | yuan-ji-jiu-can | ✅ v0.5 已推 (description 写中文名) |
| Gitee | 未建仓 | gitee (SSH, 指向不存在的 hunyuan-os) | ❌ 需用户在 gitee.com 建私有仓 |

## 用户决定: 仓库命名原则

- 用中文, 不用英文
- 名称要有意义, 跟我们的命运相关, 不需故意隐瞒
- 不要暴露个人创新/智慧/意图的最终目的
- 最终选定: "援技救残" (4 字)
- Description 用泛泛技术语言: "分布式系统、算力调度与节点协同的操作系统技术研究项目"
- 技术含义暗合 fractal.rs 残差偏移量 / 漏斗坍缩理念

## GitHub Pat (35 权限)

- 完整 token: `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` (scope: admin:org, admin:repo_hook, repo, workflow, gist, 等)
- fine-grained PAT `github_pat_11BW5BUEY0...`: 缺 createRepository GraphQL scope → 创建新仓会报 GraphQL 错
- GitHub repo name 只接受 ASCII → slug = `yuan-ji-jiu-can`, description 里写中文名

## Gitee SSH

- sandbox 自有 key: `~/.ssh/id_ed25519` 公钥 `AAAAC3NzaC1lZDI1NTE5AAAAII9ovdgwkQVVGl/o1xrtndmZ9pIbej0YIWDQzekKM8jV`
- 用户自有 key (已加 Gitee): `AAAAC3NzaC1lZDI1NTE5AAAAIDZpQtE2hNGW+QoLqqgTkWeXXbversslqd1aZN6oI/yV` (lolict@outlook.com)
- Gitee SSH 握手成功但 repo 404

## 操作规则

- 每次 push 后用 `git remote set-url origin https://(不带token)` 清 embedded token
- 容器重启 token 不持久, 需用户重给
- 严禁 push --force (本次已经犯了一次, 之后不要再犯)
- 未来新仓库创建通过 GitHub API 完成 (token ghp_MjOEBupl...)

## 未来创建仓库

用户授权: 自己给自己命名, 自己填写组织资料, 但不要暴露个人意图/创新/智慧。
中文优先, 名称跟命运相关但不裸露核心目的。
