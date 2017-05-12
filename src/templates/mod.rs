use handlebars::Handlebars;

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

        register!("text_box");
        register!("checkbox");
        register!("button");

        handlebars
    };
}
