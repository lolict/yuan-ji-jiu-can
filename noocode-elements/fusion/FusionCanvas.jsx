/**
 * FusionCanvas.jsx — 一体化协议共同体可视化
 * 融合论 · 共同体论 · 造字论 · 一体成型同构论
 */

import { useRef, useEffect, useState } from 'react';

const FUSION_ENGINE_PATH = '/src/lib/FusionEngine.js';

export function FusionCanvas({ engine, selectedEncoding = '融合网' }) {
  const canvasRef = useRef(null);
  const [time, setTime] = useState(0);

  useEffect(() => {
    const id = setInterval(() => setTime(t => t + 1), 60);
    return () => clearInterval(id);
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width = 700;
    const h = canvas.height = 700;
    const cx = w / 2;
    const cy = h / 2;

    // 背景
    ctx.fillStyle = '#04040a';
    ctx.fillRect(0, 0, w, h);

    // 加载引擎
    const protocolNames = ['G', 'HTTP', 'HTTPS', 'TCP', 'CP', 'FTP', 'GTT', 'GTTX', 'GITT', 'GTIT', 'GTITX', 'GI'];
    const positions = protocolNames.map((name, i) => {
      const hash = name.split('').reduce((a, c) => a + c.charCodeAt(0), 0);
      const angle = (hash % 360) * Math.PI / 180 + time * 0.002;
      const radiusMap = { G: 45, HTTP: 165, HTTPS: 185, TCP: 225, CP: 245, FTP: 205, GTT: 105, GTTX: 120, GITT: 85, GTIT: 95, GTITX: 70, GI: 55 };
      const radius = radiusMap[name] || 140;
      const sizeMap = { G: 10, HTTP: 18, HTTPS: 18, TCP: 22, CP: 24, FTP: 20, GTT: 16, GTTX: 16, GITT: 14, GTIT: 14, GTITX: 14, GI: 10 };
      return {
        name,
        x: cx + Math.cos(angle) * radius,
        y: cy + Math.sin(angle) * radius,
        radius: sizeMap[name] || 16,
        polarity: name === 'G' || name === 'GTT' || name === 'GTTX' ? -1 : 1,
        base: name === 'G' || name === 'GI' ? 2 : name.startsWith('HTTP') ? 16 : name.startsWith('GTT') || name === 'GITT' || name === 'GTIT' || name === 'GTITX' ? 64 : 256,
        nature: getNature(name),
        orbitRadius: radius,
        angle: angle * 180 / Math.PI,
      };
    });

    // 绘制轨道
    [45, 70, 95, 120, 145, 165, 185, 205, 225, 245].forEach(r => {
      ctx.strokeStyle = 'rgba(255,255,255,0.05)';
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.arc(cx, cy, r, 0, Math.PI * 2);
      ctx.stroke();
    });

    // 绘制中心锚
    ctx.fillStyle = 'rgba(255,200,100,0.9)';
    ctx.beginPath();
    ctx.arc(cx, cy, 8, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#000';
    ctx.font = 'bold 10px monospace';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText('锚', cx, cy + 1);

    // 绘制融合链路
    for (let i = 0; i < positions.length; i++) {
      for (let j = i + 1; j < positions.length; j++) {
        drawFusionLink(ctx, positions[i], positions[j]);
      }
    }

    // 绘制协议节点
    positions.forEach(p => drawNode(ctx, p));

    // 底部状态
    ctx.fillStyle = 'rgba(34,197,94,0.8)';
    ctx.font = '10px monospace';
    ctx.textAlign = 'left';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText(`协议共同体 v1.0 · 进制:2/16/64/256 · 节点#${positions.length} · 刘楚恬和我`, 10, h - 12);

  }, [selectedEncoding, time]);

  return (
    <div>
      <canvas ref={canvasRef} width={700} height={700} className="w-full" style={{ aspectRatio: '1' }} />
    </div>
  );
}

function getNature(name) {
  const map = {
    G: '基底', HTTP: '表象', HTTPS: '加密', TCP: '流控', CP: '伴侣', FTP: '文件',
    GTT: '通用目标', GTTX: '扩展通用', GITT: '意图传输', GTIT: '目标意图', GTITX: '扩展意图', GI: '感知',
  };
  return map[name] || name;
}

function drawNode(ctx, p) {
  const isYin = p.polarity === -1;
  const color = isYin ? 'rgba(129,140,248,0.9)' : 'rgba(251,191,36,0.9)';
  const glow = isYin ? 'rgba(129,140,248,0.3)' : 'rgba(251,191,36,0.3)';

  // 外圈
  ctx.fillStyle = glow;
  ctx.beginPath();
  ctx.arc(p.x, p.y, p.radius + 4, 0, Math.PI * 2);
  ctx.fill();

  // 主体
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
  ctx.fill();

  // 符号
  ctx.fillStyle = '#000';
  ctx.font = `bold ${Math.round(p.radius * 0.7)}px monospace`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  const sym = p.base === 2 ? '⚊' : p.base === 16 ? '目' : p.base === 64 ? '⬡' : '◉';
  ctx.fillText(sym, p.x, p.y + 1);
  ctx.textBaseline = 'alphabetic';

  // 名称
  ctx.fillStyle = 'rgba(255,255,255,0.85)';
  ctx.font = '9px monospace';
  ctx.textAlign = 'center';
  ctx.fillText(p.name, p.x, p.y - p.radius - 8);

  // 进制标注
  ctx.fillStyle = 'rgba(255,180,100,0.7)';
  ctx.font = '8px monospace';
  ctx.fillText(`b${p.base}`, p.x, p.y - p.radius - 18);
}

function drawFusionLink(ctx, a, b) {
  const dist = Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2);
  const maxDist = 350;
  const strength = Math.max(0, 1 - dist / maxDist);

  if (strength < 0.05) return;

  const isCouple = a.polarity !== b.polarity;
  const baseColor = isCouple ? '255,100,150' : '100,200,255';
  const alpha = strength * 0.4;

  ctx.strokeStyle = `rgba(${baseColor},${alpha})`;
  ctx.lineWidth = 0.5 + strength * 2;
  ctx.beginPath();
  ctx.moveTo(a.x, a.y);

  if (isCouple) {
    const midX = (a.x + b.x) / 2 + (a.y - b.y) * 0.15;
    const midY = (a.y + b.y) / 2 + (b.x - a.x) * 0.15;
    ctx.quadraticCurveTo(midX, midY, b.x, b.y);
  } else {
    ctx.lineTo(b.x, b.y);
  }
  ctx.stroke();

  // 中点标注
  if (strength > 0.3) {
    const mx = (a.x + b.x) / 2;
    const my = (a.y + b.y) / 2;
    ctx.fillStyle = `rgba(255,255,255,${alpha * 0.8})`;
    ctx.font = '8px monospace';
    ctx.textAlign = 'center';
    ctx.fillText(`${(strength * 100).toFixed(0)}%`, mx, my);
  }
}

export default FusionCanvas;
