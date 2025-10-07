//! cib: Copyable-in-Byte (example name)
//!
//! This crate provides the `Cib` trait for types that can be viewed as an
//! integer value and support certain operations for both owned and reference
//! cases.
//!
//! There are two representation cases:
//! - owned value: T (the type itself implements `Cib` for owned)
//! - reference value: &T (the reference also implements `Cib`)
//!
//! Due to coherence and orphan rules in Rust, a derive macro is provided in the
//! companion crate `cib-derive` to implement the owned case automatically. The
//! reference case is implemented generically here in this crate.
//!
//! The derive macro is necessary for the owned case because attempting to
//! implement the trait for all `T: SomeBound` would conflict with implementing
//! it for `&T` in the general case.
//!
//! See the doctest in the root of this crate for an example.#![cfg(feature = "derive")]

#[cfg(feature = "derive")]
pub use cib_derive::Cib;

/// Trait for types that can be both moved from owned and cloned from
/// references.
/// - If owned, consume it (move).
/// - If borrowed, clone it.
///
/// This is useful for functions that want to accept either owned or
///  referenced values and work with them uniformly,
///  without needing redundant clones or moves.
///
/// Example wrapper type wrapping an unsigned integer and deriving `Cib`.
///
/// The derive macro is provided by the `cib-derive` crate and implements
/// `Cib` for the owned type. The reference case is implemented in this
/// crate and so the doctest shows owned and reference usage.
///
/// ```
/// use cib::Cib;
///
/// #[derive(Clone, PartialEq, Debug)]
/// struct Small(u8);
///
/// // You could derive `Cib` instead if the `derive` feature is enabled.
/// impl Cib<Self> for Small {
///     fn cib(self) -> Self {
///         self
///     }
/// }
///
/// fn sum_all(a: impl Cib<Small>, b: impl Cib<Small>) -> Small {
///     Small(a.cib().0 + b.cib().0)
/// }
///
/// let a = || Small(7);
/// let b = || Small(3);
/// assert_eq!(sum_all(a(), b()), Small(10));
/// assert_eq!(sum_all(&a(), &b()), Small(10));
/// assert_eq!(sum_all(a(), &b()), Small(10));
/// assert_eq!(sum_all(&a(), b()), Small(10));
/// ```
pub trait Cib<T> {
    /// Converts `self` into an owned value:
    /// - moves if owned
    /// - clones if borrowed
    fn cib(self) -> T;
}

impl<T> Cib<T> for &T
where
    T: Clone,
{
    fn cib(self) -> T {
        self.clone()
    }
}

impl Cib<Self> for String {
    fn cib(self) -> Self {
        self
    }
}

impl Cib<String> for &str {
    fn cib(self) -> String {
        self.to_string()
    }
}
