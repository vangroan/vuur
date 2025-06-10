//! Constants describing the compiler limitations.

/// This is the maximum depth that interpolated strings may be nested.
/// The actual number is arbitrary, to be honest.
///
/// In this example, the nesting is 3 levels deep.
///
/// ```non-rust
/// " %( " %( " %( 1 + 2 ) " ) " ) "
/// ```
pub const MAX_INTERP_DEPTH: usize = 8;
