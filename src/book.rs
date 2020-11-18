pub use super::accounting::{Account, Transaction};
pub use super::monetary::*;
pub use chrono::{DateTime, Utc};
use std::rc::Rc;

#[derive(Debug)]
pub struct Book {
    accounts: Vec<Rc<Account>>,
    transactions: Vec<Rc<Transaction>>,
}

/// Resembles a datastore
impl Book {
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
        from: &Rc<Account>,
        to: &Rc<Account>,
        money: Money,
    ) -> Rc<Transaction> {
        self.transfer_at(Utc::now(), from, to, money)
    }

    pub fn transfer_at(
        &mut self,
        datetime: DateTime<Utc>,
        from: &Rc<Account>,
        to: &Rc<Account>,
        money: Money,
    ) -> Rc<Transaction> {
        let transaction = Rc::new(Transaction::new(datetime, from, to, money));
        self.transactions.push(Rc::clone(&transaction));
        transaction
    }

    pub fn balance(&self, account: &Account) -> Money {
        self.balance_at(Utc::now(), account)
    }
    pub fn balance_at(&self, datetime: DateTime<Utc>, account: &Account) -> Money {
        account.balance(datetime, &self.transactions)
    }

    pub fn running_balance(&self, account: &Rc<Account>) -> Vec<(Rc<Transaction>, Money)> {
        account.running_balance(&self.transactions)
    }
}
