use crate::{component, data::Component, ret};

pub fn priority_adjust() -> Component<String> {
    component! {
        component PriorityAdjust {
            let _a: String = "a", _b: String = "b", _c: String = "c", _d: String = "";
            constraint Ab {
                m1(_a: &String) -> [_b] = ret!["a"];
                m2(_b: &String) -> [_a] = ret!["b"];
            }
            constraint Bcd {
                m3(_b: &String, _c: &String) -> [_d] = ret!["b & c"];
                m4(_d: &String) -> [_b, _c] = ret!["d"];
            }
        }
    }
}
