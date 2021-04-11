use crate::{
    component,
    data::Component,
    gen_js_val, ret,
    thread::pool::{dummy_pool::DummyPool, traits::TerminationStrategy},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{convert::IntoWasmAbi, JsValue};

#[derive(Serialize, Deserialize)]
struct QueryResult(Vec<String>);

impl From<QueryResult> for JsValue {
    fn from(qr: QueryResult) -> Self {
        JsValue::from_serde(&qr).unwrap()
    }
}

impl IntoWasmAbi for QueryResult {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        JsValue::from_serde(&self).unwrap().into_abi()
    }
}

gen_js_val! {
    pub SearchValueWrapper {
        #[derive(Clone)]
        pub StringOrVecString {
            String,
            QueryResult
        }
    }
}

// Generate the constraint system wrapper.
crate::gen_js_constraint_system!(
    SearchConstraintSystem,
    SearchValueWrapper,
    StringOrVecString,
    DummyPool,
    1,
    TerminationStrategy::UnusedResultAndNotDone
);

pub fn search_component() -> Component<StringOrVecString> {
    component! {
        component Search {
            let query: String = "", result: QueryResult = Vec::new();
            constraint QueryResult {
                query(query: &String) -> [result] = {
                    let result = vec!["alpha", "beta", "charlie"].iter()
                        .filter(|s| s.contains(query))
                        .map(|s| s.to_string())
                        .collect::<QueryResult>();
                    ret![result]
                };
            }
        }
    }
}
