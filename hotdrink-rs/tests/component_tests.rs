use hotdrink_rs::model::Component;

#[test]
pub fn api() {
    let _: Component<i32> = Component::new_empty("MyComponent");
}
