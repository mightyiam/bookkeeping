use crate::sum::Sum;
use crate::unit::Unit;
use std::collections::BTreeMap;
use std::fmt;
use std::ops;
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(PartialEq, Clone)]
pub struct Balance<U: Unit>(pub(crate) BTreeMap<U, i128>);
impl<U: Unit> Balance<U> {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }
    fn apply_sum_operation(
        &mut self,
        rhs: &Sum<U>,
        amount_op: fn(i128, u64) -> i128,
    ) {
        rhs.0.iter().for_each(|(unit, amount)| {
            self.apply_unit_operation(&(unit.clone(), *amount), amount_op)
        });
    }
    fn apply_unit_operation(
        &mut self,
        (unit, amount): &(U, u64),
        amount_op: fn(i128, u64) -> i128,
    ) {
        self.0
            .entry(unit.clone())
            .and_modify(|balance| {
                *balance = amount_op(*balance, *amount);
            })
            .or_insert_with(|| amount_op(0, *amount));
    }
    /// Gets the amounts of all units in undefined order.
    pub fn amounts(&self) -> impl Iterator<Item = (&U, &i128)> {
        self.0.iter()
    }
    /// Gets the amount of a provided unit.
    pub fn unit_amount(&self, unit: U) -> Option<&i128> {
        self.0.get(&unit)
    }
}
impl<U: Unit + fmt::Debug> fmt::Debug for Balance<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
impl<U: Unit> ops::SubAssign<&Sum<U>> for Balance<U> {
    fn sub_assign(&mut self, sum: &Sum<U>) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
impl<U: Unit> ops::SubAssign<&(U, u64)> for Balance<U> {
    fn sub_assign(&mut self, unit_amount: &(U, u64)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance - amount as i128
        });
    }
}
impl<U: Unit> ops::Sub<&Sum<U>> for Balance<U> {
    type Output = Self;
    fn sub(mut self, sum: &Sum<U>) -> Self::Output {
        self -= sum;
        self
    }
}
impl<U: Unit> ops::Sub<&(U, u64)> for Balance<U> {
    type Output = Self;
    fn sub(mut self, unit_amount: &(U, u64)) -> Self::Output {
        self -= unit_amount;
        self
    }
}
impl<U: Unit> ops::AddAssign<&Sum<U>> for Balance<U> {
    fn add_assign(&mut self, sum: &Sum<U>) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
impl<U: Unit> ops::AddAssign<&(U, u64)> for Balance<U> {
    fn add_assign(&mut self, unit_amount: &(U, u64)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance + amount as i128
        });
    }
}
impl<U: Unit> ops::Add<&Sum<U>> for Balance<U> {
    type Output = Self;
    fn add(mut self, sum: &Sum<U>) -> Self::Output {
        self += sum;
        self
    }
}
impl<U: Unit> ops::Add<&(U, u64)> for Balance<U> {
    type Output = Self;
    fn add(mut self, unit_amount: &(U, u64)) -> Self::Output {
        self += unit_amount;
        self
    }
}
#[cfg(test)]
mod test {
    use super::BTreeMap;
    use super::Balance;
    use crate::unit::TestUnit;
    use maplit::btreemap;
    #[test]
    fn new() {
        let actual = Balance::<TestUnit>::new();
        let expected = Balance(BTreeMap::new());
        assert_eq!(actual, expected);
    }
    #[test]
    fn apply_sum_operation() {
        use maplit::btreemap;
        let mut actual = Balance::new();
        let usd = TestUnit("USD");
        let thb = TestUnit("THB");
        let sum = sum!(2, usd; 3, thb);
        actual.apply_sum_operation(&sum, |balance, amount| {
            balance + amount as i128
        });
        let sum = sum!(2, usd; 3, thb);
        actual.apply_sum_operation(&sum, |balance, amount| {
            balance * amount as i128
        });
        let expected = Balance(btreemap! {
            usd => 4,
            thb => 9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn fmt_debug() {
        let usd = TestUnit("USD");
        let amount_usd = 76;
        let thb = TestUnit("THB");
        let amount_thb = 45;
        let sum = sum!(amount_usd, usd; amount_thb, thb);
        let balance = Balance::new() + &sum;
        let actual = format!("{:?}", balance);
        let expected = format!(
            "Balance({{{:?}: {:?}, {:?}: {:?}}})",
            thb, amount_thb, usd, amount_usd
        );
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_assign_sum() {
        use maplit::btreemap;
        let usd = TestUnit("USD");
        let mut actual = Balance::new();
        actual -= &sum!(9, usd);
        let expected = Balance(btreemap! {
            usd => -9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn sub_sum() {
        use maplit::btreemap;
        let usd = TestUnit("USD");
        let immutable = Balance::new();
        let actual = immutable - &sum!(9, usd);
        let expected = Balance(btreemap! {
            usd => -9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_assign_sum() {
        use maplit::btreemap;
        let usd = TestUnit("USD");
        let mut actual = Balance::new();
        actual += &sum!(9, usd);
        let expected = Balance(btreemap! {
            usd => 9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_sum() {
        use maplit::btreemap;
        let usd = TestUnit("USD");
        let immutable = Balance::new();
        let actual = immutable + &sum!(9, usd);
        let expected = Balance(btreemap! {
            usd => 9,
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn amounts() {
        let usd = TestUnit("USD");
        let thb = TestUnit("THB");
        let ils = TestUnit("ILS");
        let balance = Balance::new()
            + &sum! {
                100, usd; 200, thb; 300, ils
            };
        let actual = balance.amounts().collect::<Vec<_>>();
        let expected = vec![(&ils, &300), (&thb, &200), (&usd, &100)];
        assert_eq!(actual, expected);
    }
    #[test]
    fn unit_amount() {
        let usd = TestUnit("USD");
        let thb = TestUnit("THB");
        let ils = TestUnit("ILS");
        let balance = Balance::new() + &sum!(200, usd; 100, thb);
        assert_eq!(balance.unit_amount(usd).unwrap(), &200);
        assert_eq!(balance.unit_amount(thb).unwrap(), &100);
        assert_eq!(balance.unit_amount(ils), None);
    }
}
