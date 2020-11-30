use crate::book::UnitKey;
use crate::sum::Sum;
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops;
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(Clone, PartialEq)]
pub struct Balance<'a>(pub(crate) BTreeMap<UnitKey, i128>, PhantomData<&'a ()>);
impl Balance<'_> {
    pub(crate) fn new() -> Self {
        Self(Default::default(), Default::default())
    }
    fn operation(&mut self, rhs: &Sum, amount_op: fn(i128, u64) -> i128) {
        rhs.0.iter().for_each(|(unit, amount)| {
            self.0
                .entry(unit.clone())
                .and_modify(|balance| {
                    *balance = amount_op(*balance, *amount);
                })
                .or_insert(amount_op(0, *amount));
        });
    }
    /// Gets the amounts of all units in undefined order.
    ///
    /// ```
    /// # use bookkeeping::Book;
    /// # use bookkeeping::Sum;
    /// # use std::collections::HashSet;
    /// # let mut book = Book::<&str, &str, &str, u8>::new("");
    /// # let usd = book.new_unit("");
    /// # let thb = book.new_unit("");
    /// # let ils = book.new_unit("");
    /// # let debit_account = book.new_account("");
    /// # let credit_account = book.new_account("");
    /// # let mut sum = Sum::new();
    /// # sum.set_amount_for_unit(100, usd);
    /// # sum.set_amount_for_unit(200, thb);
    /// # sum.set_amount_for_unit(300, ils);
    /// # let move_ = book.new_move(debit_account, credit_account, sum, 0);
    /// # let balance = book.account_balance_with_move(credit_account, move_, |a, b| a.cmp(b));
    /// let amounts = balance.amounts().collect::<HashSet<_>>();
    /// assert!(amounts.contains(&(&usd, &100)));
    /// assert!(amounts.contains(&(&thb, &200)));
    /// assert!(amounts.contains(&(&ils, &300)));
    /// ```
    pub fn amounts(&self) -> impl Iterator<Item = (&UnitKey, &i128)> {
        self.0.iter()
    }
}
impl fmt::Debug for Balance<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
impl ops::SubAssign<&Sum> for Balance<'_> {
    fn sub_assign(&mut self, sum: &Sum) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
impl ops::Sub<&Sum> for Balance<'_> {
    type Output = Self;
    fn sub(self, sum: &Sum) -> Self::Output {
        let mut result = self.clone();
        result -= sum;
        result
    }
}
impl ops::AddAssign<&Sum> for Balance<'_> {
    fn add_assign(&mut self, sum: &Sum) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
impl ops::Add<&Sum> for Balance<'_> {
    type Output = Self;
    fn add(self, sum: &Sum) -> Self::Output {
        let mut result = self.clone();
        result += sum;
        result
    }
}
#[cfg(test)]
mod test {
    use super::BTreeMap;
    use super::Balance;
    use super::PhantomData;
    use super::Sum;
    use maplit::btreemap;
    #[test]
    fn new() {
        let actual = Balance::new();
        let expected = Balance(BTreeMap::new(), PhantomData);
        assert_eq!(actual, expected);
    }
    #[test]
    fn operation() {
        use maplit::btreemap;
        let mut actual = Balance::new();
        let mut book = test_book!("");
        let unit_a = book.new_unit("");
        let unit_b = book.new_unit("");
        let sum = sum!(2, unit_a; 3, unit_b);
        actual.operation(&sum, |balance, amount| balance + amount as i128);
        let sum = sum!(2, unit_a; 3, unit_b);
        actual.operation(&sum, |balance, amount| balance * amount as i128);
        let expected = Balance(
            btreemap! {
                unit_a => 4,
                unit_b => 9,
            },
            PhantomData,
        );
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
        let balance = Balance::new() + &sum;
        let actual = format!("{:?}", balance);
        let expected = format!(
            "Balance({{{:?}: {:?}, {:?}: {:?}}})",
            unit_a, amount_a, unit_b, amount_b
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_assign_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit = book.new_unit("");
        let mut actual = Balance::new();
        actual -= &Sum::of(9, unit);
        let expected = Balance(
            btreemap! {
                unit.clone() => -9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit = book.new_unit("");
        let immutable = Balance::new();
        let actual = immutable - &Sum::of(9, unit);
        let expected = Balance(
            btreemap! {
                unit.clone() => -9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_assign_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit = book.new_unit("");
        let mut actual = Balance::new();
        actual += &Sum::of(9, unit);
        let expected = Balance(
            btreemap! {
                unit.clone() => 9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit = book.new_unit("");
        let immutable = Balance::new();
        let actual = immutable + &Sum::of(9, unit);
        let expected = Balance(
            btreemap! {
                unit.clone() => 9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn amounts() {
        let mut book = test_book!("");
        let usd = book.new_unit("");
        let thb = book.new_unit("");
        let ils = book.new_unit("");
        let balance = Balance::new()
            + &sum! {
                100, usd; 200, thb; 300, ils
            };
        let actual = balance.amounts().collect::<Vec<_>>();
        let expected = vec![(&usd, &100), (&thb, &200), (&ils, &300)];
        assert_eq!(actual, expected);
    }
}
