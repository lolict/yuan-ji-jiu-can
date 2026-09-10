// ============================================================
// MetaIME JNI 胶水层 (C/C++ → Rust 桥接)
//
// 生成: arm64-v8a / armeabi-v7a / x86_64 三架构 .so
// 调用约定: Rust 核心提供 C-ABI 函数
// ============================================================

#include <jni.h>
#include <string.h>
#include <stdlib.h>

// 这些函数在 libmetaime_core.so 中由 Rust 定义
extern void* metaime_create();
extern void metaime_destroy(void* handle);
extern const char* metaime_push_key(void* handle, int key_code);
extern const char* metaime_reset(void* handle);
extern const char* metaime_switch_mode(void* handle, const char* mode);
extern const char* metaime_get_stats(void* handle);

// JNI 引用缓存
static JavaVM* g_jvm = NULL;
static void* g_engine = NULL;

// 获取 JNIEnv 指针
static JNIEnv* get_env() {
    JNIEnv* env = NULL;
    if (g_jvm) {
        (*g_jvm)->GetEnv(g_jvm, (void**)&env, JNI_VERSION_1_6);
    }
    return env;
}

// 渲染缓存 (避免跨平台传输复杂结构)
static char result_buf[1024];

// ============================================================
// JNI 导出函数 (package com.mqf.metaime.jni.MetaIMEJNI)
// ============================================================

JNIEXPORT jlong JNICALL
Java_com_mqf_metaime_jni_MetaIMEJNI_nativeInit(JNIEnv* env, jobject thiz) {
    g_engine = metaime_create();
    return (jlong)(intptr_t)g_engine;
}

JNIEXPORT jstring JNICALL
Java_com_mqf_metaime_jni_MetaIMEJNI_nativePushKey(JNIEnv* env, jobject thiz, jint key_code) {
    if (!g_engine) return (*env)->NewStringUTF(env, "{\"intent\":\"CLEAR\",\"payload\":\"\"}");
    const char* raw = metaime_push_key(g_engine, (int)key_code);
    if (!raw) return NULL;
    strncpy(result_buf, raw, sizeof(result_buf) - 1);
    result_buf[sizeof(result_buf) - 1] = '\0';
    return (*env)->NewStringUTF(env, result_buf);
}

JNIEXPORT jboolean JNICALL
Java_com_mqf_metaime_jni_MetaIMEJNI_nativeReset(JNIEnv* env, jobject thiz) {
    if (!g_engine) return JNI_FALSE;
    const char* r = metaime_reset(g_engine);
    return (r != NULL) ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_com_mqf_metaime_jni_MetaIMEJNI_nativeSwitchMode(JNIEnv* env, jobject thiz, jstring mode) {
    if (!g_engine) return JNI_FALSE;
    const char* m = (*env)->GetStringUTFChars(env, mode, NULL);
    const char* r = metaime_switch_mode(g_engine, m);
    (*env)->ReleaseStringUTFChars(env, mode, m);
    return (r != NULL) ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT void JNICALL
Java_com_mqf_metaime_jni_MetaIMEJNI_nativeDestroy(JNIEnv* env, jobject thiz) {
    if (g_engine) {
        metaime_destroy(g_engine);
        g_engine = NULL;
    }
}

JNIEXPORT jstring JNICALL
Java_com_mqf_metaime_jni_MetaIMEJNI_nativeGetStats(JNIEnv* env, jobject thiz) {
    if (!g_engine) return (*env)->NewStringUTF(env, "{}");
    return (*env)->NewStringUTF(env, metaime_get_stats(g_engine));
}

// JNI_OnLoad 入口
JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void* reserved) {
    g_jvm = vm;
    return JNI_VERSION_1_6;
}
