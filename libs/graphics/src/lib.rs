mod mode;
mod run;
mod stage;
mod state;

pub use mode::AppMode;
pub use run::event_loop;
pub use stage::Stage;
pub use state::State;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
