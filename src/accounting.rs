use std::iter::FromIterator;
use std::rc::Rc;
use std::rc::Weak;

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
        R: AsRef<Transaction>,
    {
        transactions
            .iter()
            .map(AsRef::as_ref)
            .filter(|tx| tx.datetime <= datetime)
            .map(|tx| {
                let mut money = Money::none();
                if tx.to.upgrade().unwrap().as_ref() == self {
                    money += tx.money.clone();
                }
                if tx.from.upgrade().unwrap().as_ref() == self {
                    money -= tx.money.clone();
                }
                money
            })
            .collect()
    }

    pub fn running_balance<R, T>(&self, transactions: &'a [R]) -> T
    where
        R: AsRef<Transaction> + ToOwned,
        T: FromIterator<(<R as ToOwned>::Owned, Money)>,
    {
        transactions
            .iter()
            .filter_map(|tx| {
                let tx_r = tx.as_ref();
                if tx_r.to.upgrade().unwrap().as_ref() == self
                    || tx_r.from.upgrade().unwrap().as_ref() == self
                {
                    Some((tx.to_owned(), self.balance(tx_r.datetime, transactions)))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub(crate) datetime: DateTime<Utc>,
    pub(crate) from: Weak<Account>,
    pub(crate) to: Weak<Account>,
    pub(crate) money: Money,
}

impl Transaction {
    pub fn new(
        datetime: DateTime<Utc>,
        from: &Rc<Account>,
        to: &Rc<Account>,
        money: Money,
    ) -> Self {
        Transaction {
            datetime,
            from: Rc::downgrade(from),
            to: Rc::downgrade(to),
            money,
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.datetime
    }
}
