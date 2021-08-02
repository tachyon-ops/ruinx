// RSX parser
use rsx_parser::parse;
use rsx_parser::types::RSXElement;

pub struct RsxScript;

impl RsxScript {
    pub fn new(source: &str) -> RSXElement {
        let result = RsxScript::retrieve_ast(source);
        match result {
            Ok(ast) => ast,
            Err(why) => {
                println!("Error: {}", why);
                parse("<Hello>World</Hello>").unwrap().0
            }
        }
    }

    pub fn retrieve_ast(content: &str) -> Result<RSXElement, &str> {
        let parse_result = parse(content);
        match parse_result {
            Ok((ast, _)) => Ok(ast),
            Err(_) => Err("Error parsing content"),
        }
    }
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
