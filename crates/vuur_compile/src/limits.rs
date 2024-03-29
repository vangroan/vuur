/// Maximum number of constant indices allowed in a scope.
/// Limited by 24-bit instruction argument.
pub const MAX_CONSTANTS: usize = 0xFFFFFF;

/// Maxium number of functions allowed in a chunk.
pub const MAX_FUNCS: usize = 0x100_0000;

/// Maximum number of constant string values allowed in a scope.
pub const MAX_STRINGS: usize = 0xFFFFFF;

/// Maximum number of local variables allowed in a scope.
pub const MAX_LOCALS: usize = 0xFFFFFF;
