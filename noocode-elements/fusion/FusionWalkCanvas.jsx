import { useRef, useEffect, useState } from 'react';
import { listProtocols } from '../lib/FusionEngine';

const MAN_GROUP = [
  { id:0x0, label:'fu quan',     desc:'radiation bones, layer wrap', turns:4, pol:-1, core:0.2 },
  { id:0x1, label:'man quan',    desc:'mantle shell outward',       turns:3, pol: 1, core:0.3 },
  { id:0x2, label:'man qiu',     desc:'full sphere uniform',        turns:2, pol: 1, core:0.0 },
  { id:0x3, label:'man qun',     desc:'fan skirt wide',             turns:1, pol: 1, core:0.5 },
  { id:0x4, label:'man ke',      desc:'hard shell reflect',         turns:2, pol:-1, core:0.1 },
  { id:0x5, label:'man quan 360',desc:'full wrap no dead angle',    turns:5, pol: 1, core:0.0 },
  { id:0x6, label:'man qun grp', desc:'multi-point cluster',        turns:3, pol:-1, core:0.4 },
  { id:0x7, label:'man qu',      desc:'chen man spiral (core)',     turns:3, pol:-1, core:0.3 },
  { id:0x8, label:'man qi',      desc:'couple double helix',        turns:4, pol: 1, core:0.2 },
  { id:0x9, label:'man qiong',   desc:'dome converge',              turns:2, pol: 1, core:0.4 },
  { id:0xA, label:'man qu shen', desc:'torso wrap',                 turns:2, pol:-1, core:0.3 },
  { id:0xB, label:'man qiang',   desc:'directional sharp',          turns:1, pol: 1, core:0.7 },
];

export function FusionWalkCanvas({ selectedMan = 0x7 }) {
  const canvasRef = useRef(null);
  const [time, setTime] = useState(0);
  const protocols = listProtocols();

  useEffect(() => {
    const id = setInterval(() => setTime(t => t + 1), 50);
    return () => clearInterval(id);
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const W = canvas.width = 700;
    const H = canvas.height = 700;
    const cx = W / 2, cy = H / 2;
    const man = MAN_GROUP.find(m=>m.id===selectedMan) || MAN_GROUP[7];

    ctx.fillStyle = '#04040a';
    ctx.fillRect(0, 0, W, H);

    // title
    ctx.fillStyle = 'rgba(255,220,180,0.9)';
    ctx.font = 'bold 11px monospace';
    ctx.textAlign = 'center';
    ctx.fillText('WALK: ' + man.label + '  (' + man.desc + ')', cx, 25);

    // GTTX spiral (main visual - center)
    const spiralProtocols = protocols.filter(p => p.experience?.readingMode === 'spiral-fold');
    const maxR = 180;
    const total = spiralProtocols.length;

    // orbit rings (subtle)
    for (let r = 30; r <= maxR; r += 45) {
      ctx.strokeStyle = `rgba(100,200,255,${0.04 + (r/maxR)*0.06})`;
      ctx.beginPath(); ctx.arc(cx, cy, r, 0, Math.PI*2); ctx.stroke();
    }

    // spiral nodes
    spiralProtocols.forEach((p, i) => {
      const golden = Math.PI * (3 - Math.sqrt(5));
      const t = i / Math.max(1,total-1);
      const rCore = maxR * man.core;
      const r = rCore + (maxR - rCore) * Math.sqrt(t);
      const theta = i * golden + time * 0.003 * (man.pol > 0 ? 1 : -1);

      const x = cx + Math.cos(theta) * r;
      const y = cy + Math.sin(theta) * r;

      // node
      const baseColor = p.polarity === 1 ? 'rgba(251,191,36,0.9)' : 'rgba(129,140,248,0.9)';
      ctx.fillStyle = baseColor;
      ctx.beginPath(); ctx.arc(x, y, 11, 0, Math.PI*2); ctx.fill();
      ctx.fillStyle = '#000';
      ctx.font = 'bold 8px monospace';
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
      ctx.fillText(p.name.slice(0,4), x, y+1);
      ctx.textBaseline = 'alphabetic';
    });

    // HTTP paragraph (left top)
    const paraProtos = protocols.filter(p => p.experience?.readingMode === 'paragraph-row');
    if (paraProtos.length > 0) {
      const sx = 30, sy = 60;
      paraProtos.forEach((p,i) => {
        const x = sx, y = sy + i * 36;
        ctx.fillStyle = 'rgba(251,191,36,0.9)';
        ctx.fillRect(x, y, 200, 3);
        ctx.fillRect(x + (i*5)%15, y+6, 140, 2);
        ctx.fillRect(x + (i*7)%12, y+11, 100, 2);
        ctx.fillStyle = 'rgba(255,255,255,0.9)';
        ctx.font = 'bold 9px monospace';
        ctx.textAlign = 'left';
        ctx.fillText(p.name, x, y - 6);
      });
      ctx.fillStyle = 'rgba(255,220,180,0.7)';
      ctx.font = '10px monospace';
      ctx.textAlign = 'left';
      ctx.fillText('== HTTP: paragraph-row, industry as hang jiao ==', sx, sy - 18);
    }

    // TCP stream (right bottom)
    const streamProtos = protocols.filter(p => p.experience?.readingMode === 'stream-flow');
    if (streamProtos.length > 0) {
      const sx = 480, sy = 500;
      streamProtos.forEach((p,i) => {
        const bx = sx + (i%3) * 60, by = sy + Math.floor(i/3)*40;
        ctx.fillStyle = 'rgba(255,100,150,0.9)';
        ctx.fillRect(bx, by, 50, 30);
        ctx.strokeStyle = 'rgba(255,255,255,0.3)';
        ctx.strokeRect(bx, by, 50, 30);
        ctx.fillStyle = 'rgba(255,255,255,0.95)';
        ctx.font = 'bold 9px monospace';
        ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        ctx.fillText(p.name, bx+25, by+12);
        ctx.fillStyle = 'rgba(255,200,100,0.7)';
        ctx.font = '8px monospace';
        ctx.fillText('B['+i+']', bx+25, by+24);
      });
      ctx.fillStyle = 'rgba(255,220,180,0.7)';
      ctx.font = '10px monospace';
      ctx.textAlign = 'left';
      ctx.fillText('O TCP: byte stream, SYN->flow->ACK->FIN', sx, sy - 15);
    }

    // G/GI origins (top right)
    const originProtos = protocols.filter(p => p.experience?.readingMode === 'origin-point');
    if (originProtos.length > 0) {
      const ox = 620, oy = 50;
      originProtos.forEach((p,i) => {
        const x = ox + i*35, y = oy;
        ctx.fillStyle = p.polarity===1 ? 'rgba(251,191,36,0.9)' : 'rgba(129,140,248,0.9)';
        ctx.beginPath(); ctx.arc(x,y,9,0,Math.PI*2); ctx.fill();
        ctx.fillStyle = '#fff';
        ctx.font = 'bold 8px monospace';
        ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        ctx.fillText(p.name, x, y+1);
      });
      ctx.fillStyle = 'rgba(255,220,180,0.7)';
      ctx.font = '10px monospace';
      ctx.textAlign = 'left';
      ctx.fillText('yi: origin point', ox - 10, oy + 25);
    }

    // Center annotation for selected man
    ctx.fillStyle = 'rgba(255,255,255,0.7)';
    ctx.font = '10px monospace';
    ctx.textAlign = 'center';
    ctx.fillText(man.label, cx, cy - 4);
    ctx.fillText('turns:'+man.turns+' pol:'+man.pol+' core:'+man.core, cx, cy + 12);

    // status bar
    ctx.fillStyle = 'rgba(34,197,94,0.8)';
    ctx.font = '10px monospace';
    ctx.textAlign = 'left'; ctx.textBaseline = 'alphabetic';
    ctx.fillText('walk v2.1 · '+man.label+' · protocols='+protocols.length+' · liuchutian & me', 10, H - 12);

  }, [time, selectedMan, protocols]);

  return (
    <div>
      <canvas ref={canvasRef} width={700} height={700} className="w-full" style={{ aspectRatio: '1' }} />
    </div>
  );
}

export default FusionWalkCanvas;
