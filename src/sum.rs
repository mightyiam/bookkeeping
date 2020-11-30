use crate::book::UnitKey;
use std::collections::BTreeMap;
use std::fmt;
/// Represents amounts of any number of units.
#[derive(Clone, PartialEq)]
pub struct Sum(pub(crate) BTreeMap<UnitKey, u64>);
impl Sum {
    /// Creates an empty sum.
    /// ```
    /// # use bookkeeping::Sum;
    /// let mut sum = Sum::new();
    /// # assert!(sum.amounts().next().is_none());
    /// ```
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    /// Creates a sum with an amount of a unit.
    #[cfg(test)]
    pub(crate) fn of(amount: u64, unit: UnitKey) -> Self {
        let mut sum = Self::new();
        sum.set_amount_for_unit(amount, unit);
        sum
    }
    /// Sets the amount of a unit in a sum.
    /// ```
    /// # use bookkeeping::Book;
    /// # use bookkeeping::Sum;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let usd = book.new_unit("USD");
    /// # let mut sum = Sum::new();
    /// sum.set_amount_for_unit(500, usd);
    /// # assert_eq!(sum.amounts().collect::<Vec<_>>(), vec![(&usd, &500)]);
    /// ```
    pub fn set_amount_for_unit(&mut self, amount: u64, unit: UnitKey) {
        self.0.insert(unit, amount);
    }
    /// Gets the amounts of all units in undefined order.
    /// ```
    /// # use bookkeeping::Book;
    /// # use bookkeeping::Sum;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let usd = book.new_unit("USD");
    /// # let thb = book.new_unit("THB");
    /// # let mut sum = Sum::new();
    /// # sum.set_amount_for_unit(500, usd);
    /// # sum.set_amount_for_unit(900, thb);
    /// assert_eq!(
    ///     sum.amounts().collect::<Vec<_>>(),
    ///     vec![(&usd, &500), (&thb, &900)],
    /// );
    /// ```
    pub fn amounts(&self) -> impl Iterator<Item = (&UnitKey, &u64)> {
        self.0.iter()
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
    fn of() {
        let mut book = test_book!("");
        let unit = book.new_unit("");
        let actual = Sum::of(24, unit);
        let expected = Sum(btreemap! { unit => 24 });
        assert_eq!(actual, expected);
    }
    #[test]
    fn from_entries() {
        let mut book = test_book!("");
        let thb = book.new_unit("");
        let usd = book.new_unit("");
        let actual = sum!(100, thb; 200, usd);
        let expected = Sum(btreemap! {
            thb => 100,
            usd => 200,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn set_amount_for_unit() {
        let mut book = test_book!("");
        let unit = book.new_unit("");
        let mut actual = Sum::new();
        actual.set_amount_for_unit(3, unit);
        let expected = Sum(btreemap! { unit => 3 });
        assert_eq!(actual, expected);
    }
    #[test]
    fn amounts() {
        let mut book = test_book!("");
        let thb = book.new_unit("THB");
        let usd = book.new_unit("USD");
        let sum = sum!(3, thb; 10, usd);
        let actual = sum.amounts().collect::<Vec<_>>();
        let expected = vec![(&thb, &3), (&usd, &10)];
        assert_eq!(actual, expected);
    }
    #[test]
    fn fmt_debug() {
        let mut book = test_book!("");
        let unit_a = book.new_unit("");
        let amount_a = 76;
        let unit_b = book.new_unit("");
        let amount_b = 45;
        let sum = sum!(amount_a, unit_a; amount_b, unit_b);
        let actual = format!("{:?}", sum);
        let expected = format!(
            "Sum({{{:?}: {:?}, {:?}: {:?}}})",
            unit_a, amount_a, unit_b, amount_b
        );
        assert_eq!(actual, expected);
    }
}
