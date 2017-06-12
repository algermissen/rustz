# Rustz is a library for functional programming in Rust.

It provides as-pure-as-useful functional data structures to improve the
functional programming experience with Rust. It implements a set of 
instances of the foundational functional type classes (e.g. Functor, Monad)
for a large number of data structures.

The implementation tries to be as close to the pure functional implementations
of these data structures, but also tries to remain practical given the
specifics of the Rust programming language.

# Status

This is currently a means for myself to explore functional programming
in general and also to understand what is possible within the 
inherent limits of Rust. If the overall approach turns out to be 
possible without negating Rust's design principles and performance
goals the plan is definitely to move this to production grade
functional library.

You can track my plans and progress here.

| Data Structure   | Status                                                       |
|------------------|--------------------------------------------------------------|
| Applicative      | Early experiments                                            |
| ValidationNel    | Early experiments                                            |
| State Monad      | Initial attempt, working but needs refactoring on lifetimes. |
| Reader Monad     | Working version, needs cleanup and refactoring               |
| Writer Monad     | Planned                                                      |
| ReaderT          | Planned for Option, Tokio futures, other type classes usually needed with Web service development. |
| ...              | Please open issues for anything you'd like to see here       |

# Like-Minded Crates

- https://github.com/KitFreddura/Kinder (interesting macro based HKT approach)
- https://github.com/mcoffin/rust-effect-monad (Contains trampoline impl.)
- https://github.com/freebroccolo/monad.rs (Contains trampoline impl.)
- https://github.com/freebroccolo/free.rs (Free monad / trampoline)
- https://github.com/ludat/hado-rs (macro for do expressions / for comprehensions)
- https://github.com/TeXitoi/rust-mdo (macro for do expressions / for comprehensions)
- https://github.com/danslapman/rust-mdo-future (Future support for mdo)
- https://github.com/ptal/partial)
- https://github.com/srijs/rust-operational (Contains e.g. Kleisli)
- https://github.com/freebroccolo/pipes.rs (Pipes)
- https://github.com/m4rw3r/chomp (Parsing)
- https://github.com/freebroccolo/tailrec.rs
- https://github.com/asajeffrey/parsell (Parsing)

# Functional Programming in Rust Discussions

- http://blog.madhukaraphatak.com/functional-programming-in-rust-part-1/
- http://blog.madhukaraphatak.com/functional-programming-in-rust-part-2/

# Discussions on Impl-Trait

- https://github.com/rust-lang/rust/issues/34511

# Discussions on Higher Kinded Types

- http://smallcultfollowing.com/babysteps/blog/2016/11/02/associated-type-constructors-part-1-basic-concepts-and-introduction/
- http://smallcultfollowing.com/babysteps/blog/2016/11/03/associated-type-constructors-part-2-family-traits/
- http://smallcultfollowing.com/babysteps/blog/2016/11/04/associated-type-constructors-part-3-what-higher-kinded-types-might-look-like/
- http://smallcultfollowing.com/babysteps/blog/2016/11/09/associated-type-constructors-part-4-unifying-atc-and-hkt/
- https://m4rw3r.github.io/rust-and-monad-trait
- https://www.reddit.com/r/rust/comments/57267j/traits_vs_higher_kinded_types/
- https://users.rust-lang.org/t/does-rust-really-need-higher-kinded-types/5531/3
- http://typelevel.org/blog/2016/08/21/hkts-moving-forward.html
- https://internals.rust-lang.org/t/higher-kinded-types-the-difference-between-giving-up-and-moving-forward/3908



