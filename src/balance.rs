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
    fn apply_sum_operation(
        &mut self,
        rhs: &Sum,
        amount_op: fn(i128, u64) -> i128,
    ) {
        rhs.0.iter().for_each(|(unit, amount)| {
            self.apply_unit_operation(&(*unit, *amount), amount_op)
        });
    }
    fn apply_unit_operation(
        &mut self,
        (unit, amount): &(UnitKey, u64),
        amount_op: fn(i128, u64) -> i128,
    ) {
        self.0
            .entry(unit.clone())
            .and_modify(|balance| {
                *balance = amount_op(*balance, *amount);
            })
            .or_insert(amount_op(0, *amount));
    }
    /// Gets the amounts of all units in undefined order.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # use bookkeeping::Sum;
    /// # use std::collections::HashSet;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("");
    /// # let thb_key = book.new_unit("");
    /// # let ils_key = book.new_unit("");
    /// # let wallet_key = book.new_account("");
    /// # let bank_key = book.new_account("");
    /// # let mut sum = Sum::new();
    /// # sum.set_amount_for_unit(100, usd_key);
    /// # sum.set_amount_for_unit(200, thb_key);
    /// # sum.set_amount_for_unit(300, ils_key);
    /// # let move_key = book.insert_move(0, wallet_key, bank_key, sum, "");
    /// # let balance = book.account_balance_at_move(bank_key, move_key);
    /// let amounts = balance.amounts().collect::<HashSet<_>>();
    /// assert!(amounts.contains(&(usd_key, &100)));
    /// assert!(amounts.contains(&(thb_key, &200)));
    /// assert!(amounts.contains(&(ils_key, &300)));
    /// ```
    pub fn amounts(&self) -> impl Iterator<Item = (UnitKey, &i128)> {
        self.0.iter().map(|(unit_key, amount)| (*unit_key, amount))
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
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
impl ops::SubAssign<&(UnitKey, u64)> for Balance<'_> {
    fn sub_assign(&mut self, unit_amount: &(UnitKey, u64)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance - amount as i128
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
impl ops::Sub<&(UnitKey, u64)> for Balance<'_> {
    type Output = Self;
    fn sub(self, unit_amount: &(UnitKey, u64)) -> Self::Output {
        let mut result = self.clone();
        result -= unit_amount;
        result
    }
}
impl ops::AddAssign<&Sum> for Balance<'_> {
    fn add_assign(&mut self, sum: &Sum) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
impl ops::AddAssign<&(UnitKey, u64)> for Balance<'_> {
    fn add_assign(&mut self, unit_amount: &(UnitKey, u64)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance + amount as i128
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
impl ops::Add<&(UnitKey, u64)> for Balance<'_> {
    type Output = Self;
    fn add(self, unit_amount: &(UnitKey, u64)) -> Self::Output {
        let mut result = self.clone();
        result += unit_amount;
        result
    }
}
#[cfg(test)]
mod test {
    use super::BTreeMap;
    use super::Balance;
    use super::PhantomData;
    use maplit::btreemap;
    #[test]
    fn new() {
        let actual = Balance::new();
        let expected = Balance(BTreeMap::new(), PhantomData);
        assert_eq!(actual, expected);
    }
    #[test]
    fn apply_sum_operation() {
        use maplit::btreemap;
        let mut actual = Balance::new();
        let mut book = test_book!("");
        let unit_a_key = book.new_unit("");
        let unit_b_key = book.new_unit("");
        let sum = sum!(2, unit_a_key; 3, unit_b_key);
        actual.apply_sum_operation(&sum, |balance, amount| {
            balance + amount as i128
        });
        let sum = sum!(2, unit_a_key; 3, unit_b_key);
        actual.apply_sum_operation(&sum, |balance, amount| {
            balance * amount as i128
        });
        let expected = Balance(
            btreemap! {
                unit_a_key => 4,
                unit_b_key => 9,
            },
            PhantomData,
        );
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
        let balance = Balance::new() + &sum;
        let actual = format!("{:?}", balance);
        let expected = format!(
            "Balance({{{:?}: {:?}, {:?}: {:?}}})",
            unit_a_key, amount_a, unit_b_key, amount_b
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_assign_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        let mut actual = Balance::new();
        actual -= &sum!(9, unit_key);
        let expected = Balance(
            btreemap! {
                unit_key.clone() => -9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        let immutable = Balance::new();
        let actual = immutable - &sum!(9, unit_key);
        let expected = Balance(
            btreemap! {
                unit_key.clone() => -9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_assign_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        let mut actual = Balance::new();
        actual += &sum!(9, unit_key);
        let expected = Balance(
            btreemap! {
                unit_key.clone() => 9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_sum() {
        use maplit::btreemap;
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        let immutable = Balance::new();
        let actual = immutable + &sum!(9, unit_key);
        let expected = Balance(
            btreemap! {
                unit_key.clone() => 9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn amounts() {
        let mut book = test_book!("");
        let usd_key = book.new_unit("");
        let thb_key = book.new_unit("");
        let ils_key = book.new_unit("");
        let balance = Balance::new()
            + &sum! {
                100, usd_key; 200, thb_key; 300, ils_key
            };
        let actual = balance.amounts().collect::<Vec<_>>();
        let expected = vec![(usd_key, &100), (thb_key, &200), (ils_key, &300)];
        assert_eq!(actual, expected);
    }
}
