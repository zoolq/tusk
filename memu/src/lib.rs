//!

use units::{Byte, GigaByte, KiloByte, MegaByte, PetaByte, TeraByte};

/// The core memory units.
pub mod units;

/// Memory constants for conversion between units.
pub mod constants;

/// Macros for easy creation of memory units.
#[doc(hidden)]
#[cfg(feature = "macro")]
pub mod macros;

#[doc(hidden)]
mod bytes_impl;

/// Trait for converting between memory units.
pub trait MemoryUnit {
	/// Converts the unit into `bits`.
	fn as_bits(&self) -> u128;
	/// Converts the unit into [`Byte`]s.
	fn as_byte(&self) -> Byte;
	/// Converts the unit into [`KiloByte`]s.
	fn as_kilo_byte(&self) -> KiloByte;
	/// Converts the unit into [`MegaByte`]s.
	fn as_mega_byte(&self) -> MegaByte;
	/// Converts the unit into [`GigaByte`]s.
	fn as_giga_byte(&self) -> GigaByte;
	/// Converts the unit into [`TeraByte`]s.
	fn as_tera_byte(&self) -> TeraByte;
	/// Converts the unit into [`PetaByte`]s.
	fn as_peta_byte(&self) -> PetaByte;
}
