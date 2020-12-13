use crate::account::Account;
use crate::balance::Balance;
use crate::move_::Move;
use crate::sum::Sum;
use crate::unit::Unit;
use duplicate::duplicate;
use slotmap::{new_key_type, DenseSlotMap};
use std::ops;
new_key_type! {
    /// A key type for referencing accounts.
    pub struct AccountKey;
    /// A key type for referencing units.
    pub struct UnitKey;
    /// A key type for referencing moves.
    pub struct MoveKey;
}
/// Represents a book.
///
/// Where
/// - `B`: book metadata
/// - `A`: account metadata
/// - `U`: unit metadata
/// - `M`: move metadata
#[derive(Default)]
pub struct Book<B, A, U, M> {
    metadata: B,
    accounts: DenseSlotMap<AccountKey, Account<A>>,
    units: DenseSlotMap<UnitKey, Unit<U>>,
    moves: DenseSlotMap<MoveKey, Move<M>>,
    moves_order: Vec<MoveKey>,
}
impl<B, A, U, M> Book<B, A, U, M> {
    /// Creates a new book
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// let _book = Book::<&str, &str, &str, &str>::new("some book");
    /// ```
    pub fn new(metadata: B) -> Self {
        Self {
            metadata,
            accounts: DenseSlotMap::with_key(),
            units: DenseSlotMap::with_key(),
            moves: DenseSlotMap::with_key(),
            moves_order: Vec::new(),
        }
    }
    /// Gets the book's metadata.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let book = Book::<&str, &str, &str, &str>::new("some book");
    /// assert_eq!(*book.metadata(), "some book");
    /// ```
    pub fn metadata(&self) -> &B {
        &self.metadata
    }
    /// Sets the book's metadata.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("some booc");
    /// book.set_book_metadata("some book");
    /// ```
    pub fn set_book_metadata(&mut self, metadata: B) {
        self.metadata = metadata;
    }
    /// Creates a new account.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// let _wallet_key = book.new_account("wallet");
    /// let _bank_key = book.new_account("bank");
    /// ```
    pub fn new_account(&mut self, metadata: A) -> AccountKey {
        self.accounts.insert(Account { metadata })
    }
    /// Creates a new unit.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// let _usd_key = book.new_unit("USD");
    /// let _thb_key = book.new_unit("THB");
    /// let _ils_key = book.new_unit("ILS");
    /// ```
    pub fn new_unit(&mut self, metadata: U) -> UnitKey {
        self.units.insert(Unit { metadata })
    }
    /// Creates a new move and inserts it at a provided index.
    ///
    /// ## Panics
    ///
    /// - `index` is out of bounds.
    /// - Some of `debit_account_key` and `credit_account_key` are not in the book.
    /// - `debit_account_key` and `credit_account_key` are equal.
    /// - Some unit keys that are in the sum are not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let usd_key = book.new_unit("USD");
    /// let mut sum = Sum::new();
    /// sum.set_amount_for_unit(800, usd_key);
    /// let _move_key = book.insert_move(0, bank_key, wallet_key, sum, "withdrawal");
    /// ```
    pub fn insert_move(
        &mut self,
        move_index: usize,
        debit_account_key: AccountKey,
        credit_account_key: AccountKey,
        sum: Sum,
        metadata: M,
    ) -> MoveKey {
        [debit_account_key, credit_account_key].iter().for_each(
            |account_key| {
                self.assert_has_account(*account_key);
            },
        );
        sum.0.keys().for_each(|unit_key| {
            self.assert_has_unit(*unit_key);
        });
        let move_ =
            Move::new(debit_account_key, credit_account_key, sum, metadata);
        let move_key = self.moves.insert(move_);
        self.moves_order.insert(move_index, move_key);
        move_key
    }
    /// Gets an account using a key.
    ///
    /// ## Panics
    ///
    /// - No such account in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// let _wallet = book.get_account(wallet_key);
    /// ```
    pub fn get_account(&self, account_key: AccountKey) -> &Account<A> {
        self.assert_has_account(account_key);
        self.accounts.get(account_key).unwrap()
    }
    /// Gets a unit using a key.
    ///
    /// ## Panics
    ///
    /// - No such unit in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("USD");
    /// let _usd = book.get_unit(usd_key);
    /// ```
    pub fn get_unit(&self, unit_key: UnitKey) -> &Unit<U> {
        self.assert_has_unit(unit_key);
        self.units.get(unit_key).unwrap()
    }
    /// Gets a move using a key.
    ///
    /// ## Panics
    ///
    /// - No such move in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let move_key = book.insert_move(0, bank_key, wallet_key, Sum::new(), "withdrawal");
    /// let _move = book.get_move(move_key);
    /// ```
    pub fn get_move(&self, move_key: MoveKey) -> &Move<M> {
        self.assert_has_move(move_key);
        self.moves.get(move_key).unwrap()
    }
    /// Gets an iterator of existing accounts in order of creation.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let wallet = book.get_account(wallet_key);
    /// # let bank = book.get_account(bank_key);
    /// assert_eq!(
    ///     book.accounts().collect::<Vec<_>>(),
    ///     vec![(wallet_key, wallet), (bank_key, bank)],
    /// );
    /// ```
    pub fn accounts(&self) -> impl Iterator<Item = (AccountKey, &Account<A>)> {
        self.accounts.iter()
    }
    /// Gets an iterator of existing units in order of creation.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("USD");
    /// # let thb_key = book.new_unit("THB");
    /// # let usd = book.get_unit(usd_key);
    /// # let thb = book.get_unit(thb_key);
    /// assert_eq!(
    ///     book.units().collect::<Vec<_>>(),
    ///     vec![(usd_key, usd), (thb_key, thb)],
    /// );
    /// ```
    pub fn units(&self) -> impl Iterator<Item = (UnitKey, &Unit<U>)> {
        self.units.iter()
    }
    /// Gets an iterator of existing moves in their order.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// let deposit_key = book.insert_move(0, wallet_key, bank_key, Sum::new(), "deposit");
    /// let withdrawal_key = book.insert_move(1, bank_key, wallet_key, Sum::new(), "withdrawal");
    /// let deposit = book.get_move(deposit_key);
    /// let withdrawal = book.get_move(withdrawal_key);
    /// assert_eq!(
    ///     book.moves().collect::<Vec<_>>(),
    ///     vec![(0, deposit_key, deposit), (1, withdrawal_key, withdrawal)],
    /// );
    /// ```
    pub fn moves(&self) -> impl Iterator<Item = (usize, MoveKey, &Move<M>)> {
        self.moves_order.iter().enumerate().map(
            move |(move_index, move_key)| {
                (move_index, *move_key, self.moves.get(*move_key).unwrap())
            },
        )
    }
    /// Sets the metadata for an account.
    ///
    /// ## Panics
    /// - The account is not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let bank_key = book.new_account("banc");
    /// book.set_account_metadata(bank_key, "bank");
    /// ```
    pub fn set_account_metadata(
        &mut self,
        account_key: AccountKey,
        metadata: A,
    ) {
        self.assert_has_account(account_key);
        self.accounts.get_mut(account_key).unwrap().metadata = metadata;
    }
    /// Sets the metadata for a unit.
    ///
    /// ## Panics
    /// - The unit is not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::Book;
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let usd_key = book.new_unit("USd");
    /// book.set_unit_metadata(usd_key, "USD");
    /// ```
    pub fn set_unit_metadata(&mut self, unit_key: UnitKey, metadata: U) {
        self.assert_has_unit(unit_key);
        self.units.get_mut(unit_key).unwrap().metadata = metadata;
    }
    /// Sets the metadata for a move.
    ///
    /// ## Panics
    /// - The move is not in the book.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let move_key = book.insert_move(0, bank_key, wallet_key, Sum::new(), "withdrawa");
    /// book.set_move_metadata(move_key, "withdrawal");
    /// ```
    pub fn set_move_metadata(&mut self, move_key: MoveKey, metadata: M) {
        self.assert_has_move(move_key);
        self.moves.get_mut(move_key).unwrap().metadata = metadata;
    }
    /// Calculates the balance of an account at a provided move.
    ///
    /// ## Panics
    ///
    /// - The account is not in the book.
    /// - The account is not debit nor credit in the move.
    ///
    /// ## Example
    /// ```
    /// # use bookkeeping::{ Book, Sum };
    /// # let mut book = Book::<&str, &str, &str, &str>::new("");
    /// # let wallet_key = book.new_account("wallet");
    /// # let bank_key = book.new_account("bank");
    /// # let usd_key = book.new_unit("USD");
    /// # let mut sum = Sum::new();
    /// # sum.set_amount_for_unit(800, usd_key);
    /// # let move_key = book.insert_move(0, bank_key, wallet_key, sum, "withdrawal");
    /// let _balance = book.account_balance_at_move(wallet_key, move_key);
    /// ```
    pub fn account_balance_at_move<'a>(
        &'a self,
        account_key: AccountKey,
        move_key: MoveKey,
    ) -> Balance<'a> {
        self.assert_has_account(account_key);
        self.assert_has_move(move_key);
        let move_ = self.moves.get(move_key).unwrap();
        if ![move_.debit_account_key, move_.credit_account_key]
            .contains(&account_key)
        {
            panic!(
                "Provided account is not debit nor credit in provided move."
            );
        }
        self.moves_order
            .iter()
            .take_while(|cur_move_key| **cur_move_key != move_key)
            .map(|cur_move_key| self.moves.get(*cur_move_key).unwrap())
            .chain(std::iter::once(move_))
            .filter_map(|move_| -> Option<(fn(&mut Balance<'a>, _), &Sum)> {
                if move_.debit_account_key == account_key {
                    Some((ops::SubAssign::sub_assign, &move_.sum))
                } else if move_.credit_account_key == account_key {
                    Some((ops::AddAssign::add_assign, &move_.sum))
                } else {
                    None
                }
            })
            .fold(Balance::new(), |mut balance, (operation, sum)| {
                operation(&mut balance, sum);
                balance
            })
    }
}
#[duplicate(
    assert_has           Key          plural      string    ;
    [assert_has_account] [AccountKey] [accounts] ["account"];
    [assert_has_unit]    [UnitKey]    [units]    ["unit"]   ;
    [assert_has_move]    [MoveKey]    [moves]    ["move"]   ;
)]
impl<B, A, U, M> Book<B, A, U, M> {
    fn assert_has(&self, key: Key) {
        assert!(
            self.plural.contains_key(key),
            format!("No {} found for key {:?}", string, key),
        );
    }
}
#[cfg(test)]
mod test {
    use super::Balance;
    use crate::sum::Sum;
    #[test]
    fn new() {
        let book = test_book!("");
        assert_eq!(book.metadata, "");
        assert!(book.accounts.is_empty());
        assert!(book.units.is_empty());
        assert!(book.moves.is_empty());
        assert!(book.moves_order.is_empty());
    }
    #[test]
    fn new_account() {
        let mut book = test_book!("");
        book.new_account("");
        assert_eq!(book.accounts.len(), 1);
    }
    #[test]
    fn new_unit() {
        let mut book = test_book!("");
        book.new_unit("");
        assert_eq!(book.units.len(), 1,);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn insert_move_panic_debit_account_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        book.accounts.remove(debit_key);
        let credit_key = book.new_account("");
        book.insert_move(0, debit_key, credit_key, Sum::new(), "");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn insert_move_panic_credit_account_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.accounts.remove(credit_key);
        book.insert_move(0, debit_key, credit_key, Sum::new(), "");
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn insert_move_panic_unit_not_found() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        let sum = sum!(0, unit_key);
        book.insert_move(0, debit_key, credit_key, sum, "");
    }
    #[test]
    fn insert_move() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        let move_key_a =
            book.insert_move(0, debit_key, credit_key, Sum::new(), "");
        let move_key_b =
            book.insert_move(0, debit_key, credit_key, Sum::new(), "");
        let move_key_c =
            book.insert_move(1, debit_key, credit_key, Sum::new(), "");
        let move_key_d =
            book.insert_move(2, debit_key, credit_key, Sum::new(), "");
        assert_eq!(
            book.moves_order,
            vec![move_key_b, move_key_c, move_key_d, move_key_a],
        );
    }
    #[test]
    #[should_panic(expected = "insertion index (is 1) should be <= len (is 0)")]
    fn insert_move_panic_index_greater_than_len() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        book.insert_move(1, debit_key, credit_key, Sum::new(), "");
    }
    #[test]
    fn accounts() {
        let mut book = test_book!("");
        assert!(book.accounts().next().is_none());
        let account_key = book.new_account("");
        let account = book.accounts.get(account_key).unwrap();
        let expected = vec![(account_key, account)];
        let actual = book.accounts().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn units() {
        let mut book = test_book!("");
        assert!(book.units().next().is_none());
        let unit_key = book.new_unit("");
        let unit = book.units.get(unit_key).unwrap();
        let expected = vec![(unit_key, unit)];
        let actual = book.units().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn moves() {
        let mut book = test_book!("");
        assert!(book.moves().next().is_none());
        let credit_account_key = book.new_account("");
        let debit_account_key = book.new_account("");
        let move_0_key = book.insert_move(
            0,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        let move_1_key = book.insert_move(
            1,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        let move_0 = book.moves.get(move_0_key).unwrap();
        let move_1 = book.moves.get(move_0_key).unwrap();
        let expected = vec![(0, move_0_key, move_0), (1, move_1_key, move_1)];
        let actual = book.moves().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn get_account() {
        let mut book = test_book!("");
        book.new_account("");
        let account_key = book.new_account("!");
        book.new_account("");
        let account = book.get_account(account_key);
        assert_eq!(*account.metadata(), "!");
    }
    #[test]
    fn get_unit() {
        let mut book = test_book!("");
        book.new_unit("");
        let unit_key = book.new_unit("!");
        book.new_unit("");
        let unit = book.get_unit(unit_key);
        assert_eq!(*unit.metadata(), "!");
    }
    #[test]
    fn get_move() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_move(
            0,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        let move_key = book.insert_move(
            1,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "!",
        );
        book.insert_move(
            2,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        let move_ = book.get_move(move_key);
        assert_eq!(*move_.metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn assert_has_account() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.assert_has_account(account_key);
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn assert_has_unit() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        book.assert_has_unit(unit_key);
    }
    #[test]
    #[should_panic(expected = "No move found for key ")]
    fn assert_has_move() {
        let mut book = test_book!("");
        let credit_account_key = book.new_account("");
        let debit_account_key = book.new_account("");
        let move_key = book.insert_move(
            0,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        book.moves.remove(move_key);
        book.assert_has_move(move_key);
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn account_balance_at_move_account_not_found() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_key = book.insert_move(
            0,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        book.accounts.remove(debit_account_key);
        book.account_balance_at_move(debit_account_key, move_key);
    }
    #[test]
    #[should_panic(expected = "No move found for key ")]
    fn account_balance_at_move_move_not_found() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_key = book.insert_move(
            0,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        book.moves.remove(move_key);
        book.account_balance_at_move(debit_account_key, move_key);
    }
    #[test]
    #[should_panic(
        expected = "Provided account is not debit nor credit in provided move."
    )]
    fn account_balance_at_move_account_not_related_to_move() {
        let mut book = test_book!("");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        let move_key = book.insert_move(
            0,
            debit_account_key,
            credit_account_key,
            Sum::new(),
            "",
        );
        let other_account_key = book.new_account("");
        book.account_balance_at_move(other_account_key, move_key);
    }
    #[test]
    fn account_balance_at_move() {
        let mut book = test_book!("");
        let account_a_key = book.new_account("");
        let account_b_key = book.new_account("");
        let unit_key = book.new_unit("");
        let move_1 = book.insert_move(
            0,
            account_a_key,
            account_b_key,
            sum!(3, unit_key),
            "",
        );
        assert_eq!(
            book.account_balance_at_move(account_a_key, move_1),
            Balance::new() - &sum!(3, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_b_key, move_1),
            Balance::new() + &sum!(3, unit_key),
        );
        let move_2 = book.insert_move(
            1,
            account_a_key,
            account_b_key,
            sum!(4, unit_key),
            "",
        );
        assert_eq!(
            book.account_balance_at_move(account_a_key, move_1),
            Balance::new() - &sum!(3, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_b_key, move_1),
            Balance::new() + &sum!(3, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_a_key, move_2),
            Balance::new() - &sum!(7, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_b_key, move_2),
            Balance::new() + &sum!(7, unit_key),
        );
        let move_0 = book.insert_move(
            0,
            account_a_key,
            account_b_key,
            sum!(1, unit_key),
            "",
        );
        assert_eq!(
            book.account_balance_at_move(account_a_key, move_0),
            Balance::new() - &sum!(1, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_b_key, move_0),
            Balance::new() + &sum!(1, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_a_key, move_1),
            Balance::new() - &sum!(4, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_b_key, move_1),
            Balance::new() + &sum!(4, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_a_key, move_2),
            Balance::new() - &sum!(8, unit_key),
        );
        assert_eq!(
            book.account_balance_at_move(account_b_key, move_2),
            Balance::new() + &sum!(8, unit_key),
        );
    }
    #[test]
    fn set_book_metadata() {
        let mut book = test_book!("");
        book.set_book_metadata("!");
        assert_eq!(book.metadata, "!");
    }
    #[test]
    fn metadata() {
        let book = test_book!("!");
        assert_eq!(*book.metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No account found for key ")]
    fn set_account_metadata_panic() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.accounts.remove(account_key);
        book.set_account_metadata(account_key, "!");
    }
    #[test]
    fn set_account_metadata() {
        let mut book = test_book!("");
        let account_key = book.new_account("");
        book.set_account_metadata(account_key, "!");
        assert_eq!(*book.accounts.get(account_key).unwrap().metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No unit found for key ")]
    fn set_unit_metadata_panic() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        book.units.remove(unit_key);
        book.set_unit_metadata(unit_key, "!");
    }
    #[test]
    fn set_unit_metadata() {
        let mut book = test_book!("");
        let unit_key = book.new_unit("");
        book.set_unit_metadata(unit_key, "!");
        assert_eq!(*book.units.get(unit_key).unwrap().metadata(), "!");
    }
    #[test]
    #[should_panic(expected = "No move found for key ")]
    fn set_move_metadata_panic() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        let move_key =
            book.insert_move(0, debit_key, credit_key, Sum::new(), "");
        book.moves.remove(move_key);
        book.set_move_metadata(move_key, "!");
    }
    #[test]
    fn set_move_metadata() {
        let mut book = test_book!("");
        let debit_key = book.new_account("");
        let credit_key = book.new_account("");
        let move_key =
            book.insert_move(0, debit_key, credit_key, Sum::new(), "");
        book.set_move_metadata(move_key, "!");
        assert_eq!(*book.moves.get(move_key).unwrap().metadata(), "!");
    }
}
