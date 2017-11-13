//! Widgets which can make up a panel.
//!
//! Basic data types are supported -- see the `Controllable` trait.
//! In addition there are some special widgets.

use std::fmt::Display;
use std::default::Default;

use templates::{HANDLEBARS, HTML};

#[derive(Serialize)]
struct TextBoxFields<'a> {
    input_type: &'a str,
    name: &'a str,
    value: &'a str,
    label: &'a str,
    conv: &'a str,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum TextBoxType {
    Text,
    Password,
}

impl TextBoxType {
    fn as_str(&self) -> &'static str {
        match *self {
            TextBoxType::Text => "text",
            TextBoxType::Password => "password",
        }
    }
}

fn text_box<V>(name: &str, value: &V, label: &str, conv: &str, input_type: TextBoxType)
    -> HTML
    where V: Display + ?Sized
{
    HANDLEBARS.render("text_box", &TextBoxFields {
        name: name,
        value: &format!("{}", value),
        label: label,
        conv: conv,
        input_type: input_type.as_str(),
    }).unwrap()
}

#[derive(Serialize)]
struct CheckboxFields<'a> {
    name: &'a str,
    value: bool,
    label: &'a str,
}

#[derive(Serialize)]
struct ButtonFields<'a> {
    name: &'a str,
    label: &'a str,
}

/// Trait for values controllable by a panel.
pub trait Controllable {
    fn widget(&self, name: &str, label: &str) -> HTML;
}

impl Controllable for str {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, self, label, "none", TextBoxType::Text)
    }
}

impl Controllable for bool {
    fn widget(&self, name: &str, label: &str) -> HTML {
        HANDLEBARS.render("checkbox", &CheckboxFields {
            name: name,
            value: *self,
            label: label,
        }).unwrap()
    }
}

macro_rules! controllable_numbers {
    ($($num_ty:ty),*) => {
        $(
            impl Controllable for $num_ty {
                fn widget(&self, name: &str, label: &str) -> HTML {
                    text_box(name, self, label, "number", TextBoxType::Text)
                }
            }
        )*
    };
}

controllable_numbers!(i8, i16, i32, i64, isize,
                      u8, u16, u32, u64, usize,
                      f32, f64);

/// A button.
///
/// `button.0` is a `bool` indicating whether the update happened
/// as a result of the user clicking this button.
#[derive(Debug, Deserialize)]
pub struct Button(pub bool);

impl Button {
    /// Create a button.
    pub fn new() -> Button {
        Button(false)
    }
}

impl Default for Button {
    fn default() -> Button {
        Button::new()
    }
}

impl Controllable for Button {
    fn widget(&self, name: &str, label: &str) -> HTML {
        HANDLEBARS.render("button", &ButtonFields {
            name: name,
            label: label,
        }).unwrap()
    }
}

/// Password field
///
/// This is the same as `String` but rendered as password input.
#[derive(Debug, Deserialize)]
pub struct Password(pub String);

impl Password {
    pub fn new() -> Self {
        Password(String::new())
    }
}

impl Default for Password {
    fn default() -> Self {
        Password::new()
    }
}

impl Controllable for Password {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, &self.0, label, "none", TextBoxType::Password)
    }
}
