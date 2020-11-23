use crate::metadata::Metadata;
use crate::sum::Sum;
use crate::unit::Unit;
use std::collections::BTreeMap;
use std::fmt;
use std::ops;
use std::rc::Rc;
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(Clone, PartialEq)]
pub struct Balance<T: Metadata>(pub(crate) BTreeMap<Rc<Unit<T>>, i128>);
impl<T: Metadata> Balance<T> {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }
    fn operation(&mut self, rhs: &Sum<T>, amount_op: fn(i128, u64) -> i128) {
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
impl<T: Metadata> fmt::Debug for Balance<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
impl<T: Metadata> ops::SubAssign<&Sum<T>> for Balance<T> {
    fn sub_assign(&mut self, sum: &Sum<T>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
impl<T: Metadata> ops::Sub<&Sum<T>> for Balance<T> {
    type Output = Self;
    fn sub(self, sum: &Sum<T>) -> Self::Output {
        let mut result = self.clone();
        result -= sum;
        result
    }
}
impl<T: Metadata> ops::AddAssign<&Sum<T>> for Balance<T> {
    fn add_assign(&mut self, sum: &Sum<T>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
impl<T: Metadata> ops::Add<&Sum<T>> for Balance<T> {
    type Output = Self;
    fn add(self, sum: &Sum<T>) -> Self::Output {
        let mut result = self.clone();
        result += sum;
        result
    }
}
#[cfg(test)]
mod test {
    use super::BTreeMap;
    use super::Balance;
    use super::Sum;
    use crate::book::Book;
    use crate::metadata::BlankMetadata;
    #[test]
    fn new() {
        let actual = Balance::<BlankMetadata>::new();
        let expected = Balance(BTreeMap::new());
        assert_eq!(actual, expected);
    }
    #[test]
    fn operation() {
        use maplit::btreemap;
        let mut actual = Balance::new();
        let mut book = Book::<BlankMetadata>::new(());
        let unit_a = book.new_unit(());
        let unit_b = book.new_unit(());
        let sum = Sum::of(&unit_a, 2).unit(&unit_b, 3);
        actual.operation(&sum, |balance, amount| balance + amount as i128);
        let sum = Sum::of(&unit_a, 2).unit(&unit_b, 3);
        actual.operation(&sum, |balance, amount| balance * amount as i128);
        let expected = Balance(btreemap! {
            unit_a.clone() => 4,
            unit_b.clone() => 9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn fmt_debug() {
        let mut book = Book::<BlankMetadata>::new(());
        let unit_a = book.new_unit(());
        let amount_a = 76;
        let unit_b = book.new_unit(());
        let amount_b = 45;
        let sum = Sum::of(&unit_a, amount_a).unit(&unit_b, amount_b);
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
        let mut book = Book::<BlankMetadata>::new(());
        let unit = book.new_unit(());
        let mut actual = Balance::new();
        actual -= &Sum::of(&unit, 9);
        let expected = Balance(btreemap! {
            unit.clone() => -9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_sum() {
        use maplit::btreemap;
        let mut book = Book::<BlankMetadata>::new(());
        let unit = book.new_unit(());
        let balance = Balance::new();
        let actual = balance - &Sum::of(&unit, 9);
        let expected = Balance(btreemap! {
            unit.clone() => -9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_assign_sum() {
        use maplit::btreemap;
        let mut book = Book::<BlankMetadata>::new(());
        let unit = book.new_unit(());
        let mut actual = Balance::new();
        actual += &Sum::of(&unit, 9);
        let expected = Balance(btreemap! {
            unit.clone() => 9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_sum() {
        use maplit::btreemap;
        let mut book = Book::<BlankMetadata>::new(());
        let unit = book.new_unit(());
        let balance = Balance::new();
        let actual = balance + &Sum::of(&unit, 9);
        let expected = Balance(btreemap! {
            unit.clone() => 9,
        });
        assert_eq!(actual, expected);
    }
}
