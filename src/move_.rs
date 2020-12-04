use crate::book::AccountKey;
use crate::sum::Sum;
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
#[derive(Debug, PartialEq)]
pub struct Move<M> {
    pub(crate) meta: M,
    pub(crate) debit_account: AccountKey,
    pub(crate) credit_account: AccountKey,
    pub(crate) sum: Sum,
}
impl<M> Move<M> {
    pub(crate) fn new(
        debit_account: AccountKey,
        credit_account: AccountKey,
        sum: Sum,
        meta: M,
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
    /// Gets the metadata of the move.
    ///
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # use chrono::naive::NaiveDate;
    /// # struct BookMetadata { id: u8 }
    /// # struct AccountMetadata { name: String }
    /// # struct UnitMetadata { currency_code: String }
    /// # #[derive(Debug, PartialEq)]
    /// # struct MoveMetadata { date: NaiveDate }
    /// # let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    /// #     BookMetadata { id: 0 },
    /// # );
    /// # let wallet = book.new_account(AccountMetadata { name: String::from("Wallet") });
    /// # let bank = book.new_account(AccountMetadata { name: String::from("Bank") });
    /// # let usd = book.new_unit(UnitMetadata { currency_code: String::from("USD") });
    /// # let move_key = book.insert_move(0, bank, wallet, Sum::new(), MoveMetadata { date: NaiveDate::from_ymd(2020, 12, 1) });
    /// # let move_ = book.get_move(move_key);
    /// assert_eq!(
    ///     move_.metadata(),
    ///     &MoveMetadata { date: NaiveDate::from_ymd(2020, 12, 1) },
    /// );
    /// ```
    pub fn metadata(&self) -> &M {
        &self.meta
    }
}
#[cfg(test)]
mod test {
    use super::Move;
    use super::Sum;
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn new_panic_debit_and_credit_accounts_are_the_same() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        Move::new(account_key, account_key, Sum::new(), ());
    }
    #[test]
    fn new() {
        let mut book = test_book!("");
        let debit_account = book.new_account("");
        let credit_account = book.new_account("");
        let sum = Sum::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), ());
        assert_eq!(move_.debit_account, debit_account);
        assert_eq!(move_.credit_account, credit_account);
        assert_eq!(move_.sum, sum);
    }
    #[test]
    fn metadata() {
        let mut book = test_book!("");
        let debit_account = book.new_account("");
        let credit_account = book.new_account("");
        let sum = Sum::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), 5);
        assert_eq!(*move_.metadata(), 5);
    }
}
