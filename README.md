# Rust

A repository to store my work as I learn Rust

## Cargo

The initial state of this project can be recreated by executing `cargo new [projectName]` from the terminal.

* To build: `cargo build`
* To build & run: `cargo run`
* To do a compile check (but not build an executable): `cargo check`
* To build an optimized executable: `cargo build --release`
* To update dependencies: `cargo update`
* To generate dependency documentation: `cargo doc --open`

## Basics

Rust has 4 scalar types:

* Integers (i8, i16 ... i128 for signed, u8 ... u128 for unsigned)
* Floats (f32 and f64)
* Booleans (bool)
* Characters (char, uses single quotes, 4 bytes unicode compliant)

And 2 compound types:

* Tuple (collection of indexed types i.e. `(u32, f64, boo)`)
* Array (fixed size & single type, i.e. `[i32; 5]`)

Rust uses snake_case for fn and let names. fn has Swift-style parameters and
return type declaration, i.e. `fn plus_one(x: i32) -> i32`.  It uses the
following semantic concepts:

* Statement: performs an action with no return value. Has trailing `;`
* Expression: evaluates to a return value.  Has no trailing `;`
* Selection: evaluates a bool.  Swift syntax style.
* Iteration: same `for` & `while` as Swift, plus `loop` (needs `break`)
