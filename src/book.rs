use super::accounting::{Account, AccountId, Transaction};
pub use super::monetary::*;
pub use chrono::{DateTime, Utc};
pub use std::result::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct Book {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

/// Resembles a datastore
impl Book {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn create_account(&mut self, name: &str) -> Result<AccountId, CreateAccountError> {
        let acc_id = name.into();
        if self.account_exists(&acc_id) {
            Err(CreateAccountError::AlreadyExists(acc_id))
        } else {
            self.accounts.push(Account::new(acc_id.clone()));
            Ok(acc_id)
        }
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.iter()
    }

    pub fn lookup_account(&self, id: &AccountId) -> Result<&Account, LookupAccountError> {
        self.accounts
            .iter()
            .find(|acc| acc.id() == *id)
            .ok_or(LookupAccountError::DoesNotExist(id.clone()))
    }

    pub fn transfer(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        money: Money,
    ) -> Result<(), TransferError> {
        self.transfer_at(Utc::now(), from, to, money)
    }

    pub fn transfer_at(
        &mut self,
        datetime: DateTime<Utc>,
        from: &AccountId,
        to: &AccountId,
        money: Money,
    ) -> Result<(), TransferError> {
        let from = self
            .lookup_account(from)
            .map_err(TransferError::CannotTransferFromAccount)?
            .id();
        let to = self
            .lookup_account(to)
            .map_err(TransferError::CannotTransferToAccount)?
            .id();
        self.transactions
            .push(Transaction::new(datetime, from, to, money));
        Ok(())
    }

    pub fn balance(&self, acc: &AccountId) -> Result<Money, LookupAccountError> {
        self.balance_at(Utc::now(), acc)
    }

    pub fn balance_at(
        &self,
        datetime: DateTime<Utc>,
        acc: &AccountId,
    ) -> Result<Money, LookupAccountError> {
        self.lookup_account(acc)
            .map(|acc| acc.balance(datetime, &self.transactions))
    }

    pub fn running_balance(
        &self,
        id: &AccountId,
    ) -> Result<Vec<(&Transaction, Money)>, LookupAccountError> {
        self.lookup_account(id)
            .map(|acc| acc.running_balance(&self.transactions))
    }

    fn account_exists(&self, acc: &AccountId) -> bool {
        self.accounts.iter().any(|account| account.id() == *acc)
    }
}

#[derive(Error, Debug)]
pub enum CreateAccountError {
    #[error("account \"{0}\" already exists")]
    AlreadyExists(AccountId),
}

#[derive(Error, Debug)]

pub enum LookupAccountError {
    #[error("account \"{0}\" doesn't exist")]
    DoesNotExist(AccountId),
}

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("cannot transfer from account, {0}")]
    CannotTransferFromAccount(#[source] LookupAccountError),
    #[error("cannot transfer to account, {0}")]
    CannotTransferToAccount(#[source] LookupAccountError),
}
