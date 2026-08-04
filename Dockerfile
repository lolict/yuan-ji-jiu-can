# 满全法 · 囡囝共生元 — 执行内核容器 v0.8
# ManQuanFa · NanJian Symbiosis Metakernel Container
#
# 用法:
#   docker build -t mqf-kernel:0.8 .
#   docker run --rm -it mqf-kernel:0.8
#
# 这套容器把满全法 v0.1-v0.8 全部内核封装在一张镜像里:
#   C + pthread + 身份编码 + 规则引擎 + 空间代数 + 金壳封装 + 镜像代理 + 统计锥
#
# 维护者: lolict + 七妹(CatPaw)
# 版本: v0.8

FROM alpine:3.19

LABEL maintainer="lolict"
LABEL version="0.8.0"
LABEL description="满全法 · 囡囝共生元 — 执行内核(Time Judge · Identity Kernel)"

# 编译工具链 + 运行依赖
RUN apk add --no-cache \
    clang \
    make \
    musl-dev \
    linux-headers \
    libexecinfo \
    bash \
    curl \
  && ln -s /usr/bin/clang /usr/bin/cc

WORKDIR /mqf

# 源码
COPY kernel/ ./kernel/
COPY Makefile .

# 编译执行内核
RUN make clean && make kernel || (echo "[构建失败] 请检查 clang 版本" && exit 1)

# 暴露元数据(不对外端口,因为裁判内核本身不监听)
EXPOSE 12789

# 入口脚本
COPY docker-entrypoint.sh /usr/local/bin/mqf-entry
RUN chmod +x /usr/local/bin/mqf-entry

ENTRYPOINT ["mqf-entry"]
CMD ["--run-demo"]
