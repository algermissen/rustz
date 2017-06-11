
trait Semigroup {
    fn mappend(&self, b: Self) -> Self;
}

pub const XX: i32 = 10;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }

}
