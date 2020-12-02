use crate::book::AccountKey;
use crate::sum::Sum;
/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
#[derive(Debug, PartialEq)]
pub struct Account<A> {
    pub(crate) meta: A,
}
impl<A> Account<A> {
    pub(crate) fn new(meta: A) -> Self {
        Self { meta }
    }
    /// Gets the metadata of the account.
    ///
    /// ```
    /// # use bookkeeping::Book;
    /// # use chrono::naive::NaiveDate;
    /// # struct BookMetadata { id: u8 }
    /// # #[derive(Debug, PartialEq)]
    /// # struct AccountMetadata { name: String }
    /// # struct UnitMetadata { currency_code: String }
    /// # struct MoveMetadata { date: NaiveDate }
    /// # let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    /// #     BookMetadata { id: 0 },
    /// # );
    /// # let account_key = book.new_account(AccountMetadata { name: String::from("Wallet") });
    /// # let account = book.get_account(account_key);
    /// assert_eq!(
    ///     account.metadata(),
    ///     &AccountMetadata { name: String::from("Wallet") },
    /// );
    /// ```
    pub fn metadata(&self) -> &A {
        &self.meta
    }
}
/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit<U> {
    pub(crate) meta: U,
}
impl<U> Unit<U> {
    /// Creates a new unit.
    pub(crate) fn new(meta: U) -> Self {
        Self { meta }
    }
    /// Gets the metadata of the unit.
    ///
    /// ```
    /// # use bookkeeping::Book;
    /// # use chrono::naive::NaiveDate;
    /// # struct BookMetadata { id: u8 }
    /// # struct AccountMetadata { name: String }
    /// # #[derive(Debug, PartialEq)]
    /// # struct UnitMetadata { currency_code: String }
    /// # struct MoveMetadata { date: NaiveDate }
    /// # let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    /// #     BookMetadata { id: 0 },
    /// # );
    /// # let unit_key = book.new_unit(UnitMetadata { currency_code: String::from("USD") });
    /// # let unit = book.get_unit(unit_key);
    /// assert_eq!(
    ///     unit.metadata(),
    ///     &UnitMetadata { currency_code: String::from("USD") },
    /// );
    /// ```
    pub fn metadata(&self) -> &U {
        &self.meta
    }
}
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
    /// # let move_key = book.new_move(bank, wallet, Sum::new(), MoveMetadata { date: NaiveDate::from_ymd(2020, 12, 1) });
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
    use super::Account;
    use super::AccountKey;
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
        let account_key = DenseSlotMap::<AccountKey, ()>::with_key().insert(());
        Move::new(account_key, account_key, Sum::new(), ());
    }
    #[test]
    fn move_new() {
        let mut slot_map = DenseSlotMap::<AccountKey, ()>::with_key();
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
        let mut slot_map = DenseSlotMap::<AccountKey, ()>::with_key();
        let debit_account = slot_map.insert(());
        let credit_account = slot_map.insert(());
        let sum = Sum::new();
        let move_ = Move::new(debit_account, credit_account, sum.clone(), 5);
        assert_eq!(*move_.metadata(), 5);
    }
}
