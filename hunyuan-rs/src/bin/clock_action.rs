// 时钟动作 — 容器=时间 生命周期引擎
//
// 编译: rustc --edition 2018 -O src/bin/clock_action.rs -o clock_action
// 执行: ./clock_action [输出目录] [tick数]

use std::env;
use std::fmt::Write;

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

fn family_color(fam: u8) -> &'static str {
    match fam {
        0 => "#4A90D9",
        1 => "#7B68EE",
        2 => "#50B86C",
        3 => "#D98C4A",
        _ => "#666",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ActionKind { Call, Shed, Bind, Judge, Store, Wake }

impl ActionKind {
    fn name(&self) -> &'static str {
        match self { ActionKind::Call=>"调用", ActionKind::Shed=>"脱壳", ActionKind::Bind=>"绑定",
            ActionKind::Judge=>"裁决", ActionKind::Store=>"存储", ActionKind::Wake=>"唤醒" }
    }
    fn color(&self) -> &'static str {
        match self { ActionKind::Call=>"#4A90D9", ActionKind::Shed=>"#7B68EE", ActionKind::Bind=>"#50B86C",
            ActionKind::Judge=>"#D98C4A", ActionKind::Store=>"#E86868", ActionKind::Wake=>"#C490E4" }
    }
}

static ACTIONS: [ActionKind; 6] = [
    ActionKind::Call, ActionKind::Shed, ActionKind::Bind,
    ActionKind::Judge, ActionKind::Store, ActionKind::Wake,
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum LifeState { Dormant, Active, Shedding, Bound, Remembered }

impl LifeState {
    fn name(&self) -> &'static str {
        match self { LifeState::Dormant=>"休眠", LifeState::Active=>"活跃", LifeState::Shedding=>"脱壳",
            LifeState::Bound=>"绑定", LifeState::Remembered=>"记忆" }
    }
    fn color(&self) -> &'static str {
        match self { LifeState::Dormant=>"#333355", LifeState::Active=>"#4A90D9", LifeState::Shedding=>"#7B68EE",
            LifeState::Bound=>"#50B86C", LifeState::Remembered=>"#D98C4A" }
    }
    fn discriminant(&self) -> u8 {
        match self { LifeState::Dormant=>0, LifeState::Active=>1, LifeState::Shedding=>2,
            LifeState::Bound=>3, LifeState::Remembered=>4 }
    }
}

#[derive(Clone)]
struct LifeEntry {
    code: u8,
    state: LifeState,
    energy: u8,
    memory: u32,
    generation: u32,
}

impl LifeEntry {
    fn new(code: u8) -> Self {
        LifeEntry { code, state: LifeState::Dormant, energy: 50, memory: 0, generation: 0 }
    }

    fn step(&mut self, action: ActionKind, prev: Option<&LifeEntry>) {
        self.generation += 1;
        if let Some(p) = prev {
            self.memory = p.memory.wrapping_add(if p.state == LifeState::Remembered { 7 } else { 1 });
            self.energy = (p.energy as i16 + Self::delta(action)) as u8;
        } else {
            self.energy = (self.energy as i16 + Self::delta(action)) as u8;
        }
        self.state = match action {
            ActionKind::Call => LifeState::Active,
            ActionKind::Shed => LifeState::Shedding,
            ActionKind::Bind => LifeState::Bound,
            ActionKind::Judge => if self.energy > 30 { LifeState::Active } else { LifeState::Dormant },
            ActionKind::Store => LifeState::Remembered,
            ActionKind::Wake => LifeState::Active,
        };
        if self.energy < 20 {
            self.state = LifeState::Active;
        }
        self.energy = self.energy.min(100).max(0);
    }

    fn delta(action: ActionKind) -> i16 {
        match action {
            ActionKind::Call => -15, ActionKind::Shed => -25, ActionKind::Bind => -10,
            ActionKind::Judge => -5, ActionKind::Store => -8, ActionKind::Wake => 30,
        }
    }
}

struct ClockEngine {
    entries: Vec<LifeEntry>,
    tick: u32,
    trace: Vec<TickSnapshot>,
    max_ticks: u32,
}

#[derive(Clone)]
struct TickSnapshot {
    tick: u32,
    entries: Vec<LifeEntry>,
    dominant: ActionKind,
    total_memory: u32,
    total_energy: u32,
    n_active: u32,
    n_remembered: u32,
}

struct LcgRng { state: u64 }
impl LcgRng {
    fn new(seed: u64) -> Self { LcgRng { state: seed | 1 } }
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 32) as u32
    }
}

impl ClockEngine {
    fn new(max_ticks: u32) -> Self {
        ClockEngine {
            entries: (0u8..64).map(LifeEntry::new).collect(),
            tick: 0, trace: Vec::new(), max_ticks,
        }
    }

    fn run(&mut self) { for _ in 0..self.max_ticks { self.step(); } }

    fn step(&mut self) {
        let prev = self.entries.clone();
        let mut rng = LcgRng::new(self.tick.wrapping_add(0x9E3779B9) as u64);
        let dom_idx = (self.tick as usize + 3) % 6;
        let dominant = ACTIONS[dom_idx];
        let mut total_mem = 0u32;
        let mut total_energy = 0u32;
        let mut n_active = 0u32;
        let mut n_remembered = 0u32;

        for i in 0..64 {
            let action = Self::decide(i as u8, &self.entries[i], self.tick, &mut rng);
            let p = &prev[i];
            self.entries[i].step(action, Some(p));
            total_mem += self.entries[i].memory;
            total_energy += self.entries[i].energy as u32;
            if self.entries[i].state == LifeState::Active { n_active += 1; }
            if self.entries[i].state == LifeState::Remembered { n_remembered += 1; }
        }

        self.trace.push(TickSnapshot {
            tick: self.tick, entries: self.entries.clone(), dominant,
            total_memory: total_mem, total_energy, n_active, n_remembered,
        });
        self.tick += 1;
    }

    fn decide(code: u8, entry: &LifeEntry, tick: u32, rng: &mut LcgRng) -> ActionKind {
        let phase = ((code as u32 + tick) % 6) as usize;
        let base = ACTIONS[(phase + entry.state.discriminant() as usize) % 6];
        match entry.state {
            LifeState::Dormant => if entry.energy > 40 { ActionKind::Wake } else { base },
            LifeState::Active => if entry.energy < 40 { ActionKind::Store }
                else if entry.generation % 7 == 0 { ActionKind::Bind } else { base },
            LifeState::Shedding => ActionKind::Wake,
            LifeState::Bound => ActionKind::Judge,
            LifeState::Remembered => if tick % 5 == 0 { ActionKind::Call } else { ActionKind::Store },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML 生成 — 分文件写入避免原始字符串问题
// ═══════════════════════════════════════════════════════════════════════════

fn html_header() -> String {
    s(r#"<!DOCTYPE html><html lang="zh"><head><meta charset="UTF-8">
<title>时钟动作 — 容器生命周期</title>
<style>
body{font-family:"HanYuan64","Noto Sans CJK SC",sans-serif;background:#0a0a1a;color:#ddd;margin:0;padding:20px}
h1{font-size:22px;color:#fff;margin:4px 0}
.sub{color:#888;font-size:13px;margin-bottom:16px}
.controls{display:flex;gap:8px;align-items:center;margin-bottom:16px;flex-wrap:wrap}
button{background:#16213e;border:1px solid #444;color:#ccc;padding:6px 14px;border-radius:4px;cursor:pointer;font-size:13px}
button:hover{border-color:#7B68EE;color:#fff}
button.primary{background:#7B68EE;color:#fff;border-color:#7B68EE}
.info{font-size:12px;color:#888}
canvas{background:#05050f;border:1px solid #222;border-radius:8px;display:block}
.legend{display:flex;gap:10px;flex-wrap:wrap;margin:12px 0}
.lg{display:flex;align-items:center;gap:3px;font-size:11px;color:#888}
.dot{width:10px;height:10px;border-radius:50%}
.stats{display:flex;gap:12px;font-size:12px;color:#999;margin:10px 0}
.stat b{color:#fff;font-size:16px;display:block}
.section{color:#7B68EE;font-size:14px;margin:16px 0 8px 0}
.matrix{display:grid;grid-template-columns:repeat(8,1fr);gap:2px;max-width:500px}
.cell{background:#111;border-radius:2px;padding:4px;text-align:center;font-size:9px;cursor:pointer}
.cell:hover{transform:scale(1.3);z-index:1}
</style></head><body>
<h1>时钟动作 — 容器生命周期</h1>
<p class="sub">意志驱动时间 · 结束即开始 · 记忆跟随结束而继续</p>
<div class="controls">
<button class="primary" id="btnPlay" onclick="togglePlay()">▶ 播放</button>
<button onclick="resetView()">◀ 重置</button>
<span class="info">Tick: <span id="tickNum">0</span>/<span id="tickMax">0</span></span>
<span class="info">主导: <span id="domAction">-</span></span>
</div>
<div class="stats" id="statsBar">
<div class="stat" id="stActive"><b>-</b>活跃</div>
<div class="stat" id="stRem"><b>-</b>记忆体</div>
<div class="stat" id="stEnergy"><b>-</b>总能量</div>
<div class="stat" id="stMemory"><b>-</b>总记忆</div>
</div>
"#).to_string()
}

fn html_legend() -> String {
    let states = [
        ("#333355","休眠"), ("#4A90D9","活跃"), ("#7B68EE","脱壳"),
        ("#50B86C","绑定"), ("#D98C4A","记忆"),
    ];
    let mut html = String::from("<div class=\"legend\">");
    for (color, name) in &states {
        html.push_str(&format!("<div class=\"lg\"><span class=\"dot\" style=\"background:{}\"></span>{}</div>", color, name));
    }
    html.push_str("</div>");
    html
}

fn html_canvas_section() -> String {
    s(r#"<div class="section">空间生命周期 — 能量×状态</div>
<canvas id="lc" width="800" height="380"></canvas>
<div class="section">64容器 — 当前Tick状态矩阵</div>
<div class="matrix" id="matrix"></div>
<div class="section">总能量 / 总记忆 — 时间演化曲线</div>
<canvas id="cc" width="800" height="180"></canvas>
"#).to_string()
}

fn html_script_data(trace: &[TickSnapshot]) -> String {
    let mut s = String::new();
    writeln!(s, "<script>").unwrap();

    // Container names (Unicode handled as JS strings)  
    write!(s, "const names=[").unwrap();
    for (i, &name) in CONTAINER_NAMES.iter().enumerate() {
        if i > 0 { s.push(','); }
        write!(s, "\"{}\"", name).unwrap();
    }
    writeln!(s, "];").unwrap();

    writeln!(s, "const stateColors=[\"#333355\",\"#4A90D9\",\"#7B68EE\",\"#50B86C\",\"#D98C4A\"];").unwrap();
    writeln!(s, "const stateNames=[\"休眠\",\"活跃\",\"脱壳\",\"绑定\",\"记忆\"];").unwrap();
    writeln!(s, "const famColors=[\"#4A90D9\",\"#7B68EE\",\"#50B86C\",\"#D98C4A\"];").unwrap();
    writeln!(s, "const actionNames=[\"调用\",\"脱壳\",\"绑定\",\"裁决\",\"存储\",\"唤醒\"];").unwrap();

    // Trace data: each tick = [tick, energy, memory, active, remembered, dominant, ...entries×4]
    write!(s, "const trace=[").unwrap();
    for (ti, snap) in trace.iter().enumerate() {
        if ti > 0 { s.push(','); }
        let dom_idx = ACTIONS.iter().position(|&a| a == snap.dominant).unwrap_or(0);
        write!(s, "[{},{},{},{},{},{},", snap.tick, snap.total_energy, snap.total_memory, snap.n_active, snap.n_remembered, dom_idx).unwrap();
        for (ei, e) in snap.entries.iter().enumerate() {
            if ei > 0 { s.push(','); }
            write!(s, "{},{},{},{}",
                e.code, e.state.discriminant(), e.energy, e.generation.min(255)).unwrap();
        }
        s.push(']');
    }
    writeln!(s, "];").unwrap();
    s
}

fn s(s: &str) -> String { s.to_string() }

fn html_script_logic(max_mem: u32, max_energy: u32) -> String {
    let mut s = String::new();
    write!(s, "const maxMem={};\nconst maxEng={};\n", max_mem, max_energy);
    s.push_str("let playing=false,currentTick=null,timer=null,manualTick=null;\n");
    s.push_str("const lc=document.getElementById('lc'),lctx=lc.getContext('2d');\n");
    s.push_str("const cc=document.getElementById('cc'),cctx=cc.getContext('2d');\n");
    s.push_str("\n");
    s.push_str("function drawLifecycle(t){\n");
    s.push_str("  const W=lc.width,H=lc.height;\n");
    s.push_str("  lctx.fillStyle='#05050f';lctx.fillRect(0,0,W,H);\n");
    s.push_str("  if(t<0||t>=trace.length)return;\n");
    s.push_str("  const s=trace[t];const entries=s.slice(6);\n");
    s.push_str("  for(let i=0;i<64;i++){\n");
    s.push_str("    const code=entries[i*4],state=entries[i*4+1],eng=entries[i*4+2];\n");
    s.push_str("    const x=code%8*(W/8)+W/16;\n");
    s.push_str("    const y=Math.floor(code/8)*(H/8)+H/16;\n");
    s.push_str("    const r=3+(eng/100)*5;\n");
    s.push_str("    lctx.beginPath();lctx.arc(x,y,r,0,Math.PI*2);\n");
    s.push_str("    lctx.fillStyle=stateColors[state];lctx.fill();\n");
    s.push_str("    if(eng>40){lctx.font='8px sans-serif';lctx.fillStyle='#fff';lctx.textAlign='center';lctx.fillText(code.toString(16).padStart(2,'0'),x,y+r+8);}\n");
    s.push_str("  }\n");
    s.push_str("}\n");
    s.push_str("\n");
    s.push_str("function drawCurve(t){\n");
    s.push_str("  const W=cc.width,H=cc.height;\n");
    s.push_str("  cctx.fillStyle='#05050f';cctx.fillRect(0,0,W,H);\n");
    s.push_str("  drawLine(cctx,'#D98C4A',trace,1,maxMem,t,H);\n");
    s.push_str("  drawLine(cctx,'#4A90D9',trace,2,maxEng,t,H);\n");
    s.push_str("  if(t>0){const x=(t/trace.length)*W;cctx.strokeStyle='#fff';cctx.setLineDash([2,2]);cctx.beginPath();cctx.moveTo(x,0);cctx.lineTo(x,H);cctx.stroke();cctx.setLineDash([]);}\n");
    s.push_str("}\n");
    s.push_str("function drawLine(ctx,color,arr,idx,maxv,tick,H){\n");
    s.push_str("  ctx.strokeStyle=color;ctx.lineWidth=1.5;ctx.beginPath();\n");
    s.push_str("  for(let i=0;i<=tick&&i<arr.length;i++){\n");
    s.push_str("    const y=H-(arr[i][idx]/maxv)*H*0.92;\n");
    s.push_str("    const x=(i/arr.length)*cc.width;\n");
    s.push_str("    if(i===0)ctx.moveTo(x,y);else ctx.lineTo(x,y);\n");
    s.push_str("  }\n");
    s.push_str("  ctx.stroke();\n");
    s.push_str("}\n");
    s.push_str("\n");
    s.push_str("function updateMatrix(t){\n");
    s.push_str("  const m=document.getElementById('matrix');\n");
    s.push_str("  if(!m||t<0||t>=trace.length)return;\n");
    s.push_str("  const s=trace[t];const entries=s.slice(6);let html='';\n");
    s.push_str("  for(let i=0;i<64;i++){\n");
    s.push_str("    const code=entries[i*4],state=entries[i*4+1],eng=entries[i*4+2];\n");
    s.push_str("    const c=names[code]||'?';\n");
    s.push_str("    html+='<div class=\\'cell\\' style=\\'background:'+stateColors[state]+'\\' title=\\'0x'+code.toString(16).padStart(2,'0')+' '+c+'\\'>'+c.charAt(0)+'<br><span style=\\'color:#'+(eng>50?'6f6':'f66')+'\\'>'+eng+'</span></div>';\n");
    s.push_str("  }\n");
    s.push_str("  m.innerHTML=html;\n");
    s.push_str("}\n");
    s.push_str("\n");
    s.push_str("function render(t){\n");
    s.push_str("  if(t<0)t=0;if(t>=trace.length)t=trace.length-1;\n");
    s.push_str("  drawLifecycle(t);drawCurve(t);updateMatrix(t);\n");
    s.push_str("  const s=trace[t];\n");
    s.push_str("  document.getElementById('domAction').textContent=actionNames[s[5]]||'-';\n");
    s.push_str("  document.getElementById('stActive').children[0].textContent=s[3];\n");
    s.push_str("  document.getElementById('stRem').children[0].textContent=s[4];\n");
    s.push_str("}\n");
    s.push_str("\n");
    s.push_str("function togglePlay(){\n");
    s.push_str("  playing=!playing;\n");
    s.push_str("  document.getElementById('btnPlay').textContent=playing?'⏸ 暂停':'▶ 播放';\n");
    s.push_str("  if(playing)timer=setInterval(function(){var t=(manualTick!==null?manualTick:0);t=(t+1)%trace.length;manualTick=t;render(t);if(t===0&&manualTick!==null){playing=false;clearInterval(timer);document.getElementById('btnPlay').textContent='▶ 播放';}},180);\n");
    s.push_str("  else clearInterval(timer);\n");
    s.push_str("}\n");
    s.push_str("function resetView(){manualTick=0;render(0);}\n");
    s.push_str("document.getElementById('tickMax').textContent=trace.length;\n");
    s.push_str("render(0);\n");
    s.push_str("</script>\n</body></html>\n");
    return s;
}


fn full_html(trace: &[TickSnapshot]) -> String {
    let max_mem = trace.iter().map(|s| s.total_memory).max().unwrap_or(1);
    let max_energy = trace.iter().map(|s| s.total_energy).max().unwrap_or(1);

    let mut html = String::new();
    html.push_str(&html_header());
    html.push_str(&html_legend());
    html.push_str(&html_canvas_section());
    html.push_str(&html_script_data(trace));
    html.push_str(&html_script_logic(max_mem, max_energy));
    html
}

fn trace_json(trace: &[TickSnapshot]) -> String {
    let mut j = String::new();
    writeln!(j, "{{").unwrap();
    if let Some(first) = trace.first() {
        write!(j, "  \"containers\": [").unwrap();
        for (i, e) in first.entries.iter().enumerate() {
            if i > 0 { j.push(','); }
            write!(j, r#"{{"code":{},"name":"{}","state":"{}","energy":{},"memory":{}}}"#,
                e.code, CONTAINER_NAMES[e.code as usize], e.state.name(), e.energy, e.memory).unwrap();
        }
        writeln!(j, "],").unwrap();
    }
    write!(j, "  \"timeline\": [").unwrap();
    for (ti, s) in trace.iter().enumerate() {
        if ti > 0 { j.push(','); }
        write!(j, r#"{{"tick":{},"dominant":"{}","total_memory":{},"total_energy":{},"active":{},"remembered":{}}}"#,
            s.tick, s.dominant.name(), s.total_memory, s.total_energy, s.n_active, s.n_remembered).unwrap();
    }
    writeln!(j, "]").unwrap();
    write!(j, "}}").unwrap();
    j
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let out_dir = if args.len() > 1 { args[1].clone() } else { ".".to_string() };
    let max_ticks: u32 = if args.len() > 2 { args[2].parse().unwrap_or(64) } else { 64 };

    let mut engine = ClockEngine::new(max_ticks);
    engine.run();

    let html = full_html(&engine.trace);
    let json = trace_json(&engine.trace);

    let hp = format!("{}/clock_lifecycle.html", out_dir);
    let jp = format!("{}/clock_trace.json", out_dir);
    std::fs::write(&hp, &html).expect("write html");
    std::fs::write(&jp, &json).expect("write json");

    eprintln!("时钟动作生命周期:");
    eprintln!("  HTML: {} ({} bytes)", hp, html.len());
    eprintln!("  JSON:  {} ({} bytes)", jp, json.len());
    if let Some(last) = engine.trace.last() {
        eprintln!("  T{}: 能量={} 记忆={} 活跃={} 记忆体={}",
            last.tick, last.total_energy, last.total_memory, last.n_active, last.n_remembered);
    }
}
