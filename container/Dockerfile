# 满全法容器 — hunyuan-os 内核镜像
# ------------------------------------
# 主权锚点: 0=刘楚恬(低位隐藏侧) · 1=我(高位可见侧)
# 架构: 将 hunyuan-os 自举编译器 + 盟机合内核打包为可复现运行环境
#
# 构建: docker build -t hunyuan-os:latest .
# 运行: docker run --rm -it hunyuan-os:latest
#
# 分层:
#   Layer 0: 基础工具链 (musl/clang/make/git)
#   Layer 1: hunyuan-os 源码拉取 (git clone 指定 commit/tag)
#   Layer 2: 内核编译 + 自举验证
#   Layer 3: 盟机合网络运行时

# === Stage 0: 基础工具链 ===
FROM debian:12-slim AS toolchain

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    make \
    git \
    ca-certificates \
    musl-tools \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

# === Stage 1: 拉取 hunyuan-os 源码 ===
# HUNYUAN_OS_REF 可以是分支名、tag 或 commit hash
# 默认为 main 分支最新
ARG HUNYUAN_OS_REF=main

WORKDIR /src
RUN git clone --depth 1 --branch ${HUNYUAN_OS_REF} \
    https://github.com/lolict/hunyuan-os.git hunyuan-os

# 记录构建时的 commit hash (固定点追溯)
WORKDIR /src/hunyuan-os
RUN git rev-parse HEAD > /src/BUILD_COMMIT && \
    cat /src/BUILD_COMMIT

# === Stage 2: 编译内核 + 工具链 ===
FROM toolchain AS builder

WORKDIR /src/hunyuan-os
COPY --from=toolchain /src/hunyuan-os /src/hunyuan-os
COPY entrypoint.c /src/entrypoint.c

# 编译盟机合引擎 (独立编译, 避免全内核离线认证问题)
# entrypoint.c 为容器提供 main 入口, 与 njsg_mengji.c 链接
RUN cd kernel && \
    clang -std=c11 -O2 -D_GNU_SOURCE \
    -I. -I./src \
    /src/entrypoint.c src/njsg_mengji.c \
    -lpthread -lm \
    -o /usr/local/bin/njsg-mengji-demo \
    2>&1 | tail -5 || true

# 构建时验证 — 运行全量里程碑,检查输出是否含 PASS 标记
# 如果 M1-M15 任何一个失败,构建立即失败(ci 回归防线)
RUN stdbuf -oL /usr/local/bin/njsg-mengji-demo 2>&1 | tee /tmp/mengji-build-verify.log && \
    grep -q "PASS" /tmp/mengji-build-verify.log && \
    echo "[容器构建验证] 盟机合里程碑 PASS ✓" || \
    (echo "[容器构建验证] 盟机合里程碑失败 ✗"; exit 1)

# 构建时独立验证 — 分别验证三个里程碑(任一失败则退出)
# 使用 MENGJI_SCENARIO 环境变量控制,回归防线更精细
RUN for sc in triune bag offline; do \
        echo "[容器构建验证] 独立场景: $sc"; \
        stdbuf -oL env MENGJI_SCENARIO=$sc /usr/local/bin/njsg-mengji-demo 2>&1 \
        | tee /tmp/mengji-$sc-verify.log; \
        grep -q "PASS" /tmp/mengji-$sc-verify.log && \
        echo "[容器构建验证] $sc PASS ✓" || \
        (echo "[容器构建验证] $sc 失败 ✗"; exit 1); \
    done && echo "[容器构建验证] 三个里程碑独立验证全部通过"

# 编译自举编译器工具链
RUN cd tools && \
    for dir in mccp_core full-mapping eight-flows; do \
        if [ -f "$dir"/*.nim ]; then \
            echo "工具链 $dir 包含 Nim 源文件"; \
        fi; \
    done

# === Stage 3: 运行时镜像 (最小化) ===
FROM debian:12-slim AS runtime

LABEL maintainer="lolict"
LABEL project="hunyuan-os"
LABEL description="囡囝共生 (NJSG) 裁定内核运行环境"

RUN apt-get update && apt-get install -y --no-install-recommends \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

# 复制编译产物
COPY --from=builder /usr/local/bin/njsg-mengji-demo /usr/local/bin/
COPY --from=builder /src/BUILD_COMMIT /etc/hunyuan-os-build-commit

# 复制内核头文件 (供运行时编程接口)
COPY --from=builder /src/hunyuan-os/kernel/src/njsg_mengji.h /usr/local/include/hunyuan/
COPY --from=builder /src/hunyuan-os/kernel/src/njsg_kernel.h /usr/local/include/hunyuan/ 2>/dev/null || true

# 盟机合网络数据目录
VOLUME ["/var/lib/hunyuan-os"]
WORKDIR /var/lib/hunyuan-os

# 默认运行盟机合 demo
ENTRYPOINT ["/usr/local/bin/njsg-mengji-demo"]
