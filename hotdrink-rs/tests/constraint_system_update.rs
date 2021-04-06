use hotdrink_rs::{component, data::constraint_system::ConstraintSystem, event::Event, ret};

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

    cs.subscribe("Comp", "a", |e| match e {
        Event::Pending => {}
        Event::Ready(v) => assert_eq!(*v, 0),
        Event::Error(errors) => panic!("Got errors: {:?}", errors),
    });

    let update_result = cs.update();
    assert_eq!(update_result, Ok(()));
}
