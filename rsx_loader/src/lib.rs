// use std::env;
use std::{env, fs};

// RSX parser
use rsx_parser::parse;
use rsx_parser::types::RSXElement;

pub struct ArgsFilename;

impl ArgsFilename {
    pub fn new(args: &[String]) -> Result<String, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();

        Ok(filename)
    }
}

fn read_file(filename: &str) -> Result<String, &str> {
    println!("Parsing file {}", filename);
    let contents_result = fs::read_to_string(filename);
    match contents_result {
        Ok(contents) => {
            println!("Contents: \n{}", contents.as_str().clone());
            Ok(contents)
        }
        Err(_) => Err("Error reading file"),
    }
}

pub fn retrieve_ast(filename: &str) -> Result<RSXElement, &str> {
    let content = read_file(filename)?;
    let parse_result = parse(content.as_str());
    match parse_result {
        Ok((ast, remaining)) => {
            println!("result ast: {:?}, remaining: {}", ast, remaining);
            Ok(ast)
        }
        Err(_) => Err("Error parsing content"),
    }
}

pub fn get_ast_from_args() -> Result<RSXElement, String> {
    let args: Vec<String> = env::args().collect();
    let filename = ArgsFilename::new(&args)?;
    let ast = retrieve_ast(filename.as_str())?;
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let expectation = "Normal(RSXNormalElement(Name(RSXIdentifier(\"Hello\")), RSXAttributes([]), RSXChildren([Text(RSXText(\"World\"))])))";
        let (ast, _remaining): (RSXElement, _) = parse("<Hello>World</Hello>").unwrap();
        assert_eq!(expectation, format!("{:?}", ast));
    }
}
