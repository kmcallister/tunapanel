extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod macros;
pub mod widget;
pub mod panel;

pub use widget::HTML;
pub use panel::Panel;

static HTML_FOOTER: &'static str
    = include_str!("footer.html");
