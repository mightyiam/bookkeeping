use crate::book::UnitKey;
use std::collections::BTreeMap;
use std::fmt;
/// Represents amounts of any number of units.
#[derive(Clone, PartialEq, Default)]
pub struct Sum(pub(crate) BTreeMap<UnitKey, u64>);
impl Sum {
    /// Creates an empty sum.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Sets the amount of a unit in a sum.
    pub fn set_amount_for_unit(&mut self, amount: u64, unit_key: UnitKey) {
        self.0.insert(unit_key, amount);
    }
    /// Gets the amounts of all units in undefined order.
    pub fn amounts(&self) -> impl Iterator<Item = (UnitKey, &u64)> {
        self.0.iter().map(|(unit_key, amount)| (*unit_key, amount))
    }
}
impl fmt::Debug for Sum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sum(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[cfg(test)]
mod test {
    use super::Sum;
    use maplit::btreemap;
    #[test]
    fn new() {
        let actual = Sum::new();
        let expected = Sum(btreemap! {});
        assert_eq!(actual, expected);
    }
    #[test]
    fn from_entries() {
        let mut book = test_book!("");
        let thb_key = book.new_unit("");
        let usd_key = book.new_unit("");
        let actual = sum!(100, thb_key; 200, usd_key);
        let expected = Sum(btreemap! {
            thb_key => 100,
            usd_key => 200,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn set_amount_for_unit() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        let mut actual = Sum::new();
        actual.set_amount_for_unit(3, unit_key);
        let expected = Sum(btreemap! { unit_key => 3 });
        assert_eq!(actual, expected);
    }
    #[test]
    fn amounts() {
        let mut book = test_book!("");
        let thb_key = book.new_unit("THB");
        let usd_key = book.new_unit("USD");
        let sum = sum!(3, thb_key; 10, usd_key);
        let actual = sum.amounts().collect::<Vec<_>>();
        let expected = vec![(thb_key, &3), (usd_key, &10)];
        assert_eq!(actual, expected);
    }
    #[test]
    fn fmt_debug() {
        let mut book = test_book!("");
        let unit_a_key = book.new_unit("");
        let amount_a = 76;
        let unit_b_key = book.new_unit("");
        let amount_b = 45;
        let sum = sum!(amount_a, unit_a_key; amount_b, unit_b_key);
        let actual = format!("{:?}", sum);
        let expected = format!(
            "Sum({{{:?}: {:?}, {:?}: {:?}}})",
            unit_a_key, amount_a, unit_b_key, amount_b
        );
        assert_eq!(actual, expected);
    }
}
