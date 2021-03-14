use hotdrink_rs::{
    component,
    data::constraint_system::ConstraintSystem,
    ret,
    thread::{dummy_pool::DummyPool, thread_pool::TerminationStrategy},
};
use wasm_bindgen::prelude::wasm_bindgen;

// Generate the value type.
crate::gen_js_val! {
    pub TemperatureConverterValueWrapper {
        #[derive(Debug, Clone)]
        pub TemperatureConverterValue {
            f64
        }
    }
}

// Generate the constraint system wrapper.
crate::gen_js_constraint_system!(
    TemperatureConverterConstraintSystem,
    TemperatureConverterValueWrapper,
    TemperatureConverterValue,
    DummyPool,
    1,
    TerminationStrategy::UnusedResultAndNotDone
);

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
