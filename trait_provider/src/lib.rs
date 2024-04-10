use std::marker::PhantomData;

/// The trait we want to provide.
pub trait TestTrait<DUMMY> {}

/// A struct is needed because we can only do blanket implementations for
/// concrete types.
pub struct Wrapper<T, D>(pub T, pub PhantomData<D>);

/// A trait provided for types in external crates to implement.
pub trait TestTraitExt<DUMMY> {}

/// A blanket implementation. This allows external crates to implement
/// `TestTrait` for types not in their crate. To achieve that, they need to
/// define a dummy type `D` and implement `TestTraitExt` for `Wrapper<T, D>`.
impl<T, D> TestTrait<D> for T where Wrapper<T, D>: TestTraitExt<D> {}
