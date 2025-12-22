macro_rules! day {
    ($arg:ident, $test1:expr, $test2:expr,  $problem1:expr, $problem2:expr, $example_input:expr, $main_input:expr) => {
        #[allow(dead_code)]
        pub static EX_INPUT1: &str = $example_input;
        #[allow(dead_code)]
        pub static INPUT: &str = $main_input;

        #[allow(dead_code)]
        pub fn p1($arg: &str) -> usize {
            $problem1
        }

        #[allow(dead_code)]
        pub fn p2($arg: &str) -> usize {
            $problem2
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test1() {
                assert_eq!(p1(EX_INPUT1), $test1);
            }

            #[test]
            fn test2() {
                assert_eq!(p2(EX_INPUT1), $test2);
            }
        }
    };
}

pub(crate) use day;
