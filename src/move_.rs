use crate::account::Account;
use crate::balance::Balance;
use crate::book::{Book, EntityId, Index};
use crate::metadata::{BlankMetadata, Metadata};
use crate::sum::Sum;
use crate::unit::Unit;
use std::cmp::Ordering;
use std::ops;
use std::rc::Rc;
#[derive(Debug)]
pub struct Move<T: Metadata> {
    pub(crate) index: Rc<Index<T>>,
    pub(crate) id: EntityId,
    meta: T::Move,
    debit_account: Rc<Account<T>>,
    credit_account: Rc<Account<T>>,
    sum: Sum<T>,
}
impl<T: Metadata> Move<T> {
    pub fn new(
        debit_account: &Rc<Account<T>>,
        credit_account: &Rc<Account<T>>,
        sum: &Sum<T>,
        meta: T::Move,
    ) -> Rc<Self> {
        let index = {
            let index = debit_account.index.clone();
            assert_eq!(
                index.id, credit_account.index.id,
                "Debit and credit accounts are in different books."
            );
            assert!(
                debit_account != credit_account,
                "Debit and credit accounts are the same."
            );
            index
        };
        sum.0.keys().for_each(|unit| {
            assert!(
                index.units.borrow().contains(unit),
                "Some unit is not in the same book as accounts."
            );
        });
        let move_ = Rc::new(Self {
            index: index.clone(),
            id: Self::next_id(&index),
            meta,
            debit_account: debit_account.clone(),
            credit_account: credit_account.clone(),
            sum: sum.clone(),
        });
        Self::register(&move_, &index);
        move_
    }
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
            .filter(|move_| match cmp(&self.meta, &move_.meta) {
                Ordering::Less => false,
                _ => true,
            })
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
#[test]
#[should_panic(expected = "Provided account is not debit nor credit in this move.")]
fn move_balance_in_unrelated_account() {
    let book = Book::<BlankMetadata>::new(());
    let move_ = Move::new(
        &Account::new(&book, ()),
        &Account::new(&book, ()),
        &Sum::of(&Unit::new(&book, ()), 123),
        (),
    );
    move_.balance_in(&Account::new(&book, ()), |&(), &()| {
        panic!();
    });
}
#[test]
fn move_balance_in() {
    use maplit::btreemap;
    let cmp = |a: &u8, b: &u8| a.cmp(&b);
    let book = Book::<((), (), (), u8)>::new(());
    let account_a = Account::new(&book, ());
    let account_b = Account::new(&book, ());
    let unit = Unit::new(&book, ());
    let move_1 = Move::new(&account_a, &account_b, &Sum::of(&unit, 3), 1);
    assert_eq!(
        move_1.balance_in(&account_a, cmp),
        Balance(btreemap! { unit.clone() => -3 })
    );
    assert_eq!(
        move_1.balance_in(&account_b, cmp),
        Balance(btreemap! { unit.clone() => 3 })
    );

    let move_2 = Move::new(&account_a, &account_b, &Sum::of(&unit, 4), 2);
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

    let move_0 = Move::new(&account_a, &account_b, &Sum::of(&unit, 1), 0);
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
#[should_panic(expected = "Debit and credit accounts are in different books.")]
fn move_new_panic_debit_and_credit_accounts_are_in_different_books() {
    let debit = Account::<BlankMetadata>::new(&Book::new(()), ());
    let credit = Account::new(&Book::new(()), ());
    Move::new(&debit, &credit, &Sum::new(), ());
}
#[test]
#[should_panic(expected = "Debit and credit accounts are the same.")]
fn move_new_panic_debit_and_credit_accounts_are_the_same() {
    let book = Book::<BlankMetadata>::new(());
    let account = Account::new(&book, ());
    Move::new(&account, &account, &Sum::new(), ());
}
#[test]
#[should_panic(expected = "Some unit is not in the same book as accounts.")]
fn move_new_panic_some_unit_is_not_in_the_same_book_as_accounts() {
    let book = Book::<BlankMetadata>::new(());
    let debit = Account::new(&book, ());
    let credit = Account::new(&book, ());
    let unit = Unit::new(&Book::new(()), ());
    let sum = Sum::of(&unit, 0);
    Move::new(&debit, &credit, &sum, ());
}
#[test]
fn move_new() {
    use maplit::btreeset;
    let book = Book::<((), (), (), u8)>::new(());
    let debit = Account::new(&book, ());
    let credit = Account::new(&book, ());
    let thb = Unit::new(&book, ());
    let ils = Unit::new(&book, ());
    let usd = Unit::new(&book, ());
    let sum = Sum::of(&thb, 20).unit(&ils, 41).unit(&usd, 104);
    let move_a = Move::new(&debit, &credit, &sum, 45);
    let expected = Rc::new(Move {
        index: book.index.clone(),
        id: 0,
        meta: 45,
        debit_account: debit.clone(),
        credit_account: credit.clone(),
        sum: sum.clone(),
    });
    assert_eq!(move_a, expected);
    let sum = Sum::of(&thb, 13).unit(&ils, 805).unit(&usd, 10);
    let move_b = Move::new(&debit, &credit, &sum, 0);
    assert_eq!(
        *book.index.moves.borrow(),
        btreeset! { move_a.clone(), move_b.clone() }
    );
}
