
pub trait Semigroup {
    fn mappend(&self, b: Self) -> Self;
}

