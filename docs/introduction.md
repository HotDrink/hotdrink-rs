# Introduction

Before getting started with the library, it will likely help to know what constraint systems do, and if they will help you.
While the original intent is for them to be used in GUIs, there is nothing stopping you from using it in any other program.

- If you are just using Rust, then take a look at [`hotdrink-rs`](./hotdrink-rs.md), and use it as any other Rust-dependency.
  This will be easier to work with than the WebAssembly wrapper.
- If you want to use the library on the web, take a look at [`hotdrink-wasm`](hotdrink-wasm.md), which provides wrappers for defining constraint systems that can be used from JavaScript (or anything else that is able to interact with WebAssembly).

## Example

If you have a set of variables that have specific relations between them, then this library may be of use for you.
Imagine that we have the variables `a`, `b` and `c`, where some `a` and `b` have some relation `R1`, while `b` and `c` have some relation `R2`.
Whenever `a` updates, we must also update `b` to maintain the relation, or vice versa.
This quickly becomes difficult to keep track of when we also have to maintain the relations transatively.
In a simple chain like this with `n` variables, there are suddenly `n-1` other variables that must be updated every time one changes,
and this is the simplest case (other than one with no relations that must be maintained).

With this library, you define *constraints* between the variables that should be enforced.
If you additionally specify how each constraint is enforced, you can automatically enforce all relations in one go.
The following example may clarify.

```rust
use hotdrink_rs::{component, ret};

let component = component! {
    component Rectangle {
        // The variables we are interested in
        let height: i32 = 0, width: i32 = 0, area: i32 = 0, perimeter: i32 = 0;

        // Enforces area = height * width
        constraint Area {
            height_width_to_area(height: &i32, width: &i32) -> [area] {
                ret![*height * *width]
            }
            area_height_to_width(area: &i32: height: &i32) -> [width] {
                ret![*area / *height]
            }
            area_width_to_height(area: &i32: width: &i32) -> [height] {
                ret![*area / *width]
            }
        }

        // Enforces perimeter = 2* height + 2 * width
        constraint Perimeter {
            height_width_to_perimeter(height: &i32, width: &i32) -> [perimeter] {
                ret![2 * *height + 2 * *width]
            }
            perimeter_height_to_width(perimeter: &i32: height: &i32) -> [width] {
                ret![*perimeter - 2 * *height]
            }
            perimeter_width_to_height(perimeter: &i32: width: &i32) -> [height] {
                ret![*perimeter - 2 * *width]
            }
        }
    }
};

// Update the value of height
component.set_variable("height", 3);

// Automatically solve the system by calling one method per constraint.
component.update();
```

In the example above, the system could be solved by
calling `heigh_width_to_area` to update the area,
followed by `height_width_to_perimeter` to update the perimeter.
The code above would work no matter which one of the variables were updated,
and solves the system in any "direction", for instance if the area is updated with `component.set_variable("area", 40)`.

The system will also pick the method depending on which variables were updated last.
If the height was updated, then the area, then the width and perimeter would be automatically modified
in order to maintain the recently edited variables.
This is to avoid clobbering updates that were provided to the constraint system.

For this to be useful, we must also be able to retrieve the new values,
which can be done by *subscribing* to variables.

```rust
use hotdrink_rs::Event;

component.subscribe("area", event => match event {
    Event::Pending => println!("A new value for `area` is being computed"),
    Event::Ready(value) => println!("The new value for `area` is {}", value),
    Event::Error(errors) => println!("The computation failed: {:?}", errors),
});
```

Now the appropriate callback will be called when changes happen in the constraint system,
and this could for instance be used to set GUI elements to the new values.