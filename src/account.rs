/// Represents an [account](https://en.wikipedia.org/wiki/Account_(bookkeeping)).
#[derive(Debug, PartialEq)]
pub struct Account<A> {
    pub(crate) meta: A,
}
impl<A> Account<A> {
    pub(crate) fn new(meta: A) -> Self {
        Self { meta }
    }
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
        &self.meta
    }
}
#[cfg(test)]
mod test {
    use super::Account;
    #[test]
    fn metadata() {
        let account = Account::new(5);
        assert_eq!(*account.metadata(), 5);
    }
}
