pub mod semigroup;
pub use semigroup::Semigroup;

pub mod validation;
pub use validation::Validation;

//pub mod state;
//pub use state::State;
pub mod stat;
pub use stat::State;

pub mod reader;
pub use reader::Reader;

pub mod lense;
pub use lense::Lense;
pub use lense::L;
pub use lense::Compose;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it2_works() {

        assert!(10 == 10);
    }
}
