extern crate serde;
extern crate serde_json;
extern crate hyper;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate env_logger;

#[macro_use]
mod macros;
pub mod errors;
pub mod widget;
pub mod panel;
mod server;

pub use server::{ServerConfig, serve};

static HTML_FOOTER: &'static str
    = include_str!("footer.html");
