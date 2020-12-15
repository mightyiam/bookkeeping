use crate::account::Account;
use crate::balance::Balance;
use crate::move_::Move;
use crate::sum::Sum;
use crate::transaction::Transaction;
use crate::unit::Unit;
use duplicate::duplicate;
use slotmap::{new_key_type, DenseSlotMap};
use std::ops;
new_key_type! {
    /// A key type for referencing accounts.
    pub struct AccountKey;
    /// A key type for referencing units.
    pub struct UnitKey;
}
/// Represents a book.
///
/// Where
/// - `B`: book metadata
/// - `A`: account metadata
/// - `U`: unit metadata
/// - `M`: move metadata
/// - `T`: transaction metadata
pub struct Book<B, A, U, M, T> {
    metadata: B,
    accounts: DenseSlotMap<AccountKey, Account<A>>,
    units: DenseSlotMap<UnitKey, Unit<U>>,
    transactions: Vec<Transaction<M, T>>,
}
impl<B, A, U, M, T> Book<B, A, U, M, T> {
    /// Creates a new book
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// let _book = Book::<&str, &str, &str, &str, &str>::new("some book");
    /// ```
    pub fn new(metadata: B) -> Self {
        Self {
            metadata,
            accounts: DenseSlotMap::with_key(),
            units: DenseSlotMap::with_key(),
            transactions: Vec::new(),
        }
    }
    /// Gets the book's metadata.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let book = Book::<&str, &str, &str, &str, &str>::new("some book");
    /// assert_eq!(*book.metadata(), "some book");
    /// ```
    pub fn metadata(&self) -> &B {
        &self.metadata
    }
    /// Sets the book's metadata.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("some booc");
    /// book.set_book_metadata("some book");
    /// ```
    pub fn set_book_metadata(&mut self, metadata: B) {
        self.metadata = metadata;
    }
    /// Creates a new account.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// let _wallet_key = book.new_account("wallet");
    /// let _bank_key = book.new_account("bank");
    /// ```
    pub fn new_account(&mut self, metadata: A) -> AccountKey {
        self.accounts.insert(Account { metadata })
    }
    /// Creates a new unit.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// let _usd_key = book.new_unit("USD");
    /// let _thb_key = book.new_unit("THB");
    /// let _ils_key = book.new_unit("ILS");
    /// ```
    pub fn new_unit(&mut self, metadata: U) -> UnitKey {
        self.units.insert(Unit { metadata })
    }
    /// Creates a transaction and inserts it at an index.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// book.insert_transaction(0, "deposit");
    /// ```
    pub fn insert_transaction(
        &mut self,
        transaction_index: usize,
        metadata: T,
    ) {
        self.transactions.insert(
            transaction_index,
            Transaction {
                metadata,
                moves: Vec::new(),
            },
        )
    }
    fn assert_has_units_of_sum(&self, sum: &Sum) {
        sum.0.keys().for_each(|unit_key| {
            self.assert_has_unit(*unit_key);
        });
    }
    /// Creates a new move and inserts it into a transaction at an index.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    /// - Some of `debit_account_key` and `credit_account_key` are not in the book.
    /// - `debit_account_key` and `credit_account_key` are equal.
    /// - Some unit keys that are in the sum are not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let usd_key = book.new_unit("USD");
    /// let mut sum = Sum::new();
    /// sum.set_amount_for_unit(800, usd_key);
    /// book.insert_transaction(0, "");
    /// book.insert_move(0, 0, bank_key, wallet_key, sum, "withdrawal");
    /// ```
    pub fn insert_move(
        &mut self,
        transaction_index: usize,
        move_index: usize,
        debit_account_key: AccountKey,
        credit_account_key: AccountKey,
        sum: Sum,
        metadata: M,
    ) {
        [debit_account_key, credit_account_key].iter().for_each(
            |account_key| {
                self.assert_has_account(*account_key);
            },
        );
        self.assert_has_units_of_sum(&sum);
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum, metadata);
        let transaction = std::ops::IndexMut::index_mut(
            &mut self.transactions,
            transaction_index,
        );
        transaction.moves.insert(move_index, move_);
    }
    /// Gets an account using a key.
    ///
    /// ## Panics
    ///
    /// - No such account in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// let _wallet = book.get_account(wallet_key);
    /// ```
    pub fn get_account(&self, account_key: AccountKey) -> &Account<A> {
        self.assert_has_account(account_key);
        self.accounts.get(account_key).unwrap()
    }
    /// Gets a unit using a key.
    ///
    /// ## Panics
    ///
    /// - No such unit in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("USD");
    /// let _usd = book.get_unit(usd_key);
    /// ```
    pub fn get_unit(&self, unit_key: UnitKey) -> &Unit<U> {
        self.assert_has_unit(unit_key);
        self.units.get(unit_key).unwrap()
    }
    /// Gets an iterator of existing accounts in order of creation.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let wallet = book.get_account(wallet_key);
    /// # let bank = book.get_account(bank_key);
    /// assert_eq!(
    ///     book.accounts().collect::<Vec<_>>(),
    ///     vec![(wallet_key, wallet), (bank_key, bank)],
    /// );
    /// ```
    pub fn accounts(&self) -> impl Iterator<Item = (AccountKey, &Account<A>)> {
        self.accounts.iter()
    }
    /// Gets an iterator of existing units in order of creation.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("USD");
    /// # let thb_key = book.new_unit("THB");
    /// # let usd = book.get_unit(usd_key);
    /// # let thb = book.get_unit(thb_key);
    /// assert_eq!(
    ///     book.units().collect::<Vec<_>>(),
    ///     vec![(usd_key, usd), (thb_key, thb)],
    /// );
    /// ```
    pub fn units(&self) -> impl Iterator<Item = (UnitKey, &Unit<U>)> {
        self.units.iter()
    }
    /// Gets an iterator of existing transactions in their order.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// book.insert_transaction(0, "deposit");
    /// book.insert_transaction(1, "withdrawal");
    /// assert_eq!(
    ///     book.transactions()
    ///         .map(|transaction| transaction.metadata())
    ///         .collect::<Vec<_>>(),
    ///     vec![&"deposit", &"withdrawal"],
    /// );
    /// ```
    pub fn transactions(&self) -> impl Iterator<Item = &Transaction<M, T>> {
        self.transactions.iter()
    }
    /// Sets the metadata for an account.
    ///
    /// ## Panics
    /// - The account is not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let bank_key = book.new_account("banc");
    /// book.set_account_metadata(bank_key, "bank");
    /// ```
    pub fn set_account_metadata(
        &mut self,
        account_key: AccountKey,
        metadata: A,
    ) {
        self.assert_has_account(account_key);
        self.accounts.get_mut(account_key).unwrap().metadata = metadata;
    }
    /// Sets the metadata for a unit.
    ///
    /// ## Panics
    /// - The unit is not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("USd");
    /// book.set_unit_metadata(usd_key, "USD");
    /// ```
    pub fn set_unit_metadata(&mut self, unit_key: UnitKey, metadata: U) {
        self.assert_has_unit(unit_key);
        self.units.get_mut(unit_key).unwrap().metadata = metadata;
    }
    /// Sets the metadata for a transaction.
    ///
    /// ## Panics
    /// - `transaction_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # book.insert_transaction(0, "withdrawa");
    /// book.set_transaction_metadata(0, "withdrawal");
    /// ```
    pub fn set_transaction_metadata(
        &mut self,
        transaction_index: usize,
        metadata: T,
    ) {
        self.transactions
            .get_mut(transaction_index)
            .unwrap()
            .metadata = metadata;
    }
    /// Sets the metadata for a move.
    ///
    /// ## Panics
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # book.insert_transaction(0, "");
    /// # book.insert_move(0, 0, bank_key, wallet_key, Sum::new(), "withdrawa");
    /// book.set_move_metadata(0, 0, "withdrawal");
    /// ```
    pub fn set_move_metadata(
        &mut self,
        transaction_index: usize,
        move_index: usize,
        metadata: M,
    ) {
        let transaction = std::ops::IndexMut::index_mut(
            &mut self.transactions,
            transaction_index,
        );
        let move_ = &mut transaction.moves[move_index];
        move_.metadata = metadata;
    }
    /// Calculates the balance of an account at a provided transaction.
    ///
    /// Providing an out of bounds `transaction_index` is undefined behavior.
    ///
    /// ## Panics
    ///
    /// - The account is not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let usd_key = book.new_unit("USD");
    /// # let mut sum = Sum::new();
    /// # sum.set_amount_for_unit(800, usd_key);
    /// # book.insert_transaction(0, "");
    /// # book.insert_move(0, 0, bank_key, wallet_key, sum, "withdrawal");
    /// let _balance = book.account_balance_at_transaction(wallet_key, 0);
    /// ```
    pub fn account_balance_at_transaction<'a>(
        &'a self,
        account_key: AccountKey,
        transaction_index: usize,
    ) -> Balance<'a> {
        self.assert_has_account(account_key);
        self.transactions
            .iter()
            .take(transaction_index + 1)
            .flat_map(|transaction| transaction.moves.iter())
            .filter_map(|move_| -> Option<(fn(&mut Balance<'a>, _), &Sum)> {
                if move_.debit_account_key == account_key {
                    Some((ops::SubAssign::sub_assign, &move_.sum))
                } else if move_.credit_account_key == account_key {
                    Some((ops::AddAssign::add_assign, &move_.sum))
                } else {
                    None
                }
            })
            .fold(Balance::new(), |mut balance, (operation, sum)| {
                operation(&mut balance, sum);
                balance
            })
    }
    /// Removes an existing transaction from the book.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # book.insert_transaction(0, "");
    /// book.remove_transaction(0);
    /// ```
    pub fn remove_transaction(&mut self, transaction_index: usize) {
        self.transactions.remove(transaction_index);
    }
    /// Removes an existing move from the book.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # book.insert_transaction(0, "");
    /// # let wallet_key = book.new_account("");
    /// # let bank_key = book.new_account("");
    /// # book.insert_move(0, 0, wallet_key, bank_key, Sum::new(), "");
    /// book.remove_move(0, 0);
    /// ```
    pub fn remove_move(&mut self, transaction_index: usize, move_index: usize) {
        self.transactions[transaction_index]
            .moves
            .remove(move_index);
    }
    /// Sets the debit account of an existing move.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # book.insert_transaction(0, "");
    /// # let wallet_key = book.new_account("");
    /// # let bank_key = book.new_account("");
    /// # book.insert_move(0, 0, wallet_key, bank_key, Sum::new(), "");
    /// book.set_move_sum(0, 0, Sum::new());
    /// ```
    pub fn set_move_sum(
        &mut self,
        transaction_index: usize,
        move_index: usize,
        sum: Sum,
    ) {
        self.assert_has_units_of_sum(&sum);
        self.transactions[transaction_index].moves[move_index].sum = sum;
    }
    /// Sets the account for the debit side of an existing move.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # book.insert_transaction(0, "");
    /// # let safe_key = book.new_account("");
    /// # let wallet_key = book.new_account("");
    /// # book.insert_move(0, 0, safe_key, wallet_key, Sum::new(), "");
    /// # let bank_key = book.new_account("");
    /// book.set_move_debit_account(0, 0, bank_key);
    /// ```
    pub fn set_move_debit_account(
        &mut self,
        transaction_index: usize,
        move_index: usize,
        debit_account_key: AccountKey,
    ) {
        self.assert_has_account(debit_account_key);
        self.transactions[transaction_index].moves[move_index]
            .debit_account_key = debit_account_key;
    }
    /// Sets the account for the credit side of an existing move.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str, &str>::new("");
    /// # book.insert_transaction(0, "");
    /// # let wallet_key = book.new_account("");
    /// # let safe_key = book.new_account("");
    /// # book.insert_move(0, 0, wallet_key, safe_key, Sum::new(), "");
    /// # let bank_key = book.new_account("");
    /// book.set_move_credit_account(0, 0, bank_key);
    /// ```
    pub fn set_move_credit_account(
        &mut self,
        transaction_index: usize,
        move_index: usize,
        credit_account_key: AccountKey,
    ) {
        self.assert_has_account(credit_account_key);
        self.transactions[transaction_index].moves[move_index]
            .credit_account_key = credit_account_key;
    }
}
#[duplicate(
    assert_has               Key              plural         string         ;
    [assert_has_account]     [AccountKey]     [accounts]     ["account"]    ;
    [assert_has_unit]        [UnitKey]        [units]        ["unit"]       ;
)]
impl<B, A, U, M, T> Book<B, A, U, M, T> {
    fn assert_has(&self, key: Key) {
        assert!(
            self.plural.contains_key(key),
            format!("No {} found for key {:?}", string, key),
        );
    }
}
#[cfg(test)]
mod test {
    use super::Balance;
    use duplicate::duplicate_inline;
    #[test]
    fn new() {
        let book = test_book!("");
        assert_eq!(book.metadata, "");
        assert!(book.accounts.is_empty());
        assert!(book.units.is_empty());
        assert!(book.transactions.is_empty());
    }
    #[test]
    fn new_account() {
        let mut book = test_book!("");
        book.new_account("");
        assert_eq!(book.accounts.len(), 1);
    }
    #[test]
    fn new_unit() {
        let mut book = test_book!("");
        book.new_unit("");
        assert_eq!(book.units.len(), 1,);
    }
    #[test]
    #[should_panic(expected = "insertion index (is 1) should be <= len (is 0)")]
    fn insert_transaction_panic_index_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(1, "");
    }
    #[test]
    fn insert_transaction() {
        let mut book = test_book!("");
        book.insert_transaction(0, "a");
        book.insert_transaction(1, "b");
        book.insert_transaction(0, "c");
        book.insert_transaction(2, "d");
        assert_eq!(
            book.transactions
                .iter()
                .map(|transaction| transaction.metadata())
                .collect::<Vec<_>>(),
            [&"c", &"a", &"d", &"b"],
        );
    }
    #[test]
    #[should_panic(expected = "insertion index (is 1) should be <= len (is 0)")]
    fn insert_move_panic_index_out_of_bounds() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.insert_transaction(0, "");
        book.insert_move(0, 1, debit_key, credit_key, sum!(), "");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn insert_move_panic_debit_account_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        book.accounts.remove(debit_key);
        let credit_key = book.new_account("");
        book.insert_transaction(0, "");
        book.insert_move(0, 0, debit_key, credit_key, sum!(), "");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn insert_move_panic_credit_account_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.accounts.remove(credit_key);
        book.insert_transaction(0, "");
        book.insert_move(0, 0, debit_key, credit_key, sum!(), "");
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn insert_move_panic_unit_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        let sum = sum!(0, unit_key);
        book.insert_transaction(0, "");
        book.insert_move(0, 0, debit_key, credit_key, sum, "");
    }
    #[test]
    fn insert_move() {
        let mut book = test_book!("");
        book.insert_transaction(0, "");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.insert_move(0, 0, debit_key, credit_key, sum!(), "a");
        book.insert_move(0, 0, debit_key, credit_key, sum!(), "b");
        book.insert_move(0, 1, debit_key, credit_key, sum!(), "c");
        book.insert_move(0, 2, debit_key, credit_key, sum!(), "d");
        assert_eq!(
            book.transactions[0]
                .moves
                .iter()
                .map(|move_| move_.metadata)
                .collect::<Vec<_>>(),
            vec!["b", "c", "d", "a"],
        );
    }
    #[test]
    fn accounts() {
        let mut book = test_book!("");
        assert!(book.accounts().next().is_none());
        let account_key = book.new_account("");
        let account = book.accounts.get(account_key).unwrap();
        let expected = vec![(account_key, account)];
        let actual = book.accounts().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn units() {
        let mut book = test_book!("");
        assert!(book.units().next().is_none());
        let unit_key = book.new_unit("");
        let unit = book.units.get(unit_key).unwrap();
        let expected = vec![(unit_key, unit)];
        let actual = book.units().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn get_account() {
        let mut book = test_book!("");
        book.new_account("");
        let account_key = book.new_account("!");
        book.new_account("");
        let account = book.get_account(account_key);
        assert_eq!(*account.metadata(), "!");
    }
    #[test]
    fn get_unit() {
        let mut book = test_book!("");
        book.new_unit("");
        let unit_key = book.new_unit("!");
        book.new_unit("");
        let unit = book.get_unit(unit_key);
        assert_eq!(*unit.metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn assert_has_account() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.assert_has_account(account_key);
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn assert_has_unit() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        book.assert_has_unit(unit_key);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn account_balance_at_transaction_account_not_found() {
        let mut book = test_book!("");
        book.insert_transaction(0, "");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.account_balance_at_transaction(account_key, 0);
    }
    #[test]
    fn account_balance_at_transaction() {
        let mut book = test_book!("");
        let account_a_key = book.new_account("");
        let account_b_key = book.new_account("");
        let unit_key = book.new_unit("");
        book.insert_transaction(0, "");
        book.insert_move(
            0,
            0,
            account_a_key,
            account_b_key,
            sum!(3, unit_key),
            "",
        );
        assert_eq!(
            book.account_balance_at_transaction(account_a_key, 0),
            Balance::new() - &sum!(3, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_b_key, 0),
            Balance::new() + &sum!(3, unit_key),
        );
        book.insert_transaction(1, "");
        book.insert_move(
            1,
            0,
            account_a_key,
            account_b_key,
            sum!(4, unit_key),
            "",
        );
        assert_eq!(
            book.account_balance_at_transaction(account_a_key, 0),
            Balance::new() - &sum!(3, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_b_key, 0),
            Balance::new() + &sum!(3, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_a_key, 1),
            Balance::new() - &sum!(7, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_b_key, 1),
            Balance::new() + &sum!(7, unit_key),
        );
        book.insert_transaction(0, "");
        book.insert_move(
            0,
            0,
            account_a_key,
            account_b_key,
            sum!(1, unit_key),
            "",
        );
        assert_eq!(
            book.account_balance_at_transaction(account_a_key, 0),
            Balance::new() - &sum!(1, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_b_key, 0),
            Balance::new() + &sum!(1, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_a_key, 1),
            Balance::new() - &sum!(4, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_b_key, 1),
            Balance::new() + &sum!(4, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_a_key, 2),
            Balance::new() - &sum!(8, unit_key),
        );
        assert_eq!(
            book.account_balance_at_transaction(account_b_key, 2),
            Balance::new() + &sum!(8, unit_key),
        );
    }
    #[test]
    fn set_book_metadata() {
        let mut book = test_book!("");
        book.set_book_metadata("!");
        assert_eq!(book.metadata, "!");
    }
    #[test]
    fn metadata() {
        let book = test_book!("!");
        assert_eq!(*book.metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn set_account_metadata_panic() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.set_account_metadata(account_key, "!");
    }
    #[test]
    fn set_account_metadata() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.set_account_metadata(account_key, "!");
        assert_eq!(*book.accounts.get(account_key).unwrap().metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn set_unit_metadata_panic() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        book.set_unit_metadata(unit_key, "!");
    }
    #[test]
    fn set_unit_metadata() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        book.set_unit_metadata(unit_key, "!");
        assert_eq!(*book.units.get(unit_key).unwrap().metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn set_move_metadata_panic_transaction_index_out_of_bounds() {
        let mut book = test_book!("");
        book.set_move_metadata(0, 0, "");
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn set_move_metadata_panic_move_index_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(0, "");
        book.set_move_metadata(0, 1, "");
    }
    #[test]
    fn set_move_metadata() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.insert_transaction(0, "");
        book.insert_move(0, 0, debit_key, credit_key, sum!(), "");
        book.set_move_metadata(0, 0, "!");
        assert_eq!(*book.transactions[0].moves[0].metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn remove_transaction_panic_out_of_bounds() {
        let mut book = test_book!("");
        book.remove_transaction(0);
    }
    #[test]
    fn remove_transaction() {
        let mut book = test_book!("");
        book.insert_transaction(0, "a");
        book.insert_transaction(1, "b");
        book.remove_transaction(1);
        assert_eq!(&book.transactions[0].metadata, &"a");
        book.remove_transaction(0);
        assert!(book.transactions.is_empty());
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn remove_move_panic_transaction_index_out_of_bounds() {
        let mut book = test_book!("");
        book.remove_move(0, 0);
    }
    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn remove_move_panic_move_index_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(0, "");
        book.remove_move(0, 0);
    }
    #[test]
    fn remove_move() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(0, "");
        book.insert_move(
            0,
            0,
            debit_account_key,
            credit_account_key,
            sum!(),
            "a",
        );
        book.insert_move(
            0,
            1,
            debit_account_key,
            credit_account_key,
            sum!(),
            "b",
        );
        book.remove_move(0, 1);
        assert_eq!(&book.transactions[0].moves[0].metadata, &"a");
        book.remove_move(0, 0);
        assert!(book.transactions[0].moves.is_empty());
    }
    duplicate_inline! {
        [
            method                    field               ;
            [set_move_debit_account]  [debit_account_key] ;
            [set_move_credit_account] [credit_account_key];
        ]
        #[cfg(test)]
        mod method {
            #[test]
            #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
            fn panic_transaction_out_of_bounds() {
                let mut book = test_book!("");
                let account_key = book.new_account("");
                book.method(0, 0, account_key);
            }
            #[test]
            #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
            fn panic_move_out_of_bounds() {
                let mut book = test_book!("");
                let account_key = book.new_account("");
                book.insert_transaction(0, "");
                book.method(0, 0, account_key);
            }
            #[test]
            #[should_panic(expected = "No account found for key ")]
            fn panic_account_not_found() {
                let mut book = test_book!("");
                let debit_account_key = book.new_account("");
                let credit_account_key = book.new_account("");
                book.insert_transaction(0, "");
                book.insert_move(
                    0,
                    0,
                    debit_account_key,
                    credit_account_key,
                    sum!(),
                    "",
                );
                let other_account_key = book.new_account("");
                book.accounts.remove(other_account_key);
                book.method(0, 0, other_account_key);
            }
            #[test]
            fn method() {
                let mut book = test_book!("");
                let debit_account_key = book.new_account("");
                let credit_account_key = book.new_account("");
                book.insert_transaction(0, "");
                book.insert_move(
                    0,
                    0,
                    debit_account_key,
                    credit_account_key,
                    sum!(),
                    "",
                );
                let other_account_key = book.new_account("");
                book.method(0, 0, other_account_key);
                assert_eq!(
                    book.transactions[0].moves[0].field,
                    other_account_key
                );
            }
        }
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_sum_panic_transaction_out_of_bounds() {
        let mut book = test_book!("");
        book.set_move_sum(0, 0, sum!());
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_sum_panic_move_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(0, "");
        book.set_move_sum(0, 0, sum!());
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn set_move_sum_panic_unit_not_found() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(0, "");
        book.insert_move(
            0,
            0,
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        book.set_move_sum(0, 0, sum!(100, unit_key));
    }
    #[test]
    fn set_move_sum() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(0, "");
        book.insert_move(
            0,
            0,
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        let unit_key = book.new_unit("");
        book.set_move_sum(0, 0, sum!(100, unit_key));
        assert_eq!(
            book.transactions[0].moves[0].sum.0.get(&unit_key).unwrap(),
            &100,
        );
    }
}
