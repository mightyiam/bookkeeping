use crate::account::Account;
use crate::book::Book;
use crate::move_::Move;
use crate::unit::Unit;
use duplicate::duplicate;
use std::cell::Ref;
pub trait Metadata: Clone {
    type Book;
    type Account;
    type Unit;
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
pub type BlankMetadata = ((), (), (), ());
#[duplicate(Entity; [Book]; [Account]; [Unit]; [Move])]
impl<T: Metadata> Entity<T> {
    pub fn set_metadata(&self, meta: T::Entity) {
        *self.meta.borrow_mut() = meta;
    }
    pub fn get_metadata(&self) -> Ref<T::Entity> {
        self.meta.borrow()
    }
}
