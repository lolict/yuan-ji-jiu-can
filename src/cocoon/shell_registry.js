/**
 * 蛹茧寄生壳注册表
 * ============================================================
 * 本文件预注册了所有平台可用的"寄生壳"配置。
 *
 * 设计哲学：
 *   蛹茧（.cocoon）本身只是内核 WASM 字节码 + 元数据，
 *   它在浏览器 WASM 沙箱内被解析后，需要"穿上"一个与目标平台
 *   匹配的寄生壳，才能蜕变为该平台的可执行程序。
 *
 *   寄生壳不修改宿主系统、不需要系统权限授权——
 *   它通过 WASI 接口与宿主原生能力对接，实现"零安装运行"。
 *
 * 每个壳的配置字段：
 *   id              - 壳的唯一标识符
 *   name            - 显示名称（中文）
 *   platform        - 平台类别（mobile / desktop / embedded / medical / industrial / xr / io 等）
 *   icon            - 图标标识（emoji 或符号）
 *   description     - 壳的描述说明
 *   permissions     - 该壳需要向 WASM 沙箱声明的权限清单
 *   executableFormat- 穿上壳后生成的可执行文件格式
 *   launchCommand   - 在目标平台上的启动命令模板
 *   wasmRuntime     - 该壳使用的 WASM 运行时实现
 */

/**
 * ShellRegistry - 寄生壳注册系统
 * 负责注册、查询、列举所有可用寄生壳，并自动检测当前平台
 */
class ShellRegistry {
    constructor() {
        // 内部壳配置表：id -> shellConfig
        this._shells = new Map();
        // 平台检测优先级表：平台特征关键字 -> 推荐壳 id
        this._platformHints = [
            { test: /Win32|Windows/i, shellId: 'pc_windows' },
            { test: /Macintosh|Mac OS X|MacIntel/i, shellId: 'pc_mac' },
            { test: /Linux|X11/i, shellId: 'pc_linux' },
            { test: /iPhone|iPad|iPod/i, shellId: 'iphone' },
            { test: /Android/i, shellId: 'android' },
            { test: /HarmonyOS|OHOS|ArkWeb/i, shellId: 'harmony' },
        ];
    }

    /**
     * 注册一个寄生壳
     * @param {string} shellId - 壳唯一标识
     * @param {object} shellConfig - 壳配置对象
     * @returns {boolean} 注册是否成功
     */
    register(shellId, shellConfig) {
        if (!shellId || typeof shellId !== 'string') {
            throw new Error('[ShellRegistry] shellId 必须是非空字符串');
        }
        if (!shellConfig || typeof shellConfig !== 'object') {
            throw new Error('[ShellRegistry] shellConfig 必须是对象');
        }
        // 校验必需字段
        const required = ['name', 'platform', 'executableFormat'];
        for (const field of required) {
            if (!(field in shellConfig)) {
                throw new Error(`[ShellRegistry] 壳 "${shellId}" 缺少必需字段: ${field}`);
            }
        }
        // 补全默认值
        const config = {
            id: shellId,
            name: shellConfig.name,
            platform: shellConfig.platform,
            icon: shellConfig.icon || '🥚',
            description: shellConfig.description || '',
            permissions: Array.isArray(shellConfig.permissions) ? shellConfig.permissions : [],
            executableFormat: shellConfig.executableFormat,
            launchCommand: shellConfig.launchCommand || '',
            wasmRuntime: shellConfig.wasmRuntime || 'wasmtime',
        };
        this._shells.set(shellId, config);
        return true;
    }

    /**
     * 批量注册多个寄生壳
     * @param {object} shells - { id: config, ... }
     */
    registerAll(shells) {
        for (const [id, config] of Object.entries(shells)) {
            this.register(id, config);
        }
    }

    /**
     * 获取指定壳配置
     * @param {string} shellId
     * @returns {object|null} 壳配置，不存在返回 null
     */
    get(shellId) {
        return this._shells.get(shellId) || null;
    }

    /**
     * 列出所有已注册壳
     * @returns {object[]} 壳配置数组
     */
    listAll() {
        return Array.from(this._shells.values());
    }

    /**
     * 列出所有壳的精简信息（id + name + platform），用于弹窗展示
     * @returns {object[]}
     */
    listSummary() {
        return this.listAll().map(s => ({
            id: s.id,
            name: s.name,
            platform: s.platform,
            icon: s.icon,
            description: s.description,
        }));
    }

    /**
     * 按平台类别筛选壳
     * @param {string} platform
     * @returns {object[]}
     */
    filterByPlatform(platform) {
        return this.listAll().filter(s => s.platform === platform);
    }

    /**
     * 自动检测当前运行平台并推荐一个壳
     * 优先使用 navigator.userAgent / platform，回退到 Node.js 的 process.platform
     * @returns {object|null} 推荐的壳配置
     */
    detectPlatform() {
        let uaString = '';
        try {
            // 浏览器环境
            if (typeof navigator !== 'undefined') {
                uaString = navigator.userAgent || navigator.platform || '';
            }
        } catch (_) { /* 非浏览器环境忽略 */ }

        // Node.js 环境回退检测
        if (!uaString && typeof process !== 'undefined' && process.platform) {
            const p = process.platform;
            if (p === 'win32') return this.get('pc_windows');
            if (p === 'darwin') return this.get('pc_mac');
            if (p === 'linux') return this.get('pc_linux');
        }

        for (const hint of this._platformHints) {
            if (hint.test.test(uaString)) {
                const found = this.get(hint.shellId);
                if (found) return found;
            }
        }
        // 无法识别平台时，回退到通用的桌面壳
        return this.get('pc_windows');
    }

    /**
     * 清空所有已注册壳（主要用于测试）
     */
    clear() {
        this._shells.clear();
    }
}

// ============================================================
// 预注册的全部平台寄生壳配置
// 每个壳描述了它"穿上"后能蜕变成的目标平台可执行程序。
// ============================================================
const SHELL_REGISTRY = {
    // ---------- 手机平台 ----------
    harmony: {
        name: '鸿蒙蛹茧壳',
        platform: 'mobile',
        icon: '🌸',
        description: '适配华为鸿蒙 HarmonyOS / OpenHarmony 的寄生壳，穿壳后蜕变为 ArkTS/方舟可执行程序，支持原子化服务与跨设备流转。',
        permissions: ['network', 'file_storage', 'device_info', 'distributed_schedule', 'arkui', 'ace_engine', 'soft_bus'],
        executableFormat: '.hap (Harmony Ability Package)',
        launchCommand: 'aa start -a EntryAbility -b {bundle}',
        wasmRuntime: 'ark_wasm_runtime',
    },
    android: {
        name: '安卓蛹茧壳',
        platform: 'mobile',
        icon: '🤖',
        description: '适配 Android 系统的寄生壳，穿壳后蜕变为 APK，通过 WASM Runtime 运行内核，无需 root 与系统授权。',
        permissions: ['network', 'file_storage', 'device_info', 'camera', 'location', 'bluetooth', 'nfc', 'sensor'],
        executableFormat: '.apk (Android Package)',
        launchCommand: 'am start -n {package}/.MainActivity',
        wasmRuntime: 'wamr (WebAssembly Micro Runtime)',
    },
    iphone: {
        name: '苹果蛹茧壳',
        platform: 'mobile',
        icon: '🍎',
        description: '适配 iOS 系统的寄生壳，穿壳后蜕变为 App，通过 Safari/WebKit WASM 引擎运行，遵循沙箱与 App Store 规范。',
        permissions: ['network', 'file_storage', 'device_info', 'camera', 'photo_library', 'core_motion', 'metal'],
        executableFormat: '.ipa (iOS App Archive)',
        launchCommand: 'open {bundle-id}',
        wasmRuntime: 'webkit_wasm_jsc',
    },
    domestic_phone: {
        name: '国产手机蛹茧壳',
        platform: 'mobile',
        icon: '📱',
        description: '适配国产手机生态（澎湃/ColorOS/OriginOS/MagicOS 等）的通用寄生壳，统一对接各家厂商的扩展能力。',
        permissions: ['network', 'file_storage', 'device_info', 'camera', 'location', 'push_service', 'vendor_bridge'],
        executableFormat: '.apk (Android Package, 厂商签名)',
        launchCommand: 'am start -n {package}/.Main',
        wasmRuntime: 'wamr (WebAssembly Micro Runtime)',
    },

    // ---------- 电脑平台 ----------
    pc_windows: {
        name: '电脑Windows蛹茧壳',
        platform: 'desktop',
        icon: '🪟',
        description: '适配 Windows 桌面系统的寄生壳，穿壳后蜕变为 .exe 可执行程序，通过 WASI 对接 Win32 API。',
        permissions: ['network', 'file_storage', 'process_spawn', 'registry_read', 'gpu_directx', 'audio'],
        executableFormat: '.exe (PE32+ Executable)',
        launchCommand: '{name}.exe',
        wasmRuntime: 'wasmtime',
    },
    pc_linux: {
        name: '电脑Linux蛹茧壳',
        platform: 'desktop',
        icon: '🐧',
        description: '适配 Linux 桌面/服务器系统的寄生壳，穿壳后蜕变为 ELF 可执行文件，遵循 POSIX/WASI 标准。',
        permissions: ['network', 'file_storage', 'process_spawn', 'ipc_socket', 'gpu_opengl', 'audio_alsa'],
        executableFormat: 'ELF (Executable and Linkable Format)',
        launchCommand: './{name}',
        wasmRuntime: 'wasmtime',
    },
    pc_mac: {
        name: '电脑Mac蛹茧壳',
        platform: 'desktop',
        icon: '💻',
        description: '适配 macOS 系统的寄生壳，穿壳后蜕变为 .app 应用包，通过 Metal 与 CoreFoundation 对接系统能力。',
        permissions: ['network', 'file_storage', 'process_spawn', 'metal', 'core_audio', 'app_sandbox'],
        executableFormat: '.app (macOS Application Bundle)',
        launchCommand: 'open {name}.app',
        wasmRuntime: 'wasmtime',
    },

    // ---------- 嵌入式与航天 ----------
    drone: {
        name: '无人机蛹茧壳',
        platform: 'embedded',
        icon: '🚁',
        description: '适配无人机飞控系统的寄生壳，穿壳后蜕变为飞控可执行固件，对接 MAVLink 协议与姿态控制环。',
        permissions: ['flight_control', 'imu_sensor', 'gps', 'camera', 'radio_link', 'motor_pwm', 'mavlink'],
        executableFormat: '.fcfw (Flight Control Firmware)',
        launchCommand: 'mavlink start --module {name}',
        wasmRuntime: 'wamr (WebAssembly Micro Runtime)',
    },
    satellite: {
        name: '卫星系统蛹茧壳',
        platform: 'aerospace',
        icon: '🛰️',
        description: '适配在轨卫星星载计算机的寄生壳，穿壳后蜕变为星载任务可执行程序，对接 CCSDS 与遥测遥控链路。',
        permissions: ['telemetry', 'telecommand', 'orbit_data', 'star_tracker', 'rf_downlink', 'power_management', 'radiation_tolerant'],
        executableFormat: '.sat (Satellite Onboard Executable)',
        launchCommand: 'obc exec {name}.sat',
        wasmRuntime: 'wamr_aerospace',
    },

    // ---------- 医疗设备 ----------
    bci: {
        name: '脑机接口蛹茧壳',
        platform: 'medical',
        icon: '🧠',
        description: '适配脑机接口设备的寄生壳，穿壳后蜕变为神经信号采集与解码程序，对接 EEG/fNIRS 信号流。',
        permissions: ['neural_signal', 'eeg_stream', 'stimulation', 'bio_feedback', 'low_latency', 'patient_safety'],
        executableFormat: '.bcimod (BCI Module)',
        launchCommand: 'bci-daemon load {name}.bcimod',
        wasmRuntime: 'wamr_realtime',
    },
    mri: {
        name: '核磁共振蛹茧壳',
        platform: 'medical',
        icon: '🧲',
        description: '适配核磁共振成像设备的寄生壳，穿壳后蜕变为成像序列控制与重建程序，对接 K 空间数据流。',
        permissions: ['gradient_control', 'rf_pulse', 'k_space_io', 'image_reconstruct', 'safety_monitor', 'dicom'],
        executableFormat: '.mrisq (MRI Sequence)',
        launchCommand: 'mri-console run {name}.mrisq',
        wasmRuntime: 'wasmtime_medical',
    },
    xray: {
        name: 'X光技术蛹茧壳',
        platform: 'medical',
        icon: '☢️',
        description: '适配 X 光成像设备的寄生壳，穿壳后蜕变为曝光控制与图像处理程序，对接平板探测器数据。',
        permissions: ['xray_tube', 'flat_panel_detector', 'dose_control', 'image_processing', 'dicom', 'safety_interlock'],
        executableFormat: '.xrayapp (X-Ray Application)',
        launchCommand: 'xray-ctrl start {name}.xrayapp',
        wasmRuntime: 'wasmtime_medical',
    },

    // ---------- 工业与成像 ----------
    camera360: {
        name: '360度相机蛹茧壳',
        platform: 'industrial',
        icon: '🎥',
        description: '适配 360 度全景相机的寄生壳，穿壳后蜕变为全景采集与拼接程序，对接多目视频流。',
        permissions: ['multi_camera', 'video_stream', 'image_stitch', 'imu_gyro', 'storage_highspeed', 'network'],
        executableFormat: '.camapp (Camera Application)',
        launchCommand: 'cam-360 run {name}.camapp',
        wasmRuntime: 'wamr',
    },
    laser_scan: {
        name: '激光扫描蛹茧壳',
        platform: 'industrial',
        icon: '🔭',
        description: '适配激光三维扫描仪的寄生壳，穿壳后蜕变为点云采集与建模程序，对接激光测距与转台控制。',
        permissions: ['laser_rangefinder', 'galvo_control', 'point_cloud', 'calibration', 'precision_motion', 'data_export'],
        executableFormat: '.scanapp (Scan Application)',
        launchCommand: 'laser-scan exec {name}.scanapp',
        wasmRuntime: 'wamr',
    },
    projection3d: {
        name: '3D投影蛹茧壳',
        platform: 'industrial',
        icon: '📽️',
        description: '适配 3D 投影仪/光场显示设备的寄生壳，穿壳后蜕变为投影校正与光场渲染程序。',
        permissions: ['projector_control', 'light_field_render', 'geometry_calibration', 'color_management', 'gpu'],
        executableFormat: '.projapp (Projection Application)',
        launchCommand: 'proj-3d render {name}.projapp',
        wasmRuntime: 'wasmtime',
    },
    printer3d: {
        name: '3D打印蛹茧壳',
        platform: 'industrial',
        icon: '🖨️',
        description: '适配 3D 打印机的寄生壳，穿壳后蜕变为切片与运动控制程序，对接 G-code 与打印头/热床。',
        permissions: ['gcode_stream', 'extruder_control', 'hotbed_temp', 'motion_axis', 'model_slice', 'job_queue'],
        executableFormat: '.printapp (Print Application)',
        launchCommand: '3d-print run {name}.printapp',
        wasmRuntime: 'wamr',
    },

    // ---------- 通信与金融 ----------
    telecom: {
        name: '通信系统蛹茧壳',
        platform: 'telecom',
        icon: '📡',
        description: '适配通信核心网/基站的寄生壳，穿壳后蜕变为信令与协议处理程序，对接 5G/6G 协议栈。',
        permissions: ['radio_unit', 'baseband_stream', 'signaling_stack', 'spectrum', 'core_network', 'qos'],
        executableFormat: '.telmod (Telecom Module)',
        launchCommand: 'gNB load-module {name}.telmod',
        wasmRuntime: 'wamr_realtime',
    },
    banking: {
        name: '银行系统蛹茧壳',
        platform: 'finance',
        icon: '🏦',
        description: '适配银行核心系统的寄生壳，穿壳后蜕变为交易处理程序，对接加密机与账务系统，符合金融级安全合规。',
        permissions: ['hsm_crypto', 'ledger_io', 'transaction', 'audit_log', 'secure_channel', 'compliance'],
        executableFormat: '.bkapp (Banking Application)',
        launchCommand: 'core-bank deploy {name}.bkapp',
        wasmRuntime: 'wasmtime_secure',
    },
    surgical_robot: {
        name: '手术机器人蛹茧壳',
        platform: 'medical',
        icon: '🦾',
        description: '适配手术机器人控制系统的寄生壳，穿壳后蜕变为手术规划与机械臂控制程序，具备力反馈与安全联锁。',
        permissions: ['robotic_arm', 'force_feedback', 'vision_tracking', 'safety_interlock', 'surgical_plan', 'sterile_field'],
        executableFormat: '.surgapp (Surgical Application)',
        launchCommand: 'surg-console load {name}.surgapp',
        wasmRuntime: 'wamr_realtime',
    },

    // ---------- XR 与元宇宙 ----------
    metaverse: {
        name: '元宇宙蛹茧壳',
        platform: 'xr',
        icon: '🌐',
        description: '适配元宇宙平台的寄生壳，穿壳后蜕变为虚拟世界运行体，对接数字身份、经济系统与跨域互操作。',
        permissions: ['avatar_render', 'spatial_audio', 'digital_identity', 'virtual_economy', 'cross_realm', 'gpu'],
        executableFormat: '.metaapp (Metaverse Application)',
        launchCommand: 'meta-engine spawn {name}.metaapp',
        wasmRuntime: 'wasmtime',
    },
    ar_vr: {
        name: '虚拟增强现实蛹茧壳',
        platform: 'xr',
        icon: '🥽',
        description: '适配 AR/VR 头显的寄生壳，穿壳后蜕变为沉浸式应用，对接 SLAM 定位、眼动追踪与手部交互。',
        permissions: ['slam_tracking', 'eye_tracking', 'hand_tracking', 'spatial_mapping', 'gpu_vulkan', 'haptics'],
        executableFormat: '.xrapp (XR Application)',
        launchCommand: 'xr-runtime launch {name}.xrapp',
        wasmRuntime: 'wasmtime',
    },

    // ---------- 科学仪器 ----------
    telescope: {
        name: '望远镜蛹茧壳',
        platform: 'scientific',
        icon: '🌠',
        description: '适配天文望远镜的寄生壳，穿壳后蜕变为观测控制与数据采集程序，对接赤道仪、CCD 与导星系统。',
        permissions: ['mount_goto', 'ccd_capture', 'autoguider', 'filter_wheel', 'focuser', 'fits_io'],
        executableFormat: '.telapp (Telescope Application)',
        launchCommand: 'scope-ctrl run {name}.telapp',
        wasmRuntime: 'wasmtime',
    },
    microscope: {
        name: '显微镜蛹茧壳',
        platform: 'scientific',
        icon: '🔬',
        description: '适配电子/光学显微镜的寄生壳，穿壳后蜕变为成像与分析程序，对接载物台、探测器与图像分析管线。',
        permissions: ['stage_control', 'detector_stream', 'image_acquisition', 'focus_auto', 'analysis_pipeline', 'tiff_io'],
        executableFormat: '.micapp (Microscope Application)',
        launchCommand: 'micro-ctrl run {name}.micapp',
        wasmRuntime: 'wasmtime',
    },

    // ---------- 具身智能平台 ----------
    embodied: {
        name: '具身智能蛹茧壳',
        platform: 'robotics',
        icon: '🤖',
        description: '适配具身智能（Embodied AI）实体机器人平台的寄生壳，穿壳后蜕变为具身智能体控制与决策程序，对接全身运动控制、视觉-语言-动作多模态感知、任务规划与强化学习训练流。',
        permissions: ['whole_body_control', 'multimodal_perception', 'vla_model', 'motion_planner', 'task_planner', 'rl_training_pipeline', 'gripper_control', 'force_torque', 'joint_torque', 'imu_odom', 'safety_stop'],
        executableFormat: '.embapp (Embodied Intelligence Application)',
        launchCommand: 'embodied-agent spawn {name}.embapp --sim2real-transfer',
        wasmRuntime: 'wamr_realtime',
    },
    humanoid_robot: {
        name: '人形机器人蛹茧壳',
        platform: 'robotics',
        icon: '🦶',
        description: '适配仿生人形机器人（双足/双手/头部）的寄生壳，穿壳后蜕变为全身控制程序，对接步态规划、平衡控制、灵巧手操作与人机安全交互。',
        permissions: ['biped_gait', 'balance_mpc', 'dexterous_hand', 'neck_head_control', 'hri_safety', 'fall_protection', 'battery_bms', 'audio_chat', 'face_expression'],
        executableFormat: '.humapp (Humanoid Robot Application)',
        launchCommand: 'humanoid-ctrl boot {name}.humapp',
        wasmRuntime: 'wamr_realtime',
    },

    // ---------- 物联网平台 ----------
    yishi: {
        name: '易石互联蛹茧壳',
        platform: 'iot',
        icon: '🪨',
        description: '适配易石互联物联网平台的寄生壳，穿壳后蜕变为设备端边缘应用，对接云边协同与物模型。',
        permissions: ['edge_compute', 'thing_model', 'cloud_sync', 'device_shadow', 'ota', 'low_power'],
        executableFormat: '.ysapp (Yishi IoT Application)',
        launchCommand: 'yishi-edge deploy {name}.ysapp',
        wasmRuntime: 'wamr',
    },
};

// 兼容 CommonJS 与浏览器全局导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { ShellRegistry, SHELL_REGISTRY };
}
