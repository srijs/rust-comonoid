/// A comonoid in a monoidal category is a monoid
/// in the dual category.
///
/// It is useful within Rust's ownership type system,
/// representing an object that can be both cloned
/// and destroyed.
pub trait Comonoid {
    fn counit(self) -> ();
    fn comult(self) -> (Self, Self);
}

impl<T> Comonoid for T where T: Clone {
    fn counit(self) -> () {}
    fn comult(self) -> (Self, Self) {
        let c = self.clone();
        (self, c)
    }
}
