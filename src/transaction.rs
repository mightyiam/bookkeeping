use crate::move_::Move;
/// Represents a transaction.
pub struct Transaction<Unit, SumNumber, Extra, MoveExtra>
where
    Unit: Ord,
{
    pub(crate) extra: Extra,
    pub(crate) moves: Vec<Move<Unit, SumNumber, MoveExtra>>,
}
/// Used to index moves in a transaction.
pub struct MoveIndex(pub usize);
impl<Unit, SumNumber, Extra, MoveExtra>
    Transaction<Unit, SumNumber, Extra, MoveExtra>
where
    Unit: Ord,
{
    /// Gets an iterator of existing moves in their order.
    pub fn moves(
        &self,
    ) -> impl Iterator<Item = (MoveIndex, &Move<Unit, SumNumber, MoveExtra>)>
    {
        self.moves
            .iter()
            .enumerate()
            .map(|(index, move_)| (MoveIndex(index), move_))
    }
    /// Gets the extra data of the transaction.
    pub fn extra(&self) -> &Extra {
        &self.extra
    }
}

#[cfg(test)]
mod test {
    use super::{MoveIndex, Transaction};
    use crate::{book::TransactionIndex, test_utils::TestBook};
    #[test]
    fn moves() {
        let mut book = TestBook::default();
        book.insert_transaction(TransactionIndex(0), "");
        let debit_account_key = book.insert_account("");
        let credit_account_key = book.insert_account("");
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
                .map(|(_move_index, move_)| move_.extra())
                .collect::<Vec<_>>(),
            vec![&"c", &"a", &"d", &"b"],
        );
    }
    #[test]
    fn extra() {
        let transaction = Transaction::<&str, u8, &str, ()> {
            extra: "deposit",
            moves: Vec::new(),
        };
        assert_eq!(transaction.extra(), &"deposit",);
    }
}
