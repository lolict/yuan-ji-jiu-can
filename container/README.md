# hunyuan-container

满全法容器 — 囡囝共生 (NJSG) 裁定内核的运行时封装。

## 范畴边界

本仓库**不是** hunyuan-os 的替代品。它只负责把 hunyuan-os 已验证的内核编译成果装进可复现的容器运行环境。

| 仓库 | 职责 | 验证标准 |
|------|------|----------|
| hunyuan-os | 编码体系、自举编译器、裁决逻辑 | 固定点 hash、M1-M15 PASS |
| hunyuan-container (本仓库) | 容器镜像构建、盟机合传输层编排 | 镜像构建成功、容器运行测试 |

两个仓库通过**镜像标签 + commit hash** 对齐，互不侵入。

## 镜像分层

```
Layer 0: debian:12-slim + clang/make/git/musl
Layer 1: 拉取 hunyuan-os 源码 (指定 ref)
Layer 2: 编译内核 + 工具链
Layer 3: 运行时 (仅保留编译产物 + 必要库)
```

## 构建方式

### 本地构建

```bash
docker build -t hunyuan-os:latest \
  --build-arg HUNYUAN_OS_REF=main .
```

指定特定 commit：

```bash
docker build -t hunyuan-os:7d00855f \
  --build-arg HUNYUAN_OS_REF=7d00855f .
```

### 自动构建 (GitHub Actions)

推送代码到 main 分支或打 `v*` tag 时自动构建并推送到 GHCR。

## 构建时验证

每次 `docker build` 自动运行三个独立里程碑验证，任一失败则镜像不产出：

| 验证场景 | 对应里程碑 | 验证内容 |
|----------|-----------|----------|
| `triune` | 里程碑二 | 三元背书 + 冲突拒绝 |
| `bag` | 里程碑三 | 袋子合并拆分 + 权值守恒 |
| `offline` | 里程碑一 | 离线 Pull 恢复 |

### 无 Docker 验证

```bash
./scripts/local-build.sh              # 三场景全部验证
./scripts/local-build.sh triune       # 只验证三元见证
./scripts/local-build.sh bag          # 只验证袋子拆分合并
./scripts/local-build.sh offline      # 只验证离线恢复
```

### 单独运行容器某场景

```bash
# 默认运行全量 demo
docker run --rm -it hunyuan-os:latest

# 指定独立场景
docker run --rm -it -e MENGJI_SCENARIO=triune hunyuan-os:latest
docker run --rm -it -e MENGJI_SCENARIO=bag hunyuan-os:latest
docker run --rm -it -e MENGJI_SCENARIO=offline hunyuan-os:latest
```

## 盟机合网络 (里程碑一传输层)

```bash
docker compose up -d        # 启动三节点
docker compose logs -f      # 查看运行状态
docker compose down         # 停止
```

## 主权锚点

- 容器内裁决逻辑使用 hunyuan-os 内嵌的夫妻主权人锚点
- 节点类型 (PHONG/FARM) 通过环境变量注入
- 不对外暴露零维根基对应关系
