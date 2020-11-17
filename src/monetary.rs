use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;

pub type MinorAmount = i64;
pub type MajorAmount = MinorAmount;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Currency {
    code: String,
    minor_to_major: u8,
}

impl Currency {
    pub fn new(code: &str, minor_to_major: u8) -> Self {
        Self {
            code: code.to_string(),
            minor_to_major,
        }
    }

    pub fn of_major(&self, amount: MajorAmount) -> Money {
        self.of(amount, 0)
    }

    pub fn of_minor(&self, amount: MinorAmount) -> Money {
        Money::none() + (self.clone(), amount)
    }

    pub fn of(&self, major_amount: MajorAmount, minor_amount: MinorAmount) -> Money {
        self.of_minor(
            major_amount as MinorAmount * self.minor_to_major as MinorAmount + minor_amount,
        )
    }
}

pub const THB: fn() -> Currency = || Currency::new("THB", 100);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Money {
    pub(crate) amounts: HashMap<Currency, MinorAmount>,
}

impl Money {
    pub fn none() -> Self {
        Money {
            amounts: HashMap::new(),
        }
    }
    pub fn get(&self, currency: &Currency) -> Option<MinorAmount> {
        self.amounts.get(currency).map(|&x| x)
    }
}

impl FromIterator<Money> for Money {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Money>,
    {
        iter.into_iter().sum()
    }
}

impl Add for Money {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Money) {
        rhs.amounts.into_iter().for_each(|entry| {
            *self += entry;
        });
    }
}

impl std::ops::SubAssign for Money {
    fn sub_assign(&mut self, rhs: Money) {
        *self += -rhs
    }
}

impl<'a> AddAssign<(Currency, MinorAmount)> for Money {
    fn add_assign(&mut self, (currency, amount): (Currency, MinorAmount)) {
        self.amounts
            .entry(currency)
            .and_modify(|this| *this += amount)
            .or_insert(amount);
    }
}

impl<'a> Add<(Currency, MinorAmount)> for Money {
    type Output = Self;
    fn add(mut self, entry: (Currency, MinorAmount)) -> Self {
        self += entry;
        self
    }
}

impl<'a> Sum<Money> for Money {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Money>,
    {
        iter.fold(Money::none(), Add::add)
    }
}

impl Neg for Money {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.amounts.iter_mut().for_each(|(_, amount)| {
            *amount = -*amount;
        });
        self
    }
}
