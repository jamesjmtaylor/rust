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

You can create and reference your own modules like so:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

`src/main.rs` contents:

```rust
//Just like a Swift import statement, `use` shortens an absolute path.
//You can also just use a glo operator, i.e. `use std::collections::*;`
use crate::garden::vegetables::Asparagus;

//`pub` sets access to public. Modules, structs, and fns are private by default.
pub mod garden; //tells the compiler to include code it finds in src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

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

3+ collection types:

* Vector (Variable # of values, i.e. `let v: Vec<i32> = Vec::new();`)
* HashMap (Variable # of key-values, i.e. `let mut scores = HashMap::new();`)
* String (Variable # of characters, i.e. `let mut s = String::new();`)

NOTE: Srings do not support indexing, and slicing can cause runtime exceptions.

Generic types are represented with `T` by convention.  To differentiate generic
types in the same struct use `T1`, `T2`, etc. Generic types are just as
performant as any other type due to compile-time monomorphization.

Lifetimes are a kind of generic. Rather than ensuring that a type has the 
behavior we want, lifetimes ensure that references are valid as long as we need
 them to be.

```rust
i32          // a pointer
&i32         // a reference
&'a i32      // a reference with an explicit lifetime
&'a mut i32  // a mutable reference with an explicit lifetime
&'static i32 // a static explicit lifetime
```

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

You can use an `if let` statement to unwrap optionals, just like Swift:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else {
    println!("The maximum is null!");
}
```

Enum types are also used to represent error cases, and can be unwrapped as well:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
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

Rust uses "traits" for interfaces, just like Swift. Traits can have default
implementations and be passed as parameters, i.e.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String
    fn summarize(&self) -> String { 
        String::from("(Read more...)") // Default implementation
    }
}

impl Summary for Tweet { 
    //NOTE: Does not need `summarize` because of default implentation above
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
} 

//Trait as a Parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Multiple trait bounds for a single parameter (not possible in Swift)
pub fn notify(item: &(impl Summary + Display)) {
    ...
}

//Returning a Trait type:
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
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

### Errors

Normally you get the last line from the backtrace after a runtime panic. To get
more use `RUST_BACKTRACE=1 cargo run`.

Some structs have methods that provide closures instead of Result
types. In the exampled below the File struct has a `unwrap_or_else`method that
returns a closure with an error that the error type is then matched against:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

`unwrap`, `expect`, and `?` provide shortcuts for handling errors, but are not
safe on their own. Examples of each are below:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    //`unwrap()` same as `!!` in swift. `expect()` is `!!` with custom error msg.
    let mut username_file = File::open("hello.txt").unwrap();
    let mut username = String::new();
    //`?` will return early out of whole function with error as result. Error
    //must be a return type of the function
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

You can chain `?` for even shorter expressions like this:
`File::open("hello.txt")?.read_to_string(&mut username)?;`

For examples, prototype code, and tests you should panic! instead of returning a
Result type. After prototype code becomes production code you should return
Response types wherever possible.
