use hotdrink_rs::examples::components::{
    ComponentFactory, Ladder, LinearOneway, LinearTwoway, Random,
};
use std::io::{self, Write};

fn write_component<CF: ComponentFactory>(name: &str, size: usize) -> io::Result<()> {
    let component = CF::build::<()>(size);
    let mut output_file = std::fs::File::create(&format!("dots/{}.dot", name))?;
    let dot = component.to_dot_detailed().unwrap();
    write!(output_file, "{}", dot)?;
    Ok(())
}

fn write_component_simple<CF: ComponentFactory>(name: &str, size: usize) -> io::Result<()> {
    let component = CF::build::<()>(size);
    let mut output_file = std::fs::File::create(&format!("dots/{}.dot", name))?;
    let dot = component.to_dot_simple().unwrap();
    write!(output_file, "{}", dot)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let size: usize = args.nth(1).map(|s| s.parse().ok()).flatten().unwrap_or(10);

    write_component::<Ladder>("ladder", size)?;
    write_component::<LinearOneway>("linear-oneway", size)?;
    write_component::<LinearTwoway>("linear-twoway", size)?;
    write_component::<Ladder>("ladder", size)?;
    write_component::<Random>("random", size)?;
    write_component_simple::<Random>("random_simple", size)?;

    Ok(())
}
