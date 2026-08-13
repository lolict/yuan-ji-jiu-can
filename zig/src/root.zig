// root.zig - Quantitative Auxiliary State Engine · Zig Implementation
// hunyuan-os 's general layer: preload dictionary + hierarchical dispatch + quantification engine
//
// Language roles:
//   ZIG = general/strategist: preload dictionary, hierarchical command chain, dispatch system
//   RUST = guard: memory safety, type guarantee, core state machine
//   NIM = arsenal: tool manufacturing, code generation
//   C = remains: only for minimal boot / hardware operation

pub const ColorQuant = @import("color_quant.zig");
pub const ColorOntology = @import("color_ontology.zig");
pub const PreloadDict = @import("preload_dict.zig");
pub const Hierarchy = @import("hierarchy.zig");
pub const Brigade = @import("brigade.zig");

// Engine version
pub const VERSION: u32 = 0x51;

test "all modules" {
    _ = @import("color_quant.zig");
    _ = @import("color_ontology.zig");
    _ = @import("preload_dict.zig");
    _ = @import("hierarchy.zig");
    _ = @import("brigade.zig");
}
