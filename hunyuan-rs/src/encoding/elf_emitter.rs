//! ELF 可执行文件生成器
//! 
//! 目标：一次性编译，终身执行（无运行时）
//! 
//! 流程：
//! 机器码容器 → 拼接 → ELF头 → 可执行文件

use crate::macro_isa::{MachineCodeRadix, MachineCodeContainer};

/// ELF 文件头（64位）
#[derive(Debug)]
pub struct ElfHeader {
    /// ELF 魔数
    magic: [u8; 4],
    /// 类别（64位）
    class: u8,
    /// 数据（小端）
    data: u8,
    /// 版本（e_ident 中的 EI_VERSION）
    version: u8,
    /// OS/ABI
    os_abi: u8,
    /// 文件类型（ET_EXEC = 可执行文件）
    type_: u16,
    /// 机器（x86-64）
    machine: u16,
    /// e_version（4字节）
    e_version: u32,
    /// 入口点地址
    entry: u64,
    /// 程序头表偏移
    phoff: u64,
    /// 节头表偏移
    shoff: u64,
    /// 程序头数量
    phnum: u16,
    /// 节头数量
    shnum: u16,
}

impl ElfHeader {
    /// 创建64位ELF头
    pub fn new(entry: u64, phnum: u16, shnum: u16) -> Self {
        Self {
            magic: [0x7F, b'E', b'L', b'F'],
            class: 2,      // 64位
            data: 1,       // 小端
            version: 1,
            os_abi: 0,     // System V
            type_: 2,      // ET_EXEC
            machine: 0x3E, // x86-64
            e_version: 1,  // ELF 版本
            entry,
            phoff: 64,     // 程序头在64字节处
            shoff: 0,      // 无节头
            phnum,
            shnum,
        }
    }

    /// 序列化为字节（64字节）
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64);
        bytes.extend_from_slice(&self.magic);       // 4
        bytes.push(self.class);                      // 1
        bytes.push(self.data);                       // 1
        bytes.push(self.version);                    // 1
        bytes.push(self.os_abi);                     // 1
        bytes.extend_from_slice(&[0; 8]);            // 8 (padding)
        bytes.extend_from_slice(&self.type_.to_le_bytes());   // 2
        bytes.extend_from_slice(&self.machine.to_le_bytes()); // 2
        bytes.extend_from_slice(&self.e_version.to_le_bytes()); // 4
        bytes.extend_from_slice(&self.entry.to_le_bytes());   // 8
        bytes.extend_from_slice(&self.phoff.to_le_bytes());   // 8
        bytes.extend_from_slice(&self.shoff.to_le_bytes());   // 8
        bytes.extend_from_slice(&[0; 4]);                     // 4 (flags)
        bytes.extend_from_slice(&64u16.to_le_bytes());        // 2 (ehsize)
        bytes.extend_from_slice(&56u16.to_le_bytes());        // 2 (phentsize)
        bytes.extend_from_slice(&self.phnum.to_le_bytes());   // 2
        bytes.extend_from_slice(&64u16.to_le_bytes());        // 2 (shentsize)
        bytes.extend_from_slice(&self.shnum.to_le_bytes());   // 2
        bytes.extend_from_slice(&0u16.to_le_bytes());         // 2 (shstrndx)
        // 总计: 4+1+1+1+1+8+2+2+4+8+8+8+4+2+2+2+2+2+2 = 64
        bytes
    }
}

/// 程序头（64位）
#[derive(Debug)]
pub struct ProgramHeader {
    /// 段类型（PT_LOAD = 可加载）
    type_: u32,
    /// 标志（读+执行）
    flags: u32,
    /// 文件偏移
    offset: u64,
    /// 虚拟地址
    vaddr: u64,
    /// 物理地址
    paddr: u64,
    /// 文件大小
    filesz: u64,
    /// 内存大小
    memsz: u64,
    /// 对齐
    align: u64,
}

impl ProgramHeader {
    /// 创建可加载段
    pub fn load(offset: u64, vaddr: u64, size: u64) -> Self {
        Self {
            type_: 1,       // PT_LOAD
            flags: 5,       // PF_R | PF_X
            offset,
            vaddr,
            paddr: vaddr,
            filesz: size,
            memsz: size,
            align: 0x1000,
        }
    }

    /// 序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(56);
        bytes.extend_from_slice(&self.type_.to_le_bytes());
        bytes.extend_from_slice(&self.flags.to_le_bytes());
        bytes.extend_from_slice(&self.offset.to_le_bytes());
        bytes.extend_from_slice(&self.vaddr.to_le_bytes());
        bytes.extend_from_slice(&self.paddr.to_le_bytes());
        bytes.extend_from_slice(&self.filesz.to_le_bytes());
        bytes.extend_from_slice(&self.memsz.to_le_bytes());
        bytes.extend_from_slice(&self.align.to_le_bytes());
        bytes
    }
}

/// ELF 发射器
/// 
/// 把机器码容器进制变成真正的 ELF 可执行文件
pub struct ElfEmitter {
    /// 机器码进制
    radix: MachineCodeRadix,
    /// 入口地址
    entry_addr: u64,
}

impl ElfEmitter {
    /// 创建新的ELF发射器
    pub fn new(radix: MachineCodeRadix) -> Self {
        Self {
            radix,
            entry_addr: 0x400000, // 默认入口地址
        }
    }

    /// 设置入口地址
    pub fn entry(mut self, addr: u64) -> Self {
        self.entry_addr = addr;
        self
    }

    /// 生成 ELF 可执行文件
    pub fn emit(&self) -> Vec<u8> {
        // 1. 拼接所有机器码
        let code = self.radix.emit();
        let code_size = code.len() as u64;

        // 2. 创建 ELF 头
        let header = ElfHeader::new(
            self.entry_addr + 120, // 入口 = 头后
            1,                     // 1个程序头
            0,                     // 无节头
        );

        // 3. 创建程序头
        let phdr = ProgramHeader::load(
            120,                 // 代码在文件中的偏移
            self.entry_addr + 120, // 虚拟地址
            code_size,
        );

        // 4. 组合
        let mut elf = Vec::new();
        elf.extend_from_slice(&header.to_bytes());
        elf.extend_from_slice(&phdr.to_bytes());
        elf.extend_from_slice(&code);

        elf
    }

    /// 保存到文件
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let bytes = self.emit();
        std::fs::write(path, bytes)
    }
}

/// 便捷函数：从容器列表生成ELF
pub fn emit_elf(containers: Vec<MachineCodeContainer>) -> Vec<u8> {
    let mut radix = MachineCodeRadix::new();
    for c in containers {
        radix.push(c);
    }
    ElfEmitter::new(radix).emit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_isa::preset;

    #[test]
    fn test_elf_header() {
        let header = ElfHeader::new(0x400000, 1, 0);
        let bytes = header.to_bytes();
        assert_eq!(bytes.len(), 64);
        assert_eq!(&bytes[0..4], &[0x7F, b'E', b'L', b'F']);
    }

    #[test]
    fn test_program_header() {
        let phdr = ProgramHeader::load(120, 0x400000, 100);
        let bytes = phdr.to_bytes();
        assert_eq!(bytes.len(), 56);
    }

    #[test]
    fn test_elf_emitter() {
        let mut radix = MachineCodeRadix::new();
        radix.push(preset::container_add(0, 10));
        radix.push(preset::container_spiral(1, 90));

        let emitter = ElfEmitter::new(radix);
        let elf = emitter.emit();

        // 验证ELF魔数
        assert_eq!(&elf[0..4], &[0x7F, b'E', b'L', b'F']);
        // 验证文件大小
        assert!(elf.len() > 120);
    }

    #[test]
    fn test_emit_elf() {
        let containers = vec![
            preset::container_add(0, 10),
            preset::container_mirror(2, 0xFF),
        ];
        let elf = emit_elf(containers);
        assert_eq!(&elf[0..4], &[0x7F, b'E', b'L', b'F']);
    }
}
