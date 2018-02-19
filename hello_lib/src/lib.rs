#![feature(use_extern_macros)]
#![feature(catch_expr)]
#![feature(try_from)]

pub mod math;
pub mod duck;

pub mod zoo {
    pub use duck;
}