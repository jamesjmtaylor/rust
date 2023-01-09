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

### Types

Rust has 4 scalar types:

* Integers (i8, i16 ... i128 for signed, u8 ... u128 for unsigned)
* Floats (f32 and f64)
* Booleans (bool)
* Characters (char, uses single quotes, 4 bytes unicode compliant)

2 compound types:

* Tuple (collection of indexed types i.e. `(u32, f64, boo)`)
* Array (fixed size & single type, i.e. `[i32; 5]`)

You can also define your own enum types. Enum types can contain attributes.
Nullability is covered by the predifined Option enum, i.e.

```rust
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_char = Some('e');
    let absent_number: Option<i32> = None
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // Default case
    }
}
```

You can also use an `if let` statement to unwrap optionals, just like Swift:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else {
    println!("The maximum is null!");
}
```

Rust always uses snake_case for fn and let names. fn has Swift-style parameters
and return type declaration, i.e. `fn plus_one(x: i32) -> i32`.  It uses the
following semantic concepts:

* Statement: performs an action with no return value. Has trailing `;`
* Expression: evaluates to a return value.  Has no trailing `;`
* Selection: evaluates a bool.  Swift syntax style.
* Iteration: same `for` & `while` as Swift, plus `loop` (needs `break`)

### Structs

Rust does not have classes, only structs. Unlike Swift, structs are either
entirely mutable or immutable.  You can make a partial deep copy like so:

```rust
let user1 = User {
    email, // String
    username, // String
    active: true, // Bool
    sign_in_count: 1, // i32
};
//NOTE: user1 is invalidated after this b/c ownership of its username String
//is passed to user2.
let mut user2 = User {
    email: String::from("another@example.com"),
    ..user1 //comes last to specify that remaining fields come from user1
};
```

You can also create tuple structs for brevity:

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

Or unit-like structs with no fields:

```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

Structs cannot have built-in methods.  Instead you add associated functions:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Borrows immutable self w/reference; we don't want to write, just read.
    fn area(&self) -> u32 { // if you need to write, use `&mut self`
        self.width * self.height
    }
}
```

### Memory

Memory is managed through ownership. There are two possible storage mechanisms:

* Stack: Stores & removes data in LIFO order. Data must have known, fixed size.
* Heap: Stores arbitrarily sized data at the first address with enough space.
Tracks data address with pointers, which are stored on a stack.

Ownership rules:

1. Each value has an owner
2. There can only be one owner at a time
3. When an owner goes out of scope, the value is dropped

Rust memory management is best demonstrated by the following assignments:

```rust
let x = 5;
let y = x;

let s1 = String::from("hello");
let s2 = s1;
```

Both allocate data to a stack.  The first two allocate copies of a scalar value.
The second two allocate copies of a pointer to the string values' heap address
(i.e. a shallow copy). But unlike a shallow copy, Rust invalidates the 1st
pointer after the 2nd is created. Trying to use the 1st will cause a compile
error. `clone()` creates deep copy of heap data, i.e. `s1.clone()`.

NOTE: Passing a heap pointer to a fn will cause it to invalidate afterwards!
There are a number of solutions to keep the heap in scope afterwards:

* Make a clone first (slow).
* Have the fn return the pointer in a tuple at the end (ugly).
* Pass a reference type.  

Passing a reference is like letting an inner scope "borrow" a pointer.
Reference types are created by prepending the parameter type and heap variable
 with `&`.  You can explicitly de-reference with `*`.
References are immutable by default. They can be made mutable with `&mut`, but
a mutable reference can only have one borrower (compile error otherwise).

You can borrow slice references like so:

```rust
let s = String::from("hello world");

let hello = &s[..5]; //equivalent to 0..5
let world = &s[6..]; //equivalent to 6..len
```

Generally it is better for a fn to have accept a slice as a parameter, rather
than a string, because you can use strings where slices are expected but not
the reverse.  `fn first_word(s: &String)` vs. `fn first_word(s: &str)`
