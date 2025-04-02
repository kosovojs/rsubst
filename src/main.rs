use minijinja::Environment;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
};

fn main() {
    let template = if let Some(path) = env::args().nth(1) {
        fs::read_to_string(&path).expect("Failed to read template file")
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read template from stdin");
        input
    };

    let mut env = Environment::new();
    env.add_template("template", &template)
        .expect("Failed to add template");

    let tmpl = env
        .get_template("template")
        .expect("Failed to get template");
    let ctx = env::vars().collect::<HashMap<_, _>>();

    let output = tmpl.render(&ctx).expect("Failed to render template");
    println!("{}", output);
}
