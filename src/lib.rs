// foo

pub mod semigroup;
pub use semigroup::Semigroup;

pub mod validation;
pub use validation::Validation;

pub mod state;
pub use state::State;

pub mod reader;
pub use reader::Reader;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it2_works() {

        assert!(10 == 10);
    }
}
