// foo

pub mod semigroup;
pub use semigroup::XX;

pub mod state;
pub use state::State;

pub mod reader;
pub use reader::Reader;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it2_works() {

        assert!(XX == 10);
    }
}
