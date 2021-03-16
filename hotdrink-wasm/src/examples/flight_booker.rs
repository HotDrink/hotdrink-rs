//! The flight booker example from 7GUIs.

use hotdrink_rs::{
    component,
    data::constraint_system::ConstraintSystem,
    ret,
    thread::{dummy_pool::DummyPool, thread_pool::TerminationStrategy},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// The flight type.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum FlightType {
    /// A one way flight.
    OneWay = 1,
    /// A two way flight.
    TwoWay = 2,
}

impl From<FlightType> for JsValue {
    fn from(flight_type: FlightType) -> Self {
        match flight_type {
            FlightType::OneWay => JsValue::from(1),
            FlightType::TwoWay => JsValue::from(2),
        }
    }
}

// Generate the value type.
crate::gen_js_val! {
    pub FlightBookerValueWrapper {
        #[derive(Debug, Clone)]
        pub FlightBookerValue {
            FlightType,
            f64
        }
    }
}

// Generate the constraint system wrapper.
crate::gen_js_constraint_system!(
    FlightBookerConstraintSystem,
    FlightBookerValueWrapper,
    FlightBookerValue,
    DummyPool,
    1,
    TerminationStrategy::UnusedResultAndNotDone
);

/// Constructs a new [`FlightBookerConstraintSystem`].
#[wasm_bindgen]
pub fn flight_booker_cs() -> FlightBookerConstraintSystem {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component FlightBooker {
            let flight_type: FlightType = FlightType::OneWay, start_date: f64 = 1613499895672.0, return_date: f64 = 1613499895672.0;
            constraint StartBeforeReturn {
                bump_return_forwards(start_date: &f64, return_date: &f64) -> [return_date] = {
                    if start_date > return_date {
                        ret![*start_date]
                    } else {
                        ret![*return_date]
                    }
                };
                bump_start_backwards(return_date: &f64, start_date: &f64) -> [start_date] = {
                    if start_date > return_date {
                        ret![*return_date]
                    } else {
                        ret![*start_date]
                    }
                };
            }
        }
    });
    FlightBookerConstraintSystem::wrap(cs).expect("Could not create constraint system")
}
