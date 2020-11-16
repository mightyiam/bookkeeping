#![feature(map_first_last)]
use duplicate::duplicate_inline;
use maplit::{btreemap, btreeset};
use std::{
    cell::RefCell,
    cmp::{Ord, Ordering},
    collections::{BTreeMap, BTreeSet},
    fmt, ops,
    rc::Rc,
    sync::{atomic, atomic::AtomicUsize},
};
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
#[derive(Clone, PartialEq)]
struct Sum(BTreeMap<Rc<Unit>, u64>);
impl Sum {
    fn new() -> Self {
        Self(Default::default())
    }
    fn unit(mut self, unit: &Rc<Unit>, amount: u64) -> Self {
        self.0.insert(unit.clone(), amount);
        self
    }
    // TODO method `units`
}
#[test]
fn sum_new() {
    let actual = Sum::new();
    let expected = Sum(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn sum_unit() {
    let book = Book::new();
    let unit = Unit::new(&book);
    let sum = Sum::new().unit(&unit, 124);
    let mut expected = BTreeMap::new();
    expected.insert(unit.clone(), 124);
    assert_eq!(sum.0, expected);
}
impl fmt::Debug for Sum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sum(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn sum_fmt_debug() {
    let book = Book::new();
    let unit_a = Unit::new(&book);
    let amount_a = 76;
    let unit_b = Unit::new(&book);
    let amount_b = 45;
    let sum = Sum::new().unit(&unit_a, amount_a).unit(&unit_b, amount_b);
    let actual = format!("{:?}", sum);
    let expected = format!(
        "Sum({{{:?}: {:?}, {:?}: {:?}}})",
        unit_a, amount_a, unit_b, amount_b
    );
    assert_eq!(actual, expected);
}
#[derive(Clone, PartialEq)]
struct Balance(BTreeMap<Rc<Unit>, i128>);
impl Balance {
    fn new() -> Self {
        Self(Default::default())
    }
    fn operation(&mut self, rhs: &Sum, amount_op: fn(i128, u64) -> i128) {
        rhs.0.iter().for_each(|(unit, amount)| {
            self.0
                .entry(unit.clone())
                .and_modify(|balance| {
                    *balance = amount_op(*balance, *amount);
                })
                .or_insert(amount_op(0, *amount));
        });
    }
}
#[test]
fn balance_new() {
    let actual = Balance::new();
    let expected = Balance(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn balance_operation() {
    let mut actual = Balance::new();
    let book = Book::new();
    let unit_a = Unit::new(&book);
    let unit_b = Unit::new(&book);
    let sum = Sum::new().unit(&unit_a, 2).unit(&unit_b, 3);
    actual.operation(&sum, |balance, amount| balance + amount as i128);
    let sum = Sum::new().unit(&unit_a, 2).unit(&unit_b, 3);
    actual.operation(&sum, |balance, amount| balance * amount as i128);
    let expected = Balance(btreemap! {
        unit_a.clone() => 4,
        unit_b.clone() => 9,
    });
    assert_eq!(actual, expected);
}
impl fmt::Debug for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn balance_fmt_debug() {
    let book = Book::new();
    let unit_a = Unit::new(&book);
    let amount_a = 76;
    let unit_b = Unit::new(&book);
    let amount_b = 45;
    let sum = Sum::new().unit(&unit_a, amount_a).unit(&unit_b, amount_b);
    let balance = Balance::new() + &sum;
    let actual = format!("{:?}", balance);
    let expected = format!(
        "Balance({{{:?}: {:?}, {:?}: {:?}}})",
        unit_a, amount_a, unit_b, amount_b
    );
    assert_eq!(actual, expected);
}
impl ops::Sub<&Sum> for Balance {
    type Output = Balance;
    fn sub(self, sum: &Sum) -> Self::Output {
        let mut clone = self.clone();
        clone.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
        clone
    }
}
#[test]
fn balance_minus_sum() {
    let book = Book::new();
    let unit = Unit::new(&book);
    let balance = Balance::new();
    let actual = balance - &Sum::new().unit(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl ops::Add<&Sum> for Balance {
    type Output = Balance;
    fn add(self, sum: &Sum) -> Self::Output {
        let mut clone = self.clone();
        clone.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
        clone
    }
}
#[test]
fn balance_plus_sum() {
    let book = Book::new();
    let unit = Unit::new(&book);
    let balance = Balance::new();
    let actual = balance + &Sum::new().unit(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
#[derive(Debug)]
struct Move {
    book: Rc<Book>,
    id: EntityId,
    debit_account: Rc<Account>,
    credit_account: Rc<Account>,
    debit_account_balance: Balance,
    credit_account_balance: Balance,
    sum: Sum,
}
impl Move {
    fn new(debit_account: &Rc<Account>, credit_account: &Rc<Account>, sum: &Sum) -> Rc<Self> {
        let book = {
            let book = debit_account.book.clone();
            assert_eq!(
                book.id, credit_account.book.id,
                "Debit and credit accounts are in different books."
            );
            assert!(
                debit_account != credit_account,
                "Debit and credit accounts are the same."
            );
            book
        };
        sum.0.keys().for_each(|unit| {
            assert!(
                book.units.borrow().contains(unit),
                "Some unit is not in the same book as accounts."
            );
        });
        // TODO deduplicate
        let debit_account_last_balance = {
            let debit_account_moves = debit_account.moves.borrow();
            let debit_account_last_move = debit_account_moves.last();
            match debit_account_last_move {
                None => Balance::new(),
                Some(move_) => move_.balance_in(&debit_account),
            }
        };
        // TODO deduplicate
        let credit_account_last_balance = {
            let credit_account_moves = credit_account.moves.borrow();
            let credit_account_last_move = credit_account_moves.last();
            match credit_account_last_move {
                None => Balance::new(),
                Some(move_) => move_.balance_in(&credit_account),
            }
        };
        let move_ = Rc::new(Self {
            book: book.clone(),
            id: Self::next_id(&book),
            debit_account: debit_account.clone(),
            credit_account: credit_account.clone(),
            debit_account_balance: debit_account_last_balance - &sum,
            credit_account_balance: credit_account_last_balance + &sum,
            sum: sum.clone(),
        });
        debit_account.moves.borrow_mut().insert(move_.clone());
        credit_account.moves.borrow_mut().insert(move_.clone());
        Self::register(&move_, &book);
        move_
    }
    fn balance_in(&self, account: &Rc<Account>) -> Balance {
        if *account == self.debit_account {
            self.debit_account_balance.clone()
        } else if *account == self.credit_account {
            self.credit_account_balance.clone()
        } else {
            panic!("Provided account is not debit nor credit in this move.");
        }
    }
}
#[test]
fn move_balance_in() {
    let book = Book::new();
    let debit = Account::new(&book);
    let credit = Account::new(&book);
    let unit = Unit::new(&book);
    let sum = Sum::new().unit(&unit, 90);
    let move_ = Move::new(&debit, &credit, &sum);
    let actual = move_.balance_in(&debit);
    let expected = Balance(btreemap! {
        unit.clone() => -90,
    });
    assert_eq!(expected, actual);
    let actual = move_.balance_in(&credit);
    let expected = Balance(btreemap! {
        unit.clone() => 90,
    });
    assert_eq!(actual, expected);
}
#[test]
#[should_panic(expected = "Debit and credit accounts are in different books.")]
fn move_new_panic_debit_and_credit_accounts_are_in_different_books() {
    let debit = Account::new(&Book::new());
    let credit = Account::new(&Book::new());
    Move::new(&debit, &credit, &Sum::new());
}
#[test]
#[should_panic(expected = "Debit and credit accounts are the same.")]
fn move_new_panic_debit_and_credit_accounts_are_the_same() {
    let book = Book::new();
    let account = Account::new(&book);
    Move::new(&account, &account, &Sum::new());
}
#[test]
#[should_panic(expected = "Some unit is not in the same book as accounts.")]
fn move_new_panic_some_unit_is_not_in_the_same_book_as_accounts() {
    let book = Book::new();
    let debit = Account::new(&book);
    let credit = Account::new(&book);
    let unit = Unit::new(&Book::new());
    let sum = Sum::new().unit(&unit, 0);
    Move::new(&debit, &credit, &sum);
}
#[test]
fn move_new() {
    let book = Book::new();
    let debit = Account::new(&book);
    let credit = Account::new(&book);
    let thb = Unit::new(&book);
    let ils = Unit::new(&book);
    let usd = Unit::new(&book);
    let sum = Sum::new().unit(&thb, 20).unit(&ils, 41).unit(&usd, 104);
    let move_a = Move::new(&debit, &credit, &sum);
    let expected = Rc::new(Move {
        book: book.clone(),
        id: 0,
        debit_account: debit.clone(),
        credit_account: credit.clone(),
        debit_account_balance: Balance(btreemap! {
            thb.clone() => -20,
            ils.clone() => -41,
            usd.clone() => -104,
        }),
        credit_account_balance: Balance(btreemap! {
            thb.clone() => 20,
            ils.clone() => 41,
            usd.clone() => 104,
        }),
        sum: sum.clone(),
    });
    assert_eq!(move_a, expected);
    assert_eq!(*debit.moves.borrow(), btreeset! { move_a.clone() });
    assert_eq!(*credit.moves.borrow(), btreeset! { move_a.clone() });
    let sum = Sum::new().unit(&thb, 13).unit(&ils, 805).unit(&usd, 10);
    let move_b = Move::new(&debit, &credit, &sum);
    assert_eq!(
        *debit.moves.borrow(),
        btreeset! {
            move_a.clone(),
            move_b.clone(),
        }
    );
    assert_eq!(
        *credit.moves.borrow(),
        btreeset! {
            move_a.clone(),
            move_b.clone(),
        }
    );
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
// TODO do not use nightly features
