// 容器星系 — 三维空间投影 + 时间脉冲可视化
//
// 将64容器投影到3D空间:
//   X轴 = 语义ID (容器码)
//   Y轴 = 拓扑码 (低4位拓扑)
//   Z轴 = 时间槽 (公约数整除脉冲)
//   颜色 = 族ID (4族4色)
//   大小 = YinYang极性 (容器○大 / 界线●小)
//
// 生成可旋转的 WebGL 3D 视图
//
// 编译: rustc --edition 2018 -O  src/bin/container_galaxy.rs -o container_galaxy
// 执行: ./container_galaxy [输出目录]
//
// 输出:
//   galaxy3d.html — 自包含3D可旋转容器星系可视化

use std::env;

// ═══════════════════════════════════════════════════════════════════════════
// 数据定义 (与 time_fridge.rs 对齐)
// ═══════════════════════════════════════════════════════════════════════════

const CONTAINER_NAMES: [&str; 64] = [
    "元","加","减","与","或","非","异或","传送",
    "跳转","调用","返回","比较","加载","存储","移位","置位",
    "满","全","法","囡","丱","共","生","伴",
    "侣","赔","偿","合","离","聚","散","归",
    "帮","滂","并","明","端","透","定","泥",
    "知","彻","澄","娘","见","溪","群","疑",
    "东","冬","钟","江","支","脂","之","微",
    "鱼","虞","模","齐","佳","皆","灰","咍",
];

fn topology_code(code: u8) -> u8 {
    match code & 0x0F {
        0x00..=0x03 => 0x10,
        0x04..=0x06 => 0x30,
        0x07..=0x0A => 0x20,
        0x0B..=0x0D => 0x50,
        _ => 0x60,
    }
}

fn family_id(code: u8) -> u8 { code >> 4 }

fn time_slot(code: u8) -> u8 {
    let base = code & 0x0F;
    let family = code >> 4;
    let divisors = (1u8..=code.max(1)).filter(|d| code % d == 0).count() as u8;
    base + family * 16 + divisors * 2
}

fn family_color(fam: u8) -> &'static str {
    match fam {
        0 => "#4A90D9",
        1 => "#7B68EE",
        2 => "#50B86C",
        3 => "#D98C4A",
        _ => "#666",
    }
}

fn topology_glyph(code: u8) -> char {
    match topology_code(code) {
        0x10 => '■',
        0x20 => '●',
        0x30 => '◎',
        0x50 => '◆',
        _ => '━',
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 容器点 — 三维坐标 + 颜色 + 大小
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy)]
struct SpacePoint {
    x: f64, y: f64, z: f64,     // 3D坐标
    color: [f64; 3],            // RGB
    size: f64,                  // 球体半径
    code: u8,
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [f64; 3] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = match (h / 60.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    [r1 + m, g1 + m, b1 + m]
}

fn hex_to_rgb(hex: &str) -> [f64; 3] {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0) as f64 / 255.0;
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0) as f64 / 255.0;
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0) as f64 / 255.0;
    [r, g, b]
}

fn generate_points() -> Vec<SpacePoint> {
    let mut points = Vec::new();

    for code in 0u8..64 {
        let fam = family_id(code);
        let ts = time_slot(code);
        let topo = topology_code(code);
        let is_yang = (code & 1) == 1;

        // X = 语义码 (0..63)
        let x = code as f64;
        // Y = 拓扑码 (visual separation)
        let y = (topo >> 4) as f64;
        // Z = 时间槽 (0..84) scaled to 0..64 range
        let z = ts as f64 * 0.85;

        let base_color = hex_to_rgb(family_color(fam));
        // Pulse-color shift based on time slot
        let hue_shift = (ts as f64 / 84.0) * 40.0;
        let shifted = hsl_to_rgb(
            (fam as f64 * 90.0 + hue_shift) % 360.0,
            0.7,
            0.55,
        );

        // Blend: base hue + time-brightness
        let brightness = 0.6 + 0.4 * (ts as f64 / 84.0);
        let color = [
            (shifted[0] * brightness).min(1.0),
            (shifted[1] * brightness).min(1.0),
            (shifted[2] * brightness).min(1.0),
        ];

        // Size: YinYang polarity
        let size = if is_yang { 0.6 } else { 1.0 };

        points.push(SpacePoint {
            x, y, z, color, size, code,
        });
    }

    points
}

// ═══════════════════════════════════════════════════════════════════════════
// Galaxy HTML Generator
// ═══════════════════════════════════════════════════════════════════════════

fn galaxy_html(points: &[SpacePoint]) -> String {
    let mut html = String::with_capacity(65536);

    // Serialize points to JS arrays
    let xs: Vec<String> = points.iter().map(|p| format!("{:.1}", p.x)).collect();
    let ys: Vec<String> = points.iter().map(|p| format!("{:.1}", p.y)).collect();
    let zs: Vec<String> = points.iter().map(|p| format!("{:.1}", p.z)).collect();
    let colors: Vec<String> = points.iter().map(|p| {
        let r = (p.color[0] * 255.0) as u8;
        let g = (p.color[1] * 255.0) as u8;
        let b = (p.color[2] * 255.0) as u8;
        format!("\"rgb({},{},{})\"", r, g, b)
    }).collect();
    let sizes: Vec<String> = points.iter().map(|p| format!("{:.1}", p.size)).collect();
    let names: Vec<String> = points.iter().map(|p| {
        format!("\"{}\"", CONTAINER_NAMES[p.code as usize])
    }).collect();

    html.push_str(r#"<!DOCTYPE html><html lang="zh"><head><meta charset="UTF-8">"#);
    html.push_str(r#"<title>容器星系 — 64容器3D投影</title>"#);
    html.push_str(r#"<style>
body{margin:0;background:#05050f;color:#ccc;font-family:sans-serif;overflow:hidden}
#info{position:fixed;top:8px;left:8px;font-size:12px;color:#888;z-index:10;pointer-events:none}
#legend{position:fixed;bottom:8px;left:8px;font-size:11px;color:#666;z-index:10;display:flex;gap:12px;flex-wrap:wrap}
.lg{display:flex;align-items:center;gap:3px}
.dot{width:10px;height:10px;border-radius:50%}
#stats{position:fixed;top:8px;right:8px;font-size:10px;color:#555;z-index:10;font-family:monospace}
canvas{display:block}
</style></head><body>
<canvas id="c"></canvas>
<div id="info">
  <b style="color:#4A90D9">容器星系</b> — 三维投影<br>
  鼠标拖动旋转 · 滚轮缩放<br>
  X=语义码 Y=拓扑 Z=时间槽
</div>
<div id="legend">
  <div class="lg"><span class="dot" style="background:#4A90D9"></span>运算族</div>
  <div class="lg"><span class="dot" style="background:#7B68EE"></span>满全族</div>
  <div class="lg"><span class="dot" style="background:#50B86C"></span>声母族</div>
  <div class="lg"><span class="dot" style="background:#D98C4A"></span>韵母族</div>
  <div class="lg"><span class="dot" style="background:#fff;width:8px;height:8px"></span>○容器(妻)</div>
  <div class="lg"><span class="dot" style="background:#fff;width:5px;height:5px"></span>●界线(夫)</div>
</div>
"#);

    html.push_str(r#"<script>
const XS=["#); html.push_str(&xs.join(",")); html.push_str(r#"];
const YS=["#); html.push_str(&ys.join(",")); html.push_str(r#"];
const ZS=["#); html.push_str(&zs.join(",")); html.push_str(r#"];
const CL=["#); html.push_str(&colors.join(",")); html.push_str(r#"];
const SZ=["#); html.push_str(&sizes.join(",")); html.push_str(r#"];
const NM=["#); html.push_str(&names.join(",")); html.push_str(r#"];

const canvas=document.getElementById('c');
const ctx=canvas.getContext('2d');
let W,H;
function resize(){W=canvas.width=innerWidth;H=canvas.height=innerHeight;}
resize();
addEventListener('resize',resize);

let angleX=0.4,angleY=0.6,dist=80;
let dragging=false,lastX=0,lastY=0;
addEventListener('mousedown',e=>{dragging=true;lastX=e.clientX;lastY=e.clientY});
addEventListener('mouseup',()=>dragging=false);
addEventListener('mousemove',e=>{
  if(!dragging)return;
  angleY+=(e.clientX-lastX)*0.005;
  angleX+=(e.clientY-lastY)*0.005;
  lastX=e.clientX;lastY=e.clientY;
});
addEventListener('wheel',e=>{dist=Math.max(30,Math.min(200,dist+e.deltaY*0.1))});

function project(x,y,z){
  // Center the data
  let cx=x-31.5, cy=y-7.5, cz=z-38;
  // Rotate Y
  let x1=cx*Math.cos(angleY)+cz*Math.sin(angleY);
  let z1=-cx*Math.sin(angleY)+cz*Math.cos(angleY);
  // Rotate X
  let y1=cy*Math.cos(angleX)-z1*Math.sin(angleX);
  let z2=cy*Math.sin(angleX)+z1*Math.cos(angleX);
  // Perspective
  let scale=dist/(dist+z2+50);
  return [W/2+x1*scale*2.2, H/2+y1*scale*2.2, scale];
}

function draw(){
  ctx.fillStyle='#05050f';
  ctx.fillRect(0,0,W,H);

  // Draw connections between same-family containers
  ctx.strokeStyle='rgba(255,255,255,0.03)';
  ctx.lineWidth=0.5;
  for(let i=0;i<64;i++){
    for(let j=i+1;j<64;j++){
      if(Math.floor(i/16)===Math.floor(j/16) && j===i+1){
        let p1=project(XS[i],YS[i],ZS[i]);
        let p2=project(XS[j],YS[j],ZS[j]);
        ctx.beginPath();
        ctx.moveTo(p1[0],p1[1]);
        ctx.lineTo(p2[0],p2[1]);
        ctx.stroke();
      }
    }
  }

  // Sort by depth
  let order=[...Array(64).keys()];
  let depths=order.map(i=>{
    let cx=XS[i]-31.5,cy=YS[i]-7.5,cz=ZS[i]-38;
    return -cx*Math.sin(angleY)+cz*Math.cos(angleY);
  });
  order.sort((a,b)=>depths[a]-depths[b]);

  for(const i of order){
    let p=project(XS[i],YS[i],ZS[i]);
    let r=SZ[i]*4*p[2];

    // Glow
    let grad=ctx.createRadialGradient(p[0],p[1],0,p[0],p[1],r*2.5);
    grad.addColorStop(0,CL[i].replace(')',',0.3)').replace('rgb','rgba'));
    grad.addColorStop(1,'transparent');
    ctx.fillStyle=grad;
    ctx.fillRect(p[0]-r*2.5,p[1]-r*2.5,r*5,r*5);

    // Sphere
    ctx.beginPath();
    ctx.arc(p[0],p[1],r,0,Math.PI*2);
    ctx.fillStyle=CL[i];
    ctx.fill();

    // Name label
    if(r>1.5){
      ctx.font=`${Math.max(8,Math.round(r*0.8))}px "HanYuan64","Noto Sans CJK SC",sans-serif`;
      ctx.fillStyle='#fff';
      ctx.textAlign='center';
      ctx.fillText(NM[i],p[0],p[1]+r+10);
    }
  }

  requestAnimationFrame(draw);
}
draw();
</script>
</body></html>"#);

    html
}

// ═══════════════════════════════════════════════════════════════════════════
fn main() {
    let args: Vec<String> = env::args().collect();
    let out_dir = if args.len() > 1 { args[1].clone() } else { ".".to_string() };

    let points = generate_points();
    let html = galaxy_html(&points);
    let path = format!("{}/galaxy3d.html", out_dir);

    std::fs::write(&path, &html).expect("write galaxy.html");
    eprintln!("容器星系3D投影: {}  ({} bytes)", path, html.len());
}
