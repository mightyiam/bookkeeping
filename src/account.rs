/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<AccountMetadata> {
    pub(crate) metadata: AccountMetadata,
}
impl<AccountMetadata> Account<AccountMetadata> {
    /// Gets the metadata of the account.
    pub fn metadata(&self) -> &AccountMetadata {
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
