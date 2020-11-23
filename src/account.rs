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
    pub(crate) fn new(id: EntityId, index: &Rc<Index<T>>, meta: T::Account) -> Rc<Self> {
        let account = Rc::new(Self {
            index: index.clone(),
            id,
            meta: RefCell::new(meta),
        });
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
    use super::Index;
    use crate::metadata::BlankMetadata;
    #[test]
    fn new() {
        let index = Index::<((), u8, (), ())>::new();
        let account_a = Account::new(0, &index, 9);
        assert_eq!(account_a.id, 0);
        assert_eq!(account_a.index, index);
        assert_eq!(*account_a.meta.borrow(), 9);
        let account_b = Account::new(1, &index, 4);
        assert_eq!(account_b.id, 1);
        assert_eq!(account_b.index, index);
        assert_eq!(*account_b.meta.borrow(), 4);
    }
    #[test]
    fn fmt_debug() {
        let index = Index::<BlankMetadata>::new();
        let account = Account::new(0, &index, ());
        let actual = format!("{:?}", account);
        let expected = "Account { id: 0 }";
        assert_eq!(actual, expected);
        let account = Account::new(1, &index, ());
        let actual = format!("{:?}", account);
        let expected = "Account { id: 1 }";
        assert_eq!(actual, expected);
    }
    #[test]
    fn metadata() {
        let index = Index::<((), u8, (), ())>::new();
        let account = Account::new(0, &index, 3);
        assert_eq!(*account.get_metadata(), 3);
        account.set_metadata(9);
        assert_eq!(*account.get_metadata(), 9);
    }
}
