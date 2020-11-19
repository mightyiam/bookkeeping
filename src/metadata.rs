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
