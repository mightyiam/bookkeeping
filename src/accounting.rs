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

    pub fn balance(&self, datetime: DateTime<Utc>, transactions: &[&Transaction<'a>]) -> Money<'a> {
        transactions
            .iter()
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
}

#[derive(Debug)]
pub struct Transaction<'a> {
    pub(crate) datetime: DateTime<Utc>,
    pub(crate) from: &'a Account,
    pub(crate) to: &'a Account,
    pub(crate) money: Money<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(
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
}
