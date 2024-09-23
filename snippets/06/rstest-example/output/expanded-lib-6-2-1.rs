#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    #[allow(non_camel_case_types)]
    struct 任意のフィクスチャ名 {}
    impl 任意のフィクスチャ名 {
        #[allow(unused_mut)]
        pub fn get() -> i32 {
            任意のフィクスチャ名()
        }
        pub fn default() -> i32 {
            Self::get()
        }
    }
    #[allow(dead_code)]
    fn 任意のフィクスチャ名() -> i32 {
        { 24 }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::\u{4efb}\u{610f}\u{306e}\u{95a2}\u{6570}\u{540d}"]
    pub const 任意の関数名: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "tests::\u{4efb}\u{610f}\u{306e}\u{95a2}\u{6570}\u{540d}",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "snippets/06/rstest-example/src/lib.rs",
            start_line: 11usize,
            start_col: 8usize,
            end_line: 11usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(任意の関数名()),
        ),
    };
    fn 任意の関数名() {
        fn 任意の関数名(任意のフィクスチャ名: i32) {
            {
                match (&(任意のフィクスチャ名 * 2), &48) {
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
        let 任意のフィクスチャ名 = 任意のフィクスチャ名::default();
        任意の関数名(任意のフィクスチャ名)
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&任意の関数名])
}
