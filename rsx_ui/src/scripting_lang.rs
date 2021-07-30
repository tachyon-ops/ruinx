extern crate dyon;

use std::sync::Arc;

use dyon::{error, run, run_str};

pub struct Script {}

impl Script {
    pub async fn new(entry_point: &str) {
        let _ = std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap());

        macroquad::file::set_pc_assets_folder("assets");

        let file = format!("dyon/{}.dyon", entry_point);

        // error(run(file));
        println!("File: {}", file);
        let source_res = macroquad::file::load_string(&file).await;
        match source_res {
            Ok(source) => {
                println!("Source code: {}", source);
                error(run_str(entry_point, Arc::new(source)));
            }
            _ => {
                println!("Could not run entry point {}", file);
            }
        }
    }
}
