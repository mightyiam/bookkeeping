use crate::book::AccountKey;
use crate::sum::Sum;
use crate::unit::Unit;
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
pub struct Move<U: Unit, Sn, M> {
    pub(crate) metadata: M,
    pub(crate) debit_account_key: AccountKey,
    pub(crate) credit_account_key: AccountKey,
    pub(crate) sum: Sum<U, Sn>,
}
impl<U: Unit, Sn, M> Move<U, Sn, M> {
    pub(crate) fn new(
        debit_account_key: AccountKey,
        credit_account_key: AccountKey,
        sum: Sum<U, Sn>,
        metadata: M,
    ) -> Self {
        assert!(
            debit_account_key != credit_account_key,
            "Debit and credit accounts are the same."
        );
        Self {
            metadata,
            debit_account_key,
            credit_account_key,
            sum,
        }
    }
    /// Gets the debit account key of a move.
    pub fn debit_account_key(&self) -> AccountKey {
        self.debit_account_key
    }
    /// Gets the credit account key of a move.
    pub fn credit_account_key(&self) -> AccountKey {
        self.credit_account_key
    }
    /// Gets the sum of a move.
    pub fn sum(&self) -> &Sum<U, Sn> {
        &self.sum
    }
    /// Gets the metadata of the move.
    pub fn metadata(&self) -> &M {
        &self.metadata
    }
}
#[cfg(test)]
mod test {
    use super::Move;
    use crate::unit::TestUnit;
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn new_panic_debit_and_credit_accounts_are_the_same() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        Move::new(account_key, account_key, sum!(), ());
    }
    #[test]
    fn new() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let sum = sum!();
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum.clone(), ());
        assert_eq!(move_.debit_account_key, debit_account_key);
        assert_eq!(move_.credit_account_key, credit_account_key);
        assert_eq!(move_.sum, sum);
    }
    #[test]
    fn debit_account() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum!(), "");
        assert_eq!(move_.debit_account_key(), debit_account_key);
    }
    #[test]
    fn credit_account() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum!(), "");
        assert_eq!(move_.credit_account_key(), credit_account_key);
    }
    #[test]
    fn sum() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let thb = TestUnit("THB");
        let ils = TestUnit("ILS");
        let sum = sum!(100, thb; 200, ils);
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum.clone(), "");
        assert_eq!(*move_.sum(), sum);
    }
    #[test]
    fn metadata() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_ = Move::new(debit_account_key, credit_account_key, sum!(), 5);
        assert_eq!(*move_.metadata(), 5);
    }
}
