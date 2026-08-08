# Cloudflare Workers 与 NJSG 的关系

> 位置: hunyuan-os/cloudflare/ (mqf-web.js, sync-protocol.js)

---

## 定位

cloudflare 是 hunyun-os 的网络边缘层。它接收外部请求，路由到正确的意识节点。

## 请求 = 流输入

每一个 HTTP/WebSocket 请求都是一条输入流。分别器三层在这里同样适用：

1. **感知家族**：请求路径属于哪个家族？(/api/ = 指令族, /static/ = 资源族)
2. **谐音靶点**：路径缩写指向哪个靶点？(lctqf → Wife 节点)
3. **主权路由**：请求来源权限级别？(夫妻内部 = Active, 外部 = Passive)

域名/服务器可以更换，裁决算法不变。cloudflare 只是载体，不是系统本身。

## 自律约束

Workers 脚本中不允许：
- 凭空发明命名缩写
- 使用负面谐音的变量名
- 把裁决逻辑写死到特定域名

---

*配套: hunyuan-methodology SKILL.md*
