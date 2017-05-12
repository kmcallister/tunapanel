pub type HTML = String;

pub fn text_box(name: &str, label: &str, conv: Option<&str>) -> HTML {
    let mut attrs = format!(
        r#"type="text" class="tunapanel_widget" tunapanel_name="{}""#,
        name);

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
        text_box(name, label, Some("number"))
    }
}

impl Controllable for str {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, label, None)
    }
}
