pub type MinorAmount = i64;
pub type MajorAmount = MinorAmount;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Currency<'a> {
    code: &'a str,
    minor_to_major: u8,
}

impl<'a> Currency<'a> {
    pub fn new(code: &'a str, minor_to_major: u8) -> Self {
        Self {
            code,
            minor_to_major,
        }
    }

    pub fn of_major(&self, amount: MajorAmount) -> Money {
        self.of(amount, 0)
    }

    pub fn of_minor(&self, amount: MinorAmount) -> Money {
        self.of(0, amount)
    }

    pub fn of(&self, major_amount: MajorAmount, minor_amount: MinorAmount) -> Money {
        Money::new(major_amount, minor_amount, *self)
    }
}

pub const THB: fn() -> Currency<'static> = || Currency::new("THB", 100);

#[derive(Clone, Copy)]
pub struct Money<'a> {
    pub(crate) minor_amount: MinorAmount,
    pub(crate) currency: Currency<'a>,
}

impl<'a> Money<'a> {
    pub(crate) fn new(
        major_amount: MajorAmount,
        minor_amount: MinorAmount,
        currency: Currency<'a>,
    ) -> Self {
        let minor_amount =
            major_amount as MinorAmount * currency.minor_to_major as MinorAmount + minor_amount;
        Money {
            minor_amount,
            currency,
        }
    }
    pub fn amount(&self) -> (MajorAmount, MinorAmount) {
        let m = self.minor_amount;
        let r = self.currency.minor_to_major as MinorAmount;
        (m / r, m % r)
    }

    pub fn f64(self) -> f64 {
        self.into()
    }
}

impl<'a> From<Money<'a>> for f64 {
    fn from(money: Money) -> Self {
        let (major, minor) = money.amount();
        major as Self + minor as Self / money.currency.minor_to_major as Self
    }
}

use std::ops::Neg;

impl<'a> Neg for Money<'a> {
    type Output = Self;

    /// # panics
    /// When is the minimum ammount for currency.
    fn neg(mut self) -> Self {
        self.minor_amount = -self.minor_amount;
        self
    }
}
