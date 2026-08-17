// 满全法 - Rust 编码引擎核心
// 高效底层：向量运算、镜像对比、层级关系

/// 汉字几何坐标（64位单元格）
#[derive(Debug, Clone, Copy, PartialEq)]
struct 汉字坐标 {
    x: u16,  // 左右轴
    y: u16,  // 上下轴
    z: u16,  // 深度轴
    polarity: u8,  // 极性: 0=阴, 1=阳
    sovereignty: u8, // 主权位
}

impl 汉字坐标 {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z, polarity: 0, sovereignty: 0 }
    }
    
    /// 计算两个坐标的曼哈顿距离
    fn 距离(&self, 其他: &汉字坐标) -> u32 {
        let dx = (self.x as i32 - 其他.x as i32).abs() as u32;
        let dy = (self.y as i32 - 其他.y as i32).abs() as u32;
        let dz = (self.z as i32 - 其他.z as i32).abs() as u32;
        dx + dy + dz
    }
    
    /// 欧几里得距离的平方
    fn 欧距平方(&self, 其他: &汉字坐标) -> u32 {
        let dx = (self.x as i32 - 其他.x as i32).abs() as u32;
        let dy = (self.y as i32 - 其他.y as i32).abs() as u32;
        let dz = (self.z as i32 - 其他.z as i32).abs() as u32;
        dx * dx + dy * dy + dz * dz
    }
}

/// 部件对（左右关系）
#[derive(Debug, Clone)]
struct 部件对 {
    左部件: String,
    右部件: String,
    左坐标: 汉字坐标,
    右坐标: 汉字坐标,
    亲密度: f64,
}

/// 镜像（一次编码的完整快照）
#[derive(Debug, Clone)]
struct 镜像 {
    id: u64,
    timestamp: u64,
    部件对列表: Vec<部件对>,
    总分: f64,
}

impl 镜像 {
    fn new(id: u64) -> Self {
        Self {
            id,
            timestamp: 0,
            部件对列表: Vec::new(),
            总分: 0.0,
        }
    }
    
    fn 添加部件对(&mut self, 对: 部件对) {
        self.部件对列表.push(对);
        self.计算总分();
    }
    
    fn 计算总分(&mut self) {
        self.总分 = self.部件对列表.iter().map(|对| 对.亲密度).sum();
    }
}

/// 镜像对比引擎
struct 对比引擎 {
    镜像列表: Vec<镜像>,
}

impl 对比引擎 {
    fn new() -> Self {
        Self { 镜像列表: Vec::new() }
    }
    
    fn 添加镜像(&mut self, 镜像: 镜像) {
        self.镜像列表.push(镜像);
    }
    
    /// 两次镜像的差异度
    fn 差异度(&self, 镜像甲: &镜像, 镜像乙: &镜像) -> f64 {
        let mut 总差异 = 0.0;
        let 甲数 = 镜像甲.部件对列表.len();
        let 乙数 = 镜像乙.部件对列表.len();
        
        if 甲数 == 0 && 乙数 == 0 { return 0.0; }
        let 最大数 = 甲数.max(乙数) as f64;
        
        for i in 0..甲数.min(乙数) {
            let 甲亲 = 镜像甲.部件对列表[i].亲密度;
            let 乙亲 = 镜像乙.部件对列表[i].亲密度;
            总差异 += (甲亲 - 乙亲).abs();
        }
        
        // 部件数量差异也算入
        let 数量差异 = (甲数 as i32 - 乙数 as i32).abs() as f64;
        总差异 += 数量差异 * 0.5;
        
        总差异 / 最大数
    }
    
    /// PK 两个镜像，返回胜出者
    fn 对比(&self, 甲: usize, 乙: usize) -> (usize, f64) {
        if 甲 >= self.镜像列表.len() || 乙 >= self.镜像列表.len() {
            return (甲, 0.0);
        }
        let 差异 = self.差异度(&self.镜像列表[甲], &self.镜像列表[乙]);
        let 甲分 = self.镜像列表[甲].总分;
        let 乙分 = self.镜像列表[乙].总分;
        
        if 甲分 >= 乙分 {
            (甲, 差异)
        } else {
            (乙, 差异)
        }
    }
}

/// 层级分类
#[derive(Debug, Clone, PartialEq)]
enum 层级 {
    一元,  // 基础部件
    二元,  // 左右配对
    三元,  // 见证裁判
    四元,  // 关系网络
    五元,  // 模块集成
    六元,  // 系统层级
    七元,  // 意识路由
    八元,  // 流年叠加
    九元,  // 满全法本体
}

fn main() {
    println!("=== 满全法 Rust 编码引擎 ===");
    println!("");
    
    // === 测试1：坐标运算 ===
    let 女 = 汉字坐标::new(100, 200, 50);
    let 子 = 汉字坐标::new(300, 200, 50);
    let 日 = 汉字坐标::new(150, 180, 40);
    let 月 = 汉字坐标::new(320, 180, 40);
    
    println!("坐标距离测试:");
    println!("  女-子 (好): 曼哈顿={} 欧距平方={}", 女.距离(&子), 女.欧距平方(&子));
    println!("  日-月 (明): 曼哈顿={} 欧距平方={}", 日.距离(&月), 日.欧距平方(&月));
    println!("  女-日 (跨界): 曼哈顿={} 欧距平方={}", 女.距离(&日), 女.欧距平方(&日));
    
    println!("");
    
    // === 测试2：镜像对比 ===
    let mut 引擎 = 对比引擎::new();
    
    // 镜像 0: 基础版本
    let mut 镜像0 = 镜像::new(0);
    镜像0.添加部件对(部件对 {
        左部件: "女".to_string(),
        右部件: "子".to_string(),
        左坐标: 女,
        右坐标: 子,
        亲密度: 0.85,
    });
    镜像0.添加部件对(部件对 {
        左部件: "日".to_string(),
        右部件: "月".to_string(),
        左坐标: 日,
        右坐标: 月,
        亲密度: 0.78,
    });
    
    // 镜像 1: 改进版本（亲密度更高）
    let mut 镜像1 = 镜像::new(1);
    镜像1.添加部件对(部件对 {
        左部件: "女".to_string(),
        右部件: "子".to_string(),
        左坐标: 汉字坐标::new(110, 200, 50),
        右坐标: 汉字坐标::new(280, 200, 50),
        亲密度: 0.92,
    });
    镜像1.添加部件对(部件对 {
        左部件: "日".to_string(),
        右部件: "月".to_string(),
        左坐标: 汉字坐标::new(160, 180, 40),
        右坐标: 汉字坐标::new(300, 180, 40),
        亲密度: 0.88,
    });
    镜像1.添加部件对(部件对 {
        左部件: "木".to_string(),
        右部件: "木".to_string(),
        左坐标: 汉字坐标::new(120, 220, 60),
        右坐标: 汉字坐标::new(260, 220, 60),
        亲密度: 0.95,
    });
    
    引擎.添加镜像(镜像0.clone());
    引擎.添加镜像(镜像1.clone());
    
    println!("镜像对比测试:");
    println!("  镜像0 总分: {:.4}", 镜像0.总分);
    println!("  镜像1 总分: {:.4}", 镜像1.总分);
    
    let (胜者, 差异) = 引擎.对比(0, 1);
    println!("  PK结果: 镜像{} 胜出 (差异度={:.4})", 胜者, 差异);
    
    println!("");
    
    // === 测试3：层级分类 ===
    println!("层级体系演示:");
    let 层级列表 = [
        层级::一元, 层级::二元, 层级::三元,
        层级::四元, 层级::五元, 层级::六元,
        层级::七元, 层级::八元, 层级::九元,
    ];
    
    for (i, 层) in 层级列表.iter().enumerate() {
        let 深度 = i as u32 + 1;
        let 宽度 = 深度 * 64;
        println!("  {:?}: 深度={}, 编码宽度={}位", 层, 深度, 宽度);
    }
    
    println!("");
    println!("=== 引擎就绪 ===");
}
