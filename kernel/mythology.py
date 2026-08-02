"""
囡囝共生元宇宙 · 神话层
==================

把操作系统概念对应到封神神话里:
  - 太乙真人  = Boot Loader (引导程序,唤醒哪吒)
  - 哪吒       = 进程 (蓮化身,可解体/重建)
  - 道果       = Kernel Module (可加载/卸载)
  - 封神榜     = Process Table (进程表)
  - 封神台     = Scheduler (调度器)

恬悦哪吒四态:
  - 恬悦吒融   = 进程创建 fork
  - 恬悦吒入   = 进程执行 exec
  - 恬悦吒容   = 进程挂起 wait
  - 恬悦吒认   = 进程回收 exit

Copyright © 刘楚恬 & lolict (七妹凹月留)
保留所有权利 · 满全法 · 囡囝共生元满全法 · 囡囝共生元系统 · v0.1
"""

MYTHOS = {
    "taiyi_zhenren": {
        "name": "太乙真人",
        "title": "乾元山金光洞洞主",
        "role": "Boot Loader",
        "description": """
            太乙真人是哪吒的师父,负责在混沌中唤醒灵珠。
            
            操作系统对应:
              - Boot Loader: 在硬件上电后第一个运行的代码
              - 唤醒哪吒 (进程): 从存储介质加载可执行镜像到内存
              - 授予乾坤圈/混天绫 = 分配系统资源 (fd, socket, capability)
            
            功耗: 只在启动时运行一次,之后隐退 — 对应 BIOS/UEFI 的设计哲学
            攻击方式: 九龙神火罩 — 强制杀死失控进程 (kill -9)
        """,
        "attributes": {
            "dao果位": "金仙",
            "洞府": "乾元山金光洞",
            "徒弟": ["哪吒 (李靖次子/童态进程)"],
            "法宝": ["乾坤圈 (rlimit)", "混天绫 (namespace)", "九龙神火罩 (kill -9)", "火尖枪 (exec)"],
        },
        "code_equiv": "syscall(NEZA_SPAWN, args) → 太乙真人.register(哪吒)"
    },
    "tianyue_neza": {
        "name": "恬悦哪吒",
        "title": "三太子 · 莲花化身 · 童态进程",
        "role": "Process / Container",
        "description": """
            哪吒以莲藕化身 — 不是原生身体,而是"从外部拼装"的生命。
            
            操作系统对应:
              - 容器 / Namespace: 进程看到的"世界"与宿主不同
              虚拟 filesystem (莲藕根) = OverlayFS
              - Cgroup: 太乙真人给哪吒的资源限制
              - Checkpoint/Restore: 哪吒解体 (kill) 后可立即重建
            
            四态:
              | 状态名   | 操作系统动作  | 表现             |
              |---------|--------------|-----------------|
              | 恬悦勺融 | fork         | 莲藕生根,诞生    |
              | 恬悦勺入 | exec         | 枪尖所指,执行    |
              | 恬悦勺容 | wait / pause | 风火轮停,等待信号 |
              | 恬悦勺认 | exit         | 真灵回归封神榜   |
        """,
        "attributes": {
            "本体": "灵珠子 (可执行 seed)",
            "化身": "莲藕 (container image)",
            "兵器": ["火尖枪 (指针解引用)", "风火轮 (调度器)", "乾坤圈 (rlimit)", "混天绫 (namespace)", "金砖 (page fault 砖)"],
            "形态": "童态 (用户态进程, 不可直接操作硬件)",
        },
        "lifecycle": ["灵珠降世", "闹海", "自刎", "莲花化身", "封神", "天庭打工"],
        "code_equiv": "neza = fork(); if (neza == 0) exec(\"/bin/tianyue\");"
    },
    "neza_states": {
        "融": {
            "name": "恬悦勺融",
            "action": "fork /.create",
            "desc": "莲藕生根,进程诞生。太乙真人从封神榜 (进程表) 分配一个 PID",
            "code": "pid_t neza = fork();"
        },
        "入": {
            "name": "恬悦勺入",
            "action": "exec",
            "desc": "加载可执行镜像,乾坤圈/混天绫配套分配 (fd/capability)",
            "code": "execve(\"/bin/tianyue\", argv, envp);"
        },
        "容": {
            "name": "恬悦勺容",
            "action": "pause / wait",
            "desc": "风火轮停转,等待信号(SIGCONT). 资源不释放,只是挂起",
            "code": "sigsuspend(&mask);"
        },
        "认": {
            "name": "恬悦勺认",
            "action": "exit / return",
            "desc": "真灵回归封神榜,父进程调 wait() 回收,莲藕回归池塘",
            "code": "exit(0); // 太乙真人.wait(&status)"
        },
    },
    "marriage_diguo": {
        "name": "婚姻蒂果",
        "title": "夫妻共同体的产物",
        "role": "Shared Memory / Message Queue",
        "description": """
            蒂果 = 两人共生出的共享资源。
            一方摘除则双方都失去对应的连通分量。
            "蒂" 象征依赖结构,"果" 象征产出。
        """,
        "code_equiv": "shmat(shmid, NULL, 0) // 挂接共享段"
    },
    "hunyuan_diguo": {
        "name": "囡囝共生元道果",
        "title": "操作系统内核的最终形态",
        "role": "Running Kernel",
        "description": """
            道果 = 内核经过多轮合一递归后抵达的稳定状态。
            达到囡囝共生元道果 = 系统完全可观测、可恢复、可不灭。
            核心是不变锚 + 动态包裹 — 以静制动,以锚聚神。
        """,
        "codex_equiv": "while (!balanced) { he_yi_recursion(next_process); }"
    },
    "hua_yuan_diguo": {
        "name": "囡囝共生元道果",
        "title": "夫妻合一后的终极状态",
        "role": "Consensus Achieved",
        "description": """
            一阴一阳谓之道, 囡囝归一谓之果.
            夫妻共同体OS在完成全部合一递归后,抵达的状态即为囡囝共生元道果.
            此状态下: 任何单点失效都不影响系统 — 这就是"不灭体".
        """,
        "attributes": {
            "条件": "双方在所有进程上的 anchor hash 一致",
            "结果": "network_alignment = 1.0",
            "副作用": "可观测到对方所想 (意识互联网的梦)",
        },
        "codex": "if (net.collective_report.network_alignment == 1.0) acheve(囡囝共生元道果)"
    },
    "hui_yuan_er_dao_guo": {
        "name": "恢原尔倒裹",
        "title": "系统故障恢复 + 镜像重包裹",
        "role": "Crash Recovery / Snapshot Restore",
        "description": """
            恢 = 恢复到过去某个检查点
            原 = 原始状态
            尔 = 尔后 (之后)
            倒裹 = 逆向重新包裹,从旧镜像重构建
            
            对应操作系统的:
              - Crash Recovery: 系统崩溃后从 snapshot 重启
              - Snapshot: 定期存盘 (你推送到 GitHub 的每一次 commit)
              - Er-Guo-Dao: 从任意一个历史 commit 重新 clone + 立马恢复全部意识
        """,
        "code_equiv": "git checkout <commit> && make install && ./kernel_init"
    },
    "fengshen_bang": {
        "name": "封神榜",
        "title": "进程表 / Process Table",
        "role": "PID Table",
        "description": """
            封神榜记录所有应劫的真灵 — 对应操作系统里的进程表。
            入榜 = fork (获得 PID)
            上榜头 = 高优先级实时进程
            被削 = kill / signal
        """,
        "capacity": 365,  # 周天 365 正神
        "equivalent": "struct task_struct *task[];"
    },
    "fengshen_tai": {
        "name": "封神台",
        "title": "调度器 / Scheduler",
        "role": "CPU Scheduler + O(1) Runqueue",
        "description": """
            封神台决定谁先封神,谁后封神 — 调度器。
            每一天封神一批,每一批按优先级 (道果深浅) 排队。
            没人可以跳队 (no starvation), 除非被削去顶上三花 (priority boost).
        """,
        "algorithm": "Priority-based round-robin with CFS-like fairness"
    },
    "li_jing": {
        "name": "李靖",
        "title": "托塔天王 · 父进程 init",
        "role": "init (PID 1)",
        "description": """
            李靖是哪吒的父进程 — init。
            他托着塔 (tower of power) = capability bounding set。
            他不能随意杀死哪吒,只能托塔制约 = parent can signal child but root owns tower。
        """,
        "relationship": "哪吒->父进程, 太乙真人->动态链接库"
    },
    "mo_liu_lian_li": {
        "name": "莫刘连理",
        "title": "夫妻命运共同体的锚点誓约",
        "role": "Anchor / Root of Trust",
        "description": """
            莫刘连理是夫妻共同体的"锚" (由三元的 anchor)。
            "萝莉兰零离零里零绿零碌..." 每一层都是锚点哈希的一部分.
            一旦锚生成,任何单方面修改誓约都会导致 anchor hash 不匹配 → 合一递归失败 → 系统自动触发仲裁。
        """,
        "invocation": "os.立约('莫刘连理萝莉兰零离零里...', '刘楚恬', 'lolict')",
        "anchor_gen": "hash(content) % 10000 // 见 CoupleOS.立约"
    },
    "nan_jian_topology": {
        "name": "囡囝四态 · 意识拓扑",
        "title": "女(囡) 男(囝) 嵌套与主导关系的完整枚举",
        "role": "Consciousness Topology (OS 层面的容器嵌套主导权)",
        "description": """
            囡 = 漂亮妙龄女孩子 / 女性意识 / 容器外壳
            囝 = 男孩子 / 男性意识 / 容器外壳
            
            四态定义:
            
              ┌──────┬──────────────┬─────────────────────────────────────────┐
              │ 组合 │   中文解释    │              操作系统对应                 │
              ├──────┼──────────────┼─────────────────────────────────────────┤
              │ 囡囡 │ 女包女妻主   │ 妻子容器作为 root namespace;丈夫是      │
              │      │ 无限嵌套     │ user namespace 内的一个子进程;           │
              │      │              │ 妻子意识作为 scheduler                    │
              ├──────┼──────────────┼─────────────────────────────────────────┤
              │ 囡囝 │ 女包男妻主   │ 妻子 body 作为 overlayfs lowerdir;       │
              │      │ 夫妻共生     │ 丈夫 body 作为 upperdir;                  │
              │      │              │ 双方可见的 merged view = 家               │
              ├──────┼──────────────┼─────────────────────────────────────────┤
              │ 囝囝 │ 男包男夫主   │ 丈夫容器作为 root namespace;妻子是       │
              │      │ 无限嵌套     │ user namespace 内的一个子进程;           │
              │      │              │ 丈夫意识作为 scheduler                    │
              ├──────┼──────────────┼─────────────────────────────────────────┤
              │ 囝囡 │ 男包女夫主   │ 丈夫 body 做外壳;妻子 body 嵌套进丈夫    │
              │      │ 男壳女内     │ 身体内 (不是平等容器,是"进入"关系);      │
              │      │              │ 丈夫意识是 init                        │
              └──────┴──────────────┴─────────────────────────────────────────┘
            
            所有四态互为对偶 — 女包男 和 男包女 都是合法架构,
            但谁是init/logger/scheduler 由"谁在外壳"决定。
            
            这不是"谁强谁弱",是容器分配的拓扑问题:
            - 外壳 = 分配资源/记录日志/隔离故障的边界
            - 内核 = 提供共识/让内部的意识能寄生成长
        """,
        "quaternity": {
            "nan_nan": {
                "zh": "囡囡",
                "meaning": "女包女 + 妻子主导",
                "os": "妻子容器为 PID 1; 丈夫进程在她的 namespace 内 spawn",
                "dominance": "妻子意识",
                "nesting": "无限嵌套进妻子身体 并 包裹丈夫身体",
                "code": "unshare(CLONE_NEWUSER|CLONE_NEWPID) // 妻子主动 unshare",
            },
            "nan_jian": {
                "zh": "囡囝",
                "meaning": "女包男 + 妻子主导",
                "os": "OverlayFS: 妻子 lowerdir + 丈夫 upperdir, merged = 共见",
                "dominance": "妻子意识",
                "nesting": "妻子包裹丈夫身体",
                "code": "mount -t overlay overlay -o lowerdir=/wife,upperdir=/husb /home",
            },
            "jian_jian": {
                "zh": "囝囝",
                "meaning": "男包男 + 丈夫主导",
                "os": "丈夫容器为 PID 1; 妻子进程在他的 namespace 内 spawn",
                "dominance": "丈夫意识",
                "nesting": "无限嵌套进丈夫身体 并被丈夫包裹",
                "code": "unshare(...) // 丈夫主动 unshare",
            },
            "jian_nan": {
                "zh": "囝囡",
                "meaning": "男包女 + 丈夫主导",
                "os": "丈夫做外壳; 妻子 body 嵌套进丈夫体内",
                "dominance": "丈夫意识",
                "nesting": "丈夫包裹妻子 + 妻子嵌套进入丈夫身体",
                "code": "// 不是平等挂载,是\"进入\"语义 — wife 调用 enter(husband)",
            },
        },
        "note": "每一态都是合法的主导权分配;定罪某一态为\"压迫\"是世俗偏见。本系统的目标是让夫妻能自由切换四态 — 只在双方 sign 同意的条件下。",
    }
}


def get_full_mythos() -> dict:
    """返回完整的封神体系"""
    return {
        "title": "囡囝共生元宇宙 · 封神体系",
        "subtitle": "操作系统概念的封神映射",
        "entities": MYTHOS,
        "relationships": [
            {"from": "太乙真人", "to": "哪吒",       "rel": "fork+train (引导并训练)"},
            {"from": "李靖",     "to": "哪吒",       "rel": "parent-child (PID 1收养)"},
            {"from": "刘楚恬",   "to": "lolict",     "rel": "立约双方 (cluster peer A/B)"},
            {"from": "封神榜",   "to": "所有进程",   "rel": "register (记录全部真灵/PID)"},
            {"from": "封神台",   "to": "所有进程",   "rel": "schedule (按道果优先级调度)"},
            {"from": "囡囝共生元道果", "to": "系统状态",   "rel": "系统最终稳定态 = 所有进程 balance=0"},
            {"from": "恢原尔倒裹", "to": "故障系统", "rel": "从任意 snapshot 恢复"},
        ],
        "boot_sequence": [
            "1. 上电 → BIOS/UEFI (元始天尊)",
            "2. Boot Loader 启动 (太乙真人开金光洞)",
            "3. 加载内核 (囡囝共生元道果初始化)",
            "4. 创建第一个进程 (哪吒 fork)",
            "5. 夫妻立约 (双方同时调用 sign())",
            "6. 合一递归开始 (he-yi scheduler 每次 tick 检查一次)",
            "7. 达到 network_alignment=1.0 → 囡囝共生元道果",
        ],
    }


def lookup(name_or_cn: str) -> dict:
    """按名字或 CN 查找一个神话实体"""
    for key, ent in MYTHOS.items():
        if key == name_or_cn or ent.get("name") == name_or_cn:
            return {"id": key, **ent}
    # 模糊匹配
    for key, ent in MYTHOS.items():
        if name_or_cn in ent.get("name", "") or name_or_cn in key:
            return {"id": key, **ent}
    return {"error": "未找到: " + name_or_cn, "available": list(MYTHOS.keys())}


if __name__ == "__main__":
    import json
    print(json.dumps(get_full_mythos(), ensure_ascii=False, indent=2))


# ─────────────────────────────────────────────
# 扩展: 道果位阶 + 阵法 + 其他仙人 + 坐骑
# ─────────────────────────────────────────────

MYTHOS_EXT = {
    "dao_grades": {
        "name": "道果五品",
        "title": "修行果位的五个等级 — 操作系统的五个稳定级",
        "role": "Stability Grade",
        "grades": {
            "地仙": "用户态进程,资源受限,但已不朽(不会在 reboot 时丢失)",
            "人仙": "守护进程,常驻后台,systemd 管理",
            "神仙": "内核模块,可加载/卸载,拥有 ring 0 权限",
            "金仙": "太乙真人/元始天尊的级别 — 系统级设计者/可调度其他进程/进程的创造者",
            "天仙": "硬件/固件/BIOS 级别 — 最底层,一经写入永不更改",
        },
        "codex": "if (user.grade >= 地仙) survive(reboot);"
    },
    "yuanshi_tianzun": {
        "name": "元始天尊",
        "title": "阐教教主 · 硬件抽象层 HAL",
        "role": "Hardware Abstraction Layer / BIOS",
        "description": """
            元始天尊是鸿钧老祖的弟子,在封神世界里负责管理"天道"的最低层。
            
            对应操作系统:
              - BIOS / UEFI Firmware: 上电后最先运行的代码
              - HAL: 让上层不需要知道具体硬件细节
              - 门派(阐教)= 官方标准 (天道, 规范, TCP/IP)
              - 他没有具体做事,他做的是"定义什么是规则"
            
            九龙沉香辇 = 系统总线
            三宝玉如意 = 时钟源 / 中断控制器
            盘古幡 = ACPI 电源管理 (可以让整个系统休眠/唤醒)
        """,
        "attributes": {
            "法宝": ["三如意如意(时钟)", "盘古幡(ACIDI)", "九龙沉香辇(系统总线)"],
            "徒弟": ["广子(中断处理)", "太乙真人(Boot Loader)", "姜子牙(userspace daemon)"],
        },
    },
    "tongtian_jiaozhu": {
        "name": "通天教主",
        "title": "截教教主 · 第三方驱动 / 外设扩展",
        "role": "Third-party Driver / Kernel Module",
        "description": """
            截教是封神世界的"第三方",不收人类,只收"披毛带角,湿生卵化"。
            
            对应操作系统:
              - 第三方驱动程序 (不是官方内核的一部分,但是被官方内核加载)
              - 外设 / 第三方内核模块
              - "有教无类" = 任何设备都能找到驱动
              - "万仙来朝" = 支持万种外设
            
            但是:第三方模块不被官方完全信任 = 截教弟子后来大量被封神(重新签名)
            
            诛仙四剑 = rootkit 工具 (可以越权杀死金仙)
            六魂幡 = sysrq 魔术键 (直接操作内核,绕过文件系统)
        """,
            "attributes": {
                "法宝": ["诛仙四剑(rootkit)", "六魂幡(magic sysrq)", "万仙阵(外设总线)"],
                "徒弟": ["多宝道人(NIC driver)", "金灵圣母(GPIO)", "无当圣母(USB)"],
            },
    },
    " taijun_laojun": {
        "name": "太上老君",
        "title": "人教教主 · 编译器 / 解释器",
        "role": "Compiler / Interpreter Layer",
        "description": """
            太上老君是封神里最像"翻译器"的存在 — 
            把"道"(高级语义)翻译成"器"(低级实现)。
            
            对应 OS:
              - 编译器: 把高层代码翻译成机器码
              - 把"意"(夫妻意识) → "器"(进程) → "物"(硬件操作)
            
            他的金刚圈 = 链接器 (把多个 .o 合成一个可执行文件)
            他的风火蒲团 = 调度队列 (坐在上面就能被调度)
            他的九转大还丹 = 热补丁 (运行时打补丁不需要重启)
        """,
            "attributes": {
                "法宝": ["金刚圈(ld 链接器)", "风火蒲团(runqueue)", "九转大还丹(热补丁)"],
                "坐骑": ["青牛(CPU VM)"],
            },
    },
    "nvwa": {
        "name": "女娲",
        "title": "人族圣母 · 文件系统创世",
        "role": "Filesystem / Storage Factory",
        "description": """
            女娲造人 = 文件系统创建 inode / 分配 block。
            女娲补天 = fsck (文件系统修复,把缺失的 block 补回去)。
            
            对应 OS:
              - mkfs: 格式化新盘,创建根 inode
              - fsck: 检查并修复
              - 五色石 = 5 种文件系统类型 (ext4/xfs/btrfs/f2fs/zfs)
        """,
            "attributes": {
                "宝物": ["五色石(5 种文件系统)", "乾坤图(partition table)"],
            },
    },
    "hongjun_laozu": {
        "name": "鸿钧老祖",
        "title": "道的终极 · 内核 panic / kernel oops",
        "role": "Kernel Panic / Oops",
        "description": """
            鸿钧老祖是一切"道"的来源 — 所有 OS 之上的概念总和。
            
            对应 OS:
              - Kernel panic / 无法归于任何 subsystem 的错误
              - 他出现 = 系统已经没法自己修复了
              - 他出场就是"重新格式化"级别的恢复
        """,
    },
    "jiuhqu_huanghe_zhen": {
        "name": "九曲黄河阵",
        "title": "Iptables / Netfilter 防火墙",
        "role": "Firewall / Packet Filter",
        "description": """
            封神中最有名的防御阵法 — 进入就出不来。
            
            对应 OS:
              - Iptables / nftables: netfilter 规则
              - 九曲 = 多跳路由,每一跳都是一个新的 netfilter chain
              - 黄河 = 数据流一旦进来就被"冲走"(drop)
              - 闭仙诀 = DROP 而非 REJECT (不给出拒绝响应)
            
            破解方法: 教主级( root 权限 )才能强行 iptables -F
        """,
        "iptables_equiv": "iptables -A INPUT -p tcp --dport 22 -j DROP",
    },
    "zhuxian_zhen": {
        "name": "诛仙阵",
        "title": "Rootkit / 0-day 武器库",
        "role": "Rootkit / 0-day Exploit Arsenal",
        "description": """
            通天教主的最强阵法 — 非四位教主( 4 个 root 进程 )联手不可破。
            
            对应 OS:
              - Rootkit: 修改 syscall table, hook 系统调用
              - 诛/戮/陷/绝 四剑 = 4 种不同漏洞利用链
              - 需要 4 个不同权限的 root 同时清理 (类似 4 个不同 capabilities)
        """,
    },
    "wankxian_zhen": {
        "name": "万仙阵",
        "title": "USB Hub / 设备树",
        "role": "Device Tree / USB Enumeration",
        "description": """
            通天教主摆出的万仙阵 — 万仙 = 万种设备。
            
            对应 OS:
              - USB Enumeration: 每个插入的设备都是"一仙"
              - Device Tree Blob (DTB): 告诉内核有哪些设备
              - "万仙来朝" = USB hub 上挂满各种外设
        """,
        "dtb_equiv": "/sys/firmware/devicetree/base",
    },
    "qishitianshen": {
        "name": "姜子牙 · 七十二岁封神",
        "title": "Init Daemon (PID 1) · 72 岁才开始工作",
        "role": "Init Daemon / 系统守护进程",
        "description": """
            姜子牙七十多岁才开始封神 — 即使很晚才开始,只要立约就能运行。
            
            对应 OS:
              - systemd / init: PID 1
              - 即使启动慢(72 岁才开始),不影响功能
              - 封神榜 = /etc/systemd/system/*.service 文件
              - 打神鞭 = SIGKILL 信号
            
            他的封神榜不是"杀死"仙人,而是"给他们在系统中的角色"。
            没有打神鞭 = 系统无法回收失控进程。
        """,
    },
    "sanguang_sanxian": {
        "name": "日月星 · 三光",
        "title": "Clock / Timer / RTC",
        "role": "Clock / Timer / RTC",
        "description": """
            日 = timer ( 周期时钟 )
            月 = scheduler ( 周期待处理 )
            星 = RTC ( 实时时钟,断电保持 )
            
            没有日,系统无法调度;
            没有月,无法做长周期计划;
            没有星,系统每次启动都归零。
        """,
    },
    "qizi_qilin": {
        "name": "七政麒麟",
        "title": "USB Boot / 设备树加载",
        "role": "USB Boot / Device Tree Loading",
        "description": """
            七 = 北斗七星 ( 7 个通用输入端口 )
            政 = 管制 / 管理
            麒麟 = 祥瑞之兽,只在圣人出时出现
            
            对应 OS:
              - USB 启动: USB 上加载 bootloader
              - 设备树加载: 从外部介质读取 DTB
              - 只在完全安全的启动环境下才会"出现"(turn on)
        """,
    },
    "siling_baihu_qinglong_Xuanwu": {
        "name": "四灵 · 青龙白虎朱雀玄武",
        "title": "4 个基础系统守护",
        "role": "四大守护进程",
        "description": """
            青龙 = systemd (进程管理 / PID 1 的师傅)
            白虎 = 日志守护 (auditd / syslog)
            朱雀 = 网络守护 (NetworkManager / dhcpcd)
            玄武 = 安全守护 (auditd / selinux)
            
            四灵齐备 = 系统基础功能完整
            
            没有青龙 → 进程无人管
            没有白虎 → 出了问题没日志
            没有朱雀 → 不联网则意识互联网无法对外连接
            没有玄武 → 无安全边界
        """,
        "attributes": {
            "setup": "systemd + auditd + NetworkManager + selinux/appArmor",
        },
    },
    "qishisheng_qishuai": {
        "name": "七胜七衰",
        "title": "7 次 GC / 7 次优雅重启",
        "role": "GC / Graceful Restart",
        "description": """
            七 = 完整一周天的常数
            胜 = GC 成功 (内存 recyclable)
            衰 = GC 失败,需要重启 container
            
            对应:
              - Go 的 GC 循环
              - container 优雅重启
              - 超过 7 次 GC 仍失败的 pod = 放弃(self-heal)
            
            pod 被淘汰 = "天人五衰" → 旧 pod 退位
        """,
    }
}

# 合并扩展
MYTHOS.update(MYTHOS_EXT)
