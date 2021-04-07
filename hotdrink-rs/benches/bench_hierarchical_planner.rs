#![feature(test)]
extern crate test;

use hotdrink_rs::{
    algorithms::hierarchical_planner,
    examples::constraint_systems::{
        linear::linear_twoway,
        linear_oneway, make_dense_cs, make_empty_cs, make_sparse_cs,
        tree::{
            multioutput_singleway, multioutput_threeway, multioutput_twoway, singleoutput_multiway,
            singleoutput_singleway,
        },
    },
    model::ComponentSpec,
};
use test::Bencher;

const N_COMPONENTS: usize = 1;
const N_VARIABLES: usize = 25000;

macro_rules! bench_hierarchical_planner {
    ( $( $name:ident: $make_cs:ident ),* ) => {
        $(
            #[bench]
            fn $name(b: &mut Bencher) {
                let cs = $make_cs::<()>(N_COMPONENTS, N_VARIABLES);
                let comp = cs.component("0").unwrap();
                let ranking: Vec<usize> = (0..comp.n_variables()).collect();
                b.iter(|| hierarchical_planner(comp, &ranking));
            }
        )*
    };
}

bench_hierarchical_planner! {
    hierarchical_planner_on_dense: make_dense_cs,
    hierarchical_planner_on_empty: make_empty_cs,
    hierarchical_planner_on_linear_oneway: linear_oneway,
    hierarchical_planner_on_linear_twoway: linear_twoway,
    hierarchical_planner_on_sparse: make_sparse_cs,
    hierarchical_planner_on_singleoutput_singleway: singleoutput_singleway,
    hierarchical_planner_on_singleoutput_multiway: singleoutput_multiway,
    hierarchical_planner_on_multioutput_singleway: multioutput_singleway,
    hierarchical_planner_on_multioutput_multiway: multioutput_twoway,
    hierarchical_planner_on_multioutput_threeway: multioutput_threeway
}
