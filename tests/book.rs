use envelope_system::*;
use std::rc::Rc;

#[test]
fn adding_one_account() {
    let mut book = Book::new();
    let wallet: Rc<Account> = book.new_account("wallet");
    assert_eq!(
        book.accounts().first().unwrap().as_ref() as *const Account,
        wallet.as_ref() as *const Account,
        "they are the same account"
    );
}

#[test]
fn transfer_own_account() {
    let mut book = Book::new();
    let acc = book.new_account("account");
    let thb = fiat::THB();
    assert_eq!(book.balance(&acc), fiat::Money::none());
    let _tx = book.transfer(&acc, &acc, thb.of_major(5));
    assert_eq!(book.balance(&acc).get(thb).unwrap(), 0);
}

#[test]
fn transfer_between_two_accounts() {
    let mut book = Book::new();
    let bank = book.new_account("bank");
    let wallet = book.new_account("wallet");
    let thb = fiat::THB();
    let _500_baht = thb.of(500, 0);
    let _withdraw_500_from_bank = book.transfer(&bank, &wallet, _500_baht);
    let bank_balance = book.balance(&bank);
    let wallet_balance = book.balance(&wallet);
    assert_eq!(bank_balance.get(thb).unwrap(), -50000);
    assert_eq!(wallet_balance.get(thb).unwrap(), 50000);

    let _put_100_into_bank = book.transfer(&wallet, &bank, thb.of_major(100));
    assert_eq!(book.balance(&bank).get(thb).unwrap(), -40000);
    assert_eq!(book.balance(&wallet).get(thb).unwrap(), 40000);
}
