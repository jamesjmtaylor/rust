# Rust

A repository to store my work as I learn Rust

## Cargo

The initial state of this project can be recreated by executing
`cargo new [projectName]` from the terminal.

* To build: `cargo build`
* To build & run: `cargo run`
* To do a compile check (but not build an executable): `cargo check`
* To build an optimized executable: `cargo build --release`
* To update dependencies: `cargo update`
* To generate dependency documentation: `cargo doc --open`
* To create a library crate: `cargo new [libName] --lib`
* To install a binary crate for local use `cargo install [externalCrateName]`

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

In the cargo.toml you can declare additional profiles, i.e.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

See [Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) for more
information.

You can publish your own crate to crates.io.  Document library methods with `///`
Comments are formatted as markdown. Running `cargo doc --open` will build & open
the HTML for your current crate and its dependencies. Common sections include:

* Examples: How your function is used (automatically tested w/ `cargo test`)
* Panics: When your function could panic.
* Errors: For functions that return a `Result`
* Safety: How your function might be unsafe.

To document your crate, use `//!` at the top of the file.

To publish a crate, you must first login with `cargo login your_api_token`. You
can add metadata to your crate in the cargo.toml.  See [Cargo Metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
for more info.  Once ready, publish with `cargo publish`.

A workspace is a set of packages that share the same Cargo.lock and output
directory. See `./add` for an example.

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

You can use an `if let` statement to unwrap optionals, almost like Swift (you
need to wrap the unwrapped value in `Some()`):

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else {
    println!("The maximum is null!");
}
```

Unlike Swift, you can also use `while let` to unwrap arrays:

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
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

## Advanced Concepts

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

### Object Oriented Programming

Rust differs from mainstream OOP languages like Java in that it does not support
inheritence.  There is no way to define a struct that inherits the parent
struct’s fields and methods.

#### Polymorpism

In order to support polymorphism you should use
traits and generic types. An example where various UI components all support
the Draw trait is below:

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This is very similar to the pattern of "Interface-Oriented Programming" of
Kotlin & Swift.  It has the advantage of avoiding the inheritence "pyramid of
doom". Below is a depiction of how a component might implement the Draw trait:

```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

NOTE: generics that conform to traits cannot use static dispatch because the
compiler doesn’t know all the types that might be used with the code that’s
using trait object.  Instead, dynamic dispatch is used, which has a small
performance penalty.

#### Patterns and Matching

Matching is Rust's equivalent of the `switch` (Java & Swift) and has already
been used elsewhere to handle the Optional and Result structs. It can also use
the following:

* Literals: `1 | 2 => println!("one or two"),` or `1..=5 => println!("one through five"),`
* Destructured arrays: `while let x = vec.pop() {`
* Structs: `Some(50) => println!("Got 50"),`
* Enums: `Message::Write(text) =>  println!("Text message: {text}"),`
* Tuples: `let (a,b,c) = [1,2,3];`
* Variables: `x =>  println!("Got x")`
* Wildcards: `_ => println!("This is the match default case")`

Patterns are just variables assigned to expressions: `let PATTERN = EXPRESSION;`
They can be refutable or irrefutable. Patterns that match for any expression are
"irrefutable".  An example would be `x` in `let x = 5;` because `x` matches
anything and cannot fail to match. "Refutable" patterns are where a failure to
match is possible, i.e. `Some(x)` in the expression `if let Some(x) = a_value`.
Understanding the distinction is important for understanding error messages like
the one below:

```log
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0005]: refutable pattern in local binding: `None` not covered
 --> src/main.rs:3:9
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
note: `Option<i32>` defined here
  = note: the matched value is of type `Option<i32>`
help: you might want to use `if let` to ignore the variant that isn't matched
  |
3 |     let x = if let Some(x) = some_option_value { x } else { todo!() };
  |     ++++++++++                                 ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` due to previous error
```

Because we didn’t cover (and couldn’t cover!) every valid value with the pattern
Some(x), Rust rightfully produces a compiler error. This example can be fixed
with an `if let`. Inversely, you'll get a compiler warning if you use `if let`
on an irrefutable pattern such as `if let x = 5`.

### Functional Programming

The two primary functional paradigms in rust are Closures & Iterators. Because
of compile time loop "unrolling", there are no performance penalties to using
either of these concepts.

#### Closures

Rust supports passing closures as desired. If closures have parameters, they  
appear between the two vertical bars. Closure types are inferred, but can also
 be explicitly definied, as shown below.

```rust
let add_one_v2 = |x: u32| -> u32 { x + 1 }; //Max verbosity
let add_one_v4 = |x|               x + 1  ; //Min verbosity
```

The first type inferred for each parameter must be used through the program.
Closures passed to threads must explicitly declare a `move` to that thread:

```rust
let list = vec![1, 2, 3];
thread::spawn(move || println!("From thread: {:?}", list))
```

Without `move` the list object is invalidated in the spawning thread. There are
3 types of closure function traits that closures can implement:

1. FnOnce: all closures implement at least this trait. These can only be called
once, move captured values out their body, and implement none of the other Fn traits.
2. FnMut: all closures that don’t move captured values out of their body but
that might mutate the captured values implement this. Can be called more than once.
3. Fn: all closures that don’t move captured values out of their body or mutate
captured values, as well as closures that capture nothing from their environment.
Can be called more than once. Important in cases such as calling a closure
multiple times concurrently.

Using a closure trait as a function parameter type is shown below:

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

#### Iterators

Rust iterators work just like Swift iterators, and can be created from arrays
and vectors with `.iter()`. Iterator is a trait that your structs can implement.

```rust
pub trait Iterator {
    type Item; //defines an associated type with this trait
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations excluded
}
```

Iterators are consumed as they are iterated through. `for` loops actually apply
the `mut` keyword behind the scenes to accomplish `for val in vector_iterator {`.
`map` & `filter` operate on an iterator to generate a new iterator by applying
the specified closure function. To collect the results into a collection, use
`collect` afterwards.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

## Concurrency

### Threads

Each rust program runs in its own process.  A process can spawn multiple threads.
Rust operates on a 1:1 model, that is 1 rust thread = 1 OS thread (as opposed
to Kotlin corroutines which use an n:1 model). Thread creation is shown below:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

You can use thread handles, i.e. `let handle = thread::spawn(|| {` in order to
control when the thread is executed.  Calling `handle.join().unwrap();` joins
the handle into the declaring thread and causes it to execute synchronously in
that thread.

### Data Passing

If you want to use a variable from a calling thread in the called thread, you
must use the `move` keyword as shown below:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

You can also use channels to pass data from threads:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

The example above creates a mpsc (multiple producers, single consumer) channel.
That channel is then passed with the `move` keyword to a spawned thread.  The
thread then uses it to send a stream of values to the calling thread. Because
the channel is mpsc, you can call `let tx1 = tx.clone` in order to give another
thread a clone of the original transmitter.  Both will broadcast to the same rx.

### Shared-State Concurrency

Mutexes allow only one thread to access some data at any given time. You must
lock the data when you access it and unlock when finished.  Below is an example:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Note that we use "Arc", or Atomic Reference Counting, in order to share the
mutex across multiple threads.  If only one thread needs access at a time, use a
Mutex:

```rust
se std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
```

Mutexes return type Rc\<T>, while arcs return Arc\<T>.
Note even with the compile-time safety provided by Arc & Mutex, deadlocks are
still possible.

### Sync & Send Traits

Channels, mutexes, and arcs are part of the standard library and are built using
the Sync and Send traits, which are part of the language. Send is implemented by
almost every Rust type, and indicates that type can be sent across threads. Sync
indicates a type is can be referenced by multiple threads.  If a type is
composed completely of properties that conform to Sync & Send, it inherits
conformance by default.  Manually implementing these traits involves
implementing unsafe Rust code.