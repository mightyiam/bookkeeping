use super::fiat::*;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::Add;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Account {
    name: String,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub struct Transaction<'a> {
    pub(crate) from: Rc<Account>,
    pub(crate) to: Rc<Account>,
    pub(crate) money: Money<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(from: Rc<Account>, to: Rc<Account>, money: Money<'a>) -> Self {
        Transaction { from, to, money }
    }
}

pub struct Balance<'a> {
    balance: HashMap<Currency<'a>, MinorAmount>,
}

impl<'a> Balance<'a> {
    pub fn new() -> Self {
        Balance {
            balance: HashMap::new(),
        }
    }

    pub fn get(&self, currency: Currency<'a>) -> Option<MinorAmount> {
        self.balance.get(&currency).map(|x| *x)
    }
}

impl<'a> Add<Money<'a>> for Balance<'a> {
    type Output = Balance<'a>;
    fn add(mut self, rhs: Money<'a>) -> Balance<'a> {
        self.balance
            .entry(rhs.currency)
            .and_modify(|amount| {
                *amount += rhs.minor_amount;
            })
            .or_insert(rhs.minor_amount);
        self
    }
}

impl<'a> Sum<Money<'a>> for Balance<'a> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Money<'a>>,
    {
        iter.fold(Balance::new(), Add::add)
    }
}
