use std::fmt::Display;

pub type HTML = String;

pub fn text_box<V>(name: &str, value: V, label: &str, conv: Option<&str>)
    -> HTML
    where V: Display
{
    let mut attrs = format!(
        r#"type="text" value="{}" class="tunapanel_widget" tunapanel_name="{}""#,
        value, name);

    if let Some(conv) = conv {
        attrs.push_str(r#" tunapanel_conv=""#);
        attrs.push_str(conv);
        attrs.push_str(r#"""#);
    }

    format!(r#"
<div>
    <span class="tunapanel_label">{}</span>
    <input {}>
</div>
    "#, label, attrs)
}

pub trait Controllable {
    fn widget(&self, name: &str, label: &str) -> HTML;
}

impl Controllable for f32 {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, self, label, Some("number"))
    }
}

impl Controllable for str {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, self, label, None)
    }
}
