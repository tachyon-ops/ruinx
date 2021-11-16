use image::{self, DynamicImage};

// pub fn load_font(filename: &str) -> conrod_core::text::Font {
//     match utils::open_file(filename) {
//         Ok(bytes) => conrod_core::text::Font::from_bytes(bytes).unwrap(),
//         Err(err) => panic!("Can't load font {}. Error: {}", filename, err),
//     }
// }

pub fn load_image(filename: &str) -> DynamicImage {
    match utils::open_file(filename) {
        Ok(vector) => {
            let bytes = &vector;
            image::load_from_memory(bytes).unwrap()
        }
        Err(err) => panic!("Can't load image {}. {}", filename, err),
    }
}
