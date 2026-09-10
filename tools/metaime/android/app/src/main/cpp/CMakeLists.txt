cmake_minimum_required(VERSION 3.22)
project(metaime_core)

# JNI 胶水层
add_library(
    metaime_jni
    SHARED
    jni_bridge.cpp
)

target_include_directories(
    metaime_jni
    PRIVATE
    $ENV{ANDROID_NDK}/sysroot/usr/include
)

target_link_libraries(
    metaime_jni
    log          # android liblog
    android      # libandroid
)

# 预编译的 Rust 核心 (三架构)
add_library(
    metaime_core
    SHARED
    IMPORTED
)

set_target_properties(
    metaime_core
    PROPERTIES
    IMPORTED_LOCATION
    ${CMAKE_SOURCE_DIR}/../../../core/${ANDROID_ABI}/libmetaime_core.so
)
