use std::fmt::Display;

use handlebars::Handlebars;

pub type HTML = String;

lazy_static! {
    static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_string("text_box", r#"
<div>
    <span class="tunapanel_label">{{ label }}</span>
    <input
        type="text"
        class="tunapanel_widget"
        value="{{ value }}"
        tunapanel_name="{{ name }}"
        tunapanel_conv="{{ conv }}">
</div>
        "#).unwrap();

        handlebars.register_template_string("checkbox", r#"
<div>
    <span class="tunapanel_label">{{ label }}</span>
    <input
        type="checkbox"
        class="tunapanel_widget"
        {{#if value}}checked{{/if}}
        tunapanel_name="{{ name }}"
        tunapanel_conv="checkbox">
</div>
        "#).unwrap();

        handlebars
    };
}

#[derive(Serialize)]
struct TextBox<'a> {
    name: &'a str,
    value: &'a str,
    label: &'a str,
    conv: &'a str,
}

fn text_box<V>(name: &str, value: V, label: &str, conv: &str)
    -> HTML
    where V: Display
{
    HANDLEBARS.render("text_box", &TextBox {
        name: name,
        value: &format!("{}", value),
        label: label,
        conv: conv,
    }).unwrap()
}

#[derive(Serialize)]
struct Checkbox<'a> {
    name: &'a str,
    value: bool,
    label: &'a str,
}

pub trait Controllable {
    fn widget(&self, name: &str, label: &str) -> HTML;
}

impl Controllable for str {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, self, label, "none")
    }
}

impl Controllable for bool {
    fn widget(&self, name: &str, label: &str) -> HTML {
        HANDLEBARS.render("checkbox", &Checkbox {
            name: name,
            value: *self,
            label: label,
        }).unwrap()
    }
}

macro_rules! controllable_number {
    ($num_ty:ty) => {
        impl Controllable for $num_ty {
            fn widget(&self, name: &str, label: &str) -> HTML {
                text_box(name, self, label, "number")
            }
        }
    };
}

controllable_number!(i8);
controllable_number!(i16);
controllable_number!(i32);
controllable_number!(i64);
controllable_number!(isize);
controllable_number!(u8);
controllable_number!(u16);
controllable_number!(u32);
controllable_number!(u64);
controllable_number!(usize);
controllable_number!(f32);
controllable_number!(f64);
