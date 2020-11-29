use crate::book::Uk;
use crate::sum::Sum;
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops;
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(Clone, PartialEq)]
pub struct Balance<'a>(pub(crate) BTreeMap<Uk, i128>, PhantomData<&'a ()>);
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
        let sum = Sum::of(unit_a, 2).and(unit_b, 3);
        actual.operation(&sum, |balance, amount| balance + amount as i128);
        let sum = Sum::of(unit_a, 2).and(unit_b, 3);
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
        let sum = Sum::of(unit_a, amount_a).and(unit_b, amount_b);
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
        actual -= &Sum::of(unit, 9);
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
        let actual = immutable - &Sum::of(unit, 9);
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
        actual += &Sum::of(unit, 9);
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
        let actual = immutable + &Sum::of(unit, 9);
        let expected = Balance(
            btreemap! {
                unit.clone() => 9,
            },
            PhantomData,
        );
        assert_eq!(actual, expected);
    }
}
