#[macro_export]
macro_rules! assert_with_decimal_places {
    ($mand_1:expr, $mand_2:expr) => {
        $mand_1.iter().zip($mand_2).for_each(|(a, b)| {
            let x = format!("{:.05}", a);
            let y = format!("{:.05}", b);
            assert_eq!(x, y);
        });
    };
}