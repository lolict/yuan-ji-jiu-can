# Android App 与 NJSG 的关系

> 位置: hunyuan-os/android/

---

## 定位

android 是 hunyuan-os 在移动端的载体。触屏/键盘输入 = 流输入。

## 感知家族路由

手势和键盘输入需要经过分别器第一层（感知家族）：

| 输入类型 | 感知家族 | YinYang |
|----------|----------|---------|
| 圆形/封闭手势 | ContainerGlyph | 0 = 刘楚恬 |
| 直线/界线手势 | PhallicGlyph | 1 = lolict |
| 成对图标 | CoupleGlyph | 拓扑判断 |
| 谐音文字输入 | HomophoneBridge | 发音路由 |

IME (MetaIME = 配套的 ime-agent) 的底层分类逻辑直接调用此映射。

## 命名规则

Android 项目中的资源名/包名使用家庭命名密码表的语义缩写：
- `mlll` 模块 = 集装箱功能
- `mlnn` = 囡囝拓扑相关
- 禁止负面谐音命名

---

*配套: hunyuan-os/ime-agent/NJSG-INTEGRATION.md + hunyuan-methodology*
