use crate::book::Book;
use crate::metadata::{BlankMetadata, Metadata};
use crate::sum::Sum;
use crate::unit::Unit;
use std::collections::BTreeMap;
use std::fmt;
use std::ops;
use std::rc::Rc;
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
#[test]
fn balance_new() {
    let actual = Balance::<BlankMetadata>::new();
    let expected = Balance(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn balance_operation() {
    use maplit::btreemap;
    let mut actual = Balance::new();
    let book = Book::<BlankMetadata>::new(());
    let unit_a = Unit::new(&book, ());
    let unit_b = Unit::new(&book, ());
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
impl<T: Metadata> fmt::Debug for Balance<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn balance_fmt_debug() {
    let book = Book::<BlankMetadata>::new(());
    let unit_a = Unit::new(&book, ());
    let amount_a = 76;
    let unit_b = Unit::new(&book, ());
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
impl<T: Metadata> ops::SubAssign<&Sum<T>> for Balance<T> {
    fn sub_assign(&mut self, sum: &Sum<T>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
#[test]
fn balance_sub_assign_sum() {
    use maplit::btreemap;
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let mut actual = Balance::new();
    actual -= &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl<T: Metadata> ops::Sub<&Sum<T>> for Balance<T> {
    type Output = Self;
    fn sub(self, sum: &Sum<T>) -> Self::Output {
        let mut result = self.clone();
        result -= sum;
        result
    }
}
#[test]
fn balance_sub_sum() {
    use maplit::btreemap;
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let balance = Balance::new();
    let actual = balance - &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl<T: Metadata> ops::AddAssign<&Sum<T>> for Balance<T> {
    fn add_assign(&mut self, sum: &Sum<T>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
#[test]
fn balance_add_assign_sum() {
    use maplit::btreemap;
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let mut actual = Balance::new();
    actual += &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
impl<T: Metadata> ops::Add<&Sum<T>> for Balance<T> {
    type Output = Self;
    fn add(self, sum: &Sum<T>) -> Self::Output {
        let mut result = self.clone();
        result += sum;
        result
    }
}
#[test]
fn balance_add_sum() {
    use maplit::btreemap;
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let balance = Balance::new();
    let actual = balance + &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
