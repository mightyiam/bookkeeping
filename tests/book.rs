use chrono::Duration;
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
    let thb = monetary::THB();
    assert_eq!(book.balance(&acc), monetary::Money::none());
    let _tx = book.transfer(&acc, &acc, thb.of_major(5));
    assert_eq!(book.balance(&acc).get(thb).unwrap(), 0);
}

#[test]
fn transfer_between_two_accounts() {
    let mut book = Book::new();
    let bank = book.new_account("bank");
    let wallet = book.new_account("wallet");
    let thb = monetary::THB();
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

#[test]
fn balance_at_dates() {
    let mut book = Book::new();
    let thb = monetary::THB();
    let wallet = book.new_account("wallet");
    let bank = book.new_account("bank");
    let first_withdraw_datetime = DateTime::parse_from_rfc3339("2020-11-10T10:10:57+07:00")
        .unwrap()
        .with_timezone(&Utc);
    let _withdraw_900 =
        book.transfer_at(first_withdraw_datetime, &bank, &wallet, thb.of_major(900));
    let second_withdraw_datetime = first_withdraw_datetime + Duration::seconds(5);
    let _withdraw_100 =
        book.transfer_at(second_withdraw_datetime, &bank, &wallet, thb.of_major(100));
    assert_eq!(
        book.balance_at(first_withdraw_datetime - Duration::seconds(1), &wallet),
        Money::none()
    );
    assert_eq!(
        book.balance_at(first_withdraw_datetime, &wallet),
        thb.of_major(900)
    );
    assert_eq!(
        book.balance_at(second_withdraw_datetime, &wallet),
        thb.of_major(1000)
    );
}
