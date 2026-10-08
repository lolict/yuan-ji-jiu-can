// ═══════════════════════════════════════════════════════════════════════════════
// registry.rs — 注册清单 · 地址索引 · 特征编程
// ═══════════════════════════════════════════════════════════════════════════════
//
// 核心原则:
//   注册清单 = 容器地址的全局索引
//   地址即数据, 数据即地址
//   遍历注册表 = 数出所有激活容器
//   名称地址 = 完整统计范畴
//
// 两种绑定:
//   1. 注册清单绑定: name ↔ address
//   2. 特征标签分类: tag → [addresses]
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::BTreeMap;

/// 注册条目: 名称绑定到容器地址
#[derive(Clone, Debug)]
pub struct RegistryEntry {
    pub address: u8,
    pub name: String,
    pub tags: Vec<String>,
    pub cell: u64,
    pub depth: u32,
    pub polarity: u8,
    pub count: u32,
    pub seq: u32,
}

impl RegistryEntry {
    pub fn coords(&self) -> (u8, u8, u8) {
        let addr = self.address;
        let x = addr & 0x03;
        let y = (addr >> 2) & 0x03;
        let z = (addr >> 4) & 0x03;
        (x, y, z)
    }

    pub fn active_bits(&self) -> Vec<u8> {
        let mut bits = Vec::new();
        for i in 0..16u8 {
            if self.cell & (1u64 << i) != 0 {
                bits.push(i);
            }
        }
        bits
    }

    pub fn active_count(&self) -> u32 {
        (self.cell & 0xFFFF).count_ones()
    }
}

/// 注册清单: 全部容器地址的全局索引
#[derive(Clone, Debug)]
pub struct Registry {
    entries: BTreeMap<u8, RegistryEntry>,
    name_index: BTreeMap<String, u8>,
    tag_index: BTreeMap<String, Vec<u8>>,
    next_seq: u32,
    polarity_count: [u32; 2],
    total_entries: u32,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            name_index: BTreeMap::new(),
            tag_index: BTreeMap::new(),
            next_seq: 1,
            polarity_count: [0, 0],
            total_entries: 0,
        }
    }

    pub fn register(&mut self, name: &str, address: u8, cell: u64, depth: u32, tags: &[&str]) -> u32 {
        let polarity = if depth % 2 == 0 { 0u8 } else { 1u8 };
        let seq = self.next_seq;
        self.next_seq += 1;

        let entry = RegistryEntry {
            address,
            name: name.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            cell,
            depth,
            polarity,
            count: 1,
            seq,
        };

        self.polarity_count[polarity as usize] += 1;
        self.name_index.insert(name.to_string(), address);

        for &tag in tags {
            self.tag_index.entry(tag.to_string())
                .or_insert_with(Vec::new)
                .push(address);
        }

        self.entries.insert(address, entry);
        self.total_entries += 1;
        seq
    }

    pub fn touch(&mut self, address: u8) {
        if let Some(entry) = self.entries.get_mut(&address) {
            entry.count += 1;
        }
    }

    pub fn find_by_address(&self, address: u8) -> Option<&RegistryEntry> {
        self.entries.get(&address)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&RegistryEntry> {
        self.name_index.get(name).and_then(|addr| self.entries.get(addr))
    }

    pub fn find_by_tag(&self, tag: &str) -> Vec<&RegistryEntry> {
        self.tag_index.get(tag)
            .map(|addrs| addrs.iter().filter_map(|a| self.entries.get(a)).collect())
            .unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = &RegistryEntry> {
        self.entries.values()
    }

    pub fn filter_by_container(&self, container_idx: u8) -> Vec<&RegistryEntry> {
        self.entries.values()
            .filter(|e| e.cell & (1u64 << container_idx) != 0)
            .collect()
    }

    pub fn filter_by_polarity(&self, polarity: u8) -> Vec<&RegistryEntry> {
        self.entries.values()
            .filter(|e| e.polarity == polarity)
            .collect()
    }

    pub fn feature_stats(&self) -> Vec<(u8, u32)> {
        let mut stats = vec![0u32; 16];
        for entry in self.entries.values() {
            for i in 0..16u8 {
                if entry.cell & (1u64 << i) != 0 {
                    stats[i as usize] += 1;
                }
            }
        }
        stats.into_iter().enumerate().map(|(i, c)| (i as u8, c)).collect()
    }

    pub fn polarity_stats(&self) -> (u32, u32) {
        (self.polarity_count[0], self.polarity_count[1])
    }

    pub fn len(&self) -> u32 { self.total_entries }
    pub fn is_empty(&self) -> bool { self.total_entries == 0 }

    pub fn export(&self) -> String {
        let mut buf = String::new();
        buf.push_str("═══ 注册清单 ═══\n");
        buf.push_str(&format!("总数: {} · 阴: {} · 阳: {}\n\n",
            self.total_entries, self.polarity_count[0], self.polarity_count[1]));

        for entry in self.iter() {
            let (x, y, z) = entry.coords();
            buf.push_str(&format!(
                "#{:3} [{:02X}] ({},{},{}) {} cell=0x{:04X} · {} · {}\n",
                entry.seq, entry.address, x, y, z,
                entry.name, entry.cell, entry.depth,
                if entry.polarity == 1 { "阳" } else { "阴" }
            ));
        }

        buf.push_str("\n── 容器位统计 ──\n");
        for (idx, count) in self.feature_stats() {
            if count > 0 {
                buf.push_str(&format!("  容器[{}]: {} 条\n", idx, count));
            }
        }
        buf
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.total_entries.to_le_bytes());
        for entry in self.iter() {
            out.push(entry.address);
            out.extend_from_slice(&entry.cell.to_le_bytes());
            out.extend_from_slice(&entry.depth.to_le_bytes());
            out.push(entry.polarity);
            out.extend_from_slice(&entry.count.to_le_bytes());
        }
        out
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 4 { return None; }
        let total = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
        let mut reg = Self::new();
        let mut pos = 4;
        for _ in 0..total {
            if pos + 15 > bytes.len() { break; }
            let address = bytes[pos]; pos += 1;
            let cell = u64::from_le_bytes(bytes[pos..pos+8].try_into().ok()?); pos += 8;
            let depth = u32::from_le_bytes(bytes[pos..pos+4].try_into().ok()?); pos += 4;
            let polarity = bytes[pos]; pos += 1;
            let count = u32::from_le_bytes(bytes[pos..pos+4].try_into().ok()?); pos += 4;

            let name = format!("_{:02X}", address);
            reg.register(&name, address, cell, depth, &[]);
            if let Some(entry) = reg.entries.get_mut(&address) {
                entry.count = count;
                entry.polarity = polarity;
            }
        }
        Some(reg)
    }
}

impl Default for Registry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod registry_tests {
    use super::*;

    #[test]
    fn test_register_and_find() {
        let mut reg = Registry::new();
        reg.register("囝", 0x01, 0x0243, 6, &["共生", "阳"]);
        reg.register("共", 0x02, 0x001F, 6, &["共生", "阴"]);

        assert_eq!(reg.len(), 2);
        let e = reg.find_by_address(0x01).unwrap();
        assert_eq!(e.name, "囝");
        assert_eq!(e.active_bits(), vec![0, 1, 6, 9]);

        let results = reg.find_by_tag("共生");
        assert_eq!(results.len(), 2);

        let yin = reg.filter_by_polarity(0);
        assert_eq!(yin.len(), 1);
    }

    #[test]
    fn test_feature_stats() {
        let mut reg = Registry::new();
        reg.register("A", 0x00, 0x0003, 2, &[]);
        reg.register("B", 0x01, 0x0005, 4, &[]);
        reg.register("C", 0x02, 0x0001, 6, &[]);

        let stats = reg.feature_stats();
        assert_eq!(stats[0].1, 3);
        assert_eq!(stats[1].1, 1);
        assert_eq!(stats[2].1, 1);
    }

    #[test]
    fn test_roundtrip() {
        let mut reg = Registry::new();
        reg.register("test", 0x0A, 0x1234, 5, &["foo"]);
        let bytes = reg.serialize();
        let reg2 = Registry::deserialize(&bytes).unwrap();
        assert_eq!(reg2.len(), 1);
        let e = reg2.find_by_address(0x0A).unwrap();
        assert_eq!(e.cell, 0x1234);
        assert_eq!(e.depth, 5);
    }
}
