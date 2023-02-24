/// Maximum number of constant indices allowed in a scope.
/// Limited by 24-bit instruction argument.
pub const MAX_CONSTANTS: usize = 0xFFFFFF;

/// Maximum number of constant string values allowed in a scope.
pub const MAX_STRINGS: usize = std::u8::MAX as usize;

/// Maximum number of local variables allowed in a scope.
pub const MAX_LOCALS: usize = std::u8::MAX as usize;
