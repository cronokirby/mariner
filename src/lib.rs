/// A minimal trait for the functionality a field needs to implement.
///
/// This trait cannot enforce the laws a field should satisfy, related to commutativity etc.
trait Field: Eq {
    /// The additive identity element.
    const ZERO: Self;
    /// The multiplicative identity element.
    const ONE: Self;

    /// Add two field elements, producing a third.
    fn add(&self, other: &Self) -> Self;

    /// Negate a field element.
    fn negate(&self) -> Self;

    /// Multiply two field elements.
    fn mul(&self, other: &Self) -> Self;

    /// Invert a field element.
    fn invert(&self, other: &Self) -> Self;
}
