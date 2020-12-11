/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
#[derive(Debug, PartialEq)]
pub struct Account<A> {
    pub(crate) metadata: A,
}
impl<A> Account<A> {
    /// Gets the metadata of the account.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let account_key = book.new_account("wallet");
    /// # let account = book.get_account(account_key);
    /// assert_eq!(*account.metadata(), "wallet");
    /// ```
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
