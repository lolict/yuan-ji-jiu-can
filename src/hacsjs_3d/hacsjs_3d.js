/**
 * hacsjs_3d.js - UniCore v2.0 三维专属引擎
 * 
 * 核心理念：
 * - 完全脱离HTML/CSS二维束缚
 * - 原生三维渲染与操控
 * - 服从WASM底层核心
 * 
 * 功能：
 * - Three.js三维渲染
 * - 集装箱可视化
 * - 节点因果链展示
 * - 空间阵列展示
 * - 十字方向键操控
 * - 圆柱视角控制
 */

class HACSJS3D {
    constructor() {
        // Three.js 核心
        this.scene = null;
        this.camera = null;
        this.renderer = null;
        
        // 控制器
        this.orbitControls = null;
        this.cylindricalControls = null;
        
        // 对象池
        this.containers = new Map();
        this.nodes = new Map();
        this.arrays = new Map();
        this.containerMeshes = new Map();
        this.nodeMeshes = new Map();
        
        // 状态
        this.activeContainerId = null;
        this.showGrid = true;
        this.showAxes = true;
        this.animationFrame = null;
        
        // 回调
        this.onContainerClick = null;
        this.onNodeClick = null;
        
        // 初始化
        this.init();
    }

    /**
     * 初始化Three.js场景
     */
    init() {
        // 创建场景
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x0a0a0f);
        this.scene.fog = new THREE.Fog(0x0a0a0f, 100, 500);

        // 创建相机
        this.camera = new THREE.PerspectiveCamera(
            60,
            window.innerWidth / window.innerHeight,
            0.1,
            1000
        );
        this.camera.position.set(50, 50, 50);

        // 创建渲染器
        this.renderer = new THREE.WebGLRenderer({
            antialias: true,
            alpha: true
        });
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        this.renderer.setPixelRatio(window.devicePixelRatio);

        // 添加到DOM
        const container = document.getElementById('hacs3d-container');
        if (container) {
            container.appendChild(this.renderer.domElement);
        } else {
            document.body.appendChild(this.renderer.domElement);
        }

        // 添加光照
        this.addLights();

        // 添加坐标轴
        if (this.showAxes) {
            this.addAxes();
        }

        // 添加网格
        if (this.showGrid) {
            this.addGrid();
        }

        // 监听窗口大小变化
        window.addEventListener('resize', () => this.onResize());

        // 启动渲染循环
        this.animate();

        console.log('HACSJS3D initialized');
    }

    /**
     * 添加光照
     */
    addLights() {
        // 环境光
        const ambient = new THREE.AmbientLight(0x404040, 0.5);
        this.scene.add(ambient);

        // 主光源
        const mainLight = new THREE.DirectionalLight(0x00ff88, 1);
        mainLight.position.set(50, 100, 50);
        this.scene.add(mainLight);

        // 辅助光源
        const fillLight = new THREE.DirectionalLight(0x0088ff, 0.3);
        fillLight.position.set(-50, 50, -50);
        this.scene.add(fillLight);

        // 点光源
        const pointLight = new THREE.PointLight(0xffd93d, 0.5, 200);
        pointLight.position.set(0, 100, 0);
        this.scene.add(pointLight);
    }

    /**
     * 添加坐标轴
     */
    addAxes() {
        const axesHelper = new THREE.AxesHelper(100);
        this.scene.add(axesHelper);
    }

    /**
     * 添加网格
     */
    addGrid() {
        const gridHelper = new THREE.GridHelper(200, 20, 0x00ff88, 0x003322);
        this.scene.add(gridHelper);
    }

    /**
     * 渲染循环
     */
    animate() {
        this.animationFrame = requestAnimationFrame(() => this.animate());
        this.renderer.render(this.scene, this.camera);
    }

    /**
     * 窗口大小变化处理
     */
    onResize() {
        this.camera.aspect = window.innerWidth / window.innerHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(window.innerWidth, window.innerHeight);
    }

    // ==================== 集装箱操作 ====================

    /**
     * 创建集装箱3D对象
     */
    createContainer(id, x = 0, y = 0, z = 0) {
        if (this.containerMeshes.has(id)) {
            return this.containerMeshes.get(id);
        }

        // 创建几何体 (立方体)
        const geometry = new THREE.BoxGeometry(8, 8, 8);
        
        // 创建材质
        const material = new THREE.MeshPhongMaterial({
            color: 0x00ff88,
            emissive: 0x002211,
            specular: 0x00ffaa,
            shininess: 100,
            transparent: true,
            opacity: 0.9
        });

        // 创建网格
        const mesh = new THREE.Mesh(geometry, material);
        mesh.position.set(x, y, z);
        mesh.userData.containerId = id;

        // 添加边框
        const edges = new THREE.EdgesGeometry(geometry);
        const line = new THREE.LineSegments(
            edges,
            new THREE.LineBasicMaterial({ color: 0x00ffaa })
        );
        mesh.add(line);

        // 添加到场景
        this.scene.add(mesh);
        this.containerMeshes.set(id, mesh);

        // 存储数据
        this.containers.set(id, {
            id,
            x, y, z,
            value: 0,
            flags: 0
        });

        return mesh;
    }

    /**
     * 更新集装箱
     */
    updateContainer(id, data) {
        const mesh = this.containerMeshes.get(id);
        const container = this.containers.get(id);

        if (!mesh || !container) return;

        if (data.position) {
            mesh.position.set(data.position.x, data.position.y, data.position.z);
            container.x = data.position.x;
            container.y = data.position.y;
            container.z = data.position.z;
        }

        if (data.value !== undefined) {
            container.value = data.value;
            // 根据值改变颜色
            const hue = (data.value / 255) * 0.3;
            mesh.material.color.setHSL(hue, 1, 0.5);
            mesh.material.emissive.setHSL(hue, 1, 0.1);
        }

        if (data.active) {
            mesh.material.emissive.setHex(0x00ff00);
            mesh.scale.set(1.2, 1.2, 1.2);
        } else {
            mesh.scale.set(1, 1, 1);
        }

        if (data.flags !== undefined) {
            container.flags = data.flags;
        }
    }

    /**
     * 设置活动集装箱
     */
    setActiveContainer(id) {
        // 取消之前的活动状态
        if (this.activeContainerId !== null) {
            this.updateContainer(this.activeContainerId, { active: false });
        }

        this.activeContainerId = id;

        if (id !== null) {
            this.updateContainer(id, { active: true });
            
            // 聚焦到活动集装箱
            const mesh = this.containerMeshes.get(id);
            if (mesh) {
                this.focusOn(mesh.position);
            }
        }

        if (this.onContainerClick) {
            this.onContainerClick(id);
        }
    }

    /**
     * 创建所有集装箱
     */
    createAllContainers(count = 256) {
        const size = Math.ceil(Math.cbrt(count));
        let index = 0;

        for (let x = 0; x < size && index < count; x++) {
            for (let y = 0; y < size && index < count; y++) {
                for (let z = 0; z < size && index < count; z++) {
                    const pos = this.gridToWorld(x, y, z, size);
                    this.createContainer(index++, pos.x, pos.y, pos.z);
                }
            }
        }
    }

    /**
     * 网格坐标转世界坐标
     */
    gridToWorld(x, y, z, gridSize) {
        const spacing = 12;
        const offset = (gridSize - 1) * spacing / 2;
        return {
            x: x * spacing - offset,
            y: y * spacing - offset,
            z: z * spacing - offset
        };
    }

    // ==================== 节点操作 ====================

    /**
     * 创建节点3D对象
     */
    createNode(id, x = 0, y = 0, z = 0, type = 1) {
        if (this.nodeMeshes.has(id)) {
            return this.nodeMeshes.get(id);
        }

        // 根据类型选择几何体
        let geometry;
        switch (type) {
            case 1: // 普通节点 - 球体
                geometry = new THREE.SphereGeometry(3, 16, 16);
                break;
            case 2: // 动作节点 - 八面体
                geometry = new THREE.OctahedronGeometry(4);
                break;
            case 3: // 状态节点 - 正十二面体
                geometry = new THREE.DodecahedronGeometry(3);
                break;
            case 4: // 因果触发节点 - 二十面体
                geometry = new THREE.IcosahedronGeometry(4);
                break;
            default:
                geometry = new THREE.SphereGeometry(3, 16, 16);
        }

        // 创建材质
        const colors = [0x00d4ff, 0xff6b6b, 0xffd93d, 0x00ff88];
        const material = new THREE.MeshPhongMaterial({
            color: colors[type - 1] || 0x00d4ff,
            emissive: 0x001122,
            specular: 0xffffff,
            shininess: 100
        });

        // 创建网格
        const mesh = new THREE.Mesh(geometry, material);
        mesh.position.set(x, y, z);
        mesh.userData.nodeId = id;
        mesh.userData.nodeType = type;

        // 添加标签
        const canvas = document.createElement('canvas');
        canvas.width = 64;
        canvas.height = 32;
        const ctx = canvas.getContext('2d');
        ctx.fillStyle = '#00d4ff';
        ctx.font = '20px Courier New';
        ctx.textAlign = 'center';
        ctx.fillText(`#${id}`, 32, 22);

        const texture = new THREE.CanvasTexture(canvas);
        const spriteMaterial = new THREE.SpriteMaterial({ map: texture });
        const sprite = new THREE.Sprite(spriteMaterial);
        sprite.position.y = 8;
        mesh.add(sprite);

        // 添加边框
        const edges = new THREE.EdgesGeometry(geometry);
        const line = new THREE.LineSegments(
            edges,
            new THREE.LineBasicMaterial({ color: 0x00d4ff })
        );
        mesh.add(line);

        // 添加到场景
        this.scene.add(mesh);
        this.nodeMeshes.set(id, mesh);

        // 存储数据
        this.nodes.set(id, {
            id,
            x, y, z,
            type,
            triggered: false,
            cause: null
        });

        return mesh;
    }

    /**
     * 更新节点
     */
    updateNode(id, data) {
        const mesh = this.nodeMeshes.get(id);
        const node = this.nodes.get(id);

        if (!mesh || !node) return;

        if (data.position) {
            mesh.position.set(data.position.x, data.position.y, data.position.z);
            node.x = data.position.x;
            node.y = data.position.y;
            node.z = data.position.z;
        }

        if (data.triggered !== undefined) {
            node.triggered = data.triggered;
            mesh.material.emissive.setHex(data.triggered ? 0x00ff00 : 0x001122);
        }
    }

    /**
     * 连接节点（绘制因果链）
     */
    connectNodes(sourceId, targetId) {
        const sourceMesh = this.nodeMeshes.get(sourceId);
        const targetMesh = this.nodeMeshes.get(targetId);

        if (!sourceMesh || !targetMesh) return;

        // 创建连接线
        const points = [
            sourceMesh.position.clone(),
            targetMesh.position.clone()
        ];
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        const material = new THREE.LineBasicMaterial({
            color: 0xffd93d,
            linewidth: 2
        });
        const line = new THREE.Line(geometry, material);

        this.scene.add(line);

        return line;
    }

    /**
     * 创建节点因果链
     */
    createNodeChain(nodes) {
        // 清除旧的连接线
        this.clearConnections();

        // 创建节点
        const spacing = 20;
        for (let i = 0; i < nodes.length; i++) {
            const node = nodes[i];
            const x = (i % 10) * spacing - 50;
            const y = Math.floor(i / 10) * spacing - 50;
            this.createNode(node.id, x, y, 0, node.type);

            // 连接因果
            if (node.cause !== null && node.cause !== undefined) {
                this.connectNodes(node.cause, node.id);
            }
        }
    }

    /**
     * 清除连接线
     */
    clearConnections() {
        const toRemove = [];
        this.scene.traverse((obj) => {
            if (obj.isLine) {
                toRemove.push(obj);
            }
        });
        toRemove.forEach((obj) => {
            this.scene.remove(obj);
            obj.geometry.dispose();
            obj.material.dispose();
        });
    }

    // ==================== 空间阵列 ====================

    /**
     * 创建空间阵列
     */
    createSpatialArray(type, config = {}) {
        const arrayId = type + '_' + Date.now();
        
        switch (type) {
            case 'matrix':
                return this.createMatrixArray(config);
            case 'cone':
                return this.createConeArray(config);
            case 'sphere':
                return this.createSphereArray(config);
            case 'hex':
                return this.createHexArray(config);
            case 'wave':
                return this.createWaveArray(config);
            default:
                return this.createMatrixArray(config);
        }
    }

    /**
     * 创建矩阵阵列
     */
    createMatrixArray(config = {}) {
        const {
            rows = 4,
            cols = 4,
            layers = 4,
            spacing = 12
        } = config;

        const meshes = [];
        let index = 0;

        for (let l = 0; l < layers; l++) {
            for (let r = 0; r < rows; r++) {
                for (let c = 0; c < cols; c++) {
                    const x = c * spacing - (cols - 1) * spacing / 2;
                    const y = r * spacing - (rows - 1) * spacing / 2;
                    const z = l * spacing - (layers - 1) * spacing / 2;

                    const mesh = this.createContainer(index++, x, y, z);
                    meshes.push(mesh);
                }
            }
        }

        this.arrays.set('matrix', meshes);
        return meshes;
    }

    /**
     * 创建锥阵
     */
    createConeArray(config = {}) {
        const {
            layers = 5,
            baseRadius = 30,
            height = 40
        } = config;

        const meshes = [];
        let index = 0;

        for (let layer = 0; layer < layers; layer++) {
            const radius = baseRadius * (1 - layer / layers);
            const y = layer * (height / layers) - height / 2;
            const circumference = 2 * Math.PI * radius;
            const count = Math.max(1, Math.floor(circumference / 15));

            for (let i = 0; i < count; i++) {
                const angle = (i / count) * Math.PI * 2;
                const x = Math.cos(angle) * radius;
                const z = Math.sin(angle) * radius;

                const mesh = this.createContainer(index++, x, y, z);
                meshes.push(mesh);
            }
        }

        this.arrays.set('cone', meshes);
        return meshes;
    }

    /**
     * 创建球阵
     */
    createSphereArray(config = {}) {
        const {
            radius = 30,
            layers = 5
        } = config;

        const meshes = [];
        let index = 0;

        for (let layer = 0; layer < layers; layer++) {
            const phi = Math.PI * layer / layers;
            const y = Math.cos(phi) * radius;
            const ringRadius = Math.sin(phi) * radius;
            const count = Math.max(1, Math.floor(ringRadius * 4));

            for (let i = 0; i < count; i++) {
                const theta = (i / count) * Math.PI * 2;
                const x = Math.cos(theta) * ringRadius;
                const z = Math.sin(theta) * ringRadius;

                const mesh = this.createContainer(index++, x, y, z);
                meshes.push(mesh);
            }
        }

        this.arrays.set('sphere', meshes);
        return meshes;
    }

    /**
     * 创建蜂窝阵
     */
    createHexArray(config = {}) {
        const {
            size = 5,
            spacing = 12
        } = config;

        const meshes = [];
        let index = 0;

        for (let q = -size; q <= size; q++) {
            for (let r = -size; r <= size; r++) {
                if (Math.abs(q + r) > size) continue;

                const x = spacing * (3/2 * q);
                const z = spacing * (Math.sqrt(3)/2 * q + Math.sqrt(3) * r);
                const y = 0;

                const mesh = this.createContainer(index++, x, y, z);
                meshes.push(mesh);
            }
        }

        this.arrays.set('hex', meshes);
        return meshes;
    }

    /**
     * 创建波浪阵
     */
    createWaveArray(config = {}) {
        const {
            width = 10,
            height = 10,
            spacing = 12,
            amplitude = 10
        } = config;

        const meshes = [];
        let index = 0;

        for (let x = 0; x < width; x++) {
            for (let z = 0; z < height; z++) {
                const worldX = (x - width/2) * spacing;
                const worldZ = (z - height/2) * spacing;
                const y = Math.sin(x * 0.5) * amplitude + Math.cos(z * 0.5) * amplitude;

                const mesh = this.createContainer(index++, worldX, y, worldZ);
                meshes.push(mesh);
            }
        }

        this.arrays.set('wave', meshes);
        return meshes;
    }

    // ==================== 相机控制 ====================

    /**
     * 聚焦到指定位置
     */
    focusOn(position) {
        const target = new THREE.Vector3(position.x, position.y, position.z);
        
        // 平滑过渡
        const start = this.camera.position.clone();
        const end = target.clone().add(new THREE.Vector3(30, 30, 30));
        
        let t = 0;
        const animate = () => {
            t += 0.05;
            if (t >= 1) {
                this.camera.lookAt(target);
                return;
            }
            
            this.camera.position.lerpVectors(start, end, t);
            this.camera.lookAt(target);
            requestAnimationFrame(animate);
        };
        animate();
    }

    /**
     * 圆柱视角控制
     */
    setCylindricalView(theta, radius, height) {
        theta = theta * Math.PI / 180;
        
        this.camera.position.x = Math.cos(theta) * radius;
        this.camera.position.y = height;
        this.camera.position.z = Math.sin(theta) * radius;
        
        this.camera.lookAt(0, 0, 0);
    }

    /**
     * 正交视角
     */
    setOrthographicView(direction) {
        const distance = 100;
        
        switch (direction) {
            case 'top':
                this.camera.position.set(0, distance, 0);
                this.camera.lookAt(0, 0, 0);
                break;
            case 'bottom':
                this.camera.position.set(0, -distance, 0);
                this.camera.lookAt(0, 0, 0);
                break;
            case 'front':
                this.camera.position.set(0, 0, distance);
                this.camera.lookAt(0, 0, 0);
                break;
            case 'back':
                this.camera.position.set(0, 0, -distance);
                this.camera.lookAt(0, 0, 0);
                break;
            case 'left':
                this.camera.position.set(-distance, 0, 0);
                this.camera.lookAt(0, 0, 0);
                break;
            case 'right':
                this.camera.position.set(distance, 0, 0);
                this.camera.lookAt(0, 0, 0);
                break;
        }
    }

    // ==================== 十字方向键 ====================

    /**
     * 移动选中对象
     */
    moveSelected(direction, distance = 10) {
        if (this.activeContainerId === null) return;

        const mesh = this.containerMeshes.get(this.activeContainerId);
        if (!mesh) return;

        switch (direction) {
            case 'up':
                mesh.position.y += distance;
                break;
            case 'down':
                mesh.position.y -= distance;
                break;
            case 'left':
                mesh.position.x -= distance;
                break;
            case 'right':
                mesh.position.x += distance;
                break;
            case 'forward':
                mesh.position.z += distance;
                break;
            case 'backward':
                mesh.position.z -= distance;
                break;
        }

        // 更新数据
        this.updateContainer(this.activeContainerId, {
            position: {
                x: mesh.position.x,
                y: mesh.position.y,
                z: mesh.position.z
            }
        });
    }

    /**
     * 旋转选中对象
     */
    rotateSelected(axis, angle = 90) {
        if (this.activeContainerId === null) return;

        const mesh = this.containerMeshes.get(this.activeContainerId);
        if (!mesh) return;

        const rad = angle * Math.PI / 180;

        switch (axis) {
            case 'x':
                mesh.rotation.x += rad;
                break;
            case 'y':
                mesh.rotation.y += rad;
                break;
            case 'z':
                mesh.rotation.z += rad;
                break;
        }
    }

    // ==================== 导出/导入 ====================

    /**
     * 导出场景状态
     */
    exportState() {
        return {
            containers: Array.from(this.containers.entries()),
            nodes: Array.from(this.nodes.entries()),
            activeContainerId: this.activeContainerId,
            cameraPosition: {
                x: this.camera.position.x,
                y: this.camera.position.y,
                z: this.camera.position.z
            }
        };
    }

    /**
     * 导入场景状态
     */
    importState(state) {
        // 清除当前场景
        this.clearAll();

        // 恢复集装箱
        if (state.containers) {
            for (const [id, data] of state.containers) {
                this.createContainer(id, data.x, data.y, data.z);
                this.updateContainer(id, data);
            }
        }

        // 恢复节点
        if (state.nodes) {
            for (const [id, data] of state.nodes) {
                this.createNode(id, data.x, data.y, data.z, data.type);
                this.updateNode(id, data);
            }
        }

        // 恢复相机
        if (state.cameraPosition) {
            this.camera.position.set(
                state.cameraPosition.x,
                state.cameraPosition.y,
                state.cameraPosition.z
            );
        }

        // 恢复活动集装箱
        if (state.activeContainerId !== null) {
            this.setActiveContainer(state.activeContainerId);
        }
    }

    /**
     * 清除所有对象
     */
    clearAll() {
        this.containerMeshes.forEach((mesh) => {
            this.scene.remove(mesh);
            mesh.geometry.dispose();
            mesh.material.dispose();
        });
        this.containerMeshes.clear();
        this.containers.clear();

        this.nodeMeshes.forEach((mesh) => {
            this.scene.remove(mesh);
            mesh.geometry.dispose();
            mesh.material.dispose();
        });
        this.nodeMeshes.clear();
        this.nodes.clear();

        this.arrays.clear();
        this.clearConnections();

        this.activeContainerId = null;
    }

    /**
     * 销毁
     */
    destroy() {
        if (this.animationFrame) {
            cancelAnimationFrame(this.animationFrame);
        }

        this.clearAll();

        if (this.renderer) {
            this.renderer.dispose();
            if (this.renderer.domElement.parentNode) {
                this.renderer.domElement.parentNode.removeChild(this.renderer.domElement);
            }
        }

        console.log('HACSJS3D destroyed');
    }
}

// 导出
window.HACSJS3D = HACSJS3D;

// 创建全局实例
window.hacs3d = null;

function initHACSJS3D(containerId) {
    if (window.hacs3d) {
        window.hacs3d.destroy();
    }
    window.hacs3d = new HACSJS3D();
    return window.hacs3d;
}

console.log('hacsjs_3d.js loaded');
