/// A comonoid in a monoidal category is a monoid
/// in the dual category, what is the problem?
///
/// Being the dual to a monoid, it consists of:
///
/// - The dual to unit, a function from `T` to `()` called `counit`.
/// - The dual to multiplication, a function from `T` to `(T, T)` called `comult`.
///
/// It is useful within Rust's ownership type system,
/// to represent a type that can be both cloned and destroyed.
///
/// There is a trivial implementation of this trait for every
/// type that implements `Clone`, as reflected by the `Comonoidal`
/// newtype wrapper, with `discard` as `counit`, and `duplicate` as `comult`.
/// The behaviour of `counit` and `comult` can be altered by the
/// implementation of [`Drop`](http://doc.rust-lang.org/std/ops/trait.Drop.html)
/// and [`Clone`](http://doc.rust-lang.org/std/clone/trait.Clone.html),
/// respectively.
pub trait Comonoid: Sized {
    /// The dual to the monoidal unit.
    fn counit(self) -> ();
    /// The dual to the monoidal multiplication.
    fn comult(self) -> (Self, Self);
}

/// Takes ownership of a value and returns `()`,
/// rendering it unusable.
pub fn discard<T>(_: T) -> () {}
/// Takes ownership of a value and returns a tuple of
/// clones.
pub fn duplicate<T>(a: T) -> (T, T) where T: Clone {
(a.clone(), a)
}

/// A newtype wrapper to make a comonoid out of any
/// cloneable type.
pub struct Comonoidal<T>(T);

impl<T> From<T> for Comonoidal<T> {
    fn from(a: T) -> Comonoidal<T> { Comonoidal(a) }
}

impl<T> AsRef<T> for Comonoidal<T> {
    fn as_ref(&self) -> &T { &self.0 }
}

impl<T> AsMut<T> for Comonoidal<T> {
    fn as_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T> Clone for Comonoidal<T> where T: Clone {
    fn clone(&self) -> Self {
        let a = self.0.clone();
        Comonoidal(a)
    }
}

impl<T> Comonoid for Comonoidal<T> where T: Clone {
    fn counit(self) -> () { discard(self) }
    fn comult(self) -> (Self, Self) { duplicate(self) }
}

#[test]
fn test_counit() {
    let x = Comonoidal(42);
    assert_eq!(x.counit(), ());
}

#[test]
fn test_comult() {
    let x = Comonoidal(42);
    let (y, z) = x.comult();
    assert_eq!(y.0, 42);
    assert_eq!(z.0, 42);
}
