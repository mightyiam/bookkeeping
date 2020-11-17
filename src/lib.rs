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
// TODO could this be a single type parameter
struct Book<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    id: usize,
    meta: B,
    accounts: EntitySet<Account<B, A, U, M>>,
    units: EntitySet<Unit<B, A, U, M>>,
    moves: EntitySet<Move<B, A, U, M>>,
}
impl<B, A, U, M> Book<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn new(meta: B) -> Rc<Self> {
        Rc::new(Self {
            id: BOOK_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            meta,
            accounts: Default::default(),
            units: Default::default(),
            moves: Default::default(),
        })
    }
}
#[test]
fn book_new() {
    let book = Book::<(), (), (), ()>::new(());
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
    let other_book = Book::<(), (), (), ()>::new(());
    assert_ne!(book.id, other_book.id);
}
impl<B, A, U, M> PartialEq for Book<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn eq(&self, other: &Book<B, A, U, M>) -> bool {
        other.id == self.id
    }
}
#[test]
fn book_partial_eq() {
    let a = Rc::new(Book::<(), (), (), ()> {
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
    Account::new(&c, ());
    assert_eq!(a, c, "Same id, some different field");
    let d = Rc::new(Book {
        id: 1,
        ..Default::default()
    });
    assert_ne!(a, d, "Only id different");
}
impl<B, A, U, M> fmt::Debug for Book<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Book").field("id", &self.id).finish()
    }
}
#[test]
fn book_fmt_debug() {
    let book = Book::<(), (), (), ()>::default();
    let actual = format!("{:?}", book);
    let expected = "Book { id: 0 }";
    assert_eq!(actual, expected);
    let book = Book::<(), (), (), ()> {
        id: 1,
        ..Default::default()
    };
    let actual = format!("{:?}", book);
    let expected = "Book { id: 1 }";
    assert_eq!(actual, expected);
}
#[derive(Default)]
struct Account<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    id: EntityId,
    meta: A,
    book: Rc<Book<B, A, U, M>>,
    moves: EntitySet<Move<B, A, U, M>>,
}
impl<B, A, U, M> Account<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn new(book: &Rc<Book<B, A, U, M>>, meta: A) -> Rc<Self> {
        let account = Rc::new(Self {
            book: book.clone(),
            id: Self::next_id(&book),
            meta,
            moves: RefCell::new(BTreeSet::new()),
        });
        Self::register(&account, &book);
        account
    }
}
#[test]
fn account_new() {
    let book = Book::<(), (), (), ()>::new(());
    let account_a = Account::new(&book, ());
    assert_eq!(account_a.id, 0);
    assert_eq!(account_a.book, book);
    assert_eq!(*account_a.moves.borrow(), BTreeSet::new());
    let account_b = Account::new(&book, ());
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
impl<B, A, U, M> fmt::Debug for Account<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account").field("id", &self.id).finish()
    }
}
#[test]
fn account_fmt_debug() {
    let book = Book::<(), (), (), ()>::new(());
    let account = Account::new(&book, ());
    let actual = format!("{:?}", account);
    let expected = "Account { id: 0 }";
    assert_eq!(actual, expected);
    let account = Account::new(&book, ());
    let actual = format!("{:?}", account);
    let expected = "Account { id: 1 }";
    assert_eq!(actual, expected);
}
struct Unit<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    id: EntityId,
    meta: U,
    book: Rc<Book<B, A, U, M>>,
}
impl<B, A, U, M> Unit<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn new(book: &Rc<Book<B, A, U, M>>, meta: U) -> Rc<Self> {
        let unit = Rc::new(Self {
            id: Self::next_id(&book),
            book: book.clone(),
            meta,
        });
        Self::register(&unit, &book);
        unit
    }
}
#[test]
fn unit_new() {
    let book = Book::<(), (), (), ()>::new(());
    let unit_a = Unit::new(&book, ());
    assert_eq!(unit_a.id, 0);
    assert_eq!(unit_a.book, book);
    let unit_b = Unit::new(&book, ());
    assert_eq!(unit_b.id, 1);
    assert_eq!(unit_b.book, book);
    let expected = btreeset! {
        unit_a.clone(),
        unit_b.clone()
    };
    assert_eq!(*book.units.borrow(), expected, "Units are in the book");
}
impl<B, A, U, M> fmt::Debug for Unit<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Unit").field("id", &self.id).finish()
    }
}
#[test]
fn unit_fmt_debug() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let actual = format!("{:?}", unit);
    let expected = "Unit { id: 0 }";
    assert_eq!(actual, expected);
    let unit = Unit::new(&book, ());
    let actual = format!("{:?}", unit);
    let expected = "Unit { id: 1 }";
    assert_eq!(actual, expected);
}
#[derive(Clone, PartialEq)]
struct Sum<B, A, U, M>(BTreeMap<Rc<Unit<B, A, U, M>>, u64>)
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone;
impl<B, A, U, M> Sum<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn new() -> Self {
        Self(Default::default())
    }
    fn of(unit: &Rc<Unit<B, A, U, M>>, amount: u64) -> Self {
        Self::new().unit(&unit, amount)
    }
    fn unit(mut self, unit: &Rc<Unit<B, A, U, M>>, amount: u64) -> Self {
        self.0.insert(unit.clone(), amount);
        self
    }
    // TODO method `units`
}
#[test]
fn sum_new() {
    let actual = Sum::<(), (), (), ()>::new();
    let expected = Sum(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn sum_of() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let sum = Sum::of(&unit, 24);
    let mut expected = BTreeMap::new();
    expected.insert(unit.clone(), 24);
    assert_eq!(sum.0, expected);
}
#[test]
fn sum_unit() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let sum = Sum::new().unit(&unit, 124);
    let mut expected = BTreeMap::new();
    expected.insert(unit.clone(), 124);
    assert_eq!(sum.0, expected);
}
impl<B, A, U, M> fmt::Debug for Sum<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sum(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn sum_fmt_debug() {
    let book = Book::<(), (), (), ()>::new(());
    let unit_a = Unit::new(&book, ());
    let amount_a = 76;
    let unit_b = Unit::new(&book, ());
    let amount_b = 45;
    let sum = Sum::of(&unit_a, amount_a).unit(&unit_b, amount_b);
    let actual = format!("{:?}", sum);
    let expected = format!(
        "Sum({{{:?}: {:?}, {:?}: {:?}}})",
        unit_a, amount_a, unit_b, amount_b
    );
    assert_eq!(actual, expected);
}
#[derive(Clone, PartialEq)]
struct Balance<B, A, U, M>(BTreeMap<Rc<Unit<B, A, U, M>>, i128>)
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone;
impl<B, A, U, M> Balance<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn new() -> Self {
        Self(Default::default())
    }
    fn operation(&mut self, rhs: &Sum<B, A, U, M>, amount_op: fn(i128, u64) -> i128) {
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
    let actual = Balance::<(), (), (), ()>::new();
    let expected = Balance(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn balance_operation() {
    let mut actual = Balance::new();
    let book = Book::<(), (), (), ()>::new(());
    let unit_a = Unit::new(&book, ());
    let unit_b = Unit::new(&book, ());
    let sum = Sum::of(&unit_a, 2).unit(&unit_b, 3);
    actual.operation(&sum, |balance, amount| balance + amount as i128);
    let sum = Sum::of(&unit_a, 2).unit(&unit_b, 3);
    actual.operation(&sum, |balance, amount| balance * amount as i128);
    let expected = Balance(btreemap! {
        unit_a.clone() => 4,
        unit_b.clone() => 9,
    });
    assert_eq!(actual, expected);
}
impl<B, A, U, M> fmt::Debug for Balance<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn balance_fmt_debug() {
    let book = Book::<(), (), (), ()>::new(());
    let unit_a = Unit::new(&book, ());
    let amount_a = 76;
    let unit_b = Unit::new(&book, ());
    let amount_b = 45;
    let sum = Sum::of(&unit_a, amount_a).unit(&unit_b, amount_b);
    let balance = Balance::new() + &sum;
    let actual = format!("{:?}", balance);
    let expected = format!(
        "Balance({{{:?}: {:?}, {:?}: {:?}}})",
        unit_a, amount_a, unit_b, amount_b
    );
    assert_eq!(actual, expected);
}
impl<B, A, U, M> ops::SubAssign<&Sum<B, A, U, M>> for Balance<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn sub_assign(&mut self, sum: &Sum<B, A, U, M>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
#[test]
fn balance_sub_assign_sum() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let mut actual = Balance::new();
    actual -= &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl<B, A, U, M> ops::Sub<&Sum<B, A, U, M>> for Balance<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    type Output = Self;
    fn sub(self, sum: &Sum<B, A, U, M>) -> Self::Output {
        let mut result = self.clone();
        result -= sum;
        result
    }
}
#[test]
fn balance_sub_sum() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let balance = Balance::new();
    let actual = balance - &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl<B, A, U, M> ops::AddAssign<&Sum<B, A, U, M>> for Balance<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn add_assign(&mut self, sum: &Sum<B, A, U, M>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
#[test]
fn balance_add_assign_sum() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let mut actual = Balance::new();
    actual += &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
impl<B, A, U, M> ops::Add<&Sum<B, A, U, M>> for Balance<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    type Output = Self;
    fn add(self, sum: &Sum<B, A, U, M>) -> Self::Output {
        let mut result = self.clone();
        result += sum;
        result
    }
}
#[test]
fn balance_add_sum() {
    let book = Book::<(), (), (), ()>::new(());
    let unit = Unit::new(&book, ());
    let balance = Balance::new();
    let actual = balance + &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
#[derive(Debug)]
struct Move<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    book: Rc<Book<B, A, U, M>>,
    id: EntityId,
    meta: M,
    debit_account: Rc<Account<B, A, U, M>>,
    credit_account: Rc<Account<B, A, U, M>>,
    sum: Sum<B, A, U, M>,
}
impl<B, A, U, M> Move<B, A, U, M>
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    fn new(
        debit_account: &Rc<Account<B, A, U, M>>,
        credit_account: &Rc<Account<B, A, U, M>>,
        sum: &Sum<B, A, U, M>,
        meta: M,
    ) -> Rc<Self> {
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
        let move_ = Rc::new(Self {
            book: book.clone(),
            id: Self::next_id(&book),
            meta,
            debit_account: debit_account.clone(),
            credit_account: credit_account.clone(),
            sum: sum.clone(),
        });
        debit_account.moves.borrow_mut().insert(move_.clone());
        credit_account.moves.borrow_mut().insert(move_.clone());
        Self::register(&move_, &book);
        move_
    }
    fn balance_in(
        &self,
        account: &Rc<Account<B, A, U, M>>,
        cmp: impl Fn(&M, &M) -> Ordering,
    ) -> Balance<B, A, U, M> {
        // TODO more concise check
        if *account != self.debit_account && *account != self.credit_account {
            panic!("Provided account is not debit nor credit in this move.");
        };
        account
            .moves
            .borrow()
            .iter()
            .filter(|move_| match cmp(&self.meta, &move_.meta) {
                Ordering::Less => false,
                _ => true,
            })
            .fold(Balance::new(), |mut balance, move_| {
                // TODO deduplicate
                if move_.debit_account == *account {
                    balance -= &move_.sum;
                } else if move_.credit_account == *account {
                    balance += &move_.sum;
                }
                balance
            })
    }
}
#[test]
fn move_balance_in() {
    let cmp = |a: &u8, b: &u8| a.cmp(&b);
    let book = Book::<(), (), (), u8>::new(());
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
    let debit = Account::<(), (), (), ()>::new(&Book::new(()), ());
    let credit = Account::new(&Book::new(()), ());
    Move::new(&debit, &credit, &Sum::new(), ());
}
#[test]
#[should_panic(expected = "Debit and credit accounts are the same.")]
fn move_new_panic_debit_and_credit_accounts_are_the_same() {
    let book = Book::<(), (), (), ()>::new(());
    let account = Account::new(&book, ());
    Move::new(&account, &account, &Sum::new(), ());
}
#[test]
#[should_panic(expected = "Some unit is not in the same book as accounts.")]
fn move_new_panic_some_unit_is_not_in_the_same_book_as_accounts() {
    let book = Book::<(), (), (), ()>::new(());
    let debit = Account::new(&book, ());
    let credit = Account::new(&book, ());
    let unit = Unit::new(&Book::new(()), ());
    let sum = Sum::of(&unit, 0);
    Move::new(&debit, &credit, &sum, ());
}
#[test]
fn move_new() {
    let book = Book::<(), (), (), ()>::new(());
    let debit = Account::new(&book, ());
    let credit = Account::new(&book, ());
    let thb = Unit::new(&book, ());
    let ils = Unit::new(&book, ());
    let usd = Unit::new(&book, ());
    let sum = Sum::of(&thb, 20).unit(&ils, 41).unit(&usd, 104);
    let move_a = Move::new(&debit, &credit, &sum, ());
    let expected = Rc::new(Move {
        book: book.clone(),
        id: 0,
        meta: (),
        debit_account: debit.clone(),
        credit_account: credit.clone(),
        sum: sum.clone(),
    });
    assert_eq!(move_a, expected);
    assert_eq!(*debit.moves.borrow(), btreeset! { move_a.clone() });
    assert_eq!(*credit.moves.borrow(), btreeset! { move_a.clone() });
    let sum = Sum::of(&thb, 13).unit(&ils, 805).unit(&usd, 10);
    let move_b = Move::new(&debit, &credit, &sum, ());
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
    impl<B, A, U, M> Entity<B, A, U, M>
    where
        B: Clone,
        A: Clone,
        U: Clone,
        M: Clone,
    {
        fn next_id(book: &Book<B, A, U, M>) -> EntityId {
            book.book_field.borrow().len()
        }
        fn register(entity: &Rc<Self>, book: &Book<B, A, U, M>) {
            book.book_field.borrow_mut().insert(entity.clone());
        }
    }
    impl<B, A, U, M> Ord for Entity<B, A, U, M>
    where
        B: Clone,
        A: Clone,
        U: Clone,
        M: Clone,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            self.id.cmp(&other.id)
        }
    }
    impl<B, A, U, M> PartialOrd for Entity<B, A, U, M>
    where
        B: Clone,
        A: Clone,
        U: Clone,
        M: Clone,
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }
    impl<B, A, U, M> PartialEq for Entity<B, A, U, M>
    where
        B: Clone,
        A: Clone,
        U: Clone,
        M: Clone,
    {
        fn eq(&self, other: &Self) -> bool {
            other.book == self.book && other.id == self.id
        }
    }
    impl<B, A, U, M> Eq for Entity<B, A, U, M>
    where
        B: Clone,
        A: Clone,
        U: Clone,
        M: Clone,
    {}
}
// TODO do not use nightly features
