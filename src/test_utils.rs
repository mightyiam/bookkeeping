/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($metadata:expr) => {{
        type TestBook = crate::book::Book<
            &'static str,
            u64,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        >;
        TestBook::new($metadata)
    }};
}
#[cfg(test)]
macro_rules! sum {
    () => { crate::sum::Sum::<&'static str, u64>::new() };
    ($($amount:expr, $unit:ident);*) => {{
        let mut sum = crate::sum::Sum::<&str, u64>::new();
        $(sum.set_amount_for_unit($amount, $unit);)*
        sum
    }}
}
#[cfg(test)]
pub(crate) type TestBalance = crate::Balance<&'static str, i128>;
