//! A constraint system for linking a temperature with different scales.

use hotdrink_rs::{
    component,
    model::ConstraintSystem,
    ret,
    thread::{DummyPool, TerminationStrategy},
};
use wasm_bindgen::prelude::wasm_bindgen;

// Generate the value type.
crate::component_type_wrapper! {
    pub struct TemperatureConverterValueWrapper {
        #[derive(Debug, Clone)]
        pub enum TemperatureConverterValue {
            f64
        }
    }
}

// Generate the constraint system wrapper.
crate::constraint_system_wrapper!(
    TemperatureConverterConstraintSystem,
    TemperatureConverterValueWrapper,
    TemperatureConverterValue,
    DummyPool,
    1,
    TerminationStrategy::UnusedResultAndNotDone
);

/// Constructs a [`TemperatureConverterConstraintSystem`].
#[wasm_bindgen]
pub fn temperature_converter_cs() -> TemperatureConverterConstraintSystem {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component TemperatureConverter {
            let kelvin: f64 = 0, celsius: f64 = 0, fahrenheit: f64 = 0;
            constraint KelvinToCelsius {
                kelvin_to_celsius(kelvin: &f64) -> [celsius] = ret![*kelvin - 273.15];
                celsius_to_kelvin(celsius: &f64) -> [kelvin] = ret![*celsius + 273.15];
            }
            constraint CelsiusToFahrenheit {
                celsius_to_fahrenheit(celsius: &f64) -> [fahrenheit] = ret![*celsius * ( 9.0 / 5.0) + 32.0];
                fahrenheit_to_celsius(fahrenheit: &f64) -> [celsius] = ret![(*fahrenheit - 32.0) * (5.0 / 9.0)];
            }
        }
    });
    TemperatureConverterConstraintSystem::wrap(cs).expect("Could not create constraint system")
}
