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
    /// ```
    /// # use bookkeeping::Book;
    /// # use chrono::naive::NaiveDate;
    /// # struct BookMetadata { id: u8 }
    /// # #[derive(Debug, PartialEq)]
    /// # struct AccountMetadata { name: String }
    /// # struct UnitMetadata { currency_code: String }
    /// # struct MoveMetadata { date: NaiveDate }
    /// # let mut book = Book::<BookMetadata, AccountMetadata, UnitMetadata, MoveMetadata>::new(
    /// #     BookMetadata { id: 0 },
    /// # );
    /// # let account_key = book.new_account(AccountMetadata { name: String::from("Wallet") });
    /// # let account = book.get_account(account_key);
    /// assert_eq!(
    ///     account.metadata(),
    ///     &AccountMetadata { name: String::from("Wallet") },
    /// );
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
