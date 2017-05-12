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
    use widget::Button;

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

    tunapanel! {
        struct Types {
            #[label = "u8"]    f_u8:    u8    = 0,
            #[label = "u16"]   f_u16:   u16   = 0,
            #[label = "u32"]   f_u32:   u32   = 0,
            #[label = "u64"]   f_u64:   u64   = 0,
            #[label = "usize"] f_usize: usize = 0,
            #[label = "i8"]    f_i8:    i8    = 0,
            #[label = "i16"]   f_i16:   i16   = 0,
            #[label = "i32"]   f_i32:   i32   = 0,
            #[label = "i64"]   f_i64:   i64   = 0,
            #[label = "isize"] f_isize: isize = 0,
            #[label = "f32"]   f_f32:   f32   = 0.0,
            #[label = "f64"]   f_f64:   f64   = 0.0,
            #[label = "bool"]  f_bool:  bool  = false,

            #[label = "button"]
            f_button: Button = Button::new(),
        }
    }

    #[test]
    fn types() {
        let html = super::panel_html::<Types>();

        for name in &["u8", "u16", "u32", "u64", "usize",
                      "i8", "i16", "i32", "i64", "isize",
                      "f32", "f64", "bool", "button"] {
            let search = format!(r#"tunapanel_name="f_{}""#, name);
            assert!(html.contains(&search));
        }
    }
}
