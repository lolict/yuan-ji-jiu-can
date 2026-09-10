//! MetaIME · FQM Ontology Quantization Engine
//! 元输入法 · FQM 本体论量化引擎
//!
//! 零运行时内存约束：
//! - 所有数据为编译时常量（const）或栈分配（固定大小数组）
//! - 禁止使用 Vec、String、Box 等堆分配类型
//! - 所有查找表用 const 数组嵌入
//! - 使用静态生命周期（'static）引用
//! - no_std 兼容

#![cfg_attr(feature = "no_std", no_std)]

pub mod constants;
pub mod loader;
pub mod linker;
pub mod translator;
pub mod interpreter;
pub mod radix3d;
pub mod dimension;
pub mod divisor;
pub mod yuan;
pub mod ffi;

// 重新导出核心类型
pub use constants::*;
pub use loader::{Container, FQMLoader};
pub use linker::{FQMLinker, OntologyPath};
pub use translator::FQMTranslator;
pub use interpreter::FQMInterpreter;
pub use radix3d::{Point3D, Radix3DInterpreter};
pub use dimension::DimensionInterpreter;
pub use divisor::DivisorInterpreter;
pub use yuan::YuanInterpreter;

/// 操作类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Load,
    Extract,
    Compare,
    DimensionShift,
    Link,
    Translate,
}

/// 容器操作 trait
pub trait ContainerOps {
    /// 装入符号
    fn load(&mut self, domain: &str, symbol: &str) -> Result<(), &'static str>;
    /// 取出符号
    fn extract(&self, domain: &str, identity: &str) -> Option<&'static str>;
    /// 比较两个符号
    fn compare(&self, domain: &str, sym1: &str, sym2: &str) -> bool;
}

/// 链接器操作 trait
pub trait LinkerOps {
    /// 查找符号的本体论路径
    fn lookup(&self, domain: &str, symbol: &str) -> Option<OntologyPath>;
    /// 解析链接
    fn resolve(&mut self) -> Result<(), &'static str>;
}
