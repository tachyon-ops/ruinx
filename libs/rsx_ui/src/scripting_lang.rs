extern crate dyon;

use std::sync::Arc;

use dyon::{error, run_str};

pub struct Script {}

impl Script {
    pub async fn new(entry_point: &str) {
        let file = format!("dyon/{}.dyon", entry_point);

        println!("Scripting file: {}", file);
        let source_res = macroquad::file::load_string(&file).await;
        match source_res {
            Ok(source) => {
                error(run_str(entry_point, Arc::new(source)));
            }
            _ => {
                println!("Could not run entry point {}", file);
            }
        }
    }
}
