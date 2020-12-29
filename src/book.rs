use crate::account::Account;
use crate::balance::Balance;
use crate::move_::Move;
use crate::sum::Sum;
use crate::transaction::{MoveIndex, Transaction};
use crate::unit::Unit;
use slotmap::{new_key_type, DenseSlotMap};
use std::ops;
new_key_type! {
    /// A key type for referencing accounts.
    pub struct AccountKey;
}
/// Represents a book.
///
/// - U: unit type
/// - B: Book metadata
/// - A: Account metadata
/// - M: Move metadata
/// - T: Transaction metadata
pub struct Book<U: Unit, B, A, M, T> {
    metadata: B,
    accounts: DenseSlotMap<AccountKey, Account<A>>,
    transactions: Vec<Transaction<U, M, T>>,
}
/// Represents a side of a [Move].
pub enum Side {
    #[allow(missing_docs)]
    Debit,
    #[allow(missing_docs)]
    Credit,
}

/// Used to index transactions in the book.
pub struct TransactionIndex(pub usize);
impl<U: Unit, B, A, M, T> Book<U, B, A, M, T> {
    /// Creates a new book
    pub fn new(metadata: B) -> Self {
        Self {
            metadata,
            accounts: DenseSlotMap::with_key(),
            transactions: Vec::new(),
        }
    }
    /// Gets the book's metadata.
    pub fn metadata(&self) -> &B {
        &self.metadata
    }
    /// Sets the book's metadata.
    pub fn set_book_metadata(&mut self, metadata: B) {
        self.metadata = metadata;
    }
    /// Creates a new account.
    pub fn new_account(&mut self, metadata: A) -> AccountKey {
        self.accounts.insert(Account { metadata })
    }
    /// Creates a transaction and inserts it at an index.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    pub fn insert_transaction(
        &mut self,
        transaction_index: TransactionIndex,
        metadata: T,
    ) {
        self.transactions.insert(
            transaction_index.0,
            Transaction {
                metadata,
                moves: Vec::new(),
            },
        )
    }
    /// Creates a new move and inserts it into a transaction at an index.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    /// - Some of `debit_account_key` and `credit_account_key` are not in the book.
    /// - `debit_account_key` and `credit_account_key` are equal.
    /// - Some unit keys in the `sum` are not in the book.
    pub fn insert_move(
        &mut self,
        transaction_index: TransactionIndex,
        move_index: MoveIndex,
        debit_account_key: AccountKey,
        credit_account_key: AccountKey,
        sum: Sum<U>,
        metadata: M,
    ) {
        [debit_account_key, credit_account_key].iter().for_each(
            |account_key| {
                self.assert_has_account(*account_key);
            },
        );
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum, metadata);
        let transaction = std::ops::IndexMut::index_mut(
            &mut self.transactions,
            transaction_index.0,
        );
        transaction.moves.insert(move_index.0, move_);
    }
    /// Gets an account using a key.
    ///
    /// ## Panics
    ///
    /// - `account_key` is not in the book.
    pub fn get_account(&self, account_key: AccountKey) -> &Account<A> {
        self.assert_has_account(account_key);
        self.accounts.get(account_key).unwrap()
    }
    /// Gets an iterator of existing accounts in order of creation.
    pub fn accounts(&self) -> impl Iterator<Item = (AccountKey, &Account<A>)> {
        self.accounts.iter()
    }
    /// Gets an iterator of existing transactions in their order.
    pub fn transactions(
        &self,
    ) -> impl Iterator<Item = (TransactionIndex, &Transaction<U, M, T>)> {
        self.transactions
            .iter()
            .enumerate()
            .map(|(index, transaction)| (TransactionIndex(index), transaction))
    }
    /// Sets the metadata for an account.
    ///
    /// ## Panics
    /// - `account_key` is not in the book.
    pub fn set_account_metadata(
        &mut self,
        account_key: AccountKey,
        metadata: A,
    ) {
        self.assert_has_account(account_key);
        self.accounts.get_mut(account_key).unwrap().metadata = metadata;
    }
    /// Sets the metadata for a transaction.
    ///
    /// ## Panics
    /// - `transaction_index` out of bounds.
    pub fn set_transaction_metadata(
        &mut self,
        transaction_index: TransactionIndex,
        metadata: T,
    ) {
        self.transactions
            .get_mut(transaction_index.0)
            .unwrap()
            .metadata = metadata;
    }
    /// Sets the metadata for a move.
    ///
    /// ## Panics
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    pub fn set_move_metadata(
        &mut self,
        transaction_index: TransactionIndex,
        move_index: MoveIndex,
        metadata: M,
    ) {
        let transaction = std::ops::IndexMut::index_mut(
            &mut self.transactions,
            transaction_index.0,
        );
        let move_ = &mut transaction.moves[move_index.0];
        move_.metadata = metadata;
    }
    /// Calculates the balance of an account at a provided transaction.
    ///
    /// Providing an out of bounds `transaction_index` is undefined behavior.
    ///
    /// ## Panics
    ///
    /// - `account_key` is not in the book.
    #[allow(clippy::type_complexity)]
    pub fn account_balance_at_transaction<'a>(
        &'a self,
        account_key: AccountKey,
        transaction_index: TransactionIndex,
    ) -> Balance<U> {
        self.assert_has_account(account_key);
        self.transactions
            .iter()
            .take(transaction_index.0 + 1)
            .flat_map(|transaction| transaction.moves.iter())
            .filter_map(
                |move_| -> Option<(fn(&mut Balance<U>, &'a Sum<U>), &Sum<U>)> {
                    if move_.debit_account_key == account_key {
                        Some((ops::SubAssign::sub_assign, &move_.sum))
                    } else if move_.credit_account_key == account_key {
                        Some((ops::AddAssign::add_assign, &move_.sum))
                    } else {
                        None
                    }
                },
            )
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
    pub fn remove_transaction(&mut self, transaction_index: TransactionIndex) {
        self.transactions.remove(transaction_index.0);
    }
    /// Removes an existing move from the book.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    pub fn remove_move(
        &mut self,
        transaction_index: TransactionIndex,
        move_index: MoveIndex,
    ) {
        self.transactions[transaction_index.0]
            .moves
            .remove(move_index.0);
    }
    /// Sets the sum of an existing move.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    pub fn set_move_sum(
        &mut self,
        transaction_index: TransactionIndex,
        move_index: MoveIndex,
        sum: Sum<U>,
    ) {
        self.transactions[transaction_index.0].moves[move_index.0].sum = sum;
    }
    /// Sets the account for one of the sides of an existing move.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    /// - `move_index` out of bounds.
    /// - `account_key` is not in the book.
    /// - `side` is same as other side.
    pub fn set_move_side(
        &mut self,
        transaction_index: TransactionIndex,
        move_index: MoveIndex,
        side: Side,
        account_key: AccountKey,
    ) {
        self.assert_has_account(account_key);
        let move_ =
            &mut self.transactions[transaction_index.0].moves[move_index.0];
        match side {
            Side::Debit => {
                assert_ne!(account_key, move_.credit_account_key, "Provided debit account is same as existing credit account.");
                move_.debit_account_key = account_key;
            }
            Side::Credit => {
                assert_ne!(account_key, move_.debit_account_key, "Provided credit account is same as existing debit account.");
                move_.credit_account_key = account_key;
            }
        }
    }
    fn assert_has_account(&self, key: AccountKey) {
        assert!(
            self.accounts.contains_key(key),
            format!("No account found for key {:?}", key),
        );
    }
}
#[cfg(test)]
mod test {
    use super::{
        Balance,
        Side::{Credit, Debit},
        TransactionIndex,
    };
    use crate::transaction::MoveIndex;
    use crate::unit::TestUnit;
    #[test]
    fn new() {
        let book = test_book!("");
        assert_eq!(book.metadata, "");
        assert!(book.accounts.is_empty());
        assert!(book.transactions.is_empty());
    }
    #[test]
    fn new_account() {
        let mut book = test_book!("");
        book.new_account("");
        assert_eq!(book.accounts.len(), 1);
    }
    #[test]
    #[should_panic(expected = "insertion index (is 1) should be <= len (is 0)")]
    fn insert_transaction_panic_index_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(1), "");
    }
    #[test]
    fn insert_transaction() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "a");
        book.insert_transaction(TransactionIndex(1), "b");
        book.insert_transaction(TransactionIndex(0), "c");
        book.insert_transaction(TransactionIndex(2), "d");
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
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(1),
            debit_key,
            credit_key,
            sum!(),
            "",
        );
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn insert_move_panic_debit_account_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        book.accounts.remove(debit_key);
        let credit_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_key,
            credit_key,
            sum!(),
            "",
        );
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn insert_move_panic_credit_account_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.accounts.remove(credit_key);
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_key,
            credit_key,
            sum!(),
            "",
        );
    }
    #[test]
    fn insert_move() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_key,
            credit_key,
            sum!(),
            "a",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_key,
            credit_key,
            sum!(),
            "b",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(1),
            debit_key,
            credit_key,
            sum!(),
            "c",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(2),
            debit_key,
            credit_key,
            sum!(),
            "d",
        );
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
        let account_a_key = book.new_account("a");
        let account_b_key = book.new_account("b");
        let expected = vec![(account_a_key, &"a"), (account_b_key, &"b")];
        let actual = book
            .accounts()
            .map(|(account_key, account)| (account_key, &account.metadata))
            .collect::<Vec<_>>();
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
    #[should_panic(expected = "No account found for key ")]
    fn assert_has_account() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.assert_has_account(account_key);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn account_balance_at_transaction_account_not_found() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.account_balance_at_transaction(account_key, TransactionIndex(0));
    }
    #[test]
    fn account_balance_at_transaction() {
        let mut book = test_book!("");
        let account_a_key = book.new_account("");
        let account_b_key = book.new_account("");
        let usd = TestUnit("USD");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            account_a_key,
            account_b_key,
            sum!(3, usd),
            "",
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_a_key,
                TransactionIndex(0)
            ),
            Balance::new() - &sum!(3, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_b_key,
                TransactionIndex(0)
            ),
            Balance::new() + &sum!(3, usd),
        );
        book.insert_transaction(TransactionIndex(1), "");
        book.insert_move(
            TransactionIndex(1),
            MoveIndex(0),
            account_a_key,
            account_b_key,
            sum!(4, usd),
            "",
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_a_key,
                TransactionIndex(0)
            ),
            Balance::new() - &sum!(3, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_b_key,
                TransactionIndex(0)
            ),
            Balance::new() + &sum!(3, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_a_key,
                TransactionIndex(1)
            ),
            Balance::new() - &sum!(7, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_b_key,
                TransactionIndex(1)
            ),
            Balance::new() + &sum!(7, usd),
        );
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            account_a_key,
            account_b_key,
            sum!(1, usd),
            "",
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_a_key,
                TransactionIndex(0)
            ),
            Balance::new() - &sum!(1, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_b_key,
                TransactionIndex(0)
            ),
            Balance::new() + &sum!(1, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_a_key,
                TransactionIndex(1)
            ),
            Balance::new() - &sum!(4, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_b_key,
                TransactionIndex(1)
            ),
            Balance::new() + &sum!(4, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_a_key,
                TransactionIndex(2)
            ),
            Balance::new() - &sum!(8, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction(
                account_b_key,
                TransactionIndex(2)
            ),
            Balance::new() + &sum!(8, usd),
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
    #[should_panic(expected = "index out of bounds")]
    fn set_move_metadata_panic_transaction_index_out_of_bounds() {
        let mut book = test_book!("");
        book.set_move_metadata(TransactionIndex(0), MoveIndex(0), "");
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn set_move_metadata_panic_move_index_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        book.set_move_metadata(TransactionIndex(0), MoveIndex(1), "");
    }
    #[test]
    fn set_move_metadata() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_key,
            credit_key,
            sum!(),
            "",
        );
        book.set_move_metadata(TransactionIndex(0), MoveIndex(0), "!");
        assert_eq!(*book.transactions[0].moves[0].metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn remove_transaction_panic_out_of_bounds() {
        let mut book = test_book!("");
        book.remove_transaction(TransactionIndex(0));
    }
    #[test]
    fn remove_transaction() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "a");
        book.insert_transaction(TransactionIndex(1), "b");
        book.remove_transaction(TransactionIndex(1));
        assert_eq!(&book.transactions[0].metadata, &"a");
        book.remove_transaction(TransactionIndex(0));
        assert!(book.transactions.is_empty());
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn remove_move_panic_transaction_index_out_of_bounds() {
        let mut book = test_book!("");
        book.remove_move(TransactionIndex(0), MoveIndex(0));
    }
    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn remove_move_panic_move_index_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        book.remove_move(TransactionIndex(0), MoveIndex(0));
    }
    #[test]
    fn remove_move() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "a",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(1),
            debit_account_key,
            credit_account_key,
            sum!(),
            "b",
        );
        book.remove_move(TransactionIndex(0), MoveIndex(1));
        assert_eq!(&book.transactions[0].moves[0].metadata, &"a");
        book.remove_move(TransactionIndex(0), MoveIndex(0));
        assert!(book.transactions[0].moves.is_empty());
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_side_panic_transaction_out_of_bounds() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Debit,
            account_key,
        );
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_side_panic_move_out_of_bounds() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Debit,
            account_key,
        );
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn set_move_side_panic_account_not_found() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        let other_account_key = book.new_account("");
        book.accounts.remove(other_account_key);
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Debit,
            other_account_key,
        );
    }
    #[test]
    #[should_panic(
        expected = "Provided debit account is same as existing credit account."
    )]
    fn set_move_side_panic_provided_debit_same_as_credit() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Debit,
            credit_account_key,
        );
    }
    #[test]
    #[should_panic(
        expected = "Provided credit account is same as existing debit account."
    )]
    fn set_move_side_panic_provided_credit_same_as_debit() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Credit,
            debit_account_key,
        );
    }
    #[test]
    fn set_move_side() {
        let mut book = test_book!("");
        let account_a_key = book.new_account("");
        let account_b_key = book.new_account("");
        let account_c_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            account_a_key,
            account_b_key,
            sum!(),
            "",
        );
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Debit,
            account_c_key,
        );
        assert_eq!(
            book.transactions[0].moves[0].debit_account_key,
            account_c_key
        );
        assert_eq!(
            book.transactions[0].moves[0].credit_account_key,
            account_b_key
        );
        book.set_move_side(
            TransactionIndex(0),
            MoveIndex(0),
            Credit,
            account_a_key,
        );
        assert_eq!(
            book.transactions[0].moves[0].debit_account_key,
            account_c_key
        );
        assert_eq!(
            book.transactions[0].moves[0].credit_account_key,
            account_a_key
        );
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_sum_panic_transaction_out_of_bounds() {
        let mut book = test_book!("");
        book.set_move_sum(TransactionIndex(0), MoveIndex(0), sum!());
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_sum_panic_move_out_of_bounds() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        book.set_move_sum(TransactionIndex(0), MoveIndex(0), sum!());
    }
    #[test]
    fn set_move_sum() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        let usd = TestUnit("USD");
        book.set_move_sum(TransactionIndex(0), MoveIndex(0), sum!(100, usd));
        assert_eq!(
            book.transactions[0].moves[0].sum.0.get(&usd).unwrap(),
            &100,
        );
    }
}
