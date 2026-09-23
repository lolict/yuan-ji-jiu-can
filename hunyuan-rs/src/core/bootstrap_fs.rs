// ══════════════════════════════════════════════════════════════════
// bootstrap_fs.rs — 自举编译器 · 工具从文件系统长出来
// ══════════════════════════════════════════════════════════════════
//
//  「避免被他人规则绑架」= 工具源码不是 Rust 常量 · 是磁盘上的文件
//
//  核心机制:
//    1. ToolManifest — 文件命名工具清单 (工具名 → 源文件路径 + 角色)
//    2. ToolRegistry — 运行时注册表 (工具名 → 已编译 bytecode)
//    3. BootstrapFS — 文件系统驱动构建器 (读 manifest → 编译 → 注册)
//    4. seed_filesystem — 把 Rust 常量初次写入磁盘 (播种, 只执行一次)
//
//  自举链路:
//    Rust seed → 写工具源码到文件 → 销毁 Rust 常量
//    系统重启 → 读文件 → compile_l0 → 注册工具链 → 工具链编译新工具
//
// ══════════════════════════════════════════════════════════════════

use crate::l0_compiler;
use crate::os_layer::{OsError, OsLayer};
use crate::mccp_vm;
use crate::mccp::Mccp;

// ────────────────────────────────────────────────────────────
// 工具角色
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolRole {
    Classifier,
    Compiler,
    Transpiler,
    Analyzer,
}

impl ToolRole {
    pub fn tag(self) -> &'static str {
        match self {
            ToolRole::Classifier => "classifier",
            ToolRole::Compiler => "compiler",
            ToolRole::Transpiler => "transpiler",
            ToolRole::Analyzer => "analyzer",
        }
    }

    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "classifier" => Some(ToolRole::Classifier),
            "compiler" => Some(ToolRole::Compiler),
            "transpiler" => Some(ToolRole::Transpiler),
            "analyzer" => Some(ToolRole::Analyzer),
            _ => None,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 工具清单项
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ToolManifestEntry {
    pub name: String,
    pub source_path: String,
    pub role: ToolRole,
    pub entry_point: Option<String>,
}

impl ToolManifestEntry {
    pub fn serialize(&self) -> String {
        match &self.entry_point {
            Some(ep) => format!("{}|{}|{}|{}", self.name, self.source_path, self.role.tag(), ep),
            None => format!("{}|{}|{}", self.name, self.source_path, self.role.tag()),
        }
    }

    pub fn deserialize(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }
        let parts: Vec<&str> = trimmed.split('|').collect();
        if parts.len() < 3 {
            return None;
        }
        let role = ToolRole::from_tag(parts[2].trim())?;
        let entry_point = parts.get(3).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        Some(ToolManifestEntry {
            name: parts[0].trim().to_string(),
            source_path: parts[1].trim().to_string(),
            role,
            entry_point,
        })
    }
}

// ────────────────────────────────────────────────────────────
// 已注册工具
// ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct RegisteredTool {
    pub name: String,
    pub role: ToolRole,
    pub bytecode: Vec<Mccp>,
}

// ────────────────────────────────────────────────────────────
// ToolRegistry — 运行时工具注册表
// ────────────────────────────────────────────────────────────

pub struct ToolRegistry {
    tools: Vec<RegisteredTool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: RegisteredTool) {
        self.tools.retain(|t| t.name != tool.name);
        self.tools.push(tool);
    }

    pub fn get(&self, name: &str) -> Option<&RegisteredTool> {
        self.tools.iter().find(|t| t.name == name)
    }

    pub fn get_by_role(&self, role: ToolRole) -> Vec<&RegisteredTool> {
        self.tools.iter().filter(|t| t.role == role).collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.tools.iter().any(|t| t.name == name)
    }

    pub fn list(&self) -> &[RegisteredTool] {
        &self.tools
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

// ────────────────────────────────────────────────────────────
// BootstrapFS
// ────────────────────────────────────────────────────────────

pub struct BootstrapFS<'a> {
    os: &'a dyn OsLayer,
    manifest_path: String,
}

impl<'a> BootstrapFS<'a> {
    pub fn new(os: &'a dyn OsLayer, manifest_path: &str) -> Self {
        Self { os, manifest_path: manifest_path.to_string() }
    }

    /// 将清单内的相对路径解析为基于清单父目录的绝对路径
    fn resolve_path(&self, relative: &str) -> String {
        if std::path::Path::new(relative).is_absolute() {
            return relative.to_string();
        }
        let manifest_parent = std::path::Path::new(&self.manifest_path)
            .parent()
            .and_then(|p| p.to_str())
            .filter(|p| !p.is_empty());
        match manifest_parent {
            Some(parent) => format!("{}/{}", parent, relative),
            None => relative.to_string(),
        }
    }

    pub fn manifest_exists(&self) -> bool {
        self.os.file_exists(&self.manifest_path)
    }

    pub fn read_manifest(&self) -> Result<Vec<ToolManifestEntry>, OsError> {
        let data = self.os.file_read(&self.manifest_path)?;
        let text = String::from_utf8_lossy(&data);
        Ok(text.lines().filter_map(ToolManifestEntry::deserialize).collect())
    }

    pub fn build_all(&self, registry: &mut ToolRegistry) -> Result<usize, BootstrapError> {
        let entries = self.read_manifest().map_err(BootstrapError::Os)?;
        let mut built = 0;
        for entry in &entries {
            let full_path = self.resolve_path(&entry.source_path);
            let source = self.os.file_read(&full_path)
                .map_err(|e| BootstrapError::SourceRead(entry.source_path.clone(), e))?;
            let source_text = String::from_utf8_lossy(&source);
            let bytecode = l0_compiler::compile_l0(&source_text)
                .map_err(|e| BootstrapError::Compile(entry.name.clone(), e))?;
            registry.register(RegisteredTool {
                name: entry.name.clone(),
                role: entry.role,
                bytecode,
            });
            built += 1;
        }
        Ok(built)
    }

    pub fn build_one(&self, tool_name: &str, registry: &mut ToolRegistry) -> Result<(), BootstrapError> {
        let entries = self.read_manifest().map_err(BootstrapError::Os)?;
        let entry = entries.iter()
            .find(|e| e.name == tool_name)
            .ok_or_else(|| BootstrapError::NotFound(tool_name.to_string()))?;
        let full_path = self.resolve_path(&entry.source_path);
        let source = self.os.file_read(&full_path)
            .map_err(|e| BootstrapError::SourceRead(entry.source_path.clone(), e))?;
        let source_text = String::from_utf8_lossy(&source);
        let bytecode = l0_compiler::compile_l0(&source_text)
            .map_err(|e| BootstrapError::Compile(entry.name.clone(), e))?;
        registry.register(RegisteredTool {
            name: entry.name.clone(),
            role: entry.role,
            bytecode,
        });
        Ok(())
    }

    pub fn status(&self, registry: &ToolRegistry) -> Result<Vec<(String, bool)>, OsError> {
        let entries = self.read_manifest()?;
        Ok(entries.into_iter().map(|e| (e.name.clone(), registry.contains(&e.name))).collect())
    }

    pub fn read_source(&self, source_path: &str) -> Result<String, OsError> {
        let data = self.os.file_read(source_path)?;
        Ok(String::from_utf8_lossy(&data).to_string())
    }
}

// ────────────────────────────────────────────────────────────
// Errors
// ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum BootstrapError {
    Os(OsError),
    SourceRead(String, OsError),
    Compile(String, l0_compiler::CompileError),
    NotFound(String),
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Os(e) => write!(f, "OS error: {}", e),
            Self::SourceRead(p, e) => write!(f, "Read '{}' failed: {}", p, e),
            Self::Compile(name, e) => write!(f, "Compile '{}' failed: {}", name, e),
            Self::NotFound(name) => write!(f, "'{}' not in manifest", name),
        }
    }
}

impl std::error::Error for BootstrapError {}

// ────────────────────────────────────────────────────────────
// Seed — Rust 常量 → 磁盘文件
// ────────────────────────────────────────────────────────────

fn ensure_dir(path: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
}

pub fn seed_manifest(os: &dyn OsLayer, manifest_path: &str) -> Result<bool, OsError> {
    if os.file_exists(manifest_path) { return Ok(false); }
    ensure_dir(manifest_path);
    os.file_write(manifest_path, build_manifest_text().as_bytes())?;
    Ok(true)
}

pub fn seed_tools(os: &dyn OsLayer, base_dir: &str) -> Result<usize, OsError> {
    let mut seeded = 0;
    let base = base_dir.trim_end_matches('/');
    let classifier_path = format!("{}/l0_classifier.l0", base);
    let bootstrapper_path = format!("{}/bootstrapper.l0", base);

    if !os.file_exists(&classifier_path) {
        ensure_dir(&classifier_path);
        os.file_write(&classifier_path, seed_l0_classifier().as_bytes())?;
        seeded += 1;
    }
    if !os.file_exists(&bootstrapper_path) {
        ensure_dir(&bootstrapper_path);
        os.file_write(&bootstrapper_path, seed_bootstrapper().as_bytes())?;
        seeded += 1;
    }
    Ok(seeded)
}

/// 全量播种: 在 base_dir 下生成 manifest.tbm + tools/ 子目录
/// manifest 中的路径与 tools 目录保持相对一致
pub fn seed_all(os: &dyn OsLayer, base_dir: &str) -> Result<(bool, usize), OsError> {
    let base = base_dir.trim_end_matches('/');
    let tools_dir = format!("{}/tools", base);
    let manifest_path = format!("{}/manifest.tbm", base);
    let manifest_seeded = seed_manifest(os, &manifest_path)?;
    let tools_seeded = seed_tools(os, &tools_dir)?;
    Ok((manifest_seeded, tools_seeded))
}

fn build_manifest_text() -> String {
    String::from(
"# 自举编译器 工具清单
# 格式: 工具名 | 源文件路径 | 角色 | [入口标签]
l0_classifier|tools/l0_classifier.l0|classifier
bootstrapper|tools/bootstrapper.l0|compiler
")
}

fn seed_l0_classifier() -> String {
    String::from(r#"
; L0 分类器 v1 — 字符流 → IR token 字节流
wl = 0
ba = 0
bb = 0
bc = 0
bd = 0
be = 0
bf = 0
nu = 0
nf = 0
ng = 0

loop:
  input c
  if c == -1 then
    jmp flush
  end
  if c >= 48 then
    if c <= 57 then
      if nf == 0 then
        nf = 1
        nu = c
        nu = nu - 48
        jmp loop
      end
      nu = nu * 10
      nu = nu + c
      nu = nu - 48
      jmp loop
    end
  end
  if nf == 1 then
    writeraw 224
    ng = nu
    ng = ng / 256
    writeraw ng
    ng = nu
    ng = ng % 256
    writeraw ng
    nf = 0
    nu = 0
  end
  if c >= 97 then
    if c <= 122 then
      if wl == 0 then
        ba = c
        wl = 1
        jmp loop
      end
      if wl == 1 then
        bb = c
        wl = 2
        jmp loop
      end
      if wl == 2 then
        bc = c
        wl = 3
        jmp loop
      end
      if wl == 3 then
        bd = c
        wl = 4
        jmp loop
      end
      if wl == 4 then
        be = c
        wl = 5
        jmp loop
      end
      if wl == 5 then
        bf = c
        wl = 6
        jmp loop
      end
      jmp loop
    end
  end
  if c >= 65 then
    if c <= 90 then
      writeraw 226
      writeraw c
      jmp loop
    end
  end
  if c == 10 then
    writeraw 227
    jmp loop
  end
  if c == 32 then
    jmp loop
  end
  if c == 43 then
    writeraw 16
    jmp loop
  end
  if c == 45 then
    writeraw 17
    jmp loop
  end
  if c == 42 then
    writeraw 18
    jmp loop
  end
  if c == 47 then
    writeraw 19
    jmp loop
  end
  if c == 37 then
    writeraw 20
    jmp loop
  end
  if c == 61 then
    writeraw 48
    jmp loop
  end
  if c == 60 then
    writeraw 50
    jmp loop
  end
  if c == 62 then
    writeraw 51
    jmp loop
  end
  if c == 59 then
    jmp loop
  end
  jmp loop

flush:
  if nf == 1 then
    writeraw 224
    ng = nu
    ng = ng / 256
    writeraw ng
    ng = nu
    ng = ng % 256
    writeraw ng
    nf = 0
    nu = 0
  end
  if wl > 0 then
    if wl == 1 then
      ng = ba
      ng = ng - 97
      writeraw 225
      writeraw ng
      jmp flush_done
    end
    if wl == 2 then
      if ba == 105 then
        if bb == 102 then
          writeraw 242
          jmp flush_done
        end
      end
      if ba == 100 then
        if bb == 111 then
          writeraw 247
          jmp flush_done
        end
      end
      if ba == 106 then
        if bb == 122 then
          writeraw 249
          jmp flush_done
        end
      end
    end
    if wl == 3 then
      if ba == 101 then
        if bb == 110 then
          if bc == 100 then
            writeraw 245
            jmp flush_done
          end
        end
      end
      if ba == 114 then
        if bb == 101 then
          if bc == 116 then
            writeraw 252
            jmp flush_done
          end
        end
      end
      if ba == 106 then
        if bb == 109 then
          if bc == 112 then
            writeraw 248
            jmp flush_done
          end
        end
      end
      if ba == 106 then
        if bb == 110 then
          if bc == 122 then
            writeraw 250
            jmp flush_done
          end
        end
      end
    end
    if wl == 4 then
      if ba == 116 then
        if bb == 104 then
          if bc == 101 then
            if bd == 110 then
              writeraw 243
              jmp flush_done
            end
          end
        end
      end
      if ba == 101 then
        if bb == 108 then
          if bc == 115 then
            if bd == 101 then
              writeraw 244
              jmp flush_done
            end
          end
        end
      end
      if ba == 99 then
        if bb == 97 then
          if bc == 108 then
            if bd == 108 then
              writeraw 251
              jmp flush_done
            end
          end
        end
      end
      if ba == 104 then
        if bb == 97 then
          if bc == 108 then
            if bd == 116 then
              writeraw 253
              jmp flush_done
            end
          end
        end
      end
    end
    if wl == 5 then
      if ba == 112 then
        if bb == 114 then
          if bc == 105 then
            if bd == 110 then
              if be == 116 then
                writeraw 240
                jmp flush_done
              end
            end
          end
        end
      end
      if ba == 119 then
        if bb == 104 then
          if bc == 105 then
            if bd == 108 then
              if be == 101 then
                writeraw 246
                jmp flush_done
              end
            end
          end
        end
      end
      if ba == 105 then
        if bb == 110 then
          if bc == 112 then
            if bd == 117 then
              if be == 116 then
                writeraw 241
                jmp flush_done
              end
            end
          end
        end
      end
    end
    flush_done:
      wl = 0
      ba = 0
      bb = 0
      bc = 0
      bd = 0
      be = 0
      bf = 0
  end
  writeraw 239
  halt
"#)
}

fn seed_bootstrapper() -> String {
    String::from(r#"
; Bootstrapper — 自举编译管线协调器 (简化版)
s = 0
i = 1
while i < 101 do
  s = s + i
  i = i + 1
end
print s
"#)
}

// ────────────────────────────────────────────────────────────
// IR 解码
// ────────────────────────────────────────────────────────────

pub type IrToken = (u16, i32);

pub fn decode_ir_stream(raw: &[u8]) -> Vec<IrToken> {
    let mut toks = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        let byte = raw[i] as u16;
        match byte {
            0xE0 => {
                if i + 2 < raw.len() {
                    let hi = raw[i + 1] as i32;
                    let lo = raw[i + 2] as i32;
                    let val = (hi << 8) | lo;
                    toks.push((0xE0, val));
                    i += 3;
                } else { break; }
            }
            0xE1 => {
                if i + 1 < raw.len() {
                    toks.push((0xE1, raw[i + 1] as i32));
                    i += 2;
                } else { break; }
            }
            0xE2 => {
                if i + 1 < raw.len() {
                    toks.push((0xE2, raw[i + 1] as i32));
                    i += 2;
                } else { break; }
            }
            0xEF => { toks.push((0xEF, 0)); break; }
            0xE3 => { toks.push((0xE3, 0)); i += 1; }
            0xF0..=0xFE => { toks.push((byte, 0)); i += 1; }
            16 | 17 | 18 | 19 | 20 | 48 | 50 | 51 => { toks.push((byte, 0)); i += 1; }
            _ => { i += 1; }
        }
    }
    toks
}

// ────────────────────────────────────────────────────────────
// Toolchain
// ────────────────────────────────────────────────────────────

pub struct Toolchain {
    pub registry: ToolRegistry,
}

impl Toolchain {
    pub fn new() -> Self {
        Self { registry: ToolRegistry::new() }
    }

    pub fn build_from_fs(os: &dyn OsLayer, manifest_path: &str) -> Result<(Self, usize), BootstrapError> {
        let bs = BootstrapFS::new(os, manifest_path);
        let mut registry = ToolRegistry::new();
        let built = bs.build_all(&mut registry)?;
        Ok((Self { registry }, built))
    }

    pub fn compile(&self, source: &str) -> Result<Vec<Mccp>, BootstrapError> {
        let classifier = self.registry.get("l0_classifier")
            .ok_or(BootstrapError::NotFound("l0_classifier".to_string()))?;
        use crate::mccp_vm::Vm;
        let mut vm = Vm::with_input(classifier.bytecode.clone(), Default::default(), source.into());
        vm.run().map_err(|e| BootstrapError::Os(OsError::Other(format!("VM: {:?}", e))))?;
        let ir = decode_ir_stream(&vm.output_raw);
        let bytecode = crate::l0_self_hosting::compile_ir_v2(&ir)
            .map_err(|e| BootstrapError::Os(OsError::Other(format!("IR: {:?}", e))))?;
        Ok(bytecode)
    }

    pub fn run(&self, source: &str) -> Result<(String, u64), BootstrapError> {
        let bc = self.compile(source)?;
        let mut vm = mccp_vm::Vm::new(bc);
        vm.run().map_err(|e| BootstrapError::Os(OsError::Other(format!("Run: {:?}", e))))?;
        Ok((vm.output.clone(), vm.steps))
    }
}

// ────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_layer::OsError;

    #[derive(Debug)]
    struct MemFs {
        files: std::sync::RwLock<std::collections::HashMap<String, Vec<u8>>>,
    }

    impl MemFs {
        fn new() -> Self {
            Self { files: std::sync::RwLock::new(std::collections::HashMap::new()) }
        }
    }

    impl OsLayer for MemFs {
        fn alloc(&self, _: usize) -> *mut u8 { std::ptr::null_mut() }
        fn dealloc(&self, _: *mut u8, _: usize) {}
        fn file_read(&self, path: &str) -> Result<Vec<u8>, OsError> {
            self.files.read().unwrap().get(path).cloned()
                .ok_or(OsError::FileNotFound(path.to_string()))
        }
        fn file_write(&self, path: &str, data: &[u8]) -> Result<(), OsError> {
            self.files.write().unwrap().insert(path.to_string(), data.to_vec());
            Ok(())
        }
        fn file_delete(&self, path: &str) -> Result<(), OsError> {
            self.files.write().unwrap().remove(path);
            Ok(())
        }
        fn file_exists(&self, path: &str) -> bool {
            self.files.read().unwrap().contains_key(path)
        }
        fn time_millis(&self) -> u64 { 0 }
        fn net_send(&self, _: &str, _: u16, _: &[u8]) -> Result<(), OsError> { Ok(()) }
        fn net_recv(&self, _: &str, _: u16, _: &mut [u8]) -> Result<usize, OsError> { Ok(0) }
        fn print(&self, _: &str) {}
        fn platform(&self) -> &str { "test" }
        fn perf_counter_ns(&self) -> u64 { 0 }
    }

    #[test]
    fn manifest_roundtrip() {
        let entry = ToolManifestEntry {
            name: "cls".to_string(),
            source_path: "tools/cls.l0".to_string(),
            role: ToolRole::Classifier,
            entry_point: Some("main".to_string()),
        };
        let line = entry.serialize();
        let parsed = ToolManifestEntry::deserialize(&line).unwrap();
        assert_eq!(parsed.name, "cls");
        assert_eq!(parsed.entry_point, Some("main".to_string()));
    }

    #[test]
    fn registry_duplicate_overrides() {
        let mut reg = ToolRegistry::new();
        reg.register(RegisteredTool { name: "a".to_string(), role: ToolRole::Classifier, bytecode: vec![] });
        reg.register(RegisteredTool { name: "a".to_string(), role: ToolRole::Compiler, bytecode: vec![] });
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.get("a").unwrap().role, ToolRole::Compiler);
    }

    #[test]
    fn decode_ir_int() {
        let raw = vec![0xE0, 0, 42, 0xEF];
        let toks = decode_ir_stream(&raw);
        assert_eq!(toks, vec![(0xE0, 42), (0xEF, 0)]);
    }

    #[test]
    fn seed_manifest_idempotent() {
        let fs = MemFs::new();
        assert!(seed_manifest(&fs, "m.tbm").unwrap());
        assert!(!seed_manifest(&fs, "m.tbm").unwrap());
    }

    #[test]
    fn build_all_from_fs() {
        let fs = MemFs::new();
        fs.file_write("m.tbm", b"cls|cls.l0|classifier\n").unwrap();
        fs.file_write("cls.l0", b"print 7 + 8\n").unwrap();
        let bs = BootstrapFS::new(&fs, "m.tbm");
        let mut reg = ToolRegistry::new();
        assert_eq!(bs.build_all(&mut reg).unwrap(), 1);
        assert_eq!(reg.get("cls").unwrap().role, ToolRole::Classifier);
    }
}
