use crate::index::Index;
use crate::metadata::Metadata;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
/// Entry point to the API and retains ownership of accounts, units and moves.
///
/// A reference to a book is an argument in any call to create a new account, unit or move.
/// The new entity is both registered in the book and returned in an [std::rc::Rc].
/// Since the book retains an `Rc` of that entity, the returned `Rc` may be dropped.
#[derive(Default)]
pub struct Book<T: Metadata> {
    pub(crate) meta: RefCell<T::Book>,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Book<T> {
    /// Creates a new book
    pub fn new(meta: T::Book) -> Self {
        Self {
            meta: RefCell::new(meta),
            index: Index::new(),
        }
    }
}
impl<T: Metadata> Drop for Book<T> {
    fn drop(&mut self) {
        self.index.accounts.borrow_mut().clear();
        self.index.units.borrow_mut().clear();
        self.index.moves.borrow_mut().clear();
    }
}
impl<T: Metadata> PartialEq for Book<T> {
    fn eq(&self, other: &Book<T>) -> bool {
        other.index == self.index
    }
}
impl<T: Metadata> fmt::Debug for Book<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Book").field("index", &self.index).finish()
    }
}
#[cfg(test)]
mod test {
    use super::Book;
    use super::Index;
    use super::Rc;
    use crate::account::Account;
    use crate::metadata::BlankMetadata;
    use crate::move_::Move;
    use crate::sum::Sum;
    use crate::unit::Unit;
    use std::cell::RefCell;
    use std::mem;
    #[test]
    fn new() {
        let book = Book::<(u8, (), (), ())>::new(77);
        assert_eq!(*book.meta.borrow(), 77);
        assert_ne!(book, Book::new(77));
    }
    #[test]
    fn drop() {
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
        mem::drop(book);
        assert_eq!(Rc::strong_count(&account_a), 2, "account_a, move_");
        assert_eq!(Rc::strong_count(&account_b), 2, "account_b, move_");
        assert_eq!(Rc::strong_count(&unit), 2, "unit, move_.sum");
        assert_eq!(Rc::strong_count(&move_), 1, "move_");
        mem::drop(move_);
        assert_eq!(Rc::strong_count(&account_a), 1, "account_a");
        assert_eq!(Rc::strong_count(&account_b), 1, "account_b");
        assert_eq!(Rc::strong_count(&unit), 1, "unit");
    }
    #[test]
    fn partial_eq() {
        let index_0 = Rc::new(Index {
            id: 0,
            ..Default::default()
        });
        let a = Book::<(u8, (), (), ())> {
            meta: RefCell::new(0),
            index: index_0.clone(),
            ..Default::default()
        };
        let b = Book::<(u8, (), (), ())> {
            meta: RefCell::new(0),
            index: index_0.clone(),
        };
        assert_eq!(a, b, "All fields equal");
        let c = Book {
            meta: RefCell::new(1),
            index: index_0.clone(),
        };
        assert_eq!(a, c, "Same index, some different field");
        let d = Book {
            meta: RefCell::new(0),
            index: Rc::new(Index {
                id: 1,
                ..Default::default()
            }),
        };
        assert_ne!(a, d, "Only id different");
    }
    #[test]
    fn fmt_debug() {
        let book = Book::<BlankMetadata>::default();
        let actual = format!("{:?}", book);
        let expected = format!("Book {{ index: {:?} }}", book.index);
        assert_eq!(actual, expected);
    }
    #[test]
    fn metadata() {
        let book = Book::<(u8, (), (), ())>::new(3);
        assert_eq!(*book.get_metadata(), 3);
        book.set_metadata(20);
        assert_eq!(*book.get_metadata(), 20);
        book.set_metadata(9);
        assert_eq!(*book.get_metadata(), 9);
    }
}
