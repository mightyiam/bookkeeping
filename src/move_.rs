use crate::{book::AccountKey, sum::Sum};
/// Represents a side of a [Move].
pub enum Side {
    #[allow(missing_docs)]
    Debit,
    #[allow(missing_docs)]
    Credit,
}
/// Represents a move of a [Sum] from one account to another.
pub struct Move<Unit, Number, Extra>
where
    Unit: Ord,
{
    pub(crate) extra: Extra,
    pub(crate) debit_account_key: AccountKey,
    pub(crate) credit_account_key: AccountKey,
    pub(crate) sum: Sum<Unit, Number>,
}
impl<Unit, Number, Extra> Move<Unit, Number, Extra>
where
    Unit: Ord,
{
    pub(crate) fn new(
        debit_account_key: AccountKey,
        credit_account_key: AccountKey,
        sum: Sum<Unit, Number>,
        extra: Extra,
    ) -> Self {
        assert!(
            debit_account_key != credit_account_key,
            "Debit and credit accounts are the same."
        );
        Self {
            extra,
            debit_account_key,
            credit_account_key,
            sum,
        }
    }
    /// Gets the account key of one of the sides of a move.
    pub fn side_key(&self, side: Side) -> AccountKey {
        match side {
            Side::Debit => self.debit_account_key,
            Side::Credit => self.credit_account_key,
        }
    }
    /// Gets the sum of a move.
    pub fn sum(&self) -> &Sum<Unit, Number> {
        &self.sum
    }
    /// Gets the extra data of the move.
    pub fn extra(&self) -> &Extra {
        &self.extra
    }
}
#[cfg(test)]
mod test {
    use super::{Move, Side};
    use crate::test_utils::TestBook;
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn new_panic_debit_and_credit_accounts_are_the_same() {
        let mut book = TestBook::default();
        let account_key = book.insert_account("");
        Move::new(account_key, account_key, sum!(), ());
    }
    #[test]
    fn new() {
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
        let sum = sum!();
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum.clone(), ());
        assert_eq!(move_.debit_account_key, debit_account_key);
        assert_eq!(move_.credit_account_key, credit_account_key);
        assert_eq!(move_.sum, sum);
    }
    #[test]
    fn side() {
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum!(), "");
        assert_eq!(move_.side_key(Side::Debit), debit_account_key);
        assert_eq!(move_.side_key(Side::Credit), credit_account_key);
    }
    #[test]
    fn sum() {
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
        let thb = "THB";
        let ils = "ILS";
        let sum = sum!(100, thb; 200, ils);
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum.clone(), "");
        assert_eq!(*move_.sum(), sum);
    }
    #[test]
    fn extra() {
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
        let move_ = Move::new(debit_account_key, credit_account_key, sum!(), 5);
        assert_eq!(*move_.extra(), 5);
    }
}
