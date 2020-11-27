//use chrono::Duration;
//use envelope_system::*;

/*
#[test]
fn adding_one_account() {
    let book = BookHandle::new();
    let wallet: AccountHandle = book.new_account("wallet");
    assert_eq!(
        book.accounts().first().unwrap(),
        &wallet,
        "they are the same account"
    );
}


#[test]
fn transfer_own_account() {
    let book = BookHandle::new();
    let acc = book.new_account("account");
    let thb = monetary::THB();
    assert_eq!(acc.balance(), monetary::Money::none());
    let _tx = acc.transfer(&acc, thb.of_major(5));
    assert_eq!(acc.balance().get(&thb).unwrap(), 0);
}

#[test]
fn transfer_between_two_accounts() {
    let book = BookHandle::new();
    let bank = book.new_account("bank");
    let wallet = book.new_account("wallet");
    let thb = monetary::THB();
    let _500_baht = thb.of(500, 0);
    let _withdraw_500_from_bank = bank.transfer(&wallet, _500_baht);
    let bank_balance = bank.balance();
    let wallet_balance = wallet.balance();
    assert_eq!(bank_balance.get(&thb).unwrap(), -50000);
    assert_eq!(wallet_balance.get(&thb).unwrap(), 50000);

    let _put_100_into_bank = wallet.transfer(&bank, thb.of_major(100));
    assert_eq!(bank.balance().get(&thb).unwrap(), -40000);
    assert_eq!(wallet.balance().get(&thb).unwrap(), 40000);
}

#[test]
fn balance_at_dates() {
    let book = BookHandle::new();
    let thb = monetary::THB();
    let wallet = book.new_account("wallet");
    let bank = book.new_account("bank");
    let first_withdraw_datetime = DateTime::parse_from_rfc3339("2020-11-10T10:10:57+07:00")
        .unwrap()
        .with_timezone(&Utc);
    let _withdraw_900 = bank.transfer_at(first_withdraw_datetime, &wallet, thb.of_major(900));
    let second_withdraw_datetime = first_withdraw_datetime + Duration::seconds(5);
    let _withdraw_100 = bank.transfer_at(second_withdraw_datetime, &wallet, thb.of_major(100));
    assert_eq!(
        wallet.balance_at(first_withdraw_datetime - Duration::seconds(1)),
        Money::none()
    );
    assert_eq!(
        wallet.balance_at(first_withdraw_datetime),
        thb.of_major(900)
    );
    assert_eq!(
        wallet.balance_at(second_withdraw_datetime),
        thb.of_major(1000)
    );
}

#[test]
fn running_balance() {
    let book = BookHandle::new();
    let thb = monetary::THB();
    let bank = book.new_account("bank");
    let wallet = book.new_account("wallet");
    let store = book.new_account("store");
    wallet.transfer(&bank, thb.of_major(20));
    bank.transfer(&wallet, thb.of_major(4));
    wallet.transfer(&store, thb.of_major(1));
    bank.transfer(&wallet, thb.of_major(3));
    bank.transfer(&store, thb.of_major(2));
    bank.transfer(&wallet, thb.of_major(2));
    bank.transfer(&wallet, thb.of_major(1));
    let mut running_balance = bank.running_balance();
    running_balance.sort_by_key(|(tx, _)| tx.datetime());
    assert_eq!(
        running_balance
            .iter()
            .map(|(_, m)| m.get(&thb).unwrap())
            .collect::<Vec<_>>(),
        vec![2000, 1600, 1300, 1100, 900, 800]
    );
}
*/
