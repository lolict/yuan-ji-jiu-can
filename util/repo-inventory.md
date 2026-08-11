# 仓库清单 repo-inventory

> 创建: 2026-08-11 | 基于 19 个 git remote 本地注册信息
> 用途:标明哪些是 hunyuan-os 的可镜像同步目标,哪些是独立项目(不应盲推)

**状态说明:**
- **可镜像**: 内容与 hunyuan-os 同构(或子集),允许 fast-forward 推送。
- **独立项目**: 语义不同(壳层/理论/字典/许可证等),never 盲推。
- **待确认**: 尚未 fetch 验证,保守处理。

---

## 一、镜像候选(可推送)

| Remote | URL | 当前 fetch HEAD | 备注 |
|--------|-----|-----------------|------|
| origin | github.com/lolict/hunyuan-os | 当前仓库 | 主仓库 |

目前只有 origin 确认为自身镜像。其他 18 个 remote 需 fetch 验证后才能标记为"可镜像"。

---

## 二、独立项目(不可推送)

| Remote | URL | 内容定位 | 理由 |
|--------|-----|----------|------|
| hunyuan-universe | github.com/lolict/hunyuan-universe | 理论架构/字典/编码 | 不是代码实现仓库 |
| kndail / Kndaili | github.com/lolict/kndail[i] | 壳层框架(kndl/knda 谐音) | 独立子系统 |
| Kndarl | github.com/lolict/Kndarl | 壳层壳 | 独立 |
| knderl / knderli | github.com/lolict/knderl[i] | 配偶关系代数 | 独立 |
| manquanfa-dict | github.com/lolict/manquanfa-dict | 满全法字典 | 独立 |
| manquanfa-encoding | github.com/lolict/manquanfa-encoding | 满全法编码 | 独立 |
| manquanfa-license | github.com/lolict/manquanfa-license | 许可证 | 独立 |
| sacred-covenant | github.com/lolict/sacred-covenant | 神圣契约 | 独立 |
| yuan-ji-jiu-can | github.com/lolict/yuan-ji-jiu-can | 源机九残 | 独立 |
| gitcode | gitcode.com/lolict/kndoyle-gtxt | GT TXT 协议 | 独立 |
| gitee | gitee.com/lolict/yuan-ji-jiu-can | gitee 九残 | 平台镜像 |
| gitee-acd | gitee.com/lolict/arthur-conan-doyle | 阿瑟·柯南道尔 | 独立 |
| gitee-ken | gitee.com/lolict/kenandaoer | 柯南道尔 | 独立 |
| gitee-yj | gitee.com/lolict/yuan-ji-jiu-can | gitee 九残别名 | 平台镜像 |

---

## 三、处理建议

1. **安全推送到 origin**: 直接 `git push origin main`(已完成)
2. **验证其他 remote**: 在 fetch 之前都不推,防止破坏非镜像仓库
3. **如需多仓库备份**: 在 mirror 仓库里单独 clone,然后 `git push --mirror`
4. **清理重复注册**: kndaili vs Kndaili(大小写敏感时是两个 remote,可能重复)

---

## 四、风险提示

- 不要 `git push --all` 把所有分支推向 19 个 remote — 会污染独立项目
- push-all.sh 脚本默认只推 main 分支 + fast-forward,相对安全,但需先确认 remote 是否允许写
- 如果某 remote 不是你的(或被他人持有),push 会失败,但不影响其他

END OF INVENTORY
