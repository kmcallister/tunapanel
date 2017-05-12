use handlebars::Handlebars;

/// Data type representing HTML source.
///
/// For now this is just a `String`.
pub type HTML = String;

lazy_static! {
    pub static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();

        macro_rules! register {
            ($name:expr) => {
                handlebars.register_template_string(
                    $name,
                    include_str!(concat!($name, ".html"))
                ).unwrap();
            }
        }

        register!("header");
        register!("footer");
        register!("text_box");
        register!("checkbox");
        register!("button");

        handlebars
    };
}
