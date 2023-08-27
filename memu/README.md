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
