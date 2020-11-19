use crate::account::Account;
use crate::metadata::{BlankMetadata, Metadata};
use crate::move_::Move;
use crate::sum::Sum;
use crate::unit::Unit;
use duplicate::duplicate_inline;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::rc::Rc;
use std::sync::{atomic, atomic::AtomicUsize};
static INDEX_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub type EntityId = usize;
#[derive(Default)]
pub struct Index<T: Metadata> {
    pub(crate) id: usize,
    pub(crate) accounts: RefCell<BTreeSet<Rc<Account<T>>>>,
    pub(crate) units: RefCell<BTreeSet<Rc<Unit<T>>>>,
    pub(crate) moves: RefCell<BTreeSet<Rc<Move<T>>>>,
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
pub struct Book<T: Metadata> {
    meta: T::Book,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Book<T> {
    pub fn new(meta: T::Book) -> Self {
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
duplicate_inline! {
    [
        Entity index_field;
        [Account] [accounts];
        [Unit] [units];
        [Move] [moves];
    ]
    impl<T: Metadata> Entity<T> {
        pub(crate) fn next_id(index: &Index<T>) -> EntityId {
            index.index_field.borrow().len()
        }
        pub(crate) fn register(entity: &Rc<Self>, index: &Index<T>) {
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
