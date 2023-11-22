use {{crate_name}}::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, input) = parse(&s).n2a()?;

    println!("{:?}", input);

    Ok(())
}
