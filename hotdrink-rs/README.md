# hotdrink-rs

HotDrink implemented in Rust.

HotDrink lets you declaratively describe relations between values and how to enforce them,
and can then automatically do so when the value of a variable changes.

## Introduction

Before getting started, here is a quick introduction to the terminology and how it works.

### Components

A component is a collection of variables and constraints between them that should be enforced.

### Constraints

A constraint represents a relation between variables we want to maintain.
It contains a collection of *constraint satisfaction methods* that describe the different ways to do so.
In the example, we want the relation `a + b = c` to hold at all times.
One way to enforce it is to re-compute `a + b` and set `c` to that value.

### Methods

A *constraint satisfaction method* describes one way to enforce a constraint.
It reads the values of some variables, and write to others.

## Examples

```rust
use hotdrink_rs::{component, ret, ConstraintSystem};

// Define a set of variables and relations between them
let component = component! {
    // Define the component
    component Component {
        // Define variables and their default values
        let a: i32 = 0, b: i32 = 0, c: i32 = 0;
        // Define a relation that must hold between variables.
        constraint Sum {
            // Provide three ways to enforce the constraint.
            // Only one of them will be selected, and which one
            // depends on which variable was edited last.
            abc(a: &i32, b: &i32) -> [c] = ret![*a + *b];
            acb(a: &i32, c: &i32) -> [b] = ret![*c - *a];
            bca(b: &i32, c: &i32) -> [a] = ret![*c - *b];
        }
    }
};

// Add the component to a constraint system.
// One constraint system can contain many components.
let mut cs = ConstraintSystem::new();
cs.add_component(component);

// Enforce all the constraints by selecting a method for each one,
// and then executing the methods in topological order.
cs.update();
```

## Building

The project uses multiple nightly features, and must be built using nightly Rust.
I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/).

If an appropriate version of Rust is installed, it should be as simple as running the following:

```bash
cargo build --release
```

License: MIT
