/// macro: dispatch_aoc!(year_expr, day_expr, input_expr, {
///     "2025" => { "01" => day01, "02" => day02 },
///     "2024" => { "01" => day01_2024 }
/// });
#[macro_export]
macro_rules! dispatch {
    // main arm: list of years each mapping to a map of days->function
    ($year:expr, $day:expr, $input:expr, $part:expr, {
        $(
            $y:expr => {
                $(
                    $d:expr => $f:path
                ),* $(,)?
            }
        ),* $(,)?
    }) => {
        match $year {
            $(
                $y => {
                    match $day {
                        $(
                            $d => $f($input, $part),
                        )*
                        other => panic!("unknown day for year {}: {}", $y, other),
                    }
                }
            ),*
            other => panic!("unknown year: {}", other),
        }
    };
}
