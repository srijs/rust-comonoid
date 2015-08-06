/// A comonoid in a monoidal category is a monoid
/// in the dual category, what is the problem?
///
/// It is useful within Rust's ownership type system,
/// representing an object that can be both cloned
/// and destroyed.
pub trait Comonoid {
    /// The dual to the monoidal unit.
    /// It takes ownership of a value and returns `()`,
    /// rendering it unusable.
    fn counit(self) -> ();
    /// The dual to the monoidal multiplication.
    /// It takes ownership of a value and returns a tuple of
    /// values, effectively performing a clone operation.
    fn comult(self) -> (Self, Self);
}

impl<T> Comonoid for T where T: Clone {
    fn counit(self) -> () {}
    fn comult(self) -> (Self, Self) {
        (self.clone(), self)
    }
}

#[test]
fn test_counit() {
    let x = "hello";
    assert_eq!(Comonoid::counit(x), ());
}

#[test]
fn test_comult() {
    let y = "world";
    assert_eq!(Comonoid::comult(y), (y.clone(), y.clone()));
}
