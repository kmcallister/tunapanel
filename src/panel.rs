use serde::de::DeserializeOwned;

use widget::HTML;

pub trait Panel: DeserializeOwned {
    fn widgets() -> HTML;
}

pub fn panel_html<P: Panel>() -> HTML {
    let mut html = String::new();

    html.push_str(&<P as Panel>::widgets());
    html.push_str(::HTML_FOOTER);

    html
}

#[cfg(test)]
mod test {
    use serde_json;

    tunapanel! {
        #[derive(Debug)]
        struct Panel {
            #[label = "A float"]
            x: f32    = 0.0,

            #[label = "A string"]
            y: String = String::new(),
        }
    }

    #[test]
    fn direct_deserialize() {
        let s = r#"
    {
        "x": 3.4,
        "y": "hello"
    }
        "#;

        let d: Panel = serde_json::from_str(&s).unwrap();
        assert_eq!(d.x, 3.4);
        assert_eq!(d.y, "hello");
    }

    #[test]
    fn panel_html() {
        let html = super::panel_html::<Panel>();

        assert!(html.contains(r#"tunapanel_name="x""#));
        assert!(html.contains(r#"tunapanel_name="y""#));
        assert!(html.contains(r#"Status: <span id="tunapanel_status"></span>"#));
        assert!(html.contains(r#"https://code.jquery.com"#));
        assert!(html.contains(r#"JSON.stringify(obj)"#));
    }

    tunapanel! {
        struct EscTest {
            #[label = "Test \' escaping <<>"]
            x: f32 = 0.0,

            #[label = "& another `test%"]
            y: String = "Attribute \" escaping \'".to_owned(),
        }
    }

    #[test]
    fn escaping() {
        let html = super::panel_html::<EscTest>();

        assert!(html.contains(r#"Test ' escaping &lt;&lt;&gt;"#));
        assert!(html.contains(r#"&amp; another `test%"#));
        assert!(html.contains(r#""Attribute &quot; escaping '""#));
    }
}
