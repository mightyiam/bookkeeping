use crate::book::{Book, EntityId, Index};
use crate::metadata::{BlankMetadata, Metadata};
use std::fmt;
use std::rc::Rc;
pub struct Unit<T: Metadata> {
    pub(crate) id: EntityId,
    meta: T::Unit,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Unit<T> {
    pub fn new(book: &Book<T>, meta: T::Unit) -> Rc<Self> {
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
    use maplit::btreeset;
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
