use super::fiat::*;

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

    pub fn balance(&self, transactions: &[&Transaction<'a>]) -> Money<'a> {
        transactions
            .iter()
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
    pub(crate) from: &'a Account,
    pub(crate) to: &'a Account,
    pub(crate) money: Money<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(from: &'a Account, to: &'a Account, money: Money<'a>) -> Self {
        Transaction { from, to, money }
    }
}
