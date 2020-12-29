use crate::unit::Unit;
use std::collections::BTreeMap;
use std::fmt;
/// Represents amounts of any number of units.
#[derive(Clone, PartialEq, Default)]
pub struct Sum<U: Unit>(pub(crate) BTreeMap<U, u64>);
impl<U: Unit> Sum<U> {
    /// Creates an empty sum.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Sets the amount of a unit in a sum.
    pub fn set_amount_for_unit(&mut self, amount: u64, unit_: U) {
        self.0.insert(unit_, amount);
    }
    /// Gets the amounts of all units in undefined order.
    pub fn amounts(&self) -> impl Iterator<Item = (&U, &u64)> {
        self.0.iter()
    }
}
impl<U: Unit + fmt::Debug> fmt::Debug for Sum<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sum(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[cfg(test)]
mod test {
    use super::Sum;
    use crate::unit::TestUnit;
    use maplit::btreemap;
    #[test]
    fn new() {
        let actual = Sum::<TestUnit>::new();
        let expected = Sum(btreemap! {});
        assert_eq!(actual, expected);
    }
    #[test]
    fn from_entries() {
        let thb = TestUnit("THB");
        let usd = TestUnit("USD");
        let actual = sum!(100, thb; 200, usd);
        let expected = Sum(btreemap! {
            thb => 100,
            usd => 200,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn set_amount_for_unit() {
        let unit = TestUnit("USD");
        let mut actual = Sum::new();
        actual.set_amount_for_unit(3, unit);
        let expected = Sum(btreemap! { unit => 3 });
        assert_eq!(actual, expected);
    }
    #[test]
    fn amounts() {
        let thb = TestUnit("THB");
        let usd = TestUnit("USD");
        let sum = sum!(3, thb; 10, usd);
        let actual = sum.amounts().collect::<Vec<_>>();
        let expected = vec![(&thb, &3), (&usd, &10)];
        assert_eq!(actual, expected);
    }
    #[test]
    fn fmt_debug() {
        let usd = TestUnit("USD");
        let amount_usd = 76;
        let thb = TestUnit("THB");
        let amount_thb = 45;
        let sum = sum!(amount_usd, usd; amount_thb, thb);
        let actual = format!("{:?}", sum);
        let expected = format!(
            "Sum({{{:?}: {:?}, {:?}: {:?}}})",
            thb, amount_thb, usd, amount_usd
        );
        assert_eq!(actual, expected);
    }
}
