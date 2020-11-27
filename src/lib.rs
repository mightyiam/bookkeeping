pub mod accounting;
pub mod book;
pub mod monetary;

//use accounting::*;
//use std::cell::RefCell;
//use std::rc::Rc;

pub use book::{Book, Currency, DateTime, Money, Utc};
/*
#[derive(Debug)]
pub struct BookHandle {
    book: Rc<RefCell<Book>>,
}

impl BookHandle {
    pub fn new() -> Self {
        Self {
            book: Rc::new(RefCell::new(Book::new())),
        }
    }

    pub fn new_account(&self, name: &str) -> AccountHandle {
        AccountHandle::new(self.book.borrow_mut().new_account(name), self.book.clone())
    }

    pub fn accounts(&self) -> Vec<AccountHandle> {
        self.book
            .borrow()
            .accounts()
            .into_iter()
            .map(|account| AccountHandle::new(account, self.book.clone()))
            .collect()
    }
}

#[derive(Debug)]
pub struct AccountHandle {
    account: Rc<Account>,
    book: Rc<RefCell<Book>>,
}

impl AccountHandle {
    fn new(account: Rc<Account>, book: Rc<RefCell<Book>>) -> Self {
        Self { account, book }
    }

    pub fn transfer(&self, to: &AccountHandle, money: Money) -> TransactionHandle {
        TransactionHandle::new(
            self.book
                .borrow_mut()
                .transfer(&self.account, &to.account, money),
            self.book.clone(),
        )
    }

    pub fn transfer_at(
        &self,
        datetime: DateTime<Utc>,
        to: &AccountHandle,
        money: Money,
    ) -> TransactionHandle {
        TransactionHandle::new(
            self.book
                .borrow_mut()
                .transfer_at(datetime, &self.account, &to.account, money),
            self.book.clone(),
        )
    }

    pub fn balance(&self) -> Money {
        self.book.borrow().balance(&self.account)
    }

    pub fn balance_at(&self, datetime: DateTime<Utc>) -> Money {
        self.book.borrow().balance_at(datetime, &self.account)
    }

    pub fn running_balance(&self) -> Vec<(TransactionHandle, Money)> {
        self.book
            .borrow()
            .running_balance(&self.account)
            .into_iter()
            .map(|(transaction, money)| {
                (
                    TransactionHandle::new(transaction, self.book.clone()),
                    money,
                )
            })
            .collect()
    }
}

pub struct TransactionHandle {
    transaction: Rc<Transaction>,
    _book: Rc<RefCell<Book>>,
}

impl TransactionHandle {
    fn new(transaction: Rc<Transaction>, book: Rc<RefCell<Book>>) -> Self {
        Self {
            transaction,
            _book: book,
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.transaction.datetime()
    }
}
*/
