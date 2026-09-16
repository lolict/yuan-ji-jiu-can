// 冰箱层 — 时间触发执行引擎
// 消灭运行时，所有行为编译时预编排
// 执行时只查表、裁决、取袋、输出
//
// 容器包布局（与约定对齐）：
// pack_id:8B  func_code:2B  param_slot:8B  time_stamp:4B  align_flag:1B  payload:NB  witness:3B
// payload 4B 对齐（不足补零）

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::vocabulary::*;
use crate::cross_isa::compose::Sentence;

/// 容器包：存放在冰箱格子里的预编码指令
/// 二进制布局与约定一致
#[derive(Debug, Clone)]
pub struct ContainerPack {
    pub pack_id: u64,          // 8B 公约数
    pub func_code: u16,        // 2B X坐标/功能ID
    pub param_slot: u64,       // 8B Y坐标/参数
    pub time_stamp: u32,       // 4B Z坐标/时间窗
    pub align_flag: u8,        // 1B 相位对齐
    pub payload: Vec<u8>,      // NB 预编码动作序列（4B对齐）
    pub witness: [u8; 3],      // 3B 三元裁决签名
}

impl ContainerPack {
    /// 从Coord3D和原生机器码创建容器包
    pub fn new(pack_id: u64, coord: Coord3D, align_flag: u8, payload: Vec<u8>) -> Self {
        let aligned_payload = Self::align_payload(payload);
        let witness = Self::compute_witness(pack_id, coord, align_flag, &aligned_payload);
        Self {
            pack_id,
            func_code: coord.x as u16,
            param_slot: coord.y as u64,
            time_stamp: coord.z as u32,
            align_flag,
            payload: aligned_payload,
            witness,
        }
    }

    /// payload 4B 对齐（不足补零）
    fn align_payload(mut payload: Vec<u8>) -> Vec<u8> {
        let rem = payload.len() % 4;
        if rem != 0 {
            let pad = 4 - rem;
            payload.extend(std::iter::repeat(0).take(pad));
        }
        payload
    }

    /// 计算三元见证签名
    /// 空袋: pack_id==0xFFFF → 全1标记
    /// 全零: payload全零但pack_id有效 → [0xFF, 0xFE, 0xFD] 陷阱标记
    /// 有效: [sum%251, xor, (sum>>8)%241]
    pub fn compute_witness(pack_id: u64, coord: Coord3D, align_flag: u8, payload: &[u8]) -> [u8; 3] {
        if pack_id == 0xFFFF {
            return [0xFF, 0xFF, 0xFF]; // 空袋
        }
        if payload.iter().all(|&b| b == 0) {
            return [0xFF, 0xFE, 0xFD]; // 全零陷阱
        }
        let sum: u64 = payload.iter().map(|&b| b as u64).sum();
        let xor: u8 = payload.iter().fold(0u8, |a, &b| a ^ b);
        let cksum = ((coord.x as u16) << 8 | coord.z as u16) as u8;
        [
            (sum % 251) as u8,
            xor,
            ((sum >> 8) as u8).wrapping_add(cksum).wrapping_add(align_flag),
        ]
    }

    /// 验证见证签名
    pub fn verify_witness(&self) -> bool {
        // 1. 空袋拒绝
        if self.pack_id == 0xFFFF { return false; }
        // 2. 全零payload拒绝（陷阱状态）
        if self.payload.iter().all(|&b| b == 0) && self.payload.len() >= 4 {
            return false;
        }
        let expected = Self::compute_witness(self.pack_id, self.coord(), self.align_flag, &self.payload);
        self.witness == expected
    }

    /// 解包恢复坐标
    pub fn coord(&self) -> Coord3D {
        Coord3D::new(self.func_code as u8, self.param_slot as u16, self.time_stamp as u8)
    }

    /// 获取未填充的 payload 长度（去除对齐补零）
    pub fn payload_len(&self) -> usize {
        // 找到最后一个非零字节的位置（简化实现）
        self.payload.iter().rposition(|&b| b != 0).map(|p| p + 1).unwrap_or(0)
    }
}

/// 裁决结果
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Admit,   // 准入
    Reject,  // 拒绝
    Wait,    // 等待（Z轴未对齐）
}

/// 三元裁决引擎
pub struct TriuneJudge;

impl TriuneJudge {
    /// 裁决是否允许取袋
    pub fn judge(pack: &ContainerPack, current_z: u8) -> Verdict {
        // 1. 空袋判定
        if pack.pack_id == 0xFFFF {
            return Verdict::Reject;
        }
        // 2. Z轴未对齐
        if pack.time_stamp != current_z as u32 && pack.time_stamp != 0xFF {
            return Verdict::Wait;
        }
        // 3. 见证验证
        if pack.verify_witness() {
            Verdict::Admit
        } else {
            Verdict::Reject
        }
    }
}

/// 冰箱：容器包的三维存储阵列
pub struct Fridge {
    storage: std::collections::HashMap<u32, ContainerPack>,
}

impl Fridge {
    pub fn new() -> Self {
        Self { storage: std::collections::HashMap::new() }
    }

    /// 装袋：把容器包放入冰箱
    pub fn store(&mut self, pack: ContainerPack) {
        let addr = pack.coord().to_address();
        self.storage.insert(addr, pack);
    }

    /// 按32位地址取袋
    pub fn fetch(&self, addr: u32) -> Option<&ContainerPack> {
        self.storage.get(&addr)
    }

    /// 按坐标裁决并取袋（核心操作）
    pub fn fetch_judged(&self, coord: Coord3D, current_z: u8) -> Result<&ContainerPack, Verdict> {
        let addr = coord.to_address();
        match self.storage.get(&addr) {
            None => Err(Verdict::Reject),
            Some(pack) => {
                match TriuneJudge::judge(pack, current_z) {
                    Verdict::Admit => Ok(pack),
                    Verdict::Reject => Err(Verdict::Reject),
                    Verdict::Wait => Err(Verdict::Wait),
                }
            }
        }
    }

    /// 按Z轴时间窗口批量取出（同一时间段的所有包）
    pub fn fetch_time_slot(&self, z: u8) -> Vec<&ContainerPack> {
        self.storage.values()
            .filter(|p| p.time_stamp == z as u32 && TriuneJudge::judge(p, z) == Verdict::Admit)
            .collect()
    }
}

impl Default for Fridge {
    fn default() -> Self { Self::new() }
}

/// 菜单索引 — 用户可见的入口
pub struct MenuIndex;

impl MenuIndex {
    /// 把语义坐标映射为用户友好的菜单名称（ASCII art）
    pub fn describe(coord: Coord3D) -> &'static str {
        match coord.x {
            0x01 => "[加载盒体] → 内存到寄存器",
            0x02 => "[存储盒体] → 寄存器到内存",
            0x03 => "[加法球体] → 两个数相加",
            0x04 => "[加法球体] → 加立即数",
            0x05 => "[减法球体] → 两数相减",
            0x06 => "[与门双锥] → 按位与",
            0x07 => "[或门双锥] → 按位或",
            0x08 => "[异或双锥] → 按位异或",
            0x09 => "[跳转环面] → 无条件跳转",
            0x0A => "[分支环面] → 不等跳转",
            0x0B => "[分支环面] → 相等跳转",
            0x0C => "[调用管体] → 函数调用",
            0x0D => "[返回管体] → 函数返回",
            0x0E => "[比较双锥] → 比较两数",
            0x0F => "[加载球体] → 立即数传送",
            _    => "[未定义] → 保留/扩展",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_verify_valid() {
        let coord = Coord3D::new(0x03, 0x0001, 0x01);
        let payload = vec![0x33, 0x10, 0x31, 0x00];
        let pack = ContainerPack::new(1, coord, 0, payload);
        assert!(pack.verify_witness());
    }

    #[test]
    fn pack_verify_zero_payload_rejected() {
        let coord = Coord3D::new(0x03, 0x0001, 0x01);
        let payload = vec![0; 4];
        let pack = ContainerPack::new(5, coord, 0, payload);
        // 全零但id有效 → 拒绝
        assert!(!pack.verify_witness());
    }

    #[test]
    fn pack_verify_empty_id_rejected() {
        let coord = Coord3D::new(0x03, 0x0001, 0x01);
        let payload = vec![0x33, 0x10, 0x31, 0x00];
        let pack = ContainerPack::new(0xFFFF, coord, 0, payload);
        assert!(!pack.verify_witness());
    }

    #[test]
    fn pack_alignment() {
        // 5 字节 payload 应该填充到 8 字节
        let coord = Coord3D::new(0x03, 0x0001, 0x01);
        let payload = vec![0x33, 0x10, 0x31, 0x00, 0x55]; // 5 字节
        let pack = ContainerPack::new(1, coord, 0, payload);
        assert!(pack.payload.len() % 4 == 0);
        assert_eq!(pack.payload.len(), 8);
    }

    #[test]
    fn pack_fields_match_coord() {
        let coord = Coord3D::new(0x03, 0x0A10, 0x5F);
        let payload = vec![0x33, 0x10, 0x31, 0x00];
        let pack = ContainerPack::new(42, coord, 7, payload);
        assert_eq!(pack.func_code, 0x03);
        assert_eq!(pack.param_slot, 0x0A10);
        assert_eq!(pack.time_stamp, 0x5F);
        assert_eq!(pack.align_flag, 7);
        assert_eq!(pack.pack_id, 42);
    }

    #[test]
    fn fridge_fetch_judged() {
        let mut fridge = Fridge::new();
        let coord = Coord3D::new(0x03, 0x0001, 0x01);
        let payload = vec![0x33, 0x10, 0x31, 0x00];
        let pack = ContainerPack::new(1, coord, 0, payload);
        fridge.store(pack);

        let result = fridge.fetch_judged(coord, 0x01);
        assert!(result.is_ok());

        let result = fridge.fetch_judged(coord, 0x02);
        assert!(matches!(result, Err(Verdict::Wait)));
    }

    #[test]
    fn menu_describe() {
        assert!(MenuIndex::describe(Coord3D::new(0x03, 0, 0)).contains("加法"));
    }
}
