use inflector::Inflector;
use serde::*;
use tera::*;
use clap::{Arg, App };

fn main() {
    let matches = App::new("js-bindgen")
                          .version("0.0")
                          .author("Richard Anaya <richard.anaya@gmail.com>")
                          .about("Creates js-wasm bindings for various languages")
                          .arg(Arg::with_name("lang")
                               .short("l")
                               .long("language")
                               .help("Sets a custom config file")
                               .takes_value(true)
                               .required(true))
                            .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();
    

    let mut tera = Tera::default();
    tera.add_raw_template("rust/module.rs", include_str!("templates/rust/module.rs"))
        .unwrap();
    
    tera.add_raw_template("c/header.h", include_str!("templates/c/header.h"))
    .unwrap();

    tera.add_raw_template("assemblyscript/module.ts", include_str!("templates/assemblyscript/module.ts"))
    .unwrap();

    let file = matches.value_of("INPUT").unwrap();
    let text = std::fs::read_to_string(file).unwrap();

    let mut bindings: Vec<Binding> = serde_yaml::from_str(&text).unwrap();

    for n in bindings.iter_mut() {
        if let Some(fs) = &mut n.functions {
            for f in fs.iter_mut() {
                if f.friendly_name.is_none() {
                    f.friendly_name = Some(f.name.to_snake_case())
                }
                if let Some(ps) = &mut f.parameters {
                    for p in ps.iter_mut() {
                        if p.friendly_name.is_none() {
                            p.friendly_name = Some(p.name.to_snake_case())
                        }
                    }
                } else {
                    f.parameters = Some(vec![]);
                }
            }
        } else {
            n.functions = Some(vec![]);
        }
    }

    let mut context = Context::new();
    context.insert("bindings", &bindings);

    if let Some(l) = matches.value_of("lang") {
        let r = if l == "rust" {
            tera.render("rust/module.rs", &context).unwrap()
        } else if l == "assemblyscript" {
            tera.render("assemblyscript/module.ts", &context).unwrap()
        } else {
            tera.render("c/header.h", &context).unwrap()
        };
        println!("{}", r);
    }
}

#[derive(Serialize, Deserialize)]
struct JSParameter {
    name: String,
    friendly_name: Option<String>,
    parameter_type: String,
}

#[derive(Serialize, Deserialize)]
struct JSFunction {
    name: String,
    friendly_name: Option<String>,
    parameters: Option<Vec<JSParameter>>,
    output: Option<String>,
    code: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Binding {
    namespace: Option<String>,
    class: Option<String>,
    functions: Option<Vec<JSFunction>>,
}
