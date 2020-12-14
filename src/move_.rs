use crate::book::AccountKey;
use crate::sum::Sum;
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
#[derive(Debug, PartialEq)]
pub struct Move<M> {
    pub(crate) metadata: M,
    pub(crate) debit_account_key: AccountKey,
    pub(crate) credit_account_key: AccountKey,
    pub(crate) sum: Sum,
}
impl<M> Move<M> {
    pub(crate) fn new(
        debit_account_key: AccountKey,
        credit_account_key: AccountKey,
        sum: Sum,
        metadata: M,
    ) -> Self {
        assert!(
            debit_account_key != credit_account_key,
            "Debit and credit accounts are the same."
        );
        Self {
            metadata,
            debit_account_key: debit_account_key,
            credit_account_key: credit_account_key,
            sum,
        }
    }
    /// Gets the debit account key of a move.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # book.insert_transaction(0, "");
    /// book.insert_move(0, 0, wallet_key, bank_key, Sum::new(), "deposit");
    /// let move_ = book.transactions().nth(0).unwrap().moves().nth(0).unwrap();
    /// assert_eq!(move_.debit_account_key(), wallet_key);
    /// ```
    pub fn debit_account_key(&self) -> AccountKey {
        self.debit_account_key
    }
    /// Gets the credit account key of a move.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// let bank_key = book.new_account("bank");
    /// # book.insert_transaction(0, "");
    /// book.insert_move(0, 0, wallet_key, bank_key, Sum::new(), "deposit");
    /// let move_ = book.transactions().nth(0).unwrap().moves().nth(0).unwrap();
    /// assert_eq!(move_.credit_account_key(), bank_key);
    /// ```
    pub fn credit_account_key(&self) -> AccountKey {
        self.credit_account_key
    }
    /// Gets the sum of a move.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let usd_key = book.new_unit("USD");
    /// book.insert_transaction(0, "");
    /// let mut sum = Sum::new();
    /// sum.set_amount_for_unit(100, usd_key);
    /// book.insert_move(0, 0, wallet_key, bank_key, sum.clone(), "deposit");
    /// let move_ = book.transactions().nth(0).unwrap().moves().nth(0).unwrap();
    /// assert_eq!(*move_.sum(), sum);
    /// ```
    pub fn sum(&self) -> &Sum {
        &self.sum
    }
    /// Gets the metadata of the move.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # book.insert_transaction(0, "");
    /// # book.insert_move(0, 0, wallet_key, bank_key, Sum::new(), "deposit");
    /// # let move_ = book.transactions().nth(0).unwrap().moves().nth(0).unwrap();
    /// assert_eq!(*move_.metadata(), "deposit");
    /// ```
    pub fn metadata(&self) -> &M {
        &self.metadata
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
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let sum = Sum::new();
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
            Move::new(debit_account_key, credit_account_key, Sum::new(), "");
        assert_eq!(move_.debit_account_key(), debit_account_key);
    }
    #[test]
    fn credit_account() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_ =
            Move::new(debit_account_key, credit_account_key, Sum::new(), "");
        assert_eq!(move_.credit_account_key(), credit_account_key);
    }
    #[test]
    fn sum() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let thb_key = book.new_unit("");
        let ils_key = book.new_unit("");
        let sum = sum!(100, thb_key; 200, ils_key);
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum.clone(), "");
        assert_eq!(*move_.sum(), sum);
    }
    #[test]
    fn metadata() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let sum = Sum::new();
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum.clone(), 5);
        assert_eq!(*move_.metadata(), 5);
    }
}
