use super::fiat::*;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
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
