//! An example of how rayon can be used as a MethodExecutor for parallel solving.
//!
//! This specific example uses `scope` to ensure that the spawned threads join before we exit main,
//! while a real program would likely keep the constraint system alive indefinitely.
//! This means that you can use the thread pool directly, like:
//!
//! ```rust
//! let tp = ThreadPoolBuilder::new().build().unwrap();
//! let mut component = ... ;
//! component.par_solve(&tp);
//! ```
//!
//! Both [`rayon::ThreadPool`] and [`rayon::Scope`] implement [`MethodExecutor`](hotdrink_rs::executor::MethodExecutor).
//!
//! Note that performing side effects like with `ready` is not common in methods,
//! and is just used to make the example more clear.

use hotdrink_rs::{component, model::Component, ret, util::fib::slow_fib};
use rayon::ThreadPoolBuilder;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

fn main() {
    let tp = ThreadPoolBuilder::new().build().unwrap();
    let ready = Arc::new(AtomicBool::new(false));

    tp.scope(|thread_pool| {
        // Create a component
        let ready_clone = ready.clone();
        let mut component: Component<i32> = component! {
            component Component {
                let a: i32 = 0, b: i32 = 0;
                constraint Constraint {
                    m(a: &i32) -> [b] = {
                        slow_fib(43);
                        ready_clone.store(true, Ordering::SeqCst);
                        ret![*a]
                    };
                }
            }
        };

        println!("Subscribing");
        // Subscribe
        component
            .subscribe("b", |event| println!("Event: b is {:?}", event))
            .unwrap();

        println!("Setting a to 3");
        // Edit and solve
        component.edit("a", 3).unwrap();

        println!("Solving");
        component.par_solve(thread_pool).unwrap();

        while !ready.load(Ordering::SeqCst) {
            println!("Waiting");
            thread::sleep(Duration::from_millis(1000));
        }
    });
}
