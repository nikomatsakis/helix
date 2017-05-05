// use coercions::*;
use sys::{VALUE, Qnil};
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};
use ruby::Value;

impl<'a, T, U> UncheckedValue<Option<T>> for Value<'a>
    where Value<'a>: UncheckedValue<T, ToRust=U>,
          U: ToRust<T>,
{
    type ToRust = Option<U>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { self.inner() } == unsafe { Qnil } {
            Ok(None)
        } else {
            let val = UncheckedValue::<T>::to_checked(self);
            match val {
                // TODO: This transmute is caused by a compiler issue I think?
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e)
            }
        }
    }
}

impl<'a, T, U> ToRust<Option<T>> for Option<U> where U: ToRust<T> {
    fn to_rust(self) -> Option<T> {
        self.map(|t| t.to_rust())
    }
}

// impl<T> ToRust<T> for Box<ToRust<T>> {
//     fn to_rust(self) -> T {
//         self.to_rust()
//     }
// }

// struct CheckedOption<'a, T> {
//     inner: VALUE,
//     target: PhantomData<T>,
//     frame: CallFrame<'a>,
//     present: bool,
// }

// impl<T> UncheckedValue<Option<T>> for VALUE where VALUE: UncheckedValue<T> {
//     type ToRust = CheckedOption<T>;

//     fn to_checked<'a>(self, frame: CallFrame<'a>) -> CheckResult<Self::ToRust> {
//         CheckedOption { inner: self, target: PhantomData, frame: frame, present: false }
//     }
// }

// // impl<'a, TARGET, U> ToRust<Option<TARGET>> for CheckedOption<'a, U> where U: ToRust<TARGET>

// // impl<'a, T, U: ToRust<T>> UncheckedValue<Option<T>> for ruby::Value<'a> where ruby::Value<'a>: UncheckedValue<T> {
// //     type ToRust = CheckedOption<'a>;

// //     fn to_checked<'frame>(self, frame: CallFrame<'frame>) -> CheckResult<Self::ToRust> {
// //         if unsafe { self.inner() == Qnil } {
// //             Ok(CheckedOption { inner: self, present: false })
// //         } else {
// //             match UncheckedValue::<T>::to_checked(self, frame) {
// //                 Ok(checked) => Ok(CheckedOption { inner: self, present: true }),
// //                 Err(e) => Err(e)
// //             }
// //         }
// //     }
// // }

// // impl<'a, T, U> ToRust<Option<U>> for CheckedOption<'a> where U: ToRust<T> {
// //     fn to_rust(self) -> Option<U> {
// //         if self.present {
// //             ToRust::<Option<U>>::to_rust(self.inner)
// //         } else {
// //             None
// //         }
// //     }
// // }

// impl<T> ToRuby for Option<T> where T: ToRuby {
//     fn to_ruby(&self) -> VALUE {
//         match *self {
//             Some(ref value) => value.to_ruby(),
//             None => unsafe { Qnil }
//         }
//     }
// }
