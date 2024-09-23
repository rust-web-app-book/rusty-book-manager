#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use mockall::predicate::*;
use mockall::*;
trait 任意のトレイト名 {
    fn 任意のメソッド名(&self, x: u32) -> u32;
}
#[allow(non_snake_case)]
#[allow(missing_docs)]
pub mod __mock_Mock任意のトレイト名 {
    use super::*;
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(missing_docs)]
struct Mock任意のトレイト名 {
    任意のトレイト名_expectations: Mock任意のトレイト名_任意のトレイト名,
}
impl ::std::fmt::Debug for Mock任意のトレイト名 {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Mock任意のトレイト名").finish()
    }
}
impl ::std::default::Default for Mock任意のトレイト名 {
    #[allow(clippy::default_trait_access)]
    fn default() -> Self {
        Self {
            任意のトレイト名_expectations: Default::default(),
        }
    }
}
#[allow(non_snake_case)]
#[allow(missing_docs)]
pub mod __mock_Mock任意のトレイト名_任意のトレイト名 {
    use super::*;
    #[allow(missing_docs)]
    pub mod __任意のメソッド名 {
        use super::*;
        use ::mockall::CaseTreeExt;
        use ::std::{
            boxed::Box, mem, ops::{DerefMut, Range},
            sync::Mutex, vec::Vec,
        };
        #[allow(clippy::unused_unit)]
        enum Rfunc {
            Default,
            Expired,
            Mut(Box<dyn FnMut(u32) -> u32 + Send>),
            MutSt(::mockall::Fragile<Box<dyn FnMut(u32) -> u32>>),
            Once(Box<dyn FnOnce(u32) -> u32 + Send>),
            OnceSt(::mockall::Fragile<Box<dyn FnOnce(u32) -> u32>>),
            _Phantom(Box<dyn Fn() + Send>),
        }
        impl Rfunc {
            fn call_mut(&mut self, x: u32) -> std::result::Result<u32, &'static str> {
                match self {
                    Rfunc::Default => {
                        use ::mockall::ReturnDefault;
                        ::mockall::DefaultReturner::<u32>::return_default()
                    }
                    Rfunc::Expired => Err("called twice, but it returns by move"),
                    Rfunc::Mut(__mockall_f) => ::std::result::Result::Ok(__mockall_f(x)),
                    Rfunc::MutSt(__mockall_f) => {
                        ::std::result::Result::Ok((__mockall_f.get_mut())(x))
                    }
                    Rfunc::Once(_) => {
                        if let Rfunc::Once(mut __mockall_f) = mem::replace(
                            self,
                            Rfunc::Expired,
                        ) {
                            ::std::result::Result::Ok(__mockall_f(x))
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                    Rfunc::OnceSt(_) => {
                        if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                            self,
                            Rfunc::Expired,
                        ) {
                            ::std::result::Result::Ok((__mockall_f.into_inner())(x))
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                    Rfunc::_Phantom(_) => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl std::default::Default for Rfunc {
            fn default() -> Self {
                Rfunc::Default
            }
        }
        enum Matcher {
            Always,
            Func(Box<dyn Fn(&u32) -> bool + Send>),
            FuncSt(::mockall::Fragile<Box<dyn Fn(&u32) -> bool>>),
            Pred(Box<(Box<dyn ::mockall::Predicate<u32> + Send>,)>),
            _Phantom(Box<dyn Fn() + Send>),
        }
        impl Matcher {
            #[allow(clippy::ptr_arg)]
            fn matches(&self, x: &u32) -> bool {
                match self {
                    Matcher::Always => true,
                    Matcher::Func(__mockall_f) => __mockall_f(x),
                    Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(x),
                    Matcher::Pred(__mockall_pred) => {
                        [__mockall_pred.0.eval(x)].iter().all(|__mockall_x| *__mockall_x)
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl Default for Matcher {
            #[allow(unused_variables)]
            fn default() -> Self {
                Matcher::Always
            }
        }
        impl ::std::fmt::Display for Matcher {
            fn fmt(
                &self,
                __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                match self {
                    Matcher::Always => {
                        __mockall_fmt.write_fmt(format_args!("<anything>"))
                    }
                    Matcher::Func(_) => {
                        __mockall_fmt.write_fmt(format_args!("<function>"))
                    }
                    Matcher::FuncSt(_) => {
                        __mockall_fmt
                            .write_fmt(format_args!("<single threaded function>"))
                    }
                    Matcher::Pred(__mockall_p) => {
                        __mockall_fmt.write_fmt(format_args!("{0}", __mockall_p.0))
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        /// Holds the stuff that is independent of the output type
        struct Common {
            matcher: Mutex<Matcher>,
            seq_handle: Option<::mockall::SeqHandle>,
            times: ::mockall::Times,
        }
        impl std::default::Default for Common {
            fn default() -> Self {
                Common {
                    matcher: Mutex::new(Matcher::default()),
                    seq_handle: None,
                    times: ::mockall::Times::default(),
                }
            }
        }
        impl Common {
            fn call(&self, desc: &str) {
                self.times
                    .call()
                    .unwrap_or_else(|m| {
                        let desc = {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", self.matcher.lock().unwrap()),
                            );
                            res
                        };
                        {
                            ::std::rt::panic_fmt(
                                format_args!(
                                    "{0}: Expectation({1}) {2}",
                                    "Mock任意のトレイト名::任意のメソッド名",
                                    desc,
                                    m,
                                ),
                            );
                        };
                    });
                self.verify_sequence(desc);
                if ::mockall::ExpectedCalls::TooFew != self.times.is_satisfied() {
                    self.satisfy_sequence()
                }
            }
            fn in_sequence(
                &mut self,
                __mockall_seq: &mut ::mockall::Sequence,
            ) -> &mut Self {
                if !self.times.is_exact() {
                    {
                        ::std::rt::begin_panic(
                            "Only Expectations with an exact call count have sequences",
                        );
                    }
                }
                self.seq_handle = Some(__mockall_seq.next_handle());
                self
            }
            fn is_done(&self) -> bool {
                self.times.is_done()
            }
            #[allow(clippy::ptr_arg)]
            fn matches(&self, x: &u32) -> bool {
                self.matcher.lock().unwrap().matches(x)
            }
            /// Forbid this expectation from ever being called.
            fn never(&mut self) {
                self.times.never();
            }
            fn satisfy_sequence(&self) {
                if let Some(__mockall_handle) = &self.seq_handle {
                    __mockall_handle.satisfy()
                }
            }
            /// Expect this expectation to be called any number of times
            /// contained with the given range.
            fn times<MockallR>(&mut self, __mockall_r: MockallR)
            where
                MockallR: Into<::mockall::TimesRange>,
            {
                self.times.times(__mockall_r)
            }
            fn with<MockallMatcher0: ::mockall::Predicate<u32> + Send + 'static>(
                &mut self,
                x: MockallMatcher0,
            ) {
                let mut __mockall_guard = self.matcher.lock().unwrap();
                *__mockall_guard.deref_mut() = Matcher::Pred(Box::new((Box::new(x),)));
            }
            fn withf<MockallF>(&mut self, __mockall_f: MockallF)
            where
                MockallF: Fn(&u32) -> bool + Send + 'static,
            {
                let mut __mockall_guard = self.matcher.lock().unwrap();
                *__mockall_guard.deref_mut() = Matcher::Func(Box::new(__mockall_f));
            }
            fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
            where
                MockallF: Fn(&u32) -> bool + 'static,
            {
                let mut __mockall_guard = self.matcher.lock().unwrap();
                *__mockall_guard
                    .deref_mut() = Matcher::FuncSt(
                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                );
            }
            fn verify_sequence(&self, desc: &str) {
                if let Some(__mockall_handle) = &self.seq_handle {
                    __mockall_handle.verify(desc)
                }
            }
        }
        impl Drop for Common {
            fn drop(&mut self) {
                if !::std::thread::panicking() {
                    let desc = {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}", self.matcher.lock().unwrap()),
                        );
                        res
                    };
                    match self.times.is_satisfied() {
                        ::mockall::ExpectedCalls::TooFew => {
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                        "Mock任意のトレイト名::任意のメソッド名",
                                        desc,
                                        self.times.count(),
                                        self.times.minimum(),
                                    ),
                                );
                            };
                        }
                        ::mockall::ExpectedCalls::TooMany => {
                            {
                                ::std::rt::panic_fmt(
                                    format_args!(
                                        "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                        "Mock任意のトレイト名::任意のメソッド名",
                                        desc,
                                        self.times.count(),
                                        self.times.maximum(),
                                    ),
                                );
                            };
                        }
                        _ => {}
                    }
                }
            }
        }
        /// Expectation type for methods that return a `'static` type.
        /// This is the type returned by the `expect_*` methods.
        pub(in super::super) struct Expectation {
            common: Common,
            rfunc: Mutex<Rfunc>,
        }
        #[allow(clippy::unused_unit)]
        impl Expectation {
            /// Call this [`Expectation`] as if it were the real method.
            #[doc(hidden)]
            pub(in super::super) fn call(&self, x: u32) -> u32 {
                self.common
                    .call(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Mock任意のトレイト名::任意のメソッド名({0:?})",
                                    ::mockall::MaybeDebugger(&x),
                                ),
                            );
                            res
                        },
                    );
                self.rfunc
                    .lock()
                    .unwrap()
                    .call_mut(x)
                    .unwrap_or_else(|message| {
                        let desc = {
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", self.common.matcher.lock().unwrap()),
                            );
                            res
                        };
                        {
                            ::std::rt::panic_fmt(
                                format_args!(
                                    "{0}: Expectation({1}) {2}",
                                    "Mock任意のトレイト名::任意のメソッド名",
                                    desc,
                                    message,
                                ),
                            );
                        };
                    })
            }
            /// Return a constant value from the `Expectation`
            ///
            /// The output type must be `Clone`.  The compiler can't always
            /// infer the proper type to use with this method; you will
            /// usually need to specify it explicitly.  i.e.
            /// `return_const(42i32)` instead of `return_const(42)`.
            #[allow(unused_variables)]
            pub(in super::super) fn return_const<MockallOutput>(
                &mut self,
                __mockall_c: MockallOutput,
            ) -> &mut Self
            where
                MockallOutput: Clone + Into<u32> + Send + 'static,
            {
                self.returning(move |x| __mockall_c.clone().into())
            }
            /// Single-threaded version of
            /// [`return_const`](#method.return_const).  This is useful for
            /// return types that are not `Send`.
            ///
            /// The output type must be `Clone`.  The compiler can't always
            /// infer the proper type to use with this method; you will
            /// usually need to specify it explicitly.  i.e.
            /// `return_const(42i32)` instead of `return_const(42)`.
            ///
            /// It is a runtime error to call the mock method from a
            /// different thread than the one that originally called this
            /// method.
            #[allow(unused_variables)]
            pub(in super::super) fn return_const_st<MockallOutput>(
                &mut self,
                __mockall_c: MockallOutput,
            ) -> &mut Self
            where
                MockallOutput: Clone + Into<u32> + 'static,
            {
                self.returning_st(move |x| __mockall_c.clone().into())
            }
            /// Supply an `FnOnce` closure that will provide the return
            /// value for this Expectation.  This is useful for return types
            /// that aren't `Clone`.  It will be an error to call this
            /// method multiple times.
            pub(in super::super) fn return_once<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: FnOnce(u32) -> u32 + Send + 'static,
            {
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    *__mockall_guard.deref_mut() = Rfunc::Once(Box::new(__mockall_f));
                }
                self
            }
            /// Single-threaded version of
            /// [`return_once`](#method.return_once).  This is useful for
            /// return types that are neither `Send` nor `Clone`.
            ///
            /// It is a runtime error to call the mock method from a
            /// different thread than the one that originally called this
            /// method.  It is also a runtime error to call the method more
            /// than once.
            pub(in super::super) fn return_once_st<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: FnOnce(u32) -> u32 + 'static,
            {
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Rfunc::OnceSt(
                        ::mockall::Fragile::new(Box::new(__mockall_f)),
                    );
                }
                self
            }
            /// Supply a closure that will provide the return value for this
            /// `Expectation`.  The method's arguments are passed to the
            /// closure by value.
            pub(in super::super) fn returning<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: FnMut(u32) -> u32 + Send + 'static,
            {
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    *__mockall_guard.deref_mut() = Rfunc::Mut(Box::new(__mockall_f));
                }
                self
            }
            /// Single-threaded version of [`returning`](#method.returning).
            /// Can be used when the argument or return type isn't `Send`.
            ///
            /// It is a runtime error to call the mock method from a
            /// different thread than the one that originally called this
            /// method.
            pub(in super::super) fn returning_st<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: FnMut(u32) -> u32 + 'static,
            {
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    *__mockall_guard
                        .deref_mut() = Rfunc::MutSt(
                        ::mockall::Fragile::new(Box::new(__mockall_f)),
                    );
                }
                self
            }
            /// Add this expectation to a
            /// [`Sequence`](../../../mockall/struct.Sequence.html).
            pub(in super::super) fn in_sequence(
                &mut self,
                __mockall_seq: &mut ::mockall::Sequence,
            ) -> &mut Self {
                self.common.in_sequence(__mockall_seq);
                self
            }
            fn is_done(&self) -> bool {
                self.common.is_done()
            }
            /// Validate this expectation's matcher.
            #[allow(clippy::ptr_arg)]
            fn matches(&self, x: &u32) -> bool {
                self.common.matches(x)
            }
            /// Forbid this expectation from ever being called.
            pub(in super::super) fn never(&mut self) -> &mut Self {
                self.common.never();
                self
            }
            /// Create a new, default, [`Expectation`](struct.Expectation.html)
            pub(in super::super) fn new() -> Self {
                Self::default()
            }
            /// Expect this expectation to be called exactly once.  Shortcut for
            /// [`times(1)`](#method.times).
            pub(in super::super) fn once(&mut self) -> &mut Self {
                self.times(1)
            }
            /// Restrict the number of times that that this method may be called.
            ///
            /// The argument may be:
            /// * A fixed number: `.times(4)`
            /// * Various types of range:
            ///   - `.times(5..10)`
            ///   - `.times(..10)`
            ///   - `.times(5..)`
            ///   - `.times(5..=10)`
            ///   - `.times(..=10)`
            /// * The wildcard: `.times(..)`
            pub(in super::super) fn times<MockallR>(
                &mut self,
                __mockall_r: MockallR,
            ) -> &mut Self
            where
                MockallR: Into<::mockall::TimesRange>,
            {
                self.common.times(__mockall_r);
                self
            }
            /// Set matching crieteria for this Expectation.
            ///
            /// The matching predicate can be anything implemening the
            /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
            /// one matcher can be set per `Expectation` at a time.
            pub(in super::super) fn with<
                MockallMatcher0: ::mockall::Predicate<u32> + Send + 'static,
            >(&mut self, x: MockallMatcher0) -> &mut Self {
                self.common.with(x);
                self
            }
            /// Set a matching function for this Expectation.
            ///
            /// This is equivalent to calling [`with`](#method.with) with a
            /// function argument, like `with(predicate::function(f))`.
            pub(in super::super) fn withf<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: Fn(&u32) -> bool + Send + 'static,
            {
                self.common.withf(__mockall_f);
                self
            }
            /// Single-threaded version of [`withf`](#method.withf).
            /// Can be used when the argument type isn't `Send`.
            pub(in super::super) fn withf_st<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: Fn(&u32) -> bool + 'static,
            {
                self.common.withf_st(__mockall_f);
                self
            }
        }
        impl Default for Expectation {
            fn default() -> Self {
                Expectation {
                    common: Common::default(),
                    rfunc: Mutex::new(Rfunc::default()),
                }
            }
        }
        /// A collection of [`Expectation`](struct.Expectations.html)
        /// objects.  Users will rarely if ever use this struct directly.
        #[doc(hidden)]
        pub(in super::super) struct Expectations(Vec<Expectation>);
        impl Expectations {
            /// Verify that all current expectations are satisfied and clear
            /// them.
            pub(in super::super) fn checkpoint(
                &mut self,
            ) -> std::vec::Drain<Expectation> {
                self.0.drain(..)
            }
            /// Create a new expectation for this method.
            pub(in super::super) fn expect(&mut self) -> &mut Expectation {
                self.0.push(Expectation::default());
                let __mockall_l = self.0.len();
                &mut self.0[__mockall_l - 1]
            }
            pub(in super::super) fn new() -> Self {
                Self::default()
            }
        }
        impl Default for Expectations {
            fn default() -> Self {
                Expectations(Vec::new())
            }
        }
        impl Expectations {
            /// Simulate calling the real method.  Every current expectation
            /// will be checked in FIFO order and the first one with
            /// matching arguments will be used.
            pub(in super::super) fn call(&self, x: u32) -> Option<u32> {
                self.0
                    .iter()
                    .find(|__mockall_e| {
                        __mockall_e.matches(&x)
                            && (!__mockall_e.is_done() || self.0.len() == 1)
                    })
                    .map(move |__mockall_e| __mockall_e.call(x))
            }
        }
    }
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(missing_docs)]
struct Mock任意のトレイト名_任意のトレイト名 {
    任意のメソッド名: __mock_Mock任意のトレイト名_任意のトレイト名::__任意のメソッド名::Expectations,
}
impl ::std::default::Default for Mock任意のトレイト名_任意のトレイト名 {
    fn default() -> Self {
        Self {
            任意のメソッド名: Default::default(),
        }
    }
}
impl Mock任意のトレイト名_任意のトレイト名 {
    /// Validate that all current expectations for all methods have
    /// been satisfied, and discard them.
    pub fn checkpoint(&mut self) {
        {
            self.任意のメソッド名.checkpoint();
        }
    }
}
impl Mock任意のトレイト名 {
    /// Validate that all current expectations for all methods have
    /// been satisfied, and discard them.
    pub fn checkpoint(&mut self) {
        self.任意のトレイト名_expectations.checkpoint();
    }
    /// Create a new mock object with no expectations.
    ///
    /// This method will not be generated if the real struct
    /// already has a `new` method.  However, it *will* be
    /// generated if the struct implements a trait with a `new`
    /// method.  The trait's `new` method can still be called
    /// like `<MockX as TraitY>::new`
    pub fn new() -> Self {
        Self::default()
    }
}
impl 任意のトレイト名 for Mock任意のトレイト名 {
    fn 任意のメソッド名(&self, x: u32) -> u32 {
        let no_match_msg = {
            let res = ::alloc::fmt::format(
                format_args!(
                    "{0}: No matching expectation found",
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Mock任意のトレイト名::任意のメソッド名({0:?})",
                                ::mockall::MaybeDebugger(&x),
                            ),
                        );
                        res
                    },
                ),
            );
            res
        };
        self.任意のトレイト名_expectations
            .任意のメソッド名
            .call(x)
            .expect(&no_match_msg)
    }
}
impl Mock任意のトレイト名 {
    #[must_use = "Must set return value when not using the \"nightly\" feature"]
    ///Create an [`Expectation`](__mock_Mock任意のトレイト名_任意のトレイト名/__任意のメソッド名/struct.Expectation.html) for mocking the `任意のメソッド名` method
    fn expect_任意のメソッド名(
        &mut self,
    ) -> &mut __mock_Mock任意のトレイト名_任意のトレイト名::__任意のメソッド名::Expectation {
        self.任意のトレイト名_expectations.任意のメソッド名.expect()
    }
}
fn トレイトオブジェクトを引数に取る関数(
    x: &dyn 任意のトレイト名,
    v: u32,
) -> u32 {
    x.任意のメソッド名(v)
}
#[allow(dead_code)]
fn main() {
    let mut mock = Mock任意のトレイト名::new();
    mock.expect_任意のメソッド名().returning(|x| x + 1);
    match (&10, &トレイトオブジェクトを引数に取る関数(&mock, 9)) {
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
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
