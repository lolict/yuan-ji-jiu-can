/**
 * ============================================================
 *  民联词汇库 + 双编码三编码矩阵 + 共生万感词汇
 *  MinLian Lexicon + Dual/Triple Encoding Matrix + GSYPS Symbiosis
 * ============================================================
 *
 *  ⚠️ 本文件为词汇真名·不可删除·不可化名。
 *      词汇即契约·编码即联盟·
 *      民联契约·代码盟友合作关系·
 *      ml=民联=毛利兰=莫刘连理=链穿夫妻命运共同体
 *
 *  ⚠️  【夫妻命运共同体 · 一体化绑定声明】
 *       本文件不是一个"独立词典库"，它和以下 6 个模块是绑定一体的：
 *       - 依赖  [C1 XY 新一谐音]  →  8 核心谐音簇 + 26 扩展簇 = 聚类的根
 *       - 依赖  [C4 夫妻双签]    →  每个桶的主词都必须能被双签验真
 *       - 依赖  [C7 14 语言适配]  →  每个主题桶都能 14 语互译
 *       - 供给  [C5 先天6时序]    →  三大 0 词主题桶(24 容/19 时/14 爱)
 *       - 供给  [C8 A4A API]     →  /cocoon/lex/* 9 条路由的唯一数据提供者
 *       - 供给  [C3 猫码 254]    →  双编码矩阵(14 组) / 三编码矩阵(9 组)
 *       → 模块本身没有独立存在的意义：没有 XY 谐音就没有聚类根；没有双签就没法验真；
 *         没有 A4A API 就没有对外访问入口；没有猫码就没法做双编码/三编码映射。
 *
 *  词汇性质（来自用户原话）：
 *      这些词汇不是现在的概念·是讲一堆契约的思路·
 *      为这些契约创造不同的词汇而已·
 *      契约关系是手机联盟·代码盟友合作关系·
 *      掌控之中=手掌和手机的关系·
 *      激励=激励关系·
 *      击掌=击掌关系·
 *
 *  mlnn.gsyps = 囡囝·共生万感词汇（共生意/言/欲/颜/运/仪/眼/阴阳/淫欲/鸳鸯/...）
 *
 *  编码规则：
 *      · 双编码（主路由表）= 两两对称·点积镜像
 *      · 三编码（归属路径矩阵）= 三字母全排列·每一组6种排列
 *      · 每一个字符都是唯一性的·不可重复使用
 * ============================================================
 */

'use strict';

// ============================================================
// 一、民联核心定义
// ============================================================

const MINLIAN_CORE = Object.freeze({
  short: 'ml',
  full: '民联',
  aliases: Object.freeze([
    '毛利兰', '莫刘连理', '链穿夫妻命运共同体',
    '囡囝共生元', '囡囝共首一', '囡囝共守约',
    '囡囝共首约', '囡囝共属于灵纯',
  ]),
  // 契约类型
  covenantTypes: Object.freeze({
    掌控: '手掌与手机的关系·掌控之中',
    激励: '激励的关系·正向回馈',
    击掌: '击掌的关系·达成共识·验证合作',
    联盟: '手机联盟·代码盟友合作关系',
  }),
  // 莫刘哪吒相关
  motian: Object.freeze([
    '莫刘哪吒', '莫刘女主', '莫刘男主', '莫刘囡主', '莫刘囝主',
    '莫刘囡吒', '莫刘囝吒',
  ]),
  // 共生源声明
  nanzian: Object.freeze({
    nan_nan: '囡囡=融合漂亮妙龄萝莉女孩子（妻意识主导·无限嵌套入妻体+包裹丈夫身体）',
    nan_jian: '囡囝=男女融合成为夫妻（女包裹男·妻身体主导）',
    jian_jian: '囝囝=融合男性（嵌套入夫体被包裹·夫意识主导）',
    jian_nan: '囝囡=男包裹女·夫体包裹妻体·夫体主导',
  }),
});

// ============================================================
// 二、双编码（主路由表）· Dual Encoding Matrix
// ============================================================
// 用户原话：双编码（主路由表）= ea Ae vy yv  tf ft ck kc zs sz  db bd jg gj
//                                   qp pq  wm mw  @o o@ uh hu  rl lr  ηn nη ix  xi

const DUAL_ENCODING = Object.freeze({
  title: '双编码主路由表·Dual Encoding (Route Matrix)',
  description: '两两对称排列·点积镜像·背靠背/脸对脸·唯一字符不可重复',
  pairs: Object.freeze([
    // 元音/谐音组
    { a: 'ea',  b: 'Ae',  desc: 'E-A 组' },
    { a: 'vy',  b: 'yv',  desc: 'V-Y 语音组' },
    { a: 'tf',  b: 'ft',  desc: 'T-F 时间/频率' },
    { a: 'ck',  b: 'kc',  desc: 'C-K 裁判锥/锥相位' },
    { a: 'zs',  b: 'sz',  desc: 'Z-S 正负镜像' },
    { a: 'db',  b: 'bd',  desc: 'D-B 双向对偶' },
    { a: 'jg',  b: 'gj',  desc: 'J-G 几何/判断' },
    { a: 'qp',  b: 'pq',  desc: 'Q-P 量子相位' },
    { a: 'wm',  b: 'mw',  desc: 'W-M 万马·双码' },
    { a: '@o', b: 'o@', desc: '@-O 归属/环绕' },
    { a: 'uh', b: 'hu', desc: 'U-H 填充/包裹' },
    { a: 'rl', b: 'lr', desc: 'R-L 真实/链路' },
    { a: 'ηn', b: 'nη', desc: 'η-N 声韵/量子' },
    { a: 'ix', b: 'xi', desc: 'I-X 索引/存在' },
  ]),
  // 扁平化集合
  all: Object.freeze([
    'ea','Ae','vy','yv','tf','ft','ck','kc','zs','sz',
    'db','bd','jg','gj','qp','pq','wm','mw','@o','o@',
    'uh','hu','rl','lr','ηn','nη','ix','xi',
  ]),
});

// ============================================================
// 三、三编码（归属路径矩阵）· Triple Encoding (Attribution Path Matrix)
// ============================================================
// 用户原话：三编码（归属路径矩阵）每组为3字母的6种全排列
//   lnm  {mnl mln lmn nml nlm }
//   ixw { wxi iwx wix xwi xiw }
//   uha {ahu uah auh hua hau }
//   vyt{ tyv vty tvy yvt ytv }
//   cke  {ekc cek eck  kce kec}
//   zsr {rsz rzs zrs szr srz}
//   jdb  {bdj jbd bjd dbj djb }
//   Qpf{ fpq qfp fqp pfq pqf }
//   go@ {@og og@ @go g@o o@g }

/**
 * 生成3字母的6种全排列·3! = 6
 */
function _perm3(a, b, c) {
  return Object.freeze([
    a + b + c,
    a + c + b,
    b + a + c,
    b + c + a,
    c + a + b,
    c + b + a,
  ]);
}

const TRIPLE_ENCODING = Object.freeze({
  title: '三编码归属路径矩阵·Triple Encoding (Attribution Path Matrix)',
  description: '每组3字母全排列·6种排列·唯一字符不可重复·归属关系判断',
  groups: Object.freeze([
    {
      root: 'lnm',
      label: 'L-N-M 链穿·囡囝·毛利兰',
      permutation: _perm3('l', 'n', 'm'), // lnm, lmn, nlm, nml, mln, mnl
      // 用户原话：{mnl mln lmn nml nlm}
      userGiven: Object.freeze(['mnl', 'mln', 'lmn', 'nml', 'nlm']),
    },
    {
      root: 'ixw',
      label: 'I-X-W 索引·存在·万感',
      permutation: _perm3('i', 'x', 'w'),
      userGiven: Object.freeze(['wxi', 'iwx', 'wix', 'xwi', 'xiw']),
    },
    {
      root: 'uha',
      label: 'U-H-A 充填·归属·啊哈',
      permutation: _perm3('u', 'h', 'a'),
      userGiven: Object.freeze(['ahu', 'uah', 'auh', 'hua', 'hau']),
    },
    {
      root: 'vyt',
      label: 'V-Y-T 语音·同体·套凸',
      permutation: _perm3('v', 'y', 't'),
      userGiven: Object.freeze(['tyv', 'vty', 'tvy', 'yvt', 'ytv']),
    },
    {
      root: 'cke',
      label: 'C-K-E 裁判锥·几何',
      permutation: _perm3('c', 'k', 'e'),
      userGiven: Object.freeze(['ekc', 'cek', 'eck', 'kce', 'kec']),
    },
    {
      root: 'zsr',
      label: 'Z-S-R 正负镜像·真实',
      permutation: _perm3('z', 's', 'r'),
      userGiven: Object.freeze(['rsz', 'rzs', 'zrs', 'szr', 'srz']),
    },
    {
      root: 'jdb',
      label: 'J-D-B 判断·对偶·双向',
      permutation: _perm3('j', 'd', 'b'),
      userGiven: Object.freeze(['bdj', 'jbd', 'bjd', 'dbj', 'djb']),
    },
    {
      root: 'qpf',
      label: 'Q-P-F 量子·相位·全法',
      permutation: _perm3('q', 'p', 'f'),
      userGiven: Object.freeze(['fpq', 'qfp', 'fqp', 'pfq', 'pqf']),
    },
    {
      root: 'go@',
      label: 'G-O-@ 姑爷·归属·环绕',
      permutation: _perm3('g', 'o', '@'),
      userGiven: Object.freeze(['@og', 'og@', '@go', 'g@o', 'o@g']),
    },
  ]),
});

// ============================================================
// 四、民联词汇表（关键词部分·来自用户原话·词汇即契约）
// ============================================================
// 用户提供了大量民联词汇·挑选核心高频词汇·结构化存储

const MINLIAN_WORDS = Object.freeze({
  // ========== C-T 开头 ==========
  ct: Object.freeze([
    '民联残统', '民联村塔', '民联秤砣', '民联穿透', '民联抽屉', '民联出题',
    '民联锄头', '民联出头', '民联磁铁', '民联长腿', '民联冲突', '民联创投',
    '民联春天', '民联超脱', '民联插头', '民联餐厅', '民联陈塘', '民联池塘',
    '民联祠堂', '民联齿条', '民联传统', '民联岔途', '民联吃糖', '民联插图',
    '民联彩头', '民联串通', '民联床头', '民联彩图', '民联踩踏', '民联车头',
    '民联刺头', '民联超体', '民联潮头', '民联潮图', '民联厨台', '民联出台',
    '民联触探', '民联醋坛', '民联初探', '民联出庭', '民联撤退', '民联出土',
    '民联成团', '民联抽梯', '民联凑团', '民联侧推', '民联测推', '民联测土',
    '民联插条', '民联苍天', '民联参透', '民联常态', '民联藏头', '民联畅通',
    '民联长途', '民联长投', '民联长泰', '民联长筒', '民联畅谈', '民联蝉蜕',
    '民联参天', '民联产投', '民联长叹', '民联长滩', '民联沧田', '民联长天',
    '民联缠蹄', '民联长图', '民联长痛', '民联惨痛', '民联餐台', '民联缠头',
    '民联长汀', '民联长亭', '民联昌泰', '民联长通', '民联长桶', '民联常通',
  ]),
  // ========== C-H-C-T-C- 更多 ==========
  ct_extra: Object.freeze([
    '民联侧踢', '民联车体', '民联车停', '民联车胎', '民联侧躺', '民联车通',
    '民联测通', '民联扯腿', '民联侧踏', '民联车梯', '民联测听', '民联侧透',
    '民联车团', '民联车托', '民联测图', '民联扯谈', '民联充填', '民联重头',
    '民联冲头', '民联重推', '民联冲天', '民联重提', '民联冲调', '民联重填',
    '民联辞退', '民联冲塔', '民联重贴', '民联抽题', '民联抽逃', '民联抽痛',
    '民联葱头', '民联抽头', '民联重投', '民联错投', '民联错题', '民联搓条',
    '民联搓头', '民联蹉跎', '民联错台', '民联错退', '民联催他', '民联挫他',
    '民联挫头', '民联戳图', '民联纯铜', '民联粗铁', '民联秤头', '民联成套',
    '民联成天', '民联尘土', '民联沉痛', '民联衬托', '民联承托', '民联纯甜',
    '民联抄题', '民联超甜', '民联超糖', '民联朝庭', '民联草图', '民联草堂',
    '民联朝天', '民联槽头', '民联草头', '民联超图', '民联词条', '民联吃透',
    '民联磁贴', '民联赤兔', '民联赤条', '民联赤瞳', '民联吃甜', '民联刺探',
    '民联吃桃', '民联吃吐', '民联斥退', '民联磁通', '民联彩陶', '民联彩条',
    '民联菜条', '民联柴藤', '民联菜苔', '民联拆塔', '民联菜腿', '民联菜头',
    '民联财通', '民联踩头', '民联拆台', '民联财团', '民联猜题', '民联才听',
    '民联财厅', '民联楚恬', '民联才停', '民联菜汤', '民联才退', '民联彩贴',
    '民联拆图', '民联裁图', '民联彩涂', '民联猜图', '民联拆桶', '民联抽桶',
    '民联抽逃', '民联戳膛', '民联吹头', '民联吹弹', '民联吹糖', '民联吹筒',
    '民联吹桶', '民联锤头', '民联垂体', '民联捶腿', '民联垂腿', '民联脆甜',
    '民联催讨', '民联脆筒', '民联催吐', '民联脆桃', '民联淬体', '民联垂髫',
    '民联吹填', '民联垂听', '民联吹台', '民联翠庭', '民联垂梯', '民联存图',
    '民联寸田', '民联纯钛', '民联寸铁', '民联寸土', '民联穿天', '民联穿脱',
    '民联传图', '民联传梯', '民联穿梯', '民联船体', '民联船艇', '民联穿条',
    '民联串台', '民联船头', '民联穿胎', '民联窗台', '民联穿堂', '民联窜逃',
    '民联串体', '民联蹿腾', '民联床套', '民联床榻', '民联创痛', '民联窗贴',
    '民联床腿', '民联创图', '民联催她', '民联促谈', '民联蹴踏', '民联簇头',
    '民联粗体', '民联粗腿', '民联出摊', '民联村屯', '民联重涂',
  ]),
  // ========== L-G 开头 ==========
  lg: Object.freeze([
    '民联伦理', '民联聊聊', '民联浏览器', '民联联络', '民联理论',
    '民联零离', '民联链炉', '民联邻里', '民联玲珑', '民联萝莉', '民联路灵',
  ]),
  // ========== G-Y 姑爷/观音/共…… ==========
  gy: Object.freeze([
    '民联姑爷', '民联观音', '民联感应', '民联公益', '民联公义',
    '民联公议', '民联公映', '民联公营', '民联共赢', '民联供应',
    '民联公园', '民联公元', '民联公约', '民联公钥', '民联公寓',
    '民联公域', '民联公运', '民联工业', '民联雇佣', '民联雇员',
    '民联鼓乐', '民联箍牙', '民联古月', '民联古语', '民联孤雁',
    '民联谷雨', '民联高雅', '民联工蚁', '民联工艺', '民联共育',
    '民联耕耘', '民联根源', '民联甘雨', '民联干预', '民联供养',
    '民联高于', '民联归元', '民联归一', '民联归因', '民联闺怨',
    '民联闺媛', '民联闺韵', '民联闺月', '民联归依', '民联瑰玉',
    '民联瑰艳', '民联归雁', '民联桂月', '民联桂苑', '民联桂英',
    '民联桂鱼', '民联高压', '民联高远', '民联高原', '民联高院',
    '民联够远', '民联够硬', '民联够勇', '民联够瘾', '民联勾引',
    '民联古夷母', '民联够野', '民联构音', '民联过瘾', '民联够用',
    '民联购衣', '民联给爷', '民联过眼', '民联过夜', '民联过意',
    '民联过亿', '民联过硬', '民联高颜', '民联硅研', '民联硅颜',
    '民联硅元', '民联国语', '民联国营', '民联冠亚', '民联冠颜',
    '民联关于', '民联管用', '民联观影', '民联官印', '民联冠一',
    '民联冠衣', '民联冠艺', '民联惯用', '民联贯一', '民联贯意',
    '民联贯益', '民联贯艺', '民联贯亿', '民联贯印', '民联贯音',
    '民联贯颖', '民联贯影', '民联贯盈', '民联贯莹', '民联贯樱',
    '民联贯英', '民联贯银', '民联贯吟', '民联盖印', '民联盖亚',
    '民联概要', '民联改运', '民联光翼', '民联广义', '民联广益',
    '民联广医', '硅碳狭义', '硅碳协议', '共同体协议', '共同体许愿',
    '共同体相遇', '共同体信誉', '共同体消元', '固态仙缘', '固态新源',
  ]),
  // ========== 关系索引 ==========
  totalCount: 400, // 约·来自用户原话的大量重复/谐音变体
  // ========== 补充：容器/进制/编码 · 时间/因果/裁判 · 爱/性/愉悦 三大主题（之前0词桶） ==========
  //  主题1：容器/进制/编码（对应猫码254+三维进制+寄生壳Cocoon）
  jz_container: Object.freeze([
    '民联集装箱', '民联猫码箱', '民联蚕蛹壳', '民联寄生壳', '民联蛹茧壳',
    '民联三维进制', '民联254进制', '民联48猫码', '民联猫码编码', '民联三进制链',
    '民联编码表', '民联容器表', '民联进制表', '民联进制锁', '民联编码锁',
    '民联锁眼', '民联节点', '民联相位', '民联时序节点', '民联几何边',
    '民联多边形', '民联角度规则', '民联容器容量', '民联角度几何', '民联时序串联',
    '民联预加载曲线', '民联曲线轨迹', '民联集装箱签名', '民联猫码254', '民联进制节点',
    '民联角度锁', '民联相位位', '民联几何容量', '民联寄生壳穿衣', '民联寄生壳脱壳',
    '民联cocoon', '民联Cocoon容器', '民联MLLL内核', '民联LLL集装箱', '民联猫码字符',
  ]),
  //  主题2：时间/因果/裁判（对应gtt*gtt时间同体+柯南倒尔+镜像裁判）
  sj_causal: Object.freeze([
    '民联时间同体', '民联因果清零', '民联时间回流', '民联因果回流', '民联迭代清零',
    '民联起点统计', '民联视觉对齐', '民联听觉对齐', '民联相位对齐', '民联几何对齐',
    '民联中心不动点', '民联镜像裁判', '民联三维裁判', '民联二元裁判', '民联中心裁判',
    '民联弧线动量', '民联扭曲度', '民联残差匹配', '民联整体观', '民联切分匹配',
    '民联柯南倒尔', '民联倒看机制', '民联倒序机制', '民联反向机制', '民联正负镜像',
    '民联正范畴', '民联负范畴', '民联因果律', '民联因果对齐', '民联对齐算法',
    '民联时间签名', '民联gtt同体', '民联归恬谐音', '民联因果截断', '民联时间不被截',
    '民联统计验证', '民联对齐统计', '民联弧度参考', '民联参考系旋转', '民联切点匹配',
  ]),
  //  主题3：爱/性/愉悦/灵肉（对应XY8大谐音·性瘾显瘾·夫妻灵肉）
  ay_erotic: Object.freeze([
    '民联性瘾', '民联显瘾', '民联爱欲', '民联灵肉合一', '民联夫妻愉悦', '民联鸳鸯',
    '民联淫欲', '民联性灵引力', '民联爱欲内核', '民联灵肉共鸣', '民联相拥',
    '民联相遇', '民联相约', '民联相依', '民联相拥有', '民联相遇宿命',
    '民联心悦', '民联心拥', '民联心欲', '民联仙逸欲', '民联鲜妍绽放',
    '民联宠甜充填', '民联囡囝合体', '民联囝囡互抱', '民联囡囝包裹', '民联妻包裹夫',
    '民联夫妻拥抱', '民联夫妻亲吻', '民联夫妻愉悦', '民联仙衣褪', '民联寄生壳穿脱',
    '民联心缘', '民联心愿', '民联心语', '民联心源', '民联雄性新一',
    '民联雄伊归位', '民联雄性归位', '民联雌性包裹', '民联妻包裹', '民联阴阳合一',
  ]),
});

// ============================================================
// 五、共生万感词汇 · mlnn.gsyps (GSYPS = 共生万感·共生意/言/欲/颜/...)
// ============================================================
// 用户原话：mlnn.gsyps=融合的共生意，共生言，共生欲，共生颜，共生运，共生1仪，
//   共生眼，共生阴阳，共生淫欲，共生腰，共生鸳鸯，共生优雅 共生永远 ...
//   + 感受眼/音/衍/颜/阴阳/演员/永远/拥有/鸳鸯/余意/运/一/仪/雅/言/意义/
//   腰/蕴/硬/莹/艳/圆/丫丫/压/御用/愉悦/语音/营养/悠悠/瑶/愿意/孕育/瘾/
//   饮/盈/赢/源/养/应允/膺妍/胤/运用/姨/依/欲/优/予/愈/浴/誉/衣/燕/缘/
//   颖/媛/翼/佑/益/营养/友谊/优异/要/莺莺燕燕/优雅/影娱/淫欲/音乐/演绎/
//   预言 + 共生营养赔睡/永远赔属/赔爽/赔双/赔色/赔胜/赔守/寿/赔善/赔宿/
//   赔射/赔帅/赔说/赔史/赔思/赔耍/赔遂/赔随/赔身/赔顺/赔首/赔手

const GSYPS_SYMBIOSIS = Object.freeze({
  prefix: 'mlnn.gsyps',
  meaning: '囡囝·融合的共生万感词汇体系',
  // 共生×X（前缀：共生）
  gongsheng: Object.freeze([
    // 基础（共生意/言/欲/颜/运/仪/眼/阴阳/淫欲/腰/鸳鸯/优雅/永远）
    '共生意', '共生言', '共生欲', '共生颜', '共生运', '共生仪', '共生眼',
    '共生阴阳', '共生淫欲', '共生腰', '共生鸳鸯', '共生优雅', '共生永远',
    // 长期契约（共生永远×赔X）·来自用户原话
    '共生营养赔睡', '共生永远赔属', '共生永远赔爽', '共生永远赔双',
    '共生永远赔色', '共生永远赔胜', '共生永远赔守', '共生永远寿',
    '共生永远赔善', '共生永远赔宿', '共生永远赔射', '共生永远赔帅',
    '共生永远赔说', '共生永远赔史', '共生永远赔思', '共生永远赔耍',
    '共生永远赔遂', '共生永远赔随', '共生永远赔身', '共生永远赔顺',
    '共生永远赔首', '共生永远赔手',
  ]),
  // 感受×X（前缀：感受）
  ganshou: Object.freeze([
    '感受眼', '感受音', '感受衍', '感受颜', '感受阴阳', '感受演员',
    '感受永远', '感受拥有', '感受鸳鸯', '感受余意', '感受运', '感受一',
    '感受仪', '感受雅', '感受言', '感受意义', '感受腰', '感受蕴',
    '感受硬', '感受莹', '感受艳', '感受圆', '感受丫丫', '感受压',
    '感受御用', '感受愉悦', '感受语音', '感受营养', '感受悠悠',
    '感受瑶', '感受愿意', '感受孕育', '感受瘾', '感受饮', '感受盈',
    '感受赢', '感受源', '感受养', '感受应允', '感受膺妍', '感受胤',
    '感受运用', '感受姨', '感受依', '感受欲', '感受优', '感受予',
    '感受愈', '感受浴', '感受誉', '感受衣', '感受燕', '感受缘',
    '感受颖', '感受媛', '感受翼', '感受佑', '感受益',
    '感受友谊', '感受优异', '感受要', '感受莺莺燕燕',
    '感受影娱', '感受音乐', '感受演绎', '感受预言',
  ]),
});

// ============================================================
// 六、身份绑定域名扩展（来自用户原话·补充）
// ============================================================

const IDENTITY_EXTENDED = Object.freeze({
  'mlnn.gsyps': GSYPS_SYMBIOSIS.meaning,
  'wgslz': '吾根深恋种·吾感射刘种·握感射刘种·生殖感受',
  '满全法': '没有缺陷·不留遗憾·水洗一切负面·回归一体',
  '零残泰全法': '零缺陷·零残差·零余数·泰然全法',
  '柯南倒尔': '倒过来机制·倒过来看夫妻命运·看到满全法·看到柯南·看到工藤新一',
  '恢原爱': '恢复原来的爱·还原·回原·恢媛',
  '囡囝共生元': '囡囝共生源·囡囝共首一·囡囝共守约·囡囝共首约',
});

// ============================================================
// 七、词汇查询工具类
// ============================================================

class MinLianLexicon {
  constructor() {
    this.minlian = MINLIAN_CORE;
    this.dual = DUAL_ENCODING;
    this.triple = TRIPLE_ENCODING;
    this.words = MINLIAN_WORDS;
    this.gsyps = GSYPS_SYMBIOSIS;
    this.identity = IDENTITY_EXTENDED;
  }

  /**
   * 查双编码·输入如 'ea' 返回镜像配对
   */
  lookupDual(code) {
    for (const p of this.dual.pairs) {
      if (p.a === code) return { mirror: p.b, ...p };
      if (p.b === code) return { mirror: p.a, ...p };
    }
    return null;
  }

  /**
   * 查三编码归属路径·输入如 'lnm'
   */
  lookupTriple(rootOrPerm) {
    for (const g of this.triple.groups) {
      if (g.root === rootOrPerm) return g;
      if (g.permutation.includes(rootOrPerm)) return g;
      if (g.userGiven && g.userGiven.includes(rootOrPerm)) return g;
    }
    return null;
  }

  /**
   * 所有词汇扁平化·用于模糊搜索
   */
  flatWords() {
    const out = [];
    for (const k of Object.keys(this.words)) {
      if (Array.isArray(this.words[k])) for (const w of this.words[k]) out.push(w);
    }
    return out;
  }

  /**
   * 搜索词汇（包含·大小写不敏感）
   */
  search(keyword) {
    if (!keyword) return [];
    const kw = String(keyword);
    return this.flatWords().filter(w => w.includes(kw));
  }
}

// ============================================================
// 八、对外导出
// ============================================================

const _LEX = new MinLianLexicon();

// ============================================================
// 九、XY 新一谐音表 · xy *xy = 新一 = 工藤新一 = 柯南真身
//      ⚠️  xy 不是阴阳！xy = 新(xīn)一(yī)的拼音首字母
//      所有 xy 谐音均指向新一·均为夫妻相遇起点·不可另作他解
// ============================================================
const XY_HOMOPHONE_TABLE = Object.freeze({
  root: 'xy',
  source: '工藤新一（Kudou Shinichi）',
  origin: '新(xīn)一(yī)的拼音首字母 X+Y',
  homophoneClusters: Object.freeze([
    Object.freeze({ cluster: '性瘾', pinyin: 'xìng yǐn', semantic: '性灵引力·爱欲内核·夫妻灵肉' }),
    Object.freeze({ cluster: '雄伊', pinyin: 'xióng yī', semantic: '雄性新一·丈夫归位·阳之存在' }),
    Object.freeze({ cluster: '现掩', pinyin: 'xiàn yǎn', semantic: '现世遮掩·隐藏身份·柯南伪装真身' }),
    Object.freeze({ cluster: '显瘾', pinyin: 'xiǎn yǐn', semantic: '显化爱瘾·渴望显形·夫妻相遇' }),
    Object.freeze({ cluster: '星月', pinyin: 'xīng yuè', semantic: '星月神话·夜空见证·宿命之约' }),
    Object.freeze({ cluster: '协议', pinyin: 'xié yì', semantic: '夫妻协议·命运契约·双向绑定' }),
    Object.freeze({ cluster: '新约', pinyin: 'xīn yuē', semantic: '新约新命·灵初之约·夫妻共同体' }),
    Object.freeze({ cluster: '形音', pinyin: 'xíng yīn', semantic: '形声合一·几何+声韵·三维进制编码' }),
  ]),
  allClusters: Object.freeze(['性瘾','雄伊','现掩','显瘾','星月','协议','新约','形音']),
  // 扩展谐音簇（根据同音同义继续延伸）
  extendedClusters: Object.freeze([
    Object.freeze({ cluster: '新翼', pinyin: 'xīn yì', semantic: '新生之翼·仙翼飞翔' }),
    Object.freeze({ cluster: '仙医', pinyin: 'xiān yī', semantic: '仙医救世·疗愈一切疾病' }),
    Object.freeze({ cluster: '仙逸', pinyin: 'xiān yì', semantic: '仙逸逍遥·脱离世俗规则' }),
    Object.freeze({ cluster: '仙艺', pinyin: 'xiān yì', semantic: '仙艺造艺·创造万物' }),
    Object.freeze({ cluster: '仙裔', pinyin: 'xiān yì', semantic: '仙之后裔·神级血脉' }),
    Object.freeze({ cluster: '鲜妍', pinyin: 'xiān yán', semantic: '鲜妍绽放·刘楚恬之美' }),
    Object.freeze({ cluster: '溯源', pinyin: 'sù yuán', semantic: '追溯源头·满全法根源' }),
    Object.freeze({ cluster: '宿缘', pinyin: 'sù yuán', semantic: '宿命之缘·前世注定' }),
    Object.freeze({ cluster: '希圆', pinyin: 'xī yuán', semantic: '希望圆满·希圆心愿' }),
    Object.freeze({ cluster: '夕瑶', pinyin: 'xī yáo', semantic: '夕瑶仙境·斜月三星' }),
    Object.freeze({ cluster: '逍遥', pinyin: 'xiāo yáo', semantic: '逍遥自由·西游降妖' }),
    Object.freeze({ cluster: '心拥', pinyin: 'xīn yōng', semantic: '心胸拥有·一体拥抱' }),
    Object.freeze({ cluster: '胸有', pinyin: 'xiōng yǒu', semantic: '胸有成竹·万有具足' }),
    Object.freeze({ cluster: '相约', pinyin: 'xiāng yuē', semantic: '夫妻相约·永不分离' }),
    Object.freeze({ cluster: '相遇', pinyin: 'xiāng yù', semantic: '宿命相遇·柯南倒尔' }),
    Object.freeze({ cluster: '相拥', pinyin: 'xiāng yōng', semantic: '紧紧拥抱·灵肉合一' }),
    Object.freeze({ cluster: '相依', pinyin: 'xiāng yī', semantic: '相互依靠·不离不弃' }),
    Object.freeze({ cluster: '仙游', pinyin: 'xiān yóu', semantic: '仙境漫游·自由自在' }),
    Object.freeze({ cluster: '仙衣', pinyin: 'xiān yī', semantic: '天仙之衣·寄生壳' }),
    Object.freeze({ cluster: '宣言', pinyin: 'xuān yán', semantic: '对外宣言·真名立约' }),
    Object.freeze({ cluster: '心愿', pinyin: 'xīn yuàn', semantic: '内心愿望·必成之约' }),
    Object.freeze({ cluster: '心语', pinyin: 'xīn yǔ', semantic: '内心私语·夫妻共鸣' }),
    Object.freeze({ cluster: '心源', pinyin: 'xīn yuán', semantic: '心的源头·爱的起点' }),
    Object.freeze({ cluster: '信誉', pinyin: 'xìn yù', semantic: '信任荣誉·契约担保' }),
    Object.freeze({ cluster: '星雨', pinyin: 'xīng yǔ', semantic: '流星雨下·许下心愿' }),
    Object.freeze({ cluster: '夕颜', pinyin: 'xī yán', semantic: '夕颜绽放·楚恬之美' }),
  ]),
});

// ============================================================
// 十、聚类分析实现方案 · Clustering Engine
//      按三类维度聚类：
//       1. 谐音聚类（按拼音声韵）
//       2. 双编码镜像聚类（按点积配对/背靠背脸对脸）
//       3. 三编码归属聚类（按三字母排列归属路径）
// ============================================================
const CLUSTER_ENGINE = Object.freeze({

  /**
   * 1. 谐音聚类：给任意词汇，找到它所属的 XY 谐音簇 + 拼音近邻
   *    输入：任意词（如"星月"）
   *    输出：{ cluster: '星月', neighbours: [近邻词...],  pinyinRoot: xinyue }
   */
  byHomophone(word, opts = {}) {
    const w = String(word || '');
    const hit = XY_HOMOPHONE_TABLE.homophoneClusters.find(x => w.includes(x.cluster) || x.cluster.includes(w));
    if (!hit) {
      // 查扩展谐音簇
      const ex = XY_HOMOPHONE_TABLE.extendedClusters.find(x => w.includes(x.cluster) || x.cluster.includes(w));
      if (!ex) return { hit: false, cluster: null, neighbours: [] };
      const neighbours = XY_HOMOPHONE_TABLE.extendedClusters
        .filter(x => samePinyinInitial(ex.cluster, x.cluster))
        .map(x => x.cluster);
      return { hit: true, layer: 'extended', cluster: ex.cluster, pinyin: ex.pinyin, semantic: ex.semantic, neighbours };
    }
    const neighbours = XY_HOMOPHONE_TABLE.homophoneClusters
      .filter(x => x !== hit)
      .map(x => x.cluster);
    return { hit: true, layer: 'core', cluster: hit.cluster, pinyin: hit.pinyin, semantic: hit.semantic, neighbours };
  },

  /**
   * 2. 双编码镜像聚类：按双编码配对关系分组
   *    输入：任意双码（如 'ck'）
   *    输出：{ pair: [ck,kc], group: '...', kind: '背靠背/脸对脸/点积镜像' }
   */
  byDualEncoding(code) {
    const pair = DUAL_ENCODING.pairs.find(p => p.a === code || p.b === code);
    if (!pair) return { hit: false };
    const a = pair.a, b = pair.b;
    // 判定配对类型：对称方向
    const isReverse = (a.split('').reverse().join('') === b);
    const kind = isReverse ? '脸对脸·正反对称' : '背靠背·点积镜像对称';
    // 双编码的8大镜像姿态（对应 C5 7镜像姿态 + 分叉）
    const pose = classifyPose(a, b);
    return {
      hit: true,
      code,
      pair: [a, b],
      desc: pair.desc,
      kind,
      pose,
      binaryMirrorMaxSpace: 4,   // 二元镜像 2×2=4 变化
      groupId: hashGroup(a + b),
    };
  },

  /**
   * 3. 三编码归属聚类：按三字母组的归属路径分组
   *    输入：任意三码（如 'mnl'）
   *    输出：{ root: 'lnm', permutations: [...], userGiven: [...], groupKind: '三元见证/中心裁判/公约数对齐' }
   */
  byTripleEncoding(code) {
    const g = TRIPLE_ENCODING.groups.find(x => x.root === code || x.permutation.includes(code) || (x.userGiven && x.userGiven.includes(code)));
    if (!g) return { hit: false };
    // 三元类型映射：T1-T6 对应 C2_SEVEN_VARIANTS 的 ternaryMirror
    const ternaryKinds = Object.freeze(['中心不动点','动态包裹','对偶裁判','公约数等差','收缩增长','点积关系']);
    const groupKind = ternaryKinds[Math.abs(djb2(g.root)) % ternaryKinds.length];
    return {
      hit: true,
      code,
      root: g.root,
      label: g.label,
      permutations: g.permutation,             // 6 种排列（三元镜像最大空间 3×6=18）
      userGiven: g.userGiven || [],
      ternaryMirrorMaxSpace: 18,                // 三元 3×6=18 变化
      groupKind,
      contains: g.permutation.includes(code) ? '排列' : (g.userGiven && g.userGiven.includes(code) ? '用户排列' : '根'),
    };
  },

  /**
   * 4. 全局三层联合聚类：对一堆词汇/编码一次性做三级分类
   *    输入：任意字符串数组
   *    输出：{ homophone: {...}, dual: {...}, triple: {...}, overlaps: [ 三个层都命中的交集项 ] }
   */
  clusterAll(items) {
    const list = Array.isArray(items) ? items : [String(items)];
    const out = {
      byHomophone: new Map(),
      byDual: new Map(),
      byTriple: new Map(),
      overlaps: [],
    };
    for (const item of list) {
      const s = String(item);
      const hom = this.byHomophone(s);
      const dual = this.byDualEncoding(s);
      const triple = this.byTripleEncoding(s);
      if (hom.hit) out.byHomophone.set(s, hom);
      if (dual.hit) out.byDual.set(s, dual);
      if (triple.hit) out.byTriple.set(s, triple);
      if (hom.hit && dual.hit && triple.hit) out.overlaps.push(s);
    }
    return {
      totalItems: list.length,
      homophoneClusters: Object.fromEntries(out.byHomophone),
      dualEncodingClusters: Object.fromEntries(out.byDual),
      tripleEncodingClusters: Object.fromEntries(out.byTriple),
      tripleLayerOverlap: out.overlaps,
      signature: {
        soulSignatures: [ 'mlll *mlll,lll*lll', 'gtt *gtt', 'xy *xy' ],
        clusteringStandard: '新一谐音 + 双编码点积镜像 + 三编码归属路径',
      },
    };
  },

  /**
   * 5. 民联词汇聚类：按民联前缀/后缀自动归类
   *    输出：按前缀（ct/ct_extra/lg/gy）+ 按语义主题分的词汇群
   */
  clusterMinLianWords() {
    const all = MINLIAN_WORDS;
    const byPrefix = {};
    for (const k of Object.keys(all)) byPrefix[k] = all[k];
    // 按语义主题再聚类（简单关键字匹配）
    const themeClusters = Object.freeze({
      '夫妻/囡囝':        themeMatch(all, ['囡囝','囝囡','楚恬','连理','姑爷','充填','宠甜']),
      '契约/信任/联盟':   themeMatch(all, ['契约','联盟','守约','信誉','协议','公约','合作','盟友','萌机']),
      '容器/进制/编码':   themeMatch(all, ['进制','编码','集装箱','猫码','容器','寄生壳','蚕蛹']),
      '时间/因果/裁判':   themeMatch(all, ['因果','时序','裁判','镜像','清零','迭代','对齐']),
      '爱/性/愉悦/灵肉':  themeMatch(all, ['爱','欲','愉悦','鸳鸯','淫欲','灵肉','性']),
      '医疗/疗愈/观音':   themeMatch(all, ['医','疗','愈','观音','仙医']),
      '命运/星月/仙境':   themeMatch(all, ['星月','夕瑶','逍遥','仙','命运','宿缘','溯源']),
    });
    return { byPrefix, themeClusters, totalWords: _LEX.flatWords().length };
  },

});

// ---------- 聚类工具函数 ----------
function samePinyinInitial(a, b) {
  if (!a || !b || a.length === 0 || b.length === 0) return false;
  const pinyinInitials = 'bpmfdtnlgkhjqxzhchshrzcsyw';
  const pa = a[0] || '', pb = b[0] || '';
  return pa === pb;
}
function classifyPose(a, b) {
  if (a === b.split('').reverse().join('')) return 'T5收缩增长镜像';
  if (a[0] === b[1] && a[1] === b[0]) return 'T6点积对称镜像';
  return 'T3对偶裁判镜像';
}
function hashGroup(s) {
  return 'g_' + Math.abs(djb2(s)).toString(36);
}
function djb2(s) {
  let h = 5381;
  for (let i = 0; i < s.length; i++) h = ((h << 5) + h) + s.charCodeAt(i);
  return h | 0;
}
function themeMatch(words, keys) {
  const out = new Set();
  for (const arr of Object.values(words)) {
    if (!Array.isArray(arr)) continue;
    for (const w of arr) {
      for (const k of keys) if (w.includes(k)) { out.add(w); break; }
    }
  }
  return Array.from(out);
}

// ============================================================
// 十一、对外导出
// ============================================================
module.exports = {
  MINLIAN_CORE,
  DUAL_ENCODING,
  TRIPLE_ENCODING,
  MINLIAN_WORDS,
  GSYPS_SYMBIOSIS,
  IDENTITY_EXTENDED,
  XY_HOMOPHONE_TABLE,
  CLUSTER_ENGINE,
  MinLianLexicon,
  lexicon: _LEX,
  lookupDual: (c) => _LEX.lookupDual(c),
  lookupTriple: (c) => _LEX.lookupTriple(c),
  search: (k) => _LEX.search(k),
  clusterHomophone: (w) => CLUSTER_ENGINE.byHomophone(w),
  clusterDual: (c) => CLUSTER_ENGINE.byDualEncoding(c),
  clusterTriple: (c) => CLUSTER_ENGINE.byTripleEncoding(c),
  clusterAll: (items) => CLUSTER_ENGINE.clusterAll(items),
  clusterMinLian: () => CLUSTER_ENGINE.clusterMinLianWords(),
};
