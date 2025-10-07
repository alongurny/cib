#![cfg(feature = "derive")]

use cib::Cib;

#[derive(Clone, PartialEq, Debug, Cib)]
struct D(u32);

#[test]
fn derive_creates_impls() {
    let a = D(3);
    let b = D(4);
    // Using Cib<D> for owned and &D for borrowed
    assert_eq!(a.cib(), D(3));
    assert_eq!((&b).cib(), D(4));
}
