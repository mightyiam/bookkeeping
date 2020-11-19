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
static INDEX_COUNTER: AtomicUsize = AtomicUsize::new(0);
trait Metadata: Clone {
    type Book;
    type Account;
    type Unit;
    type Move;
}
impl<B, A, U, M> Metadata for (B, A, U, M)
where
    B: Clone,
    A: Clone,
    U: Clone,
    M: Clone,
{
    type Book = B;
    type Account = A;
    type Unit = U;
    type Move = M;
}
type BlankMetadata = ((), (), (), ());
#[derive(Default)]
struct Index<T: Metadata> {
    id: usize,
    accounts: RefCell<BTreeSet<Rc<Account<T>>>>,
    units: RefCell<BTreeSet<Rc<Unit<T>>>>,
    moves: RefCell<BTreeSet<Rc<Move<T>>>>,
}
impl<T: Metadata> Index<T> {
    fn new() -> Rc<Self> {
        Rc::new(Index {
            id: INDEX_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            accounts: Default::default(),
            units: Default::default(),
            moves: Default::default(),
        })
    }
}
#[test]
fn index_new() {
    let index = Index::<BlankMetadata>::new();
    assert_ne!(index.id, Index::<BlankMetadata>::new().id);
    assert_eq!(index.accounts, Default::default());
    assert_eq!(index.units, Default::default());
    assert_eq!(index.moves, Default::default());
}
impl<T: Metadata> PartialEq for Index<T> {
    fn eq(&self, other: &Index<T>) -> bool {
        other.id == self.id
    }
}
#[test]
fn index_partial_eq() {
    let index = Index::<BlankMetadata> {
        id: 0,
        ..Default::default()
    };
    assert_eq!(
        index,
        Index {
            id: 0,
            ..Default::default()
        },
        "All fields equal",
    );
    let book = Book {
        index: Default::default(),
        meta: (),
    };
    Account::new(&book, ());
    assert_eq!(index, *book.index, "id same, some fields different");
    assert_ne!(
        index,
        Index {
            id: 1,
            ..Default::default()
        },
        "id different, other fields equal"
    );
}
impl<T: Metadata> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Index").field("id", &self.id).finish()
    }
}
#[test]
fn index_fmt_debug() {
    let index = Index::<BlankMetadata>::default();
    let actual = format!("{:?}", index);
    let expected = "Index { id: 0 }";
    assert_eq!(actual, expected);
    let index = Index::<BlankMetadata> {
        id: 1,
        ..Default::default()
    };
    let actual = format!("{:?}", index);
    let expected = "Index { id: 1 }";
    assert_eq!(actual, expected);
}
#[derive(Default)]
struct Book<T: Metadata> {
    meta: T::Book,
    index: Rc<Index<T>>,
}
impl<T: Metadata> Book<T> {
    fn new(meta: T::Book) -> Self {
        Self {
            meta,
            index: Index::new(),
        }
    }
}
#[test]
fn book_new() {
    let book = Book::<(u8, (), (), ())>::new(77);
    assert_eq!(book.meta, 77);
    assert_ne!(book, Book::new(77));
}
impl<T: Metadata> Drop for Book<T> {
    fn drop(&mut self) {
        self.index.accounts.borrow_mut().clear();
        self.index.units.borrow_mut().clear();
        self.index.moves.borrow_mut().clear();
    }
}
#[test]
fn book_drop() {
    use std::rc::Rc;
    let book = Book::<BlankMetadata>::new(());
    assert_eq!(Rc::strong_count(&book.index), 1, "book");
    let account_a = Account::new(&book, ());
    assert_eq!(Rc::strong_count(&account_a), 2, "account_a, book");
    assert_eq!(Rc::strong_count(&book.index), 2, "book, account_a");
    let account_b = Account::new(&book, ());
    assert_eq!(Rc::strong_count(&account_b), 2, "account_b, book");
    assert_eq!(
        Rc::strong_count(&book.index),
        3,
        "book, account_a, account_b"
    );
    let unit = Unit::new(&book, ());
    assert_eq!(Rc::strong_count(&unit), 2, "unit, book");
    assert_eq!(
        Rc::strong_count(&book.index),
        4,
        "book, account_a, account_b, unit"
    );
    assert_eq!(Rc::strong_count(&unit), 2, "unit, book");
    let move_ = Move::new(&account_a, &account_b, &Sum::of(&unit, 0), ());
    assert_eq!(Rc::strong_count(&move_), 2, "move, book");
    assert_eq!(
        Rc::strong_count(&book.index),
        5,
        "book, account_a, account_b, unit, move_"
    );
    assert_eq!(Rc::strong_count(&account_a), 3, "account_a, book, move_");
    assert_eq!(Rc::strong_count(&account_b), 3, "account_b, book, move_");
    assert_eq!(Rc::strong_count(&unit), 3, "unit, book, move_.sum");
    drop(book);
    assert_eq!(Rc::strong_count(&account_a), 2, "account_a, move_");
    assert_eq!(Rc::strong_count(&account_b), 2, "account_b, move_");
    assert_eq!(Rc::strong_count(&unit), 2, "unit, move_.sum");
    assert_eq!(Rc::strong_count(&move_), 1, "move_");
    drop(move_);
    assert_eq!(Rc::strong_count(&account_a), 1, "account_a");
    assert_eq!(Rc::strong_count(&account_b), 1, "account_b");
    assert_eq!(Rc::strong_count(&unit), 1, "unit");
}
impl<T: Metadata> PartialEq for Book<T> {
    fn eq(&self, other: &Book<T>) -> bool {
        other.index == self.index
    }
}
#[test]
fn book_partial_eq() {
    let index_0 = Rc::new(Index {
        id: 0,
        ..Default::default()
    });
    let a = Book::<(u8, (), (), ())> {
        meta: 0,
        index: index_0.clone(),
        ..Default::default()
    };
    let b = Book::<(u8, (), (), ())> {
        meta: 0,
        index: index_0.clone(),
    };
    assert_eq!(a, b, "All fields equal");
    let c = Book {
        meta: 1,
        index: index_0.clone(),
    };
    assert_eq!(a, c, "Same index, some different field");
    let d = Book {
        meta: 0,
        index: Rc::new(Index {
            id: 1,
            ..Default::default()
        }),
    };
    assert_ne!(a, d, "Only id different");
}
impl<T: Metadata> fmt::Debug for Book<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Book").field("index", &self.index).finish()
    }
}
#[test]
fn book_fmt_debug() {
    let book = Book::<BlankMetadata>::default();
    let actual = format!("{:?}", book);
    let expected = format!("Book {{ index: {:?} }}", book.index);
    assert_eq!(actual, expected);
}
struct Account<T: Metadata> {
    id: EntityId,
    meta: T::Account,
    index: Rc<Index<T>>,
}
impl<T: Metadata> Account<T> {
    fn new(book: &Book<T>, meta: T::Account) -> Rc<Self> {
        let account = Rc::new(Self {
            index: book.index.clone(),
            id: Self::next_id(&book.index),
            meta,
        });
        Self::register(&account, &book.index);
        account
    }
}
#[test]
fn account_new() {
    let book = Book::<((), u8, (), ())>::new(());
    let account_a = Account::new(&book, 9);
    assert_eq!(account_a.id, 0);
    assert_eq!(account_a.index, book.index);
    assert_eq!(account_a.meta, 9);
    let account_b = Account::new(&book, 4);
    assert_eq!(account_b.id, 1);
    assert_eq!(account_b.index, book.index);
    assert_eq!(account_b.meta, 4);
    let expected = btreeset! {
        account_a.clone(),
        account_b.clone()
    };
    assert_eq!(
        *book.index.accounts.borrow(),
        expected,
        "Accounts are in the book"
    );
}
impl<T: Metadata> fmt::Debug for Account<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account").field("id", &self.id).finish()
    }
}
#[test]
fn account_fmt_debug() {
    let book = Book::<BlankMetadata>::new(());
    let account = Account::new(&book, ());
    let actual = format!("{:?}", account);
    let expected = "Account { id: 0 }";
    assert_eq!(actual, expected);
    let account = Account::new(&book, ());
    let actual = format!("{:?}", account);
    let expected = "Account { id: 1 }";
    assert_eq!(actual, expected);
}
struct Unit<T: Metadata> {
    id: EntityId,
    meta: T::Unit,
    index: Rc<Index<T>>,
}
impl<T: Metadata> Unit<T> {
    fn new(book: &Book<T>, meta: T::Unit) -> Rc<Self> {
        let unit = Rc::new(Self {
            id: Self::next_id(&book.index),
            index: book.index.clone(),
            meta,
        });
        Self::register(&unit, &book.index);
        unit
    }
}
#[test]
fn unit_new() {
    let book = Book::<((), (), u8, ())>::new(());
    let unit_a = Unit::new(&book, 50);
    assert_eq!(unit_a.id, 0);
    assert_eq!(unit_a.index, book.index);
    assert_eq!(unit_a.meta, 50);
    let unit_b = Unit::new(&book, 40);
    assert_eq!(unit_b.id, 1);
    assert_eq!(unit_b.index, book.index);
    assert_eq!(unit_b.meta, 40);
    let expected = btreeset! {
        unit_a.clone(),
        unit_b.clone()
    };
    assert_eq!(
        *book.index.units.borrow(),
        expected,
        "Units are in the book"
    );
}
impl<T: Metadata> fmt::Debug for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Unit").field("id", &self.id).finish()
    }
}
#[test]
fn unit_fmt_debug() {
    let book = Book::<BlankMetadata>::new(());
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
struct Sum<T: Metadata>(BTreeMap<Rc<Unit<T>>, u64>);
impl<T: Metadata> Sum<T> {
    fn new() -> Self {
        Self(Default::default())
    }
    fn of(unit: &Rc<Unit<T>>, amount: u64) -> Self {
        Self::new().unit(&unit, amount)
    }
    fn unit(mut self, unit: &Rc<Unit<T>>, amount: u64) -> Self {
        self.0.insert(unit.clone(), amount);
        self
    }
    // TODO method `units`
}
#[test]
fn sum_new() {
    let actual = Sum::<BlankMetadata>::new();
    let expected = Sum(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn sum_of() {
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let sum = Sum::of(&unit, 24);
    let mut expected = BTreeMap::new();
    expected.insert(unit.clone(), 24);
    assert_eq!(sum.0, expected);
}
#[test]
fn sum_unit() {
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let sum = Sum::new().unit(&unit, 124);
    let mut expected = BTreeMap::new();
    expected.insert(unit.clone(), 124);
    assert_eq!(sum.0, expected);
}
impl<T: Metadata> fmt::Debug for Sum<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sum(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn sum_fmt_debug() {
    let book = Book::<BlankMetadata>::new(());
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
struct Balance<T: Metadata>(BTreeMap<Rc<Unit<T>>, i128>);
impl<T: Metadata> Balance<T> {
    fn new() -> Self {
        Self(Default::default())
    }
    fn operation(&mut self, rhs: &Sum<T>, amount_op: fn(i128, u64) -> i128) {
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
    let actual = Balance::<BlankMetadata>::new();
    let expected = Balance(BTreeMap::new());
    assert_eq!(actual, expected);
}
#[test]
fn balance_operation() {
    let mut actual = Balance::new();
    let book = Book::<BlankMetadata>::new(());
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
impl<T: Metadata> fmt::Debug for Balance<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Balance(")?;
        f.debug_map().entries(self.0.clone()).finish()?;
        f.write_str(")")
    }
}
#[test]
fn balance_fmt_debug() {
    let book = Book::<BlankMetadata>::new(());
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
impl<T: Metadata> ops::SubAssign<&Sum<T>> for Balance<T> {
    fn sub_assign(&mut self, sum: &Sum<T>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount - sum_amount as i128
        });
    }
}
#[test]
fn balance_sub_assign_sum() {
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let mut actual = Balance::new();
    actual -= &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl<T: Metadata> ops::Sub<&Sum<T>> for Balance<T> {
    type Output = Self;
    fn sub(self, sum: &Sum<T>) -> Self::Output {
        let mut result = self.clone();
        result -= sum;
        result
    }
}
#[test]
fn balance_sub_sum() {
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let balance = Balance::new();
    let actual = balance - &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => -9,
    });
    assert_eq!(actual, expected);
}
impl<T: Metadata> ops::AddAssign<&Sum<T>> for Balance<T> {
    fn add_assign(&mut self, sum: &Sum<T>) {
        self.operation(sum, |balance_amount, sum_amount| {
            balance_amount + sum_amount as i128
        });
    }
}
#[test]
fn balance_add_assign_sum() {
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let mut actual = Balance::new();
    actual += &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
impl<T: Metadata> ops::Add<&Sum<T>> for Balance<T> {
    type Output = Self;
    fn add(self, sum: &Sum<T>) -> Self::Output {
        let mut result = self.clone();
        result += sum;
        result
    }
}
#[test]
fn balance_add_sum() {
    let book = Book::<BlankMetadata>::new(());
    let unit = Unit::new(&book, ());
    let balance = Balance::new();
    let actual = balance + &Sum::of(&unit, 9);
    let expected = Balance(btreemap! {
        unit.clone() => 9,
    });
    assert_eq!(actual, expected);
}
#[derive(Debug)]
struct Move<T: Metadata> {
    index: Rc<Index<T>>,
    id: EntityId,
    meta: T::Move,
    debit_account: Rc<Account<T>>,
    credit_account: Rc<Account<T>>,
    sum: Sum<T>,
}
impl<T: Metadata> Move<T> {
    fn new(
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
    fn balance_in(
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
duplicate_inline! {
    [
        Entity index_field;
        [Account] [accounts];
        [Unit] [units];
        [Move] [moves];
    ]
    impl<T: Metadata> Entity<T> {
        fn next_id(index: &Index<T>) -> EntityId {
            index.index_field.borrow().len()
        }
        fn register(entity: &Rc<Self>, index: &Index<T>) {
            index.index_field.borrow_mut().insert(entity.clone());
        }
    }
    impl<T: Metadata> Ord for Entity<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.id.cmp(&other.id)
        }
    }
    impl<T: Metadata> PartialOrd for Entity<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }
    impl<T: Metadata> PartialEq for Entity<T> {
        fn eq(&self, other: &Self) -> bool {
            other.index == self.index && other.id == self.id
        }
    }
    impl<T: Metadata> Eq for Entity<T> {}
}
// TODO do not use nightly features
