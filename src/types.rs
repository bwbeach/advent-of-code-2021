/// Result type used throughout Advent of Code
pub type AdventResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The answer to each problem is a positive integer
pub type Answer = u64;
