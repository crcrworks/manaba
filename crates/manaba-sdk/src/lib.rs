pub mod assignment;
pub mod error;

mod client;
pub use client::Client;

mod cookie;
pub use cookie::Cookie;

mod scrape;
pub use scrape::{course::Course, exam::Exam, report::Report};
