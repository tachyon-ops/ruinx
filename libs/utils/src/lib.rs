mod exec;
mod file;
pub mod fs;

pub use file::FileError;

pub struct AssetsContext {
    pc_assets_folder: Option<String>,
}

impl AssetsContext {
    fn new() -> AssetsContext {
        AssetsContext {
            pc_assets_folder: None,
        }
    }
}

// Static
#[no_mangle]
static mut ASSETS_CONTEXT: Option<AssetsContext> = None;

// Ctx will hold crazy contexts (might think of moving this out)

// Context is private

fn get_assets_context() -> &'static mut AssetsContext {
    unsafe {
        // ASSETS_CONTEXT.as_mut().unwrap_or_else(|| panic!());
        let context = ASSETS_CONTEXT.as_mut();
        match context {
            Some(ctx) => ctx,
            None => {
                ASSETS_CONTEXT = Some(AssetsContext::new());
                ASSETS_CONTEXT.as_mut().unwrap_or_else(|| panic!())
            }
        }
    }
}

pub async fn load_file(path: &str) -> Result<Vec<u8>, file::FileError> {
    file::load_file(path).await
}

pub async fn load_string(path: &str) -> Result<String, file::FileError> {
    file::load_string(path).await
}

pub fn set_pc_assets_folder(path: &str) {
    file::set_pc_assets_folder(path);
}

pub fn get_path(path: &str) -> String {
    file::get_path(path)
}
