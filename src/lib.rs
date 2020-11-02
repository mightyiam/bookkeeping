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

pub struct Budget {}

impl Budget {
    pub fn new() -> Self {
        Budget {}
    }
    pub fn apply_change(&mut self, change: &Change) {}

    pub fn budget_accounts(&self) -> Vec<&BudgetAccount> {
        self.budget_accounts.clone()
    }
}

pub enum Change<'a> {
    CreateCurrency {
        code: String,
        decimal_places: i8,
    },
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

pub struct Money<'a> {
    amount: f64,
    currency: &'a Currency,
}

pub struct Currency {
    code: String,
    decimal_places: i8,
}

impl Currency {
    pub fn new(code: String, decimal_places: i8) -> Self {
        Currency {
            code,
            decimal_places,
        }
    }

    pub fn of(&self, amount: f64) -> Money {
        Money {
            amount,
            currency: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Budget, BudgetAccount, Change, Currency, ExternalAccount, Money};
    #[test]
    fn it_works() {
        let mut budget = Budget::new();
        let employer = ExternalAccount::new("boss".to_string());
        let wallet = BudgetAccount::new("wallet".to_string());
        let create_wallet = Change::CreateBudgetAccount { account: &wallet };
        budget.apply_change(&create_wallet);
        let baht = Currency::new("THB".to_string(), 2);
        let _500_baht = baht.of(500.00);
        let put_500baht_in_wallet = Change::CreateIncomeMove {
            from: &employer,
            to: &wallet,
            amount: &_500_baht,
        };
        budget.apply_change(&put_500baht_in_wallet);
        let budget_accounts: Vec<&BudgetAccount> = budget.budget_accounts();
        let firstBudgetAccount = budget_accounts.first().unwrap();
        let firstBudgetAccountName = firstBudgetAccount.name();
        let firstBudgetAccountBalances: Vec<&Money> = firstBudgetAccount.balances();
        let firstBudgetAccountFirstBalance: &Money = firstBudgetAccountBalances.first().unwrap();
        let firstBudgetAccountFirstBalanceCurrency = firstBudgetAccountFirstBalance.currency();
        let firstBudgetAccountFirstBalanceAmount = firstBudgetAccountFirstBalance.amount();
    }
}
