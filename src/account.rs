use crate::book::Book;
use crate::index::{EntityId, Index};
use crate::metadata::Metadata;
use std::fmt;
use std::rc::Rc;
pub struct Account<T: Metadata> {
    pub(crate) id: EntityId,
    meta: T::Account,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Account<T> {
    pub fn new(book: &Book<T>, meta: T::Account) -> Rc<Self> {
        let account = Rc::new(Self {
            index: book.index.clone(),
            id: Self::next_id(&book.index),
            meta,
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
    fn account_new() {
        use maplit::btreeset;
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
}
