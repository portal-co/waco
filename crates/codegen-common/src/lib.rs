#![no_std]
#[doc(hidden)]
pub use core;

#[cfg(feature = "alloc")]
#[doc(hidden)]
pub extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod cgu;
pub mod cgu_traits;