use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use hotdrink_rs::{
    component,
    event::{Event, Ready},
    model::ConstraintSystem,
    ret,
};

#[test]
pub fn basic_constraint_system() {
    let mut cs = ConstraintSystem::<i32>::new();
    cs.add_component(component! {
        component Comp {
            let a: i32 = 0, b: i32 = 0, c: i32 = 0;
            constraint C {
                m1(a: &i32, b: &i32) -> [c] = ret![a + b];
            }
        }
    });
    assert_eq!(cs.components().len(), 1);

    let event_number = Arc::new(AtomicUsize::new(0));
    cs.subscribe("Comp", "a", move |e| match e {
        Event::Pending => {}
        Event::Ready(v) => {
            match event_number.load(Ordering::SeqCst) {
                0 => assert_eq!(v, Ready::Changed(&0)),
                1 => assert_eq!(v, Ready::Changed(&3)),
                _ => panic!("expected only two events"),
            }
            event_number.fetch_add(1, Ordering::SeqCst);
        }
        Event::Error(errors) => panic!("Got errors: {:?}", errors),
    })
    .unwrap();

    cs.edit("Comp", "a", 3).unwrap();

    let update_result = cs.solve();
    assert_eq!(update_result, Ok(()));

    cs.remove_component("Comp");

    assert_eq!(cs.components().len(), 0);
}
