//! # Bindings to the `Foundation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/foundation/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! This is the [`std`] equivalent for Objective-C, containing essential data
//! types, collections, and operating-system services.
//!
//!
//! ## Rust vs. Objective-C types
//!
//! A quick overview of some types you will encounter often in Objective-C,
//! and their approximate Rust equivalent.
//!
//! | Objective-C | (approximately) equivalent Rust |
//! | --- | --- |
//! | `NSData*` | `Arc<[u8]>` |
//! | `NSMutableData*` | `Vec<u8>` |
//! | `NSString*` | `Arc<str>` |
//! | `NSMutableString*` | `String` |
//! | `NSValue*` | `Arc<dyn Any>` |
//! | `NSNumber*` | `Arc<enum { I8(i8), U8(u8), I16(i16), U16(u16), I32(i32), U32(u32), I64(i64), U64(u64), F32(f32), F64(f64), CLong(ffi::c_long), CULong(ffi::c_ulong) }>` |
//! | `NSError*` | `Arc<dyn Error + Send + Sync>` |
//! | `NSException*` | `Arc<dyn Error + Send + Sync>` |
//! | `NSRange` | `ops::Range<usize>` |
//! | `NSComparisonResult` | `cmp::Ordering` |
//! | `NSArray<T>*` | `Arc<[T]>` |
//! | `NSMutableArray<T>*` | `Vec<T>` |
//! | `NSDictionary<K, V>*` | `Arc<HashMap<K, V>>` |
//! | `NSMutableDictionary<K, V>*` | `HashMap<K, V>` |
//! | `NSEnumerator<T>*` | `Box<dyn Iterator<T>>` |
//! | `NSCopying*` | `Box<dyn Clone>` |
//!
//!
//! ## Examples
//!
//! Basic usage of a few Foundation types.
//!
//! ```console
//! $ cargo add objc2-foundation --features=all
//! ```
//!
//! ```ignore
//! use objc2_foundation::{ns_string, NSCopying, NSArray};
//!
//! let string = ns_string!("world");
//! println!("hello {string}");
//!
//! let array = NSArray::from_id_slice(&[string.copy()]);
//! println!("{array:?}");
//! ```
//!
//! ```ignore
#![doc = include_str!("../examples/basic_usage.rs")]
//! ```
//!
//! An example showing how to define your own interfaces to parts that may be
//! missing in the autogenerated interface.
//!
//! ```ignore
#![doc = include_str!("../examples/speech_synthesis.rs")]
//! ```
#![no_std]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-foundation/0.2.0")]
#![allow(non_snake_case)]
#![recursion_limit = "256"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub mod __macro_helpers;
#[cfg(feature = "NSEnumerator")]
#[macro_use]
mod iter;
#[cfg(feature = "NSArray")]
pub mod array;
#[cfg(feature = "NSAttributedString")]
mod attributed_string;
#[cfg(feature = "NSBundle")]
mod bundle;
#[cfg(feature = "NSObjCRuntime")]
mod comparison_result;
#[cfg(feature = "NSObject")]
mod copying;
#[cfg(feature = "NSData")]
mod data;
#[cfg(feature = "NSDecimal")]
mod decimal;
#[cfg(feature = "NSDictionary")]
pub mod dictionary;
#[cfg(feature = "NSEnumerator")]
pub mod enumerator;
#[cfg(feature = "NSError")]
mod error;
#[cfg(feature = "NSException")]
mod exception;
#[cfg(feature = "NSEnumerator")]
mod fast_enumeration_state;
mod generated;
mod generics;
#[cfg(feature = "NSGeometry")]
mod geometry;
mod macros;
mod ns_consumed;
#[cfg(feature = "NSValue")]
mod number;
#[cfg(feature = "NSProcessInfo")]
mod process_info;
#[cfg(feature = "NSRange")]
mod range;
#[cfg(feature = "NSSet")]
pub mod set;
#[cfg(feature = "NSString")]
mod string;
#[cfg(test)]
mod tests;
mod thread;
#[cfg(feature = "NSObject")]
mod to_owned;
mod util;
#[cfg(feature = "NSUUID")]
mod uuid;
#[cfg(feature = "NSValue")]
mod value;

#[cfg(feature = "NSObjCRuntime")]
pub use self::comparison_result::NSComparisonResult;
#[cfg(feature = "NSObject")]
pub use self::copying::{NSCopying, NSMutableCopying};
#[cfg(feature = "NSDecimal")]
pub use self::decimal::NSDecimal;
#[cfg(feature = "NSEnumerator")]
pub use self::fast_enumeration_state::NSFastEnumerationState;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generics::*;
#[cfg(feature = "NSGeometry")]
pub use self::geometry::{CGFloat, CGPoint, CGRect, CGSize, NSPoint, NSRect, NSRectEdge, NSSize};
#[cfg(feature = "NSMapTable")]
pub use self::ns_consumed::NSFreeMapTable;
#[cfg(feature = "NSRange")]
pub use self::range::NSRange;
pub use self::thread::MainThreadMarker;
#[cfg(feature = "NSThread")]
pub use self::thread::{is_main_thread, is_multi_threaded};
#[cfg(feature = "NSThread")]
#[cfg(feature = "dispatch")]
pub use self::thread::{run_on_main, MainThreadBound};

// Available under Foundation, so makes sense here as well:
// https://developer.apple.com/documentation/foundation/numbers_data_and_basic_values?language=objc
pub use objc2::ffi::{NSInteger, NSUInteger};

// Special types that are stored in `objc2`, but really belong here
#[doc(inline)]
#[cfg(feature = "NSZone")]
pub use objc2::runtime::NSZone;
#[doc(inline)]
#[cfg(feature = "NSProxy")]
pub use objc2::runtime::__NSProxy as NSProxy;
pub use objc2::runtime::{NSObject, NSObjectProtocol};

#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-base", kind = "dylib"))]
extern "C" {}

// MacTypes.h
#[allow(unused)]
pub(crate) type Boolean = u8; // unsigned char
#[allow(unused)]
pub(crate) type FourCharCode = u32;
#[allow(unused)]
pub type OSType = FourCharCode;
#[allow(unused)]
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?

// Temporary
#[allow(non_snake_case, unused, unreachable_pub)]
mod Foundation {
    pub use crate::*;
}
