use anyhow::Context;
use minijinja::Environment;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
};

fn main() -> anyhow::Result<()> {
    let template = if let Some(path) = env::args().nth(1) {
        fs::read_to_string(&path)
            .with_context(|| format!("Failed to read template file: {}", path))?
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };

    let mut env = Environment::new();
    env.add_template("template", &template)?;

    let tmpl = env.get_template("template")?;
    let ctx = env::vars().collect::<HashMap<_, _>>();

    let output = tmpl.render(&ctx)?;
    println!("{}", output);

    Ok(())
}
