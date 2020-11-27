use std::ops::Deref;

use chrono::{DateTime, Utc};
use derive_more::Display;

use super::monetary::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Account {
    id: AccountId,
}

impl Account {
    pub fn new(id: AccountId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> AccountId {
        self.id.clone()
    }

    pub fn transfer(&self, datetime: DateTime<Utc>, to: &Account, money: Money) -> Transaction {
        Transaction::new(datetime, self.id(), to.id(), money)
    }

    pub fn balance(&self, datetime: DateTime<Utc>, transactions: &[Transaction]) -> Money {
        transactions
            .iter()
            .filter(|tx| tx.datetime <= datetime)
            .map(|tx| {
                let mut money = Money::none();
                if tx.to == self.id() {
                    money += tx.money.clone();
                }
                if tx.from == self.id() {
                    money -= tx.money.clone();
                }
                money
            })
            .collect()
    }

    pub fn running_balance<'a>(
        &self,
        transactions: &'a [Transaction],
    ) -> Vec<(&'a Transaction, Money)> {
        transactions
            .iter()
            .filter_map(|tx| {
                if tx.to == self.id() || tx.from == self.id() {
                    Some((tx, self.balance(tx.datetime, transactions)))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct Transaction {
    pub(crate) datetime: DateTime<Utc>,
    pub(crate) from: AccountId,
    pub(crate) to: AccountId,
    pub(crate) money: Money,
}

impl Transaction {
    pub fn new(datetime: DateTime<Utc>, from: AccountId, to: AccountId, money: Money) -> Self {
        Transaction {
            datetime,
            from,
            to,
            money,
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.datetime
    }
}
