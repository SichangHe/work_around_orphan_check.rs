use trait_provider::{TestTraitExternal, Wrapper};

pub struct Dummy;

// `usize` is not defined in this crate, therefore we cannot directly implement
// `TestTrait`, also not defined in this crate, for it:
/*
impl TestTrait<Dummy> for usize {}
// => conflicting implementations of trait `trait_provider::TestTrait<Dummy>` for type `usize`
*/

// However, we can implement `TestTraitExt` for `Wrapper<usize, Dummy>`, which
// results in an implementation of `TestTrait` for `usize`:
impl TestTraitExternal<Dummy> for Wrapper<usize, Dummy> {}

#[cfg(test)]
mod tests {
    use super::*;
    use trait_provider::TestTrait;

    #[test]
    fn does_impl() {
        let usize_impls_test_trait = impls::impls!(usize: TestTrait<Dummy>);
        assert!(usize_impls_test_trait);
    }
}
