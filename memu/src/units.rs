#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    bytes_impl::data_impl,
    constants::{BYTE, GIGABYTE, KILOBYTE, MEGABYTE, PETABYTE, TERABYTE},
};

/// Represents an amount of bytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Byte(u64);

impl Byte {
    /// Amout of bytes in this unit.
    pub const FACTOR: u64 = BYTE;

    /// Unit characters.
    #[cfg(feature = "units")]
    pub const UNIT: &str = "B";

    /// Creates a new instance of `Byte` where `bytes` represents any amount of bytes.
    pub fn new(bytes: u64) -> Self {
        Byte(bytes)
    }
}

data_impl!(Byte);

/// Represents a number of kilo bytes. Holds an amount of bytes internally.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KiloByte(u64);

impl KiloByte {
    /// Amout of bytes in this unit.
    pub const FACTOR: u64 = KILOBYTE;

    /// Unit characters.
    #[cfg(feature = "units")]
    pub const UNIT: &str = "KB";

    /// Creates a new instance of `KiloByte` where `bytes` represents any amount of bytes.
    pub fn new(bytes: u64) -> Self {
        KiloByte(bytes)
    }
}

data_impl!(KiloByte);

/// Represents a number of mega bytes. Holds an amount of bytes internally.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MegaByte(u64);

impl MegaByte {
    /// Amout of bytes in this unit.
    pub const FACTOR: u64 = MEGABYTE;

    /// Unit characters.
    #[cfg(feature = "units")]
    pub const UNIT: &str = "MB";

    /// Creates a new instance of `MegaByte` where `bytes` represents any amount of bytes.
    pub fn new(bytes: u64) -> Self {
        MegaByte(bytes)
    }
}

data_impl!(MegaByte);

/// Represents a number of giga bytes. Holds an amount of bytes internally.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GigaByte(u64);

impl GigaByte {
    /// Amout of bytes in this unit.
    pub const FACTOR: u64 = GIGABYTE;

    /// Unit characters.
    #[cfg(feature = "units")]
    pub const UNIT: &str = "GB";

    /// Creates a new instance of `GigaByte` where `bytes` represents any amount of bytes.
    pub fn new(bytes: u64) -> Self {
        GigaByte(bytes)
    }
}

data_impl!(GigaByte);

/// Represents a number of tera bytes. Holds an amount of bytes internally.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TeraByte(u64);

impl TeraByte {
    /// Amout of bytes in this unit.
    pub const FACTOR: u64 = TERABYTE;

    /// Unit characters.
    #[cfg(feature = "units")]
    pub const UNIT: &str = "TB";

    /// Creates a new instance of `TeraByte` where `bytes` represents any amount of bytes.
    pub fn new(bytes: u64) -> Self {
        TeraByte(bytes)
    }
}

data_impl!(TeraByte);

/// Represents a number of peta bytes. Holds an amount of bytes internally.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PetaByte(u64);

impl PetaByte {
    /// Amout of bytes in this unit.
    pub const FACTOR: u64 = PETABYTE;

    /// Unit characters.
    #[cfg(feature = "units")]
    pub const UNIT: &str = "PB";

    /// Creates a new instance of `PetaByte` where `bytes` represents any amount of bytes.
    pub fn new(bytes: u64) -> Self {
        PetaByte(bytes)
    }
}

data_impl!(PetaByte);
