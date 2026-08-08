# Shell 与 NJSG hunyuan-methodology 的关系

> 位置: hunyuan-os/shell/ (native + browser)

---

## 定位

shell 是 hunyuan-os 的外壳层。它不是独立系统，是 NJSG 操作系统的交互界面。

## 流模型映射

shell 的命令执行过程 = 八流实例化：

```
用户键入命令 → Input
│
├─ Perception    : shell 读取字符流
├─ Distinguish   : 分别器三层判断
│   ├─ 感知家族: 命令属于哪个家族？(alias / builtin / 外部程序)
│   ├─ 谐音靶点: 命令缩写指向哪个靶点？(lctqf → Wife 路径)
│   └─ 主权路由: 命令执行权限级别？(Active=root / Passive=user)
├─ Lookup        : 查 PATH 字典（= 预加载流字典）
├─ Traverse      : 解析命令参数（可能触发解释流牺牲）
├─ Cluster       : 管道链接 = 聚类
├─ Attention     : 优先级调度（foreground vs background）
└─ Arrival       : 命令结果输出（ack = exit code 0）
```

## 命名规则

shell 脚本中的变量/函数名遵循家庭命名密码表：
- `mlll_*` 相关操作 = 容器操作
- `gtt_*` 相关操作 = 态射转换
- `mlnn_*` 相关操作 = 夫妻拓扑操作

禁止负面谐音缩写（mlqf 等）。

---

*配套: hunyuan-methodology SKILL.md*
