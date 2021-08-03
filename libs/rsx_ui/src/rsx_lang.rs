use rsx_parser::types::RSXElement;

pub struct RSXLang;

impl RSXLang {
    pub async fn new(entry_point: &str) -> Option<RSXElement> {
        let file = format!("rsx/{}.rsx", entry_point);
        println!("RSX file: {}", file);

        let source_res = macroquad::file::load_string(&file).await;

        let mut ast = None;
        match source_res {
            Ok(content) => {
                ast = Some(rsx_loader::RsxScript::new(&content));
            }
            Err(why) => println!("Error: {}", why),
        }
        ast
    }
}
