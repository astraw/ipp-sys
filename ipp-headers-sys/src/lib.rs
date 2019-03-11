#![allow(non_upper_case_globals,non_snake_case,non_camel_case_types)]

#[cfg(feature="2017")]
mod ipp_2017;
#[cfg(feature="2017")]
pub use ipp_2017::*;

#[cfg(feature="2018")]
mod ipp_2018;
#[cfg(feature="2018")]
pub use ipp_2018::*;

#[cfg(feature="2019")]
mod ipp_2019;
#[cfg(feature="2019")]
pub use ipp_2019::*;

// Intentionally trigger a compile time error to prevent both 2017 and 2018
// features from being used at once.
#[cfg(all(feature = "2017", feature = "2018"))]
compile_error!("You are attempting to compile both `2017` or `2018` features, \
    but only one can be used.");

// Intentionally trigger a compile time error to prevent both 2018 and 2019
// features from being used at once.
#[cfg(all(feature = "2018", feature = "2019"))]
compile_error!("You are attempting to compile both `2018` or `2019` features, \
    but only one can be used.");

// Intentionally trigger a compile time error to prevent both 2017 and 2019
// features from being used at once.
#[cfg(all(feature = "2017", feature = "2019"))]
compile_error!("You are attempting to compile both `2017` or `2019` features, \
    but only one can be used.");

// Intentionally trigger a compile time error to force a feature flag to be
// used.
#[cfg(not(any(feature = "2017", feature = "2018", feature = "2019")))]
compile_error!("You are attempting to compile without a required feature flag \
    being used. You must use one of either `2017`, `2018`, `2019`");
