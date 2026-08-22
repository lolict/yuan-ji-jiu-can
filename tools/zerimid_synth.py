#!/usr/bin/env python3
"""
零维码发音合成器（Formant Synthesis）
输入：声母/韵母/声调编码
输出：WAV 音频文件（纯算法生成，无需人声采样）

原理：
- 浊音 = 声带振动（脉冲序列）+ 声道形状（共振峰滤波）
- 清音 = 白噪声 + 滤波
- 声调 = 基频 F0 的变化曲线
"""

import numpy as np
import wave
import struct
import os

SAMPLE_RATE = 22050
AMPLITUDE = 0.3

# ============================================================
# 声学参数表：每个编码 → 合成参数
# ============================================================

CONSONANT_PARAMS = {
    # 编码: (F0, F1, F2, F3, 清浊, 摩擦度, 时长ms)
    'I01': {'sym': 'ɓ', 'f0': 120, 'f1': 300, 'f2': 800,  'f3': 2200, 'voiced': True,  'noise': 0.0, 'dur': 80},  # 内爆b
    'I02': {'sym': 'ƃ', 'f0': 115, 'f1': 350, 'f2': 850,  'f3': 2300, 'voiced': True,  'noise': 0.0, 'dur': 80},  # 阻塞b
    'I03': {'sym': 'ƀ', 'f0': 110, 'f1': 320, 'f2': 820,  'f3': 2250, 'voiced': True,  'noise': 0.0, 'dur': 80},
    'I04': {'sym': 'ƥ', 'f0': 130, 'f1': 280, 'f2': 750,  'f3': 2100, 'voiced': True,  'noise': 0.0, 'dur': 80},
    'I05': {'sym': 'ɗ', 'f0': 105, 'f1': 350, 'f2': 1600, 'f3': 2500, 'voiced': True,  'noise': 0.0, 'dur': 80},  # 内爆d
    'I06': {'sym': 'ɖ', 'f0': 100, 'f1': 400, 'f2': 1200, 'f3': 2400, 'voiced': True,  'noise': 0.0, 'dur': 80},
    'I07': {'sym': 'ƌ', 'f0': 110, 'f1': 380, 'f2': 1500, 'f3': 2450, 'voiced': True,  'noise': 0.1, 'dur': 80},
    'I08': {'sym': 'ʈ', 'f0': 140, 'f1': 300, 'f2': 1700, 'f3': 2600, 'voiced': False, 'noise': 0.8, 'dur': 60},  # 清塞
    'I09': {'sym': 'ɠ', 'f0': 95,  'f1': 250, 'f2': 1400, 'f3': 2300, 'voiced': True,  'noise': 0.0, 'dur': 80},
    'I10': {'sym': 'ɣ', 'f0': 100, 'f1': 300, 'f2': 1300, 'f3': 2350, 'voiced': True,  'noise': 0.3, 'dur': 100}, # 浊擦
    'I11': {'sym': 'ƙ', 'f0': 145, 'f1': 200, 'f2': 1000, 'f3': 2100, 'voiced': False, 'noise': 0.7, 'dur': 60},
    'I12': {'sym': 'ʞ', 'f0': 150, 'f1': 180, 'f2': 900,  'f3': 2000, 'voiced': False, 'noise': 0.9, 'dur': 50},
    'I13': {'sym': 'ɲ', 'f0': 135, 'f1': 350, 'f2': 1800, 'f3': 2700, 'voiced': True,  'noise': 0.0, 'dur': 100}, # 腭化鼻
    'I14': {'sym': 'ɟ', 'f0': 140, 'f1': 320, 'f2': 1900, 'f3': 2800, 'voiced': True,  'noise': 0.2, 'dur': 70},
    'I15': {'sym': 'ɧ', 'f0': 130, 'f1': 300, 'f2': 1700, 'f3': 2600, 'voiced': True,  'noise': 0.4, 'dur': 100},
    'I16': {'sym': 'ʝ', 'f0': 125, 'f1': 330, 'f2': 1850, 'f3': 2750, 'voiced': True,  'noise': 0.3, 'dur': 90},
    'I17': {'sym': 'ɦ', 'f0': 115, 'f1': 400, 'f2': 1200, 'f3': 2200, 'voiced': True,  'noise': 0.5, 'dur': 100}, # 浊喉擦
    'I18': {'sym': 'ʔ', 'f0': 100, 'f1': 500, 'f2': 1000, 'f3': 2000, 'voiced': False, 'noise': 0.0, 'dur': 40},  # 声门塞
    'I19': {'sym': 'ʕ', 'f0': 90,  'f1': 450, 'f2': 900,  'f3': 1800, 'voiced': True,  'noise': 0.2, 'dur': 100},
    'I20': {'sym': 'ł', 'f0': 120, 'f1': 350, 'f2': 1100, 'f3': 2300, 'voiced': True,  'noise': 0.3, 'dur': 100},
    'I21': {'sym': 'ɬ', 'f0': 0,   'f1': 300, 'f2': 1500, 'f3': 2800, 'voiced': False, 'noise': 0.95,'dur': 120}, # 清边擦
    'I22': {'sym': 'ɭ', 'f0': 110, 'f1': 400, 'f2': 1000, 'f3': 2200, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I23': {'sym': 'ʎ', 'f0': 130, 'f1': 350, 'f2': 1900, 'f3': 2800, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I24': {'sym': 'ʃ', 'f0': 0,   'f1': 250, 'f2': 2000, 'f3': 3000, 'voiced': False, 'noise': 1.0, 'dur': 120}, # 清龈后擦
    'I25': {'sym': 'ʆ', 'f0': 0,   'f1': 280, 'f2': 1900, 'f3': 2900, 'voiced': False, 'noise': 1.0, 'dur': 120},
    'I26': {'sym': 'ʂ', 'f0': 0,   'f1': 300, 'f2': 1800, 'f3': 2800, 'voiced': False, 'noise': 1.0, 'dur': 120},
    'I27': {'sym': 'ƒ', 'f0': 0,   'f1': 200, 'f2': 1000, 'f3': 2500, 'voiced': False, 'noise': 0.9, 'dur': 110}, # 清唇齿擦
    'I28': {'sym': 'ɱ', 'f0': 130, 'f1': 350, 'f2': 1200, 'f3': 2400, 'voiced': True,  'noise': 0.0, 'dur': 100}, # 唇齿鼻
    'I29': {'sym': 'ɳ', 'f0': 100, 'f1': 450, 'f2': 1100, 'f3': 2300, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I30': {'sym': 'ŋ', 'f0': 95,  'f1': 300, 'f2': 1000, 'f3': 2200, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I31': {'sym': 'ɴ', 'f0': 110, 'f1': 350, 'f2': 1400, 'f3': 2500, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I32': {'sym': 'ʋ', 'f0': 125, 'f1': 320, 'f2': 1100, 'f3': 2350, 'voiced': True,  'noise': 0.1, 'dur': 90},
    'I33': {'sym': 'ʍ', 'f0': 0,   'f1': 250, 'f2': 700,  'f3': 2000, 'voiced': False, 'noise': 0.85,'dur': 110}, # 清唇喉擦
    'I34': {'sym': 'ƕ', 'f0': 105, 'f1': 300, 'f2': 1300, 'f3': 2400, 'voiced': True,  'noise': 0.3, 'dur': 100},
    'I35': {'sym': 'ʁ', 'f0': 100, 'f1': 400, 'f2': 1200, 'f3': 2000, 'voiced': True,  'noise': 0.4, 'dur': 100},
    'I36': {'sym': 'ɹ', 'f0': 120, 'f1': 450, 'f2': 1100, 'f3': 2200, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I37': {'sym': 'ɺ', 'f0': 130, 'f1': 380, 'f2': 1500, 'f3': 2600, 'voiced': True,  'noise': 0.0, 'dur': 70},
    'I38': {'sym': 'ɻ', 'f0': 110, 'f1': 450, 'f2': 1000, 'f3': 2100, 'voiced': True,  'noise': 0.0, 'dur': 100},
    'I39': {'sym': 'ɼ', 'f0': 115, 'f1': 420, 'f2': 1300, 'f3': 2300, 'voiced': True,  'noise': 0.2, 'dur': 100},
    'I40': {'sym': 'ɽ', 'f0': 100, 'f1': 400, 'f2': 1100, 'f3': 2200, 'voiced': True,  'noise': 0.1, 'dur': 70},
    'I41': {'sym': 'ɾ', 'f0': 135, 'f1': 380, 'f2': 1600, 'f3': 2700, 'voiced': True,  'noise': 0.0, 'dur': 50},
    'I42': {'sym': 'ɍ', 'f0': 110, 'f1': 420, 'f2': 1100, 'f3': 2150, 'voiced': True,  'noise': 0.1, 'dur': 90},
    'I43': {'sym': 'ʦ', 'f0': 0,   'f1': 300, 'f2': 2500, 'f3': 3500, 'voiced': False, 'noise': 1.0, 'dur': 100}, # 清齿龈塞擦
    'I44': {'sym': 'ʧ', 'f0': 0,   'f1': 280, 'f2': 2200, 'f3': 3200, 'voiced': False, 'noise': 1.0, 'dur': 100},
    'I45': {'sym': 'ʨ', 'f0': 0,   'f1': 320, 'f2': 2400, 'f3': 3400, 'voiced': False, 'noise': 1.0, 'dur': 100},
    'I46': {'sym': 'ɏ', 'f0': 125, 'f1': 350, 'f2': 1900, 'f3': 2800, 'voiced': True,  'noise': 0.1, 'dur': 90},
    'I47': {'sym': 'ƴ', 'f0': 120, 'f1': 340, 'f2': 1850, 'f3': 2750, 'voiced': True,  'noise': 0.0, 'dur': 80},
    'I48': {'sym': 'ɨ', 'f0': 130, 'f1': 300, 'f2': 2200, 'f3': 2800, 'voiced': True,  'noise': 0.1, 'dur': 90},
}

VOWEL_PARAMS = {
    'F01': {'sym': 'ɐ', 'f1': 650, 'f2': 1200, 'f3': 2500},  # 近开前
    'F02': {'sym': 'ɑ', 'f1': 750, 'f2': 1100, 'f3': 2400},  # 开后
    'F03': {'sym': 'ɒ', 'f1': 600, 'f2': 900,  'f3': 2200},  # 开后圆
    'F04': {'sym': 'ɔ', 'f1': 500, 'f2': 850,  'f3': 2200},  # 半开o
    'F05': {'sym': 'ɛ', 'f1': 500, 'f2': 1700, 'f3': 2400},  # 半开e
    'F06': {'sym': 'ɜ', 'f1': 550, 'f2': 1400, 'f3': 2300},  # 中央
    'F07': {'sym': 'ɘ', 'f1': 450, 'f2': 1600, 'f3': 2500},
    'F08': {'sym': 'ə', 'f1': 500, 'f2': 1500, 'f3': 2500},  # 基准韵
    'F09': {'sym': 'ɚ', 'f1': 500, 'f2': 1300, 'f3': 2200},
    'F10': {'sym': 'ǝ', 'f1': 520, 'f2': 1450, 'f3': 2450},
    'F11': {'sym': 'ɝ', 'f1': 480, 'f2': 1350, 'f3': 2350},
    'F12': {'sym': 'ɨ', 'f1': 350, 'f2': 2200, 'f3': 2800},
    'F13': {'sym': 'ɪ', 'f1': 400, 'f2': 2000, 'f3': 2700},
    'F14': {'sym': 'ɯ', 'f1': 300, 'f2': 1000, 'f3': 2200},
    'F15': {'sym': 'ʉ', 'f1': 350, 'f2': 1600, 'f3': 2600},
    'F16': {'sym': 'ʊ', 'f1': 400, 'f2': 1000, 'f3': 2300},
    'F17': {'sym': 'ɵ', 'f1': 450, 'f2': 1400, 'f3': 2500},
    'F18': {'sym': 'ø', 'f1': 400, 'f2': 1500, 'f3': 2600},
    'F19': {'sym': 'œ', 'f1': 500, 'f2': 1400, 'f3': 2300},
    'F20': {'sym': 'ɶ', 'f1': 600, 'f2': 1300, 'f3': 2200},
}

# 声调 = F0 曲线
TONE_CURVES = {
    'T01': {'name': '阴平', 'curve': [55, 55, 55, 55, 55]},
    'T02': {'name': '阳平', 'curve': [35, 35, 35, 35, 35]},
    'T03': {'name': '上声', 'curve': [214, 214, 100, 214, 214]},
    'T04': {'name': '去声', 'curve': [51, 51, 51, 51, 51]},
    'T05': {'name': '颤音', 'curve': [45, 55, 40, 55, 45]},
}


# ============================================================
# 合成引擎
# ============================================================

def generate_glottal_pulse(f0, duration_ms, sr=SAMPLE_RATE):
    """生成声带脉冲序列（浊音源）"""
    n_samples = int(sr * duration_ms / 1000)
    if f0 == 0:
        return np.zeros(n_samples)
    
    period = sr / f0
    n_pulses = int(n_samples / period) + 1
    
    t_pulse = np.arange(int(period)) / sr
    # LF 模型简化：开相→峰值→指数衰减
    pulse = np.zeros(int(period))
    open_len = int(period * 0.4)
    pulse[:open_len] = np.linspace(0, 1, open_len)
    pulse[open_len:] = np.exp(-np.arange(int(period) - open_len) / (period * 0.15))
    
    signal = np.zeros(n_samples)
    for i in range(n_pulses):
        start = int(i * period)
        if start < n_samples:
            end = min(start + len(pulse), n_samples)
            signal[start:end] = pulse[:end - start]
    
    return signal[:n_samples]


def generate_noise(duration_ms, sr=SAMPLE_RATE):
    """生成白噪声（清音摩擦源）"""
    n_samples = int(sr * duration_ms / 1000)
    return np.random.randn(n_samples) * 0.5


def formant_filter(signal, f1, f2, f3, bandwidth=(80, 100, 120), sr=SAMPLE_RATE):
    """共振峰滤波器（给信号加上声道形状）"""
    from scipy.signal import lfilter
    
    def resonance(freq, bw):
        r = np.exp(-np.pi * bw / sr)
        theta = 2 * np.pi * freq / sr
        b = [1 - r]
        a = [1, -2 * r * np.cos(theta), r * r]
        return b, a
    
    # 级联三个共振峰
    for f, bw in zip([f1, f2, f3], bandwidth):
        b, a = resonance(f, bw)
        signal = lfilter(b, a, signal)
    
    return signal


def synthesize_consonant(code):
    """合成一个声母"""
    p = CONSONANT_PARAMS.get(code)
    if not p:
        return np.zeros(1000)
    
    dur = p['dur']
    
    if p['voiced']:
        source = generate_glottal_pulse(p['f0'], dur)
    else:
        source = np.zeros(int(SAMPLE_RATE * dur / 1000))
    
    if p['noise'] > 0:
        noise = generate_noise(dur) * p['noise']
        source = source * (1 - p['noise']) + noise
    
    source = formant_filter(source, p['f1'], p['f2'], p['f3'])
    
    # 归一化
    mx = np.max(np.abs(source))
    if mx > 0:
        source = source / mx * AMPLITUDE
    
    return source


def synthesize_vowel(code, f0=120, duration_ms=200):
    """合成一个韵母（持续音）"""
    p = VOWEL_PARAMS.get(code)
    if not p:
        return np.zeros(1000)
    
    source = generate_glottal_pulse(f0, duration_ms)
    source = formant_filter(source, p['f1'], p['f2'], p['f3'])
    
    mx = np.max(np.abs(source))
    if mx > 0:
        source = source / mx * AMPLITUDE
    
    return source


def synthesize_syllable(consonant_code, vowel_code, tone_code=None):
    """合成一个完整音节 = 声母 + 韵母（+ 声调F0曲线）"""
    # 合成声母
    c_sig = synthesize_consonant(consonant_code)
    
    # 确定韵母的 F0
    f0 = 120
    if tone_code and tone_code in TONE_CURVES:
        curve = TONE_CURVES[tone_code]['curve']
        f0 = np.mean(curve[:2])  # 简化：取前两点均值
    
    v_sig = synthesize_vowel(vowel_code, f0=f0, duration_ms=250)
    
    # 过渡平滑
    transition = min(30, len(c_sig) // 3, len(v_sig) // 3)
    if transition > 0:
        fade_out = np.linspace(1, 0, transition)
        fade_in = np.linspace(0, 1, transition)
        c_sig[-transition:] *= fade_out
        v_sig[:transition] *= fade_in
    
    return np.concatenate([c_sig, v_sig])


def save_wav(signal, filename, sr=SAMPLE_RATE):
    """保存为 WAV 文件"""
    output_dir = os.path.dirname(os.path.abspath(__file__))
    filepath = os.path.join(output_dir, filename)
    
    # 16-bit PCM
    sig = np.clip(signal * 32767, -32767, 32767).astype(np.int16)
    
    with wave.open(filepath, 'w') as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(sr)
        wf.writeframes(sig.tobytes())
    
    print(f"  ✓ {filepath}")


def main():
    print("零维码发音合成器")
    print("=" * 50)
    
    # 创建输出目录
    output_dir = os.path.dirname(os.path.abspath(__file__))
    
    # 1. 声母发音
    print("\n合成声母...")
    for code in sorted(CONSONANT_PARAMS.keys())[:10]:  # 前10个
        sig = synthesize_consonant(code)
        save_wav(sig, f"consonant_{code}.wav")
    
    # 2. 韵母发音
    print("\n合成韵母...")
    for code in sorted(VOWEL_PARAMS.keys())[:10]:  # 前10个
        sig = synthesize_vowel(code)
        save_wav(sig, f"vowel_{code}.wav")
    
    # 3. 完整音节示例
    print("\n合成音节...")
    syllables = [
        ('I24', 'F08', 'T01'),  # ʃ+ə = "丝"音色
        ('I18', 'F04', 'T01'),  # ʔ+ɔ = "哦"音色
        ('I08', 'F05', 'T02'),  # ʈ+ɛ
        ('I30', 'F14', 'T04'),  # ŋ+ɯ
        ('I17', 'F08', 'T03'),  # ɦ+ə
    ]
    
    for c, v, t in syllables:
        sig = synthesize_syllable(c, v, t)
        save_wav(sig, f"syllable_{c}_{v}_{t}.wav")
    
    print("\n完成！")
    print(f"WAV 文件已保存至: {output_dir}")


if __name__ == '__main__':
    main()
