#!/usr/bin/env python3
# 满全法 · 思维路径图生成器 (Synapse Map Generator)
# 功能:
#   1. 读取 synapses.json
#   2. 计算出度/入度/介数 → 识别公倍数锚点
#   3. 生成 HTML 交互式思维路径图
#   4. 三维容器场投影 (SVG)
#
# 输出: synapses_map.html (自包含, 零外部依赖)

import json, os, math, heapq
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
SYNAPSES = os.path.join(HERE, 'synapses.json')
OUTPUT = os.path.join(HERE, 'synapses_map.html')

def load():
    with open(SYNAPSES, 'r', encoding='utf-8') as f:
        return json.load(f)

def compute_degrees(data):
    """计算节点的出度、入度"""
    out_deg = defaultdict(int)
    in_deg = defaultdict(int)
    for path, node in data['nodes'].items():
        for s in node.get('synapses', []):
            out_deg[path] += 1
            in_deg[s['to']] += 1
    return out_deg, in_deg

def find_anchor_nodes(data, out_deg, in_deg, top_k=30):
    """
    识别公倍数锚点:
    - 高出度 = 被引用的中心 (hub)
    - 高入度 = 依赖汇聚点
    - 高出度+高入度 = 核心锚点 (公倍数)
    """
    scores = {}
    for path in data['nodes']:
        o = out_deg.get(path, 0)
        i = in_deg.get(path, 0)
        # 公倍数 = 出 × 入 (类似特征向量)
        scores[path] = {
            'out': o,
            'in': i,
            'product': o * i,      # 公共倍起点×终点
            'sum': o + i,           # 总连接度
            'ratio': o / max(i, 1), # 出/入比 (1=平衡,→∞=源头,←0=终点)
        }
    # Top-K
    top_by_product = sorted(scores.items(), key=lambda x: -x[1]['product'])[:top_k]
    top_by_out = sorted(scores.items(), key=lambda x: -x[1]['out'])[:top_k]
    top_by_in = sorted(scores.items(), key=lambda x: -x[1]['in'])[:top_k]
    return scores, top_by_product, top_by_out, top_by_in

def shortest_path_bfs(data, src, dst, max_depth=8):
    """BFS最短路径"""
    if src == dst:
        return [src]
    visited = {src}
    queue = [(src, [src])]
    while queue:
        current, path = queue.pop(0)
        if len(path) > max_depth:
            break
        node = data['nodes'].get(current, {})
        for s in node.get('synapses', []):
            nxt = s['to']
            if nxt in visited:
                continue
            visited.add(nxt)
            new_path = path + [nxt]
            if nxt == dst:
                return new_path
            queue.append((nxt, new_path))
    return None

def project_field(node):
    """三维容器投影"""
    a = node['address'].split(':') if isinstance(node['address'], str) else node['address'].values()
    si, ti, yi, ci, mi = int(a[0]), int(a[1]), int(a[2]), int(a[3]), int(a[4])
    x = (ci % 7) * 40 + (si % 7) * 5 + 200
    y = ((ci >> 3) % 6) * 50 + (yi % 6) * 8 + 100
    z = yi * 15 + mi
    phase = (si + ti) % 12
    return x, y, z, phase

def generate_html(data, scores, top_product, top_out, top_in):
    """生成自包含 HTML 思维路径图"""
    total_nodes = len(data['nodes'])
    total_synapses = data['stats']['total_synapses']

    # 提取 top 节点
    core_nodes = [p for p, _ in top_product[:20]]
    core_set = set(core_nodes)
    # 为每个core提取直接邻居
    neighbor_set = set(core_nodes)
    for p in core_nodes:
        node = data['nodes'][p]
        for s in node.get('synapses', [])[:5]:
            neighbor_set.add(s['to'])
        # 反向
        for q, qnode in data['nodes'].items():
            for s in qnode.get('synapses', []):
                if s['to'] == p:
                    neighbor_set.add(q)
            if len(neighbor_set) > 500:
                break
        if len(neighbor_set) > 500:
            break

    # 提取子图
    subgraph = {}
    for p in neighbor_set:
        if p in data['nodes']:
            subgraph[p] = data['nodes'][p]

    # 构建 JSON 嵌入
    nodes_json = []
    for p, node in subgraph.items():
        x, y, z, phase = project_field(node)
        is_core = p in core_set
        addr = node['address']
        si = int(addr.split(':')[0]) if isinstance(addr, str) else addr['si']
        ci = int(addr.split(':')[3]) if isinstance(addr, str) else addr['ci']
        nodes_json.append({
            'id': p[:80],
            'fullpath': p,
            'name': node['filename'],
            'x': x, 'y': y, 'z': z,
            'phase': phase,
            'si': si, 'ci': ci,
            'is_core': is_core,
            'degree': scores.get(p, {}).get('sum', 0),
            'size': min(8 + scores.get(p, {}).get('sum', 0) * 0.5, 25),
        })

    links_json = []
    for p, node in subgraph.items():
        for s in node.get('synapses', [])[:8]:
            if s['to'] in subgraph:
                links_json.append({
                    'source': p[:80],
                    'target': s['to'][:80],
                    'type': s['type'],
                    'weight': s['weight'],
                })

    nodes_data = json.dumps(nodes_json, ensure_ascii=False)
    links_data = json.dumps(links_json, ensure_ascii=False)
    anchors_data = json.dumps([
        {'path': p[:100], 'out': v['out'], 'in_d': v['in'], 'product': v['product']}
        for p, v in top_product[:15]
    ], ensure_ascii=False)

    html = f'''<!DOCTYPE html>
<html lang="zh">
<head><meta charset="UTF-8">
<title>满全法 · 神经突触思维路径图</title>
<style>
* {{ margin:0; padding:0; box-sizing:border-box; }}
body {{ background:#0a0a0f; color:#e0e0e0; font-family:monospace; overflow:hidden; }}
#canvas {{ width:100vw; height:100vh; display:block; cursor:grab; }}
#info {{ position:fixed; top:10px; left:10px; background:rgba(20,20,30,0.9);
  border:1px solid #333; border-radius:6px; padding:12px 16px; max-width:400px;
  font-size:12px; line-height:1.6; pointer-events:none; }}
#info h3 {{ color:#ff6b9d; margin-bottom:8px; font-size:13px; }}
#info .stat {{ color:#aaa; }}
#info .val {{ color:#fff; font-weight:bold; }}
#tooltip {{ position:fixed; display:none; background:rgba(30,30,50,0.95);
  border:1px solid #555; border-radius:4px; padding:8px 12px; font-size:11px;
  max-width:300px; pointer-events:none; z-index:100; }}
.legend {{ position:fixed; bottom:10px; right:10px; background:rgba(20,20,30,0.9);
  border:1px solid #333; border-radius:6px; padding:10px 14px; font-size:11px; }}
.legend span {{ display:inline-block; margin:2px 6px; }}
.dot {{ display:inline-block; width:10px; height:10px; border-radius:50%; vertical-align:middle; }}
.closeBtn {{ position:fixed; top:10px; right:10px; background:#222; border:1px solid #444;
  color:#fff; padding:6px 14px; cursor:pointer; border-radius:4px; font-size:12px; }}
.closeBtn:hover {{ background:#444; }}
</style></head>
<body>
<canvas id="canvas"></canvas>
<div id="info">
  <h3>满全法 · 神经突触思维路径</h3>
  <div class="stat">节点: <span class="val">{total_nodes:,}</span></div>
  <div class="stat">突触: <span class="val">{total_synapses:,}</span></div>
  <div class="stat">地址空间: <span class="val">374,976,000</span></div>
  <div class="stat">子图: <span class="val" id="sg_count">...</span></div>
  <div class="stat" style="margin-top:6px;color:#ff6b9d">锚点 = 出度×入度 (公倍数)</div>
  <div class="stat" style="font-size:10px;color:#777">拖拽=平移 · 滚轮=缩放 · 悬停=详情</div>
</div>
<div id="tooltip"></div>
<div class="legend">
  <div><span class="dot" style="background:#ff6b9d"></span>核心锚点</div>
  <div><span class="dot" style="background:#6bd3ff"></span>普通节点</div>
  <div><span class="dot" style="background:#ffd86b"></span>高入度端点</div>
  <div><span class="dot" style="background:#6bff8e"></span>高出度源头</div>
  <div style="margin-top:4px;color:#888">线条: import/use/link/ref</div>
</div>
<button class="closeBtn" onclick="window.parent.postMessage({{type:'close'}},'*')">×</button>

<script>
const NODES = {nodes_data};
const LINKS = {links_data};
const ANCHORS = {anchors_data};

// Build index
const nodeMap = {{}};
NODES.forEach(n => nodeMap[n.id] = n);
document.getElementById('sg_count').textContent = NODES.length;

// Canvas setup
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
let W, H;
function resize() {{
  W = canvas.width = window.innerWidth;
  H = canvas.height = window.innerHeight;
}}
resize();
window.addEventListener('resize', resize);

// View transform
let view = {{ x: 0, y: 0, scale: 1 }};
let dragging = false, dragStart = null;
let hoveredNode = null;

canvas.addEventListener('mousedown', e => {{
  dragging = true;
  dragStart = {{ x: e.clientX - view.x, y: e.clientY - view.y }};
  canvas.style.cursor = 'grabbing';
}});
canvas.addEventListener('mousemove', e => {{
  if (dragging) {{
    view.x = e.clientX - dragStart.x;
    view.y = e.clientY - dragStart.y;
  }}
  // Hover detection
  const mx = (e.clientX - view.x) / view.scale;
  const my = (e.clientY - view.y) / view.scale;
  hoveredNode = null;
  for (const n of NODES) {{
    if ((n.x-mx)**2 + (n.y-my)**2 < n.size**2 * 4) {{
      hoveredNode = n;
      break;
    }}
  }}
  const tip = document.getElementById('tooltip');
  if (hoveredNode) {{
    tip.style.display = 'block';
    tip.style.left = (e.clientX+12)+'px';
    tip.style.top = (e.clientY+12)+'px';
    tip.html = '<b>'+hoveredNode.name+'</b><br>'+
      '地址: '+hoveredNode.si+':'+hoveredNode.ci+'<br>'+
      '度: '+hoveredNode.degree+'<br>'+
      '相位: '+(hoveredNode.phase*30)+'°<br>'+
      hoveredNode.fullpath;
  }} else {{
    tip.style.display = 'none';
  }}
}});
canvas.addEventListener('mouseup', () => {{ dragging=false; canvas.style.cursor='grab'; }});
canvas.addEventListener('wheel', e => {{
  e.preventDefault();
  const factor = e.deltaY > 0 ? 0.9 : 1.1;
  view.scale *= factor;
  view.scale = Math.max(0.1, Math.min(10, view.scale));
}}, {{ passive: false }});

// Force-directed layout (simple)
function applyLayout() {{
  const cx = W/2, cy = H/2;
  // Center the subgraph
  let avgX=0, avgY=0;
  NODES.forEach(n => {{ avgX+=n.x; avgY+=n.y; }});
  avgX/=NODES.length; avgY/=NODES.length;
  NODES.forEach(n => {{
    n.x = (n.x - avgX) * 0.3 + cx;
    n.y = (n.y - avgY) * 0.3 + cy;
  }});
  // Iterate forces
  for (let iter=0; iter<50; iter++) {{
    // Repulsion
    for (let i=0; i<NODES.length; i++) {{
      for (let j=i+1; j<NODES.length; j++) {{
        const dx = NODES[j].x - NODES[i].x;
        const dy = NODES[j].y - NODES[i].y;
        const dist = Math.sqrt(dx*dx+dy*dy) + 0.1;
        if (dist < 100) {{
          const force = (100 - dist) / dist * 0.5;
          NODES[i].x -= dx * force;
          NODES[i].y -= dy * force;
          NODES[j].x += dx * force;
          NODES[j].y += dy * force;
        }}
      }}
    }}
    // Attraction along links
    LINKS.forEach(l => {{
      const s = nodeMap[l.source], t = nodeMap[l.target];
      if (!s||!t) return;
      const dx = t.x - s.x, dy = t.y - s.y;
      const dist = Math.sqrt(dx*dx+dy*dy) + 0.1;
      const f = (dist - 30) * 0.01;
      s.x += dx*f; s.y += dy*f;
      t.x -= dx*f; t.y -= dy*f;
    }});
  }}
}}

function render() {{
  ctx.clearRect(0, 0, W, H);
  ctx.save();
  ctx.translate(view.x, view.y);
  ctx.scale(view.scale, view.scale);

  // Glow background for field impression
  const grad = ctx.createRadialGradient(W/2, H/2, 0, W/2, H/2, 600);
  grad.addColorStop(0, 'rgba(40,40,80,0.05)');
  grad.addColorStop(1, 'rgba(0,0,0,0)');
  ctx.fillStyle = grad;
  ctx.fillRect(-1000, -1000, 3000, 3000);

  // Draw links
  LINKS.forEach(l => {{
    const s = nodeMap[l.source], t = nodeMap[l.target];
    if (!s||!t) return;
    const alpha = Math.min(0.4, l.weight * 0.3);
    ctx.strokeStyle = `rgba(100,140,200,${{alpha}})`;
    ctx.lineWidth = Math.min(2, l.weight);
    ctx.beginPath();
    ctx.moveTo(s.x, s.y);
    // Curved link
    const mx = (s.x+t.x)/2, my = (s.y+t.y)/2 - 10;
    ctx.quadraticCurveTo(mx, my, t.x, t.y);
    ctx.stroke();
  }});

  // Draw nodes
  NODES.forEach(n => {{
    const r = hoveredNode === n ? n.size * 1.5 : n.size;
    ctx.beginPath();
    ctx.arc(n.x, n.y, r, 0, Math.PI*2);
    if (n.is_core) {{
      ctx.fillStyle = '#ff6b9d';
      ctx.shadowColor = '#ff6b9d';
      ctx.shadowBlur = 15;
    }} else if (n.degree > 8) {{
      ctx.fillStyle = '#ffd86b';
      ctx.shadowBlur = 5;
    }} else {{
      ctx.fillStyle = '#6bd3ff';
      ctx.shadowBlur = 0;
    }}
    ctx.fill();
    ctx.shadowBlur = 0;

    // Label for core nodes
    if (n.is_core || r > 8) {{
      ctx.fillStyle = '#fff';
      ctx.font = '9px monospace';
      const label = n.name.length > 25 ? n.name.slice(0,24)+'...' : n.name;
      ctx.fillText(label, n.x + r + 3, n.y + 3);
    }}
  }});

  ctx.restore();
  requestAnimationFrame(render);
}}

applyLayout();
render();
</script>
</body>
</html>'''
    return html

def main():
    data = load()
    print("计算度中心性...")
    out_deg, in_deg = compute_degrees(data)
    scores, top_prod, top_out, top_in = find_anchor_nodes(data, out_deg, in_deg)

    print("═══ 公倍数锚点 Top 15 (出度×入度) ═══")
    for i, (p, v) in enumerate(top_prod[:15]):
        short = p if len(p) < 60 else '...'+p[-57:]
        print(f"  {i+1:2d}. [{v['out']:2d}×{v['in']:2d}={v['product']:4d}] {short}")

    print(f"\n── 思维路径 BFS 示例 ──")
    # 从 first anchor 到 second anchor
    if len(top_prod) >= 2:
        src = top_prod[0][0]
        dst = top_prod[1][0]
        print(f"  起点: {src[:60]}")
        print(f"  终点: {dst[:60]}")
        path = shortest_path_bfs(data, src, dst)
        if path:
            print(f"  路径 ({len(path)-1}跳):")
            for p in path:
                print(f"    → {p[:60]}")
        else:
            print("  无路径")

    print("\n生成 HTML 思维路径图...")
    html = generate_html(data, scores, top_prod, top_out, top_in)
    with open(OUTPUT, 'w', encoding='utf-8') as f:
        f.write(html)
    sz = os.path.getsize(OUTPUT)
    print(f"✓ 生成 {OUTPUT} ({sz/1024:.0f} KB)")

    print("\n── 全部完成 ──")

if __name__ == '__main__':
    main()
