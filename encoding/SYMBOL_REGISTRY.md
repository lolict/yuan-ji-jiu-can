# SYMBOL_REGISTRY.md — 满全法符号注册表

跨文件统一符号表。所有新代码必须使用这里定义的符号，不得另创别名。

════════════════════════════════════════════════════════════════
一、核心容器常量
════════════════════════════════════════════════════════════════

CODE_EMPTY        = 0x00    空容器·可被填入
CODE_SELF         = 0x01    我·锚定容器
CODE_OVERFLOW     = 0xFF    溢出·终止符
BITS_PER_CHAR     = 8       每字符比特数（不可再少, log₂256）
CHAR_CAPACITY     = 254     最大容量 (256 - 2保留)

注: BITS_PER_CELL、BYTES_PER_CELL、CAPACITY 是旧别名，
    新代码统一用 BITS_PER_CHAR 和 CHAR_CAPACITY。


════════════════════════════════════════════════════════════════
二、类型枚举
════════════════════════════════════════════════════════════════

sym_type_t:
    T_BASE     = 1    基元 (a o e i u v)
    T_TONE     = 2    基调 (ā á ǎ à)
    T_COMPOUND = 3    复合 (ai ei ao ou an en ang eng ong er)
    T_SYMBOL   = 4    象征 (+ - = . , ! ? 等)
    T_EXTRA    = 5    溢出符号（未归类）
    T_MODIFIER = 7    修饰零件 (ˊ ˇ ˋ ˆ ˙)
    T_UNKNOWN  = 0xFF 未分类

sym_level_t:
    L1 = 0x10    基元本体层
    L2 = 0x20    修饰零件层
    L3 = 0x30    装配成品层
    L_UNKNOWN = 0xFF

verdict_t:
    PASS          = 0
    REJ_LEVEL     = 1    跨层级拒绝
    REJ_MODIFIER  = 2    修饰零件拒绝
    REJ_TONE      = 3    跨基调拒绝
    REJ_EXTRA     = 4    溢出符号拒绝
    REJ_UNKNOWN   = 5    未分类拒绝
    REJ_DIFFERENT = 6    类型不同拒绝


════════════════════════════════════════════════════════════════
三、各文件首次引入的符号（禁止在其他文件中重复定义）
════════════════════════════════════════════════════════════════

container_bitmap.c:
    cell_t            容器单元结构
    CELL_TABLE_SIZE   宏·计算表长
    name_to_code()    名称→编码
    code_to_name()    编码→名称

njsg_os.c:
    reg_t             注册表结构
    sig_t             部件表结构
    TABLE_SZ / SIGS_SZ  表长宏
    reg_type() / reg_level() / reg_anchor()  注册表查询
    judge()           裁决函数
    n2c() / c2n()     名称↔编码
    find_sig()        部件查找

court.c:
    crime_type_t      犯罪类型枚举
    crime_record_t    犯罪记录结构
    emerged_rule_t    涌现规则结构
    court_t           法庭结构
    MAX_RECORDS = 256
    MAX_RULES = 16
    EMERGE_THRESHOLD = 3    涌现阈值
    court_log()       记录犯罪
    simple_reg_t      简化注册表（仅court.c内部用）
    judge_and_log()   裁决+记录

separator.c:
    flow_id_t         流ID枚举
    sep_cell_t        分离器单元
    separator_t       分离器结构
    SEP_TBL_SZ        表长宏
    sep_type()        查类型
    sep_route()       路由函数
    sep_dispatch()    分发函数
    flow_preload()    预加载流
    flow_interpret()  解释流
    flow_harmony()    谐音流
    flow_sovereign()  主权流
    flow_overflow()   溢出流
    flow_unknown()    未知流

lan_3d_coord.c:
    axis_t            轴枚举 (AXIS_SURFACE/STRUCT/LATERAL)
    coord_node_t      坐标节点结构
    space_bucket_t    空间桶结构
    BUCKET_COUNT = 512
    COORD_BITS = 6
    encode_coord()    编码坐标
    decode_coord()    解码坐标
    euclidean_dist()  欧氏距离
    manhattan_dist()  曼哈顿距离
    find_nearby()     邻近搜索


════════════════════════════════════════════════════════════════
四、禁止使用的别名（新代码出现以下符号即违规）
════════════════════════════════════════════════════════════════

BITS_PER_CELL    → 改用 BITS_PER_CHAR
BYTES_PER_CELL   → 改用 1 (BITS_PER_CHAR/8)
CAPACITY         → 改用 CHAR_CAPACITY
UNKNOWN          → 改用 L_UNKNOWN 或 T_UNKNOWN（区分层级/类型）


════════════════════════════════════════════════════════════════
五、类型纪律规则（五门审判顺序，不可调换）
════════════════════════════════════════════════════════════════

第0门: Level   — L1/L2/L3 校验，跨层拒绝
第1门: Type    — BASE/TONE/COMPOUND/SYMBOL/EXTRA 类型一致
第2门: Anchor  — 同锚校验
第3门: Aesthetic — 审美过滤
第4门: Weaving — 编织平衡


════════════════════════════════════════════════════════════════
六、硬编码拒绝（不可绕过）
════════════════════════════════════════════════════════════════

a = á  → 第0门拒绝（L1≠L3 跨层级）
e = ě  → 第0门拒绝（L1≠L3 跨层级）
任意声调与任意声调配对 → 第2门拒绝（CROSS_TONE）
任意复合音与任意复合音配对 → 第2门拒绝（DIFFERENT）
任意象征符号与任意象征符号配对 → 第2门拒绝（DIFFERENT）


════════════════════════════════════════════════════════════════
七、容器本体论命名铁律
════════════════════════════════════════════════════════════════

存在单位叫"容器"，不叫"类/对象/文件/进程/模块"。
身份锚点叫"公约数"。
边界扩展叫"公倍数"。
被排斥的叫"负样本"。
运行时叫"折叠/展开"，不叫"执行/运行"。
