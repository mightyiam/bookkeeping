use std::ops::Deref;
use std::rc::Rc;

pub struct Book {
    accounts: Vec<Rc<Account>>,
}

pub struct Account {
    name: String,
}

impl Account {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Book {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
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
}
