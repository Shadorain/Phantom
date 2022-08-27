use phantom::Phantom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut phantom: Phantom = Phantom::new()?;
    phantom.run()?;
    Ok(())
}
