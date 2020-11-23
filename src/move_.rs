use crate::account::Account;
use crate::balance::Balance;
use crate::index::{EntityId, Index};
use crate::metadata::Metadata;
use crate::sum::Sum;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops;
use std::rc::Rc;
/// Represents a move of a [Sum] of [Unit](crate::Unit)s from one account to another.
#[derive(Debug)]
pub struct Move<T: Metadata> {
    pub(crate) index: Rc<Index<T>>,
    pub(crate) id: EntityId,
    pub(crate) meta: RefCell<T::Move>,
    debit_account: Rc<Account<T>>,
    credit_account: Rc<Account<T>>,
    sum: Sum<T>,
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
    /// Calculates the balance that is the result of this move in an account according to a provided order of moves.
    ///
    /// ## Panics
    ///
    /// - `account` is not debit nor credit in this move.
    pub fn balance_in(
        &self,
        account: &Rc<Account<T>>,
        cmp: impl Fn(&T::Move, &T::Move) -> Ordering,
    ) -> Balance<T> {
        if ![&self.debit_account, &self.credit_account].contains(&account) {
            panic!("Provided account is not debit nor credit in this move.");
        }
        account
            .index
            .moves
            .borrow()
            .iter()
            .filter(
                |move_| match cmp(&self.meta.borrow(), &move_.meta.borrow()) {
                    Ordering::Less => false,
                    _ => true,
                },
            )
            .filter_map(|move_| -> Option<(fn(&mut Balance<T>, _), &Sum<T>)> {
                if move_.debit_account == *account {
                    Some((ops::SubAssign::sub_assign, &move_.sum))
                } else if move_.credit_account == *account {
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
}
#[cfg(test)]
mod test {
    use super::Balance;
    use super::Move;
    use super::Rc;
    use super::RefCell;
    use super::Sum;
    use crate::book::Book;
    use crate::metadata::BlankMetadata;
    #[test]
    #[should_panic(expected = "Provided account is not debit nor credit in this move.")]
    fn move_balance_in_unrelated_account() {
        let mut book = Book::<BlankMetadata>::new(());
        let debit_account = book.new_account(());
        let credit_account = book.new_account(());
        let move_ = Move::new(
            0,
            &book.index,
            &debit_account,
            &credit_account,
            &Sum::new(),
            (),
        );
        move_.balance_in(&book.new_account(()), |&(), &()| {
            panic!();
        });
    }
    #[test]
    fn balance_in() {
        use maplit::btreemap;
        let cmp = |a: &u8, b: &u8| a.cmp(&b);
        let mut book = Book::<((), (), (), u8)>::new(());
        let account_a = book.new_account(());
        let account_b = book.new_account(());
        let unit = book.new_unit(());
        let move_1 = book.new_move(&account_a, &account_b, &Sum::of(&unit, 3), 1);
        assert_eq!(
            move_1.balance_in(&account_a, cmp),
            Balance(btreemap! { unit.clone() => -3 })
        );
        assert_eq!(
            move_1.balance_in(&account_b, cmp),
            Balance(btreemap! { unit.clone() => 3 })
        );

        let move_2 = book.new_move(&account_a, &account_b, &Sum::of(&unit, 4), 2);
        assert_eq!(
            move_1.balance_in(&account_a, cmp),
            Balance(btreemap! { unit.clone() => -3 })
        );
        assert_eq!(
            move_1.balance_in(&account_b, cmp),
            Balance(btreemap! { unit.clone() => 3 })
        );
        assert_eq!(
            move_2.balance_in(&account_a, cmp),
            Balance(btreemap! { unit.clone() => -7 })
        );
        assert_eq!(
            move_2.balance_in(&account_b, cmp),
            Balance(btreemap! { unit.clone() => 7 })
        );

        let move_0 = book.new_move(&account_a, &account_b, &Sum::of(&unit, 1), 0);
        assert_eq!(
            move_0.balance_in(&account_a, cmp),
            Balance(btreemap! { unit.clone() => -1 })
        );
        assert_eq!(
            move_0.balance_in(&account_b, cmp),
            Balance(btreemap! { unit.clone() => 1 })
        );
        assert_eq!(
            move_1.balance_in(&account_a, cmp),
            Balance(btreemap! { unit.clone() => -4 })
        );
        assert_eq!(
            move_1.balance_in(&account_b, cmp),
            Balance(btreemap! { unit.clone() => 4 })
        );
        assert_eq!(
            move_2.balance_in(&account_a, cmp),
            Balance(btreemap! { unit.clone() => -8 })
        );
        assert_eq!(
            move_2.balance_in(&account_b, cmp),
            Balance(btreemap! { unit.clone() => 8 })
        );
    }
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
