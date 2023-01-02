use crate::typescript::*;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

fn pascal_case_to_snake_case(input: &str) -> String {
    let mut result = String::new();
    for (index, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if index > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn generate_bindings(definition_file: &TypescriptDefinitionFile, output_base_dir: PathBuf) {
    println!("Generating bindings for: {:?}", definition_file);
    let mut all_known_interfaces = HashMap::new();
    for parts in definition_file.parts.iter() {
        match parts {
            TypescriptDefinitionFilePart::Interface(interface) => {
                all_known_interfaces.insert(interface.name.clone(), interface.clone());
            }
            TypescriptDefinitionFilePart::NameSpace(namespace) => {
                for parts in namespace.parts.iter() {
                    match parts {
                        TypescriptDefinitionFilePart::Interface(interface) => {
                            all_known_interfaces.insert(interface.name.clone(), interface.clone());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    for parts in definition_file.parts.iter() {
        match parts {
            TypescriptDefinitionFilePart::NameSpace(namespace) => {
                let file_name: PathBuf = output_base_dir
                    .join(format!("{}.rs", pascal_case_to_snake_case(&namespace.name)));
                let mut file = std::fs::File::create(file_name).unwrap();
                let mut writer = std::io::BufWriter::new(&mut file);
                writer.write_all(b"use js::*;\r\n").unwrap();
                writer.write_all(b"\r\n").unwrap();

                for parts in namespace.parts.iter() {
                    match parts {
                        TypescriptDefinitionFilePart::Declaration(declaration) => {
                            let declaration_name = &declaration.name;
                            if let ValueType::Interface(interface_name) = &declaration.value_type {
                                let interface =
                                    all_known_interfaces.get(&interface_name.clone()).unwrap();
                                for interface_member in interface.members.iter() {
                                    match interface_member {
                                        InterfaceMember::Field(field) => {
                                            writer.write_all(b"pub fn ").unwrap();
                                            writer.write_all(field.name.as_bytes()).unwrap();
                                            writer.write_all(b"(").unwrap();
                                            writer.write_all(b"this: &").unwrap();
                                            writer.write_all(interface.name.as_bytes()).unwrap();
                                            writer.write_all(b") -> ").unwrap();
                                            match &field.value_type {
                                                ValueType::Boolean => {
                                                    writer.write_all(b"bool").unwrap();
                                                }
                                                ValueType::Number => {
                                                    writer.write_all(b"f64").unwrap();
                                                }
                                                ValueType::String => {
                                                    writer.write_all(b"&str").unwrap();
                                                }
                                                ValueType::Interface(interface_name) => {
                                                    writer.write_all(b"&").unwrap();
                                                    writer
                                                        .write_all(interface_name.as_bytes())
                                                        .unwrap();
                                                }
                                                /*ValueType::Array(array) => {
                                                    writer.write_all(b"&[");
                                                    match &array.value_type {
                                                        ValueType::Boolean => {
                                                            writer.write_all(b"bool").unwrap();
                                                        }
                                                        ValueType::Number => {
                                                            writer.write_all(b"f64").unwrap();
                                                        }
                                                        ValueType::String => {
                                                            writer.write_all(b"&str").unwrap();
                                                        }
                                                        ValueType::Interface(interface_name) => {
                                                            writer.write_all(b"&").unwrap();
                                                            writer
                                                                .write_all(interface_name.as_bytes())
                                                                .unwrap();
                                                        }
                                                        _ => {}
                                                    }
                                                    writer.write_all(b"]").unwrap();
                                                }*/
                                                _ => {}
                                            }
                                            writer.write_all(b" {\r\n").unwrap();
                                            writer.write_all(b"    js_unwrap!(this.").unwrap();
                                            writer.write_all(field.name.as_bytes()).unwrap();
                                            writer.write_all(b")\r\n").unwrap();
                                            writer.write_all(b"}\r\n").unwrap();
                                        }
                                        InterfaceMember::Function(function) => {
                                            writer.write_all(b"pub fn ").unwrap();
                                            writer.write_all(declaration_name.as_bytes()).unwrap();
                                            writer.write_all(b"_").unwrap();
                                            writer.write_all(function.name.as_bytes()).unwrap();
                                            writer.write_all(b"(").unwrap();
                                            for (index, parameter) in
                                                function.parameters.iter().enumerate()
                                            {
                                                if index > 0 {
                                                    writer.write_all(b", ").unwrap();
                                                }
                                                writer
                                                    .write_all(parameter.name.as_bytes())
                                                    .unwrap();
                                                writer.write_all(b": ").unwrap();
                                                match &parameter.value_type {
                                                    ValueType::Boolean => {
                                                        writer.write_all(b"bool").unwrap();
                                                    }
                                                    ValueType::Number => {
                                                        writer.write_all(b"f64").unwrap();
                                                    }
                                                    ValueType::String => {
                                                        writer.write_all(b"&str").unwrap();
                                                    }
                                                    ValueType::Interface(interface_name) => {
                                                        writer.write_all(b"&").unwrap();
                                                        writer
                                                            .write_all(interface_name.as_bytes())
                                                            .unwrap();
                                                    }
                                                    /*ValueType::Array(array) => {
                                                        writer.write_all(b"&[");
                                                        match &array.value_type {
                                                            ValueType::Boolean => {
                                                                writer.write_all(b"bool").unwrap();
                                                            }
                                                            ValueType::Number => {
                                                                writer.write_all(b"f64").unwrap();
                                                            }
                                                            ValueType::String => {
                                                                writer.write_all(b"&str").unwrap();
                                                            }
                                                            ValueType::Interface(interface_name) => {
                                                                writer.write_all(b"&").unwrap();
                                                                writer.write_all(interface_name.as_bytes()).unwrap();
                                                            }
                                                            _ => {}
                                                        }
                                                        writer.write_all(b"]").unwrap();
                                                    }*/
                                                    _ => {}
                                                }
                                            }
                                            writer.write_all(b") -> ").unwrap();
                                            match &function.return_type {
                                                ValueType::Boolean => {
                                                    writer.write_all(b"bool").unwrap();
                                                }
                                                ValueType::Number => {
                                                    writer.write_all(b"f64").unwrap();
                                                }
                                                ValueType::String => {
                                                    writer.write_all(b"String").unwrap();
                                                }
                                                ValueType::Interface(interface_name) => {
                                                    writer.write_all(b"&").unwrap();
                                                    writer
                                                        .write_all(interface_name.as_bytes())
                                                        .unwrap();
                                                }
                                                /*ValueType::Array(array) => {
                                                    writer.write_all(b"&[");
                                                    match &array.value_type {
                                                        ValueType::Boolean => {
                                                            writer.write_all(b"bool").unwrap();
                                                        }
                                                        ValueType::Number => {
                                                            writer.write_all(b"f64").unwrap();
                                                        }
                                                        ValueType::String => {
                                                            writer.write_all(b"&str").unwrap();
                                                        }
                                                        ValueType::Interface(interface_name) => {
                                                            writer.write_all(b"&").unwrap();
                                                            writer.write_all(interface_name.as_bytes()).unwrap();
                                                        }
                                                        _ => {}
                                                    }
                                                    writer.write_all(b"]").unwrap();
                                                }   */
                                                _ => {
                                                    writer.write_all(b"()").unwrap();
                                                }
                                            }
                                            writer.write_all(b" {\r\n").unwrap();
                                            writer.write_all(b"    unimplemented!()\r\n").unwrap();
                                            writer.write_all(b"}\r\n").unwrap();
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                writer.flush().unwrap();
            }
            _ => {}
        }
    }
}
