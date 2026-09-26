#!/usr/bin/env python3
"""
满全法 GL 渲染器 — 通过 pygame 在 Xvfb 上绘制, 输出真正 PNG 文件.
不依赖浏览器 Canvas2D, 不依赖 GL 头文件.
原理: Xvfb 虚拟显示 + pygame 的 SDL2 渲染器 + 自定义容器几何投影.

运行:
  DISPLAY=:100 python3 gl_renderer.py
输出:
  gl_output.png
"""

import os
import math
import pygame
import sys

W, H = 800, 600

# ══════════════════════════════════════════════════════════════
# 容器数据 — 与 hunyuan-rs 64 族对齐 (容器族/操作族/声母族/韵母族)
# ══════════════════════════════════════════════════════════════
FAMILY_COLORS = [
    (139, 92, 246),   # 容器族 紫
    (245, 158, 11),   # 操作族 橙
    (56, 189, 248),   # 声母族 蓝
    (236, 72, 153),   # 韵母族 粉
]

CONTAINER_NAMES = [
    "袋","照","见","阴","加","分","元","裁","产","公","声","折","负","福","凤","栖",
    "融","鼓","蚕","民","盟","满","囡","夕","三","六","八","九","零","一","二","四",
    "桶","罐","盆","盂","壶","盒","瓶","盏","盎","通","同","铁","读","怒","辣","热",
    "呗","坡","没","德","特","那","资","此","日","字","西","衣","胡","出","书","如",
]

CONTAINER_CHARS = "袋照见阴加分元裁产公声折负福凤栖融鼓蚕民盟满囡夕三六八九零一二四桶罐盆盂壶盒瓶盏盎同铁读怒辣热呗坡没德特那资此日字西衣胡出書如"

# ══════════════════════════════════════════════════════════════
def project_3d(x, y, z, rot_y=0.4, scale=55):
    """3D → 2D 投影 (与(container-geom.rs coord3 对齐)"""
    cos_r, sin_r = math.cos(rot_y), math.sin(rot_y)
    rx = x * cos_r + y * sin_r
    ry = -x * sin_r + y * cos_r
    ry_tilted = ry * 0.5 + z * 0.5
    persp = 600.0 / (600.0 + z * 30)
    return (
        int(W / 2 + rx * scale * persp),
        int(H / 2 + ry_tilted * scale * persp),
        persp
    )

def main():
    if not os.environ.get("DISPLAY"):
        os.environ["DISPLAY"] = ":100"

    pygame.init()
    screen = pygame.display.set_mode((W, H), pygame.HIDDEN)
    pygame.display.set_caption("ManQuanFa Renderer")
    surf = pygame.Surface((W, H))

    # 背景渐变
    for y in range(H):
        r = int(4 + (y / H) * 6)
        g = int(6 + (y / H) * 10)
        b = int(14 + (y / H) * 20)
        pygame.draw.line(surf, (r, g, b), (0, y), (W, y))

    # ══════════════════════════════════════════════════════════
    # 计算每个容器的3D坐标 (6-bit → x,y,z via 三维进制)
    # ══════════════════════════════════════════════════════════
    containers = []
    for i in range(64):
        co = {'x': i & 0x7, 'y': (i >> 3) & 0x7, 'z': (i >> 6) & 0x3}
        fam = i // 16
        ch = CONTAINER_CHARS[i] if i < len(CONTAINER_CHARS) else '?'
        containers.append({
            'id': i, 'code': i, 'cx': co['x'] - 3.5, 'cy': co['y'] - 3.5, 'cz': co['z'],
            'fam': fam, 'ch': ch
        })

    # ══════════════════════════════════════════════════════════
    # 画容器节点 (含家庭色 + 极性高光)
    # ══════════════════════════════════════════════════════════
    for c in containers:
        px, py, persp = project_3d(c['cx'], c['cy'], c['cz'])
        base_r = int(8 * persp + 4)
        col = FAMILY_COLORS[c['fam']]

        # 容器圆
        pygame.draw.circle(surf, col, (px, py), base_r)
        # 内部高光
        hl = (min(255, col[0]+60), min(255, col[1]+60), min(255, col[2]+60))
        pygame.draw.circle(surf, hl, (px-1, py-1), max(2, base_r - 3))
        # 外圈
        pygame.draw.circle(surf, (255, 255, 255), (px, py), base_r, 1)

        # 字 (如果pygame能渲染 — 小字体)
        try:
            if base_r > 6:
                font = pygame.font.SysFont("Noto Sans CJK SC", max(8, int(10 * persp)))
                if font:
                    txt = font.render(c['ch'], True, (255, 255, 255))
                    trect = txt.get_rect(center=(px, py - base_r - 8))
                    surf.blit(txt, trect)
        except:
            pass

    # ══════════════════════════════════════════════════════════
    # 画连线: 同族容器按x坐标顺序串联
    # ══════════════════════════════════════════════════════════
    for fam in range(4):
        base_idx = fam * 16
        points = []
        for j in range(base_idx, base_idx + 16):
            c = containers[j]
            px, py, _ = project_3d(c['cx'], c['cy'], c['cz'])
            points.append((px, py))
        # 画链式连线
        if len(points) > 1:
            fam_col = FAMILY_COLORS[fam]
            line_col = (fam_col[0]//2, fam_col[1]//2, fam_col[2]//2)
            for k in range(len(points) - 1):
                pygame.draw.line(surf, line_col, points[k], points[k+1], 1)

    # ══════════════════════════════════════════════════════════
    # 标题
    # ══════════════════════════════════════════════════════════
    try:
        font_big = pygame.font.SysFont("Noto Sans CJK SC", 16)
        if font_big:
            title = font_big.render("满全法 · 容器64族 3D渲染 (Mesa swrast + pygame)", True, (200, 210, 230))
            surf.blit(title, (20, 15))
            legend_y = H - 80
            for fi in range(4):
                fcol = FAMILY_COLORS[fi]
                pygame.draw.rect(surf, fcol, (20 + fi * 120, legend_y, 14, 14))
                name = ["容器族", "操作族", "声母族", "韵母族"][fi]
                txt = font_big.render(name, True, (180, 190, 210))
                surf.blit(txt, (40 + fi * 120, legend_y))
    except:
        pass

    # ══════════════════════════════════════════════════════════
    # 保存 PNG
    # ══════════════════════════════════════════════════════════
    out_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/phase-radix/gl_output.png"
    pygame.image.save(surf, out_path)
    print(f"Written: {out_path} ({W}x{H} PNG)")

    pygame.quit()

if __name__ == "__main__":
    main()
