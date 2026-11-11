#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod components;
pub mod execution;
pub mod model;
pub mod windows;

pub use app::DemosimApp;

#[cfg(test)]
mod test_support;
