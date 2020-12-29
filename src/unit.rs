/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
pub trait Unit: Ord + Clone {}

#[cfg(test)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct TestUnit(pub(crate) &'static str);
#[cfg(test)]
impl Unit for TestUnit {}
