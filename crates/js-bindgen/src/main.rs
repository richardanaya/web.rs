use serde::*;
use tera::*;
use yaml_rust::{Yaml, YamlLoader};
fn main() {
    let mut tera = Tera::default();
    tera.add_raw_template("rust/module.rs", include_str!("templates/rust/module.rs"))
        .unwrap();

    let mut namespaces = vec![];

    let args = std::env::args().collect::<Vec<String>>();
    let file = &args[1];
    let text = std::fs::read_to_string(file).unwrap();
    let docs = YamlLoader::load_from_str(&text).unwrap();
    let items = docs[0].as_vec().unwrap();
    for i in items.iter() {
        let def = i.as_hash().unwrap();
        let ns = def
            .get(&Yaml::String("namespace".to_owned()))
            .unwrap()
            .as_str()
            .unwrap();
        let mut namespace = NameSpace {
            name: ns.to_owned(),
            functions: vec![],
        };
        let fns = def
            .get(&Yaml::String("functions".to_owned()))
            .unwrap()
            .as_vec()
            .unwrap();
        for f in fns.iter() {
            let func = f.as_hash().unwrap();
            let name = func
                .get(&Yaml::String("name".to_owned()))
                .unwrap()
                .as_str()
                .unwrap();
            let friendly_name = {
                if let Some(v) = func.get(&Yaml::String("friendly_name".to_owned())) {
                    Some(v.as_str().unwrap().to_owned())
                } else {
                    None
                }
            };
            let jsfunc = JSFunction {
                name: name.to_owned(),
                friendly_name,
            };
            let parameters: Vec<Yaml> = {
                if let Some(v) = func.get(&Yaml::String("parameters".to_owned())) {
                    v.as_vec().unwrap().clone()
                } else {
                    vec![]
                }
            };
            for p in parameters.iter() {
                let ps = p.as_hash().unwrap();
                let param_name = ps
                    .get(&Yaml::String("name".to_owned()))
                    .unwrap()
                    .as_str()
                    .unwrap();
                let param_type = ps
                    .get(&Yaml::String("type".to_owned()))
                    .unwrap()
                    .as_str()
                    .unwrap();
            }
            namespace.functions.push(jsfunc)
        }
        namespaces.push(namespace)
    }

    let mut context = Context::new();
    context.insert("namespaces", &namespaces);
    let r = tera.render("rust/module.rs", &context).unwrap();
    println!("{}", r);
}

#[derive(Serialize)]
struct JSFunction {
    name: String,
    friendly_name: Option<String>,
}

#[derive(Serialize)]
struct NameSpace {
    name: String,
    functions: Vec<JSFunction>,
}
