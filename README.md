# Working around Rust's orphan rule for libraries

> For library users, use the NewType pattern instead.

This example shows how libraries can work around Rust's orphan rule while providing a trait, by

1. including a dummy type parameter,
1. introducing a generic wrapper type,
1. providing another "external" trait to be implemented by external crates, and
1. using a blanket implementation.

See the code at `./src/trait_provider/src/lib.rs` for the implementation corresponding to the library.

Once the library does this workaround, library users can implement the trait for external types by implementing the "external" trait for the wrapper type.

Caveat: the trait has to have a parameter type defined in the user's crate, therefore cascading trait implementations (think of `serde-derive`) are not possible without some indirections.
