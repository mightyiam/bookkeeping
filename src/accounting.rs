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

    pub fn transfer<'a>(
        &'a self,
        datetime: DateTime<Utc>,
        to: &'a Account,
        money: Money,
    ) -> Transaction {
        Transaction::new(datetime, self, to, money)
    }

    pub fn balance(&self, datetime: DateTime<Utc>, transactions: &[Transaction]) -> Money {
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
        let transactions = vec![acc1.transfer(Utc::now(), &acc2, thb.of_major(50))];
        assert_eq!(acc1.balance(Utc::now(), &transactions), thb.of_major(-50));
        assert_eq!(acc2.balance(Utc::now(), &transactions), thb.of_major(50));
    }
}
