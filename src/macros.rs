/// Creates a concrete book in order to de-duplicate test code.
#[cfg(test)]
macro_rules! test_book {
    ($metadata:expr) => {{
        type TestBook = crate::book::Book<
            &'static str,
            crate::unit::TestUnit,
            &'static str,
            &'static str,
            &'static str,
        >;
        TestBook::new($metadata)
    }};
}
#[cfg(test)]
macro_rules! sum {
    () => { crate::sum::Sum::<crate::unit::TestUnit>::new() };
    ($($amount:expr, $unit:ident);*) => {{
        let mut sum = crate::sum::Sum::<crate::unit::TestUnit>::new();
        $(sum.set_amount_for_unit($amount, $unit);)*
        sum
    }}
}
