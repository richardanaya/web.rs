use url_open::UrlOpen;
#[macro_use]
extern crate clap;
use clap::App;
extern crate url;
extern crate url_open;
use colored::*;
use std::env;
use std::fs::create_dir;
use std::path::PathBuf;
use url::Url;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut dir = env::current_dir().unwrap();

    if let Some(matches) = matches.subcommand_matches("new") {
        dir = dir.join(matches.value_of("INPUT").unwrap());
        create_project_in_dir(&dir)
    } else if let Some(_) = matches.subcommand_matches("init") {
        create_project_in_dir(&dir)
    } else if let Some(_) = matches.subcommand_matches("build") {
        build_project_in_dir(&dir)
    } else if let Some(_) = matches.subcommand_matches("run") {
        build_project_in_dir(&dir);
        let server_dir = dir.join("dist");
        start_server(&server_dir);
        Url::parse("https://github.com/richardanaya/js-wasm")
            .unwrap()
            .open();
    }
}

fn create_project_in_dir(dir: &PathBuf) {
    let name = dir.file_name().unwrap().to_str().unwrap();
    if !dir.exists() {
        create_dir(dir).unwrap();
    }
    create_dir(dir.join("src")).unwrap();
    create_dir(dir.join("dist")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        include_str!("template/Project.toml").replace("PROJECT", name),
    )
    .expect("Failed to write");
    std::fs::write(
        dir.join("src/lib.rs"),
        include_str!("template/lib.rs").replace("PROJECT", name),
    )
    .expect("Failed to write");
    std::fs::write(
        dir.join("dist/index.html"),
        include_str!("template/index.html").replace("PROJECT", name),
    )
    .expect("Failed to write");
    std::fs::write(
        dir.join("dist/js-wasm.js"),
        include_str!("template/js-wasm.js").replace("PROJECT", name),
    )
    .expect("Failed to write");
    println!(
        "   {} webassembly `{}` package",
        "Created".green().bold(),
        name
    );
}

fn build_project_in_dir(dir: &PathBuf) {
    if !dir.join("Cargo.toml").exists() {
        println!("must execute this command in project root");
        return;
    }

    let name = dir.file_name().unwrap().to_str().unwrap();
    println!(
        "   {} webassembly `{}` package",
        "Compiling".green().bold(),
        name
    );
    use std::process::Command;
    let mut echo_hello = Command::new("cargo");
    echo_hello
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release");
    echo_hello.output().expect("could not compile");
    std::fs::copy(dir.join(format!("target/wasm32-unknown-unknown/release/{}.wasm",name)),dir.join(format!("dist/{}.wasm",name))).expect("could not copy");
    println!(
        "    {} webassembly target",
        "Finished".green().bold()
    );
}

fn start_server(dir: &PathBuf) {
    println!("starting server in {:?}", dir)
}
