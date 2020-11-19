use std::iter::FromIterator;

use chrono::{DateTime, Utc};

use super::monetary::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Account<'a> {
    name: &'a str,
}

impl<'a> Account<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn balance<I>(&self, datetime: DateTime<Utc>, transactions: I) -> Money<'a>
    where
        I: IntoIterator<Item = &'a Transaction<'a>>,
    {
        transactions
            .into_iter()
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

    pub fn running_balance<I>(
        &'a self,
        transactions: I,
    ) -> impl Iterator<Item = (&'a Transaction<'a>, Money<'a>)>
    where
        I: Iterator<Item = &'a Transaction<'a>> + Clone,
    {
        transactions.clone().filter_map(move |tx| {
            if tx.to == self || tx.from == self {
                Some((tx, self.balance(tx.datetime, transactions.clone())))
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub struct Transaction<'a> {
    pub(crate) datetime: DateTime<Utc>,
    pub(crate) from: &'a Account<'a>,
    pub(crate) to: &'a Account<'a>,
    pub(crate) money: Money<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(from: &'a Account, to: &'a Account, money: Money<'a>) -> Self {
        Transaction::new_at(Utc::now(), from, to, money)
    }
    pub fn new_at(
        datetime: DateTime<Utc>,
        from: &'a Account,
        to: &'a Account,
        money: Money<'a>,
    ) -> Self {
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
