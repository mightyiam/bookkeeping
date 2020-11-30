use std::fmt::Debug;

use chrono::{DateTime, Utc};

use super::monetary::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Account {
    id: usize,
}

impl Account {
    pub fn new() -> Self {
        fn get_id() -> usize {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static COUNTER: AtomicUsize = AtomicUsize::new(1);
            COUNTER.fetch_add(1, Ordering::Relaxed)
        }

        Self { id: get_id() }
    }

    pub fn transfer_to<'a>(
        &'a self,
        datetime: DateTime<Utc>,
        to: &'a Account,
        money: Money,
    ) -> Transaction {
        Transaction::new(datetime, self, to, money)
    }

    pub fn balance_with<'a, I, T, F, E>(
        &self,
        datetime: DateTime<Utc>,
        ts: I,
        f: F,
    ) -> Result<Money, E>
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a,
        F: Fn(&'a T) -> Result<Transaction<'a>, E>,
    {
        ts.into_iter()
            .map(f)
            .collect::<Result<Vec<_>, _>>()
            .map(|txs| self.balance(datetime, &txs))
    }

    pub fn balance<'a, I>(&self, datetime: DateTime<Utc>, transactions: I) -> Money
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

    pub fn running_balance_with<'a, I, T, F, E>(
        &'a self,
        ts: I,
        f: F,
    ) -> Result<impl Iterator<Item = (&'a T, Money)> + Debug + 'a, E>
    where
        I: IntoIterator<Item = &'a T> + Clone + 'a,
        T: Debug + 'a,
        F: Fn(&'a T) -> Result<Transaction<'a>, E> + 'a,
    {
        ts.into_iter()
            .map(|t| f(t).map(|tx| (t, tx)))
            .collect::<Result<Vec<_>, E>>()
            .map(|ttxs| {
                let txs = ttxs
                    .clone()
                    .into_iter()
                    .map(|(_, tx)| tx)
                    .collect::<Vec<_>>();
                ttxs.into_iter().filter_map(move |(t, tx)| {
                    if [tx.to, tx.from].contains(&self) {
                        Some((t, self.balance(tx.datetime, txs.as_slice())))
                    } else {
                        None
                    }
                })
            })
    }
}

#[derive(Clone, Debug)]
pub struct Transaction<'a> {
    pub(self) datetime: DateTime<Utc>,
    pub(self) from: &'a Account,
    pub(self) to: &'a Account,
    pub(self) money: Money,
}

impl<'a> Transaction<'a> {
    pub fn new(datetime: DateTime<Utc>, from: &'a Account, to: &'a Account, money: Money) -> Self {
        Self {
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

#[cfg(test)]
mod test {
    use chrono::Utc;

    use super::*;

    #[test]
    fn transfer() {
        let acc1 = Account::new();
        let acc2 = Account::new();
        let ref thb = THB();
        let transactions = vec![acc1.transfer_to(Utc::now(), &acc2, thb.of_major(50))];
        assert_eq!(acc1.balance(Utc::now(), &transactions), thb.of_major(-50));
        assert_eq!(acc2.balance(Utc::now(), &transactions), thb.of_major(50));
    }
}
