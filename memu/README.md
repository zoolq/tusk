# memu
The memu crate provides rust implementations of digital storage (or memory) units. 
The maximum amount of storage is `18446.744073709553 Petabyte`, conversion between units always happends with a factor of `1024`.
The create also provides conversions between units and information about the units such as, unit suffix and scale factors. As well as optional serde compatibility.

Licensed under MIT.

# Example
```Rust
use memu::units::{KiloByte};

let kilo_byte = KiloByte::from(913);

println!("{}", kilo_byte.as_giga_byte().print_with_unit());

```

In this example we first crate a `KiloByte` with 913 Kilobytes. We then print it as Gigabytes.

# Features
The crate contains the following features:
- serde, serialize and deserialize data with the serde crate.
- macro, use macros to create the storage structs. Enabled by default.
- units, include text based units such as "KB" and "TB". Enabled by default.
