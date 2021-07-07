//! A wind turbine example.

use super::image_scaling::{Number, NumberJsCs};
use hotdrink_rs::{
    component,
    model::{Component, ConstraintSystem},
    ret,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Generate a component that describes the constraints
/// required between properties of the image.
fn turbines_component() -> Component<Number> {
    component! {
        component Turbines {
            let blade_length: f64 = 1, air_density: f64 = 1, efficiency: f64 = 0.1, wind_speed: f64 = 0,
                wind_power: f64 = 0, power_output: f64 = 0;

            // Power (W) = 0.5 × Swept Area (m2) × Air Density (kg/m3) × Velocity^3 (m/s)
            constraint WindPower {
                a(blade_length: &f64, air_density: &f64, wind_speed: &f64) -> [wind_power] =
                    ret![0.5 * blade_length.powf(2.0) * air_density * wind_speed.powf(3.0)];
            }

            constraint PowerOutput {
                a(wind_power: &f64, efficiency: &f64) -> [power_output] = ret![wind_power * efficiency];
            }
        }
    }
}

/// Adds the component to a [`ConstraintSystem`],
/// then wraps that in the [`NumberJsCs`].
#[wasm_bindgen]
pub fn turbines() -> Result<NumberJsCs, JsValue> {
    let mut cs = ConstraintSystem::new();
    cs.add_component(turbines_component());
    NumberJsCs::wrap(cs)
}
