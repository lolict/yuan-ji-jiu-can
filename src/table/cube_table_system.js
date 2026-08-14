/**
 * UniCore v2.0 里外嵌套三维表格系统
 * 
 * 核心概念：
 * - 0=母集/女集, 1=父集/子集
 * - 里外关系：0包含1，点击进入
 * - 函子：0包含1的映射关系
 * - 大千世界：0就是一个宇宙
 * 
 * 表格层级：
 * - 集装箱表格 = 顶层母集
 * - 格子 = 符号 = 代数 = 名字 = 函数
 */

class CubeTable {
    constructor(options = {}) {
        this.id = CubeTable.nextId++;
        this.title = options.title || '未命名';
        this.level = options.level || 0;
        this.symbol = options.symbol ?? 0;
        this.parent = null;
        this.children = [];
        this.content = null;
        this.contentType = null;
        this.isOpen = false;
        this.isExpanded = false;
        this.position = { x: 0, y: 0, z: 0 };
        this.size = { width: 100, height: 30, depth: 10 };
        this.color = this.getColorBySymbol();
        this.metadata = options.metadata || {};
    }

    static nextId = 0;

    getColorBySymbol() {
        return this.symbol === 0 ? 
            { r: 0, g: 255, b: 136 } : 
            { r: 0, g: 136, b: 255 };
    }

    // 添加子表格
    addChild(child) {
        child.parent = this;
        child.level = this.level + 1;
        this.children.push(child);
        return this;
    }

    // 设置内容
    setContent(content, type = 'algebra') {
        this.content = content;
        this.contentType = type;
        return this;
    }

    // 切换展开状态
    toggle() {
        this.isOpen = !this.isOpen;
        return this;
    }

    // 获取路径
    getPath() {
        const path = [this];
        let current = this.parent;
        while (current) {
            path.unshift(current);
            current = current.parent;
        }
        return path;
    }

    // 获取完整标题路径
    getFullTitle() {
        return this.getPath().map(t => t.title).join(' > ');
    }

    // 查找子表格
    findById(id) {
        if (this.id === id) return this;
        for (const child of this.children) {
            const found = child.findById(id);
            if (found) return found;
        }
        return null;
    }

    // 导出为数据
    export() {
        return {
            id: this.id,
            title: this.title,
            level: this.level,
            symbol: this.symbol,
            content: this.content,
            contentType: this.contentType,
            isOpen: this.isOpen,
            position: { ...this.position },
            children: this.children.map(c => c.export())
        };
    }

    // 从数据导入
    static import(data, parent = null) {
        const table = new CubeTable({
            title: data.title,
            level: data.level,
            symbol: data.symbol,
            metadata: data.metadata
        });
        table.id = data.id;
        table.content = data.content;
        table.contentType = data.contentType;
        table.isOpen = data.isOpen;
        table.position = { ...data.position };
        table.parent = parent;

        for (const childData of data.children || []) {
            const child = CubeTable.import(childData, table);
            table.children.push(child);
        }

        return table;
    }
}

/**
 * 三维表格渲染器
 */
class CubeTableRenderer {
    constructor(container) {
        this.container = container;
        this.tables = new Map();
        this.rootTables = [];
        this.selectedTable = null;
        this.camera = null;
        this.scene = null;
        this.renderer = null;
        this.meshMap = new Map();
        this.animationFrame = null;
        this.spacing = 15;
        this.init();
    }

    init() {
        if (!this.container) return;

        const width = this.container.clientWidth || 800;
        const height = this.container.clientHeight || 600;

        // 场景
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x0a0a0f);

        // 相机
        this.camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 1000);
        this.camera.position.set(0, 0, 100);

        // 渲染器
        this.renderer = new THREE.WebGLRenderer({ antialias: true });
        this.renderer.setSize(width, height);
        this.container.appendChild(this.renderer.domElement);

        // 光照
        const ambient = new THREE.AmbientLight(0x404040, 0.5);
        this.scene.add(ambient);

        const directional = new THREE.DirectionalLight(0xffffff, 0.8);
        directional.position.set(50, 50, 50);
        this.scene.add(directional);

        // 坐标轴
        const axesHelper = new THREE.AxesHelper(50);
        this.scene.add(axesHelper);

        // 渲染循环
        this.animate();

        // 窗口调整
        window.addEventListener('resize', () => this.onResize());
    }

    animate() {
        this.animationFrame = requestAnimationFrame(() => this.animate());
        this.renderer.render(this.scene, this.camera);
    }

    onResize() {
        const width = this.container.clientWidth;
        const height = this.container.clientHeight;
        this.camera.aspect = width / height;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(width, height);
    }

    // 创建表格三维对象
    createTableMesh(table) {
        const { width, height, depth } = table.size;
        const color = table.color;

        // 几何体
        const geometry = new THREE.BoxGeometry(width, height, depth);
        
        // 材质
        const material = new THREE.MeshPhongMaterial({
            color: new THREE.Color(color.r / 255, color.g / 255, color.b / 255),
            emissive: new THREE.Color(0.1, 0.1, 0.1),
            transparent: true,
            opacity: 0.9
        });

        // 网格
        const mesh = new THREE.Mesh(geometry, material);
        mesh.position.set(table.position.x, table.position.y, table.position.z);
        mesh.userData.tableId = table.id;

        // 边框
        const edges = new THREE.EdgesGeometry(geometry);
        const lineMaterial = new THREE.LineBasicMaterial({ color: 0xffffff });
        const wireframe = new THREE.LineSegments(edges, lineMaterial);
        mesh.add(wireframe);

        // 标签
        const canvas = document.createElement('canvas');
        canvas.width = 256;
        canvas.height = 64;
        const ctx = canvas.getContext('2d');
        ctx.fillStyle = `rgb(${color.r}, ${color.g}, ${color.b})`;
        ctx.font = 'bold 24px Courier New';
        ctx.textAlign = 'center';
        ctx.fillText(`[${table.symbol}] ${table.title}`, 128, 35);
        
        if (table.content !== null) {
            ctx.font = '18px Courier New';
            ctx.fillStyle = '#888';
            ctx.fillText(String(table.content).substring(0, 20), 128, 55);
        }

        const texture = new THREE.CanvasTexture(canvas);
        const spriteMaterial = new THREE.SpriteMaterial({ map: texture });
        const sprite = new THREE.Sprite(spriteMaterial);
        sprite.position.y = height / 2 + 10;
        sprite.scale.set(width, height / 2, 1);
        mesh.add(sprite);

        this.scene.add(mesh);
        this.meshMap.set(table.id, mesh);

        return mesh;
    }

    // 更新表格位置
    updateTablePosition(table, x, y, z) {
        table.position = { x, y, z };
        const mesh = this.meshMap.get(table.id);
        if (mesh) {
            mesh.position.set(x, y, z);
        }
    }

    // 布局表格
    layoutTables(tables, startX = -50, startY = 30, startZ = 0) {
        let currentX = startX;
        let currentY = startY;
        let currentZ = startZ;
        let index = 0;

        for (const table of tables) {
            this.updateTablePosition(table, currentX, currentY, currentZ);
            this.createTableMesh(table);
            this.tables.set(table.id, table);

            currentX += table.size.width + this.spacing;
            index++;

            if (index % 8 === 0) {
                currentX = startX;
                currentY -= table.size.height + this.spacing;
            }
        }
    }

    // 布局嵌套表格
    layoutNestedTables(table, x, y, z, level = 0) {
        this.updateTablePosition(table, x, y, z);
        this.createTableMesh(table);
        this.tables.set(table.id, table);

        if (table.isOpen && table.children.length > 0) {
            const childSpacing = this.spacing * 3;
            let childY = y - table.size.height - this.spacing;
            
            for (const child of table.children) {
                this.layoutNestedTables(child, x, childY, z, level + 1);
                childY -= child.size.height + this.spacing;
            }
        }
    }

    // 点击选择
    onTableClick(tableId) {
        const table = this.tables.get(tableId);
        if (!table) return;

        // 清除之前的选中
        if (this.selectedTable) {
            const prevMesh = this.meshMap.get(this.selectedTable.id);
            if (prevMesh) {
                prevMesh.material.emissive.setHex(0x222222);
                prevMesh.scale.set(1, 1, 1);
            }
        }

        // 设置新的选中
        this.selectedTable = table;
        const mesh = this.meshMap.get(tableId);
        if (mesh) {
            mesh.material.emissive.setHex(0x00ff00);
            mesh.scale.set(1.1, 1.1, 1.1);
        }

        // 如果是0符号，自动展开
        if (table.symbol === 0 && table.children.length > 0) {
            table.toggle();
            this.refreshScene();
        }

        return table;
    }

    // 刷新场景
    refreshScene() {
        // 清除所有网格
        this.meshMap.forEach(mesh => {
            this.scene.remove(mesh);
            mesh.geometry.dispose();
            mesh.material.dispose();
        });
        this.meshMap.clear();
        this.tables.clear();

        // 重新创建
        let y = 30;
        for (const table of this.rootTables) {
            this.layoutNestedTables(table, 0, y, 0);
            y -= table.size.height + this.spacing * 3;
        }
    }

    // 销毁
    destroy() {
        if (this.animationFrame) {
            cancelAnimationFrame(this.animationFrame);
        }
        if (this.renderer) {
            this.renderer.dispose();
            if (this.renderer.domElement.parentNode) {
                this.renderer.domElement.parentNode.removeChild(this.renderer.domElement);
            }
        }
    }
}

/**
 * 里外映射引擎
 * 实现0包含1的函子关系
 */
class InsideOutsideMapper {
    constructor() {
        this.maps = new Map();
        this.registerDefaultMaps();
    }

    registerDefaultMaps() {
        // 0 → 1 的默认映射
        this.register(0, {
            symbol: 1,
            content: '0中必然包含1的默认函数',
            contentType: 'function',
            name: '内在1',
            description: '每个0内部必然包含一个1'
        });

        // 1 → 0 的默认映射
        this.register(1, {
            symbol: 0,
            content: '1反馈给0的信息',
            contentType: 'algebra',
            name: '反馈0',
            description: '1对0的反馈'
        });
    }

    register(symbol, mapping) {
        this.maps.set(symbol, {
            ...mapping,
            id: `map_${symbol}_${Date.now()}`
        });
    }

    getMapping(symbol) {
        return this.maps.get(symbol);
    }

    // 生成0包含1的子表格
    generateChildTable(parentTable) {
        const mapping = this.getMapping(parentTable.symbol);
        if (!mapping) return null;

        const childTable = new CubeTable({
            title: `${parentTable.title}的${mapping.name}`,
            level: parentTable.level + 1,
            symbol: mapping.symbol,
            metadata: {
                isMappedChild: true,
                parentMapping: mapping.id
            }
        });

        childTable.setContent(mapping.content, mapping.contentType);
        return childTable;
    }

    // 自动为0添加1
    autoFillZeros(table) {
        if (table.symbol === 0 && table.children.length === 0) {
            const child = this.generateChildTable(table);
            if (child) {
                table.addChild(child);
            }
        }

        for (const child of table.children) {
            this.autoFillZeros(child);
        }
    }
}

/**
 * 表格容器系统
 * 表格作为一个集装箱
 */
class TableContainer {
    constructor(id, options = {}) {
        this.id = id;
        this.title = options.title || '集装箱';
        this.tables = [];
        this.metadata = options.metadata || {};
        this.createdAt = new Date();
    }

    addTable(table) {
        this.tables.push(table);
        return this;
    }

    findTableById(id) {
        for (const table of this.tables) {
            const found = table.findById(id);
            if (found) return found;
        }
        return null;
    }

    export() {
        return {
            id: this.id,
            title: this.title,
            metadata: this.metadata,
            createdAt: this.createdAt.toISOString(),
            tables: this.tables.map(t => t.export())
        };
    }

    static import(data) {
        const container = new TableContainer(data.id, {
            title: data.title,
            metadata: data.metadata
        });

        for (const tableData of data.tables) {
            const table = CubeTable.import(tableData);
            container.tables.push(table);
        }

        return container;
    }
}

// 导出
window.CubeTable = CubeTable;
window.CubeTableRenderer = CubeTableRenderer;
window.InsideOutsideMapper = InsideOutsideMapper;
window.TableContainer = TableContainer;

console.log('CubeTable System loaded');
