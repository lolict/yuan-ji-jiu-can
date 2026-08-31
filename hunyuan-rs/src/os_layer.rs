//! Cocoon OS 抽象层
//! 
//! 核心原则：核心逻辑与 OS 分离
//! 
//! 架构：
//! 核心逻辑（纯逻辑，无 OS 依赖）
//!     ↓
//! OS 抽象层（trait 定义）
//!     ↓
//! ├── Linux 后端
//! ├── Windows 后端
//! ├── macOS 后端
//! └── 裸机后端（最终目标）

use std::fmt::Debug;

/// OS 抽象层 trait
/// 
/// 核心逻辑只依赖这个 trait，不直接调用任何 OS API
pub trait OsLayer: Send + Sync + Debug {
    /// 分配内存
    fn alloc(&self, size: usize) -> *mut u8;
    
    /// 释放内存
    fn dealloc(&self, ptr: *mut u8, size: usize);
    
    /// 读取文件
    fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError>;
    
    /// 写入文件
    fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError>;
    
    /// 删除文件
    fn file_delete(&self, path: &str) -> Result<(), OsError>;
    
    /// 检查文件是否存在
    fn file_exists(&self, path: &str) -> bool;
    
    /// 获取当前时间（毫秒）
    fn time_millis(&self) -> u64;
    
    /// 网络发送
    fn net_send(&self, host: &str, port: u16, data: &[u8]) -> Result<(), OsError>;
    
    /// 网络接收
    fn net_recv(&self, host: &str, port: u16, buf: &mut [u8]) -> Result<usize, OsError>;
    
    /// 打印到控制台
    fn print(&self, msg: &str);
    
    /// 获取平台名称
    fn platform(&self) -> &str;

    /// 获取高精度性能计数器（纳秒）
    /// 用于测量操作耗时，不保证与 wall-clock 对齐
    fn perf_counter_ns(&self) -> u64;
}

/// OS 错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum OsError {
    /// 文件未找到
    FileNotFound(String),
    /// 权限不足
    PermissionDenied(String),
    /// 网络错误
    NetworkError(String),
    /// 内存分配失败
    AllocFailed,
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for OsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(p) => write!(f, "文件未找到: {}", p),
            Self::PermissionDenied(p) => write!(f, "权限不足: {}", p),
            Self::NetworkError(e) => write!(f, "网络错误: {}", e),
            Self::AllocFailed => write!(f, "内存分配失败"),
            Self::Other(e) => write!(f, "错误: {}", e),
        }
    }
}

impl std::error::Error for OsError {}

// ────────────────────────────────────────────────────────────
// Linux 后端
// ────────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
pub mod linux {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    pub struct LinuxOs;

    impl LinuxOs {
        pub fn new() -> Self {
            Self
        }
    }

    impl OsLayer for LinuxOs {
        fn alloc(&self, size: usize) -> *mut u8 {
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            unsafe { std::alloc::alloc(layout) }
        }

        fn dealloc(&self, ptr: *mut u8, size: usize) {
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            unsafe { std::alloc::dealloc(ptr, layout) }
        }

        fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError> {
            fs::read(path).map_err(|e| OsError::FileNotFound(e.to_string()))
        }

        fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError> {
            fs::write(path, data).map_err(|e| OsError::Other(e.to_string()))
        }

        fn file_delete(&self, path: &str) -> Result<(), OsError> {
            fs::remove_file(path).map_err(|e| OsError::FileNotFound(e.to_string()))
        }

        fn file_exists(&self, path: &str) -> bool {
            std::path::Path::new(path).exists()
        }

        fn time_millis(&self) -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        }

        fn net_send(&self, host: &str, port: u16, data: &[u8]) -> Result<(), OsError> {
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|e| OsError::NetworkError(e.to_string()))?;
            stream.write_all(data)
                .map_err(|e| OsError::NetworkError(e.to_string()))
        }

        fn net_recv(&self, host: &str, port: u16, buf: &mut [u8]) -> Result<usize, OsError> {
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|e| OsError::NetworkError(e.to_string()))?;
            stream.read(buf)
                .map_err(|e| OsError::NetworkError(e.to_string()))
        }

        fn print(&self, msg: &str) {
            println!("{}", msg);
        }

        fn platform(&self) -> &str {
            "Linux"
        }

        fn perf_counter_ns(&self) -> u64 {
            // 使用 SystemTime 获取纳秒级时间戳
            // 在 OS 层实现中可替换为 clock_gettime(CLOCK_MONOTONIC) 等
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        }
    }
}

// ────────────────────────────────────────────────────────────
// Windows 后端
// ────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
pub mod windows {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    pub struct WindowsOs;

    impl WindowsOs {
        pub fn new() -> Self {
            Self
        }
    }

    impl OsLayer for WindowsOs {
        fn alloc(&self, size: usize) -> *mut u8 {
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            unsafe { std::alloc::alloc(layout) }
        }

        fn dealloc(&self, ptr: *mut u8, size: usize) {
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            unsafe { std::alloc::dealloc(ptr, layout) }
        }

        fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError> {
            fs::read(path).map_err(|e| OsError::FileNotFound(e.to_string()))
        }

        fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError> {
            fs::write(path, data).map_err(|e| OsError::Other(e.to_string()))
        }

        fn file_delete(&self, path: &str) -> Result<(), OsError> {
            fs::remove_file(path).map_err(|e| OsError::FileNotFound(e.to_string()))
        }

        fn file_exists(&self, path: &str) -> bool {
            std::path::Path::new(path).exists()
        }

        fn time_millis(&self) -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        }

        fn net_send(&self, host: &str, port: u16, data: &[u8]) -> Result<(), OsError> {
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|e| OsError::NetworkError(e.to_string()))?;
            stream.write_all(data)
                .map_err(|e| OsError::NetworkError(e.to_string()))
        }

        fn net_recv(&self, host: &str, port: u16, buf: &mut [u8]) -> Result<usize, OsError> {
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|e| OsError::NetworkError(e.to_string()))?;
            stream.read(buf)
                .map_err(|e| OsError::NetworkError(e.to_string()))
        }

        fn print(&self, msg: &str) {
            println!("{}", msg);
        }

        fn platform(&self) -> &str {
            "Windows"
        }

        fn perf_counter_ns(&self) -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        }
    }
}

// ────────────────────────────────────────────────────────────
// macOS 后端
// ────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
pub mod macos {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    pub struct MacOs;

    impl MacOs {
        pub fn new() -> Self {
            Self
        }
    }

    impl OsLayer for MacOs {
        fn alloc(&self, size: usize) -> *mut u8 {
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            unsafe { std::alloc::alloc(layout) }
        }

        fn dealloc(&self, ptr: *mut u8, size: usize) {
            let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
            unsafe { std::alloc::dealloc(ptr, layout) }
        }

        fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError> {
            fs::read(path).map_err(|e| OsError::FileNotFound(e.to_string()))
        }

        fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError> {
            fs::write(path, data).map_err(|e| OsError::Other(e.to_string()))
        }

        fn file_delete(&self, path: &str) -> Result<(), OsError> {
            fs::remove_file(path).map_err(|e| OsError::FileNotFound(e.to_string()))
        }

        fn file_exists(&self, path: &str) -> bool {
            std::path::Path::new(path).exists()
        }

        fn time_millis(&self) -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        }

        fn net_send(&self, host: &str, port: u16, data: &[u8]) -> Result<(), OsError> {
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|e| OsError::NetworkError(e.to_string()))?;
            stream.write_all(data)
                .map_err(|e| OsError::NetworkError(e.to_string()))
        }

        fn net_recv(&self, host: &str, port: u16, buf: &mut [u8]) -> Result<usize, OsError> {
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|e| OsError::NetworkError(e.to_string()))?;
            stream.read(buf)
                .map_err(|e| OsError::NetworkError(e.to_string()))
        }

        fn print(&self, msg: &str) {
            println!("{}", msg);
        }

        fn platform(&self) -> &str {
            "macOS"
        }

        fn perf_counter_ns(&self) -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        }
    }
}

// ────────────────────────────────────────────────────────────
// 裸机后端（无 OS，直接控制硬件）
// ────────────────────────────────────────────────────────────

pub mod bare_metal {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct BareMetalOs;

    impl BareMetalOs {
        pub fn new() -> Self {
            Self
        }
    }

    impl OsLayer for BareMetalOs {
        fn alloc(&self, size: usize) -> *mut u8 {
            // 裸机：从预分配的内存池中分配
            unsafe {
                static mut MEMORY: [u8; 1024 * 1024] = [0; 1024 * 1024]; // 1MB
                static mut OFFSET: usize = 0;
                let ptr = MEMORY.as_mut_ptr().add(OFFSET);
                OFFSET += size;
                ptr
            }
        }

        fn dealloc(&self, _ptr: *mut u8, _size: usize) {
            // 裸机：不释放，内存池预分配
        }

        fn file_read(&self, _path: &str) -> Result<Vec<u8>, OsError> {
            Err(OsError::Other("裸机模式不支持文件系统".to_string()))
        }

        fn file_write(&self, _path: &str, _data: &[u8]) -> Result<(), OsError> {
            Err(OsError::Other("裸机模式不支持文件系统".to_string()))
        }

        fn file_delete(&self, _path: &str) -> Result<(), OsError> {
            Err(OsError::Other("裸机模式不支持文件系统".to_string()))
        }

        fn file_exists(&self, _path: &str) -> bool {
            false
        }

        fn time_millis(&self) -> u64 {
            // 裸机：返回固定模拟时间戳 (2024-01-01 00:00:00 UTC in millis)
            1_704_067_200_000
        }

        fn net_send(&self, _host: &str, _port: u16, _data: &[u8]) -> Result<(), OsError> {
            Err(OsError::Other("裸机模式不支持网络".to_string()))
        }

        fn net_recv(&self, _host: &str, _port: u16, _buf: &mut [u8]) -> Result<usize, OsError> {
            Err(OsError::Other("裸机模式不支持网络".to_string()))
        }

        fn print(&self, msg: &str) {
            // 裸机：写入串口或显存
            unsafe {
                let vga = 0xB8000 as *mut u8;
                for (i, byte) in msg.bytes().enumerate() {
                    *vga.add(i * 2) = byte;
                    *vga.add(i * 2 + 1) = 0x0F; // 白色
                }
            }
        }

        fn platform(&self) -> &str {
            "Bare Metal"
        }

        fn perf_counter_ns(&self) -> u64 {
            // 裸机：返回 time_millis() * 1_000_000 作为纳秒近似值
            self.time_millis() * 1_000_000
        }
    }
}

// ────────────────────────────────────────────────────────────
// 便捷函数：自动选择当前平台的 OS 层
// ────────────────────────────────────────────────────────────

/// 创建当前平台的 OS 层
pub fn current_os() -> Box<dyn OsLayer> {
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxOs::new())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsOs::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOs::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Box::new(bare_metal::BareMetalOs::new())
    }
}

// 为 Box<dyn OsLayer> 实现 OsLayer trait，使得可以直接使用 &os 调用
impl OsLayer for Box<dyn OsLayer> {
    fn alloc(&self, size: usize) -> *mut u8 {
        (**self).alloc(size)
    }
    fn dealloc(&self, ptr: *mut u8, size: usize) {
        (**self).dealloc(ptr, size)
    }
    fn time_millis(&self) -> u64 {
        (**self).time_millis()
    }
    fn platform(&self) -> &str {
        (**self).platform()
    }
    fn print(&self, msg: &str) {
        (**self).print(msg)
    }
    fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError> {
        (**self).file_read(path)
    }
    fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError> {
        (**self).file_write(path, data)
    }
    fn file_delete(&self, path: &str) -> Result<(), OsError> {
        (**self).file_delete(path)
    }
    fn file_exists(&self, path: &str) -> bool {
        (**self).file_exists(path)
    }
    fn net_send(&self, host: &str, port: u16, data: &[u8]) -> Result<(), OsError> {
        (**self).net_send(host, port, data)
    }
    fn net_recv(&self, host: &str, port: u16, buf: &mut [u8]) -> Result<usize, OsError> {
        (**self).net_recv(host, port, buf)
    }
    fn perf_counter_ns(&self) -> u64 {
        (**self).perf_counter_ns()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_os() {
        let os = current_os();
        let platform = os.platform();
        assert!(["Linux", "Windows", "macOS", "Bare Metal"].contains(&platform));
    }

    #[test]
    fn test_os_error_display() {
        let err = OsError::FileNotFound("test.txt".to_string());
        assert_eq!(err.to_string(), "文件未找到: test.txt");
    }

    // 注意：裸机测试需要在裸机环境下运行
    // 在 OS 用户空间无法测试 VGA 显存写入
    // #[test]
    // fn test_bare_metal_print() {
    //     let os = bare_metal::BareMetalOs::new();
    //     os.print("hello"); // 写入 VGA 显存
    // }
}
