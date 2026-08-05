/**
 * 民联浏览器壳 · Electron Preload v1.7
 * ====================================
 * 安全隔离层 — 仅限基础 IPC
 */
const { contextBridge } = require('electron');

contextBridge.exposeInMainWorld('njsgShell', {
  version: '1.7.0',
  platform: process.platform,
  // 未来可扩展文件系统/网络代理
});
