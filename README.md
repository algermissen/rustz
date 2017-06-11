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

