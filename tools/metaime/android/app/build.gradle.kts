// MetaIME · app/build.gradle.kts · 零第三方依赖

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.mqf.metaime"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.mqf.metaime"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.0-mlll"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            // proguard 此处留空，满全法应用无代码混淆需求
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    externalNativeBuild {
        cmake {
            path = file("src/main/cpp/CMakeLists.txt")
        }
    }

    ndk {
        // 三架构全支持 (盟机场多样化)
        abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
    }
}

dependencies {
    // 仅 AndroidX 系统库 (自主依赖)
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("androidx.core:core-ktx:1.12.0")
    // 无第三方依赖 · 智能全部在编译时常量
}
