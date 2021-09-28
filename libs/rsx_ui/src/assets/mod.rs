use image::{self, DynamicImage};
use std::io;

fn open_file<'a>(path: &str) -> io::Result<Vec<u8>> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    let f = File::open(path)?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();

    // // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    // // Read.
    // // for value in buffer {
    // //     println!("BYTE: {}", value);
    // // }

    Ok(buffer)
}

pub fn load_font(filename: &str) -> conrod_core::text::Font {
    #[cfg(target_os = "android")]
    {
        use std::ffi::CString;
        use std::io::Read;
        let activity = ndk_glue::native_activity();
        let asset_manager = activity.asset_manager();

        let mut asset_res = asset_manager
            .open(&CString::new(filename).unwrap())
            .ok_or(utils::fs::Error::AndroidAssetLoadingError);

        match asset_res {
            Ok(mut asset) => {
                let mut bytes: Vec<u8> = vec![];
                asset.read_to_end(&mut bytes);
                conrod_core::text::Font::from_bytes(bytes).unwrap()
            }
            Err(err) => panic!("Can't load font {}. Error: {}", filename, err),
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        let file_path = utils::get_path(filename);
        let path = file_path.as_str();

        match open_file(path) {
            Ok(vector) => conrod_core::text::Font::from_bytes(vector).unwrap(),
            _ => panic!("Can't load font {}.", filename),
        }
    }
}

pub fn load_image(filename: &str) -> DynamicImage {
    #[cfg(target_os = "android")]
    {
        let result: Result<Vec<u8>, utils::fs::Error> = utils::fs::load_file_android_sync(filename);
        match result {
            Ok(vector) => {
                let bytes = &vector;
                image::load_from_memory(bytes).unwrap()
            }
            Err(err) => panic!("Can't load image {}. {}", filename, err),
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        let file_path = utils::get_path(filename);
        let path = file_path.as_str();
        match open_file(path) {
            Ok(vector) => {
                let bytes = &vector;
                image::load_from_memory(bytes).unwrap()
            }
            _ => panic!("Can't load font {}.", filename),
        }
    }
}
