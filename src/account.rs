/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<A> {
    pub(crate) metadata: A,
}
impl<A> Account<A> {
    /// Gets the metadata of the account.
    pub fn metadata(&self) -> &A {
        &self.metadata
    }
}
#[cfg(test)]
mod test {
    use super::Account;
    #[test]
    fn metadata() {
        let account = Account { metadata: 5 };
        assert_eq!(*account.metadata(), 5);
    }
}
