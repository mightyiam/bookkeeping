use super::accounting;
pub use super::monetary::*;
use accounting::Account;
pub use chrono::{DateTime, Utc};
use derive_more::Display;
use std::collections::HashMap;
use std::ops::Deref;
pub use std::result::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct Book {
    accounts: HashMap<AccountId, Account>,
    transactions: Vec<Transaction>,
}

/// Resembles a datastore
impl Book {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    pub fn create_account(&mut self, name: &str) -> Result<AccountId, CreateAccountError> {
        let acc_id = name.into();
        if self.account_exists(&acc_id) {
            Err(CreateAccountError::AlreadyExists(acc_id))
        } else {
            self.accounts.insert(acc_id.clone(), Account::new());
            Ok(acc_id)
        }
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    pub fn lookup_account(&self, id: &AccountId) -> Result<&Account, LookupAccountError> {
        self.accounts
            .get(id)
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
        self.lookup_account(from)
            .map_err(TransferError::CannotTransferFromAccount)?;
        self.lookup_account(to)
            .map_err(TransferError::CannotTransferToAccount)?;
        self.transactions.push(Transaction {
            datetime,
            from: from.clone(),
            to: to.clone(),
            money,
        });
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
            .map(|acc| acc.balance(datetime, &self.accounting_transactions()))
    }

    pub fn running_balance<'a>(
        &'a self,
        id: &'a AccountId,
    ) -> Result<impl Iterator<Item = (&Transaction, Money)> + 'a, LookupAccountError> {
        self.lookup_account(id).map(move |_| {
            self.transactions.iter().filter_map(move |tx| {
                if tx.to == *id || tx.from == *id {
                    Some((tx, self.balance_at(tx.datetime, id).unwrap()))
                } else {
                    None
                }
            })
        })
    }

    fn account_exists(&self, acc: &AccountId) -> bool {
        self.accounts.contains_key(acc)
    }

    fn accounting_transactions(&self) -> Vec<accounting::Transaction> {
        self.transactions
            .iter()
            .map(|tx| {
                self.lookup_account(&tx.from).unwrap().transfer(
                    tx.datetime,
                    self.lookup_account(&tx.to).unwrap(),
                    tx.money.clone(),
                )
            })
            .collect()
    }
}

#[derive(Hash, Clone, Debug, Display, PartialEq, Eq)]
pub struct AccountId(String);

impl Deref for AccountId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AccountId {
    fn new(id: &str) -> Self {
        AccountId(id.to_string())
    }
}

impl From<&str> for AccountId {
    fn from(name: &str) -> Self {
        AccountId::new(name)
    }
}

#[derive(Debug)]
pub struct Transaction {
    datetime: DateTime<Utc>,
    from: AccountId,
    to: AccountId,
    money: Money,
}

impl Transaction {
    pub fn datetime(&self) -> DateTime<Utc> {
        self.datetime
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
