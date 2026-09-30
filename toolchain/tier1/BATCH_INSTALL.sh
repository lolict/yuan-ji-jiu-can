#!/bin/bash
# 分片安装脚本 — 单步跑，断点续传
# 每步独立，会话断了下次从上次地址继续

REGISTRY="/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/registry"
VERSION_FILE="$REGISTRY/version.txt"

get_current_addr() {
    cat "$VERSION_FILE" 2>/dev/null || echo "0"
}

log_step() {
    echo "[$(date '+%H:%M:%S')] addr=$(get_current_addr) | $1" | tee -a "$REGISTRY/install.log"
}

# ─── 步骤函数（每步幂等）───
step_rust() {
    log_step "检查 Rust"
    if command -v rustc &>/dev/null; then
        log_step "Rust 已就绪 $(rustc --version)"
        return 0
    fi
    RUSTUP_DIST_SERVER="https://rsproxy.cn" RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
        bash /tmp/rustup-init.sh -y --default-toolchain stable --profile minimal 2>&1 | tail -3
    source ~/.cargo/env 2>/dev/null
    log_step "Rust 安装完成"
}

step_python_fonts() {
    log_step "检查 Python 字体工具"
    python3 -c "
import sys
sys.path.insert(0, '/usr/local/lib/python3.12/dist-packages')
sys.path.insert(0, '/mnt/data/catpaw/home/.local/lib/python3.12/site-packages')
import fontTools, freetype, uharfbuzz, PIL, numpy
print('全部就绪')
" 2>&1 && return 0
    
    pip3 install -i https://pypi.tuna.tsinghua.edu.cn/simple fonttools freetype-py uharfbuzz pillow numpy 2>&1 | tail -5
    log_step "Python 字体工具完成"
}

step_renderer_test() {
    log_step "验证渲染器"
    cd /mnt/data/catpaw/home/workspace/硅碳心源-果套循因/toolchain/tier1
    python3 pure_math_renderer.py 2>&1 | tail -2
    python3 pure_3d_renderer.py 2>&1 | tail -2
    python3 render_shengmu48.py 2>&1 | tail -2
    log_step "渲染器验证完成"
}

step_push_registry() {
    log_step "推送注册清单"
    cd /mnt/data/catpaw/home/workspace/硅碳心源-果套循因/hunyuan-os-registry
    cp ../registry/docs/*.md docs/ 2>/dev/null
    cp ../registry/registry.md . 2>/dev/null
    cp ../registry/version.txt . 2>/dev/null
    cp ../registry/namespaces.md . 2>/dev/null
    git add -A
    git commit -m "batch: $(date '+%m-%d %H:%M') addr=$(get_current_addr)" --allow-empty 2>&1 | tail -1
    git push --all 2>&1 | tail -3
    log_step "推送完成"
}

# ─── 主流程 ───
ADDR=$(get_current_ADDR 2>/dev/null || get_current_addr)
log_step "=== 开始分片安装 (当前地址: $ADDR) ==="

step_rust
step_python_fonts
step_renderer_test
step_push_registry

log_step "=== 分片安装结束 ==="
echo "当前地址: $(get_current_addr)"
echo "下一可用地址: $(( $(get_current_addr) + 1 ))"
