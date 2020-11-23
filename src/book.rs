use crate::account::Account;
use crate::index::Index;
use crate::metadata::Metadata;
use crate::move_::Move;
use crate::sum::Sum;
use crate::unit::Unit;
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
    /// Creates a new account.
    pub fn new_account(&mut self, meta: T::Account) -> Rc<Account<T>> {
        let account = Account::new(self.index.accounts.borrow().len(), &self.index, meta);
        self.index.accounts.borrow_mut().insert(account.clone());
        account
    }
    /// Creates a new unit.
    pub fn new_unit(&mut self, meta: T::Unit) -> Rc<Unit<T>> {
        let unit = Unit::new(self.index.units.borrow().len(), &self.index, meta);
        self.index.units.borrow_mut().insert(unit.clone());
        unit
    }
    /// Creates a new move.
    ///
    /// ## Panics
    ///
    /// - `debit_account` and `credit_account` are in different books.
    /// - `debit_account` and `credit_account` are the same.
    /// - Some [Unit][crate::Unit] in the [Sum] is not in the same book as the accounts.
    pub fn new_move(
        &mut self,
        debit_account: &Rc<Account<T>>,
        credit_account: &Rc<Account<T>>,
        sum: &Sum<T>,
        meta: T::Move,
    ) -> Rc<Move<T>> {
        assert_eq!(
            self.index, debit_account.index,
            "Debit account is not in the book."
        );
        assert_eq!(
            self.index, credit_account.index,
            "Credit account is not in the book."
        );
        sum.0.keys().for_each(|unit| {
            assert!(
                self.index.units.borrow().contains(unit),
                "Some unit is not in the same book as accounts."
            );
        });
        let move_ = Move::new(
            self.index.moves.borrow().len(),
            &self.index,
            debit_account,
            credit_account,
            sum,
            meta,
        );
        self.index.moves.borrow_mut().insert(move_.clone());
        move_
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
    use crate::metadata::BlankMetadata;
    use crate::sum::Sum;
    use std::cell::RefCell;
    use std::mem;
    #[test]
    fn new() {
        let book = Book::<(u8, (), (), ())>::new(77);
        assert_eq!(*book.meta.borrow(), 77);
        assert_ne!(book, Book::new(77));
    }
    #[test]
    fn new_account() {
        use maplit::btreeset;
        let mut book = Book::<BlankMetadata>::new(());
        let account_a = book.new_account(());
        let account_b = book.new_account(());
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
    #[test]
    fn new_unit() {
        use maplit::btreeset;
        let mut book = Book::<BlankMetadata>::new(());
        let unit_a = book.new_unit(());
        let unit_b = book.new_unit(());
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
    #[test]
    #[should_panic(expected = "Debit account is not in the book.")]
    fn move_new_panic_debit_account_is_not_in_the_book() {
        let mut book = Book::<BlankMetadata>::new(());
        let debit = Book::new(()).new_account(());
        let credit = book.new_account(());
        book.new_move(&debit, &credit, &Sum::new(), ());
    }
    #[test]
    #[should_panic(expected = "Credit account is not in the book.")]
    fn move_new_panic_credit_account_is_not_in_the_book() {
        let mut book = Book::<BlankMetadata>::new(());
        let debit = book.new_account(());
        let credit = Book::new(()).new_account(());
        book.new_move(&debit, &credit, &Sum::new(), ());
    }
    #[test]
    #[should_panic(expected = "Some unit is not in the same book as accounts.")]
    fn move_new_panic_some_unit_is_not_in_the_same_book_as_accounts() {
        let mut book = Book::<BlankMetadata>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        let unit = Book::new(()).new_unit(());
        let sum = Sum::of(&unit, 0);
        book.new_move(&debit, &credit, &sum, ());
    }
    #[test]
    fn new_move() {
        use maplit::btreeset;
        let mut book = Book::<BlankMetadata>::new(());
        let debit = book.new_account(());
        let credit = book.new_account(());
        let sum = Sum::new();
        let move_a = book.new_move(&debit, &credit, &sum, ());
        let sum = Sum::new();
        let move_b = book.new_move(&debit, &credit, &sum, ());
        assert_eq!(
            *book.index.moves.borrow(),
            btreeset! { move_a.clone(), move_b.clone() }
        );
    }
    #[test]
    fn drop() {
        use std::rc::Rc;
        let mut book = Book::<BlankMetadata>::new(());
        assert_eq!(Rc::strong_count(&book.index), 1, "book");
        let account_a = book.new_account(());
        assert_eq!(Rc::strong_count(&account_a), 2, "account_a, book");
        assert_eq!(Rc::strong_count(&book.index), 2, "book, account_a");
        let account_b = book.new_account(());
        assert_eq!(Rc::strong_count(&account_b), 2, "account_b, book");
        assert_eq!(
            Rc::strong_count(&book.index),
            3,
            "book, account_a, account_b"
        );
        let unit = book.new_unit(());
        assert_eq!(Rc::strong_count(&unit), 2, "unit, book");
        assert_eq!(
            Rc::strong_count(&book.index),
            4,
            "book, account_a, account_b, unit"
        );
        assert_eq!(Rc::strong_count(&unit), 2, "unit, book");
        let move_ = book.new_move(&account_a, &account_b, &Sum::of(&unit, 0), ());
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
