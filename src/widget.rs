pub type HTML = String;

pub fn text_box(name: &str, label: &str) -> HTML {
    format!(r#"
<div>
    <span class="tunapanel_label">{}</span>
    <input type="text" class="tunapanel_widget" tunapanel_name="{}">
</div>
    "#, label, name)
}

pub trait Controllable {
    fn widget(&self, name: &str, label: &str) -> HTML;
}

impl Controllable for f32 {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, label)
    }
}

impl Controllable for str {
    fn widget(&self, name: &str, label: &str) -> HTML {
        text_box(name, label)
    }
}
