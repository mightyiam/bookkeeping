#[cfg(test)]
pub(crate) type TestBook = crate::book::Book<
    &'static str,
    u64,
    &'static str,
    &'static str,
    &'static str,
>;
#[cfg(test)]
macro_rules! sum {
    () => { crate::sum::Sum::<&'static str, u64>::default() };
    ($($amount:expr, $unit:ident);*) => {{
        let mut sum = crate::sum::Sum::<&str, u64>::default();
        $(sum.set_amount_for_unit($amount, $unit);)*
        sum
    }}
}
#[cfg(test)]
pub(crate) type TestBalance = crate::Balance<&'static str, i128>;
