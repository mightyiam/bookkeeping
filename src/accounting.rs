use std::iter::FromIterator;

use chrono::{DateTime, Utc};

use super::monetary::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Account {
    name: String,
}

impl<'a> Account {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn balance<R>(&self, datetime: DateTime<Utc>, transactions: &[R]) -> Money
    where
        R: AsRef<Transaction<'a>>,
    {
        transactions
            .iter()
            .map(AsRef::as_ref)
            .filter(|tx| tx.datetime <= datetime)
            .map(|tx| {
                let mut money = Money::none();
                if tx.to == self {
                    money += tx.money.clone();
                }
                if tx.from == self {
                    money -= tx.money.clone();
                }
                money
            })
            .collect()
    }

    pub fn running_balance<R, T>(&self, transactions: &'a [R]) -> T
    where
        R: AsRef<Transaction<'a>> + ToOwned,
        T: FromIterator<(<R as ToOwned>::Owned, Money)>,
    {
        transactions
            .iter()
            .filter_map(|tx| {
                let tx_r = tx.as_ref();
                if tx_r.to == self || tx_r.from == self {
                    Some((tx.to_owned(), self.balance(tx_r.datetime, transactions)))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Transaction<'a> {
    pub(crate) datetime: DateTime<Utc>,
    pub(crate) from: &'a Account,
    pub(crate) to: &'a Account,
    pub(crate) money: Money,
}

impl<'a> Transaction<'a> {
    pub fn new(datetime: DateTime<Utc>, from: &'a Account, to: &'a Account, money: Money) -> Self {
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
