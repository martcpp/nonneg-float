# nonneg-float

A generic Rust crate providing a wrapper for non-negative floating-point numbers with a convenient macro for safe construction.

## Features

- Generic over any floating-point type (`f32`, `f64`, etc.) implementing `num_traits::Float`.
- Ensures values are non-negative and finite.
- Macro `nonneg!` for easy, safe instantiation with optional defaulting to zero.
- Panics at runtime if negative values are used with the macro.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nonneg-float = "0.1.0"
num-traits = "0.2"

```

## Examples

```use nonneg_float::{NonNegative, nonneg};

fn main() {
    let a = nonneg!(f64);        // defaults to 0.0
    let b = nonneg!(5.5f64);     // from literal
    let c = nonneg!(f32, 3.14);  // explicit type and value

    println!("{}, {}, {}", a.get(), b.get(), c.get());
} 
```