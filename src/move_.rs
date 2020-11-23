use crate::account::Account;
use crate::index::{EntityId, Index};
use crate::metadata::Metadata;
use crate::sum::Sum;
use std::cell::RefCell;
use std::rc::Rc;
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
#[derive(Debug)]
pub struct Move<T: Metadata> {
    pub(crate) index: Rc<Index<T>>,
    pub(crate) id: EntityId,
    pub(crate) meta: RefCell<T::Move>,
    pub(crate) debit_account: Rc<Account<T>>,
    pub(crate) credit_account: Rc<Account<T>>,
    pub(crate) sum: Sum<T>,
}
impl<T: Metadata> Move<T> {
    pub(crate) fn new(
        id: EntityId,
        index: &Rc<Index<T>>,
        debit_account: &Rc<Account<T>>,
        credit_account: &Rc<Account<T>>,
        sum: &Sum<T>,
        meta: T::Move,
    ) -> Rc<Self> {
        assert!(
            debit_account != credit_account,
            "Debit and credit accounts are the same."
        );
        let move_ = Rc::new(Self {
            id,
            index: index.clone(),
            meta: RefCell::new(meta),
            debit_account: debit_account.clone(),
            credit_account: credit_account.clone(),
            sum: sum.clone(),
        });
        move_
    }
}
#[cfg(test)]
mod test {
    use super::Move;
    use super::Rc;
    use super::RefCell;
    use super::Sum;
    use crate::book::Book;
    use crate::metadata::BlankMetadata;
    #[test]
    #[should_panic(expected = "Debit and credit accounts are the same.")]
    fn move_new_panic_debit_and_credit_accounts_are_the_same() {
        let mut book = Book::<BlankMetadata>::new(());
        let account = book.new_account(());
        Move::new(0, &book.index, &account, &account, &Sum::new(), ());
    }
    #[test]
    fn new() {
        let mut book = Book::<((), (), (), u8)>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        let thb = book.new_unit(());
        let ils = book.new_unit(());
        let usd = book.new_unit(());
        let sum = Sum::of(&thb, 20).unit(&ils, 41).unit(&usd, 104);
        let move_a = Move::new(0, &book.index, &debit, &credit, &sum, 45);
        let expected = Rc::new(Move {
            index: book.index.clone(),
            id: 0,
            meta: RefCell::new(45),
            debit_account: debit.clone(),
            credit_account: credit.clone(),
            sum: sum.clone(),
        });
        assert_eq!(move_a, expected);
    }
    #[test]
    fn metadata() {
        let mut book = Book::<((), (), (), u8)>::new(());
        let account_a = book.new_account(());
        let account_b = book.new_account(());
        let move_ = Move::new(0, &book.index, &account_a, &account_b, &Sum::new(), 5);
        assert_eq!(*move_.get_metadata(), 5);
        move_.set_metadata(9);
        assert_eq!(*move_.get_metadata(), 9);
    }
}
