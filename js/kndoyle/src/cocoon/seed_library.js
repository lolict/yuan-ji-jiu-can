/**
 * ============================================================
 * 蛹茧系统 · 高未来科技种子库 (TECH_SEEDS)
 * ============================================================
 *
 * 把用户预测的"高未来科技 + 数学概念 + 认知逻辑 + 人类社会常识"交叉组合，
 * 按 26 个 x4x 杂交槽分类登记父本母本，用于 h4h 混元乘法爆发生态。
 *
 * 每个种子包含：
 *   - id         唯一 id (槽名_数字)
 *   - name       人类可读名 (用于杂交产物显示"父×母")
 *   - category   分类 (ai / math / spacetime / robotics / medical / aerospace / xr …)
 *   - meaning    这个种子的语义 (便于意义解释器大齿轮串联)
 *   - payload    可选：扩展属性 { TRL 技术就绪度 / 风险 / 行业 / ... }
 *
 * 用法：
 *   const { TECH_SEEDS, registerTechSeeds, registerSlotSeeds } = require('./seed_library');
 *   const h4h = require('./h4h_hybrid').create();
 *   registerTechSeeds(h4h);  // 一次把 26 槽 × 6+ 真实科技种子都灌进去
 */
'use strict';

// ============================================================
// 26 槽 × 8~14 条真实科技种子 = 约 240 条种子库
// ============================================================
const TECH_SEEDS = {
  // —— a4a · 伙码 Agent × API 顶层入口
  a4a: [
    { id:'a4a_1', name:'多智能体 AutoGen 群聊',      category:'ai',         meaning:'多 Agent 协作结伙群组决策' },
    { id:'a4a_2', name:'伙码行码 CocoonLang API',    category:'api',        meaning:'声韵调编程语言对外 HTTP 接口' },
    { id:'a4a_3', name:'WASM 插件能力调用',          category:'sandbox',    meaning:'浏览器沙箱桥接宿主能力对外 API' },
    { id:'a4a_4', name:'WebSocket 协同控制总线',     category:'api',        meaning:'脑机/无人机/具身 协同总线' },
    { id:'a4a_5', name:'MQF 消息链审计查询',         category:'mqf',        meaning:'杂交/审判/记账 对外查询 API' },
    { id:'a4a_6', name:'联邦学习跨平台协同',         category:'ai',         meaning:'多壳数据隐私协同结伙' },
    { id:'a4a_7', name:'OpenAPI 3.1 自动描述',       category:'api',        meaning:'A4A 路由自动生成 Swagger 描述' },
  ],

  // —— b4b · 绑定/贝叶斯/双足 → 人形机器人
  b4b: [
    { id:'b4b_1', name:'双足步态 biped_gait',       category:'robotics',   meaning:'人形双足离线轨迹 + 在线反馈 MPC' },
    { id:'b4b_2', name:'全身平衡 balance_mpc',      category:'robotics',   meaning:'全身关节 MPC 质心零力矩点' },
    { id:'b4b_3', name:'灵巧手操作 dexterous_hand', category:'robotics',   meaning:'多指抓取/拧螺丝/穿针引线' },
    { id:'b4b_4', name:'贝叶斯概率绑定 bayes_binding', category:'ai',      meaning:'感知到动作的概率绑定' },
    { id:'b4b_5', name:'肌肉肌腱绑定 actuation_bind', category:'robotics', meaning:'液压/人工肌肉/绳驱绑定' },
    { id:'b4b_6', name:'足式地形 SLAM terrain_foot_slam', category:'robotics', meaning:'足端接触里程计 + 视觉 SLAM' },
    { id:'b4b_7', name:'跌倒恢复起身 fall_recovery',  category:'robotics',  meaning:'跌倒检测→起身轨迹' },
    { id:'b4b_8', name:'全身动捕 retarget mocap',    category:'robotics',   meaning:'VR 手套/动捕→全身关节重定向' },
  ],

  // —— c4c · 范畴/全景/容器 → 360 相机
  c4c: [
    { id:'c4c_1', name:'360° 全景拼接 panorama',     category:'xr',         meaning:'多路鱼眼→球面投影拼接' },
    { id:'c4c_2', name:'范畴论 函子 functor',         category:'math',       meaning:'跨域态射映射' },
    { id:'c4c_3', name:'容器化 WASM 沙箱',            category:'sandbox',    meaning:'蛹茧壳作为容器' },
    { id:'c4c_4', name:'车载环视 4×鱼眼拼接',         category:'automotive', meaning:'ADAS 360 环视' },
    { id:'c4c_5', name:'自然语言 范畴组合 CCG',       category:'ai',         meaning:'组合范畴语法 语义合成' },
    { id:'c4c_6', name:'VR 全景直播 live360',         category:'xr',         meaning:'360° 低延迟直播' },
    { id:'c4c_7', name:'数字孪生 WebGL 容器',         category:'iot',        meaning:'Cocoon 壳里孵化数字孪生' },
  ],

  // —— d4d · 道/动力递推 → 递归 · 递推 · 数论
  d4d: [
    { id:'d4d_1', name:'尾递归 TCO 递推',             category:'math',       meaning:'声韵母程序尾递归优化' },
    { id:'d4d_2', name:'动力系统递推 DNN 反向',       category:'ai',         meaning:'DNN 反传梯度递推' },
    { id:'d4d_3', name:'道家 阴阳递推 64 卦',         category:'philosophy', meaning:'0元阴阳→2仪→4象→8卦→64卦' },
    { id:'d4d_4', name:'动态规划 DP 递推表',          category:'math',       meaning:'最优子结构 × 重叠子问题' },
    { id:'d4d_5', name:'RNN 时间递推 LSTM',           category:'ai',         meaning:'时序依赖门控递推' },
    { id:'d4d_6', name:'扩散模型 去噪步递推',         category:'ai',         meaning:'一步步从噪声还原数据' },
    { id:'d4d_7', name:'有限差分法 PDE 递推',         category:'math',       meaning:'物理仿真 时空步递推' },
    { id:'d4d_8', name:'编译器 自举递归编译',         category:'compiler',   meaning:'CocoonLang 自举编译' },
  ],

  // —— e4e · 易石互联 / 边缘计算
  e4e: [
    { id:'e4e_1', name:'6G 太赫兹通信 THz',          category:'telecom',    meaning:'边缘节点 太赫兹高速链路' },
    { id:'e4e_2', name:'易石互联 IoT 设备',           category:'iot',        meaning:'低功耗工业互联网' },
    { id:'e4e_3', name:'边缘 AI 推理 端侧大模型',     category:'ai',         meaning:'手机/具身 端侧大模型' },
    { id:'e4e_4', name:'数字城市 边缘节点',           category:'iot',        meaning:'智慧城市多接入边缘' },
    { id:'e4e_5', name:'边缘容器 K3S/WASMEdge',       category:'sandbox',    meaning:'蛹茧在边缘节点穿壳孵化' },
    { id:'e4e_6', name:'TSN 时间敏感网络 工业',        category:'telecom',    meaning:'工业物联网 确定性调度' },
  ],

  // —— f4f · 分形/傅立叶/联邦学习
  f4f: [
    { id:'f4f_1', name:'分形几何 曼德博集合',         category:'math',       meaning:'自相似结构无限细节' },
    { id:'f4f_2', name:'快速傅立叶 FFT',              category:'math',       meaning:'时域↔频域桥梁' },
    { id:'f4f_3', name:'联邦学习 Federated',          category:'ai',         meaning:'跨平台参数聚合不碰数据' },
    { id:'f4f_4', name:'小波分解 Wavelet',            category:'signal',     meaning:'多尺度信号' },
    { id:'f4f_5', name:'联邦金融隐私计算',            category:'finance',    meaning:'银行壳与数据隐私' },
    { id:'f4f_6', name:'神经傅立叶算子 FNO',          category:'ai',         meaning:'PDE 求解 傅立叶神经算子' },
    { id:'f4f_7', name:'分形脑电 EEG',                category:'medical',    meaning:'脑电分形维数认知负荷' },
  ],

  // —— g4g · 群论/激光/LiDAR
  g4g: [
    { id:'g4g_1', name:'激光雷达 LiDAR 点云',         category:'sensing',    meaning:'自动驾驶/机器人 3D 感知' },
    { id:'g4g_2', name:'群论 对称群 Lie 群',          category:'math',       meaning:'李群李代数 机器人运动' },
    { id:'g4g_3', name:'激光 3D 打印 SLA',            category:'manufacturing', meaning:'光固化 3D 打印' },
    { id:'g4g_4', name:'激光 SLAM 扫地机',            category:'robotics',   meaning:'室内 LiDAR 激光 SLAM' },
    { id:'g4g_5', name:'哥德尔不完备 证明',            category:'philosophy', meaning:'形式系统边界' },
    { id:'g4g_6', name:'概率图模型 贝叶斯群',          category:'ai',         meaning:'群结构 × 概率推断' },
    { id:'g4g_7', name:'激光扫描考古 3D',             category:'scientific', meaning:'文物/化石 毫米级扫描' },
  ],

  // —— h4h · ★混元本体枢纽槽 (已挂基础种子，这里再加高价值)
  h4h: [
    { id:'h4h_1', name:'混元 0 元有无因果',            category:'philosophy', meaning:'介于有无之间的混元初始态' },
    { id:'h4h_2', name:'四元镜像 归属主导权',         category:'philosophy', meaning:'高维镜像归属/从属逻辑' },
    { id:'h4h_3', name:'大齿轮 时空层级聚类',         category:'spacetime',  meaning:'一维时间层级聚类代数' },
    { id:'h4h_4', name:'天平裁判 红黑白账',           category:'judge',      meaning:'二元三元相互参考' },
    { id:'h4h_5', name:'猫码 好码判定系统',            category:'catcode',    meaning:'白猫黑猫抓到老鼠就是好猫' },
    { id:'h4h_6', name:'锥相位枢纽链 根毛深度',       category:'conephase',  meaning:'根毛深度可信度注意力锚点' },
    { id:'h4h_7', name:'蛹茧 穿壳寄生机制',            category:'cocoon',     meaning:'WASM+26 平台自由穿壳' },
    { id:'h4h_8', name:'人类常识 五感 + 语言',         category:'commonsense',meaning:'视觉听觉触觉语言 + 道德常识' },
    { id:'h4h_9', name:'社会规则 法律 + 金融',         category:'commonsense',meaning:'货币/合同/法规' },
  ],

  // —— i4i · 意念输入 BCI 脑机接口
  i4i: [
    { id:'i4i_1', name:'侵入式 BCI Neuralink',        category:'bci',        meaning:'皮层电极 意念打字' },
    { id:'i4i_2', name:'非侵入 EEG 意念',             category:'bci',        meaning:'头环/头盔 脑电采集' },
    { id:'i4i_3', name:'fNIRS 近红外脑成像',           category:'medical',    meaning:'额叶/运动区近红外' },
    { id:'i4i_4', name:'P300 拼写脑机',                category:'bci',        meaning:'视觉诱发电位 打字系统' },
    { id:'i4i_5', name:'运动想象 MI 控制机械臂',       category:'robotics',   meaning:'mu 节律 机械臂 7 DOF' },
    { id:'i4i_6', name:'闭环 BCI 深度脑刺激 DBS',     category:'medical',    meaning:'帕金森 DBS 自适应闭环' },
    { id:'i4i_7', name:'意念输入 大模型 思维补全',     category:'ai',         meaning:'EEG→LLM 意念解码' },
  ],

  // —— j4j · 几何/积分/激光
  j4j: [
    { id:'j4j_1', name:'微分几何 黎曼几何',           category:'math',       meaning:'流形 曲率 联络' },
    { id:'j4j_2', name:'积分 测度论 Lebesgue',         category:'math',       meaning:'可测函数积分' },
    { id:'j4j_3', name:'几何深度学习 网格 MeshCNN',    category:'ai',         meaning:'3D 形状几何深度网络' },
    { id:'j4j_4', name:'激光雕刻 精密微加工',          category:'manufacturing', meaning:'亚微米激光制造' },
    { id:'j4j_5', name:'逆运动学 IK 7 轴机械臂',       category:'robotics',   meaning:'7 DOF 逆解雅可比' },
    { id:'j4j_6', name:'3D 打印路径 几何切片',         category:'manufacturing', meaning:'STL→Gcode' },
    { id:'j4j_7', name:'计算机图形学 几何着色器',      category:'graphics',   meaning:'程序化几何生成' },
  ],

  // —— k4k · 3D 空间感知/康德先验
  k4k: [
    { id:'k4k_1', name:'3D 空间视觉 双目立体',        category:'sensing',    meaning:'双目视差 深度图' },
    { id:'k4k_2', name:'NeRF 3D 辐射场',              category:'ai',         meaning:'2D→3D 神经重建' },
    { id:'k4k_3', name:'康德先验时空十二范畴',         category:'philosophy', meaning:'先天直观形式' },
    { id:'k4k_4', name:'RGBD SLAM ORB-SLAM3',         category:'robotics',   meaning:'视觉惯导紧耦合 SLAM' },
    { id:'k4k_5', name:'空间音频 Ambisonics 3D',       category:'audio',      meaning:'7.1.4 三维声场' },
    { id:'k4k_6', name:'具身 VLA 空间动作 grounding',  category:'ai',         meaning:'语言指令→3D 动作' },
    { id:'k4k_7', name:'空间计算 visionOS',            category:'xr',         meaning:'空间计算应用' },
  ],

  // —— l4l · 李群/链/MQF 消息链 (l4l 作为槽，也存种子)
  l4l: [
    { id:'l4l_1', name:'李群 SE(3) 位姿变换',         category:'math',       meaning:'6D 位姿 机器人' },
    { id:'l4l_2', name:'黎曼流形 梯度下降',            category:'ai',         meaning:'流形上的优化' },
    { id:'l4l_3', name:'区块链 哈希存证链',            category:'ledger',     meaning:'不可篡改交易链' },
    { id:'l4l_4', name:'MQF 消息链 锥相位注意力',       category:'mqf',        meaning:'杂交/审判/记账 消息' },
    { id:'l4l_5', name:'大语言模型 Chain-of-Thought',   category:'ai',         meaning:'思维链多步推理' },
    { id:'l4l_6', name:'供应链金融区块链',              category:'finance',    meaning:'仓单/应收 确权' },
    { id:'l4l_7', name:'LiDAR-Inertial 因子图链',      category:'robotics',   meaning:'SLAM 因子图优化' },
  ],

  // —— m4m · MRI 核磁/货币/美感
  m4m: [
    { id:'m4m_1', name:'MRI 核磁成像 fMRI/DTI',       category:'medical',    meaning:'大脑结构/功能/白质束' },
    { id:'m4m_2', name:'CBDC 央行数字货币',            category:'finance',    meaning:'数字人民币/数字欧元' },
    { id:'m4m_3', name:'美学评估 多模态审美模型',       category:'ai',         meaning:'视觉/音乐/诗歌 审美打分' },
    { id:'m4m_4', name:'银行核心交易清算',              category:'finance',    meaning:'银行壳 核心账务' },
    { id:'m4m_5', name:'核磁谱 NMR 有机结构',           category:'scientific', meaning:'化学结构解析' },
    { id:'m4m_6', name:'货币供应量 M2 流动性',         category:'finance',    meaning:'宏观金融流动性' },
    { id:'m4m_7', name:'数字艺术 AI 美感生成',          category:'xr',         meaning:'文本→美感图像' },
    { id:'m4m_8', name:'脑磁图 MEG 认知',              category:'medical',    meaning:'毫秒级脑磁成像' },
  ],

  // —— n4n · 神经网络/NeRF
  n4n: [
    { id:'n4n_1', name:'Transformer 多头注意力',       category:'ai',         meaning:'LLM 基础单元' },
    { id:'n4n_2', name:'NeRF-Instant NGP',            category:'ai',         meaning:'实时 3D 神经辐射场' },
    { id:'n4n_3', name:'数论 素数分布',                category:'math',       meaning:'黎曼ζ 素数加密' },
    { id:'n4n_4', name:'神经符号 NeSy 结合',            category:'ai',         meaning:'深度 + 逻辑符号' },
    { id:'n4n_5', name:'神经网络 剪枝量化蒸馏',         category:'ai',         meaning:'大模型→端侧小模型' },
    { id:'n4n_6', name:'GNN 图神经网络',                category:'ai',         meaning:'分子/社交/知识图谱' },
  ],

  // —— o4o · 输出/对偶
  o4o: [
    { id:'o4o_1', name:'语音输出 TTS 自然语音合成',     category:'audio',      meaning:'大模型 TTS 24kHz' },
    { id:'o4o_2', name:'对偶线性规划 LP 对偶',         category:'math',       meaning:'原问题↔对偶问题' },
    { id:'o4o_3', name:'机械臂轨迹输出 Torque',        category:'robotics',   meaning:'关节力矩控制输出' },
    { id:'o4o_4', name:'AR 眼镜全息显示输出',           category:'xr',         meaning:'视网膜级 3D 全息' },
    { id:'o4o_5', name:'偶配对 最优运输 OT',            category:'math',       meaning:'Wasserstein 距离' },
    { id:'o4o_6', name:'卫星下行帧 数传输出',           category:'aerospace',  meaning:'X 频段 高速下行' },
  ],

  // —— p4p · 3D 打印/排列/平衡
  p4p: [
    { id:'p4p_1', name:'FDM 熔融沉积 3D 打印',         category:'manufacturing', meaning:'桌面 3D 打印机' },
    { id:'p4p_2', name:'金属 SLM 激光烧结',            category:'manufacturing', meaning:'航空航天金属件' },
    { id:'p4p_3', name:'排列群 Permutation',           category:'math',       meaning:'全排列组合爆炸' },
    { id:'p4p_4', name:'前庭觉平衡 站立姿态',           category:'robotics',   meaning:'IMU→倒立摆平衡' },
    { id:'p4p_5', name:'生物 3D 打印器官支架',         category:'medical',    meaning:'细胞支架 类器官' },
    { id:'p4p_6', name:'PCB 3D 打印 电子电路',         category:'manufacturing', meaning:'柔性电路打印' },
    { id:'p4p_7', name:'混凝土 3D 打印建筑',           category:'manufacturing', meaning:'建筑结构快速成型' },
  ],

  // —— q4q · 量子/群/潜意识
  q4q: [
    { id:'q4q_1', name:'超导量子计算 qubit 1000+',      category:'quantum',    meaning:'含噪声中规模量子 NISQ' },
    { id:'q4q_2', name:'量子群 SU(2) 自旋',            category:'math',       meaning:'量子态/角动量' },
    { id:'q4q_3', name:'弗洛伊德潜意识 梦境解码',       category:'philosophy', meaning:'无意识→意识内容' },
    { id:'q4q_4', name:'量子密钥分发 QKD BB84',         category:'crypto',     meaning:'不可窃听密钥' },
    { id:'q4q_5', name:'量子机器学习 QML',              category:'ai',         meaning:'量子态 × 参数化线路' },
    { id:'q4q_6', name:'潜意识 AI 故事生成',            category:'ai',         meaning:'梦境符号→故事' },
    { id:'q4q_7', name:'量子传感 磁强计',               category:'sensing',    meaning:'脑磁/心磁 高灵敏' },
  ],

  // —— r4r · 奖励 RL/检索 RAG/仁义
  r4r: [
    { id:'r4r_1', name:'强化学习 RLHF 奖励模型',        category:'ai',         meaning:'人类偏好对齐' },
    { id:'r4r_2', name:'RAG 检索增强生成',             category:'ai',         meaning:'向量库+大模型 事实性' },
    { id:'r4r_3', name:'仁义 道德大模型对齐',           category:'ai',         meaning:'中文伦理 仁义礼智信' },
    { id:'r4r_4', name:'具身强化学习 VLA 奖励',         category:'robotics',   meaning:'语言指令→奖励 shaping' },
    { id:'r4r_5', name:'荣格原型 心理学符号',           category:'philosophy', meaning:'阴影/自性/人格面具' },
    { id:'r4r_6', name:'信用评分 RL 违约风险',          category:'finance',    meaning:'时间差分信用评估' },
    { id:'r4r_7', name:'多跳 RAG 链式检索',             category:'ai',         meaning:'复杂问题分解检索' },
  ],

  // —— s4s · 时空/熵/卫星
  s4s: [
    { id:'s4s_1', name:'大齿轮时空层级聚类',           category:'spacetime',  meaning:'意义解释器 多层时间空间' },
    { id:'s4s_2', name:'卫星星链 Starlink 下行',       category:'aerospace',  meaning:'低轨卫星 互联网' },
    { id:'s4s_3', name:'熵 热力学/信息熵 S=klnW',      category:'physics',    meaning:'有序↔无序 价值' },
    { id:'s4s_4', name:'数论 椭圆曲线 ECC',             category:'math',       meaning:'区块链/数字签名' },
    { id:'s4s_5', name:'时空图神经网络 交通预测',       category:'ai',         meaning:'时空交通/气象预测' },
    { id:'s4s_6', name:'北斗/GNSS 时空坐标',            category:'aerospace',  meaning:'纳秒级时间同步' },
    { id:'s4s_7', name:'地球观测卫星 SAR 成像',         category:'scientific', meaning:'合成孔径雷达 全天候' },
    { id:'s4s_8', name:'狭义相对论 时空变换 Lorentz',  category:'physics',    meaning:'高速运动 时间膨胀' },
  ],

  // —— t4t · 拓扑/3D 投影/痛觉
  t4t: [
    { id:'t4t_1', name:'拓扑学 同伦群/同调群',          category:'math',       meaning:'连续形变不变量' },
    { id:'t4t_2', name:'3D 投影 全息成像 Hologram',    category:'xr',         meaning:'相位恢复 真3D' },
    { id:'t4t_3', name:'痛觉模型 触觉电子皮肤',         category:'robotics',   meaning:'柔性压力/温度 皮肤' },
    { id:'t4t_4', name:'拓扑数据分析 TDA Mapper',       category:'ai',         meaning:'高维数据拓扑骨架' },
    { id:'t4t_5', name:'3D 投影 剧场全息',              category:'xr',         meaning:'全息投影剧场' },
    { id:'t4t_6', name:'图神经网络 TopoGNN',            category:'ai',         meaning:'拓扑感知图网络' },
    { id:'t4t_7', name:'3D 投影教育 解剖',              category:'medical',    meaning:'立体解剖教学' },
  ],

  // —— u4u · 无人机蜂群
  u4u: [
    { id:'u4u_1', name:'无人机蜂群 UAV Swarm 1000+',   category:'robotics',   meaning:'自组织编队' },
    { id:'u4u_2', name:'垂直起降 VTOL eVTOL',          category:'aerospace',  meaning:'低空载人飞行器' },
    { id:'u4u_3', name:'无人机 电力巡检 LiDAR',         category:'robotics',   meaning:'高压线路 缺陷识别' },
    { id:'u4u_4', name:'农用植保无人机 喷药',           category:'iot',        meaning:'亩级精准农业' },
    { id:'u4u_5', name:'蜂群对抗 博弈强化学习',         category:'ai',         meaning:'红蓝对抗 无人机博弈' },
    { id:'u4u_6', name:'快递无人机 末端配送',            category:'iot',        meaning:'10km 2kg 配送' },
    { id:'u4u_7', name:'火星无人机 Ingenuity 类',       category:'aerospace',  meaning:'大气稀薄 飞行' },
  ],

  // —— v4v · AR/VR/视频生成/向量空间
  v4v: [
    { id:'v4v_1', name:'VR 全自由度 全身追踪',         category:'xr',         meaning:'头手 6DOF + 全身IK' },
    { id:'v4v_2', name:'AR 眼镜 空间锚点',             category:'xr',         meaning:'现实空间持久锚点' },
    { id:'v4v_3', name:'文生视频 Sora/VDT',            category:'ai',         meaning:'自然语言→视频' },
    { id:'v4v_4', name:'向量数据库 Embedding',         category:'ai',         meaning:'亿级向量 近似检索' },
    { id:'v4v_5', name:'MR 混合现实 手术导航',         category:'medical',    meaning:'CT→患者 全息引导' },
    { id:'v4v_6', name:'向量空间语义 词嵌入',           category:'ai',         meaning:'word2vec→LLM embedding' },
    { id:'v4v_7', name:'VR 元宇宙 社交',               category:'xr',         meaning:'虚拟形象 多人空间' },
  ],

  // —— w4w · 世界模型/可穿戴/微分几何
  w4w: [
    { id:'w4w_1', name:'AGI 世界模型 World Model',     category:'ai',         meaning:'通用智能 环境仿真预测' },
    { id:'w4w_2', name:'Apple Vision 可穿戴空间',       category:'xr',         meaning:'可穿戴空间计算' },
    { id:'w4w_3', name:'微分几何 流形 GNN',             category:'ai',         meaning:'图+流形深度学习' },
    { id:'w4w_4', name:'智能手表 健康监测',             category:'medical',    meaning:'PPG/ECG/SpO2 24h' },
    { id:'w4w_5', name:'气候大模型 全球预测',           category:'scientific', meaning:'2 周/季度 天气模拟' },
    { id:'w4w_6', name:'微分方程 求解器 神经 ODE',      category:'ai',         meaning:'动力系统 NN 解' },
    { id:'w4w_7', name:'可穿戴 外骨骼辅助',             category:'robotics',   meaning:'助力行走 康复' },
  ],

  // —— x4x · X光/线性代数/信息论
  x4x: [
    { id:'x4x_1', name:'X光 DR/CT 医学影像',           category:'medical',    meaning:'2D 平片/3D CT' },
    { id:'x4x_2', name:'线性代数 矩阵分解 SVD/LU',     category:'math',       meaning:'LLM 核心数学工具' },
    { id:'x4x_3', name:'香农信息论 熵 信道容量',        category:'math',       meaning:'通信极限' },
    { id:'x4x_4', name:'CT 锥束重建 CBCT',              category:'medical',    meaning:'牙科/放疗 CT' },
    { id:'x4x_5', name:'线性注意力 LinearAttention',    category:'ai',         meaning:'长序列 O(n) 注意力' },
    { id:'x4x_6', name:'安检 X 光 危险物品识别',        category:'sensing',    meaning:'CNN 危险品检测' },
    { id:'x4x_7', name:'压缩感知 CS-MRI 欠采样',        category:'medical',    meaning:'减少辐射 加速成像' },
  ],

  // —— y4y · 易经/因果/元宇宙
  y4y: [
    { id:'y4y_1', name:'易经 64 卦因果推演',            category:'philosophy', meaning:'卦变 爻辞 预测' },
    { id:'y4y_2', name:'因果推断 Causal DAG',          category:'ai',         meaning:'do-calculus 反事实' },
    { id:'y4y_3', name:'元宇宙 多人同步仿真',           category:'xr',         meaning:'大规模数字孪生' },
    { id:'y4y_4', name:'博弈论 纳什均衡',              category:'math',       meaning:'多智能体 非合作博弈' },
    { id:'y4y_5', name:'元强化学习 MetaRL',            category:'ai',         meaning:'学习如何学习' },
    { id:'y4y_6', name:'元宇宙 经济系统 NFT+通货',     category:'finance',    meaning:'数字资产确权' },
    { id:'y4y_7', name:'因果发现 PC 算法',              category:'ai',         meaning:'观测→因果图' },
  ],

  // —— z4z · 组合/张量/中医辨证
  z4z: [
    { id:'z4z_1', name:'组合爆炸 NP 旅行商',           category:'math',       meaning:'TSP/组合优化' },
    { id:'z4z_2', name:'张量网络 Tensor Train/MPO',    category:'math',       meaning:'高维张量 低秩分解' },
    { id:'z4z_3', name:'中医辨证 八纲辨证',             category:'medical',    meaning:'阴阳表里寒热虚实' },
    { id:'z4z_4', name:'大模型 Transformer 张量并行',   category:'ai',         meaning:'8 卡张量并行训练' },
    { id:'z4z_5', name:'AI 中药配伍推荐',               category:'medical',    meaning:'君臣佐使 辨证组方' },
    { id:'z4z_6', name:'量子张量网络 MPS',             category:'quantum',    meaning:'多体波函数压缩' },
    { id:'z4z_7', name:'组合设计 实验 DOE',             category:'math',       meaning:'因子实验 正交表' },
  ],
};

// 特殊槽：具身智能 embodied/humanoid/drone/medical 等直接对应寄生壳，种子可"跨槽"——我们也额外提供：
const EXTRA_SEEDS = {
  // 通用跨类：AI × 具身 VLA 综合（挂在 j4j b4b u4u 同时，也在 h4h 里重复，允许）
};

// ============================================================
// 批量注册函数
// ============================================================
function registerTechSeeds(h4hRegistry, options = {}) {
  if (!h4hRegistry || typeof h4hRegistry.registerSeed !== 'function') {
    throw new Error('seed_library.registerTechSeeds 需要 H4HRegistry 实例');
  }
  let count = 0;
  const skipSlots = options.skipSlots || [];
  const onlySlots = options.onlySlots || null;
  for (const slotId of Object.keys(TECH_SEEDS)) {
    if (skipSlots.includes(slotId)) continue;
    if (onlySlots && !onlySlots.includes(slotId)) continue;
    const list = TECH_SEEDS[slotId];
    // 允许 override 每个种子的 payload
    for (const seed of list) {
      try {
        h4hRegistry.registerSeed(slotId, Object.assign(
          {}, seed,
          (options.enrichPayload ? { payload: options.enrichPayload(seed, slotId) } : {})
        ));
        count++;
      } catch (_) { /* ignore per-slot errors */ }
    }
  }
  // 兜底：任何槽 registerSeed < 6 条的，用默认种子补齐（保证 multiplyAll(6) 时每个槽 C(6,2)=15 对）
  if (options.ensureAtLeastPerSlot && options.ensureAtLeastPerSlot > 0) {
    const need = options.ensureAtLeastPerSlot;
    for (const slotId of h4hRegistry.listSlotIds()) {
      const s = h4hRegistry.getSlot(slotId);
      if (!s) continue;
      const have = s.registered ? s.registered.length : 0;
      for (let i = have; i < need; i++) {
        try {
          h4hRegistry.registerSeed(slotId, {
            id: `${slotId}_fill_${i}`,
            name: `${s.name || slotId} 基础填充 ${i + 1}`,
            category: s.category || 'general',
            meaning: `${s.meaning || slotId} 基础填充概念 ${i + 1}`,
          });
          count++;
        } catch (_) {}
      }
    }
  }
  return { registered: count, librarySize: Object.values(TECH_SEEDS).reduce((s, a) => s + a.length, 0) };
}

/** 单槽注册便捷函数 */
function registerSlotSeeds(h4hRegistry, slotId, list) {
  let n = 0;
  for (const s of list || []) {
    try { h4hRegistry.registerSeed(slotId, s); n++; } catch (_) {}
  }
  return n;
}

/** 按 TOP 价值分数取已杂交产物：从 h4h marriages 或 MQF audit 链里取 */
function topMarriagesByScore(h4hRegistry, n = 10, filterFn) {
  const arr = h4hRegistry.getMarriagesBySlot ? h4hRegistry.getMarriagesBySlot() : [];
  const scored = arr.map(m => Object.assign({}, m, {
    score: m.child && typeof m.child.valueScore === 'number' ? m.child.valueScore : 0,
  }));
  const filtered = filterFn ? scored.filter(filterFn) : scored;
  filtered.sort((a, b) => (b.score || 0) - (a.score || 0));
  return filtered.slice(0, n);
}

// ============================================================
// 挂载到 H4HRegistry 原型（可选：调用 enableTopMarriages() 才生效）
// ============================================================
function enableTopMarriages(H4HRegistryClass) {
  if (!H4HRegistryClass || !H4HRegistryClass.prototype) return;
  if (H4HRegistryClass.prototype.topMarriages) return; // 幂等
  H4HRegistryClass.prototype.topMarriages = function (n = 10, filterFn) {
    return topMarriagesByScore(this, n, filterFn);
  };
  H4HRegistryClass.prototype.registerTechSeeds = function (options) {
    return registerTechSeeds(this, options);
  };
}

module.exports = {
  TECH_SEEDS,
  EXTRA_SEEDS,
  registerTechSeeds,
  registerSlotSeeds,
  topMarriagesByScore,
  enableTopMarriages,
};
