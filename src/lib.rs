use rust_decimal::Decimal;

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

    pub fn of(&self, amount: i64) -> Money {
        Money {
            amount: Amount::new(amount, self.decimal_places),
            currency: self,
        }
    }
}

pub struct ExternalAccount {
    name: String,
}

impl ExternalAccount {
    fn new(name: String) -> Self {
        ExternalAccount { name }
    }
}

pub struct BudgetAccount {
    name: String,
}

impl BudgetAccount {
    fn name(&self) -> &str {
        &self.name[..]
    }
}

impl BudgetAccount {
    fn new(name: String) -> Self {
        BudgetAccount { name }
    }
}

pub enum Transaction_<'a> {
    Income {
        from: &'a ExternalAccount,
        to: &'a BudgetAccount,
        amount: &'a Money<'a>,
    },
}

pub struct Budget<'a> {
    transactions: Vec<&'a Transaction_<'a>>,
}

impl<'a> Budget<'a> {
    pub fn new() -> Self {
        Budget {
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, tx: &'a Transaction_<'a>) {
        self.transactions.push(tx);
    }

    pub fn budget_accounts(&self) -> Vec<&BudgetAccount> {
        Vec::new()
    }
}

pub enum Change<'a> {
    /*CreateCurrency {
        code: String,
        decimal_places: i8,
    },*/
    CreateBudgetAccount {
        account: &'a BudgetAccount,
    },
    CreateExternalAccount {},
    CreateMove {
        //date: // some date,
        amount: &'a Money<'a>,
        notes: String,
    },
    CreateTransferMove {},
    CreateIncomeMove {
        from: &'a ExternalAccount,
        to: &'a BudgetAccount,
        amount: &'a Money<'a>,
    },
}

struct Book<'a> {
    accounts: Vec<&'a Account>,
}

struct Account {
    name: String,
}

impl<'a> Book<'a> {
    fn new() -> Self {
        Book {
            accounts: Vec::new(),
        }
    }

    fn add_account(&mut self, account: &'a Account) {
        self.accounts.push(account);
    }

    fn add_transaction(&mut self, tx: &Transaction) -> Result<(),Error> {
        // validate accounts are in book
        // adds transactions
    }

    fn accounts(&self) -> &[&Account] {
        &self.accounts[..]
    }
}

pub struct Transaction<'a>(Vec<TransactionItem<'a>>);

pub struct TransactionItem<'a> {
    account: &'a Account,
    amount: Amount,
}

pub struct DraftTransaction<'a>(Transaction<'a>);

impl<'a> DraftTransaction<'a> {
    fn new() -> Self {
        DraftTransaction(Transaction(Vec::new()))
    }

    // TODO fn add
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn inner_works() {
        let mut budget = Budget::new();
        let employer = ExternalAccount::new("boss".to_string());
        let wallet = BudgetAccount::new("wallet".to_string());

        let baht = Currency::new("THB".to_string(), 2);
        let _500_baht = baht.of(50000);

        let earn_500_baht = Transaction_::Income {
            // date:
            from: &employer,
            to: &wallet,
            amount: &_500_baht,
        };

        budget.add(&earn_500_baht);

        let wallet_balances_at_some_date: Vec<&Money> =
            buget.balances_at_date(&wallet, at_some_date);
        let wallet_balances_after_transaction =
            budget.balances_after_transaction(&wallet, &transaction);
    }
}
