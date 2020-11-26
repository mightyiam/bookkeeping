use crate::sum::Sum;
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops;
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(Clone, PartialEq)]
pub struct Balance<'a, KU>(pub(crate) BTreeMap<KU, i128>, PhantomData<&'a ()>);
impl<KU: Ord + Clone> Balance<'_, KU> {
    pub(crate) fn new() -> Self {
        Self(Default::default(), Default::default())
    }
    fn operation(&mut self, rhs: &Sum<KU>, amount_op: fn(i128, u64) -> i128) {
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
impl<KU: Clone + fmt::Debug> fmt::Debug for Balance<'_, KU> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
impl<KU: Ord + Clone> ops::SubAssign<&Sum<KU>> for Balance<'_, KU> {
    fn sub_assign(&mut self, sum: &Sum<KU>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
impl<KU: Ord + Clone> ops::Sub<&Sum<KU>> for Balance<'_, KU> {
    type Output = Self;
    fn sub(self, sum: &Sum<KU>) -> Self::Output {
        let mut result = self.clone();
        result -= sum;
        result
    }
}
impl<KU: Ord + Clone> ops::AddAssign<&Sum<KU>> for Balance<'_, KU> {
    fn add_assign(&mut self, sum: &Sum<KU>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
impl<KU: Ord + Clone> ops::Add<&Sum<KU>> for Balance<'_, KU> {
    type Output = Self;
    fn add(self, sum: &Sum<KU>) -> Self::Output {
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
    use crate::book::Book;
    #[test]
    fn new() {
        let actual = Balance::<()>::new();
        let expected = Balance(BTreeMap::new(), PhantomData);
        assert_eq!(actual, expected);
    }
    #[test]
    fn operation() {
        test_book!(Book, TestBook);
        use maplit::btreemap;
        let mut balance = Balance::new();
        let mut book = TestBook::new(0);
        let unit_a = book.new_unit(0);
        let unit_b = book.new_unit(0);
        let sum = Sum::of(unit_a, 2).unit(unit_b, 3);
        balance.operation(&sum, |balance, amount| balance + amount as i128);
        let sum = Sum::of(unit_a, 2).unit(unit_b, 3);
        balance.operation(&sum, |balance, amount| balance * amount as i128);
        let expected = btreemap! {
            unit_a => 4,
            unit_b => 9,
        };
        assert_eq!(balance.0, expected);
    }
    #[test]
    fn fmt_debug() {
        test_book!(Book, TestBook);
        let mut book = TestBook::new(0);
        let unit_a = book.new_unit(0);
        let amount_a = 76;
        let unit_b = book.new_unit(0);
        let amount_b = 45;
        let sum = Sum::of(unit_a, amount_a).unit(unit_b, amount_b);
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
        test_book!(Book, TestBook);
        use maplit::btreemap;
        let mut book = TestBook::new(0);
        let unit = book.new_unit(0);
        let mut balance = Balance::new();
        balance -= &Sum::of(unit, 9);
        let expected = btreemap! {
            unit.clone() => -9,
        };
        assert_eq!(balance.0, expected);
    }
    #[test]
    fn sub_sum() {
        test_book!(Book, TestBook);
        use maplit::btreemap;
        let mut book = TestBook::new(0);
        let unit = book.new_unit(0);
        let balance = Balance::new();
        let balance = balance - &Sum::of(unit, 9);
        let expected = btreemap! {
            unit.clone() => -9,
        };
        assert_eq!(balance.0, expected);
    }
    #[test]
    fn add_assign_sum() {
        test_book!(Book, TestBook);
        use maplit::btreemap;
        let mut book = TestBook::new(0);
        let unit = book.new_unit(0);
        let mut balance = Balance::new();
        balance += &Sum::of(unit, 9);
        let expected = btreemap! {
            unit.clone() => 9,
        };
        assert_eq!(balance.0, expected);
    }
    #[test]
    fn add_sum() {
        test_book!(Book, TestBook);
        use maplit::btreemap;
        let mut book = TestBook::new(0);
        let unit = book.new_unit(0);
        let balance = Balance::new();
        let balance = balance + &Sum::of(unit, 9);
        let expected = btreemap! {
            unit.clone() => 9,
        };
        assert_eq!(balance.0, expected);
    }
}
