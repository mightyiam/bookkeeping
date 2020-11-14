pub mod accounting;
pub mod monetary;

pub use accounting::*;
pub use chrono::{DateTime, Utc};
pub use monetary::*;
use std::rc::Rc;

pub struct Book<'a> {
    accounts: Vec<Rc<Account>>,
    transactions: Vec<Rc<Transaction<'a>>>,
}

/// Resembles a datastore
impl<'a> Book<'a> {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn new_account(&mut self, name: &str) -> Rc<Account> {
        let account = Rc::new(Account::new(name));
        self.accounts.push(Rc::clone(&account));
        account
    }

    pub fn accounts(&self) -> Vec<Rc<Account>> {
        self.accounts.clone()
    }
    pub fn transfer(
        &mut self,
        from: &'a Account,
        to: &'a Account,
        money: monetary::Money<'a>,
    ) -> Rc<Transaction<'a>> {
        self.transfer_at(Utc::now(), from, to, money)
    }

    pub fn transfer_at(
        &mut self,
        datetime: DateTime<Utc>,
        from: &'a Account,
        to: &'a Account,
        money: monetary::Money<'a>,
    ) -> Rc<Transaction<'a>> {
        let transaction = Rc::new(Transaction::new(datetime, from, to, money));
        self.transactions.push(Rc::clone(&transaction));
        transaction
    }

    pub fn balance(&self, account: &Account) -> Money<'a> {
        self.balance_at(Utc::now(), account)
    }
    pub fn balance_at(&self, datetime: DateTime<Utc>, account: &Account) -> Money<'a> {
        account.balance(
            datetime,
            &self.transactions.iter().map(Rc::as_ref).collect::<Vec<_>>(),
        )
    }
}
