use crate::account::Account;
use crate::metadata::Metadata;
use crate::move_::Move;
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
    pub(crate) fn new() -> Rc<Self> {
        Rc::new(Index {
            id: INDEX_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            accounts: Default::default(),
            units: Default::default(),
            moves: Default::default(),
        })
    }
}
impl<T: Metadata> PartialEq for Index<T> {
    fn eq(&self, other: &Index<T>) -> bool {
        other.id == self.id
    }
}
impl<T: Metadata> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Index").field("id", &self.id).finish()
    }
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
#[cfg(test)]
mod test {
    use super::Index;
    use crate::metadata::BlankMetadata;
    #[test]
    fn index_new() {
        let index = Index::<BlankMetadata>::new();
        assert_ne!(index.id, Index::<BlankMetadata>::new().id);
        assert_eq!(index.accounts, Default::default());
        assert_eq!(index.units, Default::default());
        assert_eq!(index.moves, Default::default());
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
        assert_ne!(
            index,
            Index {
                id: 1,
                ..Default::default()
            },
            "id different, other fields equal"
        );
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
}
