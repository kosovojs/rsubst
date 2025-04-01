use anyhow::Context;
use minijinja::Environment;
use std::{collections::HashMap, env, fs, process};

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        let name = args.next().unwrap();
        eprintln!("Usage: {} <template_file>", name);
        process::exit(1);
    }

    let path = args.nth(1).unwrap();
    let template = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read template file: {}", path))?;

    let mut env = Environment::new();
    env.add_template("template", &template)?;

    let tmpl = env.get_template("template")?;
    let ctx = env::vars().collect::<HashMap<_, _>>();

    let output = tmpl.render(&ctx)?;
    println!("{}", output);

    Ok(())
}
