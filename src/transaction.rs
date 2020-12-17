use crate::move_::Move;
/// Represents a transaction.
pub struct Transaction<M, T> {
    pub(crate) metadata: T,
    pub(crate) moves: Vec<Move<M>>,
}
/// Used to index moves in a transaction.
pub struct MoveIndex(pub usize);
impl<M, T> Transaction<M, T> {
    /// Gets an iterator of existing moves in their order.
    ///
    /// # Example
    /// ```
    /// # use bookkeeping::*;
    /// # let mut book = Book::<(), (), (), &str, ()>::new(());
    /// # let wallet_key = book.new_account(());
    /// # let bank_key = book.new_account(());
    /// # book.insert_transaction(TransactionIndex(0), ());
    /// book.insert_move(
    ///     TransactionIndex(0),
    ///     MoveIndex(0),
    ///     wallet_key,
    ///     bank_key,
    ///     Sum::new(),
    ///     "deposit"
    /// );
    /// book.insert_move(
    ///     TransactionIndex(0),
    ///     MoveIndex(1),
    ///     bank_key,
    ///     wallet_key,
    ///     Sum::new(),
    ///     "withdrawal"
    /// );
    /// assert_eq!(
    ///     book
    ///         .transactions()
    ///         .nth(0)
    ///         .unwrap()
    ///         .1
    ///         .moves()
    ///         .map(|(_move_index, move_)| move_.metadata())
    ///         .collect::<Vec<_>>(),
    ///     [&"deposit", &"withdrawal"],
    /// );
    pub fn moves(&self) -> impl Iterator<Item = (MoveIndex, &Move<M>)> {
        self.moves
            .iter()
            .enumerate()
            .map(|(index, move_)| (MoveIndex(index), move_))
    }
    /// Gets the metadata of the transaction.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::*;
    /// # let mut book = Book::<(), (), (), (), &str>::new(());
    /// # book.insert_transaction(TransactionIndex(0), "deposit");
    /// # let (_transaction_index, transaction) = book
    /// #     .transactions()
    /// #     .nth(0)
    /// #     .unwrap();
    /// assert_eq!(
    ///     *transaction.metadata(),
    ///     "deposit",
    /// );
    /// ```
    pub fn metadata(&self) -> &T {
        &self.metadata
    }
}

#[cfg(test)]
mod test {
    use super::{MoveIndex, Transaction};
    use crate::book::TransactionIndex;
    #[test]
    fn moves() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
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
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "c",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(2),
            debit_account_key,
            credit_account_key,
            sum!(),
            "d",
        );
        assert_eq!(
            book.transactions()
                .next()
                .unwrap()
                .1
                .moves()
                .map(|(_move_index, move_)| move_.metadata())
                .collect::<Vec<_>>(),
            vec![&"c", &"a", &"d", &"b"],
        );
    }
    #[test]
    fn metadata() {
        let transaction = Transaction::<(), &str> {
            metadata: "deposit",
            moves: Vec::new(),
        };
        assert_eq!(transaction.metadata(), &"deposit",);
    }
}
