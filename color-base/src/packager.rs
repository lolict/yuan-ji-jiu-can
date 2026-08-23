//! 颜色进制打包器
//!
//! 打包为 .cbpack 分发包:
//!   - 元数据 JSON (名称/作者/版本/依赖)
//!   - CB 源代码 + CBF 二进制
//!   - 校验和
//!   - 清单
//!
//! 打包格式 (简化 ZIP-like):
//!   [magic:4][header_len:4][header_json][file_data...]

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use sha1::{Sha1, Digest};
use serde::{Deserialize, Serialize};

/// 文件条目
#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub size: usize,
    #[serde(rename = "sha256")]
    pub checksum: String,
    pub offset: usize,
}

/// CBPK 头部
#[derive(Debug, Serialize, Deserialize)]
pub struct PackHeader {
    pub metadata: HashMap<String, String>,
    pub files: HashMap<String, FileEntry>,
}

/// 打包/解包工具
#[derive(Clone)]
pub struct ColorBasePackager {
    pub files: HashMap<String, Vec<u8>>,
    pub metadata: HashMap<String, String>,
}

impl ColorBasePackager {
    const MAGIC: &'static [u8; 4] = b"CBPK";
    const VERSION: u32 = 1;

    pub fn new() -> ColorBasePackager {
        ColorBasePackager {
            files: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn set_metadata(&mut self, name: &str, author: &str, version: &str, description: &str) {
        self.metadata.insert("name".to_string(), name.to_string());
        self.metadata.insert("author".to_string(), author.to_string());
        self.metadata
            .insert("version".to_string(), version.to_string());
        self.metadata
            .insert("description".to_string(), description.to_string());
        self.metadata
            .insert("format_version".to_string(), Self::VERSION.to_string());
    }

    /// 添加文件到包
    pub fn add_file(&mut self, path: &str, data: Vec<u8>) {
        self.files.insert(path.to_string(), data);
    }

    /// 从磁盘加载文件
    pub fn add_file_from_disk(&mut self, path: &str, disk_path: &str) {
        if Path::new(disk_path).exists() {
            if let Ok(data) = fs::read(disk_path) {
                self.files.insert(path.to_string(), data);
            }
        }
    }

    /// 打包为 .cbpack 二进制数据
    pub fn pack(&self) -> Vec<u8> {
        let mut file_info = HashMap::new();
        let mut offset = 4 + 4; // magic + header_len placeholder
        let header_meta_only = PackHeader {
            metadata: self.metadata.clone(),
            files: HashMap::new(),
        };
        let header_json_meta = serde_json::to_vec(&header_meta_only).unwrap_or_default();
        offset += header_json_meta.len();

        for (path, data) in &self.files {
            let checksum = Self::sha16(data);
            file_info.insert(
                path.clone(),
                FileEntry {
                    size: data.len(),
                    checksum,
                    offset: 0,
                },
            );
        }

        // compute offsets
        let mut current_offset = offset;
        let mut ordered_files: Vec<(&String, &Vec<u8>)> = self.files.iter().collect();
        ordered_files.sort_by(|a, b| a.0.cmp(b.0));

        let mut offsets = HashMap::new();
        for (path, data) in &ordered_files {
            offsets.insert((*path).clone(), current_offset);
            current_offset += data.len();
        }

        // rebuild file_info with correct offsets
        for (path, info) in file_info.iter_mut() {
            if let Some(&off) = offsets.get(path) {
                info.offset = off;
            }
        }

        let header = PackHeader {
            metadata: self.metadata.clone(),
            files: file_info,
        };
        let header_json = serde_json::to_vec(&header).unwrap_or_default();

        let mut out = Vec::new();
        out.extend_from_slice(Self::MAGIC);
        out.extend_from_slice(&(header_json.len() as u32).to_be_bytes());
        out.extend_from_slice(&header_json);
        for (path, data) in &ordered_files {
            out.extend_from_slice(data);
        }
        out
    }

    /// 解包 .cbpack 数据
    pub fn unpack(data: &[u8]) -> Result<(PackHeader, HashMap<String, Vec<u8>>), String> {
        if data.len() < 8 {
            return Err("数据过短".to_string());
        }
        if &data[..4] != Self::MAGIC {
            return Err("不是有效的 .cbpack 文件".to_string());
        }
        let header_len = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
        if data.len() < 8 + header_len {
            return Err("头部长度超出数据范围".to_string());
        }
        let header: PackHeader = serde_json::from_slice(&data[8..8 + header_len])
            .map_err(|e| format!("JSON 解析失败: {}", e))?;

        let mut files = HashMap::new();
        for (path, info) in &header.files {
            let start = info.offset;
            let end = start + info.size;
            if end > data.len() {
                return Err(format!("文件 '{}' 超出数据范围", path));
            }
            files.insert(path.clone(), data[start..end].to_vec());
        }
        Ok((header, files))
    }

    /// 保存为 .cbpack 文件
    pub fn save(&self, output_path: &str) -> Result<usize, String> {
        let data = self.pack();
        fs::write(output_path, &data).map_err(|e| format!("写入失败: {}", e))?;
        Ok(data.len())
    }

    /// 加载 .cbpack 文件
    pub fn load(path: &str) -> Result<(PackHeader, HashMap<String, Vec<u8>>), String> {
        let data = fs::read(path).map_err(|e| format!("读取失败: {}", e))?;
        Self::unpack(&data)
    }

    /// 包信息摘要
    pub fn info(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "包名: {}",
            self.metadata.get("name").map(|s| s.as_str()).unwrap_or("-")
        ));
        lines.push(format!(
            "作者: {}",
            self.metadata.get("author").map(|s| s.as_str()).unwrap_or("-")
        ));
        lines.push(format!(
            "版本: {}",
            self.metadata.get("version").map(|s| s.as_str()).unwrap_or("-")
        ));
        lines.push(format!("文件: {} 个", self.files.len()));
        let total: usize = self.files.values().map(|d| d.len()).sum();
        lines.push(format!("大小: {} 字节", total));
        lines.join("\n")
    }

    /// SHA-16 checksum (first 16 chars of hex-encoded SHA-1)
    fn sha16(data: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)[..16.min(format!("{:x}", result).len())].to_string()
    }
}

impl Default for ColorBasePackager {
    fn default() -> ColorBasePackager {
        ColorBasePackager::new()
    }
}

impl fmt::Display for ColorBasePackager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ColorBasePackager(files={}, meta={})",
            self.files.len(),
            self.metadata.len()
        )
    }
}
