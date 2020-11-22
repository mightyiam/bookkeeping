use crate::account::Account;
use crate::book::Book;
use crate::move_::Move;
use crate::unit::Unit;
use duplicate::duplicate;
use std::cell::Ref;
/// Implement this trait to specify the type of metadata for each of the bookkeeping entities.
pub trait Metadata: Clone {
    /// Metadata type for [Book](crate::Book).
    type Book;
    /// Metadata type for [Account](crate::Account).
    type Account;
    /// Metadata type for [Unit](crate::Unit).
    type Unit;
    /// Metadata type for [Move](crate::Move).
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
#[cfg(test)]
pub type BlankMetadata = ((), (), (), ());
#[duplicate(Entity; [Book]; [Account]; [Unit]; [Move])]
impl<T: Metadata> Entity<T> {
    /// Sets the metadata value on this entity.
    pub fn set_metadata(&self, meta: T::Entity) {
        *self.meta.borrow_mut() = meta;
    }
    /// Gets the metadata value on this entity.
    ///
    /// Internally, the field that holds metadata is an [std::cell::RefCell].
    /// Ideally, I would not like to leak this detail through the public API by returning [std::cell::Ref] here.
    /// Yet, I'm not sure what the alternative is.
    /// Perhaps returning a clone?
    pub fn get_metadata(&self) -> Ref<T::Entity> {
        self.meta.borrow()
    }
}
