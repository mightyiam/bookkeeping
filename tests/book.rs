use envelope_system::*;

#[test]
fn adding_one_account() {
    let mut book = Book::new();
    let wallet = book.new_account("wallet");
    assert_eq!(
        book.accounts().first().unwrap().as_ref() as *const Account,
        wallet.as_ref() as *const Account,
        "they are the same account"
    );
}

#[test]
fn transfer_between_two_accounts() {
    let mut book = Book::new();
    let bank = book.new_account("bank");
    let wallet = book.new_account("wallet");
    let thb = fiat::THB();
    let _500_baht = thb.of(500, 0);
    let _take_500_baht_out_of_bank = book.transfer(bank.clone(), wallet.clone(), _500_baht);
    let bank_balance = book.balance(bank);
    let wallet_balance = book.balance(wallet);
    assert_eq!(bank_balance.get(thb).unwrap(), -50000);
    assert_eq!(wallet_balance.get(thb).unwrap(), 50000);
}
