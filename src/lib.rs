extern crate derive_more;
use derive_more::Add;
use rust_decimal::Decimal;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[derive(Add)]
struct Amount(Decimal);

impl Amount {
    fn new(num: i64, scale: u32) -> Self {
        Amount(Decimal::new(num, scale))
    }
}

pub struct Money<'a> {
    amount: Amount,
    currency: &'a Currency,
}

impl<'a> Money<'a> {
    fn currency(&self) -> &Currency {
        self.currency
    }
}

/*
impl<'a> Add for Money<'a> {
    type Output = Money<'a>;
    fn add(self, rhs: Money) -> Self::Output {
        let currency = self.currency();
        if currency != rhs.currency() {
            panic!()
        };
        currency.of(self.amount + rhs.amount.into())
    }
}
*/

pub struct Currency {
    code: String,
    decimal_places: u32,
}

impl Currency {
    pub fn new(code: String, decimal_places: u32) -> Self {
        Currency {
            code,
            decimal_places,
        }
    }

    pub fn code(&self) -> &str {
        &self.code[..]
    }

    pub fn of(&self, amount: i64) -> Money {
        Money {
            amount: Amount::new(amount, self.decimal_places),
            currency: self,
        }
    }
}

impl Hash for Currency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code().hash(state);
    }
}

impl PartialEq for Currency {
    fn eq(&self, other: &Currency) -> bool {
        self.code() == other.code()
    }
}

impl Eq for Currency {}

pub struct Book<'a> {
    accounts: Vec<&'a Account>,
    transactions: Vec<&'a Transaction<'a>>,
}

pub struct Account {
    name: String,
}

impl Account {
    fn new(name: String) -> Self {
        Account { name }
    }
}

impl<'a> Book<'a> {
    fn new() -> Self {
        Book {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    fn add_account(&mut self, account: &'a Account) {
        self.accounts.push(account);
    }

    fn add_transaction(&mut self, transaction: &'a Transaction<'a>) {
        // validate accounts are in book
        self.transactions.push(transaction)
    }

    fn accounts(&self) -> &[&Account] {
        &self.accounts[..]
    }
}

pub struct Transaction<'a>(Vec<Move<'a>>);

pub struct Move<'a> {
    account: &'a Account,
    money: Money<'a>,
}

impl<'a> Move<'a> {
    fn new(account: &'a Account, money: Money<'a>) -> Self {
        Move {
            account,
            money: money,
        }
    }
    fn money(&self) -> &Money<'a> {
        &self.money
    }
}

pub struct TransactionDraft<'a> {
    moves: Vec<Move<'a>>,
}

impl<'a> TransactionDraft<'a> {
    fn new() -> Self {
        TransactionDraft { moves: Vec::new() }
    }

    fn add_move(&mut self, mov: Move<'a>) {
        self.moves.push(mov);
    }

    fn balances(&self) -> HashMap<Currency, Money<'a>> {
        self.moves.iter().fold(HashMap::new(), |mut hm, mov| {
            let money = mov.money();
            let currency = money.currency();
            let mut balance = hm.get_mut(currency);
            balance = balance.map_or(Some(&mut currency.of(0)), |acc| acc + money);
            hm
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn inner_works() {
        let mut book = Book::new();
        let employer = Account::new("boss".to_string());
        let wallet = Account::new("wallet".to_string());

        let baht = Currency::new("THB".to_string(), 2);
        let _500_baht = baht.of(50000);

        let mut earn_500_baht = TransactionDraft::new();
        earn_500_baht.add_move(Move::new(&employer, baht.of(50000)));
        earn_500_baht.add_move(Move::new(&wallet, baht.of(50000)));

        book.add_transaction(&earn_500_baht.finalize());

        let wallet_balances_at_some_date: Vec<&Money> =
            buget.balances_at_date(&wallet, at_some_date);
        let wallet_balances_after_transaction =
            budget.balances_after_transaction(&wallet, &transaction);
    }
}
