use rust_decimal::Decimal;
use std::collections::HashMap;
use std::rc::Rc;

type AccountBalances = HashMap<Rc<Account>, Balances>;
type Balances = HashMap<money::Currency, Decimal>;

pub struct Book<'a> {
    accounts: Vec<Rc<Account>>,
    transactions: Vec<Rc<Transaction<'a>>>,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Account {
    name: String,
}

pub mod money;

impl Account {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl<'a> Book<'a> {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn new_account(&mut self, name: &str) -> Rc<Account> {
        let account = Rc::new(Account::new(name));
        self.accounts.push(Rc::clone(&account));
        account
    }

    pub fn accounts(&self) -> Vec<Rc<Account>> {
        self.accounts.clone()
    }

    pub fn transfer(
        &mut self,
        from: Rc<Account>,
        to: Rc<Account>,
        money: money::Money<'a>,
    ) -> Rc<Transaction> {
        let transaction = Rc::new(Transaction::new(from, to, money));
        self.transactions.push(Rc::clone(&transaction));
        transaction
    }

    pub fn account_balances(&self, account: Rc<Account>) -> AccountBalances {
        self.accounts
            .iter()
            .cloned()
            .map(|account| {
                (
                    account,
                    self.transactions.iter().filter_map(|tx| {
                        if tx.to == account {
                            Some(tx.money)
                        } else if tx.from == account {
                            Some(tx.money.negate())
                        } else {
                            None
                        }
                    }).,
                )
            })
            .collect()
    }
}

pub struct Transaction<'a> {
    pub(crate) from: Rc<Account>,
    pub(crate) to: Rc<Account>,
    pub(crate) money: money::Money<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(from: Rc<Account>, to: Rc<Account>, money: money::Money<'a>) -> Self {
        Transaction { from, to, money }
    }
}
