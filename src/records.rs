use crate::sum::Sum;
/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<MA> {
    pub(crate) meta: MA,
}
impl<MA> Account<MA> {
    pub(crate) fn new(meta: MA) -> Self {
        Self { meta }
    }
}
/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<MA> {
    pub(crate) meta: MA,
}
impl<MA> Unit<MA> {
    /// Creates a new unit.
    pub(crate) fn new(meta: MA) -> Self {
        Self { meta }
    }
}
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
pub struct Move<KA, KU: Ord, MM> {
    pub(crate) meta: MM,
    pub(crate) debit_account: KA,
    pub(crate) credit_account: KA,
    pub(crate) sum: Sum<KU>,
}
impl<KA: PartialEq, KU: Ord, MM> Move<KA, KU, MM> {
    pub(crate) fn new(debit_account: KA, credit_account: KA, sum: Sum<KU>, meta: MM) -> Self {
        assert!(
            debit_account != credit_account,
            "Debit and credit accounts are the same."
        );
        Self {
            meta,
            debit_account,
            credit_account,
            sum,
        }
    }
}
#[cfg(test)]
mod test {
    use super::Move;
    use super::Sum;
    use slotmap::{DefaultKey, DenseSlotMap};
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn move_new_panic_debit_and_credit_accounts_are_the_same() {
        let account_key = DenseSlotMap::new().insert(());
        Move::<_, DefaultKey, _>::new(account_key, account_key, Sum::new(), ());
    }
    #[test]
    fn new() {
        let mut slot_map = DenseSlotMap::new();
        let debit_account = slot_map.insert(());
        let credit_account = slot_map.insert(());
        let sum = Sum::<DefaultKey>::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), ());
        assert_eq!(move_.debit_account, debit_account);
        assert_eq!(move_.credit_account, credit_account);
        assert_eq!(move_.sum, sum);
    }
}
