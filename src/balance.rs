use crate::sum::Sum;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
/// Represents a [balance](https://en.wikipedia.org/wiki/Balance_(accounting)), yet not necessarily the current balance.
#[derive(PartialEq, Clone)]
pub struct Balance<Unit, Number>(pub(crate) BTreeMap<Unit, Number>);
impl<Unit, Number> Balance<Unit, Number>
where
    Unit: Ord + Clone,
{
    fn apply_sum_operation<SumNumber>(
        &mut self,
        rhs: &Sum<Unit, SumNumber>,
        amount_op: fn(Number, SumNumber) -> Number,
    ) where
        Number: Default + Clone,
        SumNumber: Clone,
    {
        rhs.0.iter().for_each(|(unit, amount)| {
            self.apply_unit_operation(
                &(unit.clone(), amount.clone()),
                amount_op,
            )
        });
    }
    fn apply_unit_operation<SumNumber>(
        &mut self,
        (unit, amount): &(Unit, SumNumber),
        amount_op: fn(Number, SumNumber) -> Number,
    ) where
        Number: Default + Clone,
        SumNumber: Clone,
    {
        self.0
            .entry(unit.clone())
            .and_modify(|balance| {
                *balance = amount_op(balance.clone(), amount.clone());
            })
            .or_insert_with(|| amount_op(Default::default(), amount.clone()));
    }
    /// Gets the amounts of all units in undefined order.
    pub fn amounts(&self) -> impl Iterator<Item = (&Unit, &Number)> {
        self.0.iter()
    }
    /// Gets the amount of a provided unit.
    pub fn unit_amount(&self, unit: Unit) -> Option<&Number> {
        self.0.get(&unit)
    }
}
impl<Unit, Number> Default for Balance<Unit, Number>
where
    Unit: Ord,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<Unit, Number> fmt::Debug for Balance<Unit, Number>
where
    Unit: fmt::Debug,
    Number: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.iter()).finish()?;
        f.write_str(")")
    }
}
impl<Unit, Number, SumNumber> SubAssign<&Sum<Unit, SumNumber>>
    for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Sub<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    fn sub_assign(&mut self, sum: &Sum<Unit, SumNumber>) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount.into()
        });
    }
}
impl<Unit, Number, SumNumber> SubAssign<&(Unit, SumNumber)>
    for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Sub<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    fn sub_assign(&mut self, unit_amount: &(Unit, SumNumber)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance - amount.into()
        });
    }
}
impl<Unit, Number, SumNumber> Sub<&Sum<Unit, SumNumber>>
    for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Sub<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    type Output = Self;
    fn sub(mut self, sum: &Sum<Unit, SumNumber>) -> Self::Output {
        self -= sum;
        self
    }
}
impl<Unit, Number, SumNumber> Sub<&(Unit, SumNumber)> for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Sub<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    type Output = Self;
    fn sub(mut self, unit_amount: &(Unit, SumNumber)) -> Self::Output {
        self -= unit_amount;
        self
    }
}
impl<Unit, Number, SumNumber> AddAssign<&Sum<Unit, SumNumber>>
    for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Add<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    fn add_assign(&mut self, sum: &Sum<Unit, SumNumber>) {
        self.apply_sum_operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount.into()
        });
    }
}
impl<Unit, Number, SumNumber> AddAssign<&(Unit, SumNumber)>
    for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Add<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    fn add_assign(&mut self, unit_amount: &(Unit, SumNumber)) {
        self.apply_unit_operation(unit_amount, |balance, amount| {
            balance + amount.into()
        });
    }
}
impl<Unit, Number, SumNumber> Add<&Sum<Unit, SumNumber>>
    for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Add<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    type Output = Self;
    fn add(mut self, sum: &Sum<Unit, SumNumber>) -> Self::Output {
        self += sum;
        self
    }
}
impl<Unit, Number, SumNumber> Add<&(Unit, SumNumber)> for Balance<Unit, Number>
where
    Unit: Ord + Clone,
    Number: Default + Add<Output = Number> + Clone,
    SumNumber: Clone + Into<Number>,
{
    type Output = Self;
    fn add(mut self, unit_amount: &(Unit, SumNumber)) -> Self::Output {
        self += unit_amount;
        self
    }
}
#[cfg(test)]
mod test {
    use super::Balance;
    use crate::test_utils::{TestBalance, TestUnit};
    use maplit::btreemap;
    #[test]
    fn apply_sum_operation() {
        use maplit::btreemap;
        let mut actual: TestBalance = Default::default();
        let usd = TestUnit("USD");
        let thb = TestUnit("THB");
        let sum = sum!(2, usd; 3, thb);
        actual.apply_sum_operation(&sum, |balance, amount| {
            let rhs: i128 = amount.into();
            balance + rhs
        });
        let sum = sum!(2, usd; 3, thb);
        actual.apply_sum_operation(&sum, |balance, amount| {
            let rhs: i128 = amount.into();
            balance * rhs
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
        let balance = TestBalance::default() + &sum;
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
        let mut actual: TestBalance = Default::default();
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
        let immutable: TestBalance = Default::default();
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
        let mut actual: TestBalance = Default::default();
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
        let immutable: TestBalance = Default::default();
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
        let balance = TestBalance::default()
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
        let balance = TestBalance::default() + &sum!(200, usd; 100, thb);
        assert_eq!(balance.unit_amount(usd).unwrap(), &200);
        assert_eq!(balance.unit_amount(thb).unwrap(), &100);
        assert_eq!(balance.unit_amount(ils), None);
    }
}
