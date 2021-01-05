use crate::sum::Sum;
use crate::unit::Unit;
use std::collections::BTreeMap;
use std::fmt;
use std::ops;
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(PartialEq, Clone)]
pub struct Balance<U: Unit, Bn>(pub(crate) BTreeMap<U, Bn>);
impl<U: Unit, Bn> Balance<U, Bn> {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }
    fn apply_sum_operation<Sn>(
        &mut self,
        rhs: &Sum<U, Sn>,
        amount_op: fn(Bn, Sn) -> Bn,
    ) where
        Bn: Default + Clone,
        Sn: Clone,
    {
        rhs.0.iter().for_each(|(unit, amount)| {
            self.apply_unit_operation(
                &(unit.clone(), amount.clone()),
                amount_op,
            )
        });
    }
    fn apply_unit_operation<Sn>(
        &mut self,
        (unit, amount): &(U, Sn),
        amount_op: fn(Bn, Sn) -> Bn,
    ) where
        Bn: Default + Clone,
        Sn: Clone,
    {
        self.0
            .entry(unit.clone())
            .and_modify(|balance| {
                *balance = amount_op(balance.clone(), amount.clone());
            })
            .or_insert_with(|| amount_op(Default::default(), amount.clone()));
    }
    /// Gets the amounts of all units in undefined order.
    pub fn amounts(&self) -> impl Iterator<Item = (&U, &Bn)> {
        self.0.iter()
    }
    /// Gets the amount of a provided unit.
    pub fn unit_amount(&self, unit: U) -> Option<&Bn> {
        self.0.get(&unit)
    }
}
impl<U: Unit + fmt::Debug, Bn: fmt::Debug> fmt::Debug for Balance<U, Bn> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.iter()).finish()?;
        f.write_str(")")
    }
}
impl<U: Unit, Bn, Sn> ops::SubAssign<&Sum<U, Sn>> for Balance<U, Bn>
where
    Bn: Default + ops::Sub<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    fn sub_assign(&mut self, sum: &Sum<U, Sn>) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount.into()
        });
    }
}
impl<U: Unit, Bn, Sn> ops::SubAssign<&(U, Sn)> for Balance<U, Bn>
where
    Bn: Default + ops::Sub<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    fn sub_assign(&mut self, unit_amount: &(U, Sn)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance - amount.into()
        });
    }
}
impl<U: Unit, Bn, Sn> ops::Sub<&Sum<U, Sn>> for Balance<U, Bn>
where
    Bn: Default + ops::Sub<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    type Output = Self;
    fn sub(mut self, sum: &Sum<U, Sn>) -> Self::Output {
        self -= sum;
        self
    }
}
impl<U: Unit, Bn, Sn> ops::Sub<&(U, Sn)> for Balance<U, Bn>
where
    Bn: Default + ops::Sub<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    type Output = Self;
    fn sub(mut self, unit_amount: &(U, Sn)) -> Self::Output {
        self -= unit_amount;
        self
    }
}
impl<U: Unit, Bn, Sn> ops::AddAssign<&Sum<U, Sn>> for Balance<U, Bn>
where
    Bn: Default + ops::Add<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    fn add_assign(&mut self, sum: &Sum<U, Sn>) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount.into()
        });
    }
}
impl<U: Unit, Bn, Sn> ops::AddAssign<&(U, Sn)> for Balance<U, Bn>
where
    Bn: Default + ops::Add<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    fn add_assign(&mut self, unit_amount: &(U, Sn)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance + amount.into()
        });
    }
}
impl<U: Unit, Bn, Sn> ops::Add<&Sum<U, Sn>> for Balance<U, Bn>
where
    Bn: Default + ops::Add<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    type Output = Self;
    fn add(mut self, sum: &Sum<U, Sn>) -> Self::Output {
        self += sum;
        self
    }
}
impl<U: Unit, Bn, Sn> ops::Add<&(U, Sn)> for Balance<U, Bn>
where
    Bn: Default + ops::Add<Output = Bn> + Clone,
    Sn: Clone + Into<Bn>,
{
    type Output = Self;
    fn add(mut self, unit_amount: &(U, Sn)) -> Self::Output {
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
        let actual = Balance::<TestUnit, i128>::new();
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
        actual.apply_sum_operation(&sum, |balance, amount| balance + amount);
        let sum = sum!(2, usd; 3, thb);
        actual.apply_sum_operation(&sum, |balance, amount| balance * amount);
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
        let balance = Balance::<TestUnit, i128>::new() + &sum;
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
        let mut actual = Balance::<TestUnit, i128>::new();
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
        let immutable = Balance::<TestUnit, i128>::new();
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
        let mut actual = Balance::<TestUnit, i128>::new();
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
        let immutable = Balance::<TestUnit, i128>::new();
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
        let balance = Balance::<TestUnit, i128>::new()
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
        let balance =
            Balance::<TestUnit, i128>::new() + &sum!(200, usd; 100, thb);
        assert_eq!(balance.unit_amount(usd).unwrap(), &200);
        assert_eq!(balance.unit_amount(thb).unwrap(), &100);
        assert_eq!(balance.unit_amount(ils), None);
    }
}
