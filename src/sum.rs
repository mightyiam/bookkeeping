use crate::metadata::Metadata;
use crate::unit::Unit;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;
/// Represents amounts of any number of units.
#[derive(Clone, PartialEq)]
pub struct Sum<T: Metadata>(pub(crate) BTreeMap<Rc<Unit<T>>, u64>);
impl<T: Metadata> Sum<T> {
    /// Creates an empty sum.
    pub fn new() -> Self {
        Self(Default::default())
    }
    /// Creates a sum with an amount of a single unit.
    pub fn of(unit: &Rc<Unit<T>>, amount: u64) -> Self {
        Self::new().unit(&unit, amount)
    }
    /// Sets the amount of a unit in a sum.
    pub fn unit(mut self, unit: &Rc<Unit<T>>, amount: u64) -> Self {
        // TODO check that new unit is in same book as existing ones.
        self.0.insert(unit.clone(), amount);
        self
    }
    // TODO method `units`
}
impl<T: Metadata> fmt::Debug for Sum<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sum(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[cfg(test)]
mod test {
    use super::BTreeMap;
    use super::Sum;
    use super::Unit;
    use crate::book::Book;
    use crate::metadata::BlankMetadata;
    #[test]
    fn new() {
        let actual = Sum::<BlankMetadata>::new();
        let expected = Sum(BTreeMap::new());
        assert_eq!(actual, expected);
    }
    #[test]
    fn of() {
        let book = Book::<BlankMetadata>::new(());
        let unit = Unit::new(&book, ());
        let sum = Sum::of(&unit, 24);
        let mut expected = BTreeMap::new();
        expected.insert(unit.clone(), 24);
        assert_eq!(sum.0, expected);
    }
    #[test]
    fn unit() {
        let book = Book::<BlankMetadata>::new(());
        let unit = Unit::new(&book, ());
        let sum = Sum::new().unit(&unit, 124);
        let mut expected = BTreeMap::new();
        expected.insert(unit.clone(), 124);
        assert_eq!(sum.0, expected);
    }
    #[test]
    fn fmt_debug() {
        let book = Book::<BlankMetadata>::new(());
        let unit_a = Unit::new(&book, ());
        let amount_a = 76;
        let unit_b = Unit::new(&book, ());
        let amount_b = 45;
        let sum = Sum::of(&unit_a, amount_a).unit(&unit_b, amount_b);
        let actual = format!("{:?}", sum);
        let expected = format!(
            "Sum({{{:?}: {:?}, {:?}: {:?}}})",
            unit_a, amount_a, unit_b, amount_b
        );
        assert_eq!(actual, expected);
    }
}
