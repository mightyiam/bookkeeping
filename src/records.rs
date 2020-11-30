use crate::book::Ak;
use crate::sum::Sum;
use duplicate::duplicate;
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
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
pub struct Move<Mm> {
    pub(crate) meta: Mm,
    pub(crate) debit_account: Ak,
    pub(crate) credit_account: Ak,
    pub(crate) sum: Sum,
}
impl<Mm> Move<Mm> {
    pub(crate) fn new(
        debit_account: Ak,
        credit_account: Ak,
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
#[duplicate(
    Record    M   ;
    [Account] [Ma];
    [Unit]    [Mu];
    [Move]    [Mm];
)]
impl<M> Record<M> {
    /// Gets the metadata of the record.
    pub fn metadata(&self) -> &M {
        &self.meta
    }
}
#[cfg(test)]
mod test {
    use super::Account;
    use super::Ak;
    use super::Move;
    use super::Sum;
    use super::Unit;
    use slotmap::DenseSlotMap;
    #[test]
    fn account_metadata() {
        let account = Account::new(5);
        assert_eq!(*account.metadata(), 5);
    }
    #[test]
    fn unit_metadata() {
        let unit = Unit::new(5);
        assert_eq!(*unit.metadata(), 5);
    }
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn move_new_panic_debit_and_credit_accounts_are_the_same() {
        let account_key = DenseSlotMap::<Ak, ()>::with_key().insert(());
        Move::new(account_key, account_key, Sum::new(), ());
    }
    #[test]
    fn move_new() {
        let mut slot_map = DenseSlotMap::<Ak, ()>::with_key();
        let debit_account = slot_map.insert(());
        let credit_account = slot_map.insert(());
        let sum = Sum::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), ());
        assert_eq!(move_.debit_account, debit_account);
        assert_eq!(move_.credit_account, credit_account);
        assert_eq!(move_.sum, sum);
    }
    #[test]
    fn move_metadata() {
        let mut slot_map = DenseSlotMap::<Ak, ()>::with_key();
        let debit_account = slot_map.insert(());
        let credit_account = slot_map.insert(());
        let sum = Sum::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), 5);
        assert_eq!(*move_.metadata(), 5);
    }
}
