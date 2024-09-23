#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub fn sub(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[cfg(test)]
    fn test_sub(a: i32, b: i32, expected: i32) {
        {
            match (&sub(a, b), &expected) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
    }
    #[cfg(test)]
    mod test_sub {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "tests::test_sub::case_1"]
        pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("tests::test_sub::case_1"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "snippets/06/rstest-example/src/lib.rs",
                start_line: 11usize,
                start_col: 8usize,
                end_line: 11usize,
                end_col: 16usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_1()),
            ),
        };
        fn case_1() {
            let a = 10;
            let b = 0;
            let expected = 10;
            test_sub(a, b, expected)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker = "tests::test_sub::case_2"]
        pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("tests::test_sub::case_2"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "snippets/06/rstest-example/src/lib.rs",
                start_line: 11usize,
                start_col: 8usize,
                end_line: 11usize,
                end_col: 16usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_2()),
            ),
        };
        fn case_2() {
            let a = 100;
            let b = 5;
            let expected = 95;
            test_sub(a, b, expected)
        }
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&case_1, &case_2])
}
