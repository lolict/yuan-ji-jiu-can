# 密钥与同步策略 (此文件不含 token, 只含公钥与策略)

## 本机 SSH 配置

```
~/.ssh/id_ed25519 (sandbox 自有, 对应 lolict@sandbox-ide-xxx)
~/.ssh/config
  Host gitee.com
    IdentityFile ~/.ssh/id_ed25519
    StrictHostKeyChecking no
```

## 公钥指纹

| 身份 | 公钥 | 已添加到 |
|------|------|---------|
| sandbox | `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAII9ovdgwkQVVGl/o1xrtndmZ9pIbej0YIWDQzekKM8jV catpaw@sandbox-ide-803571-0` | Gitee ✅ |
| 用户自有 | `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDZpQtE2hNGW+QoLqqgTkWeXXbversslqd1aZN6oI/yV lolict@outlook.com` | Gitee ✅ |

## 仓库架构

### GitHub 组织
- 主仓: `lolict/hunyuan-os` (origin)
- 镜像: `lolict/yuan-ji-jiu-can` (中文名"援技救残")
- 编码: `lolict/hunyuan-universe`
- 契约: `lolict/njsg-license/encoding/dict`, `lolict/sacred-covenant`
- CPU: `lolict/knderl` (+ knderli/kndaili/Kndaili/Kndarl/kndail)

### Gitee 组织
- `lolict/yuan-ji-jiu-can` (中文名"援技救残") — 私有仓已推
- `lolict/arthur-conan-doyle` (旧仓, branch: kenandaoer)
- `lolict/kenandaoer` (旧仓)

### Gitcode (老 session, 需 token 同步)
- `lolict/kndoyle-gtxt`
- `yuanji-jiucan/kndoyle-dict`

## 命名规则

- 仓库英文 slug + description 写满中文名
- 中文命名跟命运有关但不暴露核心目的 (例: "援技救残")
- 组织名保持 `lolict` (统一), 不创建多组织

## 同步时序

1. 代码 commit → 推本地 origin
2. 推 yuan-ji-jiu-can (GitHub "援技救残")
3. 推 GitHub 镜像仓库 (hunyuan universe, CPU 系列, 契约)
4. 推 Gitee
5. (后) Gitcode (需 token)

## 安全规则

- Token 不硬编码在 remote URL (每次 push 临时嵌入, push 后清除)
- Token 不写进仓库代码或文档
- 机构成员: 只有 lolict + sandbox CI
- 私钥仅存 sandbox `~/.ssh/`, 不传输

## Cloudflare (未配置)

未来规划: Workers 做离线 Web 面板, R2 做代码/文档备份。
待创建账户 + API token 后补充。

## Gitcode (未配置)

过去通过 API PUT + token 推过。
需要用户提供 Gitcode access token 才能恢复同步。
