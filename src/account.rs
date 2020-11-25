/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<T> {
    pub(crate) meta: T,
}
impl<T> Account<T> {
    pub(crate) fn new(meta: T) -> Self {
        Self { meta }
    }
}
