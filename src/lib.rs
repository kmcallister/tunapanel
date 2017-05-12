extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate env_logger;
extern crate handlebars;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
mod macros;
mod templates;
pub mod errors;
pub mod widget;
mod panel;
mod server;

pub use templates::HTML;
pub use panel::Panel;
pub use server::{ServerConfig, serve};
