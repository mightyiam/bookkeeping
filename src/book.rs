use crate::{
    balance::Balance,
    move_::{Move, Side},
    sum::Sum,
    transaction::{MoveIndex, Transaction},
};
use slotmap::{new_key_type, DenseSlotMap};
use std::ops::{Add, AddAssign, Sub, SubAssign};
new_key_type! {
    /// A key type for referencing accounts.
    pub struct AccountKey;
}
/// Represents a book.
pub struct Book<Unit, SumNumber, Account, TransactionMeta, MoveMeta>
where
    Unit: Ord,
{
    accounts: DenseSlotMap<AccountKey, Account>,
    transactions: Vec<Transaction<Unit, SumNumber, TransactionMeta, MoveMeta>>,
}

/// Used to index transactions in the book.
pub struct TransactionIndex(pub usize);
impl<Unit, SumNumber, Account, TransactionMeta, MoveMeta> Default
    for Book<Unit, SumNumber, Account, TransactionMeta, MoveMeta>
where
    Unit: Ord,
{
    fn default() -> Self {
        Self {
            accounts: DenseSlotMap::with_key(),
            transactions: Vec::new(),
        }
    }
}
impl<Unit, SumNumber, Account, TransactionMeta, MoveMeta>
    Book<Unit, SumNumber, Account, TransactionMeta, MoveMeta>
where
    Unit: Ord,
{
    /// Inserts an account.
    pub fn insert_account(&mut self, account: Account) -> AccountKey {
        self.accounts.insert(account)
    }
    /// Creates a transaction and inserts it at an index.
    ///
    /// ## Panics
    ///
    /// - `transaction_index` out of bounds.
    pub fn insert_transaction(
        &mut self,
        transaction_index: TransactionIndex,
        metadata: TransactionMeta,
    ) where
        Unit: Ord,
    {
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
        sum: Sum<Unit, SumNumber>,
        metadata: MoveMeta,
    ) where
        Unit: Ord,
    {
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
    pub fn get_account(&self, account_key: AccountKey) -> &Account {
        self.assert_has_account(account_key);
        self.accounts.get(account_key).unwrap()
    }
    /// Gets an iterator of existing accounts in order of creation.
    pub fn accounts(&self) -> impl Iterator<Item = (AccountKey, &Account)> {
        self.accounts.iter()
    }
    /// Gets an iterator of existing transactions in their order.
    pub fn transactions(
        &self,
    ) -> impl Iterator<
        Item = (
            TransactionIndex,
            &Transaction<Unit, SumNumber, TransactionMeta, MoveMeta>,
        ),
    > {
        self.transactions
            .iter()
            .enumerate()
            .map(|(index, transaction)| (TransactionIndex(index), transaction))
    }
    /// Sets an existing account.
    ///
    /// ## Panics
    /// - `account_key` is not in the book.
    pub fn set_account(&mut self, account_key: AccountKey, account: Account) {
        self.assert_has_account(account_key);
        *self.accounts.get_mut(account_key).unwrap() = account;
    }
    /// Sets the metadata for a transaction.
    ///
    /// ## Panics
    /// - `transaction_index` out of bounds.
    pub fn set_transaction_metadata(
        &mut self,
        transaction_index: TransactionIndex,
        metadata: TransactionMeta,
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
        metadata: MoveMeta,
    ) where
        Unit: Ord,
    {
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
    pub fn account_balance_at_transaction<'a, BalanceNumber>(
        &'a self,
        account_key: AccountKey,
        transaction_index: TransactionIndex,
    ) -> Balance<Unit, BalanceNumber>
    where
        Unit: Ord + Clone,
        BalanceNumber: Default
            + Sub<Output = BalanceNumber>
            + Add<Output = BalanceNumber>
            + Clone,
        SumNumber: Clone + Into<BalanceNumber>,
    {
        self.assert_has_account(account_key);
        self.transactions
            .iter()
            .take(transaction_index.0 + 1)
            .flat_map(|transaction| transaction.moves.iter())
            .filter_map(
                |move_| -> Option<(
                    fn(
                        &mut Balance<Unit, BalanceNumber>,
                        &'a Sum<Unit, SumNumber>,
                    ),
                    &Sum<Unit, SumNumber>,
                )> {
                    if move_.debit_account_key == account_key {
                        Some((SubAssign::sub_assign, &move_.sum))
                    } else if move_.credit_account_key == account_key {
                        Some((AddAssign::add_assign, &move_.sum))
                    } else {
                        None
                    }
                },
            )
            .fold(
                <Balance<Unit, BalanceNumber> as Default>::default(),
                |mut balance, (operation, sum)| {
                    operation(&mut balance, sum);
                    balance
                },
            )
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
    ) where
        Unit: Ord,
    {
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
        sum: Sum<Unit, SumNumber>,
    ) where
        Unit: Ord,
    {
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
    ) where
        Unit: Ord,
    {
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
        Side::{Credit, Debit},
        TransactionIndex,
    };
    use crate::{
        test_utils::{TestBalance, TestBook},
        transaction::MoveIndex,
    };
    #[test]
    fn default() {
        let book = TestBook::default();
        assert!(book.accounts.is_empty());
        assert!(book.transactions.is_empty());
    }
    #[test]
    fn insert_account() {
        let mut book = TestBook::default();
        book.insert_account("");
        assert_eq!(book.accounts.len(), 1);
    }
    #[test]
    #[should_panic(expected = "insertion index (is 1) should be <= len (is 0)")]
    fn insert_transaction_panic_index_out_of_bounds() {
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(1), "");
    }
    #[test]
    fn insert_transaction() {
        let mut book = TestBook::default();
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
        let mut book = TestBook::default();
        let debit_key = book.insert_account("");
        let credit_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let debit_key = book.insert_account("");
        book.accounts.remove(debit_key);
        let credit_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let debit_key = book.insert_account("");
        let credit_key = book.insert_account("");
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
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(0), "");
        let debit_key = book.insert_account("");
        let credit_key = book.insert_account("");
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
        let mut book = TestBook::default();
        assert!(book.accounts().next().is_none());
        let account_a_key = book.insert_account("a");
        let account_b_key = book.insert_account("b");
        let expected = vec![(account_a_key, &"a"), (account_b_key, &"b")];
        let actual = book.accounts().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn get_account() {
        let mut book = TestBook::default();
        book.insert_account("");
        let account_key = book.insert_account("!");
        book.insert_account("");
        let account = book.get_account(account_key);
        assert_eq!(*account, "!");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn assert_has_account() {
        let mut book = TestBook::default();
        let account_key = book.insert_account("");
        book.accounts.remove(account_key);
        book.assert_has_account(account_key);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn account_balance_at_transaction_account_not_found() {
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(0), "");
        let account_key = book.insert_account("");
        book.accounts.remove(account_key);
        book.account_balance_at_transaction::<i128>(
            account_key,
            TransactionIndex(0),
        );
    }
    #[test]
    fn account_balance_at_transaction() {
        let mut book = TestBook::default();
        let account_a_key = book.insert_account("");
        let account_b_key = book.insert_account("");
        let usd = "USD";
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
            book.account_balance_at_transaction::<i128>(
                account_a_key,
                TransactionIndex(0)
            ),
            TestBalance::default() - &sum!(3, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_b_key,
                TransactionIndex(0)
            ),
            TestBalance::default() + &sum!(3, usd),
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
            book.account_balance_at_transaction::<i128>(
                account_a_key,
                TransactionIndex(0)
            ),
            TestBalance::default() - &sum!(3, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_b_key,
                TransactionIndex(0)
            ),
            TestBalance::default() + &sum!(3, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_a_key,
                TransactionIndex(1)
            ),
            TestBalance::default() - &sum!(7, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_b_key,
                TransactionIndex(1)
            ),
            TestBalance::default() + &sum!(7, usd),
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
            book.account_balance_at_transaction::<i128>(
                account_a_key,
                TransactionIndex(0)
            ),
            TestBalance::default() - &sum!(1, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_b_key,
                TransactionIndex(0)
            ),
            TestBalance::default() + &sum!(1, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_a_key,
                TransactionIndex(1)
            ),
            TestBalance::default() - &sum!(4, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_b_key,
                TransactionIndex(1)
            ),
            TestBalance::default() + &sum!(4, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_a_key,
                TransactionIndex(2)
            ),
            TestBalance::default() - &sum!(8, usd),
        );
        assert_eq!(
            book.account_balance_at_transaction::<i128>(
                account_b_key,
                TransactionIndex(2)
            ),
            TestBalance::default() + &sum!(8, usd),
        );
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn set_account_panic() {
        let mut book = TestBook::default();
        let account_key = book.insert_account("");
        book.accounts.remove(account_key);
        book.set_account(account_key, "!");
    }
    #[test]
    fn set_account() {
        let mut book = TestBook::default();
        let account_key = book.insert_account("");
        book.set_account(account_key, "!");
        assert_eq!(*book.accounts.get(account_key).unwrap(), "!");
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn set_move_metadata_panic_transaction_index_out_of_bounds() {
        let mut book = TestBook::default();
        book.set_move_metadata(TransactionIndex(0), MoveIndex(0), "");
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn set_move_metadata_panic_move_index_out_of_bounds() {
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(0), "");
        book.set_move_metadata(TransactionIndex(0), MoveIndex(1), "");
    }
    #[test]
    fn set_move_metadata() {
        let mut book = TestBook::default();
        let debit_key = book.insert_account("");
        let credit_key = book.insert_account("");
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
        let mut book = TestBook::default();
        book.remove_transaction(TransactionIndex(0));
    }
    #[test]
    fn remove_transaction() {
        let mut book = TestBook::default();
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
        let mut book = TestBook::default();
        book.remove_move(TransactionIndex(0), MoveIndex(0));
    }
    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn remove_move_panic_move_index_out_of_bounds() {
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(0), "");
        book.remove_move(TransactionIndex(0), MoveIndex(0));
    }
    #[test]
    fn remove_move() {
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let account_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let account_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        let other_account_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
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
        let mut book = TestBook::default();
        let account_a_key = book.insert_account("");
        let account_b_key = book.insert_account("");
        let account_c_key = book.insert_account("");
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
        let mut book = TestBook::default();
        book.set_move_sum(TransactionIndex(0), MoveIndex(0), sum!());
    }
    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 0 but the index is 0"
    )]
    fn set_move_sum_panic_move_out_of_bounds() {
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(0), "");
        book.set_move_sum(TransactionIndex(0), MoveIndex(0), sum!());
    }
    #[test]
    fn set_move_sum() {
        let mut book = TestBook::default();
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
        book.insert_transaction(TransactionIndex(0), "");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "",
        );
        let usd = "USD";
        book.set_move_sum(TransactionIndex(0), MoveIndex(0), sum!(100, usd));
        assert_eq!(
            book.transactions[0].moves[0].sum.0.get(&usd).unwrap(),
            &100,
        );
    }
}
