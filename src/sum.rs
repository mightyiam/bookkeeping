use std::collections::BTreeMap;
use std::fmt;
/// Represents amounts of any number of units.
#[derive(Clone, PartialEq)]
pub struct Sum<KU: Ord>(pub(crate) BTreeMap<KU, u64>);
impl<KU: Ord + Clone> Sum<KU> {
    /// Creates an empty sum.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Creates a sum with an amount of a single unit.
    pub fn of(unit: KU, amount: u64) -> Self {
        Self::new().unit(unit, amount)
    }
    /// Sets the amount of a unit in a sum.
    pub fn unit(mut self, unit: KU, amount: u64) -> Self {
        // TODO check that new unit is in same book as existing ones.
        self.0.insert(unit.clone(), amount);
        self
    }
    // TODO method `units`
}
impl<KU: Ord + Clone + fmt::Debug> fmt::Debug for Sum<KU> {
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
    use slotmap::{new_key_type, DenseSlotMap};
    #[test]
    fn new() {
        new_key_type! { struct KU; }
        let actual = Sum::<KU>::new();
        let expected = Sum(BTreeMap::new());
        assert_eq!(actual, expected);
    }
    #[test]
    fn of() {
        let mut slot_map = DenseSlotMap::new();
        let unit = slot_map.insert(());
        let actual = Sum::of(unit, 24);
        let mut expected = BTreeMap::new();
        expected.insert(unit, 24);
        assert_eq!(actual.0, expected);
    }
    #[test]
    fn unit() {
        let mut slot_map = DenseSlotMap::new();
        let unit = slot_map.insert(());
        let sum = Sum::new().unit(unit, 124);
        let mut expected = BTreeMap::new();
        expected.insert(unit, 124);
        assert_eq!(sum.0, expected);
    }
    #[test]
    fn fmt_debug() {
        let mut slot_map = DenseSlotMap::new();
        let unit_a = slot_map.insert(());
        let amount_a = 76;
        let unit_b = slot_map.insert(());
        let amount_b = 45;
        let sum = Sum::of(unit_a, amount_a).unit(unit_b, amount_b);
        let actual = format!("{:?}", sum);
        let expected = format!(
            "Sum({{{:?}: {:?}, {:?}: {:?}}})",
            unit_a, amount_a, unit_b, amount_b
        );
        assert_eq!(actual, expected);
    }
}
