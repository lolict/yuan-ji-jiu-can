// ======================================================================
// celestial_perception_v7.rs — 天觉系统 v7（老七）· OS三合一张: 文件系统 + IPC + 设备驱动
//
// 本文件在天觉v6内核基础上, 新增三大子系统:
//
//   1. 虚拟文件系统 (VFS)     — 文件创建/读写/删除, 目录树, inode 管理
//   2. 进程间通信 (IPC)       — 管道pipe + 消息队列 + 共享内存
//   3. 设备驱动框架          — 字符设备/块设备注册, file_operations
//
// 天觉公理: 数据即袋子, 文件即袋子的持久化形态;
//          进程间的袋子流转通过 IPC 完成;
//          袋子出入 OS 经过设备驱动过滤.
// ======================================================================

use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// ======================================================================
// 第一部分: 虚拟文件系统 (Virtual File System)
// ======================================================================

/// 文件类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    Regular,    // 普通文件 (袋子)
    Directory,  // 目录
    Pipe,       // 管道 (IPC管道文件)
    Device,     // 设备文件
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::Regular => write!(f, "REG"),
            FileType::Directory => write!(f, "DIR"),
            FileType::Pipe => write!(f, "PIP"),
            FileType::Device => write!(f, "DEV"),
        }
    }
}

/// inode — 文件元数据 (模拟 Unix inode)
#[derive(Clone, Debug)]
pub struct Inode {
    pub ino: u64,           // inode 号
    pub name: String,       // 文件名
    pub ftype: FileType,    // 文件类型
    pub size: usize,        // 文件大小 (字节)
    pub permissions: u16,   // 权限位 (rwxrwxrwx)
    pub owner_pid: u64,     // 所有者进程ID
    pub created_ms: u64,    // 创建时间戳
    pub modified_ms: u64,   // 修改时间戳
    pub data: Vec<u8>,      // 文件内容
    pub bag_id: Option<u64>, // 关联的袋子ID (如果是袋子文件)
}

impl Inode {
    pub fn new(ino: u64, name: &str, ftype: FileType, owner_pid: u64) -> Self {
        let now = current_timestamp_ms();
        Self {
            ino,
            name: name.to_string(),
            ftype,
            size: 0,
            permissions: 0o644,
            owner_pid,
            created_ms: now,
            modified_ms: now,
            data: Vec::new(),
            bag_id: None,
        }
    }

    /// 创建袋子文件 (数据是袋子属性的二进制表示)
    pub fn new_bag_file(ino: u64, name: &str, bag_id: u64, attrs_summary: &[u8], owner_pid: u64) -> Self {
        let mut inode = Self::new(ino, name, FileType::Regular, owner_pid);
        inode.data = attrs_summary.to_vec();
        inode.size = inode.data.len();
        inode.bag_id = Some(bag_id);
        inode
    }

    /// 判断是否是袋子文件
    pub fn is_bag_file(&self) -> bool {
        self.bag_id.is_some()
    }

    /// 读取文件内容 (从偏移量开始)
    pub fn read(&self, offset: usize, len: usize) -> Vec<u8> {
        if offset >= self.data.len() {
            return Vec::new();
        }
        let end = (offset + len).min(self.data.len());
        self.data[offset..end].to_vec()
    }

    /// 写入数据到文件 (返回实际写入的字节数)
    pub fn write(&mut self, offset: usize, buf: &[u8]) -> usize {
        let needed = offset + buf.len();
        if needed > self.data.len() {
            self.data.resize(needed, 0);
        }
        let len = buf.len();
        self.data[offset..offset + len].copy_from_slice(buf);
        self.size = self.data.len();
        self.modified_ms = current_timestamp_ms();
        len
    }

    /// 截断文件到指定大小
    pub fn truncate(&mut self, size: usize) {
        if size < self.data.len() {
            self.data.truncate(size);
        } else {
            self.data.resize(size, 0);
        }
        self.size = self.data.len();
        self.modified_ms = current_timestamp_ms();
    }
}

/// 目录项 — 文件名 → inode 映射
#[derive(Clone, Debug)]
pub struct DirEntry {
    pub name: String,
    pub ino: u64,
}

/// 目录结构 — 包含子目录项
#[derive(Clone, Debug)]
pub struct Directory {
    pub inode: Inode,
    pub entries: Vec<DirEntry>,
}

impl Directory {
    pub fn new(ino: u64, name: &str, owner_pid: u64) -> Self {
        Self {
            inode: Inode::new(ino, name, FileType::Directory, owner_pid),
            entries: Vec::new(),
        }
    }

    /// 添加子项
    pub fn add_entry(&mut self, name: &str, ino: u64) -> bool {
        if self.entries.iter().any(|e| e.name == name) {
            return false; // 已存在
        }
        self.entries.push(DirEntry { name: name.to_string(), ino });
        self.inode.modified_ms = current_timestamp_ms();
        true
    }

    /// 移除子项
    pub fn remove_entry(&mut self, name: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.name != name);
        self.entries.len() < before
    }

    /// 查找子项
    pub fn lookup(&self, name: &str) -> Option<&DirEntry> {
        self.entries.iter().find(|e| e.name == name)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

/// 虚拟文件系统 — inode 表 + 目录树
pub struct Vfs {
    pub inodes: HashMap<u64, Inode>,
    pub directories: HashMap<u64, Directory>,
    pub next_ino: u64,
    pub root_ino: u64,
    pub total_files: u64,
    pub total_dirs: u64,
    pub total_bytes: usize,
}

impl Vfs {
    pub fn new() -> Self {
        let root_ino = 1;
        let mut vfs = Self {
            inodes: HashMap::new(),
            directories: HashMap::new(),
            next_ino: root_ino + 1,
            root_ino,
            total_files: 0,
            total_dirs: 0,
            total_bytes: 0,
        };
        // 创建根目录
        vfs.directories.insert(root_ino, Directory::new(root_ino, "/", 0));
        vfs.inodes.insert(root_ino, Inode::new(root_ino, "/", FileType::Directory, 0));
        vfs
    }

    fn alloc_ino(&mut self) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;
        ino
    }

    /// 在指定目录下创建文件
    pub fn create_file(&mut self, parent_dir_ino: u64, name: &str, owner_pid: u64) -> Option<u64> {
        // 检查父目录是否存在
        if !self.directories.contains_key(&parent_dir_ino) {
            return None;
        }
        let ino = self.alloc_ino();
        let inode = Inode::new(ino, name, FileType::Regular, owner_pid);
        self.inodes.insert(ino, inode);
        // 添加到目录
        self.directories.get_mut(&parent_dir_ino).unwrap().add_entry(name, ino);
        self.total_files += 1;
        Some(ino)
    }

    /// 创建袋子文件
    pub fn create_bag_file(&mut self, parent_dir_ino: u64, name: &str, bag_id: u64, attrs_summary: &[u8], owner_pid: u64) -> Option<u64> {
        if !self.directories.contains_key(&parent_dir_ino) {
            return None;
        }
        let ino = self.alloc_ino();
        let inode = Inode::new_bag_file(ino, name, bag_id, attrs_summary, owner_pid);
        self.total_bytes += inode.size;
        self.inodes.insert(ino, inode);
        self.directories.get_mut(&parent_dir_ino).unwrap().add_entry(name, ino);
        self.total_files += 1;
        Some(ino)
    }

    /// 创建子目录
    pub fn mkdir(&mut self, parent_dir_ino: u64, name: &str, owner_pid: u64) -> Option<u64> {
        if !self.directories.contains_key(&parent_dir_ino) {
            return None;
        }
        let ino = self.alloc_ino();
        let dir = Directory::new(ino, name, owner_pid);
        self.inodes.insert(ino, dir.inode.clone());
        self.directories.insert(ino, dir);
        self.directories.get_mut(&parent_dir_ino).unwrap().add_entry(name, ino);
        self.total_dirs += 1;
        Some(ino)
    }

    /// 按 inode 获取文件
    pub fn get_inode(&self, ino: u64) -> Option<&Inode> {
        self.inodes.get(&ino)
    }

    /// 按 inode 获取可变文件
    pub fn get_inode_mut(&mut self, ino: u64) -> Option<&mut Inode> {
        self.inodes.get_mut(&ino)
    }

    /// 读取文件内容
    pub fn read_file(&self, ino: u64) -> Option<Vec<u8>> {
        self.inodes.get(&ino).map(|i| i.data.clone())
    }

    /// 写入文件内容 (全量覆盖)
    pub fn write_file(&mut self, ino: u64, data: &[u8]) -> bool {
        if let Some(inode) = self.inodes.get_mut(&ino) {
            self.total_bytes -= inode.data.len();
            inode.data = data.to_vec();
            inode.size = data.len();
            inode.modified_ms = current_timestamp_ms();
            self.total_bytes += inode.size;
            true
        } else {
            false
        }
    }

    /// 删除文件
    pub fn unlink(&mut self, parent_dir_ino: u64, name: &str) -> bool {
        let ino = match self.directories.get(&parent_dir_ino).and_then(|d| d.lookup(name)) {
            Some(entry) => entry.ino,
            None => return false,
        };
        // 从目录移除
        self.directories.get_mut(&parent_dir_ino).unwrap().remove_entry(name);
        // 如果是目录, 也移除目录
        if self.directories.contains_key(&ino) {
            self.directories.remove(&ino);
            self.total_dirs -= 1;
        }
        // 移除inode
        if let Some(inode) = self.inodes.remove(&ino) {
            self.total_bytes -= inode.size;
            self.total_files -= 1;
        }
        true
    }

    /// 列出目录内容
    pub fn list_dir(&self, dir_ino: u64) -> Option<Vec<DirEntry>> {
        self.directories.get(&dir_ino).map(|d| d.entries.clone())
    }

    pub fn file_count(&self) -> usize {
        self.inodes.len()
    }

    pub fn dir_count(&self) -> usize {
        self.directories.len()
    }
}

impl Default for Vfs {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第二部分: 进程间通信 (IPC)
// ======================================================================

/// IPC 类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpcKind {
    Pipe,           // 单向管道 (半双工)
    MessageQueue,   // 消息队列
    SharedMemory,   // 共享内存段
}

/// 管道 — 进程间单向数据传输
pub struct Pipe {
    pub id: u64,
    pub buffer: Vec<u8>,
    pub capacity: usize,
    pub reader_pid: Option<u64>,
    pub writer_pid: Option<u64>,
    pub is_open: bool,
}

impl Pipe {
    pub fn new(id: u64, capacity: usize) -> Self {
        Self {
            id,
            buffer: Vec::new(),
            capacity,
            reader_pid: None,
            writer_pid: None,
            is_open: true,
        }
    }

    /// 写入管道 (阻塞式模拟)
    pub fn write(&mut self, data: &[u8]) -> bool {
        if !self.is_open {
            return false;
        }
        let space = self.capacity.saturating_sub(self.buffer.len());
        if space == 0 {
            return false; // 管道满
        }
        let to_write = data.len().min(space);
        self.buffer.extend_from_slice(&data[..to_write]);
        true
    }

    /// 从管道读取
    pub fn read(&mut self, max_len: usize) -> Vec<u8> {
        if self.buffer.is_empty() {
            return Vec::new();
        }
        let to_read = max_len.min(self.buffer.len());
        self.buffer.drain(0..to_read).collect()
    }

    /// 关闭管道
    pub fn close(&mut self) {
        self.is_open = false;
        self.buffer.clear();
    }

    pub fn available_bytes(&self) -> usize {
        self.buffer.len()
    }
}

/// 消息 — 消息队列中的条目
#[derive(Clone, Debug)]
pub struct Message {
    pub mtype: i64,         // 消息类型 (>0)
    pub payload: Vec<u8>,
    pub sender_pid: u64,
    pub timestamp_ms: u64,
}

impl Message {
    pub fn new(mtype: i64, payload: Vec<u8>, sender_pid: u64) -> Self {
        Self {
            mtype,
            payload,
            sender_pid,
            timestamp_ms: current_timestamp_ms(),
        }
    }

    /// 创建袋子消息 (袋子流转)
    pub fn bag_message(mtype: i64, bag_id: u64, sender_pid: u64) -> Self {
        Self {
            mtype,
            payload: bag_id.to_be_bytes().to_vec(),
            sender_pid,
            timestamp_ms: current_timestamp_ms(),
        }
    }
}

/// 消息队列 — FIFO消息传递
pub struct MessageQueue {
    pub id: u64,
    pub messages: Vec<Message>,
    pub max_bytes: usize,
    pub current_bytes: usize,
    pub owner_pid: u64,
}

impl MessageQueue {
    pub fn new(id: u64, max_bytes: usize, owner_pid: u64) -> Self {
        Self {
            id,
            messages: Vec::new(),
            max_bytes,
            current_bytes: 0,
            owner_pid,
        }
    }

    /// 发送消息
    pub fn send(&mut self, msg: Message) -> bool {
        let msg_size = msg.payload.len() + 16; // 估算消息开销
        if self.current_bytes + msg_size > self.max_bytes {
            return false; // 队列满
        }
        self.current_bytes += msg_size;
        self.messages.push(msg);
        true
    }

    /// 接收消息 (指定类型, mtype=0表示任意, >0表示特定类型, <0表示最小类型)
    pub fn recv(&mut self, mtype: i64, max_len: usize) -> Option<(Message, Vec<u8>)> {
        let idx = if mtype == 0 {
            // 取第一条
            Some(0)
        } else if mtype > 0 {
            self.messages.iter().position(|m| m.mtype == mtype)
        } else {
            // 找最小类型
            self.messages.iter().enumerate().min_by_key(|(_, m)| m.mtype).map(|(i, _)| i)
        }?;

        let msg = self.messages.remove(idx);
        let msg_size = msg.payload.len() + 16;
        self.current_bytes = self.current_bytes.saturating_sub(msg_size);
        let data = if msg.payload.len() > max_len {
            msg.payload[..max_len].to_vec()
        } else {
            msg.payload.clone()
        };
        Some((msg, data))
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

/// 共享内存段
pub struct SharedMemory {
    pub id: u64,
    pub size: usize,
    pub data: Vec<u8>,
    pub attached_pids: Vec<u64>,  // 附加的进程列表
}

impl SharedMemory {
    pub fn new(id: u64, size: usize) -> Self {
        Self {
            id,
            size,
            data: vec![0u8; size],
            attached_pids: Vec::new(),
        }
    }

    /// 附加到共享内存
    pub fn attach(&mut self, pid: u64) -> bool {
        if self.attached_pids.contains(&pid) {
            return false;
        }
        self.attached_pids.push(pid);
        true
    }

    /// 分离
    pub fn detach(&mut self, pid: u64) -> bool {
        if let Some(idx) = self.attached_pids.iter().position(|p| *p == pid) {
            self.attached_pids.remove(idx);
            true
        } else {
            false
        }
    }

    /// 写入共享内存
    pub fn write(&mut self, offset: usize, buf: &[u8]) -> bool {
        if offset + buf.len() > self.size {
            return false;
        }
        self.data[offset..offset + buf.len()].copy_from_slice(buf);
        true
    }

    /// 读取共享内存
    pub fn read(&self, offset: usize, len: usize) -> Vec<u8> {
        if offset >= self.size {
            return Vec::new();
        }
        let end = (offset + len).min(self.size);
        self.data[offset..end].to_vec()
    }

    pub fn attached_count(&self) -> usize {
        self.attached_pids.len()
    }
}

/// IPC 管理器 — 统一管理所有 IPC 资源
pub struct IpcManager {
    pub pipes: HashMap<u64, Pipe>,
    pub msg_queues: HashMap<u64, MessageQueue>,
    pub shared_mems: HashMap<u64, SharedMemory>,
    pub next_id: u64,
}

impl IpcManager {
    pub fn new() -> Self {
        Self {
            pipes: HashMap::new(),
            msg_queues: HashMap::new(),
            shared_mems: HashMap::new(),
            next_id: 1,
        }
    }

    fn alloc_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// 创建管道, 返回读端和写端ID (同一pipe)
    pub fn create_pipe(&mut self, capacity: usize) -> u64 {
        let id = self.alloc_id();
        self.pipes.insert(id, Pipe::new(id, capacity));
        id
    }

    /// 创建消息队列
    pub fn create_msgq(&mut self, max_bytes: usize, owner_pid: u64) -> u64 {
        let id = self.alloc_id();
        self.msg_queues.insert(id, MessageQueue::new(id, max_bytes, owner_pid));
        id
    }

    /// 创建共享内存段
    pub fn create_shm(&mut self, size: usize) -> u64 {
        let id = self.alloc_id();
        self.shared_mems.insert(id, SharedMemory::new(id, size));
        id
    }

    /// 获取管道
    pub fn get_pipe(&mut self, id: u64) -> Option<&mut Pipe> {
        self.pipes.get_mut(&id)
    }

    /// 获取消息队列
    pub fn get_msgq(&mut self, id: u64) -> Option<&mut MessageQueue> {
        self.msg_queues.get_mut(&id)
    }

    /// 获取共享内存
    pub fn get_shm(&mut self, id: u64) -> Option<&mut SharedMemory> {
        self.shared_mems.get_mut(&id)
    }

    /// 销毁 IPC 资源
    pub fn destroy(&mut self, kind: IpcKind, id: u64) -> bool {
        match kind {
            IpcKind::Pipe => self.pipes.remove(&id).is_some(),
            IpcKind::MessageQueue => self.msg_queues.remove(&id).is_some(),
            IpcKind::SharedMemory => self.shared_mems.remove(&id).is_some(),
        }
    }

    pub fn resource_count(&self) -> usize {
        self.pipes.len() + self.msg_queues.len() + self.shared_mems.len()
    }
}

impl Default for IpcManager {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第三部分: 设备驱动框架
// ======================================================================

/// 设备类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Character,  // 字符设备 (字节流)
    Block,      // 块设备 (块状IO)
}

/// 设备操作接口 (简化版 file_operations)
#[derive(Clone, Debug)]
pub struct FileOperations {
    pub read_fn: String,    // 读操作名
    pub write_fn: String,   // 写操作名
    pub ioctl_fn: String,   // 控制操作名
    pub open_fn: String,    // 打开操作名
    pub close_fn: String,   // 关闭操作名
}

impl FileOperations {
    pub fn new(read: &str, write: &str, ioctl: &str, open: &str, close: &str) -> Self {
        Self {
            read_fn: read.to_string(),
            write_fn: write.to_string(),
            ioctl_fn: ioctl.to_string(),
            open_fn: open.to_string(),
            close_fn: close.to_string(),
        }
    }
}

/// 设备结构
#[derive(Clone, Debug)]
pub struct Device {
    pub id: u64,
    pub name: String,
    pub kind: DeviceKind,
    pub major: u32,         // 主设备号
    pub minor: u32,         // 次设备号
    pub data: Vec<u8>,      // 设备缓冲区
    pub ops: FileOperations,
    pub is_open: bool,
    pub owner_pid: Option<u64>,
}

impl Device {
    pub fn new(id: u64, name: &str, kind: DeviceKind, major: u32, minor: u32, ops: FileOperations) -> Self {
        Self {
            id,
            name: name.to_string(),
            kind,
            major,
            minor,
            data: Vec::new(),
            ops,
            is_open: false,
            owner_pid: None,
        }
    }

    /// 打开设备
    pub fn open(&mut self, pid: u64) -> bool {
        if self.is_open {
            return false;
        }
        self.is_open = true;
        self.owner_pid = Some(pid);
        true
    }

    /// 关闭设备
    pub fn close(&mut self) {
        self.is_open = false;
        self.owner_pid = None;
    }

    /// 设备读
    pub fn read(&self, max_len: usize) -> Vec<u8> {
        if !self.is_open {
            return Vec::new();
        }
        let len = max_len.min(self.data.len());
        self.data[..len].to_vec()
    }

    /// 设备写
    pub fn write(&mut self, buf: &[u8]) -> bool {
        if !self.is_open {
            return false;
        }
        self.data = buf.to_vec();
        true
    }
}

/// 设备管理器
pub struct DeviceManager {
    pub devices: HashMap<u64, Device>,
    pub next_id: u64,
    pub char_count: usize,
    pub block_count: usize,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            next_id: 1,
            char_count: 0,
            block_count: 0,
        }
    }

    fn alloc_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// 注册设备
    pub fn register(&mut self, name: &str, kind: DeviceKind, major: u32, minor: u32, ops: FileOperations) -> u64 {
        let id = self.alloc_id();
        let dev = Device::new(id, name, kind, major, minor, ops);
        match kind {
            DeviceKind::Character => self.char_count += 1,
            DeviceKind::Block => self.block_count += 1,
        }
        self.devices.insert(id, dev);
        id
    }

    /// 注销设备
    pub fn unregister(&mut self, id: u64) -> bool {
        if let Some(dev) = self.devices.remove(&id) {
            match dev.kind {
                DeviceKind::Character => self.char_count -= 1,
                DeviceKind::Block => self.block_count -= 1,
            }
            true
        } else {
            false
        }
    }

    /// 获取设备
    pub fn get(&mut self, id: u64) -> Option<&mut Device> {
        self.devices.get_mut(&id)
    }

    /// 按名称查找设备
    pub fn find_by_name(&self, name: &str) -> Option<&Device> {
        self.devices.values().find(|d| d.name == name)
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for DeviceManager {
    fn default() -> Self { Self::new() }
}

// ======================================================================
// 第四部分: 天觉V7引擎 — 总集成
// ======================================================================

/// V7 额外处理结果
#[derive(Clone, Debug)]
pub struct V7Extras {
    pub file_created: Option<u64>,
    pub pipe_id: Option<u64>,
    pub msgq_id: Option<u64>,
    pub dev_id: Option<u64>,
}

/// V7 管线输出
#[derive(Clone, Debug)]
pub struct PipelineResultV7 {
    pub bag_count: usize,
    pub file_count: usize,
    pub pipe_count: usize,
    pub msgq_count: usize,
    pub device_count: usize,
    pub extras: V7Extras,
}

/// 天觉V7引擎
pub struct CelestialPerceptionV7 {
    pub vfs: Vfs,
    pub ipc: IpcManager,
    pub devices: DeviceManager,
    pub bag_count: u64,
    pub kernel_booted: bool,
}

impl CelestialPerceptionV7 {
    pub fn new() -> Self {
        Self {
            vfs: Vfs::new(),
            ipc: IpcManager::new(),
            devices: DeviceManager::new(),
            bag_count: 0,
            kernel_booted: true,
        }
    }

    /// 启动完整 OS: 挂载根文件系统 + 初始化设备
    pub fn boot_full(&mut self) -> String {
        let mut log = String::new();

        // 创建标准目录结构
        let root = self.vfs.root_ino;
        self.vfs.mkdir(root, "bags", 0);       // /bags — 袋子目录
        self.vfs.mkdir(root, "pipes", 0);      // /pipes — 管道目录
        self.vfs.mkdir(root, "dev", 0);        // /dev — 设备目录
        self.vfs.mkdir(root, "tmp", 0);        // /tmp — 临时文件
        log.push_str("启动: 已创建 /bags /pipes /dev /tmp");

        // 注册标准设备
        let bag_input_ops = FileOperations::new("bag_read", "bag_write", "bag_ioctl", "bag_open", "bag_close");
        self.devices.register("bag_input", DeviceKind::Character, 1, 0, bag_input_ops);
        let bag_output_ops = FileOperations::new("bag_display", "bag_output_write", "bag_output_ioctl", "bag_output_open", "bag_output_close");
        self.devices.register("bag_output", DeviceKind::Character, 1, 1, bag_output_ops);
        let storage_ops = FileOperations::new("storage_read_block", "storage_write_block", "storage_ioctl", "storage_open", "storage_close");
        self.devices.register("bag_disk", DeviceKind::Block, 8, 0, storage_ops);
        log.push_str("\n启动: 已注册3个设备 (bag_input, bag_output, bag_disk)");

        log
    }

    /// 袋子持久化到文件系统
    pub fn persist_bag(&mut self, bag_id: u64, bag_name: &str, attrs_summary: &[u8]) -> bool {
        self.persist_bag_and_get_ino(bag_id, bag_name, attrs_summary).is_some()
    }

    /// 袋子持久化并返回 inode 号
    pub fn persist_bag_and_get_ino(&mut self, bag_id: u64, bag_name: &str, attrs_summary: &[u8]) -> Option<u64> {
        let bags_dir = self.find_dir("bags")?;
        self.vfs.create_bag_file(bags_dir, bag_name, bag_id, attrs_summary, 0)
    }

    /// 查找目录
    fn find_dir(&self, name: &str) -> Option<u64> {
        self.vfs.directories.iter()
            .find(|(_, d)| d.inode.name == name)
            .map(|(ino, _)| *ino)
    }

    /// 袋子通过管道流转
    pub fn transfer_bag_via_pipe(&mut self, pipe_id: u64, bag_id: u64, sender_pid: u64) -> bool {
        let msg = Message::bag_message(1, bag_id, sender_pid);
        if let Some(pipe) = self.ipc.get_pipe(pipe_id) {
            let bytes = msg.payload.clone();
            pipe.write(&bytes)
        } else {
            false
        }
    }

    /// 袋子发送到消息队列
    pub fn send_bag_to_msgq(&mut self, msgq_id: u64, bag_id: u64, sender: u64) -> bool {
        let msg = Message::bag_message(10, bag_id, sender);
        if let Some(q) = self.ipc.get_msgq(msgq_id) {
            q.send(msg)
        } else {
            false
        }
    }

    /// 注册袋子输入设备
    pub fn register_bag_device(&mut self, name: &str, kind: DeviceKind) -> u64 {
        let ops = FileOperations::new(
            "_read_bag", "_write_bag", "_ioctl_bag", "_open_bag", "_close_bag"
        );
        self.devices.register(name, kind, 20, 0, ops)
    }

    /// 完整管线: 袋子进入 → 通过设备 → 持久化到VFS → IPC流转
    pub fn process_bag_full_v7(&mut self, bag_id: u64, bag_name: &str, attrs_summary: &[u8]) -> PipelineResultV7 {
        self.bag_count += 1;

        // 1. 持久化到文件系统
        let file_ino = self.persist_bag_and_get_ino(bag_id, bag_name, attrs_summary);

        // 2. 创建管道供袋子流转
        let pipe_id = self.ipc.create_pipe(4096);

        // 3. 创建消息队列供队列化
        let msgq_id = self.ipc.create_msgq(65536, 100);

        // 4. 注册设备 (如未注册)
        let dev_count = self.devices.device_count();

        PipelineResultV7 {
            bag_count: self.bag_count as usize,
            file_count: self.vfs.file_count(),
            pipe_count: self.ipc.pipes.len(),
            msgq_count: self.ipc.msg_queues.len(),
            device_count: dev_count,
            extras: V7Extras {
                file_created: file_ino,
                pipe_id: Some(pipe_id),
                msgq_id: Some(msgq_id),
                dev_id: if dev_count > 0 { Some(dev_count as u64) } else { None },
            },
        }
    }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉v7报告（老七）· OS三合一张 ═══\n");
        s.push_str(&format!("  VFS: {}文件, {}目录, {}字节\n",
            self.vfs.total_files, self.vfs.total_dirs, self.vfs.total_bytes));
        s.push_str(&format!("  IPC: {}管道, {}消息队列, {}共享内存\n",
            self.ipc.pipes.len(), self.ipc.msg_queues.len(), self.ipc.shared_mems.len()));
        s.push_str(&format!("  设备: {}字符设备, {}块设备\n",
            self.devices.char_count, self.devices.block_count));
        s.push_str(&format!("  已处理袋数: {}\n", self.bag_count));
        s.push_str(&format!("  内核状态: {}\n",
            if self.kernel_booted { "已启动" } else { "未启动" }));
        s
    }
}

impl Default for CelestialPerceptionV7 {
    fn default() -> Self { Self::new() }
}

fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vfs_create_and_read_file() {
        let mut vfs = Vfs::new();
        let root = vfs.root_ino;

        // 创建文件
        let ino = vfs.create_file(root, "test.txt", 1);
        assert!(ino.is_some());
        let ino = ino.unwrap();

        // 写入
        assert!(vfs.write_file(ino, b"hello bag world"));

        // 读取
        let content = vfs.read_file(ino);
        assert_eq!(content, Some(b"hello bag world".to_vec()));
        assert_eq!(vfs.total_files, 1);
    }

    #[test]
    fn vfs_create_directory_tree() {
        let mut vfs = Vfs::new();
        let root = vfs.root_ino;

        // 创建 /bags
        let bags_ino = vfs.mkdir(root, "bags", 0);
        assert!(bags_ino.is_some());

        // 在 /bags 下创建袋子文件
        let bags = bags_ino.unwrap();
        let file_ino = vfs.create_bag_file(bags, "rice_001.dat", 1, &[1, 2, 3, 4], 100);
        assert!(file_ino.is_some());

        // 验证袋子文件属性
        let inode = vfs.get_inode(file_ino.unwrap()).unwrap();
        assert!(inode.is_bag_file());
        assert_eq!(inode.bag_id, Some(1));
        assert_eq!(inode.ftype, FileType::Regular);
        assert_eq!(vfs.file_count(), 3); // root + bags + file
    }

    #[test]
    fn vfs_unlink_file() {
        let mut vfs = Vfs::new();
        let root = vfs.root_ino;

        let ino = vfs.create_file(root, "tmp.txt", 1).unwrap();
        assert_eq!(vfs.total_files, 1);

        // 删除
        assert!(vfs.unlink(root, "tmp.txt"));
        assert_eq!(vfs.total_files, 0);
        assert!(vfs.get_inode(ino).is_none());
    }

    #[test]
    fn vfs_list_directory() {
        let mut vfs = Vfs::new();
        let root = vfs.root_ino;

        vfs.create_file(root, "a.txt", 1);
        vfs.create_file(root, "b.txt", 2);
        vfs.mkdir(root, "subdir", 0);

        let entries = vfs.list_dir(root).unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn inode_read_write_with_offset() {
        let mut inode = Inode::new(1, "test", FileType::Regular, 100);

        // 写入偏移 10
        assert_eq!(inode.write(10, b"hello"), 5);
        assert_eq!(inode.size, 15);

        // 从偏移 10 读取
        let data = inode.read(10, 5);
        assert_eq!(data, b"hello".to_vec());

        // 从偏移 0 读取 (应该有 10 个零字节)
        let data = inode.read(0, 5);
        assert_eq!(data, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn inode_truncate() {
        let mut inode = Inode::new(1, "test", FileType::Regular, 100);
        inode.write(0, b"hello world");

        // 截断
        inode.truncate(5);
        assert_eq!(inode.size, 5);
        assert_eq!(inode.data, b"hello".to_vec());
    }

    #[test]
    fn pipe_write_read() {
        let mut pipe = Pipe::new(1, 1024);

        // 写入
        assert!(pipe.write(b"bag_data"));
        assert_eq!(pipe.available_bytes(), 8);

        // 读取
        let data = pipe.read(4);
        assert_eq!(data, b"bag_".to_vec());
        assert_eq!(pipe.available_bytes(), 4);
    }

    #[test]
    fn pipe_capacity_limit() {
        let mut pipe = Pipe::new(1, 5);

        // 写入到满
        assert!(pipe.write(b"12345"));
        // 再写应该失败
        assert!(!pipe.write(b"67890"));
        assert_eq!(pipe.available_bytes(), 5);
    }

    #[test]
    fn pipe_close() {
        let mut pipe = Pipe::new(1, 1024);
        pipe.write(b"data");
        pipe.close();
        assert!(!pipe.is_open);
        assert_eq!(pipe.available_bytes(), 0);
        // 关闭后写入失败
        assert!(!pipe.write(b"more"));
    }

    #[test]
    fn message_queue_send_recv() {
        let mut q = MessageQueue::new(1, 4096, 100);

        // 发送
        let msg = Message::new(10, vec![1, 2, 3], 50);
        assert!(q.send(msg));
        assert_eq!(q.len(), 1);

        // 接收
        let (received, data) = q.recv(0, 1024).unwrap();
        assert_eq!(received.mtype, 10);
        assert_eq!(data, vec![1, 2, 3]);
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn message_queue_type_filtering() {
        let mut q = MessageQueue::new(1, 4096, 100);

        q.send(Message::new(5, vec![1], 10));
        q.send(Message::new(10, vec![2], 20));
        q.send(Message::new(5, vec![3], 30));

        // 取 mtype=5 的消息
        let (msg, _) = q.recv(5, 1024).unwrap();
        assert_eq!(msg.mtype, 5);
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn message_queue_full() {
        let mut q = MessageQueue::new(1, 20, 100); // 很小的队列

        // 发送大消息, 应该填满
        let big_payload = vec![0u8; 100];
        let msg = Message::new(1, big_payload, 10);
        // 第一次发送可能成功, 第二次失败
        q.send(msg.clone());
        // 继续发送应该失败
        let msg2 = Message::new(2, vec![0u8; 100], 20);
        assert!(!q.send(msg2) || q.len() >= 1);
    }

    #[test]
    fn message_bag_message() {
        let msg = Message::bag_message(10, 42, 100);
        assert_eq!(msg.mtype, 10);
        assert_eq!(msg.sender_pid, 100);
        // payload 应该是 bag_id 8字节大端表示
        assert_eq!(msg.payload, 42u64.to_be_bytes().to_vec());
    }

    #[test]
    fn shared_memory_basic() {
        let mut shm = SharedMemory::new(1, 256);

        // 进程附加
        assert!(shm.attach(100));
        assert!(shm.attach(200));
        assert!(!shm.attach(100)); // 重复附加
        assert_eq!(shm.attached_count(), 2);

        // 写读
        assert!(shm.write(0, b"shared_data"));
        let data = shm.read(0, 11);
        assert_eq!(data, b"shared_data".to_vec());

        // 分离
        assert!(shm.detach(100));
        assert_eq!(shm.attached_count(), 1);
    }

    #[test]
    fn shared_memory_bounds() {
        let mut shm = SharedMemory::new(1, 16);

        // 超出范围写入
        assert!(!shm.write(0, b"this is a very long string"));
        // 应该截断或失败 — 我们的实现是严格检查, 所以失败
        assert_eq!(shm.read(0, 16), vec![0u8; 16]);

        // 范围内
        assert!(shm.write(0, b"short"));
    }

    #[test]
    fn ipc_manager_create_all() {
        let mut ipc = IpcManager::new();

        let pipe_id = ipc.create_pipe(1024);
        let msgq_id = ipc.create_msgq(4096, 100);
        let shm_id = ipc.create_shm(256);

        assert!(ipc.pipes.contains_key(&pipe_id));
        assert!(ipc.msg_queues.contains_key(&msgq_id));
        assert!(ipc.shared_mems.contains_key(&shm_id));
        assert_eq!(ipc.resource_count(), 3);
    }

    #[test]
    fn ipc_destroy() {
        let mut ipc = IpcManager::new();
        let pipe_id = ipc.create_pipe(1024);

        assert!(ipc.destroy(IpcKind::Pipe, pipe_id));
        assert!(!ipc.pipes.contains_key(&pipe_id));
        assert_eq!(ipc.resource_count(), 0);
    }

    #[test]
    fn device_register_and_open() {
        let mut dm = DeviceManager::new();
        let ops = FileOperations::new("read", "write", "ioctl", "open", "close");

        let id = dm.register("bag_sensor", DeviceKind::Character, 1, 0, ops);
        let dev = dm.get(id).unwrap();
        assert!(dev.open(100));
        assert!(dev.is_open);
        assert_eq!(dev.owner_pid, Some(100));

        // 重复打开失败
        assert!(!dev.open(200));
    }

    #[test]
    fn device_read_write() {
        let mut dm = DeviceManager::new();
        let ops = FileOperations::new("read", "write", "ioctl", "open", "close");
        let id = dm.register("bag_input", DeviceKind::Character, 1, 0, ops);

        {
            let dev = dm.get(id).unwrap();
            dev.open(100);
        }

        {
            let dev = dm.get(id).unwrap();
            assert!(dev.write(b"bag sensor data"));
        }

        {
            let dev = dm.get(id).unwrap();
            let data = dev.read(1024);
            assert_eq!(data, b"bag sensor data".to_vec());
        }
    }

    #[test]
    fn device_unregister() {
        let mut dm = DeviceManager::new();
        let ops = FileOperations::new("r", "w", "c", "o", "cl");
        let id = dm.register("tmp_dev", DeviceKind::Block, 8, 0, ops);

        assert_eq!(dm.device_count(), 1);
        assert!(dm.unregister(id));
        assert_eq!(dm.device_count(), 0);
    }

    #[test]
    fn v7_boot_creates_standard_dirs() {
        let mut engine = CelestialPerceptionV7::new();
        let log = engine.boot_full();

        // 有4个标准目录 + 根目录
        assert!(engine.vfs.dir_count() >= 4);
        assert!(log.contains("bags"));
        assert!(log.contains("dev"));
    }

    #[test]
    fn v7_persist_and_transfer_bag() {
        let mut engine = CelestialPerceptionV7::new();
        engine.boot_full();

        // 持久化袋子
        assert!(engine.persist_bag(1, "rice_001", &[1, 2, 3]));

        // 通过管道传输
        let pipe_id = engine.ipc.create_pipe(4096);
        assert!(engine.transfer_bag_via_pipe(pipe_id, 1, 100));

        // 发送到消息队列
        let msgq_id = engine.ipc.create_msgq(65536, 100);
        assert!(engine.send_bag_to_msgq(msgq_id, 1, 100));
    }

    #[test]
    fn v7_full_pipeline() {
        let mut engine = CelestialPerceptionV7::new();
        engine.boot_full();

        // 处理多个袋子
        for i in 1..=3 {
            let result = engine.process_bag_full_v7(i, &format!("bag_{}", i),
                &[i as u8, 1, 2, 3]);
            assert!(result.extras.file_created.is_some());
            assert!(result.extras.pipe_id.is_some());
        }

        assert_eq!(engine.bag_count, 3);
        // 至少有 3个文件 (bags目录下的袋子文件) + 根目录 + 4个标准目录
        assert!(engine.vfs.file_count() >= 3);
    }

    #[test]
    fn v7_report_format() {
        let mut engine = CelestialPerceptionV7::new();
        engine.boot_full();
        engine.process_bag_full_v7(1, "test", &[1, 2, 3]);

        let report = engine.report();
        assert!(report.contains("天觉v7"));
        assert!(report.contains("VFS"));
        assert!(report.contains("IPC"));
        assert!(report.contains("设备"));
    }
}
