use std::cell::RefCell;
use std::rc::Rc;
type MinorAmount = i64;
#[derive(Eq, PartialEq, Debug)]
pub struct Currency {
    pub(crate) code: String,
    pub(crate) decimal_places: u8,
}
impl Currency {
    pub fn new(code: &str, decimal_places: u8) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            code: code.into(),
            decimal_places,
        }))
    }
}
#[derive(Eq, PartialEq, Debug)]
pub struct Account {
    moves: Vec<Rc<RefCell<Move>>>,
}
impl Account {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { moves: Vec::new() }))
    }
    pub fn moves(&self) -> Vec<Rc<RefCell<Move>>> {
        self.moves.clone()
    }
}
pub struct DraftTransaction {
    moves: Vec<DraftMove>,
}
impl DraftTransaction {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }
    pub fn add_move(&mut self, move_: DraftMove) {
        self.moves.push(move_);
    }
    pub fn finalize(&mut self) {
        self.moves.iter().for_each(|draft| {
            let final_ = Move::new(draft.account(), draft.currency(), draft.amount);
            draft.account().borrow_mut().moves.push(final_);
        });
    }
}
pub struct DraftMove {
    account: Rc<RefCell<Account>>,
    currency: Rc<RefCell<Currency>>,
    amount: MinorAmount,
}
impl DraftMove {
    pub fn new(
        account: Rc<RefCell<Account>>,
        currency: Rc<RefCell<Currency>>,
        amount: MinorAmount,
    ) -> Self {
        Self {
            account,
            currency,
            amount,
        }
    }
    pub fn account(&self) -> Rc<RefCell<Account>> {
        self.account.clone()
    }
    pub fn currency(&self) -> Rc<RefCell<Currency>> {
        self.currency.clone()
    }
    pub fn amount(&self) -> MinorAmount {
        self.amount
    }
}
#[derive(Eq, PartialEq, Debug)]
pub struct Move {
    account: Rc<RefCell<Account>>,
    currency: Rc<RefCell<Currency>>,
    amount: MinorAmount,
}
impl Move {
    fn new(
        account: Rc<RefCell<Account>>,
        currency: Rc<RefCell<Currency>>,
        amount: MinorAmount,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            account,
            currency,
            amount,
        }))
    }
    pub fn account(&self) -> Rc<RefCell<Account>> {
        self.account.clone()
    }
    pub fn currency(&self) -> Rc<RefCell<Currency>> {
        self.currency.clone()
    }
    pub fn amount(&self) -> MinorAmount {
        self.amount
    }
}
#[test]
fn last_story() {
    let bank = Account::new();
    let wallet = Account::new();
    let mut thb_atm_withdrawal = DraftTransaction::new();
    let thb = Currency::new("THB", 2);
    thb_atm_withdrawal.add_move(DraftMove::new(bank.clone(), thb.clone(), -10000));
    thb_atm_withdrawal.add_move(DraftMove::new(wallet.clone(), thb.clone(), 10000));
    let thb_atm_withdrawal = thb_atm_withdrawal.finalize();
    let bank_moves = bank.borrow().moves();
    let wallet_moves = wallet.borrow().moves();
    assert_eq!(bank_moves, vec![Move::new(bank, thb.clone(), -10000)]);
    assert_eq!(wallet_moves, vec![Move::new(wallet, thb.clone(), 10000)]);
}
