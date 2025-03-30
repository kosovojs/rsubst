use anyhow::{Context, Result};
use minijinja::Environment;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        let name = args.next().unwrap();
        eprintln!("Usage: {} <template_file>", name);
        std::process::exit(1);
    }

    let template_path = PathBuf::from(args.nth(1).unwrap());
    let template = fs::read_to_string(&template_path)
        .with_context(|| format!("Failed to read template file: {}", template_path.display()))?;

    let mut env = Environment::new();
    env.add_template("template", &template)?;

    let tmpl = env.get_template("template")?;
    let ctx = env::vars().collect::<std::collections::HashMap<_, _>>();

    let output = tmpl.render(&ctx)?;
    println!("{}", output);

    Ok(())
}
