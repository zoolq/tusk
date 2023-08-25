/// The amount of bits in a byte.
pub const BITS: u64 = 8;

/// Amount of bytes in a byte. This is obviously one but is used for reference.
/// This is not recomended you use it.
pub const BYTE: u64 = 1;

/// Amount of bytes in a kilo byte.
pub const KILOBYTE: u64 = 1024;

/// Amount of bytes in a mega byte.
pub const MEGABYTE: u64 = KILOBYTE * 1024;

/// Amount of bytes in a giga byte.
pub const GIGABYTE: u64 = MEGABYTE * 1024;

/// Amount of bytes in a tera byte.
pub const TERABYTE: u64 = GIGABYTE * 1024;

/// Amount of bytes in a peta byte.
pub const PETABYTE: u64 = TERABYTE * 1024;

/// Amount of bytes in a exa byte.
pub const EXABYTE: u64 = PETABYTE * 1024;

/// Amount of bytes in a zetta byte.
pub const ZETTABYTE: u128 = EXABYTE as u128 * 1024;

/// Amount of bytes in a yotta byte.
#[deprecated = "Unplausible amount of storage"]
pub const YOTTABYTE: u128 = ZETTABYTE * 1024;

/// Amount of bytes in a ronna byte.
#[deprecated = "Unplausible amount of storage"]
#[allow(deprecated)]
pub const RONNABYTE: u128 = YOTTABYTE * 1024;

/// Amount of bytes in a quetta byte.
#[deprecated = "Unplausible amount of storage"]
#[allow(deprecated)]
pub const QUETTABYTE: u128 = RONNABYTE * 1024;
