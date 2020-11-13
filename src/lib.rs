use duplicate::duplicate_inline;
use maplit::{btreemap, btreeset};
use std::{
    cell::RefCell,
    cmp::{Ord, Ordering},
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    sync::{atomic, atomic::AtomicUsize},
};
type Amount = u64;
type Sum = BTreeMap<Rc<Unit>, Amount>;
type EntityId = usize;
type MutableEntitySet<T> = RefCell<BTreeSet<Rc<T>>>;
fn mutable_entity_set<T: Ord>() -> MutableEntitySet<T> {
    RefCell::new(BTreeSet::new())
}
#[test]
fn mutable_entity_set_return() {
    let actual = mutable_entity_set::<()>();
    let expected = RefCell::new(BTreeSet::new());
    assert_eq!(actual, expected);
}
static BOOK_COUNTER: AtomicUsize = AtomicUsize::new(0);
#[derive(Debug)]
struct Book {
    id: usize,
    accounts: MutableEntitySet<Account>,
    units: MutableEntitySet<Unit>,
    moves: MutableEntitySet<Move>,
}
impl Book {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: BOOK_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            accounts: mutable_entity_set(),
            units: mutable_entity_set(),
            moves: mutable_entity_set(),
        })
    }
}
impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        other.id == self.id
    }
}
#[test]
fn book_new() {
    let book = Book::new();
    duplicate_inline! {
        [
            Entity field_name;
            [Account] [accounts];
            [Unit] [units];
            [Move] [moves];
        ]
        let actual = &book.field_name;
        let expected = mutable_entity_set::<Entity>();
        assert_eq!(*actual, expected);
    }
}
#[derive(Debug)]
struct Account {
    id: EntityId,
    book: Rc<Book>,
    debits: MutableEntitySet<Move>,
    credits: MutableEntitySet<Move>,
}
impl Account {
    fn new(book: Rc<Book>) -> Rc<Self> {
        let account = Rc::new(Self {
            book: book.clone(),
            id: Self::next_id(&book),
            debits: RefCell::new(BTreeSet::new()),
            credits: RefCell::new(BTreeSet::new()),
        });
        Self::register(account.clone(), &book);
        account
    }
}
#[test]
fn account_new() {
    let book = Book::new();
    let account = Account::new(book.clone());
    // TODO use PartialEq to compare Account with Account
    let actual = account.id;
    let expected = 0;
    assert_eq!(actual, expected);
    let actual = &account.book;
    let expected = &book;
    // TODO use PartialEq to compare Book with Book
    assert_eq!(actual.id, expected.id);
    let actual = account.debits.borrow();
    let expected = BTreeSet::new();
    assert_eq!(*actual, expected);
    let actual = account.credits.borrow();
    let expected = BTreeSet::new();
    assert_eq!(*actual, expected);
    assert_eq!(book.accounts.borrow().len(), 1);
    assert_eq!(book.accounts.borrow().iter().next().unwrap().id, account.id);
}
#[derive(Debug)]
struct Unit {
    id: EntityId,
    book: Rc<Book>,
}
impl Unit {
    fn new(book: Rc<Book>) -> Rc<Self> {
        let unit = Rc::new(Self {
            id: Self::next_id(&book),
            book: book.clone(),
        });
        Self::register(unit.clone(), &book);
        unit
    }
}
#[derive(Debug)]
struct Move {
    book: Rc<Book>,
    id: EntityId,
    debit: Rc<Account>,
    credit: Rc<Account>,
    sum: Sum,
}
impl Move {
    fn new(debit: Rc<Account>, credit: Rc<Account>, sum: Sum) -> Rc<Self> {
        let book = {
            let book = debit.book.clone();
            assert_eq!(
                // TODO use PartialEq to compare Book with Book
                book.id,
                credit.book.id,
                "Debit and credit accounts are in different books."
            );
            assert!(debit != credit, "Debit and credit accounts are the same.");
            book
        };
        sum.keys().for_each(|unit| {
            assert!(
                book.units.borrow().contains(unit),
                "Some unit is not in the same book as accounts."
            );
        });
        let move_ = Rc::new(Self {
            book: book.clone(),
            id: Self::next_id(&book),
            debit: debit.clone(),
            credit: credit.clone(),
            sum,
        });
        debit.debits.borrow_mut().insert(move_.clone());
        credit.credits.borrow_mut().insert(move_.clone());
        Self::register(move_.clone(), &book);
        move_
    }
}
#[test]
#[should_panic(expected = "Debit and credit accounts are in different books.")]
fn move_new_panic_debit_and_credit_accounts_are_in_different_books() {
    let debit = Account::new(Book::new());
    let credit = Account::new(Book::new());
    Move::new(debit.clone(), credit.clone(), Sum::new());
}
#[test]
#[should_panic(expected = "Debit and credit accounts are in different books.")]
fn move_new_panic_debit_and_credit_accounts_with_same_id_in_different_books() {
    let book = Book::new();
    let debit = Account::new(book.clone());
    let credit = Account::new(Book::new());
    assert_eq!(debit.id, credit.id);
    Move::new(debit.clone(), credit.clone(), Sum::new());
}
#[test]
#[should_panic(expected = "Debit and credit accounts are the same.")]
fn move_new_panic_debit_and_credit_accounts_are_the_same() {
    let book = Book::new();
    let account = Account::new(book.clone());
    Move::new(account.clone(), account.clone(), Sum::new());
}
#[test]
#[should_panic(expected = "Some unit is not in the same book as accounts.")]
fn move_new_panic_some_unit_is_not_in_the_same_book_as_accounts() {
    let book = Book::new();
    let debit = Account::new(book.clone());
    let credit = Account::new(book.clone());
    let unit = Unit::new(Book::new());
    let sum = btreemap! { unit.clone() => 0 };
    Move::new(debit.clone(), credit.clone(), sum);
}
#[test]
fn move_new() {
    let book = Book::new();
    let debit = Account::new(book.clone());
    let credit = Account::new(book.clone());
    let thb = Unit::new(book.clone());
    let ils = Unit::new(book.clone());
    let usd = Unit::new(book.clone());
    let sum = btreemap! {
        thb.clone() => 20,
        ils.clone() => 41,
        usd.clone() => 104,
    };
    let move_ = Move::new(debit.clone(), credit.clone(), sum.clone());
}
duplicate_inline! {
    [
        Entity book_field;
        [Account] [accounts];
        [Unit] [units];
        [Move] [moves];
    ]
    impl Entity {
        fn next_id(book: &Book) -> EntityId {
            book.book_field.borrow().len()
        }
        fn register(entity: Rc<Self>, book: &Book) {
            book.book_field.borrow_mut().insert(entity);
        }
    }
    impl Ord for Entity {
        fn cmp(&self, other: &Self) -> Ordering {
            self.id.cmp(&other.id)
        }
    }
    impl PartialOrd for Entity {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }
    impl PartialEq for Entity {
        fn eq(&self, other: &Self) -> bool {
            // TODO use PartialEq to compare Book with Book
            other.book.id == self.book.id && other.id == self.id
        }
    }
    impl Eq for Entity {}
}
// TODO implicit cloning?
