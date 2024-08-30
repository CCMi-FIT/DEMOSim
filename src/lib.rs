#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod model;
pub mod execution;
pub mod components;
pub mod windows;
pub mod persistence;

pub use app::DemosimApp;
