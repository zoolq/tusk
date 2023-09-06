# namenf

The [namefn](https://crates.io/crates/namefn) crate provides a flag for retrieving the name of a function. Use the `#[name]` flag to get the function name. You can access the name via the `NAME` variable. The name is a `&str`, by default.
This crate is extremely useful for logging and other tracking purposes.

Licensed under MIT.

## Example

The basic functionality consists of getting the functions name via a const variable called `NAME`.

```Rust
use namefn::name;

#[name]
fn main() {
    assert_eq!("main", NAME);
}
```

You can also crate a custom function name:

```Rust
use namefn::name;

#[name(alias = "cool_name")]
fn uncool_name() {
    assert_eq!("cool_name", NAME);
}
```

Here the name is `cool_name` instead of `uncool_name`.

If you for some reason already have a constant called name you can also rename the constant.

```Rust
use namefn::name;

#[name(const = FUNCTION)]
fn main() {
    assert_eq!("main", FUNCTION);
}
```

The name is still `main` but the constant is now called `FUNCTION`.

Note: If the const attribute is lowercase it will be converted to uppercase.
