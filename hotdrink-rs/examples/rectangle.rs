use hotdrink_rs::{model::Component, component, ret};

pub fn main() {
    // Create a new "component", a set of variables with constraints between them.
    let mut my_component: Component<i32> = component! {
        component MyComponent {

            // Define four variables of type i32 with initial value 0.
            let height: i32 = 0, width: i32 = 0,
                area: i32 = 0, perimeter: i32 = 0;

            // Define a constraint representing `height * width = area`.
            constraint HeightTimesWidthEqualsArea {
                // Define three ways to enforce it.
                hwa(height: &i32, width: &i32) -> [area] = ret![*height * *width];
                haw(height: &i32, area: &i32) -> [width] = ret![*area / *height];
                wah(width: &i32, area: &i32) -> [height] = ret![*area / *width];
            }

            // Define a constraint representing `2 * height + 2 * width = perimeter`.
            constraint TwoHeightPlusTwoAreaEqualsPerimeter {
                // Define three ways to enforce it.
                hwp(height: &i32, width: &i32) -> [perimeter] = ret![*height * *width];
                hpw(height: &i32, perimeter: &i32) -> [width] = ret![*perimeter / *height];
                wph(width: &i32, perimeter: &i32) -> [height] = ret![*perimeter / *width];
            }
        }
    };

    // Tell the constraint system what to do when something happens to a variable.
    my_component.subscribe("height", |e| { println!("height: {:?}", e); }).unwrap();
    my_component.subscribe("width", |e| { println!("width: {:?}", e); }).unwrap();
    my_component.subscribe("area", |e| { println!("area: {:?}", e); }).unwrap();
    my_component.subscribe("perimeter", |e| { println!("perimeter: {:?}", e); }).unwrap();

    // Set a variable's value to something new.
    my_component.set_variable("height", 3).unwrap();
    my_component.set_variable("width", 5).unwrap();

    // Update variables in the constraint system and watch it work.
    my_component.update().unwrap();
}
