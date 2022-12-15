// Copyright 2019 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A glyph rasterizer for webrender
//!
//! ## Overview
//!
//! ## Usage
//!

mod gamma_lut;
mod rasterizer;
mod telemetry;
mod types;

pub mod profiler;

pub use rasterizer::*;
pub use types::*;

#[macro_use]
extern crate malloc_size_of_derive;
#[macro_use]
extern crate tracy_rs;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate smallvec;

#[cfg(any(feature = "serde"))]
#[macro_use]
extern crate serde;

extern crate malloc_size_of;

pub mod platform {
    #[cfg(target_os = "macos")]
    pub use crate::platform::macos::font;
    #[cfg(any(target_os = "android", all(unix, not(target_os = "macos"))))]
    pub use crate::platform::unix::font;
    #[cfg(target_os = "windows")]
    pub use crate::platform::windows::font;

    #[cfg(target_os = "macos")]
    pub mod macos {
        pub mod font;
    }
    #[cfg(any(target_os = "android", all(unix, not(target_os = "macos"))))]
    pub mod unix {
        pub mod font;
    }
    #[cfg(target_os = "windows")]
    pub mod windows {
        pub mod font;
    }
}
