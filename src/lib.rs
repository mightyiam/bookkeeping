use duplicate::duplicate_inline;
use maplit::{btreemap, btreeset};
use std::{
    cell::RefCell,
    cmp::{Ord, Ordering},
    collections::{BTreeMap, BTreeSet},
    fmt,
    rc::Rc,
    sync::{atomic, atomic::AtomicUsize},
};
type Amount = u64;
type Sum = BTreeMap<Rc<Unit>, Amount>;
type EntityId = usize;
type EntitySet<T> = RefCell<BTreeSet<Rc<T>>>;
static BOOK_COUNTER: AtomicUsize = AtomicUsize::new(0);
#[derive(Default)]
struct Book {
    id: usize,
    accounts: EntitySet<Account>,
    units: EntitySet<Unit>,
    moves: EntitySet<Move>,
}
impl Book {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: BOOK_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            ..Default::default()
        })
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
        let expected = EntitySet::default();
        assert_eq!(*actual, expected);
    }
    let other_book = Book::new();
    assert_ne!(book.id, other_book.id);
}
impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        other.id == self.id
    }
}
#[test]
fn book_partial_eq() {
    let a = Rc::new(Book {
        id: 0,
        ..Default::default()
    });
    let b = Rc::new(Book {
        id: 0,
        ..Default::default()
    });
    assert_eq!(a, b, "All fields equal");
    let c = Rc::new(Book {
        id: 0,
        ..Default::default()
    });
    Account::new(&c);
    assert_eq!(a, c, "Same id, some different field");
    let d = Rc::new(Book {
        id: 1,
        ..Default::default()
    });
    assert_ne!(a, d, "Only id different");
}
impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Book").field("id", &self.id).finish()
    }
}
#[test]
fn book_fmt_debug() {
    let book = Book::default();
    let actual = format!("{:?}", book);
    let expected = "Book { id: 0 }";
    assert_eq!(actual, expected);
    let book = Book {
        id: 1,
        ..Default::default()
    };
    let actual = format!("{:?}", book);
    let expected = "Book { id: 1 }";
    assert_eq!(actual, expected);
}
#[derive(Default)]
struct Account {
    id: EntityId,
    book: Rc<Book>,
    moves: EntitySet<Move>,
}
impl Account {
    fn new(book: &Rc<Book>) -> Rc<Self> {
        let account = Rc::new(Self {
            book: book.clone(),
            id: Self::next_id(&book),
            moves: RefCell::new(BTreeSet::new()),
        });
        Self::register(&account, &book);
        account
    }
}
#[test]
fn account_new() {
    let book = Book::new();
    let account_a = Account::new(&book);
    assert_eq!(account_a.id, 0);
    assert_eq!(account_a.book, book);
    assert_eq!(*account_a.moves.borrow(), BTreeSet::new());
    let account_b = Account::new(&book);
    assert_eq!(account_b.id, 1);
    assert_eq!(account_b.book, book);
    assert_eq!(*account_b.moves.borrow(), BTreeSet::new());
    let expected = btreeset! {
        account_a.clone(),
        account_b.clone()
    };
    assert_eq!(
        *book.accounts.borrow(),
        expected,
        "Accounts are in the book"
    );
}
impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account").field("id", &self.id).finish()
    }
}
#[test]
fn account_fmt_debug() {
    let book = Book::new();
    let account = Account::new(&book);
    let actual = format!("{:?}", account);
    let expected = "Account { id: 0 }";
    assert_eq!(actual, expected);
    let account = Account::new(&book);
    let actual = format!("{:?}", account);
    let expected = "Account { id: 1 }";
    assert_eq!(actual, expected);
}
struct Unit {
    id: EntityId,
    book: Rc<Book>,
}
impl Unit {
    fn new(book: &Rc<Book>) -> Rc<Self> {
        let unit = Rc::new(Self {
            id: Self::next_id(&book),
            book: book.clone(),
        });
        Self::register(&unit, &book);
        unit
    }
}
#[test]
fn unit_new() {
    let book = Book::new();
    let unit_a = Unit::new(&book);
    assert_eq!(unit_a.id, 0);
    assert_eq!(unit_a.book, book);
    let unit_b = Unit::new(&book);
    assert_eq!(unit_b.id, 1);
    assert_eq!(unit_b.book, book);
    let expected = btreeset! {
        unit_a.clone(),
        unit_b.clone()
    };
    assert_eq!(*book.units.borrow(), expected, "Units are in the book");
}
impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Unit").field("id", &self.id).finish()
    }
}
#[test]
fn unit_fmt_debug() {
    let book = Book::new();
    let unit = Unit::new(&book);
    let actual = format!("{:?}", unit);
    let expected = "Unit { id: 0 }";
    assert_eq!(actual, expected);
    let unit = Unit::new(&book);
    let actual = format!("{:?}", unit);
    let expected = "Unit { id: 1 }";
    assert_eq!(actual, expected);
}
#[derive(Debug)]
struct Move {
    book: Rc<Book>,
    id: EntityId,
    debit: Rc<Account>,
    credit: Rc<Account>,
    sum: Sum,
}
#[test]
fn move_fmt_debug() {
    let id = 0;
    let book = Rc::new(Book {
        id,
        ..Default::default()
    });
    let debit = Account::new(&book);
    let credit = Account::new(&book);
    let unit = Unit::new(&book);
    let sum = btreemap! { unit.clone() => 76 };
    let move_ = Move::new(&debit, &credit, sum.clone());
    let actual = format!("{:?}", move_);
    let expected = format!(
        "Move {{ book: {:?}, id: {:?}, debit: {:?}, credit: {:?}, sum: {:?} }}",
        book, id, debit, credit, sum,
    );
    assert_eq!(actual, expected);
}
impl Move {
    fn new(debit: &Rc<Account>, credit: &Rc<Account>, sum: Sum) -> Rc<Self> {
        let book = {
            let book = debit.book.clone();
            assert_eq!(
                book.id, credit.book.id,
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
        debit.moves.borrow_mut().insert(move_.clone());
        credit.moves.borrow_mut().insert(move_.clone());
        Self::register(&move_, &book);
        move_
    }
}
#[test]
#[should_panic(expected = "Debit and credit accounts are in different books.")]
fn move_new_panic_debit_and_credit_accounts_are_in_different_books() {
    let debit = Account::new(&Book::new());
    let credit = Account::new(&Book::new());
    Move::new(&debit, &credit, Sum::new());
}
#[test]
#[should_panic(expected = "Debit and credit accounts are the same.")]
fn move_new_panic_debit_and_credit_accounts_are_the_same() {
    let book = Book::new();
    let account = Account::new(&book);
    Move::new(&account, &account, Sum::new());
}
#[test]
#[should_panic(expected = "Some unit is not in the same book as accounts.")]
fn move_new_panic_some_unit_is_not_in_the_same_book_as_accounts() {
    let book = Book::new();
    let debit = Account::new(&book);
    let credit = Account::new(&book);
    let unit = Unit::new(&Book::new());
    let sum = btreemap! { unit.clone() => 0 };
    Move::new(&debit, &credit, sum);
}
#[test]
fn move_new() {
    let book = Book::new();
    let debit = Account::new(&book);
    let credit = Account::new(&book);
    let thb = Unit::new(&book);
    let ils = Unit::new(&book);
    let usd = Unit::new(&book);
    let sum = btreemap! {
        thb.clone() => 20,
        ils.clone() => 41,
        usd.clone() => 104,
    };
    let move_a = Move::new(&debit, &credit, sum.clone());
    let sum = btreemap! {
        thb.clone() => 13,
        ils.clone() => 805,
        usd.clone() => 10,
    };
    let move_b = Move::new(&debit, &credit, sum.clone());
    assert_eq!(
        *book.moves.borrow(),
        btreeset! { move_a.clone(), move_b.clone() }
    );
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
        fn register(entity: &Rc<Self>, book: &Book) {
            book.book_field.borrow_mut().insert(entity.clone());
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
            other.book == self.book && other.id == self.id
        }
    }
    impl Eq for Entity {}
}
// TODO Macro that creates sums
