use hotdrink_rs::{
    examples::{
        components::{ladder::ladder, random::make_random},
        constraint_systems::{
            linear::linear_twoway,
            linear_oneway, make_dense_cs, multioutput_singleway,
            tree::{singleoutput_multiway, singleoutput_singleway, unprunable},
        },
    },
    Component,
};
use std::io::{self, Write};

fn write_component<T: Clone>(
    name: &str,
    size: usize,
    make_component: impl Fn(usize) -> Component<T>,
) -> io::Result<()> {
    let component = make_component(size);
    let mut output_file = std::fs::File::create(&format!("dot/{}.dot", name))?;
    let dot = component.to_dot_detailed().unwrap();
    write!(output_file, "{}", dot)?;
    Ok(())
}

fn write_component_simple<T: Clone>(
    name: &str,
    size: usize,
    make_component: impl Fn(usize) -> Component<T>,
) -> io::Result<()> {
    let component = make_component(size);
    let mut output_file = std::fs::File::create(&format!("dot/{}.dot", name))?;
    let dot = component.to_dot_simple().unwrap();
    write!(output_file, "{}", dot)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let size: usize = args.nth(1).map(|s| s.parse().ok()).flatten().unwrap_or(10);

    write_component("ladder", size, |nv| ladder::<()>("ladder".to_string(), nv))?;
    write_component("linear-oneway", size, |nv| {
        let cs = linear_oneway::<()>(1, nv);
        let component = cs.get_component("0").clone();
        component
    })?;
    write_component("linear-twoway", size, |nv| {
        let cs = linear_twoway::<()>(1, nv);
        let component = cs.get_component("0").clone();
        component
    })?;
    write_component("dense", size, |nv| {
        let component = make_dense_cs::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("singleoutput-singleway", size, |nv| {
        let component = singleoutput_singleway::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("singleoutput-multiway", size, |nv| {
        let component = singleoutput_multiway::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("multioutput-singleway", size, |nv| {
        let component = multioutput_singleway::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("unprunable", size, |nv| {
        let component = unprunable::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    let max_vars_per_constraint = 5;
    write_component("new_random_low_clustering", size, |nv| {
        make_random::<()>(nv, max_vars_per_constraint, 1, 1.0)
    })?;
    write_component("new_random_medium_clustering", size, |nv| {
        make_random::<()>(nv, max_vars_per_constraint, 5, 1.5)
    })?;
    write_component("new_random_high_clustering", size, |nv| {
        make_random::<()>(nv, max_vars_per_constraint, 10, 2.0)
    })?;
    write_component_simple("new_random_low_clustering_simple", size, |nv| {
        make_random::<()>(nv, max_vars_per_constraint, 1, 1.0)
    })?;
    write_component_simple("new_random_medium_clustering_simple", size, |nv| {
        make_random::<()>(nv, max_vars_per_constraint, 5, 1.5)
    })?;
    write_component_simple("new_random_high_clustering_simple", size, |nv| {
        make_random::<()>(nv, max_vars_per_constraint, 10, 2.0)
    })?;

    Ok(())
}
