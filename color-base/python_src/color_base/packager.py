"""
颜色进制打包器

打包为 .cbpack 分发包:
  - 元数据 JSON (名称/作者/版本/依赖)
  - CB 源代码 + CBF 二进制
  - 校验和
  - 清单

打包格式 (简化 ZIP-like):
  [magic:4][header_len:4][header_json][file_data...]
"""

from __future__ import annotations
import struct
import json
import hashlib
import time
import os
from typing import Dict, List, Optional, Tuple


class ColorBasePackager:
    """打包/解包工具"""
    
    MAGIC = b'CBPK'
    VERSION = 1
    
    def __init__(self):
        self.files: Dict[str, bytes] = {}
        self.metadata: Dict = {}
    
    def set_metadata(self, name: str, author: str, version: str,
                     description: str = "", depends: List[str] = None):
        self.metadata = {
            "name": name,
            "author": author,
            "version": version,
            "description": description,
            "depends": depends or [],
            "created_at": time.strftime("%Y-%m-%d %H:%M:%S"),
            "format_version": self.VERSION,
        }
    
    def add_file(self, path: str, data: bytes):
        """添加文件到包"""
        self.files[path] = data
    
    def add_file_from_disk(self, path: str, disk_path: str = None):
        """从磁盘加载文件"""
        disk_path = disk_path or path
        if os.path.exists(disk_path):
            with open(disk_path, 'rb') as f:
                self.files[path] = f.read()
    
    def pack(self) -> bytes:
        """打包为 .cbpack 二进制数据"""
        header = {
            "metadata": self.metadata,
            "files": {
                path: {
                    "size": len(data),
                    "sha256": hashlib.sha256(data).hexdigest()[:16],
                    "offset": 0,  # 占位
                }
                for path, data in self.files.items()
            }
        }
        header_json = json.dumps(header, ensure_ascii=False).encode('utf-8')
        
        # 计算文件偏移
        offset = 4 + 4 + len(header_json)  # magic + header_len + header
        for path in self.files:
            header["files"][path]["offset"] = offset
            offset += len(self.files[path])
        
        # 重新序列化（带偏移）
        header_json = json.dumps(header, ensure_ascii=False).encode('utf-8')
        
        # 组装
        out = bytearray()
        out.extend(self.MAGIC)
        out.extend(struct.pack('>I', len(header_json)))
        out.extend(header_json)
        for path, data in self.files.items():
            out.extend(data)
        
        return bytes(out)
    
    @classmethod
    def unpack(cls, data: bytes) -> Tuple[Dict, Dict[str, bytes]]:
        """解包 .cbpack 数据"""
        if data[:4] != cls.MAGIC:
            raise ValueError("不是有效的 .cbpack 文件")
        
        header_len = struct.unpack('>I', data[4:8])[0]
        header = json.loads(data[8:8+header_len])
        
        files = {}
        for path, info in header["files"].items():
            offset = info["offset"]
            size = info["size"]
            files[path] = data[offset:offset+size]
        
        return header, files
    
    def save(self, output_path: str):
        """保存为 .cbpack 文件"""
        data = self.pack()
        with open(output_path, 'wb') as f:
            f.write(data)
        return len(data)
    
    @classmethod
    def load(cls, path: str) -> Tuple[Dict, Dict[str, bytes]]:
        """加载 .cbpack 文件"""
        with open(path, 'rb') as f:
            return cls.unpack(f.read())
    
    def info(self) -> str:
        """包信息摘要"""
        lines = [
            f"包名: {self.metadata.get('name', '-')}",
            f"作者: {self.metadata.get('author', '-')}",
            f"版本: {self.metadata.get('version', '-')}",
            f"文件: {len(self.files)} 个",
        ]
        total = sum(len(d) for d in self.files.values())
        lines.append(f"大小: {total} 字节")
        return '\n'.join(lines)
