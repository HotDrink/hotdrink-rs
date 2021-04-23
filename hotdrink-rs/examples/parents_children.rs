//! An example that always ensures that the two views of the graph are synchronized.
//! If the parent-to-children map is modified, then the child-to-parents map will change
//! to match this.

use hotdrink_rs::{component, ret, model::{Component, Activation}};
use std::collections::BTreeMap;

/// For each edge a -> b in the graph represented by the [`HashMap`], construct an edge b -> a.
fn invert<T: Clone + Ord>(hm: &BTreeMap<T, Vec<T>>) -> BTreeMap<T, Vec<T>> {
    let mut inverted: BTreeMap<T, Vec<T>> = BTreeMap::new();
    for (k, vs) in hm {
        for v in vs {
            inverted.entry(v.clone()).or_insert(Vec::new()).push(k.clone());
        }
    }
    inverted
}

type Graph = BTreeMap<&'static str, Vec<&'static str>>;

pub fn main() {
    let mut comp: Component<Graph> = component! {
        component ParentsChildren {
            let children: Graph,
                parents: Graph;

            constraint Mirror {
                one(parents: &Graph) -> [children] = ret![invert(parents)];
                two(children: &Graph) -> [parents] = ret![invert(children)];
            }
        }
    };

    comp.set_variable("children", vec![
        ("a", vec!["b", "c"]),
        ("b", vec!["c", "d"]),
        ("c", vec!["e", "f", "g"]),
    ].into_iter().collect()).unwrap();
    comp.update().unwrap();

    assert_eq!(comp.value("parents"), Ok(Activation::from(vec![
                ("d", vec!["b"]),
                ("b", vec!["a"]),
                ("e", vec!["c"]),
                ("f", vec!["c"]),
                ("g", vec!["c"]),
                ("c", vec!["a", "b"]),
    ].into_iter().collect::<Graph>())));
}
