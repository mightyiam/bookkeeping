/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<T> {
    pub(crate) meta: T,
}
impl<T> Unit<T> {
    /// Creates a new unit.
    pub(crate) fn new(meta: T) -> Self {
        Self { meta }
    }
}
