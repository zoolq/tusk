# memu
The memu crate provides rust implementations of digital storage (or memory) units. 
The maximum amount of storage is `18446.744073709553 Petabyte`, conversion between units always happends with a factor of `1024`.
The create also provides conversions between units and information about the units such as, unit suffix and scale factors. As well as optional serde compatibility.

Dual-licensed under MIT or the UNLICENSE.

# Example
```Rust
use memu::units::{KiloByte};

let kilo_byte = KiloByte::from(913);

println!("{}", kilo_byte.as_giga_byte().print_with_unit());

```

In this example we first crate a `KiloByte` with 913 Kilobytes. We then print it as Gigabytes.
