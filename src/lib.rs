pub mod accounting;
pub mod book;
pub mod monetary;

use accounting::*;
use std::cell::RefCell;
use std::rc::Rc;

pub use book::{Book, Currency, DateTime, Money, Utc};

#[derive(Debug, PartialEq, Eq)]
pub struct BookHandle<'a> {
    book: Rc<RefCell<Book<'a>>>,
}

impl<'a> BookHandle<'a> {
    pub fn new() -> Self {
        Self { book: Rc::new(RefCell::new(Book::new())) }
    }

    pub fn new_account(&'a self, name: &'a str) -> AccountHandle<'a> {
        AccountHandle::new(self.book.borrow_mut().new_account(name), self.book.clone())
    }

    pub fn accounts(&'a self) -> Vec<AccountHandle<'a>> {
        self.book
            .borrow()
            .accounts()
            .into_iter()
            .map(|account| AccountHandle::new(account,self.book.clone()))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AccountHandle<'a> {
    account: Rc<Account>,
    book: Rc<RefCell<Book<'a>>>,
}

impl<'a> AccountHandle<'a> {
    fn new(account: Rc<Account>, book: Rc<RefCell<Book<'a>>>) -> Self /* AcconutHandle<'a> */ {
        AccountHandle { account, book }
    }

    pub fn transfer(
        &'a self,
        to: &'a AccountHandle<'a>,
        money: Money,
    ) -> TransactionHandle<'a> {
        TransactionHandle::new(
            self.book.borrow_mut().transfer(&self.account, &to.account, money),
            self.book.clone(),
        )
    }

    pub fn balance(&self) -> Money {
        self.book.borrow().balance(&self.account)
    }
}

pub struct TransactionHandle<'a> {
    transaction: Rc<Transaction<'a>>,
    book: Rc<RefCell<Book<'a>>>,
}

impl<'a> TransactionHandle<'a> {
    fn new(transaction: Rc<Transaction<'a>>, book: Rc<RefCell<Book<'a>>>) -> Self {
        Self { transaction, book }
    }
}
