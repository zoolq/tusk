#![allow(clippy::tabs_in_doc_comments)]
#![deny(missing_docs)]

/*!
# memu
The memu crate provides rust implementations of digital storage (or memory) units.
The maximum amount of storage is `18446.744073709553 Petabyte`, conversion between units always happends with a factor of `1024`.

The create also provides conversions between units and information about the units such as, unit suffixes and scale factors. As well as optional serde compatibility.

Licensed under MIT.

# Example
In the following example we first crate a `KiloByte` with 913 Kilobytes. We then print it as Gigabytes.

```Rust
use memu::units::MegaByte;

let kilo_byte = MegaByte::from(913);

println!("{}", kilo_byte.as_giga_byte().as_string_with_unit());

```

In the next example we first fetch some system info using the `sysinfo` crate and then print the memory usage for every process.
We use the `KiloByte::new()` method here, since `process.memory()` returns bytes and the new method constructs the unit from an amount of bytes.


```Rust
use memu::units::KiloByte;
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
	let mut sys = System::new_all();
	sys.refresh_all();

	for (pid, process) in sys.processes() {
		let memory = KiloByte::new(process.memory());
		println!("{}: {}", pid, memory.as_string_with_unit())
	}
}
```

Now we use normal addition with the units to sum all of the processes memory.

```Rust
use memu::units::Byte;
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
	let mut sys = System::new_all();
	sys.refresh_all();

	let mut total = Byte::default();

	for (pid, process) in sys.processes() {
		total += process.memory();
	}

	println!("Total: {}", total.as_gigabyte().as_string_with_unit())

	let average = total / sys.processes().len();

	println!("Average: {}", average.as_gigabyte().as_string_with_unit())
}
```

# Features
The crate contains the following features:
-  serde, serialize and deserialize data with the serde crate.
-  macro, use macros to create the storage structs. Enabled by default.
-  units, include text units such as "KB" and "TB". Enabled by default.
*/

use constants::{GIGABYTE, KILOBYTE, MEGABYTE, TERABYTE};
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

/// This functions finds the first memory unit without a
/// leading 0 infront of the comma.
#[inline(never)]
pub fn best_fit(bytes: u64) -> Box<dyn MemoryUnit> {
	// There most likley is a faster way to do this using bitshifts by 10
	// and then comparing something..?
	if bytes < KILOBYTE {
		Box::new(Byte::new(bytes))
	} else if bytes < MEGABYTE {
		Box::new(KiloByte::new(bytes))
	} else if bytes < GIGABYTE {
		Box::new(MegaByte::new(bytes))
	} else if bytes < TERABYTE {
		Box::new(GigaByte::new(bytes))
	} else {
		Box::new(PetaByte::new(bytes))
	}
}
