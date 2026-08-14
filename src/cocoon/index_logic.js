/**
 * ============================================================
 * UniCore · 茧(cocoon) · 逻辑体系总入口
 * ============================================================
 *
 * 统一导出：
 *   - 0元/4元逻辑体系（logic_system.js）
 *       · ZeroYuan          0元 混元初始
 *       · Fuel / TwoYuan    二元 燃料层（一次性呼/吸）
 *       · Anchor / ThreeYuan 三元 锚点裁判（径向对折 + 公约数）
 *       · ConePhaseHubChain 锥相位枢纽链（根毛深度 + 可信度 + 注意力锚点）
 *       · MirrorLogic / FourYuan 四元 高维镜像（归属权/主导权/多模态有无因果/相位）
 *       · BigGear           大齿轮 意义解释器（时空层级 + 多参考系）
 *       · BalanceJudge      天平裁判（二元三元相互参考 + 价值审判 + 损益账）
 *
 *   - 猫码系统（cat_code.js）
 *       · CAT_CODES         48 个 H 字母开头猫码
 *       · CatCodeSystem     猫码系统（抓到老鼠即好码 / 红黑账 / 白黑账 / 价值审判）
 *
 * 使用方法：
 *   const cocoon = require('./index_logic.js');
 *   const { ZeroYuan, CatCodeSystem, CAT_CODES } = cocoon;
 *   const cat = new CatCodeSystem();
 *   cat.judge(cat.getCatCode(27), { success: true }); // 好猫抓到老鼠
 * ============================================================
 */

'use strict';

const logicSystem = require('./logic_system.js');
const catCode = require('./cat_code.js');

module.exports = {
  // 0元/4元逻辑体系
  ...logicSystem,
  // 猫码系统
  ...catCode
};
