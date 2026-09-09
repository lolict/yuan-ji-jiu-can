//! 机器码容器进制 (Macro Instruction Set Architecture)
//! 
//! 核心思想：
//! - 原始机器码 = 0x48 0x89 0x3C ...
//! - 多个机器码按先后顺序绑定 = 一个"宏指令"（容器）
//! - 命名这个宏指令 = 新的"进制位"
//! - 多个宏指令 = 更大的结构

/// 机器码容器：把多条原始机器码绑定到一个名字
/// 
/// 这就是"进制"的本质：
/// - 二进制：0 和 1 的组合
/// - 机器码进制：多条机器码的组合 + 命名
#[derive(Debug, Clone)]
pub struct MachineCodeContainer {
    /// 容器名字（就是这个"进制位"的名称）
    name: String,
    /// 容器内部的机器码序列
    code: Vec<u8>,
    /// 容器的元数据（描述这个容器做什么）
    metadata: ContainerMetadata,
}

/// 容器元数据
#[derive(Debug, Clone)]
pub struct ContainerMetadata {
    /// 容器类型
    pub container_type: ContainerType,
    /// 输入参数数量
    pub input_count: u8,
    /// 输出结果数量
    pub output_count: u8,
    /// 是否可折叠（进位）
    pub foldable: bool,
}

/// 容器类型 = 进制的"位权"
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerType {
    /// 加法容器：254个单位 + 溢出
    Addition,
    /// 螺旋容器：旋转相位
    Spiral,
    /// 镜像容器：对称翻转
    Mirror,
    /// 折叠容器：进位/层叠
    Fold,
    /// 颜色容器：色系编码
    Color,
    /// 声母容器：发音位置
    Shengmu,
    /// 韵母容器：口腔形状
    Yunmu,
    /// 声调容器：音高变化
    Tone,
}

impl MachineCodeContainer {
    /// 创建一个新的机器码容器
    pub fn new(name: &str, code: Vec<u8>, container_type: ContainerType) -> Self {
        Self {
            name: name.to_string(),
            code,
            metadata: ContainerMetadata {
                container_type,
                input_count: 0,
                output_count: 1,
                foldable: true,
            },
        }
    }

    /// 从原始机器码创建容器（绑定命名）
    pub fn bind(name: &str, raw_machine_code: Vec<u8>) -> Self {
        // 分析机器码，自动推断容器类型
        let container_type = Self::infer_type(&raw_machine_code);
        Self::new(name, raw_machine_code, container_type)
    }

    /// 推断机器码的容器类型
    fn infer_type(code: &[u8]) -> ContainerType {
        match code.first() {
            // 0x00-0x3F: 加法类指令
            Some(0x00..=0x3F) => ContainerType::Addition,
            // 0x40-0x7F: 数据传输类（螺旋/旋转）
            Some(0x40..=0x7F) => ContainerType::Spiral,
            // 0x80-0xBF: 逻辑运算类（镜像/对称）
            Some(0x80..=0xBF) => ContainerType::Mirror,
            // 0xC0-0xFF: 控制流类（折叠/进位）
            Some(0xC0..=0xFF) => ContainerType::Fold,
            _ => ContainerType::Addition,
        }
    }

    /// 获取容器名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取机器码
    pub fn code(&self) -> &[u8] {
        &self.code
    }

    /// 获取容器类型
    pub fn container_type(&self) -> ContainerType {
        self.metadata.container_type
    }

    /// 容器长度（机器码字节数）
    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}

/// 机器码进制：多个容器的组合
/// 
/// 这就是"进制"：
/// - 二进制：位 = 0或1
/// - 机器码进制：位 = 一个命名的容器
#[derive(Debug)]
pub struct MachineCodeRadix {
    /// 进制位列表（每个位都是一个容器）
    digits: Vec<MachineCodeContainer>,
    /// 进制基数（254+2=256）
    radix: u16,
    /// 当前层号
    layer: u8,
}

impl MachineCodeRadix {
    /// 创建254+2=256进制
    pub fn new() -> Self {
        Self {
            digits: Vec::new(),
            radix: 256, // 254容器 + 2锚点
            layer: 0,
        }
    }

    /// 添加一个容器（进制位）
    pub fn push(&mut self, container: MachineCodeContainer) {
        self.digits.push(container);
    }

    /// 获取指定位置的容器
    pub fn get(&self, index: usize) -> Option<&MachineCodeContainer> {
        self.digits.get(index)
    }

    /// 容器数量
    pub fn len(&self) -> usize {
        self.digits.len()
    }

    /// 生成完整的机器码序列（所有容器拼接）
    pub fn emit(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for digit in &self.digits {
            result.extend_from_slice(&digit.code);
        }
        result
    }

    /// 折叠进位：当容器数量达到254时，溢出到下一层
    pub fn fold(&mut self) -> bool {
        if self.digits.len() >= 254 {
            self.layer += 1;
            true // 发生了折叠
        } else {
            false
        }
    }

    /// 获取当前层号
    pub fn layer(&self) -> u8 {
        self.layer
    }
}

/// 预定义的宏指令集（容器库）
pub mod preset {
    use super::*;

    /// 加法容器：ADD reg, val
    pub fn container_add(register: u8, value: u8) -> MachineCodeContainer {
        // x86: ADD register, immediate
        let code = vec![0x80, 0xC0 | register, value];
        MachineCodeContainer::new("container_add", code, ContainerType::Addition)
    }

    /// 螺旋容器：ROTATE reg, angle
    pub fn container_spiral(register: u8, angle: u8) -> MachineCodeContainer {
        // x86: ROL register, angle
        let code = vec![0xC1, 0xC0 | register, angle];
        MachineCodeContainer::new("container_spiral", code, ContainerType::Spiral)
    }

    /// 镜像容器：XOR reg, mask（对称翻转）
    pub fn container_mirror(register: u8, mask: u8) -> MachineCodeContainer {
        // x86: XOR register, mask
        let code = vec![0x80, 0xF0 | register, mask];
        MachineCodeContainer::new("container_mirror", code, ContainerType::Mirror)
    }

    /// 折叠容器：CALL label（进位/层叠）
    pub fn container_fold(label_addr: u32) -> MachineCodeContainer {
        // x86: CALL rel32
        let code = vec![
            0xE8,
            (label_addr & 0xFF) as u8,
            ((label_addr >> 8) & 0xFF) as u8,
            ((label_addr >> 16) & 0xFF) as u8,
            ((label_addr >> 24) & 0xFF) as u8,
        ];
        MachineCodeContainer::new("container_fold", code, ContainerType::Fold)
    }

    /// 颜色容器：设置颜色（色系, 相位, 亮度）
    pub fn container_color(hue: u8, phase: u8, brightness: u8) -> MachineCodeContainer {
        // 自定义编码：0xCC = Color Container
        let code = vec![0xCC, hue, phase, brightness];
        MachineCodeContainer::new("container_color", code, ContainerType::Color)
    }

    /// 声母容器：设置发音位置
    pub fn container_shengmu(position: u8) -> MachineCodeContainer {
        // 自定义编码：0x53 = 'S' = Shengmu Set
        let code = vec![0x53, position];
        MachineCodeContainer::new("container_shengmu", code, ContainerType::Shengmu)
    }

    /// 韵母容器：设置口腔形状
    pub fn container_yunmu(shape: u8) -> MachineCodeContainer {
        // 自定义编码：0x59 = 'Y' = Yunmu Set
        let code = vec![0x59, shape];
        MachineCodeContainer::new("container_yunmu", code, ContainerType::Yunmu)
    }

    /// 声调容器：设置音高
    pub fn container_tone(pitch: u8) -> MachineCodeContainer {
        // 自定义编码：0x54 = 'T' = Tone Set
        let code = vec![0x54, pitch];
        MachineCodeContainer::new("container_tone", code, ContainerType::Tone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let add = preset::container_add(0, 10);
        assert_eq!(add.name(), "container_add");
        assert_eq!(add.len(), 3);
        assert_eq!(add.container_type(), ContainerType::Addition);
    }

    #[test]
    fn test_radix_emit() {
        let mut radix = MachineCodeRadix::new();
        radix.push(preset::container_add(0, 10));
        radix.push(preset::container_spiral(1, 90));
        radix.push(preset::container_mirror(2, 0xFF));

        let code = radix.emit();
        assert_eq!(code.len(), 9); // 3 + 3 + 3
    }

    #[test]
    fn test_fold() {
        let mut radix = MachineCodeRadix::new();
        for i in 0..254 {
            radix.push(preset::container_add(i as u8, 1));
        }
        assert_eq!(radix.len(), 254);
        assert!(radix.fold()); // 应该发生折叠
        assert_eq!(radix.layer(), 1);
    }

    #[test]
    fn test_color_container() {
        let color = preset::container_color(0, 0x12, 0x80);
        assert_eq!(color.name(), "container_color");
        assert_eq!(color.code(), &[0xCC, 0, 0x12, 0x80]);
    }
}
