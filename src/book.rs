use super::accounting::{Account, Transaction};
pub use super::monetary::*;
pub use chrono::{DateTime, Utc};
pub use std::result::Result;

#[derive(Debug)]
pub struct Book<'a> {
    accounts: Vec<Account<'a>>,
    transactions: Vec<Transaction<'a>>,
}

/// Resembles a datastore
impl<'a> Book<'a> {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn new_account(&mut self, name: &'a str) -> Result<(), String> {
        if self.accounts.iter().any(|acc| acc.name() == name) {
            return Err(format!("account with name \"{}\" already exists", name));
        }
        self.accounts.push(Account::new(name));
        Ok(())
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.iter()
    }

    pub fn account_with_name(&self, name: &'_ str) -> Option<&Account> {
        self.accounts.iter().find(|acc| acc.name() == name)
    }

    pub fn transfer(&'a mut self, from: &str, to: &str, money: Money<'a>) {
        /*if [from, to].iter().any(|acc| !self.account_exists(acc)) {
            panic!("some accounts do not exist");
        }*/
        let from = self.accounts.iter().find(|acc| acc.name() == from).unwrap();
        let to = self.accounts.iter().find(|acc| acc.name() == to).unwrap();
        self.transactions.push(Transaction::new(from, to, money));
    }

    pub fn balance(&'a self, account: &'a Account) -> Money<'_> {
        self.balance_at(Utc::now(), account)
    }
    pub fn balance_at(&'a self, datetime: DateTime<Utc>, account: &'a Account) -> Money<'a> {
        account.balance(datetime, self.transactions.iter())
    }

    pub fn running_balance(
        &'a self,
        account: &'a Account,
    ) -> impl Iterator<Item = (&Transaction, Money)> {
        account.running_balance(self.transactions.iter())
    }

    fn account_exists(&self, account: &Account) -> bool {
        self.accounts.contains(account)
    }
}
