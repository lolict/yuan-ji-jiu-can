/**
 * 民联浏览器壳 · 移动端统一桥 v1.10
 * ==================================
 * 自动检测 Android WebView / iOS WKWebView / 普通浏览器
 * 提供统一的 window.njsgNative 接口
 *
 * 使用方式:
 *   const history = await njsgNative.load('verdict_history');
 *   await njsgNative.save('verdict_history', JSON.stringify(data));
 *   const info = await njsgNative.getPlatform();
 */

'use strict';

(function () {
  // 已存在则不覆盖 (原生壳可能已经注入)
  if (window.njsgNative) return;

  // 检测环境
  const isAndroid = /Android/i.test(navigator.userAgent) &&
                    typeof window.njsgNative !== 'undefined';
  const isIOS = /iPhone|iPad|iPod/i.test(navigator.userAgent) &&
                window.webkit && window.webkit.messageHandlers;

  // 优先使用原生桥,回退到 localStorage
  if (isAndroid) {
    // Android 的 njsgNative 已在 WebView 注入,直接使用
    window.njsgShellEnv = { platform: 'android', bridge: 'native' };
    return;
  }

  if (isIOS) {
    // iOS 已在 WKUserScript 中注入,直接使用
    window.njsgShellEnv = { platform: 'ios', bridge: 'native' };
    return;
  }

  // 浏览器回退:用 localStorage 模拟 native 持久化
  window.njsgNative = {
    save: function (key, value) {
      try {
        localStorage.setItem('njsg_' + key, value);
        return true;
      } catch (e) { return false; }
    },
    load: function (key) {
      return localStorage.getItem('njsg_' + key);
    },
    clear: function (key) {
      if (key) localStorage.removeItem('njsg_' + key);
      else {
        Object.keys(localStorage)
          .filter(k => k.startsWith('njsg_'))
          .forEach(k => localStorage.removeItem(k));
      }
      return true;
    },
    getPlatform: function () {
      return JSON.stringify({
        platform: 'browser',
        userAgent: navigator.userAgent,
        language: navigator.language,
        bridge: 'localStorage'
      });
    }
  };

  window.njsgShellEnv = { platform: 'browser', bridge: 'localStorage' };
})();
