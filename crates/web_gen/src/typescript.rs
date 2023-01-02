use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::{
    bytes::complete::take_while,
    character::complete::{alpha1, char},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::many0,
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Clone)]
pub enum ValueType {
    Void,
    Interface(String),
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct Comment(pub String);

#[derive(Debug, Clone)]
pub enum InterfaceMember {
    Function(Function),
    Field(Declaration),
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub members: Vec<InterfaceMember>,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub parts: Vec<TypescriptDefinitionFilePart>,
}

#[derive(Debug, Clone)]
pub enum TypescriptDefinitionFilePart {
    Comment(Comment),
    Interface(Interface),
    Declaration(Declaration),
    NameSpace(Namespace),
}

#[derive(Debug, Clone)]
pub struct TypescriptDefinitionFile {
    pub parts: Vec<TypescriptDefinitionFilePart>,
}

fn parse_declaration(input: &str) -> IResult<&str, TypescriptDefinitionFilePart> {
    let (input, _) = multispace0(input)?;
    // look for string "def"
    let (input, _) = tag("var")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value_type) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((
        input,
        TypescriptDefinitionFilePart::Declaration(Declaration {
            name: String::from(name),
            value_type: match value_type {
                "string" => ValueType::String,
                "number" => ValueType::Number,
                "boolean" => ValueType::Boolean,
                _ => ValueType::Interface(String::from(value_type)),
            },
        }),
    ))
}

fn parse_interface_member(input: &str) -> IResult<&str, InterfaceMember> {
    let (input, _) = multispace0(input)?;
    alt((parse_function, parse_field))(input)
}

fn parse_interface(input: &str) -> IResult<&str, TypescriptDefinitionFilePart> {
    let (input, _) = multispace0(input)?;
    // look for string "def"
    let (input, _) = tag("interface")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('{')(input)?;
    let (input, members) = map(opt(many0(parse_interface_member)), |nodes| {
        nodes.unwrap_or(vec![])
    })(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('}')(input)?;
    Ok((
        input,
        TypescriptDefinitionFilePart::Interface(Interface {
            name: String::from(name),
            members: members,
        }),
    ))
}

fn parse_parameter(input: &str) -> IResult<&str, Parameter> {
    let (input, _) = multispace0(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value_type) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    Ok((
        input,
        Parameter {
            name: String::from(name),
            value_type: match value_type {
                "void" => panic!("void is not a valid parameter type"),
                "string" => ValueType::String,
                "number" => ValueType::Number,
                "boolean" => ValueType::Boolean,
                _ => ValueType::Interface(String::from(value_type)),
            },
        },
    ))
}

fn parse_parameters(input: &str) -> IResult<&str, Vec<Parameter>> {
    separated_list0(char(','), parse_parameter)(input)
}

fn parse_function(input: &str) -> IResult<&str, InterfaceMember> {
    let (input, _) = multispace0(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, parameters) = map(opt(parse_parameters), |args| args.unwrap_or(vec![]))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, return_type) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((
        input,
        InterfaceMember::Function(Function {
            name: String::from(name),
            parameters: parameters,
            return_type: match return_type {
                "void" => ValueType::Void,
                "string" => ValueType::String,
                "number" => ValueType::Number,
                "boolean" => ValueType::Boolean,
                _ => ValueType::Interface(String::from(return_type)),
            },
        }),
    ))
}

fn parse_field(input: &str) -> IResult<&str, InterfaceMember> {
    let (input, _) = multispace0(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, return_type) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(';')(input)?;
    Ok((
        input,
        InterfaceMember::Field(Declaration {
            name: String::from(name),
            value_type: match return_type {
                "void" => panic!("Field cannot have void type"),
                "string" => ValueType::String,
                "number" => ValueType::Number,
                "boolean" => ValueType::Boolean,
                _ => ValueType::Interface(String::from(return_type)),
            },
        }),
    ))
}

fn parse_comment(input: &str) -> IResult<&str, TypescriptDefinitionFilePart> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('/')(input)?;
    let (input, _) = char('/')(input)?;
    let (input, comment) = take_while(|c: char| c != '\n' && c != '\r')(input)?;
    Ok((
        input,
        TypescriptDefinitionFilePart::Comment(Comment(comment.to_string())),
    ))
}

fn parse_namespace(input: &str) -> IResult<&str, TypescriptDefinitionFilePart> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("declare")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("namespace")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('{')(input)?;
    let (input, parts) = map(opt(many0(parse_part)), |nodes| nodes.unwrap_or(vec![]))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('}')(input)?;
    Ok((
        input,
        TypescriptDefinitionFilePart::NameSpace(Namespace {
            name: String::from(name),
            parts: parts,
        }),
    ))
}

fn parse_part(input: &str) -> IResult<&str, TypescriptDefinitionFilePart> {
    let (input, _) = multispace0(input)?;
    alt((
        parse_interface,
        parse_comment,
        parse_declaration,
        parse_namespace,
    ))(input)
}

fn parse_file(input: &str) -> IResult<&str, TypescriptDefinitionFile> {
    map(opt(many0(parse_part)), |nodes| TypescriptDefinitionFile {
        parts: nodes.unwrap_or(vec![]),
    })(input)
}

pub fn parse(buffer: &[u8]) -> Result<TypescriptDefinitionFile, &'static str> {
    if let Ok(buffer_str) = std::str::from_utf8(buffer) {
        let r = parse_file(buffer_str);
        match r {
            Ok((_, usd)) => {
                return Ok(usd);
            }
            Err(e) => {
                println!("{:?}", e);
                return Err("Parse error");
            }
        }
    } else {
        return Err("Invalid UTF-8");
    }
}
