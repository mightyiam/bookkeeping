use crate::index::{EntityId, Index};
use crate::metadata::Metadata;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
pub struct Unit<T: Metadata> {
    pub(crate) id: EntityId,
    pub(crate) meta: RefCell<T::Unit>,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Unit<T> {
    /// Creates a new unit.
    pub(crate) fn new(id: EntityId, index: &Rc<Index<T>>, meta: T::Unit) -> Rc<Self> {
        let unit = Rc::new(Self {
            id,
            index: index.clone(),
            meta: RefCell::new(meta),
        });
        unit
    }
}
impl<T: Metadata> fmt::Debug for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Unit").field("id", &self.id).finish()
    }
}
#[cfg(test)]
mod test {
    use super::Index;
    use super::Unit;
    use crate::metadata::BlankMetadata;
    #[test]
    fn new() {
        let index = Index::<((), (), u8, ())>::new();
        let unit_a = Unit::new(0, &index, 50);
        assert_eq!(unit_a.id, 0);
        assert_eq!(unit_a.index, index);
        assert_eq!(*unit_a.meta.borrow(), 50);
        let unit_b = Unit::new(1, &index, 40);
        assert_eq!(unit_b.id, 1);
        assert_eq!(unit_b.index, index);
        assert_eq!(*unit_b.meta.borrow(), 40);
    }
    #[test]
    fn fmt_debug() {
        let index = Index::<BlankMetadata>::new();
        let unit = Unit::new(0, &index, ());
        let actual = format!("{:?}", unit);
        let expected = "Unit { id: 0 }";
        assert_eq!(actual, expected);
        let unit = Unit::new(1, &index, ());
        let actual = format!("{:?}", unit);
        let expected = "Unit { id: 1 }";
        assert_eq!(actual, expected);
    }
    #[test]
    fn metadata() {
        let index = Index::<((), (), u8, ())>::new();
        let unit = Unit::new(0, &index, 3);
        assert_eq!(*unit.get_metadata(), 3);
        unit.set_metadata(9);
        assert_eq!(*unit.get_metadata(), 9);
    }
}
