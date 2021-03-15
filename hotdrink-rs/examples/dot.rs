use hotdrink_rs::{
    examples::{
        components::ladder::ladder,
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
    make_component: impl Fn(usize) -> Component<T>,
) -> io::Result<()> {
    let component = make_component(9);
    let mut output_file = std::fs::File::create(&format!("dot/{}.dot", name))?;
    let dot = component.to_dot_detailed().unwrap();
    write!(output_file, "{}", dot)?;
    Ok(())
}

fn main() -> io::Result<()> {
    write_component("ladder", |nv| ladder::<()>("ladder".to_string(), nv))?;
    write_component("linear-oneway", |nv| {
        let cs = linear_oneway::<()>(1, nv);
        let component = cs.get_component("0").clone();
        component
    })?;
    write_component("linear-twoway", |nv| {
        let cs = linear_twoway::<()>(1, nv);
        let component = cs.get_component("0").clone();
        component
    })?;
    write_component("dense", |nv| {
        let component = make_dense_cs::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("singleoutput-singleway", |nv| {
        let component = singleoutput_singleway::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("singleoutput-multiway", |nv| {
        let component = singleoutput_multiway::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("multioutput-singleway", |nv| {
        let component = multioutput_singleway::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;
    write_component("unprunable", |nv| {
        let component = unprunable::<()>(1, nv);
        let component = component.get_component("0").clone();
        component
    })?;

    Ok(())
}
