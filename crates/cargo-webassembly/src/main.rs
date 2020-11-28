#[macro_use]
extern crate clap;
use clap::App;
extern crate url;
use colored::*;
use std::env;
use std::fs::create_dir;
use std::path::PathBuf;

pub fn from_extension(extension: impl AsRef<str>) -> Option<tide::http::mime::Mime> {
    match extension.as_ref() {
        "html" => Some(tide::http::mime::HTML),
        "js" | "mjs" | "jsonp" => Some(tide::http::mime::JAVASCRIPT),
        "json" => Some(tide::http::mime::JSON),
        "css" => Some(tide::http::mime::CSS),
        "svg" => Some(tide::http::mime::SVG),
        "xml" => Some(tide::http::mime::XML),
        "png" => Some(tide::http::mime::PNG),
        "jpg" | "jpeg" => Some(tide::http::mime::JPEG),
        "wasm" => Some(tide::http::mime::WASM),
        "ico" => Some(tide::http::mime::ICO),
        _ => None,
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let yaml = load_yaml!("cli.yaml");
    let mut app = App::from_yaml(yaml);
    let matches = app.clone().get_matches();

    let mut dir = env::current_dir().unwrap();

    if let Some(matches) = matches.subcommand_matches("webassembly") {
        if let Some(matches) = matches.subcommand_matches("new") {
            dir = dir.join(matches.value_of("INPUT").unwrap());
            create_project_in_dir(&dir)
        } else if let Some(_) = matches.subcommand_matches("init") {
            create_project_in_dir(&dir)
        } else if let Some(_) = matches.subcommand_matches("build") {
            build_project_in_dir(&dir)
        } else if let Some(matches) = matches.subcommand_matches("run") {
            build_project_in_dir(&dir);
            let name = dir.file_name().unwrap().to_str().unwrap();
            let server_dir = dir.join("dist");
            let mut app = tide::new();
            let server_dir2 = server_dir.clone();
            app.at("/").get(move |_req: tide::Request<()>| {
                let index = server_dir.join("index.html");
                async move {
                    tide::Result::Ok(
                        tide::Response::builder(200)
                            .body(std::fs::read(index).unwrap())
                            .content_type(tide::http::mime::HTML)
                            .build(),
                    )
                }
            });
            app.at("/*").get(move |req: tide::Request<()>| {
                let server_dir3 = server_dir2.clone();
                async move {
                    let index = server_dir3.join("index.html");
                    let p = server_dir3.to_str().unwrap();
                    let p2 = req.url().path();
                    let s = format!("{}{}", p, p2).to_string();
                    let p3 = std::path::Path::new(&s);
                    if p3.exists() {
                        tide::Result::Ok(
                            tide::Response::builder(200)
                                .body(std::fs::read(p3).unwrap())
                                .content_type(
                                    from_extension(p3.extension().unwrap().to_str().unwrap())
                                        .unwrap(),
                                )
                                .build(),
                        )
                    } else {
                        tide::Result::Ok(
                            tide::Response::builder(200)
                                .body(std::fs::read(index).unwrap())
                                .content_type(tide::http::mime::HTML)
                                .build(),
                        )
                    }
                }
            });
            let port = matches.value_of("port").unwrap().parse::<u32>().unwrap();
            let addr = format!("{}{}", "http://127.0.0.1:", port);
            let addr2 = addr.clone();
            async_std::task::spawn(async move { webbrowser::open(&addr2) });
            println!(
                "   {} webassembly `{}` package on port {}",
                "Running".green().bold(),
                name,
                addr
            );
            app.listen(addr).await?;
        } else {
            if matches.is_present("version") {
                println!("{}", env!("CARGO_PKG_VERSION"))
            } else {
                app.print_long_help().unwrap();
            }
        }
    }
    Ok(())
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
    use std::io::{self, Write};
    use std::process::Command;

    if !dir.join("Cargo.toml").exists() {
        println!("must execute this command in project root");
        return;
    }

    let name = dir.file_name().unwrap().to_str().unwrap();
    println!(
        "   {} webassembly `{}` package",
        "Pre-compile check".green().bold(),
        name
    );
    let mut target_check = Command::new("cargo");
    target_check
        .arg("check")
        .arg("--target")
        .arg("wasm32-unknown-unknown");
    let command_output = target_check
        .output()
        .expect("Build pre-check failed! (check that wasm32 build target is installed)");
    io::stdout().write_all(&command_output.stdout).unwrap();
    io::stderr().write_all(&command_output.stderr).unwrap();
    println!(
        "   Pre-compile check exit code status: {}",
        command_output.status
    );

    let name = dir.file_name().unwrap().to_str().unwrap();
    println!(
        "   {} webassembly `{}` package",
        "Compiling".green().bold(),
        name
    );

    println!(
        "   {} webassembly `{}` package",
        "Compiling".green().bold(),
        name
    );
    let mut echo_hello = Command::new("cargo");
    echo_hello
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release");
    let compile_command_output = echo_hello.output().expect("Could not compile to wasm");

    io::stdout()
        .write_all(&compile_command_output.stdout)
        .unwrap();
    io::stderr()
        .write_all(&compile_command_output.stderr)
        .unwrap();
    println!(
        "   Compilation exit code status: {}",
        compile_command_output.status
    );

    std::fs::copy(
        dir.join(format!(
            "target/wasm32-unknown-unknown/release/{}.wasm",
            name.replace("-", "_")
        )),
        dir.join(format!("dist/{}.wasm", name)),
    )
    .expect("Could not copy built file! (check that wasm32 build target is installed)");
    println!("    {} webassembly target", "Finished".green().bold());
}
