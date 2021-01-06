/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<Meta> {
    pub(crate) metadata: Meta,
}
impl<Meta> Account<Meta> {
    /// Gets the metadata of the account.
    pub fn metadata(&self) -> &Meta {
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
