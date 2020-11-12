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
            .filter_map(|tx| {
                if tx.to == self {
                    Some(tx.money.clone())
                } else if tx.from == self {
                    Some(-tx.money.clone())
                } else {
                    None
                }
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
