pub const THB: fn() -> Currency = || Currency::new("THB", 2);

pub struct Currency {
    code: String,
    scale: u32,
}

impl Currency {
    pub fn new(code: &str, scale: u32) -> Self {
        Self {
            code: code.to_string(),
            scale,
        }
    }

    pub fn of(&self, amount: i64) -> Money {
        Money::new(amount, self)
    }
}

pub struct Money<'a> {
    amount: rust_decimal::Decimal,
    currency: &'a Currency,
}

impl<'a> Money<'a> {
    pub fn new(amount: i64, currency: &'a Currency) -> Self {
        Money {
            amount: rust_decimal::Decimal::new(amount, currency.scale),
            currency,
        }
    }

    pub fn negate(&self) -> Self {
        let amount = self.amount.clone();
        amount.set_sign_negative(true);
        Money {
            amount,
            currency: self.currency,
        }
    }
}
