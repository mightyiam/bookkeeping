/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
pub struct Account<AccountMetadata> {
    pub(crate) metadata: AccountMetadata,
}
impl<AccountMetadata> Account<AccountMetadata> {
    /// Gets the metadata of the account.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::*;
    /// # let mut book = Book::<(), &str, (), (), ()>::new(());
    /// # let account_key = book.new_account("wallet");
    /// # let account = book.get_account(account_key);
    /// assert_eq!(*account.metadata(), "wallet");
    /// ```
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
