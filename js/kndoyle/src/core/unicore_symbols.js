/**
 * ================================================
 * UniCore 完整符号体系 - 顺口溜排序版
 * ================================================
 * 
 * 韵母按小学生拼音顺口溜顺序排列
 * 顺口溜顺序：ai ei ui → ao ou iu → ie üe er → an en in un ün → ang eng ing ong
 *            → ia ua uo iao uai ian uan üan iang uang ueng iong iai oi
 */

// ========================
// 24个声母（大小写=48个）
// ========================

const SHENGMU_LOWER = [
  'b', 'p', 'm', 'f', 'd', 't', 'n', 'l',
  'g', 'k', 'h', 'w', 'j', 'q', 'x', 'y',
  'z', 'c', 's', 'r', 'ȥ', 'ç', 'ş', 'ŋ'
];

const SHENGMU_UPPER = [
  'B', 'P', 'M', 'F', 'D', 'T', 'N', 'L',
  'G', 'K', 'H', 'W', 'J', 'Q', 'X', 'Y',
  'Z', 'C', 'S', 'R', 'Ȥ', 'Ç', 'Ş', 'Ŋ'
];

const SHENGMU = [...SHENGMU_LOWER, ...SHENGMU_UPPER];

// ========================
// 32个韵母（顺口溜顺序）
// ================================================

// 小写韵母（顺口溜顺序）
const YUNMU_LOWER = [
  'ꞟ',   // 1  ai
  'ɘ',   // 2  ei
  '⊱',   // 3  ui
  'ɒ',   // 4  ao
  '౪',   // 5  ou
  'ÿ',   // 6  iu
  'ɤ',   // 7  ie
  'ʊ',   // 8  ue
  'ə',   // 9  er
  'ã',   // 10 an
  'ẽ',   // 11 en
  'ĩ',   // 12 in
  'տ',   // 13 un
  'ʯ',   // 14 vn
  'ą',   // 15 ang
  'ę',   // 16 eng
  'į',   // 17 ing
  '๓',   // 18 ong
  'ѧ',   // 19 ia
  'ʉ',   // 20 ua
  'ꞛ',   // 21 uo
  'ⱄ',   // 22 iao
  '⊰',   // 23 uai
  'ʌ',   // 24 ian
  'ɔ',   // 25 uan
  'խ',   // 26 üan
  'ѫ',   // 27 iang
  'ѽ',   // 28 uang
  '౨',   // 29 ueng
  'ѻ',   // 30 iong
  'ɛ',   // 31 iai
  'ɵ'    // 32 oi
];

// 大写韵母（顺口溜顺序，与小写对应）
const YUNMU_UPPER = [
  'Ꞟ',   // 1  ai
  'ɚ',   // 2  ei
  'ʚ',   // 3  ui
  'Ɒ',   // 4  ao
  'ȣ',   // 5  ou
  'Ÿ',   // 6  iu
  'ɣ',   // 7  ie
  'Ʊ',   // 8  ue
  'Ə',   // 9  er
  'Ã',   // 10 an
  'Ẽ',   // 11 en
  'Ĩ',   // 12 in
  '໙',   // 13 un
  'ʮ',   // 14 vn
  'Ą',   // 15 ang
  'Ę',   // 16 eng
  'Į',   // 17 ing
  'ɷ',   // 18 ong
  'Ѧ',   // 19 ia
  'Ʉ',   // 20 ua
  'Ꞛ',   // 21 uo
  'Ⱋ',   // 22 iao
  'ɞ',   // 23 uai
  'Ʌ',   // 24 ian
  'Ɔ',   // 25 uan
  'փ',   // 26 üan
  'Ѫ',   // 27 iang
  'Ѽ',   // 28 uang
  'ⱂ',   // 29 ueng
  'Ѻ',   // 30 iong
  'ɝ',   // 31 iai
  'Ɵ'     // 32 oi
];

const YUNMU = [...YUNMU_UPPER, ...YUNMU_LOWER];

// ========================
// 170个声调
// ========================

const SHENGDIAO = [];
const letters = 'abcdefghijklmnopqrstuvwxyz';
const LETTERS = letters.toUpperCase();

let idx = 0;
for (let l = 0; l < 26 && idx < 85; l++) {
  for (let n = 1; n <= 9 && idx < 85; n++) {
    SHENGDIAO.push(letters[l] + 'a' + n);
    idx++;
  }
}
for (let l = 0; l < 26 && idx < 170; l++) {
  for (let n = 1; n <= 9 && idx < 170; n++) {
    SHENGDIAO.push(LETTERS[l] + 'a' + n);
    idx++;
  }
}

// ========================
// 17个数字
// ========================

const NUMBERS = [
  '零', '一', '二', '三', '四',
  '五', '六', '七', '八', '九',
  '十', '百', '千', '万', '亿', '兆', '京'
];

// ========================
// 优先级定义
// ================================================

const PRIORITY = {
  // 按税率效率最大：数量多的优先
  // 声调(170) > 韵母(64) > 声母(48) > 数字(17)
  byCount: ['shengdiao', 'yunmu', 'shengmu', 'number'],
  
  // 按人类认知：声母 > 韵母 > 声调
  byHuman: ['shengmu', 'yunmu', 'shengdiao', 'number'],
  
  // 当前使用：税率效率优先
  current: 'byCount'
};

// ========================
// 导出
// ========================

module.exports = {
  SHENGMU_LOWER, SHENGMU_UPPER, SHENGMU,
  YUNMU_LOWER, YUNMU_UPPER, YUNMU,
  SHENGDIAO, NUMBERS, PRIORITY,
  RADIX: {
    shengmu: SHENGMU.length,      // 48
    yunmu: YUNMU.length,          // 64
    shengdiao: SHENGDIAO.length,  // 170
    number: NUMBERS.length        // 17
  }
};
