# hotdrink-rs

HotDrink implemented in Rust.

HotDrink lets you describe relations between values declaratively and how to enforce them,
and can then automatically do so when the value of a variable changes.

[![Crates.io][crates-badge]][crates-url]
[![docs.rs](https://docs.rs/hotdrink-rs/badge.svg)](https://docs.rs/hotdrink-rs)

[crates-badge]: https://img.shields.io/crates/v/hotdrink-rs.svg
[crates-url]: https://crates.io/crates/hotdrink-rs

## Prerequisites

The project uses multiple nightly features, and must be built using nightly Rust.
I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/).

## Introduction

Before getting started, here is a quick introduction to the terminology and how it works.
A [`Component`](crate::model::Component) is a set of variables with a set of [`Constraint`](crate::model::Constraint)s between them.
A `Constraint` consists of a set of [`Method`](crate::model::Method)s that are essentially functions that enforce the constraint
by reading from some subset of the variables of the `Component` and writing to another.
`Components` can be gathered in a [`ConstraintSystem`](crate::model::ConstraintSystem), which provides an API
for interacting with multiple `Component`s at once, such as [`solve`](crate::model::ConstraintSystem::solve).

### Components

A *component* is a collection of variables and constraints between them that should be enforced.
One can easily be created by using the [`component!`] macro, as shown in the example below.

### Constraints

A *constraint* represents a relation between variables we want to maintain.
It contains a collection of *constraint satisfaction methods* that describe the different ways to do so.
In the example, we want the relation `a + b = c` to hold at all times.
One way to enforce it is to re-compute `a + b` and set `c` to that value.

### Methods

A *constraint satisfaction method* describes one way to enforce a constraint.
It reads the values of some variables, and write to others.

### Examples

```rust
use hotdrink_rs::{component, model::ConstraintSystem, ret, event::Event};

// Define a set of variables and relations between them
let mut component = component! {
    // Define a component `Component`.
    component Component {
        // Define variables and their default values.
        // The value can be omitted for any type that implements `Default`.
        let a: i32 = 0, b: i32, c: i32 = 3;
        // Define a constraint `Sum` that must hold between variables.
        constraint Sum {
            // Provide three ways to enforce the constraint.
            // Only one will be selected, so each one *MUST* enforce the constraint.
            abc(a: &i32, b: &i32) -> [c] = ret![a + b];
            acb(a: &i32, c: &i32) -> [b] = ret![c - a];
            bca(b: &i32, c: &i32) -> [a] = ret![c - b];
        }
    }
};

// Describe what should happen when `a` changes.
component.subscribe("a", |event| match event {
    Event::Pending => println!("A new value for `a` is being computed"),
    Event::Ready(value) => println!("New value for `a`: {:?}", value),
    Event::Error(errors) => println!("Computation for `a` failed: {:?}", errors),
});

// Edit the value of `a`
component.edit("a", 3);

// Enforce all the constraints by selecting one method from each,
// and then executing the them in topological order.
component.solve();

// Add the component to a constraint system.
// One constraint system can contain many components.
let mut cs = ConstraintSystem::new();
cs.add_component(component);

// Solve each component in the constraint system.
cs.solve();
```

More examples can be found in `./examples`, and can be run with `cargo run --example <name>`.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
