use crate::book::Book;
use crate::index::{EntityId, Index};
use crate::metadata::Metadata;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<T: Metadata> {
    pub(crate) id: EntityId,
    pub(crate) meta: RefCell<T::Account>,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Account<T> {
    /// Creates a new account.
    pub fn new(book: &Book<T>, meta: T::Account) -> Rc<Self> {
        let account = Rc::new(Self {
            index: book.index.clone(),
            id: Self::next_id(&book.index),
            meta: RefCell::new(meta),
        });
        Self::register(&account, &book.index);
        account
    }
}
impl<T: Metadata> fmt::Debug for Account<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account").field("id", &self.id).finish()
    }
}
#[cfg(test)]
mod test {
    use super::Account;
    use super::Book;
    use crate::metadata::BlankMetadata;
    #[test]
    fn new() {
        use maplit::btreeset;
        let book = Book::<((), u8, (), ())>::new(());
        let account_a = Account::new(&book, 9);
        assert_eq!(account_a.id, 0);
        assert_eq!(account_a.index, book.index);
        assert_eq!(*account_a.meta.borrow(), 9);
        let account_b = Account::new(&book, 4);
        assert_eq!(account_b.id, 1);
        assert_eq!(account_b.index, book.index);
        assert_eq!(*account_b.meta.borrow(), 4);
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
    fn fmt_debug() {
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
    #[test]
    fn metadata() {
        let book = Book::<((), u8, (), ())>::new(());
        let account = Account::new(&book, 3);
        assert_eq!(*account.get_metadata(), 3);
        account.set_metadata(9);
        assert_eq!(*account.get_metadata(), 9);
    }
}
