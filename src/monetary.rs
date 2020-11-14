use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;

pub type MinorAmount = i64;
pub type MajorAmount = MinorAmount;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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
        Money::none() + (*self, amount)
    }

    pub fn of(&self, major_amount: MajorAmount, minor_amount: MinorAmount) -> Money {
        self.of_minor(
            major_amount as MinorAmount * self.minor_to_major as MinorAmount + minor_amount,
        )
    }
}

pub const THB: fn() -> Currency<'static> = || Currency::new("THB", 100);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Money<'a> {
    pub(crate) amounts: HashMap<Currency<'a>, MinorAmount>,
}

impl<'a> Money<'a> {
    pub fn none() -> Self {
        Money {
            amounts: HashMap::new(),
        }
    }
    pub fn get(&self, currency: Currency<'a>) -> Option<MinorAmount> {
        self.amounts.get(&currency).map(|x| *x)
    }
}

impl<'a> FromIterator<Money<'a>> for Money<'a> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Money<'a>>,
    {
        iter.into_iter().sum()
    }
}

impl<'a> Add for Money<'a> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<'a> AddAssign for Money<'a> {
    fn add_assign(&mut self, rhs: Money<'a>) {
        rhs.amounts.into_iter().for_each(|entry| {
            *self += entry;
        });
    }
}

impl<'a> std::ops::SubAssign for Money<'a> {
    fn sub_assign(&mut self, rhs: Money<'a>) {
        *self += -rhs
    }
}

impl<'a> AddAssign<(Currency<'a>, MinorAmount)> for Money<'a> {
    fn add_assign(&mut self, (currency, amount): (Currency<'a>, MinorAmount)) {
        self.amounts
            .entry(currency)
            .and_modify(|this| *this += amount)
            .or_insert(amount);
    }
}

impl<'a> Add<(Currency<'a>, MinorAmount)> for Money<'a> {
    type Output = Self;
    fn add(mut self, entry: (Currency<'a>, MinorAmount)) -> Self {
        self += entry;
        self
    }
}

impl<'a> Sum<Money<'a>> for Money<'a> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Money<'a>>,
    {
        iter.fold(Money::none(), Add::add)
    }
}

impl<'a> Neg for Money<'a> {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.amounts.iter_mut().for_each(|(_, amount)| {
            *amount = -*amount;
        });
        self
    }
}
