#![feature(test)]
extern crate test;

use hotdrink_rs::{
    algorithms::simple_planner,
    examples::constraint_systems::{
        linear::{linear_oneway, linear_twoway},
        make_dense_cs, make_empty_cs, make_sparse_cs,
        tree::{multioutput_singleway, unprunable},
    },
};
use test::Bencher;

const N_COMPONENTS: usize = 1;
const N_VARIABLES: usize = 50000;

#[bench]
fn simple_planner_on_dense(b: &mut Bencher) {
    let cs = make_dense_cs::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}

#[bench]
fn simple_planner_on_empty(b: &mut Bencher) {
    let cs = make_empty_cs::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}

#[bench]
fn simple_planner_on_linear_oneway(b: &mut Bencher) {
    let cs = linear_oneway::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}

#[bench]
fn simple_planner_on_linear_twoway(b: &mut Bencher) {
    let cs = linear_twoway::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}

#[bench]
fn simple_planner_on_sparse(b: &mut Bencher) {
    let cs = make_sparse_cs::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}

#[bench]
fn simple_planner_on_tree(b: &mut Bencher) {
    let cs = multioutput_singleway::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}

#[bench]
fn simple_planner_on_unprunable(b: &mut Bencher) {
    let cs = unprunable::<()>(N_COMPONENTS, N_VARIABLES);
    let comp = cs.component("0").unwrap();
    b.iter(|| simple_planner(comp));
}
