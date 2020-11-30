use inflector::Inflector;
use serde::*;
use tera::*;

fn main() {
    let mut tera = Tera::default();
    tera.add_raw_template("rust/module.rs", include_str!("templates/rust/module.rs"))
        .unwrap();

    let args = std::env::args().collect::<Vec<String>>();
    let file = &args[1];
    let text = std::fs::read_to_string(file).unwrap();

    let mut namespaces: Vec<NameSpace> = serde_yaml::from_str(&text).unwrap();

    for n in namespaces.iter_mut() {
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
    context.insert("namespaces", &namespaces);
    let r = tera.render("rust/module.rs", &context).unwrap();
    println!("{}", r);
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
}

#[derive(Serialize, Deserialize)]
struct NameSpace {
    name: String,
    functions: Option<Vec<JSFunction>>,
}
