pub mod app;
pub mod dom;
pub mod engine;
pub mod ui;
pub mod widgets;

pub mod rsx_lang;
pub mod scripting_lang;

pub use app::App;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
