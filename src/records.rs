/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<Am> {
    pub(crate) meta: Am,
}
impl<Am> Account<Am> {
    pub(crate) fn new(meta: Am) -> Self {
        Self { meta }
    }
}
/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<Um> {
    pub(crate) meta: Um,
}
impl<Um> Unit<Um> {
    /// Creates a new unit.
    pub(crate) fn new(meta: Um) -> Self {
        Self { meta }
    }
}
use crate::book::AccountKey;
use crate::sum::Sum;
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
pub struct Move<Mm> {
    pub(crate) meta: Mm,
    pub(crate) debit_account: AccountKey,
    pub(crate) credit_account: AccountKey,
    pub(crate) sum: Sum,
}
impl<Mm> Move<Mm> {
    pub(crate) fn new(
        debit_account: AccountKey,
        credit_account: AccountKey,
        sum: Sum,
        meta: Mm,
    ) -> Self {
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
    use super::AccountKey;
    use super::Move;
    use super::Sum;
    use slotmap::DenseSlotMap;
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn move_new_panic_debit_and_credit_accounts_are_the_same() {
        let account_key = DenseSlotMap::<AccountKey, ()>::with_key().insert(());
        Move::new(account_key, account_key, Sum::new(), ());
    }
    #[test]
    fn new() {
        let mut slot_map = DenseSlotMap::<AccountKey, ()>::with_key();
        let debit_account = slot_map.insert(());
        let credit_account = slot_map.insert(());
        let sum = Sum::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), ());
        assert_eq!(move_.debit_account, debit_account);
        assert_eq!(move_.credit_account, credit_account);
        assert_eq!(move_.sum, sum);
    }
}
