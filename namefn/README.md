# namenf

The namefn crate provides a flag for retriving the name of a function. Use the `#[name]` flag to get the function name. You can access the name via the `NAME` variable. The name is a `&str`, by default.
This crate is extremly usefull for logging and other tracking purposes.

## Example

```Rust
use namefn::name;

#[name]
fn main() {
    println!("{}", NAME)
}
```

This example prints `main`.

You can also crate a custom function name:

```Rust
use namefn::name;

#[name(alias = "cool_name")]
fn uncool_name() {
    println!("{}", NAME);
}
```

This would print `cool_name` instead.

If you for some reason already have a constant called name you can also rename the constant.

```Rust
use namefn::name;

#[name(FUNCTION)]
fn main() {
    println!("{}", FUNCTION);
}
```

This would still print `main` but you now use `FUNCTION` as the name const.
